use crossterm::event::{self, Event, KeyCode};
use mockall::automock;
use std::io::{self, Write};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Paragraph, Widget, Wrap},
    Frame, Terminal,
};

use crate::config::Config;
use crate::expected_input::ExpectedInputTrait;

enum InputMode {
    Normal,
    Editing,
}

pub struct Runner<T: ExpectedInputTrait> {
    input: String,
    input_mode: InputMode,
    config: Config,
    expected_input: T,
}

impl<T: ExpectedInputTrait> Runner<T> {
    pub fn new(config: Config, expected_input: T) -> Runner<T> {
        Runner {
            input: String::new(),
            input_mode: InputMode::Normal,
            config,
            expected_input,
        }
    }

    pub fn run<W: Write>(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<W>>,
    ) -> io::Result<()> {
        let _config = &self.config;

        loop {
            terminal.draw(|f: &mut Frame<CrosstermBackend<W>>| {
                let mut frame_wrapper = FrameWrapper::new(f);
                self.render(&mut frame_wrapper);
            })?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('e') => {
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Char(c) => {
                            self.input.push(c);
                        }
                        KeyCode::Backspace => {
                            self.input.pop();
                        }
                        KeyCode::Esc => {
                            self.input_mode = InputMode::Normal;
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    fn render(&self, frame: &mut impl FrameWrapperInterface) {
        let areas = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
            .split(frame.size());
        let input_area = areas[0];
        let help_area = areas[1];

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
        );

        self.move_cursor(
            frame,
            input_area,
            input_current_line_len,
            current_line_index,
        );

        self.print_help_message(frame, help_area);
    }

    fn print_input(
        &self,
        frame: &mut impl FrameWrapperInterface,
        expected_input: String,
        input_area: Rect,
        frame_width: usize,
    ) {
        for ((input_char_index, input_char), (_, expected_input_char)) in
            self.input.char_indices().zip(expected_input.char_indices())
        {
            let input = Paragraph::new(input_char.to_string()).style(
                match input_char == expected_input_char {
                    true => Style::default().fg(Color::Green),
                    false => Style::default().fg(Color::Red),
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
    ) {
        let mut text = Text::from(text_str);
        text.patch_style(Style::default().fg(color));
        let mut paragraph = Paragraph::new(text);

        if wrap {
            paragraph = paragraph.wrap(Wrap { trim: false });
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
                // Don't need to do anything here, because `Frame` already hid the cursor
                {}

            InputMode::Editing => frame.set_cursor(
                area.x + input_current_line_len as u16,
                area.y + current_line_index,
            ),
        }
    }

    fn print_help_message(&self, frame: &mut impl FrameWrapperInterface, area: Rect) {
        let mut text = match self.input_mode {
            InputMode::Normal => Text::from("press 'e' to start editing, press 'q' to quit"),
            InputMode::Editing => Text::from("press 'Esc' to stop editing"),
        };

        text.patch_style(Style::default().fg(Color::Yellow));
        let help_message = Paragraph::new(text);
        frame.render_widget(help_message, area);
    }
}

#[automock]
trait FrameWrapperInterface {
    fn render_widget<W: Widget + 'static>(&mut self, widget: W, area: Rect);
    fn set_cursor(&mut self, x: u16, y: u16);
    fn size(&self) -> Rect;
}

pub struct FrameWrapper<'a, 'b, W: Write> {
    frame: &'a mut Frame<'b, CrosstermBackend<W>>,
}

impl<'a, 'b, W: Write> FrameWrapper<'a, 'b, W> {
    pub fn new(frame: &'a mut Frame<'b, CrosstermBackend<W>>) -> Self {
        FrameWrapper { frame }
    }
}

impl<'a, 'b, W: Write> FrameWrapperInterface for FrameWrapper<'a, 'b, W> {
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
    use crate::expected_input::ExpectedInput;

    use super::*;

    #[test]
    fn should_print_help_message_for_normal_mode() {
        let config = Config {
            duration: 10,
            numbers: true,
        };
        let expected_input = ExpectedInput::new(&config);
        let runner = Runner::new(config, expected_input);

        let mut frame = MockFrameWrapperInterface::default();

        frame
            .expect_render_widget::<Paragraph>()
            .withf(|_widget, area| {
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

        runner.print_help_message(
            &mut frame,
            Rect {
                x: 0,
                y: 0,
                width: 50,
                height: 1,
            },
        );
    }
}
