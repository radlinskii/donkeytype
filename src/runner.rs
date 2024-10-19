//! Test runner module. Controls the test flow, checks input, and returns results.
//!
//! This is the main module that is orchestrating the flow of the test.
//! It prints the expected input as placeholder, then it reads user input and reacts to it.
//! When program is started `Normal` mode is turned on.
//! To go to `Editing` mode user needs to press `e`.
//! To go to `Normal` mode from `Editing` mode, and effectively pause the test, press `<Esc>`.
//!
//! When a test is started it checks the user input
//! and prints it to indicate valid characters and mistakes.
//! After the `duration` (amount of seconds) specified in config has passed the test is finished.
//! And test statistics are returned from the runner.

use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use mockall::automock;
use std::time::{Duration, Instant};

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Paragraph, Widget, Wrap},
    Frame, Terminal,
};

use crate::config::Config;
use crate::expected_input::ExpectedInputInterface;
use crate::helpers::split_by_char_index;
use crate::test_results::{Stats, TestResults};

/// To switch from Normal to Editing press `e`.
/// To switch from Editing to Normal press `<Esc>`.
enum InputMode {
    Normal,
    Editing,
}

/// Struct that runs and controls the test.
pub struct Runner {
    input: String,
    input_mode: InputMode,
    config: Config,
    expected_input: Box<dyn ExpectedInputInterface>,
    raw_mistakes_count: u64,
    raw_valid_characters_count: u64,
    is_started: bool,
}

