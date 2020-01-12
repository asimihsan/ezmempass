use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::{Compression, GzBuilder};
use scoped_threadpool::Pool;
use std::cmp::max;
use std::io;
use std::sync::mpsc;
use unicode_normalization::UnicodeNormalization;

/// If a word is not in the dictionry change it to this. This will never appear in the corpus
/// because we trim puncutation from the beginning and ends of words.
const OUT_OF_VOCABULARY_WORD: &str = "<unk>";

/// References
/// -   https://rust-lang-nursery.github.io/rust-cookbook/concurrency/threads.html
pub fn handle_create_arpa_model(
    input_dir: &Path,
    output_file: &String,
) -> Result<(), Box<dyn Error>> {
    println!("create_arpa_model entry");

    let en_dict: &[u8] = include_bytes!("3of6game.txt");
    let en_dict = load_dict(en_dict)?;
    println!("calculating ngrams...");
    let ngrams = calculate_ngrams_threaded(input_dir, &en_dict);
    ngrams.persist_to_file(input_dir, output_file)?;

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

impl NgramsResult {
    fn persist_to_file(
        &self,
        output_dir: &Path,
        output_file: &String,
    ) -> Result<(), Box<dyn Error>> {
        let gzip_output_filepath = NgramsResult::get_gzip_output_filename(output_dir, output_file);
        println!(
            "NgramsResult writing ARPA model to {:?}...",
            gzip_output_filepath
        );
        let mut output_file =
            NgramsResult::get_gzip_output_file(output_file, &gzip_output_filepath);
        let unigram_count = self.total_unigrams as f64;
        writeln!(&mut output_file, "\\data\\")?;
        writeln!(&mut output_file, "ngram 1 = {}", self.unigram_counts.len())?;
        writeln!(&mut output_file, "ngram 2 = {}", self.bigram_counts.len())?;
        writeln!(&mut output_file)?;
        writeln!(&mut output_file, "\\1-grams:")?;
        for (token, count) in self.unigram_counts.iter() {
            let prob = (*count as f64 / unigram_count).log10();
            writeln!(&mut output_file, "{:.4}\t{}", prob, token)?;
        }
        writeln!(&mut output_file)?;
        writeln!(&mut output_file, "\\2-grams:")?;
        for ((token1, token2), count) in self.bigram_counts.iter() {
            let token1_count = *self.unigram_counts.get(token1).unwrap();
            let bigram_prob = (*count as f64 / token1_count as f64).log10();
            writeln!(
                &mut output_file,
                "{:.4}\t{}\t{}",
                bigram_prob, token1, token2
            )?;
        }
        writeln!(&mut output_file)?;
        writeln!(&mut output_file, "\\end\\")?;

        Ok(())
    }

    fn get_gzip_output_filename(output_dir: &Path, output_file: &String) -> PathBuf {
        let output_file_path = Path::new(output_file);
        let output_file_extension = output_file_path
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .to_string();
        let output_file_path =
            output_file_path.with_extension(format!("{}.gz", output_file_extension));
        output_dir.join(output_file_path)
    }

    fn get_gzip_output_file(
        original_output_file: &String,
        gzip_output_filepath: &PathBuf,
    ) -> BufWriter<GzEncoder<File>> {
        let gzip_output_file = File::create(gzip_output_filepath).unwrap_or_else(|err| {
            panic!(
                "Could not create output file {:?} due to {:?}",
                gzip_output_filepath, err
            )
        });
        let gzip_output_file = GzBuilder::new()
            .filename(original_output_file.as_str())
            .write(gzip_output_file, Compression::best());
        BufWriter::new(gzip_output_file)
    }
}

fn merge_ngrams_results(iter: impl Iterator<Item = NgramsResult>) -> NgramsResult {
    let mut total_unigrams = 0;
    let mut unigram_counts = BTreeMap::new();
    let mut bigram_counts = BTreeMap::new();
    for result in iter {
        total_unigrams += result.total_unigrams;

        for (word, count) in result.unigram_counts.into_iter() {
            let existing_count = unigram_counts.entry(word).or_insert(0);
            *existing_count += count;
        }

        for ((word1, word2), count) in result.bigram_counts.into_iter() {
            let existing_count = bigram_counts.entry((word1, word2)).or_insert(0);
            *existing_count += count;
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
    unigram_counts: BTreeMap<String, u64>,

    /// Counts of specific bigrams. The probability of a bigram (w_1, w_2) is the count of
    /// (w_1, w_2) divided by the count of w_1, which you can get from unigram_counts.
    bigram_counts: BTreeMap<(String, String), u64>,
}

fn calculate_ngrams(
    input_file: &Path,
    dict: &HashSet<String>,
) -> Result<NgramsResult, std::io::Error> {
    let mut total_unigrams = 0;
    let mut unigram_counts = BTreeMap::new();
    let mut bigram_counts = BTreeMap::new();
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
                .entry(((*token1).to_string(), (*token2).to_string()))
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
