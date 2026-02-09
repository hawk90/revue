//! Terminal rendering implementation

use crossterm::{
    queue,
    style::{
        Attribute, Color as CrosstermColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
    },
};
use std::io::Write;

use super::super::{cell::Modifier, diff, Buffer, Cell};
use super::types::Terminal;
use crate::style::Color;
use crate::utils::unicode::char_width;
use crate::Result;

impl<W: Write> Terminal<W> {
    /// Draws a given set of changes to the terminal and updates the internal current buffer.
    pub(crate) fn draw_changes(
        &mut self,
        changes: Vec<diff::Change>,
        buffer: &Buffer,
    ) -> Result<()> {
        let mut state = super::types::RenderState::default();

        for change in changes {
            // Only draw if not a continuation cell (continuation cells are handled by the wide char)
            if !change.cell.is_continuation() {
                // Look up hyperlink URL if cell has one
                let hyperlink_url = change
                    .cell
                    .hyperlink_id
                    .and_then(|id| buffer.get_hyperlink(id));
                // Look up escape sequence if cell has one
                let escape_sequence = change
                    .cell
                    .sequence_id
                    .and_then(|id| buffer.get_sequence(id));
                self.draw_cell_stateful(
                    change.x,
                    change.y,
                    &change.cell,
                    hyperlink_url,
                    escape_sequence,
                    &mut state,
                )?;
            }
            // Update current buffer with changed cell (Cell is Copy, no allocation)
            self.current.set(change.x, change.y, change.cell);
        }

        // Close any open hyperlink at end of frame
        if state.hyperlink_id.is_some() {
            self.write_hyperlink_end()?;
        }

        // Reset state at end of frame
        if state.fg.is_some() || state.bg.is_some() || !state.modifier.is_empty() {
            queue!(self.writer, SetAttribute(Attribute::Reset))?;
        }

        self.writer.flush()?;
        Ok(())
    }

