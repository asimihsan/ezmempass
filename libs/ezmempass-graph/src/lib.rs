/*
 * Graph search implementation for EzMemPass using petgraph.
 */

//! # EzMemPass Graph
//!
//! This crate provides the graph search implementation for EzMemPass,
//! using the petgraph library for efficient graph operations.

use petgraph::algo;
use petgraph::graph::{DiGraph, NodeIndex};
use std::collections::HashMap;

use ezmempass_core::error::{EzMemPassError, Result};
use ezmempass_core::generator::PasswordGenerator;
use ezmempass_core::types::{GeneratedPassword, GenerationMethod, GenerationOptions};

/// WordGraph represents a directed graph of words and their connections
pub struct WordGraph {
    /// The underlying directed graph from petgraph
    graph: DiGraph<String, f64>,

    /// Mapping from words to their node indices
    word_to_node: HashMap<String, NodeIndex>,
}

impl Default for WordGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl WordGraph {
    /// Create a new empty word graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            word_to_node: HashMap::new(),
        }
    }

    /// Add a word to the graph if it doesn't already exist
    pub fn add_word(&mut self, word: &str) -> NodeIndex {
        if let Some(&node_idx) = self.word_to_node.get(word) {
            return node_idx;
        }

        let node_idx = self.graph.add_node(word.to_string());
        self.word_to_node.insert(word.to_string(), node_idx);
        node_idx
    }

    /// Add a weighted edge between two words
    pub fn add_edge(&mut self, from: &str, to: &str, weight: f64) -> Result<()> {
        let from_idx = self.add_word(from);
        let to_idx = self.add_word(to);

        // Add the edge with the given weight
        self.graph.add_edge(from_idx, to_idx, weight);

        Ok(())
    }

    /// Find the shortest path between two words
    pub fn shortest_path(&self, from: &str, to: &str) -> Result<Vec<String>> {
        let from_idx = self
            .word_to_node
            .get(from)
            .ok_or_else(|| EzMemPassError::GraphError(format!("Word '{}' not in graph", from)))?;

        let to_idx = self
            .word_to_node
            .get(to)
            .ok_or_else(|| EzMemPassError::GraphError(format!("Word '{}' not in graph", to)))?;

        // Use Dijkstra's algorithm to find the shortest path
        let path = algo::astar(
            &self.graph,
            *from_idx,
            |n| n == *to_idx,
            |e| *e.weight(),
            |_| 0.0, // No heuristic for now
        );

        match path {
            Some((_, nodes)) => {
                let mut words = Vec::new();
                for node_idx in nodes {
                    let word = self.graph.node_weight(node_idx).ok_or_else(|| {
                        EzMemPassError::InternalError("Invalid node index".to_string())
                    })?;
                    words.push(word.clone());
                }
                Ok(words)
            }
            None => Err(EzMemPassError::GraphError(format!(
                "No path from '{}' to '{}'",
                from, to
            ))),
        }
    }

    /// Get a random path of the specified length
    pub fn random_path(&self, length: usize) -> Result<Vec<String>> {
        // This is a stub implementation
        // In a real implementation, we'd use petgraph to find a random path

        if self.graph.node_count() == 0 {
            return Err(EzMemPassError::GraphError("Graph is empty".to_string()));
        }

        // Just return some words from the graph
        let mut words = Vec::new();
        for (i, node_idx) in self.graph.node_indices().take(length).enumerate() {
            if let Some(word) = self.graph.node_weight(node_idx) {
                words.push(word.clone());
            } else if i == 0 {
                // If we couldn't get any words, return a dummy path
                return Ok(vec![
                    "correct".to_string(),
                    "horse".to_string(),
                    "battery".to_string(),
                    "staple".to_string(),
                ]);
            }
        }

        // If we didn't get enough words, cycle the ones we have
        while words.len() < length {
            if let Some(word) = words.get(words.len() % words.len()) {
                words.push(word.clone());
            }
        }

        Ok(words)
    }

    /// Get the number of nodes in the graph
    pub fn node_count(&self) -> usize {
        self.graph.node_count()
    }

    /// Get the number of edges in the graph
    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }
}

/// Graph-based password generator
pub struct GraphPasswordGenerator {
    /// The word graph
    graph: WordGraph,
}

impl Default for GraphPasswordGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphPasswordGenerator {
    /// Create a new graph-based password generator
    pub fn new() -> Self {
        // Initialize with a minimal graph
        let mut graph = WordGraph::new();

        // Add some basic words and connections
        let words = ["correct", "horse", "battery", "staple"];
        for i in 0..words.len() {
            for j in 0..words.len() {
                if i != j {
                    graph.add_edge(words[i], words[j], 1.0).unwrap();
                }
            }
        }

        Self { graph }
    }
}

impl PasswordGenerator for GraphPasswordGenerator {
    fn generate(&self, options: &GenerationOptions) -> Result<GeneratedPassword> {
        // Use the graph to generate a random path
        let words = self.graph.random_path(options.word_count)?;
        let mut password = words.join("-");

        // Apply transformations based on options
        if options.include_uppercase {
            password = password
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().next().unwrap()
                    } else {
                        c
                    }
                })
                .collect();
        }

        if options.include_digits {
            password.push_str("42");
        }

        if options.include_symbols {
            password.push('!');
        }

        Ok(GeneratedPassword {
            password,
            entropy_bits: 60.0, // Placeholder
            method: GenerationMethod::GraphSearch,
            memory_aids: Some(vec![
                "Imagine walking a path through these words".to_string(),
            ]),
        })
    }

    fn calculate_entropy(&self, _password: &str) -> Result<f64> {
        // Stub implementation
        Ok(60.0)
    }

    fn generate_memory_aids(&self, password: &str) -> Result<Vec<String>> {
        // Stub implementation
        Ok(vec![format!("Visualize a path through: {}", password)])
    }

    fn supports_language(&self, language_code: &str) -> bool {
        // Only support English in this stub
        language_code == "en"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_graph() {
        let mut graph = WordGraph::new();
        graph.add_word("hello");
        graph.add_word("world");
        graph.add_edge("hello", "world", 1.0).unwrap();

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
    }

    #[test]
    fn test_graph_generator() {
        let generator = GraphPasswordGenerator::new();
        let options = GenerationOptions::default();

        let result = generator.generate(&options).unwrap();
        assert_eq!(result.method, GenerationMethod::GraphSearch);
        assert!(result.memory_aids.is_some());
    }
}
