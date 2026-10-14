//! Glyph size, the glyph rows, `measure` and `render`

use super::{patterns, DigitStyle, Digits};
use crate::widget::{RenderContext, View};

impl Digits {
    /// Get the height of digits in this style
    pub fn height(&self) -> usize {
        match self.style {
            DigitStyle::Braille => 4,
            _ => 5,
        }
    }

    /// Get the width of a single digit in this style
    pub fn digit_width(&self) -> usize {
        match self.style {
            DigitStyle::Braille => 2,
            _ => 3,
        }
    }

    /// Render to lines
    pub fn render_lines(&self) -> Vec<String> {
        let value = self.format_value();
        let height = self.height();
        let mut lines: Vec<String> = vec![String::new(); height];

        // Add prefix (at bottom line, small text)
        // For now, we'll skip prefix/suffix in the large display

        // Render each character
        for c in value.chars() {
            let pattern = patterns::char_pattern(self.style, c);
            for (i, row) in pattern.iter().enumerate() {
                if i < height {
                    lines[i].push_str(row);
                    lines[i].push(' '); // spacing between digits
                }
            }
        }

        lines
    }
}

impl View for Digits {
    crate::impl_view_meta!("Digits");

    /// The glyph rows ([`height`](Digits::height)), plus a row for the
    /// prefix/suffix label if there is one; as wide as the widest of them.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let glyphs = self
            .render_lines()
            .iter()
            .map(|line| crate::utils::unicode::display_width(line.trim_end()))
            .max()
            .unwrap_or(0);
        let (label, label_rows) = if self.prefix.is_some() || self.suffix.is_some() {
            let label = crate::utils::unicode::display_width(self.prefix.as_deref().unwrap_or(""))
                + crate::utils::unicode::display_width(&self.value)
                + crate::utils::unicode::display_width(self.suffix.as_deref().unwrap_or(""));
            (label, 1)
        } else {
            (0, 0)
        };
        let width = glyphs.max(label).min(u16::MAX as usize) as u16;
        let height = (self.height() + label_rows).min(u16::MAX as usize) as u16;
        Some((width.min(max_width), height.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        let lines = self.render_lines();
        let mut stack = vstack();

        for line in lines {
            let mut text = Text::new(line);
            // Builder first; a rule can colour digits that named none.
            if let Some(fg) = self.fg.or_else(|| ctx.css_color_if_set()) {
                text = text.fg(fg);
            }
            if let Some(bg) = self.bg {
                text = text.bg(bg);
            }
            // One row per pattern line: an auto-sized child would get an
            // equal share of a taller area and pull the glyph rows apart.
            stack = stack.child_sized(text, 1);
        }

        // Add prefix/suffix as regular text below
        if self.prefix.is_some() || self.suffix.is_some() {
            let label = format!(
                "{}{}{}",
                self.prefix.as_deref().unwrap_or(""),
                self.value,
                self.suffix.as_deref().unwrap_or("")
            );
            let mut text = Text::new(label);
            if let Some(fg) = self.fg {
                text = text.fg(fg);
            }
            stack = stack.child_sized(text, 1);
        }

        stack.render(ctx);
    }
}
