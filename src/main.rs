use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Paragraph, Wrap},
    Frame, Terminal,
};

enum InputMode {
    Normal,
    Editing,
}

/// App holds the state of the application
struct App {
    /// Current value of the input box
    input: String,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
}

impl Default for App {
    fn default() -> App {
        App {
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::default();
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('q') => {
                        return Ok(());
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        app.messages.push(app.input.drain(..).collect());
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)].as_ref())
        .split(frame.size());

    let frame_width = frame.size().width as usize;
    let input_len = app.input.len();
    let input_line_index = (input_len / frame_width) as u16;
    let input_current_line_len = input_len % frame_width;

    let expected_input_str: String = format!("frame_width: {}, input_len: {}, input_line_index: {}, input_current_line_len: {}, chunks[0]: {:?} input example input example input example input example input example input example input example input example input example input example input example input", frame_width,input_len, input_line_index, input_current_line_len, chunks[0]);
    let (expected_input_str, _) = expected_input_str.split_at(frame_width);
    let expected_input_str = expected_input_str
        .to_string()
        .repeat(input_line_index as usize + 2);

    let (expected_input_first_line, expected_input_rest) =
        expected_input_str.split_at(((input_line_index as usize) + 1) * frame_width);
    let (_expected_input_first_line_already_typed, expected_input_first_line_rest) =
        expected_input_first_line.split_at(input_len);

    let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::White),
        })
        .block(Block::default())
        .wrap(Wrap { trim: false });
    frame.render_widget(input, chunks[0]);

    let mut expected_input_current_line_text = Text::from(expected_input_first_line_rest);
    expected_input_current_line_text.patch_style(Style::default().fg(Color::LightGreen));
    let paragraph = Paragraph::new(expected_input_current_line_text);
    frame.render_widget(
        paragraph,
        Rect {
            x: chunks[0].x + input_current_line_len as u16,
            y: chunks[0].y + input_line_index,
            width: frame_width as u16 - input_current_line_len as u16,
            height: 1,
        },
    );

    let mut expected_input_rest_text = Text::from(expected_input_rest);
    expected_input_rest_text.patch_style(Style::default().fg(Color::LightBlue));
    let paragraph = Paragraph::new(expected_input_rest_text).wrap(Wrap { trim: false });
    frame.render_widget(
        paragraph,
        Rect {
            x: chunks[0].x,
            y: chunks[0].y + input_line_index + 1,
            height: chunks[0].height - input_line_index - 1,
            width: chunks[0].width,
        },
    );

    match app.input_mode {
        InputMode::Normal =>
            // Don't need to do anything here, because `Frame` already hid the cursor
            {}

        InputMode::Editing => frame.set_cursor(
            chunks[0].x + input_current_line_len as u16,
            chunks[0].y + input_line_index,
        ),
    }

    match app.input_mode {
        InputMode::Normal => {
            let mut text = Text::from("press 'e' to start editing, press 'q' to quit");
            text.patch_style(Style::default().fg(Color::Yellow));
            let help_message = Paragraph::new(text);
            frame.render_widget(help_message, chunks[1]);
        }

        InputMode::Editing => {
            let mut text = Text::from("press 'Esc' to stop editing");
            text.patch_style(Style::default().fg(Color::Yellow));
            let help_message = Paragraph::new(text);
            frame.render_widget(help_message, chunks[1]);
        }
    }
}
