use crate::{config::Config, expected_input::ExpectedInput};
use std::io::{self, Stdout, Write};
use termion::{
    color,
    cursor::DetectCursorPos,
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

pub struct Runner<'a> {
    expected_input: ExpectedInput<'a>,
    actual_input: String,
    ignored_chars: [char; 3],
}

impl<'a> Runner<'a> {
    pub fn new(_config: Config, expected_input: ExpectedInput) -> Runner {
        return Runner {
            expected_input: expected_input,
            actual_input: String::new(),
            ignored_chars: ['\n', '\r', '\t'],
        };
    }

    pub fn run(&mut self) {
        // Create a raw terminal instance
        let mut stdout = io::stdout()
            .into_raw_mode()
            .expect("Unable to get stdout into raw terminal");
        let stdin = io::stdin();
        // Get the initial cursor position
        let (column, row) = stdout.cursor_pos().unwrap();

        // set color of the placeholder
        write!(stdout, "{}", color::Fg(color::LightBlack)).unwrap();
        // Print the expected input as placeholder
        print!("{}", self.expected_input.to_str());
        stdout.flush().unwrap();

        // Prepare cursor
        write!(stdout, "{}", termion::cursor::BlinkingBar).unwrap();
        write!(stdout, "{}", termion::cursor::Goto(column, row)).unwrap();
        stdout.flush().unwrap();

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char(c) => {
                    if self.ignored_chars.contains(&c) {
                        continue;
                    }

                    self.handle_char(c, &mut stdout);
                }
                Key::Backspace => {
                    self.handle_backspace(&mut stdout);
                }
                Key::Alt(c) => {
                    if c == '\x7F' {
                        self.handle_alt_backspace(&mut stdout);
                    }
                }
                Key::Ctrl('c') => {
                    // on Ctrl+C exit the loop
                    break;
                }
                _ => {}
            }

            if self.actual_input.len() == self.expected_input.to_str().len() {
                break;
            }
        }

        write!(stdout, "{}", termion::cursor::Goto(1, row + 1)).unwrap();
        writeln!(stdout).unwrap();
        write!(stdout, "{}", color::Fg(color::Black)).unwrap();
        writeln!(stdout, "Good job!").unwrap();
        stdout.flush().unwrap();

        // Show the cursor again before exiting
        write!(stdout, "{}", termion::cursor::Show).unwrap();
        write!(stdout, "{}", termion::cursor::Goto(1, row + 3)).unwrap();
    }

    fn check_if_correct_char(&self, c: char) -> bool {
        return c
            == self
                .expected_input
                .to_str()
                .chars()
                .nth(self.actual_input.len())
                .unwrap();
    }

    fn print_rest_of_expected_input(&self, stdout: &mut RawTerminal<Stdout>) {
        print!(
            "{}",
            &self.expected_input.to_str()[self.actual_input.len()..]
        );

        print!(
            "{}",
            termion::cursor::Left(
                self.expected_input.to_str()[self.actual_input.len()..]
                    .len()
                    .try_into()
                    .unwrap()
            )
        );

        stdout.flush().unwrap();
    }

    fn handle_char(&mut self, c: char, stdout: &mut RawTerminal<Stdout>) {
        // print the character in green if it's correct, otherwise in red
        if self.check_if_correct_char(c) {
            write!(stdout, "{}", color::Fg(color::Green)).unwrap();
        } else {
            write!(stdout, "{}", color::Fg(color::Red)).unwrap();
        }
        print!("{}", c);
        write!(stdout, "{}", color::Fg(color::LightBlack)).unwrap();
        stdout.flush().unwrap();

        self.actual_input.push(c);
    }

    fn handle_backspace(&mut self, stdout: &mut RawTerminal<Stdout>) {
        // delete the last character
        print!("{}", termion::cursor::Left(1));
        self.actual_input.pop();
        print!("{}", termion::clear::AfterCursor);

        self.print_rest_of_expected_input(stdout)
    }

    fn handle_alt_backspace(&mut self, stdout: &mut RawTerminal<Stdout>) {
        // delete the current word
        let mut chars_to_delete = 0;
        let mut found_non_space_char = false;
        for c in self.actual_input.chars().rev() {
            if c == ' ' && found_non_space_char {
                break;
            } else {
                found_non_space_char = true;
            }
            chars_to_delete += 1;
        }

        for _ in 0..chars_to_delete {
            print!("{}", termion::cursor::Left(1));
            self.actual_input.pop();
            print!("{}", termion::clear::AfterCursor);
        }

        self.print_rest_of_expected_input(stdout)
    }
}
