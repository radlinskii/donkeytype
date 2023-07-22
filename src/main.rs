use std::io::{self, Write};
use termion::{color, cursor::DetectCursorPos, event::Key, input::TermRead, raw::IntoRawMode};

fn main() {
    let expected_input = "hello world";

    // Create a raw terminal instance
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let stdin = io::stdin();
    // Get the initial cursor position
    let (column, row) = stdout.cursor_pos().unwrap();

    // set color of the placeholder
    write!(stdout, "{}", color::Fg(color::LightBlack)).unwrap();
    // Print the expected input
    print!("{}", expected_input);
    stdout.flush().unwrap();

    // Prepare cursor
    write!(stdout, "{}", termion::cursor::BlinkingBar).unwrap();
    write!(stdout, "{}", termion::cursor::Goto(column, row)).unwrap();
    stdout.flush().unwrap();

    let mut actual_input = String::new();

    // Start a non-blocking input loops
    for key in stdin.keys() {
        match key.unwrap() {
            Key::Char(c) => {
                // print user input

                // ignore Return key
                if c == '\n' {
                    continue;
                }

                if c == expected_input.chars().nth(actual_input.len()).unwrap() {
                    // if char is correct print in green
                    write!(stdout, "{}", color::Fg(color::Green)).unwrap();
                } else {
                    // if char is incorrect print in red
                    write!(stdout, "{}", color::Fg(color::Red)).unwrap();
                }
                print!("{}", c);
                write!(stdout, "{}", color::Fg(color::LightBlack)).unwrap();
                stdout.flush().unwrap();

                actual_input.push(c);
            }
            Key::Backspace => {
                // Move the cursor back one space
                print!("{}", termion::cursor::Left(1));
                actual_input.pop();
                print!("{}", termion::clear::AfterCursor);
                print!("{}", &expected_input[actual_input.len()..]);

                print!(
                    "{}",
                    termion::cursor::Left(
                        expected_input[actual_input.len()..]
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

        if actual_input.len() == expected_input.len() {
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
