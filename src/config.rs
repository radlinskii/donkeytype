//! Module reading and parsing config
//!
//! Default options of configuration are:
//!
//! | name              | default value          | type in JSON | description                                                          |
//! | ----------------- | ---------------------- | ------------ | -------------------------------------------------------------------- |
//! | `duration`        | `30`                   | number       | duration of the test in seconds                                      |
//! | `numbers`         | `false`                | boolean      | flag indicating if numbers should be inserted in expected input      |
//! | `dictionary_path` | `"src/dict/words.txt"` | string       | dictionary words to sample from while creating test's expected input |
//!
//! Configuration will grow when more features are added (_different modes_, _different languages_, _configuring colors_).
//!
//! You can provide this config as options when running the program like so:
//!
//! ```shell
//! cargo run -- --duration 60 --dictionary-path "/usr/share/dict/words" --numbers true
//! ```
//!
//! or put them in a config file in `~/.config/donkeytype/donkeytype-config.json`:
//!
//! ```json
//! {
//!     "duration": 60,
//!     "dictionary_path": "/usr/share/dict/words",
//!     "numbers": false
//! }
//! ```

use anyhow::{Context, Result};
use mockall::*;
use serde::{Deserialize, Serialize};
use std::{fs, io::Read, path::PathBuf, time::Duration};

use crate::Args;

/// Main program configuration
#[derive(Debug)]
pub struct Config {
    pub duration: Duration,
    pub numbers: bool,
    pub dictionary_path: PathBuf,
}

/// Used by `serde` crate to parse config file into a rust struct
#[derive(Deserialize, Serialize, Debug)]
struct ConfigFile {
    pub duration: Option<u64>,
    pub numbers: Option<bool>,
    pub dictionary_path: Option<String>,
}

#[automock]
impl Config {
    /// Provide default values for configuration options
    pub fn default() -> Self {
        Self {
            duration: Duration::from_secs(30),
            numbers: false,
            dictionary_path: PathBuf::from("src/dict/words.txt"),
        }
    }

    /// Setup configuration
    ///
    /// Create config with default values
    /// then overwrite them with any values provided in config file
    /// then overwrite it again with any values provide as arguments to the program
    pub fn new(args: Args, config_file_path: PathBuf) -> Result<Self> {
        let config = {
            let mut config = Self::default();

            let config_file = open_config_file_if_exists(config_file_path.clone())
                .context("Unable to open config file")?;
            if let Some(config_file) = config_file {
                augment_config_with_config_file(&mut config, config_file)
                    .context("Unable to augment config with config file")?;
            }
            augment_config_with_args(&mut config, args);

            config
        };

        Ok(config)
    }
}

/// Overwrite provided config with options parsed from configuration file
fn augment_config_with_config_file(config: &mut Config, mut config_file: fs::File) -> Result<()> {
    if config_file.metadata().is_ok() {
        let mut config_file_content = String::new();
        config_file
            .read_to_string(&mut config_file_content)
            .context("Unable to read file")?;

        let config_from_file: ConfigFile =
            serde_json::from_str(&config_file_content).context("Unable to parse config file")?;

        if let Some(duration) = config_from_file.duration {
            config.duration = Duration::from_secs(duration);
        }

        if let Some(numbers) = config_from_file.numbers {
            config.numbers = numbers;
        }

        if let Some(dictionary_path) = config_from_file.dictionary_path {
            config.dictionary_path = PathBuf::from(dictionary_path);
        }
    }

    Ok(())
}

fn open_config_file_if_exists(config_file: PathBuf) -> Result<Option<fs::File>> {
    if config_file.exists() {
        let config_file = fs::File::open(config_file).context("Unable to open config file")?;
        return Ok(Some(config_file));
    }

    return Ok(None);
}

/// Overwrite provided config with values from args object
fn augment_config_with_args(config: &mut Config, args: Args) {
    if let Some(numbers_flag) = args.numbers {
        config.numbers = numbers_flag;
    }
    if let Some(duration) = args.duration {
        config.duration = Duration::from_secs(duration);
    }
    if let Some(dictionary_path) = args.dictionary_path {
        config.dictionary_path = PathBuf::from(dictionary_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;

    #[test]
    fn should_create_default_values() {
        let config = Config::default();

        assert_eq!(config.duration, Duration::from_secs(30));
        assert_eq!(config.numbers, false);
    }

    #[test]
    fn should_create_new_with_default_values() {
        let args = Args {
            duration: None,
            numbers: None,
            dictionary_path: None,
        };
        let config = Config::new(args, PathBuf::new()).expect("Unable to create config");

        assert_eq!(config.duration, Duration::from_secs(30));
        assert_eq!(config.numbers, false);
    }

    #[test]
    fn should_create_new_config_with_config_file_values() {
        let mut config_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
        config_file
            .write_all(r#"{"duration": 10, "numbers": true }"#.as_bytes())
            .expect("Unable to write to temp file");

        let args = Args {
            duration: None,
            numbers: None,
            dictionary_path: None,
        };
        let config =
            Config::new(args, config_file.path().to_path_buf()).expect("Unable to create config");

        assert_eq!(config.duration, Duration::from_secs(10));
        assert_eq!(config.numbers, true);
    }

    #[test]
    fn should_create_new_with_argument_values() {
        let args = Args {
            duration: Some(10),
            numbers: Some(true),
            dictionary_path: None,
        };
        let config = Config::new(args, PathBuf::new()).expect("Unable to create config");

        assert_eq!(config.duration, Duration::from_secs(10));
        assert_eq!(config.numbers, true);
    }

    #[test]
    fn args_should_take_precedence_over_config_file() {
        let mut config_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
        config_file
            .write_all(r#"{"duration": 10, "numbers": true }"#.as_bytes())
            .expect("Unable to write to temp file");

        let args = Args {
            duration: Some(20),
            numbers: Some(false),
            dictionary_path: Some(String::from("/etc/dict/words")),
        };
        let config =
            Config::new(args, config_file.path().to_path_buf()).expect("Unable to create config");

        assert_eq!(config.duration, Duration::from_secs(20));
        assert_eq!(config.numbers, false);
        assert_eq!(config.dictionary_path, PathBuf::from("/etc/dict/words"));
    }
}
