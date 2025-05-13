/*
 * Core functionality for the EzMemPass password generator.
 */

//! # EzMemPass Core
//!
//! This crate provides the core functionality for the EzMemPass password generator,
//! including interfaces, types, and error handling for the entire library ecosystem.

pub mod error;
pub mod generator;
pub mod types;

/// Version of the library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Returns the version of the library
pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}
