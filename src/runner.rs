use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Paragraph, Wrap},
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

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        let _config = &self.config;

        loop {
            terminal.draw(|f| self.render(f))?;

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

    fn render<B: Backend>(&self, frame: &mut Frame<B>) {
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
            expected_input_current_line_rest,
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
            expected_input_following_lines,
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

    fn print_input<B: Backend>(
        &self,
        frame: &mut Frame<B>,
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

    fn print_block_of_text<B: Backend>(
        &self,
        frame: &mut Frame<B>,
        text_str: &str,
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

    fn move_cursor<B: Backend>(
        &self,
        frame: &mut Frame<B>,
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

    fn print_help_message<B: Backend>(&self, frame: &mut Frame<B>, area: Rect) {
        match self.input_mode {
            InputMode::Normal => {
                let mut text = Text::from("press 'e' to start editing, press 'q' to quit");
                text.patch_style(Style::default().fg(Color::Yellow));
                let help_message = Paragraph::new(text);
                frame.render_widget(help_message, area);
            }

            InputMode::Editing => {
                let mut text = Text::from("press 'Esc' to stop editing");
                text.patch_style(Style::default().fg(Color::Yellow));
                let help_message = Paragraph::new(text);
                frame.render_widget(help_message, area);
            }
        }
    }
}
