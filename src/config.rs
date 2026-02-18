use std::env;
use std::error::Error;

/// Default stop words
pub const STOP_WORDS: &[&str] = &[
    "the", "a", "an", "and", "in", "on", "of", "is", "to", "for", "with", "by"
];

#[derive(Debug)]
pub enum OutputFormat {
    Json,
    Csv,
}

pub struct Config {
    pub file_path: String,
    pub min_length: Option<usize>,
    pub starts_with: Option<char>,
    pub no_stop_word: bool,
    pub output_file: Option<String>,
    pub format: Option<OutputFormat>,
}

impl Config {
    pub fn from_args() -> Result<Self, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();

        if args.len() < 2 {
            return Err("Usage: text_analyzer <file_path> [--min-length N] [--starts-with C] [--no-stop-word] [--output file] [--format json|csv]".into());
        }

        let file_path = args[1].clone();

        let min_length = args.iter()
            .position(|arg| arg == "--min-length")
            .and_then(|i| args.get(i + 1))
            .map(|n| n.parse::<usize>())
            .transpose()?; 

        let starts_with = args.iter()
            .position(|arg| arg == "--starts-with")
            .and_then(|i| args.get(i + 1))
            .and_then(|c| c.chars().next());

        let no_stop_word = args.iter().any(|arg| arg == "--no-stop-word");

        let output_file = args.iter()
            .position(|arg| arg == "--output")
            .and_then(|i| args.get(i + 1))
            .cloned();

        let format = args.iter()
            .position(|arg| arg == "--format")
            .and_then(|i| args.get(i + 1))
            .map(|s| match s.as_str() {
                "json" => OutputFormat::Json,
                "csv" => OutputFormat::Csv,
                _ => OutputFormat::Json,
            });

        Ok(Config {
            file_path,
            min_length,
            starts_with,
            no_stop_word,
            output_file,
            format,
        })
    }
}
