mod config;
mod analyzer;
mod stats;

use std::error::Error;
use std::fs;

use config::Config;
use analyzer::analyze;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::from_args()?;

    let contents = fs::read_to_string(&config.file_path)?;

    let result = analyze(&contents, &config);

    result.display();

    Ok(())
}
