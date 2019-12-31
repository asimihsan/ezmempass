use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::Rc;

use flate2::read::GzDecoder;
use std::cmp::{max, Reverse};
use std::io;
use std::sync::mpsc;
use threadpool::ThreadPool;
use unicode_normalization::UnicodeNormalization;

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
    let top_onegram_count = 100_000;
    let number_of_prefixes = 1024;
    const PREFIX_SIZE: usize = 3;

    let en_dict = load_dict(en_dict)?;
    let onegrams = calculate_onegrams_threaded(input_dir);
    let twograms = calculate_twograms_threaded(input_dir);
    Ok(())
}

fn load_dict(dict_bytes: &[u8]) -> Result<HashSet<String>, Box<dyn Error>> {
    let dict = io::Cursor::new(dict_bytes);
    let dict = BufReader::new(dict);
    let dict: HashSet<String> = dict
        .lines()
        .map(|result| result.unwrap())
        .map(|line| line.nfc().collect::<String>())
        .collect();
    Ok(dict)
}

fn calculate_onegrams_threaded(input_dir: &Path) -> OnegramsResult {
    let pool = ThreadPool::new(max(num_cpus::get() - 1, 1));
    let (tx, rx) = mpsc::channel();
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
            pool.execute(move || {
                let result = calculate_onegrams(input_file.as_ref());
                if result.is_ok() {
                    tx.send(result.unwrap()).unwrap();
                } else {
                    panic!(
                        "failed to determine counts for file {:?}: {:?}",
                        input_file, result
                    );
                }
            });
        });
    drop(tx);
    merge_onegrams_results(rx.iter())
}

fn merge_onegrams_results(iter: impl Iterator<Item = OnegramsResult>) -> OnegramsResult {
    let mut total = 0;
    let mut counts = HashMap::new();
    for result in iter {
        total += result.total;
        for (word, count) in result.counts.into_iter() {
            let existing_count = counts.entry(word).or_insert(0);
            *existing_count += count;
        }
    }
    OnegramsResult { total, counts }
}

#[derive(Debug)]
struct OnegramsResult {
    total: u64,
    counts: HashMap<String, u64>,
}

fn calculate_onegrams(input_file: &Path) -> Result<OnegramsResult, Box<dyn Error>> {
    let mut total = 0;
    let mut counts = HashMap::new();
    for line in LineIterator::new(input_file).unwrap() {
        let line_borrowed = line.borrow();
        let line_trimmed = line_borrowed.trim_end();
        for token in line_trimmed
            .split_whitespace()
            .map(|token| {
                token.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace())
            })
            .filter(|token| !token.is_empty())
        //            .filter(|token| dict.contains(*token))
        {
            total += 1;
            let counter = counts.entry(token.to_string()).or_insert(0);
            *counter += 1;
        }
    }

    Ok(OnegramsResult { total, counts })
}

fn merge_twograms_results(iter: impl Iterator<Item = TwogramsResult>) -> TwogramsResult {
    let mut total = 0;
    let mut counts = HashMap::new();
    for result in iter {
        total += result.total;
        for (word1, word2_counts) in result.counts.into_iter() {
            let existing_count_word1 = counts.entry(word1).or_insert_with(HashMap::new);
            for (word2, count) in word2_counts.into_iter() {
                let existing_count_word2 = existing_count_word1.entry(word2).or_insert(0);
                *existing_count_word2 += count;
            }
        }
    }
    TwogramsResult { total, counts }
}

fn calculate_twograms_threaded(input_dir: &Path) -> TwogramsResult {
    let pool = ThreadPool::new(max(num_cpus::get() - 1, 1));
    let (tx, rx) = mpsc::channel();
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
            pool.execute(move || {
                let result = calculate_twograms(input_file.as_ref());
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
    drop(tx);
    merge_twograms_results(rx.iter())
}

#[derive(Debug)]
struct TwogramsResult {
    total: u64,
    counts: HashMap<String, HashMap<String, u32>>,
}

fn calculate_twograms(input_file: &Path) -> Result<TwogramsResult, std::io::Error> {
    let mut total = 0;
    let mut counts = HashMap::new();
    for line in LineIterator::new(input_file).unwrap() {
        let line_borrowed = line.borrow();
        let token1_iter = line_borrowed.split_whitespace().map(|token| token.trim());
        let token2_iter = line_borrowed.split_whitespace().map(|token| token.trim());
        token1_iter
            .zip(token2_iter.skip(1))
            .map(|(token1, token2)| {
                (
                    token1.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace()),
                    token2.trim_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace()),
                )
            })
            .filter(|(token1, token2)| !token1.is_empty() && !token2.is_empty())
            //            .filter(|(token1, token2)| {
            //                top_onegrams_lookup.contains(*token1) && top_onegrams_lookup.contains(*token2)
            //            })
            .for_each(|(token1, token2)| {
                total += 1;
                let entry = counts
                    .entry(token1.to_owned())
                    .or_insert_with(HashMap::new)
                    .entry(token2.to_owned())
                    .or_insert(0);
                *entry += 1;
            });
    }
    Ok(TwogramsResult { total, counts })
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
