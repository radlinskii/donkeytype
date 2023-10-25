//! Donkeytype - a very minimalistic cli typing test.
//!
//! It shows an expected input in the terminal window of the user and measures how many characters
//! user types in the given amount of time.
//! WPM (words per minute) score is calculated as amount of typed characters divided by 5 (word)
//! divided by the duration normalized to 60 seconds (minute).
//!
//! ## Usage
//!
//! To start a test simply run
//!
//! ```sh
//! cargo run
//! ```
//!
//!
//! ## Configuration
//!
//! Default options of configuration are:
//!
//! | name              | default value                | type in JSON | description                                                                                                                                                                                                           |
//! | ----------------- | ---------------------------- | ------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
//! | `duration`        | `30`                         | number       | duration of the test in seconds                                                                                                                                                                                       |
//! | `numbers`         | `false`                      | boolean      | flag indicating if numbers should be inserted in expected input                                                                                                                                                       |
//! | `numbers_ratio`   | `0.05` (if numbers=true)     | number       | ratio for putting numbers in the test                                                                                                                                                                                 |
//! | `symbols`         | `false`                      | boolean      | flag indicating if symbols should be inserted in expected input                                                                                                                                                       |
//! | `symbols_ratio`   | `0.10` (if symbols=true)     | number       | ratio for putting symbols in the test                                                                                                                                                                                 |
//! | `uppercase`       | `false`                      | boolean      | flag indicating if uppercase letters should be inserted in expected input                                                                                                                                             |
//! | `uppercase_ratio` | `0.15` (if uppercase=true)   | boolean      | ratio for putting uppercase letters in test                                                                                                                                                                           |
//! | `dictionary_path` |  `None` (builtin dictionary) | string       | path to file with dictionary words to sample from while creating test's expected input                                                                                                                                |
//! | `save_results`    | `true`                       | boolean      | flag indicating if results should be saved to a file  ( `~/.local/share/donkeytype/donkeytype-results.csv`  on Linux and MacOS, and `C:\Users\{Username}\AppData\Local\donkeytype\donkeytype-results.csv` on Windows) |
//!
//! NOTE: If provided `numbers_ratio` is not between `0` to `1.0`, default `numbers_ratio = 0.15` will be used. Same happens with `uppercase_ratio` and `symbols_ratio`.
//!
//! Configuration will grow when more features are added (_different modes_, _different languages_).
//!
//! You can provide this config by putting it in a config file in `~/.config/donkeytype/donkeytype-config.json`:
//!
//! ```json
//! {
//!     "duration": 60,
//!     "dictionary_path": "/usr/share/dict/words",
//!     "numbers": true,
//!     "numbers_ratio": 0.1,
//!     "uppercase": true,
//!     "uppercase_ratio": 0.3,
//!     "colors": {
//!         "correct_match_fg": "green",
//!         "correct_match_bg": "white",
//!         "incorrect_match_fg": "#ff00ff",
//!         "incorrect_match_bg": "#0f000f"
//!     }
//! }
//!
//! > Note: Providing config in a file supports passing custom color values.
//!
//! Apart from `colors` you can set configuration by passing options when running the program:
//!
//! ```shell
//! cargo run -- --duration 60 --dictionary-path "/usr/share/dict/words" --numbers true
//! --numbers-ratio 0.1 --uppercase true --uppercase-ratio 0.3 --save-results false
//! ```
//!
//! To get all the available options run
//!
//! ```shell
//! cargo run -- --help
//! ```

mod args;
mod color_scheme;
mod config;
mod dictionary;
mod expected_input;
mod helpers;
mod runner;
mod test_results;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::supports_keyboard_enhancement;
use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use test_results::{read_previous_results, render_results};

use args::Args;
use config::Config;
use expected_input::ExpectedInput;
use runner::Runner;

/// main entry to the program
/// - parses arguments,
/// - reads config
/// - creates expected input
/// - prepares terminal window
/// - starts the test
/// - restores terminal configuration
/// - if test was completed, prints the results and saves them.
fn main() -> Result<()> {
    let args = Args::parse();

    let mut terminal = configure_terminal().context("Unable to configure terminal")?;

    let res = match &args.history {
        Some(_) => handle_history_command(&mut terminal),
        None => handle_main_command(&mut terminal, args),
    };

    match res {
        Err(err) => {
            restore_terminal(&mut terminal).context("Unable to restore terminal")?;
            return Err(err);
        }
        Ok(_) => Ok(()),
    }
}

fn handle_history_command(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let records = read_previous_results().context("Unable to read history results")?;
    render_results(terminal, &records).context("Unable to render history results")?;
    restore_terminal(terminal).context("Unable to restore terminal")?;
    Ok(())
}

fn handle_main_command(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    args: Args,
) -> Result<()> {
    let config_file_path = if cfg!(target_os = "windows") {
        dirs::config_local_dir().context("Unable to get local config directory")?
    } else {
        dirs::home_dir()
            .context("Unable to get home directory")?
            .join(".config")
    }
    .join("donkeytype")
    .join("donkeytype-config.json");

    let config = Config::new(args, config_file_path).context("Unable to create config")?;
    let expected_input = ExpectedInput::new(&config).context("Unable to create expected input")?;

    let mut app = Runner::new(config, expected_input);
    let test_results = app.run(terminal).context("Error while running the test")?;

    if test_results.completed {
        test_results
            .render(terminal)
            .context("Unable to render test results")?;
        if test_results.save {
            test_results
                .save_to_file()
                .context("Unable to save results to file")?;
        }
        restore_terminal(terminal).context("Unable to restore terminal")?;
    } else {
        restore_terminal(terminal).context("Unable to restore terminal")?;
        println!("Test not finished.");
    }
    Ok(())
}

