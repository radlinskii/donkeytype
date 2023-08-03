mod args;
mod config;
mod expected_input;
mod runner;

use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{backend::CrosstermBackend, Terminal};

use args::Args;
use config::Config;
use expected_input::ExpectedInput;
use runner::Runner;

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = dirs::home_dir()
        .expect("Unable to get home directory")
        .join(".config")
        .join("donkeytype")
        .join("donkeytype-config.json");

    let args = Args::parse();
    let config = Config::new(args, config_file_path);
    let expected_input = ExpectedInput::new(&config);

    let mut terminal = prepare_terminal().expect("Unable to configure terminal");

    let mut app = Runner::new(config, expected_input);
    let res: Result<(), io::Error> = app.run(&mut terminal);

    restore_terminal(terminal).expect("Unable to restore terminal configuration");

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn prepare_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode().expect("Unable to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Unable to enter alternate screen");

    let backend = CrosstermBackend::new(stdout);

    let terminal = Terminal::new(backend).expect("Unable to create terminal");

    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode().expect("Unable to disable raw mode");
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .expect("Unable to leave alternate screen");
    terminal.show_cursor().expect("Unable to show cursor");

    Ok(())
}
