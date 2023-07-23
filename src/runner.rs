use crate::expected_input::ExpectedInput;
use std::io::{self, Write};
use termion::{color, cursor::DetectCursorPos, event::Key, input::TermRead, raw::IntoRawMode};

pub struct Runner<'a> {
    expected_input: ExpectedInput<'a>,
    actual_input: String,
    ignored_chars: [char; 3],
}

impl<'a> Runner<'a> {
    pub fn new(expected_input: ExpectedInput) -> Runner {
        return Runner {
            expected_input: expected_input,
            actual_input: String::new(),
            ignored_chars: ['\n', '\r', '\t'],
        };
    }

    pub fn check_if_correct_char(&self, c: char) -> bool {
        return c
            == self
                .expected_input
                .to_str()
                .chars()
                .nth(self.actual_input.len())
                .unwrap();
    }

    pub fn run(&mut self) {
        // Create a raw terminal instance
        let mut stdout = io::stdout().into_raw_mode().unwrap();
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

        // Start a non-blocking input loops
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char(c) => {
                    if self.ignored_chars.contains(&c) {
                        continue;
                    }

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
                Key::Backspace => {
                    // Move the cursor back one space
                    print!("{}", termion::cursor::Left(1));
                    self.actual_input.pop();
                    print!("{}", termion::clear::AfterCursor);

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
                Key::Ctrl('c') => {
                    break; // Exit the loop when the user presses Ctrl+C
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
}
