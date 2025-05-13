/*
 * Password generator traits and interfaces.
 */

use crate::error::Result;
use crate::types::{GeneratedPassword, GenerationOptions};

/// Trait for password generators
pub trait PasswordGenerator {
    /// Generate a password with the given options
    fn generate(&self, options: &GenerationOptions) -> Result<GeneratedPassword>;

    /// Calculate the entropy of a given password
    fn calculate_entropy(&self, password: &str) -> Result<f64>;

    /// Generate memory aids for a password to help memorization
    fn generate_memory_aids(&self, password: &str) -> Result<Vec<String>>;

    /// Check if the generator supports a specific language
    fn supports_language(&self, language_code: &str) -> bool;
}

/// Factory for creating password generators
pub struct PasswordGeneratorFactory;

impl PasswordGeneratorFactory {
    /// Create a new password generator based on the preferred method
    pub fn create(_method: crate::types::GenerationMethod) -> Box<dyn PasswordGenerator> {
        // This is just a stub implementation that returns a dummy generator
        Box::new(DummyPasswordGenerator)
    }
}

/// Dummy implementation of PasswordGenerator for testing
#[derive(Debug)]
pub struct DummyPasswordGenerator;

impl PasswordGenerator for DummyPasswordGenerator {
    fn generate(&self, options: &GenerationOptions) -> Result<GeneratedPassword> {
        // Just return a dummy implementation
        let words = ["correct", "horse", "battery", "staple"];
        let selected: Vec<&str> = words
            .iter()
            .copied()
            .cycle()
            .take(options.word_count)
            .collect();
        let password = selected.join("-");

        Ok(GeneratedPassword {
            password,
            entropy_bits: 60.0,
            method: options.preferred_method,
            memory_aids: Some(vec![
                "Picture a correct horse with a battery staple".to_string(),
            ]),
        })
    }

    fn calculate_entropy(&self, _password: &str) -> Result<f64> {
        // Dummy implementation
        Ok(60.0)
    }

    fn generate_memory_aids(&self, password: &str) -> Result<Vec<String>> {
        // Dummy implementation
        Ok(vec![format!("Try to visualize: {}", password)])
    }

    fn supports_language(&self, language_code: &str) -> bool {
        // Only support English in this stub
        language_code == "en"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::GenerationMethod;

    #[test]
    fn test_dummy_generator() {
        let options = GenerationOptions {
            word_count: 4,
            preferred_method: GenerationMethod::LanguageModel,
            ..Default::default()
        };

        let generator = DummyPasswordGenerator;
        let result = generator.generate(&options).unwrap();

        assert_eq!(result.password, "correct-horse-battery-staple");
        assert_eq!(result.entropy_bits, 60.0);
        assert_eq!(result.method, GenerationMethod::LanguageModel);
        assert!(result.memory_aids.is_some());
    }
}
