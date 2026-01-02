//! Backend trait definition
//!
//! Defines the interface that all terminal backends must implement.

use std::io::Write;
use crate::style::Color;
use crate::render::cell::Modifier;
use crate::Result;

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
