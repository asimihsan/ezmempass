/*
 * Graph model and serialization.
 */

use std::collections::HashMap;
use std::path::Path;
use serde::{Deserialize, Serialize};
use anyhow::Result;

use crate::WordGraph;
use crate::error::GraphError;

/// Serializable representation of a graph
#[derive(Debug, Serialize, Deserialize)]
pub struct SerializableGraph {
    /// Graph nodes (words)
    pub nodes: Vec<String>,
    
    /// Graph edges as (from_index, to_index, weight)
    pub edges: Vec<(usize, usize, f64)>,
}

impl SerializableGraph {
    /// Convert a WordGraph to a serializable representation
    pub fn from_word_graph(graph: &WordGraph) -> Self {
        // This is a placeholder implementation
        // In a real implementation, we would convert the petgraph structure
        SerializableGraph {
            nodes: vec![],
            edges: vec![],
        }
    }
    
    /// Convert back to a WordGraph
    pub fn to_word_graph(&self) -> Result<WordGraph, GraphError> {
        let mut graph = WordGraph::new();
        
        // Add all nodes
        for word in &self.nodes {
            graph.add_word(word);
        }
        
        // Add all edges
        for &(from_idx, to_idx, weight) in &self.edges {
            if from_idx >= self.nodes.len() || to_idx >= self.nodes.len() {
                return Err(GraphError::LoadError(
                    format!("Invalid node indices: {} or {}", from_idx, to_idx)
                ));
            }
            
            graph.add_edge(&self.nodes[from_idx], &self.nodes[to_idx], weight)
                .map_err(|e| GraphError::LoadError(e.to_string()))?;
        }
        
        Ok(graph)
    }
    
    /// Save the graph to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), GraphError> {
        let file = std::fs::File::create(path)
            .map_err(|e| GraphError::SaveError(e.to_string()))?;
        
        serde_json::to_writer(file, self)
            .map_err(|e| GraphError::SaveError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Load a graph from a file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, GraphError> {
        let file = std::fs::File::open(path)
            .map_err(|e| GraphError::LoadError(e.to_string()))?;
        
        serde_json::from_reader(file)
            .map_err(|e| GraphError::LoadError(e.to_string()))?;
        
        Ok(Self {
            nodes: vec![],
            edges: vec![],
        })
    }
}