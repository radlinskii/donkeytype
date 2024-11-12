use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::runner::FrameWrapperInterface;

pub struct HelpWindow;

impl HelpWindow {
    pub fn new() -> Self {
        HelpWindow
    }

    pub fn render(&self, frame: &mut impl FrameWrapperInterface) {
        let frame_rect = frame.area();

        if frame_rect.height < 3 {
            frame.render_widget(Clear, frame_rect);
            return;
        }

        let help_text = vec![
            "",
            " Navigation:",
            " 's'   - Start/resume the test",
            " <Esc> - Pause the test",
            " 'q'   - Quit",
            " '?'   - Toggle this window",
            "",
            " Configuration:",
            " --duration <seconds> - Set test duration",
            " --numbers - Include numbers in the test",
            " --uppercase - Include uppercase letters",
            "",
            " Run 'donkeytype help' in your terminal to get more information ",
            "",
        ];

        let longest_help_msg_len = help_text.iter().map(|s| s.len()).max().unwrap();
        let help_text_lines_count = help_text.len();

        // check if there is enough space vertically to display the help message
        if frame_rect.height <= help_text_lines_count as u16 {
            let paragraph =
                Paragraph::new( "Terminal window is too short to display the help window\nresize the terminal or press \"?\" to return to the test")
                .style(Style::default().fg(Color::Red).bg(Color::Black));

            frame.render_widget(Clear, frame_rect);
            frame.render_widget(paragraph, frame_rect);

            return;
        }

        // check if there is enough space horizontally to display the help message
        if frame_rect.width - 2 <= longest_help_msg_len as u16 {
            let paragraph = Paragraph::new(
                "Terminal window is too narrow\nto display the help window\nresize the terminal\nor press the \"?\" key\nto return to the test",
            )
            .style(Style::default().fg(Color::Red).bg(Color::Black));

            frame.render_widget(Clear, frame_rect);
            frame.render_widget(paragraph, frame_rect);

            return;
        }

        // Create a clear overlay to dim the background
        frame.render_widget(
            Paragraph::new("")
                .style(Style::default().bg(Color::Black).fg(Color::DarkGray))
                .block(Block::default()),
            frame_rect,
        );

        let area = Self::get_centered_rect(
            longest_help_msg_len.try_into().unwrap(),
            help_text_lines_count.try_into().unwrap(),
            frame.area(),
        );

        // Clear the background area first.
        frame.render_widget(Clear, area);

        let block = Block::default().title(" Help ").borders(Borders::ALL);

        let inner_area = block.inner(area);

        // Create constraints dynamically based on help_text length
        let constraints = vec![Constraint::Length(1); help_text_lines_count];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        frame.render_widget(block, area);

        // Render text paragraphs
        for (i, &text) in help_text.iter().enumerate() {
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, chunks[i]);
        }
    }

    fn get_centered_rect(window_width: u16, window_height: u16, r: Rect) -> Rect {
        let x = r.x + (r.width.saturating_sub(window_width + 2)) / 2;
        let y = if r.height > window_height + 4 { 3 } else { 0 };

        Rect::new(x, y, window_width + 2, window_height + 1)
    }
}

// TODO: add tests
