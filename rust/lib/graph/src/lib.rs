/*
TODO create a Graph impl backed by the wordlist data file, without needing to create
edges
*/

use priority_queue::PriorityQueue;
use rand::Rng;
use std::cmp::min;
use std::collections::{hash_set, HashMap, HashSet};

/// Graph trait that allows algorithms to be applied.
///
/// TODO for simplicity I avoided generics and we can only store string slices. Make this generic.
pub trait Graph {
    fn new() -> Self;
    fn get_neighbors(&self, node: u32) -> Option<hash_set::Iter<u32>>;
    fn get_edge_weight(&self, node1: u32, node2: u32) -> Option<i64>;
}

/// Simple completely in-memory modifiable directed graph that supports edge weights.
pub struct SimpleInMemoryGraph {
    edges: HashMap<u32, HashSet<u32>>,
    weights: HashMap<(u32, u32), i64>,
}

impl Graph for SimpleInMemoryGraph {
    fn new() -> Self {
        SimpleInMemoryGraph {
            edges: HashMap::new(),
            weights: HashMap::new(),
        }
    }

    fn get_neighbors(&self, node: u32) -> Option<hash_set::Iter<u32>> {
        match self.edges.get(&node) {
            None => None,
            Some(neighbors) => Some(neighbors.iter()),
        }
    }

    fn get_edge_weight(&self, node1: u32, node2: u32) -> Option<i64> {
        if let Some(i) = self.weights.get(&(node1, node2)) {
            Some(*i)
        } else {
            None
        }
    }
}

impl SimpleInMemoryGraph {
    pub fn add_edge(&mut self, node1: u32, node2: u32, weight: i64) {
        let existing_edges = self.edges.get_mut(&node1);
        if existing_edges.is_none() {
            let mut new_destinations = HashSet::new();
            new_destinations.insert(node2);
            self.edges.insert(node1, new_destinations);
        } else {
            existing_edges.unwrap().insert(node2);
        }
        self.weights.insert((node1, node2), weight);
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
    graph: &impl Graph,
    starts: Vec<u32>,
    goals: Vec<u32>,
    rng: &mut impl Rng,
) -> Option<(Vec<u32>, i64)> {
    let mut current_shortest_path_cost: i64 = i64::min_value();
    let mut current_shortest_path: Vec<u32> = Vec::new();
    let mut found_any_path: bool = false;

    // THIS IS WRONG!!! This is not a Cartesian product, just a zip. TODO add a test that repros this
    // being broken.
    /*
    for (start, goal) in starts.iter().zip(goals.iter()) {
        match shortest_path(graph, start, goal) {
            None => continue,
            Some((path, cost)) => {
                found_any_path = true;
                println!(
                    "shortest_path_multiple start {}, goal {}, cost {}, current_shortest_path_cost {}, path: {:?}",
                    start, goal, cost, current_shortest_path_cost, path
                );
                if cost < current_shortest_path_cost {
                    current_shortest_path_cost = cost;
                    current_shortest_path = path;
                }
            }
        }
    }
    */
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

        match shortest_path(graph, start, goal) {
            None => continue,
            Some((path, cost)) => {
                found_any_path = true;
                if cost > current_shortest_path_cost {
                    current_shortest_path_cost = cost;
                    current_shortest_path = path;
                }
            }
        }
        //        if i % 100 == 0 {
        //            println!(
        //                "current best path {:?} cost {}",
        //                current_shortest_path, current_shortest_path_cost
        //            );
        //        }
    }
    if !found_any_path {
        None
    } else {
        Some((current_shortest_path, current_shortest_path_cost))
    }
}

/// Returns shortest path from a single start node to a single goal node.
/// TODO fix borrow checking and prevent needing so many string copies
pub fn shortest_path(graph: &impl Graph, start: u32, goal: u32) -> Option<(Vec<u32>, i64)> {
    let mut frontier: PriorityQueue<u32, i64, _> = PriorityQueue::new();
    let mut came_from: HashMap<u32, u32> = HashMap::new();
    let mut cost_so_far: HashMap<u32, i64> = HashMap::new();
    frontier.push(start, 0);
    cost_so_far.insert(start, 0);
    let mut found_goal: bool = false;

    while !frontier.is_empty() {
        let (current, _current_priority) = frontier.pop().unwrap();
        if current == goal {
            found_goal = true;
            break;
        }
        let current_cost = *cost_so_far.get_mut(&current).unwrap();
        let current_neighbors = graph.get_neighbors(current);
        if current_neighbors.is_none() {
            continue;
        }
        for next in current_neighbors.unwrap() {
            let new_cost = current_cost + graph.get_edge_weight(current, *next).unwrap();
            let existing_next_cost = cost_so_far.get(&next);
            if existing_next_cost.is_none() || new_cost > *existing_next_cost.unwrap() {
                cost_so_far.insert(*next, new_cost);
                frontier.push(*next, new_cost);
                came_from.insert(*next, current);
            }
        }
    }

    if !found_goal {
        None
    } else {
        let mut result = Vec::new();
        let mut current_node = goal;
        while current_node != start {
            result.push(current_node);
            current_node = *came_from.get(&current_node).unwrap();
        }
        result.push(current_node);
        result.reverse();
        let total_cost = *cost_so_far.get(&goal).unwrap();
        Some((result, total_cost))
    }
}

