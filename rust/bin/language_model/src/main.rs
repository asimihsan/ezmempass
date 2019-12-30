extern crate clap;

use clap::{App, Arg, SubCommand};
use std::error::Error;
use std::path::Path;

pub mod create_arpa_model;
pub mod split;

fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new("Language model generator for EzMemPass")
        .subcommand(
        SubCommand::with_name("split")
            .about("Split a cirrussearch JSON GZ file into pieces")
            .arg(
                Arg::with_name("input_path")
                    .long("input-path")
                    .short("p")
                    .required(true)
                    .takes_value(true)
                    .validator(input_path_is_file)
                    .help("Path to cirrussearch JSON GZ file, download from https://dumps.wikimedia.org/other/cirrussearch/")
                    .value_name("FILE"),
            )
            .arg(
                Arg::with_name("output_dir")
                    .long("output-dir")
                    .short("o")
                    .required(true)
                    .takes_value(true)
                    .help("Output directory for split files. Will be deleted if exists.")
                    .value_name("DIR"),
            )
            .arg(
                Arg::with_name("pieces")
                    .long("pieces")
                    .short("s")
                    .required(false)
                    .takes_value(true)
                    .validator(validate_pieces)
                    .default_value("12")
                    .help("How many pieces to split the input file into.")
                    .value_name("POSITIVE INTEGER"),
            ))
        .subcommand(
        SubCommand::with_name("create-arpa-model")
            .about("Create an ARPA language model from line-delimited files of articles")
            .arg(
                Arg::with_name("input_dir")
                    .long("input-dir")
                    .short("d")
                    .required(true)
                    .takes_value(true)
                    .validator(validate_input_dir)
                    .help("Directory full of line-delimited GZ files. Will put output ARPA model file here.")
                    .value_name("DIR"),
            )
        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("split", Some(split_matches)) => {
            let input_path = Path::new(split_matches.value_of("input_path").unwrap());
            let output_dir = Path::new(split_matches.value_of("output_dir").unwrap());
            let pieces = split_matches
                .value_of("pieces")
                .unwrap()
                .parse::<u32>()
                .unwrap();
            split::handle_split(input_path, output_dir, pieces)
        }
        ("create-arpa-model", Some(create_arpa_model_matches)) => {
            let input_dir = Path::new(create_arpa_model_matches.value_of("input_dir").unwrap());
            create_arpa_model::handle_create_arpa_model(input_dir)
        }
        ("", None) => {
            let err: Box<dyn Error> = String::from("Need to specify a sub-command.").into();
            Err(err)
        }
        _ => unreachable!(),
    }
}

fn validate_pieces(input: String) -> Result<(), String> {
    match input.parse::<u32>() {
        Ok(value) => {
            if value == 0 {
                Err(String::from("Pieces cannot be 0."))
            } else if value > 1024 {
                Err(String::from("Pieces too large, must be smaller than 1024."))
            } else {
                Ok(())
            }
        }
        Err(_) => Err(String::from("Pieces is not a valid integer.")),
    }
}

fn input_path_is_file(input: String) -> Result<(), String> {
    if Path::new(&input).is_file() {
        Ok(())
    } else {
        Err(String::from(
            "Cirrus JSON GZ input filepath does not exist or isn't a file.",
        ))
    }
}

fn validate_input_dir(input: String) -> Result<(), String> {
    if Path::new(&input).is_dir() {
        Ok(())
    } else {
        Err(String::from(
            "Input path doesn't exist or isn't a directory.",
        ))
    }
}
