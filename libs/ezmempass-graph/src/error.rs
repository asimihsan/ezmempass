/*
 * Error types for graph operations.
 */

use thiserror::Error;

/// Errors specific to graph operations
#[derive(Error, Debug)]
pub enum GraphError {
    /// Error when building a graph
    #[error("Failed to build graph: {0}")]
    BuildError(String),
    
    /// Error when searching a graph
    #[error("Graph search error: {0}")]
    SearchError(String),
    
    /// Error when loading a graph from storage
    #[error("Failed to load graph: {0}")]
    LoadError(String),
    
    /// Error when saving a graph to storage
    #[error("Failed to save graph: {0}")]
    SaveError(String),
}