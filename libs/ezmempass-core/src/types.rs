/*
 * Core types for the EzMemPass library.
 */

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a generated password with additional information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedPassword {
    /// The generated password string
    pub password: String,

    /// Entropy bits (measure of password strength)
    pub entropy_bits: f64,

    /// Method used to generate the password
    pub method: GenerationMethod,

    /// Additional memory aids to help remember the password (optional)
    pub memory_aids: Option<Vec<String>>,
}

/// Method used to generate the password
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GenerationMethod {
    /// Generated using language model (most memorable)
    LanguageModel,

    /// Generated using graph search algorithm
    GraphSearch,

    /// Generated using random algorithm (least memorable but highest entropy)
    Random,
}

impl fmt::Display for GenerationMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenerationMethod::LanguageModel => write!(f, "Language Model"),
            GenerationMethod::GraphSearch => write!(f, "Graph Search"),
            GenerationMethod::Random => write!(f, "Random"),
        }
    }
}

/// Options for password generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationOptions {
    /// Target length of the generated password in words
    pub word_count: usize,

    /// Whether to include uppercase letters
    pub include_uppercase: bool,

    /// Whether to include digits
    pub include_digits: bool,

    /// Whether to include symbols
    pub include_symbols: bool,

    /// Preferred generation method
    pub preferred_method: GenerationMethod,

    /// Minimum entropy bits required
    pub min_entropy_bits: f64,

    /// Language for generation (ISO 639-1 code)
    pub language: String,
}

impl Default for GenerationOptions {
    fn default() -> Self {
        Self {
            word_count: 4,
            include_uppercase: true,
            include_digits: true,
            include_symbols: false,
            preferred_method: GenerationMethod::LanguageModel,
            min_entropy_bits: 60.0,
            language: "en".to_string(),
        }
    }
}
