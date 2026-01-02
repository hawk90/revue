//! Crossterm backend implementation
//!
//! This backend uses the crossterm library for cross-platform terminal I/O.

use std::io::{self, Write};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{
        Attribute, Color as CrosstermColor, SetAttribute,
        SetBackgroundColor, SetForegroundColor, ResetColor,
    },
    terminal::{
        self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};

use super::traits::{Backend, BackendCapabilities, StdoutBackend};
use crate::style::Color;
use crate::render::cell::Modifier;
use crate::Result;

/// Crossterm-based terminal backend
///
/// This is the default backend, providing cross-platform support
/// for Windows, macOS, and Linux.
pub struct CrosstermBackend<W: Write> {
    writer: W,
    raw_mode: bool,
    mouse_enabled: bool,
}

impl<W: Write> CrosstermBackend<W> {
    /// Create a new crossterm backend with the given writer
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            raw_mode: false,
            mouse_enabled: false,
        }
    }

    /// Get a reference to the underlying writer
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Get a mutable reference to the underlying writer
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }
}

impl StdoutBackend for CrosstermBackend<io::Stdout> {
    fn stdout() -> io::Result<Self> {
        Ok(Self::new(io::stdout()))
    }
}

impl<W: Write> Write for CrosstermBackend<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.writer.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    fn init(&mut self) -> Result<()> {
        self.init_with_mouse(true)
    }

    fn init_with_mouse(&mut self, enable_mouse: bool) -> Result<()> {
        enable_raw_mode()?;
        self.raw_mode = true;

        if enable_mouse {
            execute!(
                self.writer,
                EnterAlternateScreen,
                EnableMouseCapture,
                Hide,
                Clear(ClearType::All)
            )?;
            self.mouse_enabled = true;
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

    fn restore(&mut self) -> Result<()> {
        if self.raw_mode {
            if self.mouse_enabled {
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
            self.mouse_enabled = false;
        }
        Ok(())
    }

    fn size(&self) -> Result<(u16, u16)> {
        Ok(terminal::size()?)
    }

    fn clear(&mut self) -> Result<()> {
        execute!(self.writer, Clear(ClearType::All))?;
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Hide)?;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Show)?;
        Ok(())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        queue!(self.writer, MoveTo(x, y))?;
        Ok(())
    }

    fn set_fg(&mut self, color: Color) -> Result<()> {
        queue!(self.writer, SetForegroundColor(to_crossterm_color(color)))?;
        Ok(())
    }

    fn set_bg(&mut self, color: Color) -> Result<()> {
        queue!(self.writer, SetBackgroundColor(to_crossterm_color(color)))?;
        Ok(())
    }

    fn reset_fg(&mut self) -> Result<()> {
        queue!(self.writer, SetForegroundColor(CrosstermColor::Reset))?;
        Ok(())
    }

    fn reset_bg(&mut self) -> Result<()> {
        queue!(self.writer, SetBackgroundColor(CrosstermColor::Reset))?;
        Ok(())
    }

    fn set_modifier(&mut self, modifier: Modifier) -> Result<()> {
        if modifier.contains(Modifier::BOLD) {
            queue!(self.writer, SetAttribute(Attribute::Bold))?;
        }
        if modifier.contains(Modifier::ITALIC) {
            queue!(self.writer, SetAttribute(Attribute::Italic))?;
        }
        if modifier.contains(Modifier::UNDERLINE) {
            queue!(self.writer, SetAttribute(Attribute::Underlined))?;
        }
        if modifier.contains(Modifier::DIM) {
            queue!(self.writer, SetAttribute(Attribute::Dim))?;
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            queue!(self.writer, SetAttribute(Attribute::CrossedOut))?;
        }
        Ok(())
    }

    fn reset_style(&mut self) -> Result<()> {
        queue!(self.writer, SetAttribute(Attribute::Reset))?;
        Ok(())
    }

    fn enable_mouse(&mut self) -> Result<()> {
        if !self.mouse_enabled {
            execute!(self.writer, EnableMouseCapture)?;
            self.mouse_enabled = true;
        }
        Ok(())
    }

    fn disable_mouse(&mut self) -> Result<()> {
        if self.mouse_enabled {
            execute!(self.writer, DisableMouseCapture)?;
            self.mouse_enabled = false;
        }
        Ok(())
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            true_color: true,
            hyperlinks: true,
            mouse: true,
            bracketed_paste: true,
            focus_events: true,
        }
    }

    fn name(&self) -> &'static str {
        "crossterm"
    }
}

impl<W: Write> Drop for CrosstermBackend<W> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_backend_name() {
        let backend = CrosstermBackend::new(MockWriter::new());
        assert_eq!(backend.name(), "crossterm");
    }

    #[test]
    fn test_capabilities() {
        let backend = CrosstermBackend::new(MockWriter::new());
        let caps = backend.capabilities();
        assert!(caps.true_color);
        assert!(caps.hyperlinks);
        assert!(caps.mouse);
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
}