/// prepares terminal window for rendering using tui
fn configure_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, anyhow::Error> {
    enable_raw_mode().context("Unable to enable raw mode")?;
    let mut stdout = io::stdout();
    if matches!(supports_keyboard_enhancement(), Ok(true)) {
        execute!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
            )
        )
        .context("Unable to push keyboard enhancement flags")?;
    }

    execute!(stdout, EnterAlternateScreen).context("Unable to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend).context("Unable to create terminal")?;

    Ok(terminal)
}

/// restores terminal window configuration
fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), anyhow::Error> {
    disable_raw_mode().context("Unable to disable raw mode")?;
    if matches!(supports_keyboard_enhancement(), Ok(true)) {
        execute!(terminal.backend_mut(), PopKeyboardEnhancementFlags)
            .context("Unable to pop keyboard enhancement flags")?;
    }

    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("Unable to leave alternate screen")?;
    terminal.show_cursor().context("Unable to show cursor")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{io::Write, time::Instant};

    use anyhow::{Context, Result};
    use predicates::Predicate;
    use ratatui::{backend::TestBackend, buffer::Buffer, Frame, Terminal};

    use crate::{
        args::Args,
        config::Config,
        expected_input::ExpectedInput,
        runner::{FrameWrapper, Runner},
    };

    fn configure_terminal() -> Result<Terminal<TestBackend>, anyhow::Error> {
        let backend = TestBackend::new(400, 400);
        let terminal = Terminal::new(backend).context("Unable to create terminal")?;

        Ok(terminal)
    }

    fn extract_text_from_buffer(buffer: &Buffer) -> String {
        let mut text = String::new();

        for y in 0..buffer.area.height {
            for x in 0..buffer.area.height {
                let cell = buffer.get(x, y);
                text.push_str(&cell.symbol);
            }
            text.push('\n');
        }

        text
    }

    fn setup_terminal(args: Args) -> Result<(Config, ExpectedInput, Terminal<TestBackend>)> {
        let config_file_path = if cfg!(target_os = "windows") {
            dirs::config_local_dir().context("Unable to get local config directory")?
        } else {
            dirs::home_dir()
                .context("Unable to get home directory")?
                .join(".config")
        }
        .join("donkeytype")
        .join("donkeytype-config.json");

        let config = Config::new(args, config_file_path).context("Unable to create config")?;
        let expected_input =
            ExpectedInput::new(&config).context("Unable to create expected input")?;
        let terminal = configure_terminal().context("Unable to configure terminal")?;

        Ok((config, expected_input, terminal))
    }

    #[test]
    fn should_print_default_expected_input() -> Result<()> {
        let mut temp_dict_file =
            tempfile::NamedTempFile::new().expect("Unable to create temp file");
        temp_dict_file
            .write_all(r#"hello world"#.as_bytes())
            .expect("Unable to write to temp file");

        let args = Args {
            dictionary_path: Some(temp_dict_file.path().display().to_string()),
            duration: None,
            numbers: None,
            uppercase: None,
            uppercase_ratio: None,
            numbers_ratio: None,
            symbols: None,
            symbols_ratio: None,
            save_results: None,
            history: None,
        };

        let (config, expected_input, mut terminal) = setup_terminal(args)?;

        let mut app = Runner::new(config, expected_input);
        let start_time = Instant::now();

        terminal
            .draw(|f: &mut Frame<TestBackend>| {
                let mut frame_wrapper = FrameWrapper::new(f);
                app.render(&mut frame_wrapper, start_time.elapsed().as_secs());
            })
            .context("Unable to draw in terminal")?;

        let text = extract_text_from_buffer(terminal.backend().buffer());

        let predicate = predicates::str::contains("hello world");

        assert_eq!(true, predicate.eval(&text));

        Ok(())
    }

    #[test]
    fn should_print_help_message_for_normal_mode() -> Result<()> {
        let args = Args {
            dictionary_path: None,
            duration: None,
            uppercase: None,
            uppercase_ratio: None,
            numbers: None,
            numbers_ratio: None,
            symbols: None,
            symbols_ratio: None,
            save_results: None,
            history: None,
        };

        let (config, expected_input, mut terminal) = setup_terminal(args)?;

        let mut app = Runner::new(config, expected_input);
        let start_time = Instant::now();

        terminal
            .draw(|f: &mut Frame<TestBackend>| {
                let mut frame_wrapper = FrameWrapper::new(f);
                app.render(&mut frame_wrapper, start_time.elapsed().as_secs());
            })
            .context("Unable to draw in terminal")?;

        let text = extract_text_from_buffer(terminal.backend().buffer());

        let predicate = predicates::str::contains("press 'e' to start the test, press 'q' to quit");

        assert_eq!(true, predicate.eval(&text));

        Ok(())
    }
}
