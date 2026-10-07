//! Crossterm backend implementation
//!
//! This backend uses the crossterm library for cross-platform terminal I/O.

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{
        DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
        EnableFocusChange, EnableMouseCapture,
    },
    execute, queue,
    style::{
        Attribute, Color as CrosstermColor, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::{self, Write};

use super::traits::{Backend, BackendCapabilities};
use crate::render::cell::Modifier;
use crate::style::Color;
use crate::Result;

/// Crossterm-based terminal backend
///
/// This is the default backend, providing cross-platform support
/// for Windows, macOS, and Linux.
pub struct CrosstermBackend<W: Write> {
    writer: W,
    raw_mode: bool,
    mouse_enabled: bool,
    bracketed_paste_enabled: bool,
    focus_events_enabled: bool,
}

impl<W: Write> CrosstermBackend<W> {
    /// Create a new crossterm backend with the given writer
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            raw_mode: false,
            mouse_enabled: false,
            bracketed_paste_enabled: false,
            focus_events_enabled: false,
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
        // `Drop` does not run on abort, and the release profile aborts on panic.
        // The hook is what actually restores the terminal - see `panic_hook`.
        crate::render::install_panic_hook();

        if enable_mouse {
            execute!(
                self.writer,
                EnterAlternateScreen,
                EnableMouseCapture,
                EnableBracketedPaste,
                EnableFocusChange,
                Hide,
                Clear(ClearType::All)
            )?;
            self.mouse_enabled = true;
        } else {
            execute!(
                self.writer,
                EnterAlternateScreen,
                EnableBracketedPaste,
                EnableFocusChange,
                Hide,
                Clear(ClearType::All)
            )?;
        }
        self.bracketed_paste_enabled = true;
        self.focus_events_enabled = true;
        Ok(())
    }

    fn restore(&mut self) -> Result<()> {
        if self.raw_mode {
            let mouse_enabled = self.mouse_enabled;
            self.raw_mode = false;
            self.mouse_enabled = false;
            self.bracketed_paste_enabled = false;
            self.focus_events_enabled = false;
            // Restore once per session: if the panic hook already did, leaving
            // the alternate screen again would move the cursor back over the
            // panic message. See `panic_hook`.
            if !crate::runtime::render::terminal::panic_hook::claim_restore() {
                return Ok(());
            }
            let written = if mouse_enabled {
                execute!(
                    self.writer,
                    DisableMouseCapture,
                    DisableBracketedPaste,
                    DisableFocusChange,
                    ResetColor,
                    Show,
                    LeaveAlternateScreen
                )
            } else {
                execute!(
                    self.writer,
                    DisableBracketedPaste,
                    DisableFocusChange,
                    ResetColor,
                    Show,
                    LeaveAlternateScreen
                )
            };
            // There is no second attempt, so leave raw mode even if the
            // writer refused the sequence.
            let raw = disable_raw_mode();
            written?;
            raw?;
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
    use crate::runtime::render::terminal::panic_hook::restores_performed;

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

    /// A backend as `init_with_mouse` leaves it, minus the parts that need a
    /// real TTY.
    fn live_backend() -> CrosstermBackend<MockWriter> {
        let mut backend = CrosstermBackend::new(MockWriter::new());
        backend.raw_mode = true;
        backend.mouse_enabled = true;
        crate::render::install_panic_hook();
        backend
    }

    #[cfg(not(windows))]
    fn leaves_alt_screen(backend: &CrosstermBackend<MockWriter>) -> usize {
        String::from_utf8_lossy(&backend.writer().buffer)
            .matches("\x1b[?1049l")
            .count()
    }

    #[test]
    #[serial_test::serial]
    fn restore_leaves_the_alternate_screen_once() {
        let mut backend = live_backend();
        let before = restores_performed();

        // Raw mode is only faked here, and Windows refuses to leave a raw mode
        // that was never entered, so the result is not what is under test.
        let _ = backend.restore();
        let _ = backend.restore();

        assert_eq!(restores_performed() - before, 1);
        // Where crossterm speaks ANSI, the bytes agree. On Windows it drives
        // the console through WinAPI and writes nothing to count.
        #[cfg(not(windows))]
        assert_eq!(leaves_alt_screen(&backend), 1);
    }

    /// The unwinding `Drop` after a panic must not undo the panic hook's work
    /// by leaving the alternate screen a second time.
    #[test]
    #[serial_test::serial]
    fn restore_after_the_panic_hook_writes_nothing() {
        let mut backend = live_backend();

        assert!(crate::runtime::render::terminal::panic_hook::claim_restore());
        let before = restores_performed();

        // Raw mode is only faked here, and Windows refuses to leave a raw mode
        // that was never entered, so the result is not what is under test.
        let _ = backend.restore();

        assert_eq!(restores_performed() - before, 0);
        #[cfg(not(windows))]
        assert_eq!(leaves_alt_screen(&backend), 0);
        assert!(!backend.raw_mode);
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