impl Runner {
    /// Create new test runner instance
    pub fn new(config: Config, expected_input: impl ExpectedInputInterface + 'static) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            config,
            expected_input: Box::new(expected_input),
            raw_mistakes_count: 0,
            raw_valid_characters_count: 0,
            is_started: false,
        }
    }

    /// Removes the last word from user input
    fn remove_last_word(&mut self) {
        let mut words = self.input.split_whitespace().collect::<Vec<&str>>();
        words.pop();

        self.input = words.join(" ");

        if !self.input.is_empty() {
            self.input.push(' ');
        }
    }

    /// Method that runs the test.
    ///
    /// It renders the application using the `tui` crate and reacts to user input.
    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<TestResults> {
        let mut start_time = Instant::now();
        let mut pause_time = Instant::now();
        let tick_rate = Duration::from_secs(1);
        let mut last_tick = Instant::now();

        loop {
            if let InputMode::Editing = self.input_mode {
                if self.is_started && start_time.elapsed() >= self.config.duration {
                    return Ok(TestResults::new(
                        self.get_stats(),
                        self.config.clone(),
                        true,
                    ));
                }
            }

            let time_left = match self.input_mode {
                InputMode::Normal => match self.is_started {
                    false => self.config.duration,
                    true => self
                        .config
                        .duration
                        .checked_sub(
                            start_time
                                .elapsed()
                                .checked_sub(pause_time.elapsed())
                                .unwrap_or(Duration::from_secs(0)),
                        )
                        .unwrap_or(Duration::from_secs(0)),
                },
                InputMode::Editing => self
                    .config
                    .duration
                    .checked_sub(start_time.elapsed())
                    .unwrap_or(Duration::from_secs(0)),
            };

            terminal
                .draw(|f: &mut Frame<B>| {
                    let mut frame_wrapper = FrameWrapper::new(f);
                    self.render(&mut frame_wrapper, time_left.as_secs());
                })
                .context("Unable to draw in terminal")?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).context("Unable to poll for event")? {
                if let Event::Key(key) = event::read().context("Unable to read event")? {
                    if key.kind == KeyEventKind::Press {
                        match self.input_mode {
                            InputMode::Normal => match key.code {
                                KeyCode::Char('s') => {
                                    start_time = if self.is_started {
                                        start_time + pause_time.elapsed()
                                    } else {
                                        Instant::now()
                                    };
                                    self.is_started = true;
                                    self.input_mode = InputMode::Editing;
                                }
                                KeyCode::Char('q') => {
                                    // todo return canceled test error and handle it in main
                                    return Ok(TestResults::new(
                                        Stats::default(),
                                        self.config.clone(),
                                        false,
                                    ));
                                }
                                _ => {}
                            },
                            InputMode::Editing => match key.code {
                                // Crossterm returns `ctrl+w` or ``ctrl+h` when `ctrl+backspace` is pressed
                                // see: https://github.com/crossterm-rs/crossterm/issues/504
                                KeyCode::Char('h') | KeyCode::Char('w')
                                    if key.modifiers.contains(KeyModifiers::CONTROL) =>
                                {
                                    self.remove_last_word();
                                }
                                KeyCode::Char(c) => {
                                    self.input.push(c);

                                    let expected_input = self
                                        .expected_input
                                        .get_string(self.input.len())
                                        .chars()
                                        .collect::<Vec<char>>();

                                    let is_correct =
                                        self.input.chars().last() == expected_input.last().copied();

                                    if !is_correct {
                                        self.raw_mistakes_count += 1;
                                    } else {
                                        self.raw_valid_characters_count += 1;
                                    }
                                }
                                KeyCode::Backspace
                                    if key.modifiers.contains(KeyModifiers::ALT)
                                        | key.modifiers.contains(KeyModifiers::CONTROL) =>
                                {
                                    self.remove_last_word();
                                }
                                KeyCode::Backspace => {
                                    self.input.pop();
                                }
                                KeyCode::Esc => {
                                    pause_time = Instant::now();
                                    self.input_mode = InputMode::Normal;
                                }
                                _ => {}
                            },
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    /// Render a frame with each visual elements of the program in terminal.
    ///
    /// There are two areas being rendered,
    /// info area - where help message and time remaining is rendered.
    /// and input area - where user input and expected input are displayed,
    pub fn render(&mut self, frame: &mut impl FrameWrapperInterface, time_left: u64) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)].as_ref())
            .split(frame.size());
        let info_area = areas[0];
        let input_area = areas[1];

        let frame_width: usize = frame.size().width as usize;
        let input_chars_count: usize = self.input.chars().count();
        let current_line_index = (input_chars_count / frame_width) as u16;
        let input_current_line_len = input_chars_count % frame_width;

        let expected_input_str = self
            .expected_input
            .get_string((current_line_index as usize + 2) * frame_width);

        let (expected_input_current_line, expected_input_following_lines) = split_by_char_index(
            &expected_input_str,
            ((current_line_index as usize) + 1) * frame_width,
        );

        let (expected_input_current_line_already_typed, expected_input_current_line_rest) =
            split_by_char_index(&expected_input_current_line, input_chars_count);

        let expected_input_str = expected_input_current_line_already_typed.to_string()
            + expected_input_current_line_rest
            + expected_input_following_lines;

        self.print_input(frame, &expected_input_str, input_area, frame_width);

        self.print_block_of_text(
            frame,
            expected_input_current_line_rest.to_string(),
            Rect {
                x: input_area.x + input_current_line_len as u16,
                y: input_area.y + current_line_index,
                width: frame_width as u16 - input_current_line_len as u16,
                height: 1,
            },
            Color::Gray,
            false,
            false,
        );

        self.print_block_of_text(
            frame,
            expected_input_following_lines.to_string(),
            Rect {
                x: input_area.x,
                y: input_area.y + current_line_index + 1,
                height: input_area.height - current_line_index - 1,
                width: input_area.width,
            },
            Color::DarkGray,
            true,
            false,
        );

        self.move_cursor(
            frame,
            input_area,
            input_current_line_len,
            current_line_index,
        );

        let label = match time_left {
            1 => "second",
            _ => "seconds",
        };
        let time_left_message = format!("{} {label} left", time_left);

        self.print_block_of_text(
            frame,
            time_left_message,
            info_area,
            Color::Yellow,
            true,
            false,
        );

        let help_message = match self.input_mode {
            InputMode::Normal => match self.is_started {
                false => "press 's' to start the test, press 'q' to quit",
                true => "press 's' to unpause the test, press 'q' to quit",
            },
            InputMode::Editing => "press 'Esc' to pause the test",
        };
        self.print_block_of_text(
            frame,
            help_message.to_string(),
            info_area,
            Color::Yellow,
            true,
            true,
        )
    }

    /// Iterate over characters in user input
    /// and print them using different colors indicating if they are valid or wrong.
    fn print_input(
        &mut self,
        frame: &mut impl FrameWrapperInterface,
        expected_input: &str,
        input_area: Rect,
        frame_width: usize,
    ) {
        for ((input_char_index, input_char), expected_input_char) in
            self.input.chars().enumerate().zip(expected_input.chars())
        {
            let input: Paragraph<'_> = Paragraph::new(expected_input_char.to_string()).style(
                match input_char == expected_input_char {
                    true => Style::default()
                        .bg(self.config.colors.correct_match_bg)
                        .fg(self.config.colors.correct_match_fg),
                    false => Style::default()
                        .bg(self.config.colors.incorrect_match_bg)
                        .fg(self.config.colors.incorrect_match_fg),
                },
            );
            frame.render_widget(
                input,
                Rect {
                    x: (input_area.x + input_char_index as u16) % frame_width as u16,
                    y: input_area.y + input_char_index as u16 / frame_width as u16,
                    width: 1,
                    height: 1,
                },
            );
        }
    }

    /// Used for rendering text within given area and adjusted with given color.
    fn print_block_of_text(
        &self,
        frame: &mut impl FrameWrapperInterface,
        text_str: String,
        area: Rect,
        color: Color,
        wrap: bool,
        align_right: bool,
    ) {
        let mut text = Text::from(text_str);
        text.patch_style(Style::default().fg(color));
        let mut paragraph = Paragraph::new(text);

        if wrap {
            paragraph = paragraph.wrap(Wrap { trim: false });
        }

        if align_right {
            paragraph = paragraph.alignment(Alignment::Right);
        }

        frame.render_widget(paragraph, area);
    }

    /// Move the user cursor to place after the end of user input.
    fn move_cursor(
        &self,
        frame: &mut impl FrameWrapperInterface,
        area: Rect,
        input_current_line_len: usize,
        current_line_index: u16,
    ) {
        match self.input_mode {
            InputMode::Normal =>
                // Don't do anything, because `Frame` already hid the cursor
                {}

            InputMode::Editing => frame.set_cursor(
                area.x + input_current_line_len as u16,
                area.y + current_line_index,
            ),
        }
    }

    /// Calculate the statistics of the test and return them.
    ///
    /// WPM is number of valid characters that are in the input after the test has finished
    /// divided by 5, to get the "number of words typed", and divided by the duration of the test
    /// normalized to 60 seconds.
    /// This way WPM is only counted in valid characters, so each mistake that wasn't corrected is
    /// not taken into consideration when calculating it.
    ///
    /// `raw_valid_characters_count` is number of times when valid character was pressed.
    /// `raw_mistakes_count is number` of times when invalid character was pressed.
    /// `raw_typed_characters_count` is number of key presses that happened in `Editing` mode during
    /// the test.
    /// `raw_accuracy` is ratio of `raw_valid_characters_count` to `raw_typed_characters_count`.
    ///
    /// `valid_characters_count`is number of valid characters in the input after the test has
    /// finished, so if any corrections where made, it will consider the state of the input after
    /// them.
    /// `mistakes_count` is number of invalid characters in the input after the test has finished.
    /// `typed_characters_count` is number of characters in the input after the test has finished.
    /// `accuracy` is ratio of `valid_characters_count` to `typed_characters_count`.
    ///
    fn get_stats(&self) -> Stats {
        let typed_characters = self.input.chars();
        let typed_characters_count = typed_characters.clone().count();
        let expected_input_str = self.expected_input.get_string(typed_characters_count);
        let expected_characters = expected_input_str.chars();

        let mistakes_count = typed_characters
            .clone()
            .zip(expected_characters.clone())
            .filter(|(input_char, expected_input_char)| input_char != expected_input_char)
            .count() as u64;
        let valid_characters_count = typed_characters_count as u64 - mistakes_count;

        fn get_percentage(numerator: f64, denominator: f64) -> f64 {
            if denominator == 0.0 {
                return 0.0;
            }

            numerator / denominator * 100.0
        }

        Stats {
            wpm: valid_characters_count as f64 / 5.0 * 60.0 / self.config.duration.as_secs() as f64,

            raw_accuracy: get_percentage(
                self.raw_valid_characters_count as f64,
                (self.raw_valid_characters_count + self.raw_mistakes_count) as f64,
            ),
            raw_valid_characters_count: self.raw_valid_characters_count,
            raw_mistakes_count: self.raw_mistakes_count,
            raw_typed_characters_count: self.raw_valid_characters_count + self.raw_mistakes_count,

            accuracy: get_percentage(
                (typed_characters_count - mistakes_count as usize) as f64,
                typed_characters_count as f64,
            ),
            valid_characters_count,
            mistakes_count,
            typed_characters_count: typed_characters_count as u64,
        }
    }
}

