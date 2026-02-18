use std::collections::HashMap;

use crate::config::{Config, STOP_WORDS};
use crate::stats::Stats;

/// Clean a word by removing punctuation and converting to lowercase
fn clean_word(word: &str) -> String {
    word.chars()
        .filter(|c| c.is_alphanumeric()) // remove punctuation
        .collect::<String>()
        .to_lowercase()
}

/// Analyze text with filters and return Stats
pub fn analyze(text: &str, config: &Config) -> Stats {
    // Build filter closure dynamically
    let filter = |word: &str| {
        let length_ok = config.min_length.map_or(true, |min| word.len() > min);
        let starts_ok = config.starts_with.map_or(true, |c| word.starts_with(c));

        // Stop words filtering
        let stop_ok = if config.no_stop_word {
            !STOP_WORDS.contains(&word)
        } else {
            true
        };

        length_ok && starts_ok && stop_ok
    };

    // Functional pipeline: split -> clean -> filter -> fold
    let word_counts: HashMap<String, usize> = text
        .split_whitespace()
        .map(|w| clean_word(w))       // remove punctuation and lowercase
        .filter(|w| !w.is_empty())    // remove empty strings
        .filter(|w| filter(w))        // dynamic filter
        .fold(HashMap::new(), |mut acc, word| {
            *acc.entry(word).or_insert(0) += 1;
            acc
        });

    let total_words = word_counts.values().sum();
    let unique_words = word_counts.len();
    let most_common = word_counts
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, count)| (word.clone(), *count));

    Stats {
        total_words,
        unique_words,
        most_common,
    }
}
