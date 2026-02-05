//! Backend trait definition
//!
//! Defines the interface that all terminal backends must implement.

use crate::render::cell::Modifier;
use crate::style::Color;
use crate::Result;
use std::io::Write;

/// Capabilities that a backend may support
#[derive(Debug, Clone, Default)]
pub struct BackendCapabilities {
    /// Supports true color (24-bit RGB)
    pub true_color: bool,
    /// Supports hyperlinks (OSC 8)
    pub hyperlinks: bool,
    /// Supports mouse input
    pub mouse: bool,
    /// Supports bracketed paste
    pub bracketed_paste: bool,
    /// Supports focus events
    pub focus_events: bool,
}

/// Terminal backend trait
///
/// This trait defines the interface for terminal backends. Implementations
/// handle the low-level terminal I/O operations.
///
/// # Responsibilities
///
/// - Terminal initialization and restoration
/// - Cursor control
/// - Color and style management
/// - Screen clearing and flushing
/// - Mouse capture control
pub trait Backend: Write {
    /// Initialize the terminal for TUI mode
    ///
    /// This should:
    /// - Enable raw mode
    /// - Enter alternate screen
    /// - Hide cursor
    /// - Clear the screen
    fn init(&mut self) -> Result<()>;

    /// Initialize with optional mouse capture
    fn init_with_mouse(&mut self, enable_mouse: bool) -> Result<()>;

    /// Restore the terminal to normal mode
    ///
    /// This should:
    /// - Disable raw mode
    /// - Leave alternate screen
    /// - Show cursor
    /// - Reset colors
    fn restore(&mut self) -> Result<()>;

    /// Get terminal size (width, height)
    fn size(&self) -> Result<(u16, u16)>;

    /// Clear the entire screen
    fn clear(&mut self) -> Result<()>;

    /// Hide the cursor
    fn hide_cursor(&mut self) -> Result<()>;

    /// Show the cursor
    fn show_cursor(&mut self) -> Result<()>;

    /// Move cursor to position (x, y)
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<()>;

    /// Set foreground color
    fn set_fg(&mut self, color: Color) -> Result<()>;

    /// Set background color
    fn set_bg(&mut self, color: Color) -> Result<()>;

    /// Reset foreground to default
    fn reset_fg(&mut self) -> Result<()>;

    /// Reset background to default
    fn reset_bg(&mut self) -> Result<()>;

    /// Set text modifier (bold, italic, etc.)
    fn set_modifier(&mut self, modifier: Modifier) -> Result<()>;

    /// Reset all styles to default
    fn reset_style(&mut self) -> Result<()>;

    /// Enable mouse capture
    fn enable_mouse(&mut self) -> Result<()>;

    /// Disable mouse capture
    fn disable_mouse(&mut self) -> Result<()>;

    /// Write a hyperlink start sequence (OSC 8)
    fn write_hyperlink_start(&mut self, url: &str) -> Result<()> {
        write!(self, "\x1b]8;;{}\x1b\\", url)?;
        Ok(())
    }

    /// Write a hyperlink end sequence (OSC 8)
    fn write_hyperlink_end(&mut self) -> Result<()> {
        write!(self, "\x1b]8;;\x1b\\")?;
        Ok(())
    }

    /// Get backend capabilities
    fn capabilities(&self) -> BackendCapabilities {
        BackendCapabilities::default()
    }

    /// Get backend name for debugging
    fn name(&self) -> &'static str;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // Mock backend for testing trait default implementations
    struct MockBackend {
        data: Vec<u8>,
    }

    impl MockBackend {
        fn new() -> Self {
            Self { data: Vec::new() }
        }
    }

    impl Write for MockBackend {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.data.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl Backend for MockBackend {
        fn init(&mut self) -> Result<()> {
            Ok(())
        }

        fn init_with_mouse(&mut self, _enable_mouse: bool) -> Result<()> {
            Ok(())
        }

        fn restore(&mut self) -> Result<()> {
            Ok(())
        }

        fn size(&self) -> Result<(u16, u16)> {
            Ok((80, 24))
        }

        fn clear(&mut self) -> Result<()> {
            Ok(())
        }

        fn hide_cursor(&mut self) -> Result<()> {
            Ok(())
        }

        fn show_cursor(&mut self) -> Result<()> {
            Ok(())
        }

        fn set_cursor(&mut self, _x: u16, _y: u16) -> Result<()> {
            Ok(())
        }

        fn set_fg(&mut self, _color: Color) -> Result<()> {
            Ok(())
        }

        fn set_bg(&mut self, _color: Color) -> Result<()> {
            Ok(())
        }

        fn reset_fg(&mut self) -> Result<()> {
            Ok(())
        }

        fn reset_bg(&mut self) -> Result<()> {
            Ok(())
        }

        fn set_modifier(&mut self, _modifier: Modifier) -> Result<()> {
            Ok(())
        }

        fn reset_style(&mut self) -> Result<()> {
            Ok(())
        }

        fn enable_mouse(&mut self) -> Result<()> {
            Ok(())
        }

        fn disable_mouse(&mut self) -> Result<()> {
            Ok(())
        }

        fn name(&self) -> &'static str {
            "mock"
        }
    }

    #[test]
    fn test_backend_capabilities_default() {
        let caps = BackendCapabilities::default();
        assert!(!caps.true_color);
        assert!(!caps.hyperlinks);
        assert!(!caps.mouse);
        assert!(!caps.bracketed_paste);
        assert!(!caps.focus_events);
    }

    #[test]
    fn test_backend_capabilities_all_true() {
        let caps = BackendCapabilities {
            true_color: true,
            hyperlinks: true,
            mouse: true,
            bracketed_paste: true,
            focus_events: true,
        };
        assert!(caps.true_color);
        assert!(caps.hyperlinks);
        assert!(caps.mouse);
        assert!(caps.bracketed_paste);
        assert!(caps.focus_events);
    }

    #[test]
    fn test_backend_write_hyperlink_start() {
        let mut backend = MockBackend::new();
        let result = backend.write_hyperlink_start("https://example.com");
        assert!(result.is_ok());
        let output = String::from_utf8_lossy(&backend.data);
        assert!(output.contains("\x1b]8;;"));
        assert!(output.contains("https://example.com"));
        assert!(output.contains("\x1b\\"));
    }

    #[test]
    fn test_backend_write_hyperlink_end() {
        let mut backend = MockBackend::new();
        let result = backend.write_hyperlink_end();
        assert!(result.is_ok());
        let output = String::from_utf8_lossy(&backend.data);
        assert_eq!(output, "\x1b]8;;\x1b\\");
    }

    #[test]
    fn test_backend_capabilities_default_impl() {
        let backend = MockBackend::new();
        let caps = backend.capabilities();
        // Default implementation returns default capabilities
        assert!(!caps.true_color);
        assert!(!caps.hyperlinks);
    }

    #[test]
    fn test_backend_name() {
        let backend = MockBackend::new();
        assert_eq!(backend.name(), "mock");
    }
}
