/*
TODO create a Graph impl backed by the wordlist data file, without needing to create
edges
*/

use priority_queue::PriorityQueue;
use rand::Rng;
use std::collections::{HashMap, HashSet};

/// Graph trait that allows algorithms to be applied.
///
/// TODO for simplicity I avoided generics and we can only store string slices. Make this generic.
pub trait Graph {
    fn new() -> Self;
    fn get_neighbors(&self, node: &str) -> HashSet<String>;
    fn get_edge_weight(&self, node1: &str, node2: &str) -> Option<i64>;
}

/// Simple completely in-memory modifiable directed graph that supports edge weights.
pub struct SimpleInMemoryGraph {
    edges: HashMap<String, HashSet<String>>,
    weights: HashMap<(String, String), i64>,
}

impl Graph for SimpleInMemoryGraph {
    fn new() -> Self {
        SimpleInMemoryGraph {
            edges: HashMap::new(),
            weights: HashMap::new(),
        }
    }

    fn get_neighbors(&self, node: &str) -> HashSet<String> {
        match self.edges.get(node) {
            None => HashSet::new(),
            Some(result) => result.clone(),
        }
    }

    fn get_edge_weight(&self, node1: &str, node2: &str) -> Option<i64> {
        if let Some(i) = self.weights.get(&(node1.to_string(), node2.to_string())) {
            Some(*i)
        } else {
            None
        }
    }
}

impl SimpleInMemoryGraph {
    pub fn add_edge(&mut self, node1: &str, node2: &str, weight: i64) {
        let existing_edges = self.edges.get_mut(node1);
        if existing_edges.is_none() {
            let mut new_destinations = HashSet::new();
            new_destinations.insert(node2.to_string());
            self.edges.insert(node1.to_string(), new_destinations);
        } else {
            existing_edges.unwrap().insert(node2.to_string());
        }
        self.weights
            .insert((node1.to_string(), node2.to_string()), weight);
    }
}

