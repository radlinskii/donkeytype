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

    pub fn render(&self, frame: &mut impl FrameWrapperInterface, area: Rect) {
        // Clear the background area first.
        frame.render_widget(Clear, area);

        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let inner_area = block.inner(area);

        let help_text = vec![
            "Navigation:",
            "s - Start/unpause the test",
            "Esc - Pause the test",
            "q - Quit",
            "",
            "Configuration:",
            "--duration <seconds> - Set test duration",
            "--numbers - Include numbers in the test",
            "--uppercase - Include uppercase letters",
            "Run 'donkeytype help' for more options",
        ];

        // Create constraints dynamically based on help_text length
        let constraints = vec![Constraint::Length(1); help_text.len()];

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(inner_area);

        // Render block
        frame.render_widget(block, area);

        // Render text paragraphs
        for (i, &text) in help_text.iter().enumerate() {
            let paragraph = Paragraph::new(text).style(Style::default().fg(Color::White));
            frame.render_widget(paragraph, chunks[i]);
        }
    }
}
