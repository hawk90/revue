//! Terminal backend using crossterm

use std::io::{self, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{EnableMouseCapture, DisableMouseCapture},
    execute, queue,
    style::{
        Attribute, Color as CrosstermColor, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor, ResetColor,
    },
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};

use super::{Buffer, Cell};
use super::diff::diff;
use super::cell::Modifier;
use crate::style::Color;
use crate::Result;

/// Tracks current terminal styling state to minimize escape sequences
#[derive(Default)]
struct RenderState {
    fg: Option<Color>,
    bg: Option<Color>,
    modifier: Modifier,
    /// Current hyperlink ID (None means no hyperlink active)
    hyperlink_id: Option<u16>,
}

/// Terminal backend for rendering
pub struct Terminal<W: Write> {
    /// Output writer
    writer: W,
    /// Current buffer (what's on screen)
    current: Buffer,
    /// Whether we're in raw mode
    raw_mode: bool,
    /// Whether mouse capture is enabled
    mouse_capture: bool,
}

impl<W: Write> Terminal<W> {
    /// Create a new terminal with the given writer
    pub fn new(writer: W) -> Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            writer,
            current: Buffer::new(width, height),
            raw_mode: false,
            mouse_capture: false,
        })
    }

    /// Initialize the terminal for TUI mode with mouse capture
    pub fn init(&mut self) -> Result<()> {
        self.init_with_mouse(true)
    }

    /// Initialize the terminal for TUI mode with optional mouse capture
    ///
    /// When `mouse_capture` is false, text selection in terminal works normally.
    /// Use this for keyboard-only applications.
    pub fn init_with_mouse(&mut self, mouse_capture: bool) -> Result<()> {
        enable_raw_mode()?;
        self.raw_mode = true;
        self.mouse_capture = mouse_capture;
        if mouse_capture {
            execute!(
                self.writer,
                EnterAlternateScreen,
                EnableMouseCapture,
                Hide,
                Clear(ClearType::All)
            )?;
        } else {
            execute!(
                self.writer,
                EnterAlternateScreen,
                Hide,
                Clear(ClearType::All)
            )?;
        }
        Ok(())
    }

    /// Restore the terminal to normal mode
    pub fn restore(&mut self) -> Result<()> {
        if self.raw_mode {
            if self.mouse_capture {
                execute!(
                    self.writer,
                    DisableMouseCapture,
                    ResetColor,
                    Show,
                    LeaveAlternateScreen
                )?;
            } else {
                execute!(
                    self.writer,
                    ResetColor,
                    Show,
                    LeaveAlternateScreen
                )?;
            }
            disable_raw_mode()?;
            self.raw_mode = false;
        }
        Ok(())
    }

    /// Get terminal size
    pub fn size(&self) -> (u16, u16) {
        (self.current.width(), self.current.height())
    }

    /// Resize the terminal buffer
    pub fn resize(&mut self, width: u16, height: u16) {
        self.current.resize(width, height);
    }

    /// Render a buffer to the terminal using diff-based updates
    pub fn render(&mut self, buffer: &Buffer) -> Result<()> {
        let changes = diff(&self.current, buffer, &[]); // Old diff call, uses full screen if no rects

        self.draw_changes(changes, buffer)
    }

    /// Draws a given set of changes to the terminal and updates the internal current buffer.
    pub fn draw_changes(&mut self, changes: Vec<super::diff::Change>, buffer: &Buffer) -> Result<()> {
        let mut state = RenderState::default();

        for change in changes {
            // Only draw if not a continuation cell (continuation cells are handled by the wide char)
            if !change.cell.is_continuation() {
                // Look up hyperlink URL if cell has one
                let hyperlink_url = change.cell.hyperlink_id
                    .and_then(|id| buffer.get_hyperlink(id));
                self.draw_cell_stateful(change.x, change.y, &change.cell, hyperlink_url, &mut state)?;
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

    /// Force a full redraw
    pub fn force_redraw(&mut self, buffer: &Buffer) -> Result<()> {
        queue!(self.writer, Clear(ClearType::All))?;

        let mut state = RenderState::default();

        for (x, y, cell) in buffer.iter_cells() {
            if !cell.is_continuation() {
                let hyperlink_url = cell.hyperlink_id
                    .and_then(|id| buffer.get_hyperlink(id));
                self.draw_cell_stateful(x, y, cell, hyperlink_url, &mut state)?;
            }
            // Update current buffer (Cell is Copy, no allocation)
            self.current.set(x, y, *cell);
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
    fn draw_cell_stateful(
        &mut self,
        x: u16,
        y: u16,
        cell: &Cell,
        hyperlink_url: Option<&str>,
        state: &mut RenderState,
    ) -> Result<()> {
        queue!(self.writer, MoveTo(x, y))?;

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

            state.modifier = cell.modifier;
        }

        // Print the character
        queue!(self.writer, Print(cell.symbol))?;

        Ok(())
    }

    /// Write OSC 8 hyperlink start sequence
    /// Format: ESC ] 8 ; ; URL ST (where ST is ESC \)
    fn write_hyperlink_start(&mut self, url: &str) -> Result<()> {
        write!(self.writer, "\x1b]8;;{}\x1b\\", url)?;
        Ok(())
    }

    /// Write OSC 8 hyperlink end sequence
    fn write_hyperlink_end(&mut self) -> Result<()> {
        write!(self.writer, "\x1b]8;;\x1b\\")?;
        Ok(())
    }

    /// Clear the screen
    pub fn clear(&mut self) -> Result<()> {
        execute!(self.writer, Clear(ClearType::All))?;
        self.current.clear();
        Ok(())
    }

    /// Show the cursor
    pub fn show_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Show)?;
        Ok(())
    }

    /// Hide the cursor
    pub fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Hide)?;
        Ok(())
    }

    /// Move cursor to position
    pub fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        execute!(self.writer, MoveTo(x, y))?;
        Ok(())
    }
}

impl<W: Write> Drop for Terminal<W> {
    fn drop(&mut self) {
        let _ = self.restore();
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

/// Create a terminal with stdout
pub fn stdout_terminal() -> Result<Terminal<io::Stdout>> {
    Terminal::new(io::stdout())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Mock writer for testing
    #[allow(dead_code)]
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { buffer: Vec::new() }
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_to_crossterm_color() {
        let color = Color::rgb(255, 128, 64);
        let ct_color = to_crossterm_color(color);

        match ct_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 128);
                assert_eq!(b, 64);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_constants_conversion() {
        let red = to_crossterm_color(Color::RED);
        match red {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 0);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected RGB color"),
        }
    }
}