/// Used for generating mocks using `mockall` crate
#[automock]
pub trait FrameWrapperInterface {
    fn render_widget<W: Widget + 'static>(&mut self, widget: W, area: Rect);
    fn set_cursor(&mut self, x: u16, y: u16);
    fn size(&self) -> Rect;
}

/// Used for generating mocks using `mockall` crate
pub struct FrameWrapper<'a, 'b, B: Backend> {
    frame: &'a mut Frame<'b, B>,
}

impl<'a, 'b, B: Backend> FrameWrapper<'a, 'b, B> {
    pub fn new(frame: &'a mut Frame<'b, B>) -> Self {
        FrameWrapper { frame }
    }
}

impl<'a, 'b, B: Backend> FrameWrapperInterface for FrameWrapper<'a, 'b, B> {
    fn render_widget<T: Widget + 'static>(&mut self, widget: T, area: Rect) {
        self.frame.render_widget(widget, area);
    }

    fn set_cursor(&mut self, x: u16, y: u16) {
        self.frame.set_cursor(x, y);
    }

    fn size(&self) -> Rect {
        self.frame.size()
    }
}

#[cfg(test)]
mod test {
    use mockall::predicate;

    use crate::expected_input::{ExpectedInput, MockExpectedInputInterface};
    use ratatui::{backend::TestBackend, buffer::Buffer};
    use std::io::Write;

