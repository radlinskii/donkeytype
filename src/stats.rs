//! Module with test statistics structure.
//!
//! Used to display to the user results of the test
//! and save those results and configuration of the test to a file.

/// Struct holding the test results.
#[derive(Debug)]
pub struct Stats {
    pub wpm: f64,
    pub raw_accuracy: f64,
    pub raw_valid_characters_count: u64,
    pub raw_mistakes_count: u64,
    pub raw_typed_characters_count: u64,
    pub accuracy: f64,
    pub valid_characters_count: u64,
    pub typed_characters_count: u64,
    pub mistakes_count: u64,
}

impl Stats {
    /// prints statistics in an easy to read fashion
    pub fn print(self) {
        println!("WPM: {:.2}", self.wpm);
        println!("Raw accuracy: {:.2}%", self.raw_accuracy);
        println!("Raw valid characters: {}", self.raw_valid_characters_count);
        println!("Raw mistakes: {}", self.raw_mistakes_count);
        println!("Raw characters typed: {}", self.raw_typed_characters_count);
        println!("Accuracy after corrections: {:.2}%", self.accuracy);
        println!(
            "Valid characters after corrections: {}",
            self.valid_characters_count
        );
        println!("Mistakes after corrections: {}", self.mistakes_count);
        println!(
            "Characters typed after corrections: {}",
            self.typed_characters_count
        );
    }
}
