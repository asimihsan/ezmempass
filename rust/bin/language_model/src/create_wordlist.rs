use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use flate2::read::GzDecoder;
use scoped_threadpool::Pool;
use std::cmp::max;
use std::io;
use std::sync::mpsc;
use unicode_normalization::UnicodeNormalization;

/// If a word is not in the dictionry change it to this. This will never appear in the corpus
/// because we trim puncutation from the beginning and ends of words.
const OUT_OF_VOCABULARY_WORD: &str = "<unk>";

/// TODO
///
/// -   Use the twogram iterator and calculate onegram and twogram counts at the same time, return
///     both.
/// -   Use sqlite to store the counts instead of in-memory.
///
/// References
/// -   https://rust-lang-nursery.github.io/rust-cookbook/concurrency/threads.html
pub fn handle_create_arpa_model(input_dir: &Path) -> Result<(), Box<dyn Error>> {
    println!("create_arpa_model entry");

    let en_dict: &[u8] = include_bytes!("3of6game.txt");
    let en_dict = load_dict(en_dict)?;

    println!("calculating ngrams...");
    let ngrams = calculate_ngrams_threaded(input_dir, &en_dict);
    for (token, count) in ngrams.unigram_counts.iter().take(100) {
        println!("onegram token {} count {}", token, count);
    }
    for (token, count) in ngrams.bigram_counts.iter().take(100) {
        let counts: Vec<(&String, &u64)> = count.iter().take(10).collect();
        println!("twogram token {} count {:?}", token, counts);
    }

    Ok(())
}

fn load_dict(dict_bytes: &[u8]) -> Result<HashSet<String>, Box<dyn Error>> {
    let dict = io::Cursor::new(dict_bytes);
    let dict = BufReader::new(dict);
    let dict = dict
        .lines()
        .map(|result| result.unwrap())
        .map(|line| line.nfc().collect::<String>())
        .map(|line| {
            String::from(line.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace()))
        })
        .collect();
    Ok(dict)
}

fn merge_ngrams_results(iter: impl Iterator<Item = NgramsResult>) -> NgramsResult {
    let mut total_unigrams = 0;
    let mut unigram_counts = HashMap::new();
    let mut bigram_counts = HashMap::new();
    for result in iter {
        total_unigrams += result.total_unigrams;

        for (word, count) in result.unigram_counts.into_iter() {
            let existing_count = unigram_counts.entry(word).or_insert(0);
            *existing_count += count;
        }

        for (word1, word2_counts) in result.bigram_counts.into_iter() {
            let existing_count_word1 = bigram_counts.entry(word1).or_insert_with(HashMap::new);
            for (word2, count) in word2_counts.into_iter() {
                let existing_count_word2 = existing_count_word1.entry(word2).or_insert(0);
                *existing_count_word2 += count;
            }
        }
    }
    NgramsResult {
        total_unigrams,
        unigram_counts,
        bigram_counts,
    }
}

fn calculate_ngrams_threaded(input_dir: &Path, dict: &HashSet<String>) -> NgramsResult {
    let mut pool = Pool::new(max(num_cpus::get() as u32 - 1, 1));
    let (tx, rx) = mpsc::channel();
    pool.scoped(|scope| {
        input_dir
            .read_dir()
            .unwrap()
            .map(|entry| entry.unwrap())
            .map(|entry| entry.path())
            .filter(|path| path.is_file())
            .filter(|path| {
                path.file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .contains("split")
            })
            .for_each(|input_file| {
                let tx = tx.clone();
                scope.execute(move || {
                    let result = calculate_ngrams(input_file.as_ref(), dict);
                    if result.is_ok() {
                        tx.send(result.unwrap()).unwrap();
                    } else {
                        panic!(
                            "failed to determine twogram counts for file {:?}: {:?}",
                            input_file, result
                        );
                    }
                });
            });
    });
    drop(tx);
    merge_ngrams_results(rx.iter())
}

#[derive(Debug)]
struct NgramsResult {
    /// Total number of unigrams in the corpus. The probability of a given unigram is the frequency
    /// of the unigram divided by this.
    total_unigrams: u64,

    /// Counts of specific unigrams. When you divide this by unigram_context_count you get the
    /// unigram probability.
    unigram_counts: HashMap<String, u64>,

    /// Counts of specific bigrams. The probability of a bigram (w_1, w_2) is the count of
    /// (w_1, w_2) divided by the count of w_1, which you can get from unigram_counts.
    bigram_counts: HashMap<String, HashMap<String, u64>>,
}

fn calculate_ngrams(
    input_file: &Path,
    dict: &HashSet<String>,
) -> Result<NgramsResult, std::io::Error> {
    let mut total_unigrams = 0;
    let mut unigram_counts = HashMap::new();
    let mut bigram_counts = HashMap::new();
    for line in LineIterator::new(input_file).unwrap() {
        let line_borrowed = line.borrow();
        let tokens: Vec<&str> = line_borrowed
            .split_whitespace()
            .map(|token| {
                token.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace())
            })
            .map(|token| {
                if dict.contains(token) {
                    token
                } else {
                    OUT_OF_VOCABULARY_WORD
                }
            })
            .collect();
        for (token1, token2) in tokens.iter().zip(tokens.iter().skip(1)) {
            total_unigrams += 1;

            let unigram_entry = unigram_counts.entry((*token1).to_string()).or_insert(0);
            *unigram_entry += 1;

            let bigram_entry = bigram_counts
                .entry((*token1).to_string())
                .or_insert_with(HashMap::new)
                .entry((*token2).to_string())
                .or_insert(0);
            *bigram_entry += 1;
        }

        // The iteration above missed the last token as a unigram so we tack it on here.
        if tokens.len() >= 2 {
            let last_token = tokens[tokens.len() - 1];
            total_unigrams += 1;
            let unigram_entry = unigram_counts.entry(last_token.to_string()).or_insert(0);
            *unigram_entry += 1;
        }
    }
    Ok(NgramsResult {
        total_unigrams,
        unigram_counts,
        bigram_counts,
    })
}

struct LineIterator {
    reader: Box<dyn BufRead>,
    buf: Rc<RefCell<String>>,
}

impl LineIterator {
    fn new(input_file: &Path) -> Result<LineIterator, Box<dyn Error>> {
        let file = File::open(input_file).unwrap();
        match input_file.extension().and_then(OsStr::to_str) {
            Some("gz") => {
                let file = GzDecoder::new(file);
                let file = BufReader::new(file);
                Ok(LineIterator {
                    reader: Box::new(file),
                    buf: Rc::new(RefCell::new(String::new())),
                })
            }
            _ => {
                let file = BufReader::new(file);
                Ok(LineIterator {
                    reader: Box::new(file),
                    buf: Rc::new(RefCell::new(String::new())),
                })
            }
        }
    }
}

impl Iterator for LineIterator {
    type Item = Rc<RefCell<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.buf.borrow_mut().clear();
        match self.reader.read_line(&mut self.buf.borrow_mut()) {
            Ok(0) => None,
            Ok(_) => Some(Rc::clone(&self.buf)),
            Err(_) => None,
        }
    }
}
