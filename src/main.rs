mod args;
mod config;
mod expected_input;
mod runner;

use clap::Parser;
use dirs;

use args::Args;
use config::Config;
use expected_input::ExpectedInput;
use runner::Runner;

fn main() {
    let args = Args::parse();

    let config_file_path = dirs::home_dir()
        .expect("Unable to get home directory")
        .join(".config")
        .join("donkeytype")
        .join("donkeytype-config.json");

    let config = Config::new(args, config_file_path);
    let expected_input = ExpectedInput::new(&config);
    let mut runner = Runner::new(config, expected_input);

    runner.run();
}
