//! Public API functions for figlet rendering

use super::types::FigletFont;
use super::{
    banner::render_banner, block::render_block, mini::render_mini, slant::render_slant,
    small::render_small,
};

/// Render text as figlet ASCII art
pub fn figlet(text: &str) -> String {
    figlet_with_font(text, FigletFont::default())
}

/// Render text as figlet ASCII art with specified font
pub fn figlet_with_font(text: &str, font: FigletFont) -> String {
    match font {
        FigletFont::Block => render_block(text),
        FigletFont::Slant => render_slant(text),
        FigletFont::Banner => render_banner(text),
        FigletFont::Small => render_small(text),
        FigletFont::Mini => render_mini(text),
    }
}

/// Get the height of a figlet font in rows
pub fn font_height(font: FigletFont) -> usize {
    match font {
        FigletFont::Block => 6,
        FigletFont::Slant => 6,
        FigletFont::Banner => 7,
        FigletFont::Small => 5,
        FigletFont::Mini => 3,
    }
}

/// Get lines of figlet text as a vector
pub fn figlet_lines(text: &str, font: FigletFont) -> Vec<String> {
    figlet_with_font(text, font)
        .lines()
        .map(|s| s.to_string())
        .collect()
}