/// Given a vector with [0..max-1] elements inside it you want a random index that is weighted to
/// prefer elements at the front of the list. This is because our word lists are weighted most
/// common to least common.
fn get_weighted_random_index(max: usize) -> usize {
    let mut lo: i32 = 0;
    let mut hi: i32 = (max - 1) as i32;
    while lo <= hi {
        let mid: i32 = lo + (hi - lo) / 2;
        let random_value: f64 = rand::thread_rng().gen();
        if random_value < 0.8 {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }
    if hi < 1 {
        hi = 1;
    }
    rand::thread_rng().gen_range(0, hi) as usize
}

/// Returns shortest path from multiple start nodes to multiple goal nodes.
pub fn shortest_path_multiple(
    graph: &impl Graph,
    starts: Vec<&str>,
    goals: Vec<&str>,
) -> Option<(Vec<String>, i64)> {
    let mut current_shortest_path_cost: i64 = i64::min_value();
    let mut current_shortest_path: Vec<String> = Vec::new();
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
    while i < MAX_ITERS {
        let start = &starts[get_weighted_random_index(starts_len)];
        let goal = &goals[get_weighted_random_index(goals_len)];
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
        i += 1;
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
pub fn shortest_path(graph: &impl Graph, start: &str, goal: &str) -> Option<(Vec<String>, i64)> {
    let mut frontier: PriorityQueue<String, i64, _> = PriorityQueue::new();
    let mut came_from: HashMap<String, String> = HashMap::new();
    let mut cost_so_far: HashMap<String, i64> = HashMap::new();
    frontier.push(String::from(start), 0);
    cost_so_far.insert(String::from(start), 0);
    let mut found_goal: bool = false;

    while !frontier.is_empty() {
        let (current, _current_priority) = frontier.pop().unwrap();
        if current == goal {
            found_goal = true;
            break;
        }
        let current_cost = *cost_so_far.get_mut(&current).unwrap();
        for next in &graph.get_neighbors(&current) {
            let new_cost = current_cost + graph.get_edge_weight(&current, &next).unwrap();
            let existing_next_cost = cost_so_far.get(next);
            if existing_next_cost.is_none() || new_cost > *existing_next_cost.unwrap() {
                cost_so_far.insert(next.to_string(), new_cost);
                frontier.push(next.to_string(), new_cost);
                came_from.insert(next.to_string(), current.to_string());
            }
        }
    }

    if !found_goal {
        None
    } else {
        let mut result: Vec<String> = Vec::new();
        let mut current_node = goal;
        while current_node != start {
            result.push(current_node.to_string());
            current_node = came_from.get(current_node).unwrap();
        }
        result.push(current_node.to_string());
        result.reverse();
        let total_cost = *cost_so_far.get(goal).unwrap();
        Some((result, total_cost))
    }
}

#[cfg(test)]
mod shortest_path_simple_in_memory_graph_tests {
    use super::*;

    /// A -> B -> C -> D -> E, all edges weight of 1.
    /// A, E shortest path should return A, B, C, D, E
    #[test]
    fn test_basic_single() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge("A", "B", 1);
        g.add_edge("B", "C", 1);
        g.add_edge("C", "D", 1);
        g.add_edge("D", "E", 1);

        // == when ==
        let shortest_path = shortest_path(&g, "A", "E");

        // == then ==
        assert_eq!(shortest_path.is_some(), true);
        assert_eq!(
            shortest_path.unwrap(),
            (
                vec![
                    String::from("A"),
                    String::from("B"),
                    String::from("C"),
                    String::from("D"),
                    String::from("E"),
                ],
                4
            )
        );
    }

    /// A -> (1) -> B -> (5) -> E.
    /// A -> (3) -> D -> (1) -> E
    /// A, E shortest path should return A, D, E.
    #[test]
    fn test_basic_two_paths() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge("A", "B", 1);
        g.add_edge("B", "E", 5);
        g.add_edge("A", "D", 3);
        g.add_edge("D", "E", 1);

        // == when ==
        let shortest_path = shortest_path(&g, "A", "E");

        // == then ==
        assert_eq!(shortest_path.is_some(), true);
        assert_eq!(
            shortest_path.unwrap(),
            (
                vec![String::from("A"), String::from("D"), String::from("E")],
                4
            )
        );
    }

    /// A -> (1) -> B -> (5) -> C.
    /// D -> (3) -> E
    /// A, E shortest path should return None.
    #[test]
    fn test_no_path() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge("A", "B", 1);
        g.add_edge("B", "C", 5);
        g.add_edge("D", "E", 3);

        // == when ==
        let shortest_path = shortest_path(&g, "A", "E");

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
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge("A", "B", -1);
        g.add_edge("B", "C", -5);
        g.add_edge("D", "E", -1);
        g.add_edge("E", "F", -1);

        // == when ==
        let shortest_path = shortest_path_multiple(&g, vec!["A", "D"], vec!["C", "F"]);

        // == then ==
        assert_eq!(shortest_path.is_some(), true);
        assert_eq!(
            shortest_path.unwrap(),
            (
                vec![String::from("D"), String::from("E"), String::from("F")],
                2,
            )
        );
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
        let neighbors = g.get_neighbors(&String::from("foo"));

        // == then ==
        assert_eq!(neighbors.len(), 0);
    }

    #[test]
    fn test_add_edge_then_get_neighbors_for_present_node() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge("foo", "bar", 1);

        // == when ==
        let neighbors = g.get_neighbors(&String::from("foo"));

        // == then ==
        assert_eq!(neighbors.len(), 1);

        let mut expected_neighbors = HashSet::new();
        expected_neighbors.insert("bar".to_string());
        assert_eq!(neighbors, expected_neighbors);
    }

    #[test]
    fn test_get_weight_for_absent_node() {
        // == given ==
        let g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();

        // == when ==
        let edge_weight = g.get_edge_weight(&String::from("foo"), &String::from("bar"));

        // == then ==
        assert_eq!(edge_weight, None);
    }

    #[test]
    fn test_add_edge_then_get_weight() {
        // == given ==
        let mut g: SimpleInMemoryGraph = SimpleInMemoryGraph::new();
        g.add_edge("foo", "bar", 1);

        // == when ==
        let edge_weight = g.get_edge_weight(&String::from("foo"), &String::from("bar"));

        // == then ==
        assert_eq!(edge_weight, Some(1));
    }
}
