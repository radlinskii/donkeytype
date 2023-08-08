use anyhow::Context;
use mockall::automock;
use rand::{seq::SliceRandom, thread_rng};
use std::io::Read;

use crate::config::Config;

#[derive(Debug)]
pub struct ExpectedInput {
    str: String,
}

impl ExpectedInput {
    pub fn new(config: &Config) -> Result<Self, anyhow::Error> {
        let mut file = std::fs::File::open(config.dictionary_path.clone())
            .context("Unable to open dictionary file")?;
        let mut str = String::new();
        file.read_to_string(&mut str)
            .context("Unable to read dictionary file")?;

        let mut str = str.split("\n").collect::<Vec<&str>>();
        let mut rng = thread_rng();
        str.shuffle(&mut rng);
        let str = str.join(" ").trim().to_string();

        Ok(Self { str })
    }
}

#[automock]
pub trait ExpectedInputInterface {
    fn get_string(&self, len: usize) -> String;
}

impl ExpectedInputInterface for ExpectedInput {
    fn get_string(&self, len: usize) -> String {
        let s = self.str.clone() + " ";
        let s = s.repeat(len / s.len() + 1);
        let (s, _) = s.split_at(len);

        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, time::Duration};

    use super::*;

    #[test]
    fn new_expected_input_should_correctly_convert_to_str() {
        let config = Config::default();
        let expected_input = ExpectedInput::new(&config).expect("unable to create expected input");

        assert_eq!(expected_input.get_string(12).len(), 12);
    }

    #[test]
    fn should_read_file() {
        let mut config_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
        config_file
            .write_all(r#"halo"#.as_bytes())
            .expect("Unable to write to temp file");
        let config = Config {
            duration: Duration::from_secs(30),
            numbers: true,
            dictionary_path: config_file.path().to_path_buf(),
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
}
