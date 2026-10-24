//! Terminal widget for embedded terminal emulator
//!
//! Provides an embedded terminal with ANSI color support and scrollback.

pub use types::{CursorStyle, TermCell, TermLine, TerminalAction};

mod ansi;
mod input;
mod screen;
mod scroll;
#[cfg(test)]
mod tests;
mod types;
mod view;

use crate::style::Color;
use crate::utils::{EditState, ScrollState};
use crate::widget::theme::{EDITOR_BG, MUTED_TEXT, SECONDARY_TEXT};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};
use ansi::AnsiParser;

/// Embedded terminal widget
#[derive(Clone)]
pub struct Terminal {
    /// Terminal lines (scrollback + visible)
    lines: Vec<TermLine>,
    /// Current cursor row (relative to lines)
    cursor_row: usize,
    /// Current cursor column
    cursor_col: usize,
    /// Where the lines are scrolled to, from the top: it follows the newest
    /// line until scrolled back, and a scrolled-back view stays on its lines
    scroll: ScrollState,
    /// Maximum scrollback lines
    max_scrollback: usize,
    /// Terminal width
    width: u16,
    /// Terminal height
    height: u16,
    /// ANSI parser
    parser: AnsiParser,
    /// Default foreground color
    /// The color the builder named, if it named one - see #656.
    default_fg: Option<Color>,
    /// Default background color
    /// The color the builder named, if it named one - see #656.
    default_bg: Option<Color>,
    /// Show cursor
    show_cursor: bool,
    /// Cursor style
    cursor_style: CursorStyle,
    /// Title
    title: Option<String>,
    /// The command line, edited as every single-line field is (#985)
    input_buffer: EditState,
    /// Horizontal scroll of the command line, in columns
    input_scroll: std::cell::Cell<usize>,
    /// Command history
    history: Vec<String>,
    /// History position
    history_pos: usize,
    /// Is focused
    focused: bool,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl Terminal {
    /// Create a new terminal
    pub fn new(width: u16, height: u16) -> Self {
        // Clamp to reasonable terminal dimensions to prevent excessive allocation
        let width = width.min(1000);
        let height = height.min(1000);
        let mut lines = Vec::with_capacity(height as usize);
        for _ in 0..height {
            lines.push(TermLine::with_capacity(width as usize));
        }

        let mut scroll = ScrollState::new().follow_end(true);
        scroll.set_content(lines.len());
        scroll.set_viewport(height as usize);
        scroll.scroll_to_end();

        Self {
            lines,
            cursor_row: 0,
            cursor_col: 0,
            scroll,
            max_scrollback: 10000,
            width,
            height,
            parser: AnsiParser::new(),
            default_fg: None,
            default_bg: None,
            show_cursor: true,
            cursor_style: CursorStyle::Block,
            title: None,
            input_buffer: EditState::single_line(),
            input_scroll: std::cell::Cell::new(0),
            history: Vec::new(),
            history_pos: 0,
            focused: false,
            props: WidgetProps::new(),
        }
    }

    /// Set maximum scrollback lines
    pub fn max_scrollback(mut self, lines: usize) -> Self {
        self.max_scrollback = lines;
        self
    }

    /// Set default foreground color
    pub fn default_fg(mut self, color: Color) -> Self {
        self.default_fg = Some(color);
        self
    }

    /// Set default background color
    pub fn default_bg(mut self, color: Color) -> Self {
        self.default_bg = Some(color);
        self
    }

    /// Show/hide cursor
    pub fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        self
    }

    /// Set cursor style
    pub fn cursor_style(mut self, style: CursorStyle) -> Self {
        self.cursor_style = style;
        self
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Get title
    pub fn get_title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Focus the terminal
    pub fn focus(&mut self) {
        self.focused = true;
    }

    /// Unfocus the terminal
    pub fn blur(&mut self) {
        self.focused = false;
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    // Presets

    /// Create a shell-style terminal
    pub fn shell(width: u16, height: u16) -> Self {
        Self::new(width, height)
            .default_fg(SECONDARY_TEXT)
            .default_bg(EDITOR_BG)
            .cursor_style(CursorStyle::Block)
    }

    /// Create a log viewer terminal
    pub fn log_viewer(width: u16, height: u16) -> Self {
        Self::new(width, height)
            .default_fg(MUTED_TEXT)
            .default_bg(Color::rgb(20, 20, 20))
            .show_cursor(false)
            .max_scrollback(50000)
    }
}

impl Default for Terminal {
    fn default() -> Self {
        Self::new(80, 24)
    }
}

impl_styled_view!(Terminal);
impl_props_builders!(Terminal);

/// Helper function to create a terminal widget
pub fn terminal(width: u16, height: u16) -> Terminal {
    Terminal::new(width, height)
}
