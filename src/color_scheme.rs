//! Module with ColorScheme struct used to define what colors should be used
//! for different elements in test.
//! Default is `green` foreground for correct characters,
//! and `red` background for incorrect.

use ratatui::style::Color;

/// Struct used in config for defining colors used in test.
#[derive(Debug, Copy, Clone)]
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
            correct_match_bg: Color::Reset,
            incorrect_match_fg: Color::Reset,
            incorrect_match_bg: Color::Red,
        }
    }
}
