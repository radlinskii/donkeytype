mod args;
mod config;
mod expected_input;
mod runner;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

use args::Args;
use config::Config;
use expected_input::ExpectedInput;
use runner::Runner;

fn main() -> anyhow::Result<()> {
    let config_file_path = dirs::home_dir()
        .context("Unable to get home directory")?
        .join(".config")
        .join("donkeytype")
        .join("donkeytype-config.json");

    let args = Args::parse();
    let config = Config::new(args, config_file_path).context("Unable to create config")?;
    let expected_input = ExpectedInput::new(&config).context("Unable to create expected input")?;

    let mut terminal = configure_terminal().context("Unable to configure terminal")?;

    let mut app = Runner::new(config, expected_input);
    let res = app.run(&mut terminal);

    restore_terminal(terminal).context("Unable to restore terminal")?;

    if let Err(err) = res {
        println!("{:?}", err)
    } else if let Ok(score) = res {
        println!("{:?}", score);
    }

    Ok(())
}

fn configure_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, anyhow::Error> {
    enable_raw_mode().context("Unable to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("Unable to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Unable to create terminal")?;

    Ok(terminal)
}

fn restore_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), anyhow::Error> {
    disable_raw_mode().context("Unable to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Unable to leave alternate screen")?;
    terminal.show_cursor().context("Unable to show cursor")?;

    Ok(())
}
