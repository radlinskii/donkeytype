use ratatui::style::Color;

#[derive(Debug)]
pub struct ColorScheme {
    pub correct_match_fg: Color,
    pub correct_match_bg: Color,
    pub incorrect_match_fg: Color,
    pub incorrect_match_bg: Color,
}

impl ColorScheme {

    pub fn default() -> Self {
        Self {
            correct_match_fg: Color::Green,
            correct_match_bg: Color::Black,
            incorrect_match_fg: Color::Gray,
            incorrect_match_bg: Color::Red,
        }
    }
}
