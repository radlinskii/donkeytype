use crate::Args;
use mockall::*;
use serde::{Deserialize, Serialize};

use std::{fs, io::Read, path::PathBuf};

pub struct Config {
    pub duration: u16,
    pub numbers: bool,
    pub dictionary_path: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
    pub duration: Option<u16>,
    pub numbers: Option<bool>,
    pub dictionary_path: Option<String>,
}

#[automock]
impl Config {
    #[allow(dead_code)]
    pub fn default() -> Self {
        Self {
            duration: 30,
            numbers: false,
            dictionary_path: PathBuf::from("src/dict/words.txt"),
        }
    }

    pub fn new(args: Args, config_file_path: PathBuf) -> Self {
        let config = {
            let mut config = Self::default();

            let config_file = Self::open_config_file_if_exists(config_file_path.clone());
            if let Some(config_file) = config_file {
                Self::augment_config_with_config_file(&mut config, config_file);
            }
            Self::augment_config_with_args(&mut config, args);

            config
        };

        config
    }

    fn augment_config_with_config_file(config: &mut Config, mut config_file: fs::File) {
        if config_file.metadata().is_ok() {
            let mut config_file_content = String::new();
            config_file
                .read_to_string(&mut config_file_content)
                .expect("Unable to read file");

            let config_from_file: ConfigFile =
                serde_json::from_str(&config_file_content).expect("Unable to parse config file");

            if let Some(duration) = config_from_file.duration {
                config.duration = duration;
            }

            if let Some(numbers) = config_from_file.numbers {
                config.numbers = numbers;
            }

            if let Some(dictionary_path) = config_from_file.dictionary_path {
                config.dictionary_path = PathBuf::from(dictionary_path);
            }
        }
    }

    fn open_config_file_if_exists(config_file: PathBuf) -> Option<fs::File> {
        if config_file.exists() {
            let config_file = fs::File::open(config_file).expect("Unable to open config file");
            return Some(config_file);
        }

        return None;
    }

    fn augment_config_with_args(config: &mut Config, args: Args) {
        if let Some(numbers_flag) = args.numbers {
            config.numbers = numbers_flag;
        }
        if let Some(duration) = args.duration {
            config.duration = duration;
        }
        if let Some(dictionary_path) = args.dictionary_path {
            config.dictionary_path = PathBuf::from(dictionary_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;

    #[test]
    fn should_create_default_values() {
        let config = Config::default();

        assert_eq!(config.duration, 30);
        assert_eq!(config.numbers, false);
    }

    #[test]
    fn should_create_new_with_default_values() {
        let args = Args {
            duration: None,
            numbers: None,
            dictionary_path: None,
        };
        let config = Config::new(args, PathBuf::new());

        assert_eq!(config.duration, 30);
        assert_eq!(config.numbers, false);
    }

    #[test]
    fn should_create_new_config_with_config_file_values() {
        let mut config_file = tempfile::NamedTempFile::new().unwrap();
        config_file
            .write_all(r#"{"duration": 10, "numbers": true }"#.as_bytes())
            .unwrap();

        let args = Args {
            duration: None,
            numbers: None,
            dictionary_path: None,
        };
        let config = Config::new(args, config_file.path().to_path_buf());

        assert_eq!(config.duration, 10);
        assert_eq!(config.numbers, true);
    }

    #[test]
    fn should_create_new_with_argument_values() {
        let args = Args {
            duration: Some(10),
            numbers: Some(true),
            dictionary_path: None,
        };
        let config = Config::new(args, PathBuf::new());

        assert_eq!(config.duration, 10);
        assert_eq!(config.numbers, true);
    }

    #[test]
    fn args_should_take_precedence_over_config_file() {
        let mut config_file = tempfile::NamedTempFile::new().unwrap();
        config_file
            .write_all(r#"{"duration": 10, "numbers": true }"#.as_bytes())
            .unwrap();

        let args = Args {
            duration: Some(20),
            numbers: Some(false),
            dictionary_path: Some(String::from("/etc/dict/words")),
        };
        let config = Config::new(args, config_file.path().to_path_buf());

        assert_eq!(config.duration, 20);
        assert_eq!(config.numbers, false);
        assert_eq!(config.dictionary_path, PathBuf::from("/etc/dict/words"));
    }
}