#[cfg(test)]
mod shortest_path_simple_in_memory_graph_tests {
    use super::*;
    use rand::rngs::mock;

    /// A -> B -> C -> D -> E, all edges weight of 1.
    /// A, E shortest path should return A, B, C, D, E
    #[test]
    fn test_basic_single() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge(1, 2, 1);
        g.add_edge(2, 3, 1);
        g.add_edge(3, 4, 1);
        g.add_edge(4, 5, 1);

        // == when ==
        let shortest_path = shortest_path(&g, 1, 5);

        // == then ==
        assert_eq!(shortest_path.is_some(), true);
        assert_eq!(shortest_path.unwrap(), (vec![1, 2, 3, 4, 5], 4));
    }

    /// A -> (1) -> B -> (5) -> E.
    /// A -> (3) -> D -> (1) -> E
    /// A, E shortest path should return A, D, E.
    #[test]
    fn test_basic_two_paths() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge(1, 2, 1);
        g.add_edge(2, 5, 5);
        g.add_edge(1, 4, 3);
        g.add_edge(4, 5, 1);

        // == when ==
        let shortest_path = shortest_path(&g, 1, 5);

        // == then ==
        assert_eq!(shortest_path.is_some(), true);
        assert_eq!(shortest_path.unwrap(), (vec![1, 4, 5], 4));
    }

    /// A -> (1) -> B -> (5) -> C.
    /// D -> (3) -> E
    /// A, E shortest path should return None.
    #[test]
    fn test_no_path() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge(1, 2, 1);
        g.add_edge(2, 3, 5);
        g.add_edge(4, 5, 3);

        // == when ==
        let shortest_path = shortest_path(&g, 1, 5);

        // == then ==
        assert_eq!(shortest_path.is_none(), true);
    }

    /// A -> (-1) -> B -> (-5) -> C
    /// D -> (-1) -> E -> (-1) -> F
    /// A, D to C, F shortest path multiple should return D, E, F
    /// TODO this is now probabilistic, refactor to use seed / known values
    #[test]
    #[ignore]
    fn test_shortest_path_multiple_basic() {
        // == given ==
        let mut rng = mock::StepRng::new(0, 1);
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge(1, 2, -1);
        g.add_edge(2, 3, -5);
        g.add_edge(4, 5, -1);
        g.add_edge(5, 6, -1);

        // == when ==
        let shortest_path = shortest_path_multiple(&g, vec![1, 4], vec![3, 6], &mut rng);

        // == then ==
        assert_eq!(shortest_path.is_some(), true);
        assert_eq!(shortest_path.unwrap(), (vec![4, 5, 6], 2));
    }
}

#[cfg(test)]
mod simple_in_memory_graph_tests {
    use super::*;

    #[test]
    fn test_new() {
        // == given ==
        let _g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
    }

    #[test]
    fn test_get_neighbors_for_absent_node() {
        // == given ==
        let g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();

        // == when ==
        let neighbors = g.get_neighbors(1);

        // == then ==
        assert_eq!(neighbors.is_none(), true);
    }

    #[test]
    fn test_add_edge_then_get_neighbors_for_present_node() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge(1, 2, 1);

        // == when ==
        let neighbors = g.get_neighbors(1);

        // == then ==
        assert_eq!(neighbors.is_some(), true);
        let mut iter = neighbors.unwrap();
        let next = iter.next();
        assert_eq!(next.is_some(), true);
        assert_eq!(*next.unwrap(), 2);
        let next = iter.next();
        assert_eq!(next.is_some(), false);
    }

    #[test]
    fn test_get_weight_for_absent_node() {
        // == given ==
        let g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();

        // == when ==
        let edge_weight = g.get_edge_weight(1, 2);

        // == then ==
        assert_eq!(edge_weight, None);
    }

    #[test]
    fn test_add_edge_then_get_weight() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge(1, 2, 1);

        // == when ==
        let edge_weight = g.get_edge_weight(1, 2);

        // == then ==
        assert_eq!(edge_weight, Some(1));
    }
}
