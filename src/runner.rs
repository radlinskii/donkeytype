use anyhow::{Context, Result};
use crossterm::event::{self, Event, KeyCode};
use mockall::automock;
use std::time::{Duration, Instant};
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Paragraph, Widget, Wrap},
    Frame, Terminal,
};

use crate::config::Config;
use crate::expected_input::ExpectedInputInterface;

enum InputMode {
    Normal,
    Editing,
}

pub struct Runner {
    input: String,
    input_mode: InputMode,
    config: Config,
    expected_input: Box<dyn ExpectedInputInterface>,
    raw_mistakes_count: u64,
    raw_valid_characters_count: u64,
}

#[derive(Debug)]
pub struct Stats {
    pub wpm: f64,
    pub raw_accuracy: f64,
    pub raw_valid_characters_count: u64,
    pub raw_mistakes_count: u64,
    pub raw_typed_characters_count: u64,
    pub accuracy: f64,
    pub valid_characters_count: u64,
    pub typed_characters_count: u64,
    pub mistakes_count: u64,
}

impl Runner {
    pub fn new(config: Config, expected_input: impl ExpectedInputInterface + 'static) -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            config,
            expected_input: Box::new(expected_input),
            raw_mistakes_count: 0,
            raw_valid_characters_count: 0,
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<Stats> {
        let mut start_time = Instant::now();
        let mut pause_time = Instant::now();
        let mut is_started = false;
        let tick_rate = Duration::from_secs(1);
        let mut last_tick = Instant::now();

        loop {
            if let InputMode::Editing = self.input_mode {
                if is_started && start_time.elapsed() >= self.config.duration {
                    return Ok(self.get_stats());
                }
            }

            terminal
                .draw(|f: &mut Frame<B>| {
                    let mut frame_wrapper = FrameWrapper::new(f);
                    self.render(&mut frame_wrapper, start_time.elapsed().as_secs());
                })
                .context("Unable to draw in terminal")?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).context("Unable to poll for event")? {
                if let Event::Key(key) = event::read().context("Unable to read event")? {
                    match self.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('e') => {
                                start_time = if is_started {
                                    start_time + pause_time.elapsed()
                                } else {
                                    Instant::now()
                                };
                                is_started = true;
                                self.input_mode = InputMode::Editing;
                            }
                            KeyCode::Char('q') => {
                                // todo return canceled test error and handle it in main
                                return Ok(Stats {
                                    wpm: 0.0,
                                    raw_accuracy: 0.0,
                                    raw_valid_characters_count: 0,
                                    raw_mistakes_count: 0,
                                    raw_typed_characters_count: 0,
                                    accuracy: 0.0,
                                    valid_characters_count: 0,
                                    mistakes_count: 0,
                                    typed_characters_count: 0,
                                });
                            }
                            _ => {}
                        },
                        InputMode::Editing => match key.code {
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

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn render(&mut self, frame: &mut impl FrameWrapperInterface, time_elapsed: u64) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(frame.size());
        let input_area = areas[0];
        let info_area = areas[1];

        let frame_width = frame.size().width as usize;
        let input_len = self.input.len();
        let current_line_index = (input_len / frame_width) as u16;
        let input_current_line_len = input_len % frame_width;

        let expected_input_str = self
            .expected_input
            .get_string((current_line_index as usize + 2) * frame_width);
        let (expected_input_current_line, expected_input_following_lines) =
            expected_input_str.split_at(((current_line_index as usize) + 1) * frame_width);
        let (expected_input_current_line_already_typed, expected_input_current_line_rest) =
            expected_input_current_line.split_at(input_len);
        let expected_input_str = expected_input_current_line_already_typed.to_string()
            + expected_input_current_line_rest
            + expected_input_following_lines;

        self.print_input(frame, expected_input_str, input_area, frame_width);

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

        let time_left = self
            .config
            .duration
            .checked_sub(Duration::from_secs(time_elapsed))
            .unwrap_or(Duration::from_secs(0));
        let label = match time_left.as_secs() {
            1 => "second",
            _ => "seconds",
        };
        let time_left_message = match self.input_mode {
            InputMode::Normal => String::new(),
            InputMode::Editing => format!("{} {label} left", time_left.as_secs()),
        };
        self.print_block_of_text(
            frame,
            time_left_message,
            info_area,
            Color::Yellow,
            true,
            false,
        );

        let help_message = match self.input_mode {
            InputMode::Normal => "press 'e' to start the test, press 'q' to quit",
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

    fn print_input(
        &mut self,
        frame: &mut impl FrameWrapperInterface,
        expected_input: String,
        input_area: Rect,
        frame_width: usize,
    ) {
        for ((input_char_index, input_char), (_, expected_input_char)) in
            self.input.char_indices().zip(expected_input.char_indices())
        {
            let input = Paragraph::new(expected_input_char.to_string()).style(
                match input_char == expected_input_char {
                    true => Style::default().fg(Color::Green),
                    false => Style::default().bg(Color::Red).fg(Color::Gray),
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

        Stats {
            wpm: valid_characters_count as f64 / 5.0 * 60.0 / self.config.duration.as_secs() as f64,

            raw_accuracy: (self.raw_valid_characters_count) as f64
                / (self.raw_valid_characters_count + self.raw_mistakes_count) as f64
                * 100.0,
            raw_valid_characters_count: self.raw_valid_characters_count,
            raw_mistakes_count: self.raw_mistakes_count,
            raw_typed_characters_count: self.raw_valid_characters_count + self.raw_mistakes_count,

            accuracy: (typed_characters_count - mistakes_count as usize) as f64
                / typed_characters_count as f64
                * 100.0,
            valid_characters_count,
            mistakes_count,
            typed_characters_count: typed_characters_count as u64,
        }
    }
}

#[automock]
trait FrameWrapperInterface {
    fn render_widget<W: Widget + 'static>(&mut self, widget: W, area: Rect);
    fn set_cursor(&mut self, x: u16, y: u16);
    fn size(&self) -> Rect;
}

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

    use crate::expected_input::MockExpectedInputInterface;

    use super::*;

    #[test]
    fn should_render_single_line_input() {
        let config = Config::default();
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

        runner.render(&mut frame, 0);
    }

    #[test]
    fn should_render_multi_line_input() {
        let config = Config::default();
        let mut expected_input = MockExpectedInputInterface::default();

        expected_input
            .expect_get_string()
            .with(predicate::eq(4 * 3))
            .return_const("foobarbazqux".to_string());

        let mut runner = Runner::new(config, expected_input);

        runner.input_mode = InputMode::Editing;
        runner.input = "foobar".to_string();

        let mut frame = MockFrameWrapperInterface::default();

        frame.expect_size().times(2).return_const(Rect {
            x: 0,
            y: 0,
            width: 4,
            height: 3,
        });

        frame
            .expect_render_widget::<Paragraph>()
            .times(10)
            .return_const(());

        frame
            .expect_set_cursor()
            .with(predicate::eq(2), predicate::eq(1))
            .times(1)
            .return_const(());

        runner.render(&mut frame, 0);
    }

    #[test]
    fn should_print_input() {
        let config = Config::default();
        let expected_input = MockExpectedInputInterface::default();
        let mut runner = Runner::new(config, expected_input);

        runner.input = "foo".to_string();

        let mut frame = MockFrameWrapperInterface::default();

        frame
            .expect_render_widget::<Paragraph>()
            .times(3)
            .return_const(());

        runner.print_input(
            &mut frame,
            "foo".to_string(),
            Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 1,
            },
            50,
        );
    }

    #[test]
    fn should_print_block_of_text() {
        let config = Config::default();
        let expected_input = MockExpectedInputInterface::default();
        let runner = Runner::new(config, expected_input);

        let mut frame = MockFrameWrapperInterface::default();

        frame
            .expect_render_widget::<Paragraph>()
            .withf(|_widget: &Paragraph<'_>, area| {
                *area
                    == Rect {
                        x: 0,
                        y: 0,
                        width: 50,
                        height: 1,
                    }
            })
            .times(1)
            .return_const(());

        runner.print_block_of_text(
            &mut frame,
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
