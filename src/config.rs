use serde::{Deserialize, Serialize};

use clap::Parser;
use dirs;

use std::fs::{self};

const DEFAULT_CONFIG: Config = Config {
    duration: 30,
    numbers: false,
};

/// cli typing test
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// duration of the test in seconds
    #[arg(short, long)]
    duration: Option<u16>,

    /// Should test include numbers
    #[arg(short, long)]
    numbers: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
struct FileConfig {
    pub duration: u16,
    pub numbers: bool,
}

#[derive(Serialize, Debug)]
pub struct Config {
    pub duration: u16,
    pub numbers: bool,
}

impl Config {
    fn augment_config_with_config_file(config: &mut Config) {
        let home_dir = dirs::home_dir().unwrap();
        let config_file = home_dir
            .join(".config")
            .join("donkeytype")
            .join("donkeytype-config.json");

        if !config_file.exists() {
            fs::create_dir_all(config_file.parent().unwrap()).unwrap();
            serde_json::to_writer_pretty(
                fs::File::create(&config_file).expect("Unable to create file"),
                &DEFAULT_CONFIG,
            )
            .expect("Unable to write config file");

            println!(
                "Created config file with default values at {:?}",
                config_file
            );
        } else {
            let config_file_content = fs::read_to_string(config_file).expect("Unable to read file");

            let config_from_file: FileConfig =
                serde_json::from_str(&config_file_content).expect("Unable to parse config file");

            config.duration = config_from_file.duration;
            config.numbers = config_from_file.numbers;
        }
    }

    fn augment_config_with_args(config: &mut Config) {
        let args = Args::parse();

        if let Some(numbers_flag) = args.numbers {
            config.numbers = numbers_flag;
        }
        if let Some(duration) = args.duration {
            config.duration = duration;
        }
    }

    pub fn new() -> Config {
        let config = {
            let mut config = DEFAULT_CONFIG;

            Self::augment_config_with_config_file(&mut config);
            Self::augment_config_with_args(&mut config);

            config
        };

        config
    }
}
