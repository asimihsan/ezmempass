/*
 * Language definitions and utilities.
 */

use serde::{Deserialize, Serialize};
use std::fmt;

/// Supported languages for passphrase generation
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Language {
    /// ISO 639-1 language code (e.g., "en" for English)
    pub code: String,

    /// English name of the language
    pub name: String,

    /// Native name of the language
    pub native_name: String,

    /// Whether the language is available in the current build
    pub available: bool,
}

impl Language {
    /// Create a new language definition
    pub fn new(code: &str, name: &str, native_name: &str, available: bool) -> Self {
        Self {
            code: code.to_string(),
            name: name.to_string(),
            native_name: native_name.to_string(),
            available,
        }
    }

    /// Get the Huggingface dataset ID for this language
    pub fn dataset_id(&self) -> String {
        format!("wikipedia/20231201.{}", self.code)
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.code)
    }
}

/// Get all supported languages
pub fn get_supported_languages() -> Vec<Language> {
    vec![
        Language::new("en", "English", "English", true),
        Language::new("es", "Spanish", "Español", false),
        Language::new("fr", "French", "Français", false),
        Language::new("de", "German", "Deutsch", false),
        Language::new("zh", "Chinese", "中文", false),
        Language::new("ja", "Japanese", "日本語", false),
        Language::new("ru", "Russian", "Русский", false),
        Language::new("ar", "Arabic", "العربية", false),
    ]
}

/// Get a language by its ISO code
pub fn get_language_by_code(code: &str) -> Option<Language> {
    get_supported_languages()
        .into_iter()
        .find(|lang| lang.code == code)
}
