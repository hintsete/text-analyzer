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
            Some((word, count)) => {
                println!("Most common word: '{}' ({})", word, count);
            }
            None => {
                println!("Most common word: None");
            }
        }
    }
}
