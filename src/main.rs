//! a very minimalistic cli typing test.
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
//! ## Configuration
//!
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
//!
//! To get all the available options run
//!
//! ```shell
//! cargo run -- --help
//! ```

mod args;
mod config;
mod expected_input;
mod runner;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

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
/// - prints and handles test results
fn main() -> anyhow::Result<()> {
    let config_file_path = dirs::home_dir()
        .context("Unable to get home directory")?
        .join(".config")
        .join("donkeytype")
        .join("donkeytype-config.json");

    let args = Args::parse();
    let config = Config::new(args, config_file_path).context("Unable to create config")?;
    let duration = config.duration.as_secs();
    let expected_input = ExpectedInput::new(&config).context("Unable to create expected input")?;
    let mut terminal = configure_terminal().context("Unable to configure terminal")?;

    let mut app = Runner::new(config, expected_input);
    let res = app.run(&mut terminal);

    restore_terminal(terminal).context("Unable to restore terminal")?;

    match res {
        Ok(stats) => {
            println!("WPM: {:.2}", stats.wpm);
            println!("Raw accuracy: {:.2}%", stats.raw_accuracy);
            println!("Raw valid characters: {}", stats.raw_valid_characters_count);
            println!("Raw mistakes: {}", stats.raw_mistakes_count);
            println!("Raw characters typed: {}", stats.raw_typed_characters_count);
            println!("Accuracy after corrections: {:.2}%", stats.accuracy);
            println!(
                "Valid characters after corrections: {}",
                stats.valid_characters_count
            );
            println!("Mistakes after corrections: {}", stats.mistakes_count);
            println!(
                "Characters typed after corrections: {}",
                stats.typed_characters_count
            );
            println!("Time: {} seconds", duration);

            Ok(())
        }
        Err(err) => {
            println!("Error: {:?}", err);

            Err(err)
        }
    }
}

/// prepares terminal window for rendering using tui
fn configure_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, anyhow::Error> {
    enable_raw_mode().context("Unable to enable raw mode")?;
    let mut stdout = io::stdout();
    if matches!(terminal::supports_keyboard_enhancement(), Ok(true)) {
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
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), anyhow::Error> {
    disable_raw_mode().context("Unable to disable raw mode")?;
    if matches!(terminal::supports_keyboard_enhancement(), Ok(true)) {
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
    use tui::{backend::TestBackend, buffer::Buffer, Frame, Terminal};

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
        let config_file_path = dirs::home_dir()
            .context("Unable to get home directory")?
            .join(".config")
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
