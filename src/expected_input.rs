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

use random_str as random;

/// Struct used by runner to hold generate the text used for validation and as a placeholder
#[derive(Debug)]
pub struct ExpectedInput {
    str: String,
}

impl ExpectedInput {
    /// Create new struct instance by reading the dictionary file
    ///
    /// After reading the file shuffle its content
    /// then replace some words with numbers if specified in config
    /// then save one long string to memory
    pub fn new(config: &Config) -> Result<Self, anyhow::Error> {
        let mut str = dictionary::WORDS.to_string();
        if let Some(dictionary_path) = &config.dictionary_path {
            str = String::from("");
            let mut file =
                std::fs::File::open(dictionary_path).context("Unable to open dictionary file")?;
            file.read_to_string(&mut str)
                .context("Unable to read dictionary file")?;
        }

        let mut rng = thread_rng();
        let mut str_vec = str.split("\n").collect::<Vec<&str>>();
        let mut string_vec: Vec<String> = str_vec.iter().map(|s| s.to_string()).collect();
        str_vec.shuffle(&mut rng);

        // creating a pointer which points to where the words starts in the vector to help with uppercase words since we replace
        // words in the beginning with numbers when numbers are enabled.
        let mut words_start_pos: usize = 0;

        if config.numbers == true {
            words_start_pos =
                replace_words_with_numbers(&mut string_vec, &mut rng, config.numbers_ratio);
            str_vec = string_vec.iter().map(|s| s.as_str()).collect();
        }

        if config.uppercase == true {
            create_uppercase_words(&mut string_vec, words_start_pos, config.uppercase_ratio)
                .context("Unable to create uppercase words")?;
            str_vec = string_vec.iter().map(|s| s.as_str()).collect();
        }

        if config.symbols == true {
            replace_words_with_symbols(
                &mut string_vec,
                &mut rng,
                words_start_pos,
                config.symbols_ratio,
            )
            .context("Unable to create symbols")?;
            str_vec = string_vec.iter().map(|s| s.as_str()).collect();
        }

        str_vec.shuffle(&mut rng);

        let str = str_vec.join(" ").trim().to_string();

        Ok(Self { str })
    }
}

/// In given vector of words replace some of them
///
/// with words consisting only of numbers
/// number_ratio should be between [0, 1.0]
/// and tells how many percent of words should become numbers
fn replace_words_with_numbers(
    string_vec: &mut Vec<String>,
    rng: &mut rand::rngs::ThreadRng,
    numbers_ratio: f64,
) -> usize {
    let change_to_num_threshold = (numbers_ratio * string_vec.len() as f64).round() as usize;

    *string_vec = string_vec
        .iter()
        .enumerate()
        .map(|(index, word)| {
            if index < change_to_num_threshold {
                let random_digits: String = (0..word.len())
                    .map(|_| rng.gen_range(b'0'..=b'9') as char)
                    .collect();
                return random_digits;
            }
            return word.to_string();
        })
        .collect();

    return change_to_num_threshold - 1;
}

fn create_uppercase_words(
    string_vec: &mut Vec<String>,
    pos: usize,
    uppercase_ratio: f64,
) -> Result<()> {
    let num_uppercase_words = (uppercase_ratio * string_vec[pos..].len() as f64).round() as usize;
    for i in pos..pos + num_uppercase_words {
        if string_vec[i] != "" {
            let mut v: Vec<char> = string_vec[i].chars().collect();
            v[0] = v[0]
                .to_uppercase()
                .nth(0)
                .context("Unable to get first character of a word")?;
            let s: String = v.into_iter().collect();
            string_vec[i] = s;
        }
    }

    Ok(())
}

fn replace_words_with_symbols(
    string_vec: &mut Vec<String>,
    rng: &mut rand::rngs::ThreadRng,
    pos: usize,
    symbols_ratio: f64,
) -> Result<()> {
    let num_symbols = (symbols_ratio * string_vec[pos..].len() as f64).round() as usize;
    for i in pos..pos + num_symbols {
        if string_vec[i] != "" {
            let mut v: Vec<char> = string_vec[i].chars().collect();
            if v.len() >= 2 {
                //start from one to avoid overriding uppercase letters
                let index: usize = rng.gen_range(1..v.len());
                v[index] = random::get_symbol();
                let s: String = v.into_iter().collect();
                string_vec[i] = s;
            }
        }
    }

    Ok(())
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
            dictionary_path: Some(config_file.path().to_path_buf()),
            uppercase: false,
            uppercase_ratio: 0.45,
            colors: ColorScheme::default(),
            save_results: false,
            symbols: false,
            symbols_ratio: 0.2,
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
    fn should_replace_words_with_numbers() {
        let mut string_vec = vec![
            "item1".to_string(),
            "item2".to_string(),
            "item3".to_string(),
            "item4".to_string(),
            "item5".to_string(),
            "item6".to_string(),
            "item7".to_string(),
            "item8".to_string(),
        ];
        let mut rng = thread_rng();
        let numbers_ratio = 0.5;

        replace_words_with_numbers(&mut string_vec, &mut rng, numbers_ratio);

        let items_with_only_digits: Vec<&String> = string_vec
            .iter()
            .filter(|item| item.chars().all(|c| c.is_digit(10)))
            .collect();

        let change_to_num_threshold = (numbers_ratio * string_vec.len() as f64).round() as usize;
        assert_eq!(change_to_num_threshold, 4);
        assert_eq!(
            items_with_only_digits.len(),
            4,
            "At least 4 items contain only digits"
        );
    }
    #[test]
    fn should_work_with_non_ascii_chars() {
        let expected_input = ExpectedInput {
            str: "Բարեւ Ձեզ".to_string(),
        };

        assert_eq!(expected_input.get_string(5), "Բարեւ");
    }
}
