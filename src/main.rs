use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::{self, Read};
use std::process;
use thiserror::Error;
use url_parser_rust::{parse_url, ParseUrlException};

#[derive(Debug, Error)]
enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("URL parsing error: {0}")]
    UrlParsingError(#[from] ParseUrlException),
    #[error("Invalid subcommand. Use '--help' for more information.")]
    InvalidSubcommand,
}

impl From<&str> for AppError {
    fn from(s: &str) -> Self {
        AppError::UrlParsingError(ParseUrlException::ParsingException(s.to_string()))
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), AppError> {
    let matches = App::new("URL Parser")
        .version("0.1")
        .author("Sofiia Yeremeieva")
        .about("Parse URLs from a file or command-line input")
        .subcommand(
            SubCommand::with_name("parse")
                .about("Parse a URL")
                .arg(Arg::with_name("input").help("The URL or File path to parse")),
        )
        .subcommand(SubCommand::with_name("help").about("Print help information"))
        .subcommand(SubCommand::with_name("credits").about("Pring credit information"))
        .get_matches();

    match matches.subcommand() {
        ("parse", Some(parse_matches)) => {
            if let Some(input) = parse_matches.value_of("input") {
                if let Ok(mut file) = File::open(input) {
                    let mut content = String::new();
                    file.read_to_string(&mut content)?;
                    let parsed_url = parse_url(&content)?;
                    println!("{:#?}", parsed_url);
                } else {
                    let parsed_url = parse_url(input)?;
                    println!("{:#?}", parsed_url);
                }
            } else {
                return Err(AppError::InvalidSubcommand);
            }
        }
        ("help", _) => {
            println!("Custom url parser tool. \n");
            println!("1. Parse a URL from a String:\n");
            println!("```bash");
            println!("cargo run -- parse http://example.com");
            println!("```");
        }
        ("credits", _) => {
            println!("Credits: Author © Sofiia Yeremeieva");
        }
        _ => {
            return Err(AppError::InvalidSubcommand);
        }
    }

    Ok(())
}
