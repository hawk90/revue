//! Digits builders: style, colors, label and number formatting

use super::{DigitStyle, Digits};
use crate::style::Color;

impl Digits {
    /// Set display style
    pub fn style(mut self, style: DigitStyle) -> Self {
        self.style = style;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set prefix, shown before the value on the row below the digits
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.prefix = Some(prefix.into());
        self
    }

    /// Set suffix, shown after the value on the row below the digits
    pub fn suffix(mut self, suffix: impl Into<String>) -> Self {
        self.suffix = Some(suffix.into());
        self
    }

    /// Set thousands separator, put in a value that is a number (a sign,
    /// digits and maybe a fraction); a clock or other text is left whole
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = Some(sep);
        self
    }

    /// Set minimum width: the value's characters before grouping, padded
    /// with zeros (or blanks, see [`leading_zeros`](Self::leading_zeros))
    pub fn min_width(mut self, width: usize) -> Self {
        self.min_width = Some(width);
        self
    }

    /// Show leading zeros when padding to [`min_width`](Self::min_width)
    ///
    /// Padding uses zeros by default. Pass `false` to pad with blank
    /// columns instead, so the number is right-aligned without zeros: it
    /// takes the width the zero-padded number would, separators included.
    pub fn leading_zeros(mut self, show: bool) -> Self {
        self.leading_zeros = Some(show);
        self
    }
}
