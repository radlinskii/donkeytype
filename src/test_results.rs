//! Module with test statistics structure.
//!
//! Used to display to the user results of the test
//! and save those results and configuration of the test to a file.

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use std::fs::create_dir_all;

use crate::config::Config;

/// TestResults struct is combining test statistics with configuration of the test.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResults {
    pub local_datetime: DateTime<Local>,

    pub wpm: Option<f64>,
    pub raw_accuracy: Option<f64>,
    pub raw_valid_characters_count: Option<u64>,
    pub raw_mistakes_count: Option<u64>,
    pub raw_typed_characters_count: Option<u64>,
    pub accuracy: Option<f64>,
    pub valid_characters_count: Option<u64>,
    pub typed_characters_count: Option<u64>,
    pub mistakes_count: Option<u64>,

    pub duration: Option<u64>,
    pub numbers: Option<bool>,
    pub numbers_ratio: Option<f64>,
    pub dictionary_path: Option<String>,
    pub uppercase: Option<bool>,
    pub uppercase_ratio: Option<f64>,

    // tells if test was successfully completed and results should be displayed and saved.
    #[serde(skip)]
    pub completed: bool,
    #[serde(skip)]
    pub save: bool,
}

/// Struct holding numeric test results.
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
    pub fn default() -> Self {
        Stats {
            wpm: 0.0,
            raw_accuracy: 0.0,
            raw_valid_characters_count: 0,
            raw_mistakes_count: 0,
            raw_typed_characters_count: 0,
            accuracy: 0.0,
            valid_characters_count: 0,
            mistakes_count: 0,
            typed_characters_count: 0,
        }
    }
}

impl TestResults {
    /// creates TestResults object from Stats and Config
    pub fn new(stats: Stats, config: Config, completed: bool) -> Self {
        TestResults {
            local_datetime: Local::now(),
            // stats
            wpm: Some(stats.wpm),
            raw_accuracy: Some(stats.raw_accuracy),
            raw_valid_characters_count: Some(stats.raw_valid_characters_count),
            raw_mistakes_count: Some(stats.raw_mistakes_count),
            raw_typed_characters_count: Some(stats.raw_typed_characters_count),
            accuracy: Some(stats.accuracy),
            valid_characters_count: Some(stats.valid_characters_count),
            typed_characters_count: Some(stats.typed_characters_count),
            mistakes_count: Some(stats.mistakes_count),
            // config
            duration: Some(config.duration.as_secs()),
            numbers: Some(config.numbers),
            numbers_ratio: Some(config.numbers_ratio),
            dictionary_path: if let Some(str) = config.dictionary_path.to_str() {
                Some(str.to_string())
            } else {
                None
            },
            uppercase: Some(config.uppercase),
            uppercase_ratio: Some(config.uppercase_ratio),

            completed,
            save: config.save_results,
        }
    }

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

        let mut reader = csv::Reader::from_path(results_file_path.clone())
            .context("Unable to create CSV Reader")?;

        let records: Vec<TestResults> = reader
            .deserialize()
            .collect::<Result<_, csv::Error>>()
            .context("Unable to deserialize results")?;

        let mut writer =
            csv::Writer::from_path(results_file_path).context("Unable to create CSV Writer")?;

        for record in &records {
            println!("{}", record.local_datetime);
            writer
                .serialize(record)
                .context("Unable to serialize one of previous results")?;
        }

        writer
            .serialize(self)
            .context("Unable to serialize current test results")?;

        writer
            .flush()
            .context("Unable to flush inner csv crate buffer to writer")?;

        Ok(())
    }

    /// prints statistics in an easy to read format
    pub fn print_stats(&self) {
        if let Some(wpm) = self.wpm {
            println!("WPM: {:.2}", wpm);
        }
        if let Some(raw_accuracy) = self.raw_accuracy {
            println!("Raw accuracy: {:.2}%", raw_accuracy);
        }
        if let Some(raw_valid_characters_count) = self.raw_valid_characters_count {
            println!("Raw valid characters: {}", raw_valid_characters_count);
        }
        if let Some(raw_mistakes_count) = self.raw_mistakes_count {
            println!("Raw mistakes: {}", raw_mistakes_count);
        }
        if let Some(raw_typed_characters_count) = self.raw_typed_characters_count {
            println!("Raw characters typed: {}", raw_typed_characters_count);
        }
        if let Some(accuracy) = self.accuracy {
            println!("Accuracy after corrections: {:.2}%", accuracy);
        }
        if let Some(valid_characters_count) = self.valid_characters_count {
            println!(
                "Valid characters after corrections: {}",
                valid_characters_count
            );
        }
        if let Some(mistakes_count) = self.mistakes_count {
            println!("Mistakes after corrections: {}", mistakes_count);
        }
        if let Some(typed_characters_count) = self.typed_characters_count {
            println!(
                "Characters typed after corrections: {}",
                typed_characters_count
            );
        }
    }
}
