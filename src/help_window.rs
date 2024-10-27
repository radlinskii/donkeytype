use anyhow::{Context, Result};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Clear, Paragraph},
};

use crate::runner::FrameWrapperInterface;

pub struct HelpWindow;

impl HelpWindow {
    pub fn new() -> Self {
        HelpWindow
    }

    pub fn render(&self, frame: &mut impl FrameWrapperInterface) -> Result<()> {
        let help_text = vec![
            "",
            " Navigation:",
            " 's' - Start/resume the test",
            " <Esc> - Pause the test",
            " 'q' - Quit",
            " '?' - Close this window",
            "",
            " Configuration:",
            " --duration <seconds> - Set test duration",
            " --numbers - Include numbers in the test",
            " --uppercase - Include uppercase letters",
            "",
            " Run 'donkeytype help' in your terminal to get more information ",
            "",
        ];

        let longest_help_msg_len = help_text
            .iter()
            .map(|s| s.len())
            .max()
            .context("Unable to get the length of longest line from help window text")?;
        let help_text_lines_count = help_text.len();

        let area = Self::centered_rect(
            longest_help_msg_len.try_into().unwrap(),
            help_text_lines_count.try_into().unwrap(),
            frame.size(),
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

        // Render block
        frame.render_widget(block, area);

        // Render text paragraphs
        for (i, &text) in help_text.iter().enumerate() {
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, chunks[i]);
        }

        Ok(())
    }

    fn centered_rect(window_width: u16, window_height: u16, r: Rect) -> Rect {
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(0),
                Constraint::Length(window_height + 2),
                Constraint::Length(r.height - window_height - 2),
            ])
            .split(r);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length((r.width - window_width - 2) / 2),
                Constraint::Length(window_width + 2),
                Constraint::Length((r.width - window_width - 2) / 2),
            ])
            .split(popup_layout[1])[1]
    }
}
