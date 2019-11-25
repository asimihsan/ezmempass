use bimap::BiHashMap;
use std::collections::HashMap;
use std::io;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

use rand::Rng;

use graph::{shortest_path_multiple, Graph, SimpleInMemoryGraph};
use integer_coding::DeltaDecoder;

const ONE_GRAM_INCREMENTAL_COST: i64 = 1_000_000;

pub struct GeneratePassphraseInput {
    pub passphrase_length: i32,
    pub add_capital_letter: bool,
    pub add_digit: bool,
    pub add_symbol: bool,
}

#[derive(Debug)]
pub struct GeneratePassphraseOutput {
    pub password: String,
    pub passphrase: String,
    pub prefixes: Vec<String>,
    pub words: Vec<String>,
    pub cost: i64,
}

const WORDLIST_ENWIKI: &'static [u8] = include_bytes!("wordlist_enwiki.txt");

pub fn generate_passphrase(
    input: &GeneratePassphraseInput,
) -> io::Result<(GeneratePassphraseOutput)> {
    let mut rng = rand::thread_rng();
    generate_passphrase_internal(input, &mut rng)
}

pub fn generate_passphrase_internal(
    input: &GeneratePassphraseInput,
    rng: &mut impl Rng,
) -> io::Result<(GeneratePassphraseOutput)> {
    let mut data_file_reader = BufReader::new(WORDLIST_ENWIKI);

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

    let random_prefixes = get_random_prefixes(&prefixes, input.passphrase_length, rng);
    let (random_passphrase, cost) = get_random_passphrase_graph(
        &random_prefixes,
        &prefix_to_words,
        &words,
        &word_to_index,
        &word_to_edges_encoded,
        rng,
    );
    let (password, passphrase) = convert_prefixes_to_password_and_passphrase(
        &random_prefixes,
        &random_passphrase,
        input.add_digit,
        input.add_capital_letter,
        input.add_symbol,
        rng,
    );
    Ok(GeneratePassphraseOutput {
        password: password,
        passphrase: passphrase,
        prefixes: random_prefixes,
        words: random_passphrase,
        cost,
    })
}

fn convert_prefixes_to_password_and_passphrase(
    random_prefixes: &Vec<String>,
    random_passphrase: &Vec<String>,
    add_digit: bool,
    add_capital_letter: bool,
    add_symbol: bool,
    rng: &mut impl Rng,
) -> (String, String) {
    let random_digits: Vec<&str> = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let random_symbols: Vec<&str> = vec!["@", "#", "$", "&", "%", "!"];

    let mut prefixes_result = String::new();
    let mut passphrase_result = String::new();
    let number_of_prefixes = random_prefixes.len();
    let random_elem = rng.gen_range(0, number_of_prefixes);
    let insert_before_elem = rng.gen_bool(0.5);
    let random_digit = random_digits[rng.gen_range(0, random_digits.len())];
    let random_symbol = random_symbols[rng.gen_range(0, random_symbols.len())];
    let mut random_insertion = String::with_capacity(2);
    if rng.gen_bool(0.5) {
        if add_digit {
            random_insertion.push_str(random_digit);
        }
        if add_symbol {
            random_insertion.push_str(random_symbol);
        }
    } else {
        if add_symbol {
            random_insertion.push_str(random_symbol);
        }
        if add_digit {
            random_insertion.push_str(random_digit);
        }
    }
    for i in 0..random_prefixes.len() {
        if i != random_elem {
            prefixes_result.push_str(&random_prefixes[i]);
            passphrase_result.push_str(&random_passphrase[i]);
            passphrase_result.push_str(" ");
            continue;
        }
        if insert_before_elem && random_insertion.len() > 0 {
            prefixes_result.push_str(&random_insertion);
            passphrase_result.push_str(&random_insertion);
            passphrase_result.push_str(" ");
        }
        if add_capital_letter {
            prefixes_result.push_str(&random_prefixes[i].to_uppercase());
            passphrase_result.push_str(&random_passphrase[i].to_uppercase());
            passphrase_result.push_str(" ");
        } else {
            prefixes_result.push_str(&random_prefixes[i]);
            passphrase_result.push_str(&random_passphrase[i]);
            passphrase_result.push_str(" ");
        }
        if !insert_before_elem && random_insertion.len() > 0 {
            prefixes_result.push_str(&random_insertion);
            passphrase_result.push_str(&random_insertion);
            passphrase_result.push_str(" ");
        }
    }
    (prefixes_result, passphrase_result.trim_end().to_string())
}

#[derive(Eq, Hash, PartialEq)]
struct PrefixLevelAndWord {
    pub level: u32,
    pub word_index: u32,
}

type GraphIdentifier = u32;

fn update_prefix_level_bimap(
    prefix_level_bimap: &mut BiHashMap<PrefixLevelAndWord, GraphIdentifier>,
    current_graph_id: u32,
    word_to_index: &HashMap<String, u32>,
    level: u32,
    word: &str,
) -> (u32, u32) {
    let mut new_graph_id = current_graph_id;
    let word_id: u32;
    let p = PrefixLevelAndWord {
        level,
        word_index: word_to_index[word],
    };
    if !prefix_level_bimap.contains_left(&p) {
        prefix_level_bimap.insert(p, new_graph_id);
        word_id = new_graph_id;
        new_graph_id += 1;
    } else {
        word_id = *prefix_level_bimap.get_by_left(&p).unwrap();
    }
    (new_graph_id, word_id)
}

