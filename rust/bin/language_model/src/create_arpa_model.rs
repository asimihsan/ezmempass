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
/// References
/// -   https://rust-lang-nursery.github.io/rust-cookbook/concurrency/threads.html
pub fn handle_create_arpa_model(input_dir: &Path) -> Result<(), Box<dyn Error>> {
    println!("create_arpa_model entry");

    let en_dict: &[u8] = include_bytes!("3of6game.txt");
    let top_onegram_count = 100_000;
    let number_of_prefixes = 1024;
    const PREFIX_SIZE: usize = 3;

    let en_dict = load_dict(en_dict)?;
    let onegrams = calculate_onegrams_threaded(input_dir, &en_dict);

    // ------------------------------------------------------------------------
    //  Take top-most onegrams that match word pattern.
    // ------------------------------------------------------------------------
    let mut top_onegrams: Vec<(&String, &u64)> = onegrams
        .counts
        .iter()
        .filter(|(onegram, count)| onegram.len() >= PREFIX_SIZE)
        .collect();
    top_onegrams.sort_by_key(|(_onegram, count)| Reverse(*count));
    let top_onegrams: Vec<&String> = top_onegrams
        .into_iter()
        .map(|(onegram, _count)| onegram)
        .take(top_onegram_count)
        .collect();
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    // Find the top "number_of_prefixes" prefixes of all top onegrams.
    // ------------------------------------------------------------------------
    let mut prefixes_count: HashMap<String, u32> = HashMap::new();
    top_onegrams
        .iter()
        .map(|word| word.chars().take(PREFIX_SIZE).collect())
        .for_each(|prefix| {
            *prefixes_count.entry(prefix).or_insert(0) += 1;
        });
    let mut prefixes_sorted_count: Vec<(String, u32)> = prefixes_count.into_iter().collect();
    prefixes_sorted_count.sort_by_key(|(_word, count)| Reverse(*count));
    let prefixes_sorted_count: Vec<String> = prefixes_sorted_count
        .into_iter()
        .take(number_of_prefixes)
        .map(|(prefix, _count)| prefix)
        .collect();
    let mut prefix_lookup: HashSet<String> = HashSet::with_capacity(prefixes_sorted_count.len());
    prefixes_sorted_count.into_iter().for_each(|prefix| {
        prefix_lookup.insert(prefix);
    });
    if prefix_lookup.len() != number_of_prefixes {
        panic!(
            "not enough prefixes found, only {} need {}",
            prefix_lookup.len(),
            number_of_prefixes
        );
    }
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    // Re-filter top onegrams to only include those that start with a wanted prefix.
    // ------------------------------------------------------------------------
    let top_onegrams: Vec<&String> = top_onegrams
        .into_iter()
        .filter(|onegram| {
            let actual_prefix: String = onegram.chars().take(PREFIX_SIZE).collect::<String>();
            prefix_lookup.contains(&actual_prefix)
        })
        .collect();
    println!(
        "top_onegrams.len() that match prefixes: {}",
        top_onegrams.len()
    );
    // ------------------------------------------------------------------------

    // ------------------------------------------------------------------------
    //  Finally now that we have the final onegrams sorted by frequency we can do a second pass on
    //  the wiki dump and look for twograms instead that both are in this top onegram list.
    // ------------------------------------------------------------------------

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

fn calculate_onegrams_threaded(input_dir: &Path, dict: &HashSet<String>) -> OnegramsResult {
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
            let dict = dict.clone();
            pool.execute(move || {
                let result = calculate_onegrams(input_file.as_ref(), &dict);
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

fn calculate_onegrams(
    input_file: &Path,
    dict: &HashSet<String>,
) -> Result<OnegramsResult, Box<dyn Error>> {
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
            .filter(|token| dict.contains(*token))
        {
            let counter = counts.entry(token.to_string()).or_insert(0);
            *counter += 1;
            total += 1;
        }
    }

    Ok(OnegramsResult { total, counts })
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
