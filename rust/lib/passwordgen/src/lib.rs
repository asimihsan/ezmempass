use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use rand::Rng;

use graph::{shortest_path_multiple, Graph, SimpleInMemoryGraph};
use integer_coding::DeltaDecoder;

const ONE_GRAM_INCREMENTAL_COST: i64 = 1_000_000;

pub struct GeneratePassphraseOutput {
    pub prefixes: Vec<String>,
    pub passphrase: Vec<String>,
    pub cost: i64,
}

pub fn generate_passphrase(passphrase_size: i32) -> io::Result<(GeneratePassphraseOutput)> {
    let wordlist_bytes = include_bytes!("wordlist_enwiki.txt");
    let wordlist_bytes_pointer = &wordlist_bytes[..];
    let mut data_file_reader = BufReader::new(wordlist_bytes_pointer);

    let mut number_of_words = String::new();
    data_file_reader.read_line(&mut number_of_words)?;
    let number_of_words = number_of_words.trim();
    let number_of_words = u32::from_str(&number_of_words).unwrap();

    let mut words: Vec<String> = Vec::with_capacity(number_of_words as usize);
    words.push(String::new());
    for _ in 0..number_of_words {
        let mut word = String::new();
        data_file_reader.read_line(&mut word)?;
        let word = word.trim();
        words.push(word.to_owned());
    }
    let mut word_to_edges_encoded: Vec<Vec<u8>> = Vec::with_capacity(number_of_words as usize);
    word_to_edges_encoded.push(Vec::new());
    for _i in 0..number_of_words {
        let mut edges_encoded_bytes: Vec<u8> = Vec::new();
        data_file_reader.read_until(b'\n', &mut edges_encoded_bytes)?;
        edges_encoded_bytes.remove(edges_encoded_bytes.len() - 1);
        word_to_edges_encoded.push(edges_encoded_bytes);
    }
    let prefix_to_words = get_prefix_to_words(&words);
    let prefixes: Vec<&String> = prefix_to_words.keys().collect();
    let word_to_index = get_word_to_index(&words);

    let random_prefixes = get_random_prefixes(&prefixes, passphrase_size);
    let (random_passphrase, cost) = get_random_passphrase_graph(
        &random_prefixes,
        &prefix_to_words,
        &words,
        &word_to_index,
        &word_to_edges_encoded,
    );

    Ok(GeneratePassphraseOutput {
        prefixes: random_prefixes,
        passphrase: random_passphrase,
        cost,
    })
}

fn get_random_passphrase_graph(
    prefixes: &[String],
    prefix_to_words: &HashMap<String, Vec<String>>,
    words: &[String],
    word_to_index: &HashMap<String, usize>,
    word_to_edges_encoded: &Vec<Vec<u8>>,
) -> (Vec<String>, i64) {
    let mut g = SimpleInMemoryGraph::new();
    let mut start = true;
    for (prefix1, prefix2) in prefixes.iter().zip(prefixes.iter().skip(1)) {
        let prefix1_1gram_words = prefix_to_words.get(prefix1).unwrap();
        let prefix2_1gram_words = prefix_to_words.get(prefix2).unwrap();
        for prefix1_1gram_word in prefix1_1gram_words {
            for prefix2_2gram_word in &get_next_words(
                prefix1_1gram_word,
                prefix2,
                words,
                word_to_index,
                word_to_edges_encoded,
            ) {
                let weight: i64 = if start {
                    -(word_to_index[prefix1_1gram_word] as i64
                        + word_to_index[prefix2_2gram_word] as i64)
                } else {
                    -(word_to_index[prefix2_2gram_word] as i64)
                };
                g.add_edge(prefix1_1gram_word, prefix2_2gram_word, weight);
                //                println!(
                //                    "2gram word1 {} word2 {} weight {}",
                //                    prefix1_1gram_word, prefix2_2gram_word, weight
                //                );
            }
            for prefix2_1gram_word in prefix2_1gram_words {
                if g.get_edge_weight(prefix1_1gram_word, prefix2_1gram_word)
                    .is_none()
                {
                    let weight: i64 = if start {
                        -(word_to_index[prefix1_1gram_word] as i64
                            + word_to_index[prefix2_1gram_word] as i64
                            + ONE_GRAM_INCREMENTAL_COST)
                    } else {
                        -(word_to_index[prefix2_1gram_word] as i64 + ONE_GRAM_INCREMENTAL_COST)
                    };
                    //                    println!(
                    //                        "1gram word1 {} word2 {} weight {}",
                    //                        prefix1_1gram_word, prefix2_1gram_word, weight
                    //                    );
                    g.add_edge(prefix1_1gram_word, prefix2_1gram_word, weight);
                }
            }
            start = false;
        }
    }
    let first_prefix_words: Vec<&str> = prefix_to_words
        .get(prefixes.first().unwrap())
        .unwrap()
        .iter()
        .map(AsRef::as_ref)
        .collect();
    let last_prefix_words: Vec<&str> = prefix_to_words
        .get(prefixes.last().unwrap())
        .unwrap()
        .iter()
        .map(AsRef::as_ref)
        .collect();
    //    println!("first_prefix_words: {:?}", first_prefix_words);
    //    println!("last_prefix_words: {:?}", last_prefix_words);
    let (shortest_path, cost) =
        shortest_path_multiple(&g, first_prefix_words, last_prefix_words).unwrap();
    (shortest_path, cost)
}

fn get_next_words(
    word: &str,
    prefix_filter: &str,
    words: &[String],
    word_to_index: &HashMap<String, usize>,
    word_to_edges_encoded: &Vec<Vec<u8>>,
) -> Vec<String> {
    let index = word_to_index[word];
    let edges_encoded_bytes = &word_to_edges_encoded[index];
    let decoder = DeltaDecoder::new(edges_encoded_bytes);
    decoder
        .decode()
        .iter()
        .map(|edge: &u32| &words[*edge as usize])
        .filter(|word: &&String| word.starts_with(prefix_filter))
        .map(|word: &String| word.to_owned())
        .collect()
}

fn get_random_prefixes(prefixes: &Vec<&String>, length: i32) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(length as usize);
    let max = prefixes.len();
    for _ in 0..length {
        let index: usize = rand::thread_rng().gen_range(0, max);
        result.push(prefixes[index].parse().unwrap());
    }
    result
}

fn get_word_to_index(words: &Vec<String>) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::with_capacity(words.len());
    for (i, word) in words.iter().enumerate().skip(1) {
        result.insert(word.parse().unwrap(), i);
    }
    result
}

fn get_prefix_to_words(words: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut prefixes: HashMap<String, Vec<String>> = HashMap::new();
    for word in words.iter().skip(1) {
        let prefix = word[0..3].to_string();
        prefixes
            .entry(prefix)
            .or_insert_with(Vec::new)
            .push(word.to_owned());
    }
    prefixes
}