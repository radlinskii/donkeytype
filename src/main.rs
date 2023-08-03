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

use args::Args;
use config::Config;
use expected_input::ExpectedInput;
use runner::Runner;

#[cfg(not(feature = "ci"))]
use tui::backend::CrosstermBackend;

#[cfg(feature = "ci")]
use tui::backend::TestBackend;

#[cfg(not(feature = "ci"))]
type TerminalBackend = CrosstermBackend<std::io::Stdout>;

#[cfg(feature = "ci")]
type TerminalBackend = TestBackend;

type Terminal = tui::terminal::Terminal<TerminalBackend>;

fn main() -> Result<(), Box<dyn Error>> {
    let config_file_path = dirs::home_dir()
        .expect("Unable to get home directory")
        .join(".config")
        .join("donkeytype")
        .join("donkeytype-config.json");

    let args = Args::parse();
    let config = Config::new(args, config_file_path);
    let expected_input = ExpectedInput::new(&config);

    let mut terminal = prepare_terminal()?;

    let mut app = Runner::new(config, expected_input);
    let res = app.run(&mut terminal);

    #[cfg(not(feature = "ci"))]
    restore_terminal(terminal)?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn prepare_terminal() -> Result<Terminal, Box<dyn Error>> {
    #[cfg(not(feature = "ci"))]
    let backend = fun_name();

    #[cfg(feature = "ci")]
    let backend = TestBackend::new(60, 60);

    let terminal = Terminal::new(backend).expect("Unable to create terminal");

    Ok(terminal)
}

fn fun_name() -> TerminalBackend {
    enable_raw_mode().expect("Unable to enable raw mode");
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).expect("Unable to enter alternate screen");

    CrosstermBackend::new(stdout)
}

#[cfg(not(feature = "ci"))]
fn restore_terminal(mut terminal: Terminal) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
