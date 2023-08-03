use std::io::Read;

use crate::config::Config;

use mockall::automock;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct ExpectedInput {
    str: String,
}

impl ExpectedInput {
    #[cfg(feature = "ci")]
    pub fn new(_config: &Config) -> Self {
        Self {
            str: "hello world".to_string(),
        }
    }

    #[cfg(not(feature = "ci"))]
    pub fn new(config: &Config) -> Self {
        let mut file =
            std::fs::File::open(config.dictionary_path.clone()).expect("Unable to open file");
        let mut s = String::new();
        file.read_to_string(&mut s).expect("Unable to read file");

        let mut s = s.split("\n").collect::<Vec<&str>>();
        let mut rng = thread_rng();
        s.shuffle(&mut rng);
        let s = s.join(" ").trim().to_string();

        Self { str: s }
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
    use std::io::Write;

    use super::*;

    #[test]
    fn new_expected_input_should_correctly_convert_to_str() {
        let config = Config::default();
        let expected_input = ExpectedInput::new(&config);

        assert_eq!(expected_input.get_string(12).len(), 12);
    }

    #[cfg(not(feature = "ci"))]
    #[test]
    fn should_read_file() {
        let mut config_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
        config_file
            .write_all(r#"halo"#.as_bytes())
            .expect("Unable to write to temp file");
        let config = Config {
            duration: 10,
            numbers: true,
            dictionary_path: config_file.path().to_path_buf(),
        };

        let expected_input = ExpectedInput::new(&config);

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
