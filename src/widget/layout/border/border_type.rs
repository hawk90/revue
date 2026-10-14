//! The glyph set each border style draws with.

use crate::utils::border::BorderChars;

/// Border style characters
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum BorderType {
    /// No border
    None,
    /// Single line: ┌─┐│└┘
    #[default]
    Single,
    /// Double line: ╔═╗║╚╝
    Double,
    /// Rounded: ╭─╮│╰╯
    Rounded,
    /// Thick: ┏━┓┃┗┛
    Thick,
    /// ASCII: +-+|
    Ascii,
}

/// Empty border chars (for BorderType::None)
const NONE_CHARS: BorderChars = BorderChars {
    top_left: ' ',
    top_right: ' ',
    bottom_left: ' ',
    bottom_right: ' ',
    horizontal: ' ',
    vertical: ' ',
};

impl BorderType {
    /// Get the character set for this border type
    pub fn chars(&self) -> BorderChars {
        match self {
            BorderType::None => NONE_CHARS,
            BorderType::Single => BorderChars::SINGLE,
            BorderType::Double => BorderChars::DOUBLE,
            BorderType::Rounded => BorderChars::ROUNDED,
            BorderType::Thick => BorderChars::BOLD,
            BorderType::Ascii => BorderChars::ASCII,
        }
    }
}
