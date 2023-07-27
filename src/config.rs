use crate::Args;
use mockall::*;
use serde::{Deserialize, Serialize};

use std::{fs, io::Read, path::PathBuf};

const DEFAULT_CONFIG: Config = Config {
    duration: 30,
    numbers: false,
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub duration: u16,
    pub numbers: bool,
}

#[automock]
impl Config {
    pub fn new(args: Args, config_file_path: PathBuf) -> Config {
        let config = {
            let mut config = DEFAULT_CONFIG;

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

            let config_from_file: Config =
                serde_json::from_str(&config_file_content).expect("Unable to parse config file");

            config.duration = config_from_file.duration;
            config.numbers = config_from_file.numbers;
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
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn should_create_new_with_default_values() {
        let args = Args {
            duration: None,
            numbers: None,
        };
        let config = Config::new(args, PathBuf::new());

        assert_eq!(DEFAULT_CONFIG.duration, 30);
        assert_eq!(config.duration, DEFAULT_CONFIG.duration);
        assert_eq!(DEFAULT_CONFIG.numbers, false);
        assert_eq!(config.numbers, DEFAULT_CONFIG.numbers);
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
        };
        let config = Config::new(args, config_file.path().to_path_buf());

        assert_eq!(config.duration, 20);
        assert_eq!(config.numbers, false);
    }
}
