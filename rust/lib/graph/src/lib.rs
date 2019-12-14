/*
TODO create a Graph impl backed by the wordlist data file, without needing to create
edges
*/

use fast_paths::{InputGraph, Params};
use rand::Rng;
use std::cmp::min;
use std::collections::HashSet;

/// Simple completely in-memory modifiable directed graph that supports edge weights.
pub struct SimpleInMemoryGraph {
    graph: InputGraph,
}

impl Default for SimpleInMemoryGraph {
    fn default() -> SimpleInMemoryGraph {
        SimpleInMemoryGraph {
            graph: InputGraph::new(),
        }
    }
}

impl SimpleInMemoryGraph {
    pub fn add_edge(&mut self, node1: u32, node2: u32, weight: i64) {
        self.graph
            .add_edge(node1 as usize, node2 as usize, weight as usize);
    }

    pub fn freeze(&mut self) {
        self.graph.freeze();
    }
}

/// Given a vector with [0..max-1] elements inside it you want a random index that is weighted to
/// prefer elements at the front of the list. This is because our word lists are weighted most
/// common to least common.
fn get_weighted_random_index(max: usize, rng: &mut impl Rng) -> usize {
    let mut lo: i32 = 0;
    let mut hi: i32 = (max - 1) as i32;
    while lo <= hi {
        let mid: i32 = lo + (hi - lo) / 2;
        let random_value: f64 = rng.gen();
        if random_value < 0.8 {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    if hi < 1 {
        hi = 1;
    }
    rng.gen_range(0, hi) as usize
}

/// Returns shortest path from multiple start nodes to multiple goal nodes and the associated cost.
pub fn shortest_path_multiple(
    graph: &mut SimpleInMemoryGraph,
    starts: Vec<u32>,
    goals: Vec<u32>,
    rng: &mut impl Rng,
) -> Option<(Vec<u32>, i64)> {
    graph.freeze();
    let fast_graph = fast_paths::prepare_with_params(
        &graph.graph,
        &Params {
            hierarchy_depth_factor: 0.05,
            edge_quotient_factor: 0.05,
        },
    );
    let mut path_calculator = fast_paths::create_calculator(&fast_graph);

    let mut current_shortest_path_cost: i64 = i64::max_value();
    let mut current_shortest_path: Vec<u32> = Vec::new();
    let mut found_any_path: bool = false;

    let mut i = 0;
    const MAX_ITERS: i32 = 100;
    let starts_len = starts.len();
    let goals_len = goals.len();
    if starts_len == 0 || goals_len == 0 {
        return None;
    }
    let mut seen_starts_goals: HashSet<(u32, u32)> =
        HashSet::with_capacity(min(starts_len * goals_len, 100));
    while i <= MAX_ITERS {
        i += 1;
        let start = starts[get_weighted_random_index(starts_len, rng)];
        let goal = goals[get_weighted_random_index(goals_len, rng)];
        let key = (start, goal);
        if seen_starts_goals.contains(&key) {
            continue;
        }
        seen_starts_goals.insert(key);

        match path_calculator.calc_path(&fast_graph, start as usize, goal as usize) {
            None => continue,
            Some(p) => {
                found_any_path = true;
                let weight = p.get_weight() as i64;
                if weight < current_shortest_path_cost {
                    current_shortest_path_cost = weight;
                    let path: Vec<u32> = p.get_nodes().iter().map(|node| *node as u32).collect();
                    current_shortest_path = path;
                }
            }
        }
    }
    if !found_any_path {
        None
    } else {
        Some((current_shortest_path, current_shortest_path_cost))
    }
}
