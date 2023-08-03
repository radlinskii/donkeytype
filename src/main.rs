mod args;
mod config;
mod expected_input;
mod runner;

use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
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

    let mut _terminal = prepare_terminal()?;

    println!("press 'e' to start editing, press 'q' to quit");

    let mut _app = Runner::new(config, expected_input);
    // let res = app.run(&mut terminal);

    // restore_terminal(terminal)?;

    // if let Err(err) = res {
    //     println!("{:?}", err)
    // }

    Ok(())
}

fn prepare_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
