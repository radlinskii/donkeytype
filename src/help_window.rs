use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Clear},
    Frame, backend::Backend,
};

pub struct HelpWindow;

impl HelpWindow {
    pub fn new() -> Self {
        HelpWindow
    }

    pub fn render<B: Backend>(&self, f: &mut Frame<B>, area: Rect) {
        let block = Block::default()
            .title("Help")
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::White).bg(Color::Black));

        let inner_area = block.inner(area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner_area);

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

        for (i, &text) in help_text.iter().enumerate() {
            let paragraph = Paragraph::new(text)
                .style(Style::default().fg(Color::White));
            f.render_widget(paragraph, chunks[i]);
        }

        f.render_widget(Clear, area);
        f.render_widget(block, area);
    }
}