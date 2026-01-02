//! Termion backend implementation (Unix only)
//!
//! This backend uses the termion library for Unix terminal I/O.
//! It provides a lightweight alternative to crossterm on Unix systems.

use std::io::{self, Write};
use std::os::unix::io::AsRawFd;

use termion::{
    clear, cursor,
    color::{self, Bg, Fg, Rgb},
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
    screen::{AlternateScreen, IntoAlternateScreen},
    style::{self, Bold, CrossedOut, Faint, Italic, Reset, Underline},
};

use super::traits::{Backend, BackendCapabilities, StdoutBackend};
use crate::style::Color;
use crate::render::cell::Modifier;
use crate::Result;

/// Termion-based terminal backend (Unix only)
///
/// This backend provides a lightweight alternative to crossterm
/// on Unix systems. It uses termion for terminal I/O.
pub struct TermionBackend<W: Write + AsRawFd> {
    writer: Option<MouseTerminal<AlternateScreen<RawTerminal<W>>>>,
    raw_writer: Option<W>,
    initialized: bool,
    mouse_enabled: bool,
}

impl<W: Write + AsRawFd> TermionBackend<W> {
    /// Create a new termion backend with the given writer
    ///
    /// Note: The terminal is not initialized until `init()` is called.
    pub fn new(writer: W) -> Self {
        Self {
            writer: None,
            raw_writer: Some(writer),
            initialized: false,
            mouse_enabled: false,
        }
    }
}

impl StdoutBackend for TermionBackend<io::Stdout> {
    fn stdout() -> io::Result<Self> {
        Ok(Self::new(io::stdout()))
    }
}

impl<W: Write + AsRawFd> Write for TermionBackend<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(ref mut w) = self.writer {
            w.write(buf)
        } else if let Some(ref mut w) = self.raw_writer {
            w.write(buf)
        } else {
            Err(io::Error::new(io::ErrorKind::NotConnected, "No writer available"))
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(ref mut w) = self.writer {
            w.flush()
        } else if let Some(ref mut w) = self.raw_writer {
            w.flush()
        } else {
            Ok(())
        }
    }
}

impl<W: Write + AsRawFd> Backend for TermionBackend<W> {
    fn init(&mut self) -> Result<()> {
        self.init_with_mouse(true)
    }

    fn init_with_mouse(&mut self, enable_mouse: bool) -> Result<()> {
        if self.initialized {
            return Ok(());
        }

        if let Some(raw_writer) = self.raw_writer.take() {
            let raw = raw_writer.into_raw_mode()?;
            let alternate = raw.into_alternate_screen()?;

            if enable_mouse {
                let mouse_terminal = MouseTerminal::from(alternate);
                self.writer = Some(mouse_terminal);
                self.mouse_enabled = true;
            } else {
                // For non-mouse mode, we still need MouseTerminal wrapper
                // but mouse events won't be captured
                let mouse_terminal = MouseTerminal::from(alternate);
                self.writer = Some(mouse_terminal);
                self.mouse_enabled = false;
            }

            self.initialized = true;

            // Clear screen and hide cursor
            if let Some(ref mut w) = self.writer {
                write!(w, "{}{}", clear::All, cursor::Hide)?;
                w.flush()?;
            }
        }

        Ok(())
    }

    fn restore(&mut self) -> Result<()> {
        if !self.initialized {
            return Ok(());
        }

        if let Some(ref mut w) = self.writer {
            write!(w, "{}{}", cursor::Show, style::Reset)?;
            w.flush()?;
        }

        // Dropping the writer will restore the terminal
        self.writer = None;
        self.initialized = false;
        self.mouse_enabled = false;

        Ok(())
    }

    fn size(&self) -> Result<(u16, u16)> {
        let (cols, rows) = termion::terminal_size()?;
        Ok((cols, rows))
    }

    fn clear(&mut self) -> Result<()> {
        write!(self, "{}", clear::All)?;
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<()> {
        write!(self, "{}", cursor::Hide)?;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<()> {
        write!(self, "{}", cursor::Show)?;
        Ok(())
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        // Termion uses 1-based coordinates
        write!(self, "{}", cursor::Goto(x + 1, y + 1))?;
        Ok(())
    }

    fn set_fg(&mut self, color: Color) -> Result<()> {
        write!(self, "{}", Fg(Rgb(color.r, color.g, color.b)))?;
        Ok(())
    }

    fn set_bg(&mut self, color: Color) -> Result<()> {
        write!(self, "{}", Bg(Rgb(color.r, color.g, color.b)))?;
        Ok(())
    }

    fn reset_fg(&mut self) -> Result<()> {
        write!(self, "{}", Fg(color::Reset))?;
        Ok(())
    }

    fn reset_bg(&mut self) -> Result<()> {
        write!(self, "{}", Bg(color::Reset))?;
        Ok(())
    }

    fn set_modifier(&mut self, modifier: Modifier) -> Result<()> {
        if modifier.contains(Modifier::BOLD) {
            write!(self, "{}", Bold)?;
        }
        if modifier.contains(Modifier::ITALIC) {
            write!(self, "{}", Italic)?;
        }
        if modifier.contains(Modifier::UNDERLINE) {
            write!(self, "{}", Underline)?;
        }
        if modifier.contains(Modifier::DIM) {
            write!(self, "{}", Faint)?;
        }
        if modifier.contains(Modifier::CROSSED_OUT) {
            write!(self, "{}", CrossedOut)?;
        }
        Ok(())
    }

    fn reset_style(&mut self) -> Result<()> {
        write!(self, "{}", Reset)?;
        Ok(())
    }

    fn enable_mouse(&mut self) -> Result<()> {
        // Mouse is automatically enabled with MouseTerminal wrapper
        // This is a no-op if already initialized with mouse
        if self.initialized && !self.mouse_enabled {
            // We can't dynamically enable mouse in termion
            // It needs to be set at initialization time
        }
        Ok(())
    }

    fn disable_mouse(&mut self) -> Result<()> {
        // Similar to enable_mouse, termion doesn't support dynamic toggle
        // Mouse capture is determined at initialization
        Ok(())
    }

    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities {
            true_color: true,
            hyperlinks: true,
            mouse: self.mouse_enabled,
            bracketed_paste: false,
            focus_events: false,
        }
    }

    fn name(&self) -> &'static str {
        "termion"
    }
}

impl<W: Write + AsRawFd> Drop for TermionBackend<W> {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Most tests require an actual TTY and cannot be run in CI
    // These are basic unit tests that don't require terminal access

    #[test]
    fn test_backend_name() {
        let backend = TermionBackend::new(io::stdout());
        assert_eq!(backend.name(), "termion");
    }
}
