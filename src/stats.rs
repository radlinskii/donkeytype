//! Module with test statistics structure.
//!
//! Used to display to the user results of the test
//! and save those results and configuration of the test to a file.

use anyhow::{Context, Result};
use serde::Serialize;
use std::fs::{create_dir_all, OpenOptions};

#[derive(Debug, Serialize)]
struct Record {
    city: String,
    region: String,
    country: String,
    population: Option<u64>,
}
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
    /// saves test statistics and configuration to a file in users home directory
    pub fn save_to_file(self) -> Result<(), anyhow::Error> {
        let results_dir_path = dirs::home_dir()
            .context("Unable to get home directory")?
            .join(".local")
            .join("share")
            .join("donkeytype");

        if !results_dir_path.exists() {
            create_dir_all(results_dir_path.clone())
                .context("Unable to create results directory for results file")?;
        }

        let results_file_path = results_dir_path.join("donkeytype-results.csv");

        let results_file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .append(true)
            .open(results_file_path)
            .unwrap();

        let mut rdr =
            csv::Reader::from_reader(results_file.try_clone().context("Unable to clone file")?);

        let should_append_headers = rdr
            .headers()
            .context("Unable to read headers from results file")?
            .is_empty();

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(should_append_headers)
            .flexible(true)
            .from_writer(results_file);

        wtr.serialize(Record {
            city: "City 5".to_string(),
            region: "MA".to_string(),
            country: "United States".to_string(),
            population: Some(14061),
        })
        .context("Unable to serialize results")?;

        wtr.flush()
            .context("Unable to flush inner csv crate buffer to writer")?;

        Ok(())
    }

    /// prints statistics in an easy to read fashion
    pub fn print(&self) {
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
