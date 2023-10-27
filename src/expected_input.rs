//! Module creating the expected input for the test runner
//!
//! It reads dictionary file to get list of words
//! then shuffles this list
//! optionally replaces words with numbers if flag `numbers` is set to true in config
//! and returns as a string
//!
//! Dictionary file should be a text file in format of single words per line.

use anyhow::{Context, Result};
use mockall::automock;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::io::Read;

use crate::config::Config;
use crate::dictionary;
use crate::helpers::split_by_char_index;

/// Struct used by runner to hold generate the text used for validation and as a placeholder
#[derive(Debug)]
pub struct ExpectedInput {
    str: String,
}

impl ExpectedInput {
    /// Create new struct instance by reading the dictionary file
    ///
    /// After reading the file iterate over the words and apply the
    /// specified settings.
    ///
    /// Each setting is applied according to the specified ratio.
    /// * `uppercase` will capitalize the word. ("hello" => "Hello")
    /// * `numbers` will turn each letter of a word into a random number. (
    /// "hello" => "52139")
    /// * `symbols` will either append a symbol or surround the word with
    /// matching symbols. ("hello" => "hello!", "hello" => "{hello}")
    pub fn new(config: &Config) -> Result<Self, anyhow::Error> {
        let mut str = dictionary::WORDS.to_string();
        if let Some(dictionary_path) = &config.dictionary_path {
            str = String::from("");
            let mut file =
                std::fs::File::open(dictionary_path).context("Unable to open dictionary file")?;
            file.read_to_string(&mut str)
                .context("Unable to read dictionary file")?;
        }

        let ending_symbols = ['.', ',', '!', '?'];
        let surrounding_symbols = ['[', ']', '{', '}', '(', ')', '"', '"', '\'', '\''];

        let mut rng = thread_rng();
        let mut str = str
            .split("\n")
            .map(|word| {
                let mut word = word.to_string();

                // uppercase
                if config.uppercase && rng.gen::<f64>() < config.uppercase_ratio {
                    let mut c = word.chars();
                    word = match c.next() {
                        None => String::new(),
                        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                    };
                }

                // numbers
                if config.numbers && rng.gen::<f64>() < config.numbers_ratio {
                    word = (0..word.len())
                        .map(|_| rng.gen_range(b'0'..=b'9') as char)
                        .collect();
                }

                // symbols
                if config.symbols && rng.gen::<f64>() < config.symbols_ratio {
                    word = match rng.gen::<usize>() % 2 {
                        0 => {
                            let index = rng.gen::<usize>() % ending_symbols.len();
                            format!("{}{}", word, ending_symbols[index])
                        }
                        1 => {
                            let index = (rng.gen::<usize>() % (surrounding_symbols.len() / 2)) * 2;
                            format!(
                                "{}{}{}",
                                surrounding_symbols[index],
                                word,
                                surrounding_symbols[index + 1]
                            )
                        }
                        _ => word.to_string(),
                    }
                }
                word
            })
            .collect::<Vec<_>>();
        str.shuffle(&mut rng);
        let str = str.join(" ").trim().to_string();

        Ok(Self { str })
    }
}

/// extracted to trait to create mock with `mockall` crate
#[automock]
pub trait ExpectedInputInterface {
    fn get_string(&self, len: usize) -> String;
}

impl ExpectedInputInterface for ExpectedInput {
    /// Cuts string saved in ExpectedInput at specified length instance and returns it
    ///
    /// If string is shorter than the specified length it repeats it enough times for it to be long
    /// enough.
    fn get_string(&self, len: usize) -> String {
        let s = self.str.clone() + " ";
        let s = s.repeat((len / s.chars().count()) as usize + 1);
        let (s, _) = split_by_char_index(&s, len);

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::color_scheme::ColorScheme;
    use std::{io::Write, time::Duration};

    use super::*;

    #[test]
    fn new_expected_input_should_correctly_convert_to_str() {
        let config = Config::default();
        let expected_input = ExpectedInput::new(&config).expect("unable to create expected input");

        assert_eq!(expected_input.get_string(12).chars().count(), 12);
    }

    #[test]
    fn should_read_file() {
        let mut config_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
        config_file
            .write_all(r#"halo"#.as_bytes())
            .expect("Unable to write to temp file");
        let config = Config {
            duration: Duration::from_secs(30),
            numbers: false,
            numbers_ratio: 0.05,
            symbols: false,
            symbols_ratio: 0.10,
            dictionary_path: Some(config_file.path().to_path_buf()),
            uppercase: false,
            uppercase_ratio: 0.45,
            colors: ColorScheme::default(),
            save_results: false,
            results_path: None,
        };

        let expected_input = ExpectedInput::new(&config).expect("unable to create expected input");

        assert_eq!(expected_input.get_string(4), "halo");
    }

    #[test]
    fn should_trim_string_to_match_len() {
        let expected_input = ExpectedInput {
            str: "abcdef".to_string(),
        };

        assert_eq!(expected_input.get_string(3), "abc");
    }

    #[test]
    fn should_repeat_string_if_len_is_too_big() {
        let expected_input = ExpectedInput {
            str: "abc".to_string(),
        };

        assert_eq!(expected_input.get_string(11), "abc abc abc");
    }

    #[test]
    fn should_work_with_non_ascii_chars() {
        let expected_input = ExpectedInput {
            str: "Բարեւ Ձեզ".to_string(),
        };

        assert_eq!(expected_input.get_string(5), "Բարեւ");
    }
}
