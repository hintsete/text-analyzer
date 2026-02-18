mod config;
mod analyzer;
mod stats;

use std::error::Error;
use std::fs;

use crate::config::{Config, OutputFormat};
use crate::analyzer::analyze;
use crate::stats::{Stats, export_stats};

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args()?;

    let contents = fs::read_to_string(&config.file_path)?;

    let result = analyze(&contents, &config);

    // Export or display
    match (&config.output_file, &config.format) {
        (Some(file), Some(format)) => {
            export_stats(&result, file, format)?;
        }
        _ => result.display(),
    }

    Ok(())
}
