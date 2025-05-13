/*
 * Command-line interface for EzMemPass.
 */

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use ezmempass_core::generator::PasswordGenerator;
use ezmempass_core::types::{GenerationMethod, GenerationOptions};
use ezmempass_graph::GraphPasswordGenerator;
use ezmempass_models::LanguageModelGenerator;

/// EzMemPass - Memorable Password Generator
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new password
    Generate {
        /// Length of password in words
        #[arg(short, long, default_value_t = 4)]
        length: usize,

        /// Include uppercase letters
        #[arg(short = 'u', long)]
        uppercase: bool,

        /// Include digits
        #[arg(short, long)]
        digits: bool,

        /// Include symbols
        #[arg(short, long)]
        symbols: bool,

        /// Generation method (model, graph, random)
        #[arg(short, long, default_value = "model")]
        method: String,

        /// Language code (e.g., en for English)
        #[arg(short, long, default_value = "en")]
        language: String,
    },

    /// Build or train models
    Build {
        /// Type of model to build (language, graph)
        #[arg(short, long, default_value = "language")]
        model_type: String,

        /// Path to input data
        #[arg(short, long)]
        input: Option<PathBuf>,

        /// Path to output location
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Language code (e.g., en for English)
        #[arg(short, long, default_value = "en")]
        language: String,
    },
}

fn main() -> ezmempass_core::error::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate {
            length,
            uppercase,
            digits,
            symbols,
            method,
            language,
        }) => {
            println!("EzMemPass: Generating password");

            // Set up generation options
            let options = GenerationOptions {
                word_count: *length,
                include_uppercase: *uppercase,
                include_digits: *digits,
                include_symbols: *symbols,
                preferred_method: match method.as_str() {
                    "graph" => GenerationMethod::GraphSearch,
                    "random" => GenerationMethod::Random,
                    _ => GenerationMethod::LanguageModel,
                },
                language: language.clone(),
                ..Default::default()
            };

            // Create the appropriate generator
            let result = match options.preferred_method {
                GenerationMethod::GraphSearch => {
                    let generator = GraphPasswordGenerator::new();
                    generator.generate(&options)?
                }
                _ => {
                    let generator = LanguageModelGenerator::new();
                    generator.generate(&options)?
                }
            };

            // Display the result
            println!("\nGenerated Password:");
            println!("  {}", result.password);
            println!("\nDetails:");
            println!("  Method: {}", result.method);
            println!("  Entropy: {:.1} bits", result.entropy_bits);

            if let Some(memory_aids) = result.memory_aids {
                println!("\nMemory Aid:");
                for aid in memory_aids {
                    println!("  {}", aid);
                }
            }

            Ok(())
        }

        Some(Commands::Build {
            model_type,
            input,
            output,
            language,
        }) => {
            println!("EzMemPass: Building model");
            println!("  Model type: {}", model_type);
            println!("  Language: {}", language);

            if let Some(input_path) = input {
                println!("  Input: {}", input_path.display());
            }

            if let Some(output_path) = output {
                println!("  Output: {}", output_path.display());
            }

            println!("\nModel building not implemented in this stub.");

            Ok(())
        }

        None => {
            println!("EzMemPass CLI");
            println!("Run 'ezmempass-cli generate' to generate a password.");
            println!("Run 'ezmempass-cli --help' for usage information.");

            Ok(())
        }
    }
}