    /// Draw a single cell with stateful tracking to minimize escape sequences
    pub(crate) fn draw_cell_stateful(
        &mut self,
        x: u16,
        y: u16,
        cell: &Cell,
        hyperlink_url: Option<&str>,
        escape_sequence: Option<&str>,
        state: &mut super::types::RenderState,
    ) -> Result<()> {
        use crossterm::cursor::MoveTo;
        use crossterm::style::Print;

        // Only emit MoveTo if cursor isn't already at the expected position
        // This reduces escape sequences for contiguous same-row cells
        if state.cursor != Some((x, y)) {
            queue!(self.writer, MoveTo(x, y))?;
        }

        // If cell has an escape sequence, write it directly and skip normal rendering
        if let Some(seq) = escape_sequence {
            // Reset any active styling before writing raw sequence
            if state.hyperlink_id.is_some() {
                self.write_hyperlink_end()?;
                state.hyperlink_id = None;
            }
            if state.fg.is_some() || state.bg.is_some() || !state.modifier.is_empty() {
                queue!(self.writer, SetAttribute(Attribute::Reset))?;
                state.fg = None;
                state.bg = None;
                state.modifier = Modifier::empty();
            }
            // Write the raw escape sequence
            write!(self.writer, "{}", seq)?;
            // Escape sequences can move cursor unpredictably, invalidate position
            state.cursor = None;
            return Ok(());
        }

        // Handle hyperlink state changes
        let new_hyperlink_id = cell.hyperlink_id;
        if new_hyperlink_id != state.hyperlink_id {
            // Close previous hyperlink if any
            if state.hyperlink_id.is_some() {
                self.write_hyperlink_end()?;
            }
            // Open new hyperlink if any
            if let Some(url) = hyperlink_url {
                self.write_hyperlink_start(url)?;
            }
            state.hyperlink_id = new_hyperlink_id;
        }

        // Only emit color changes when different from current state
        if cell.fg != state.fg {
            if let Some(fg) = cell.fg {
                queue!(self.writer, SetForegroundColor(to_crossterm_color(fg)))?;
            } else if state.fg.is_some() {
                // Reset to default foreground
                queue!(self.writer, SetForegroundColor(CrosstermColor::Reset))?;
            }
            state.fg = cell.fg;
        }

        if cell.bg != state.bg {
            if let Some(bg) = cell.bg {
                queue!(self.writer, SetBackgroundColor(to_crossterm_color(bg)))?;
            } else if state.bg.is_some() {
                // Reset to default background
                queue!(self.writer, SetBackgroundColor(CrosstermColor::Reset))?;
            }
            state.bg = cell.bg;
        }

        // Only emit modifier changes when different
        if cell.modifier != state.modifier {
            // If we had modifiers before and new cell has different ones, reset first
            if !state.modifier.is_empty() && cell.modifier != state.modifier {
                queue!(self.writer, SetAttribute(Attribute::Reset))?;
                // Re-apply colors after reset
                if let Some(fg) = cell.fg {
                    queue!(self.writer, SetForegroundColor(to_crossterm_color(fg)))?;
                }
                if let Some(bg) = cell.bg {
                    queue!(self.writer, SetBackgroundColor(to_crossterm_color(bg)))?;
                }
            }

            // Apply new modifiers
            if cell.modifier.contains(Modifier::BOLD) {
                queue!(self.writer, SetAttribute(Attribute::Bold))?;
            }
            if cell.modifier.contains(Modifier::ITALIC) {
                queue!(self.writer, SetAttribute(Attribute::Italic))?;
            }
            if cell.modifier.contains(Modifier::UNDERLINE) {
                queue!(self.writer, SetAttribute(Attribute::Underlined))?;
            }
            if cell.modifier.contains(Modifier::DIM) {
                queue!(self.writer, SetAttribute(Attribute::Dim))?;
            }
            if cell.modifier.contains(Modifier::CROSSED_OUT) {
                queue!(self.writer, SetAttribute(Attribute::CrossedOut))?;
            }
            if cell.modifier.contains(Modifier::REVERSE) {
                queue!(self.writer, SetAttribute(Attribute::Reverse))?;
            }

            state.modifier = cell.modifier;
        }

        // Print the character
        queue!(self.writer, Print(cell.symbol))?;

        // Update expected cursor position (cursor advances by character width)
        let width = char_width(cell.symbol) as u16;
        state.cursor = Some((x.saturating_add(width), y));

        Ok(())
    }

    /// Write OSC 8 hyperlink start sequence
    /// Format: ESC ] 8 ; ; URL ST (where ST is ESC \)
    pub(crate) fn write_hyperlink_start(&mut self, url: &str) -> Result<()> {
        write!(self.writer, "\x1b]8;;{}\x1b\\", url)?;
        Ok(())
    }

    /// Write OSC 8 hyperlink end sequence
    pub(crate) fn write_hyperlink_end(&mut self) -> Result<()> {
        write!(self.writer, "\x1b]8;;\x1b\\")?;
        Ok(())
    }
}

/// Convert our Color to crossterm Color
fn to_crossterm_color(color: Color) -> CrosstermColor {
    CrosstermColor::Rgb {
        r: color.r,
        g: color.g,
        b: color.b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::style::Color;

    #[test]
    fn test_to_crossterm_color_rgb() {
        let color = Color {
            r: 255,
            g: 128,
            b: 0,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 128);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_to_crossterm_color_black() {
        let color = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 0);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_to_crossterm_color_white() {
        let color = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 255);
                assert_eq!(b, 255);
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_to_crossterm_color_gray() {
        let color = Color {
            r: 128,
            g: 128,
            b: 128,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 128);
                assert_eq!(g, 128);
                assert_eq!(b, 128);
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_to_crossterm_color_red() {
        let color = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 0);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_to_crossterm_color_green() {
        let color = Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected Rgb color"),
        }
    }

    #[test]
    fn test_to_crossterm_color_blue() {
        let color = Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        };
        let crossterm_color = to_crossterm_color(color);
        match crossterm_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 0);
                assert_eq!(b, 255);
            }
            _ => panic!("Expected Rgb color"),
        }
    }
}