/// TODO wow this is remarkably ugly.
fn get_random_passphrase_graph(
    prefixes: &Vec<String>,
    prefix_to_words: &HashMap<String, Vec<String>>,
    words: &[String],
    word_to_index: &HashMap<String, u32>,
    word_to_edges_encoded: &Vec<Vec<u8>>,
    rng: &mut impl Rng,
) -> (Vec<String>, i64) {
    let mut g = SimpleInMemoryGraph::new();
    let mut start = true;
    let mut prefix_level_bimap = BiHashMap::<PrefixLevelAndWord, GraphIdentifier>::new();
    let mut current_graph_id: u32 = 1;
    let prefixes_and_levels: Vec<(u32, &String)> = prefixes
        .iter()
        .enumerate()
        .map(|(index, prefix)| (index as u32, prefix))
        .collect();
    let last_prefix_level = (prefixes.len() - 1) as u32;
    for ((prefix1_level, prefix1), (prefix2_level, prefix2)) in prefixes_and_levels
        .iter()
        .zip(prefixes_and_levels.iter().skip(1))
    {
        let prefix1_1gram_words = prefix_to_words.get(*prefix1).unwrap();
        let prefix2_1gram_words = prefix_to_words.get(*prefix2).unwrap();
        for prefix1_1gram_word in prefix1_1gram_words {
            // ----------------------------------------------------------------
            //  Add prefix1 1gram -> prefix2 2gram edges. These have lower
            //  weights because we prefer them.
            // ----------------------------------------------------------------
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
                let (new_graph_id, word1_index) = update_prefix_level_bimap(
                    &mut prefix_level_bimap,
                    current_graph_id,
                    word_to_index,
                    *prefix1_level,
                    prefix1_1gram_word,
                );
                current_graph_id = new_graph_id;
                let (new_graph_id, word2_index) = update_prefix_level_bimap(
                    &mut prefix_level_bimap,
                    current_graph_id,
                    word_to_index,
                    *prefix2_level,
                    prefix2_2gram_word,
                );
                current_graph_id = new_graph_id;
                g.add_edge(word1_index, word2_index, weight);
            }
            // ----------------------------------------------------------------

            // ----------------------------------------------------------------
            //  Add prefix1 1gram -> prefix2 1gram edges.
            // ----------------------------------------------------------------
            for prefix2_1gram_word in prefix2_1gram_words {
                let (new_graph_id, word1_index) = update_prefix_level_bimap(
                    &mut prefix_level_bimap,
                    current_graph_id,
                    word_to_index,
                    *prefix1_level,
                    prefix1_1gram_word,
                );
                current_graph_id = new_graph_id;
                let (new_graph_id, word2_index) = update_prefix_level_bimap(
                    &mut prefix_level_bimap,
                    current_graph_id,
                    word_to_index,
                    *prefix2_level,
                    prefix2_1gram_word,
                );
                current_graph_id = new_graph_id;

                if g.get_edge_weight(word1_index, word2_index).is_none() {
                    let weight: i64 = if start {
                        -(word_to_index[prefix1_1gram_word] as i64
                            + word_to_index[prefix2_1gram_word] as i64
                            + ONE_GRAM_INCREMENTAL_COST)
                    } else {
                        -(word_to_index[prefix2_1gram_word] as i64 + ONE_GRAM_INCREMENTAL_COST)
                    };
                    g.add_edge(word1_index, word2_index, weight);
                }
            }
            // ----------------------------------------------------------------
            start = false;
        }
    }
    let first_prefix_words: Vec<u32> = prefix_to_words
        .get(prefixes.first().unwrap())
        .unwrap()
        .iter()
        .map(|word| PrefixLevelAndWord {
            level: 0,
            word_index: word_to_index[word],
        })
        .map(|p| *prefix_level_bimap.get_by_left(&p).unwrap())
        .collect();
    let last_prefix_words: Vec<u32> = prefix_to_words
        .get(prefixes.last().unwrap())
        .unwrap()
        .iter()
        .map(|word| PrefixLevelAndWord {
            level: last_prefix_level,
            word_index: word_to_index[word],
        })
        .map(|p| *prefix_level_bimap.get_by_left(&p).unwrap())
        .collect();
    let (shortest_path, cost) =
        shortest_path_multiple(&g, first_prefix_words, last_prefix_words, rng).unwrap();
    let shortest_path = shortest_path
        .iter()
        .map(|graph_index| {
            prefix_level_bimap
                .get_by_right(graph_index)
                .unwrap()
                .word_index
        })
        .map(|word_index| words[word_index as usize].clone())
        .collect();
    (shortest_path, cost)
}

fn get_next_words(
    word: &str,
    prefix_filter: &str,
    words: &[String],
    word_to_index: &HashMap<String, u32>,
    word_to_edges_encoded: &Vec<Vec<u8>>,
) -> Vec<String> {
    let index = word_to_index[word] as usize;
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

fn get_random_prefixes(prefixes: &Vec<&String>, length: i32, rng: &mut impl Rng) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(length as usize);
    let max = prefixes.len();
    for _ in 0..length {
        let index: usize = rng.gen_range(0, max);
        result.push(prefixes[index].parse().unwrap());
    }
    result
}

fn get_word_to_index(words: &Vec<String>) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::with_capacity(words.len());
    for (i, word) in words.iter().enumerate().skip(1) {
        result.insert(word.parse().unwrap(), i as u32);
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