    use super::*;

    fn get_config(dictionary: Vec<&str>) -> (Config, tempfile::NamedTempFile) {
        let mut config_file = tempfile::NamedTempFile::new().expect("Unable to create temp file");
        config_file
            .write_all(dictionary.join(" ").as_bytes())
            .expect("Unable to write to temp file");

        (
            Config {
                dictionary_path: Some(config_file.path().to_path_buf()),
                ..Config::default()
            },
            config_file, // It keeps tmp file while test is running
        )
    }

    /// Tests the [`Runner`] widget against the expected [`Buffer`] by rendering it onto an equal
    /// area and comparing the rendered and expected content.
    fn test_runner(
        runner: &mut Runner,
        expected: Buffer,
        callback: fn(frame: &mut FrameWrapper<'_, '_, TestBackend>, runner: &mut Runner) -> (),
    ) {
        let backend = TestBackend::new(expected.area.width, expected.area.height);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|f| {
                let mut frame_wrapper = FrameWrapper::new(f);
                callback(&mut frame_wrapper, runner);
            })
            .unwrap();

        terminal.backend().assert_buffer(&expected);
    }

    fn create_buffer(rect: Rect, lines: Vec<Vec<(&str, Color)>>) -> Buffer {
        let mut buffer = Buffer::empty(rect);
        for (y, line) in lines.iter().enumerate() {
            let mut x: usize = 0;
            for (sub_string, color) in line.iter() {
                buffer.set_string(x as u16, y as u16, sub_string, Style::default().fg(*color));
                x += sub_string.chars().count();
            }
        }

        buffer
    }

    #[test]
    fn should_render_single_line_input() {
        let config = Config::default();
        let time_left = config.duration;

        let mut expected_input = MockExpectedInputInterface::default();

        expected_input
            .expect_get_string()
            .with(predicate::eq(2 * 50))
            .return_const("foobarbaaz".repeat(10));

        let mut runner = Runner::new(config, expected_input);

        runner.input_mode = InputMode::Editing;
        runner.input = "foo".to_string();

        let mut frame = MockFrameWrapperInterface::default();

        frame.expect_size().times(2).return_const(Rect {
            x: 39,
            y: 1,
            width: 50,
            height: 1,
        });

        frame
            .expect_render_widget::<Paragraph>()
            .times(7)
            .return_const(());

        frame
            .expect_set_cursor()
            .with(predicate::eq(42), predicate::eq(1))
            .times(1)
            .return_const(());

        runner.render(&mut frame, time_left.as_secs());
    }

    #[test]
    fn should_render_multi_line_input() {
        let (mut config, _config_file) = get_config(vec!["foobarbazquxaboba"]);
        config.duration = Duration::from_secs(30);
        let expected_input = ExpectedInput::new(&config).expect("unable to create expected input");

        let mut runner = Runner::new(config, expected_input);
        runner.input_mode = InputMode::Editing;
        runner.input = "foobar".to_string();

        let buffer = create_buffer(
            Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 3,
            },
            vec![
                vec![
                    ("30 seconds left", Color::Yellow),
                    ("      ", Color::Reset),
                    ("press 'Esc' to pause the test", Color::Yellow),
                ],
                vec![
                    ("foobar", Color::Green),
                    ("bazquxaboba foobarbazquxaboba foobarbazquxab", Color::Gray),
                ],
                vec![(
                    "oba foobarbazquxaboba foobarbazquxaboba foobarbazq",
                    Color::DarkGray,
                )],
            ],
        );

        test_runner(&mut runner, buffer, |frame, runner| {
            runner.render(frame, 30);
        });
    }

    #[test]
    fn should_print_input() {
        let (config, _config_file) = get_config(vec!["foo"]);
        let expected_input = ExpectedInput::new(&config).expect("unable to create expected input");
        let mut runner = Runner::new(config, expected_input);

        runner.input = "foo".to_string();

        let buffer = create_buffer(
            Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 1,
            },
            vec![vec![("foo", Color::Green)]],
        );

        test_runner(&mut runner, buffer, |frame, runner| {
            runner.print_input(
                frame,
                "foo",
                Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 1,
                },
                50,
            );
        });
    }

    #[test]
    fn should_print_block_of_text() {
        let (config, _config_file) = get_config(vec!["foo"]);
        let expected_input = ExpectedInput::new(&config).expect("unable to create expected input");
        let mut runner = Runner::new(config, expected_input);

        let buffer = create_buffer(
            Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 1,
            },
            vec![vec![("foo", Color::Gray)]],
        );

        test_runner(&mut runner, buffer, |frame, runner| {
            runner.print_block_of_text(
                frame,
                "foo".to_string(),
                Rect {
                    x: 0,
                    y: 0,
                    width: 50,
                    height: 1,
                },
                Color::Gray,
                false,
                false,
            );
        });
    }

    #[test]
    fn should_not_move_cursor_in_normal_mode() {
        let config = Config::default();
        let expected_input = MockExpectedInputInterface::default();
        let runner = Runner::new(config, expected_input);

        let mut frame = MockFrameWrapperInterface::default();

        frame.expect_set_cursor().times(0).return_const(());

        let area = Rect {
            x: 40,
            y: 11,
            width: 50,
            height: 1,
        };
        let input_current_line_len = 2;
        let current_line_index = 16;

        runner.move_cursor(&mut frame, area, input_current_line_len, current_line_index)
    }

    #[test]
    fn should_move_cursor_in_editing_mode() {
        let config = Config::default();

        let expected_input = MockExpectedInputInterface::default();
        let mut runner = Runner::new(config, expected_input);

        runner.input_mode = InputMode::Editing;

        let mut frame = MockFrameWrapperInterface::default();

        frame
            .expect_set_cursor()
            .with(predicate::eq(42), predicate::eq(27))
            .times(1)
            .return_const(());

        let area = Rect {
            x: 40,
            y: 11,
            width: 50,
            height: 1,
        };
        let input_current_line_len = 2;
        let current_line_index = 16;

        runner.move_cursor(&mut frame, area, input_current_line_len, current_line_index)
    }
}
