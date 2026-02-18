use std::fs::File;
use std::error::Error;

use serde::Serialize;

#[derive(Serialize)]
pub struct Stats {
    pub total_words: usize,
    pub unique_words: usize,
    pub most_common: Option<(String, usize)>,
}

impl Stats {
    pub fn display(&self) {
        println!("Total words: {}", self.total_words);
        println!("Unique words: {}", self.unique_words);
        match &self.most_common {
            Some((word, count)) => println!("Most common word: '{}' ({})", word, count),
            None => println!("Most common word: None"),
        }
    }
}

pub fn export_stats(stats: &Stats, filename: &str, format: &crate::config::OutputFormat) -> Result<(), Box<dyn Error>> {
    match format {
        crate::config::OutputFormat::Json => {
            let file = File::create(filename)?;
            serde_json::to_writer_pretty(file, stats)?;
        }
        crate::config::OutputFormat::Csv => {
            let mut wtr = csv::Writer::from_path(filename)?;
            wtr.write_record(&["metric", "value"])?;
            wtr.write_record(&["total_words", &stats.total_words.to_string()])?;
            wtr.write_record(&["unique_words", &stats.unique_words.to_string()])?;
            if let Some((word, count)) = &stats.most_common {
                let combined = format!("{} ({})", word, count);
                wtr.write_record(&["most_common_word", &combined])?;
            }
            wtr.flush()?;
        }
    }
    Ok(())
}

