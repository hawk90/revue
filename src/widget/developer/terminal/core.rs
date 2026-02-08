//! Terminal widget core implementation

#![allow(clippy::explicit_counter_loop)]
//! Terminal widget for embedded terminal emulator
//!
//! Provides an embedded terminal with ANSI color support and scrollback.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::{Key, KeyEvent, MouseButton, MouseEventKind};
    use crate::style::Color;

    // =========================================================================
    // Constructor tests
    // =========================================================================

    #[test]
    fn test_terminal_new() {
        let terminal = Terminal::new(80, 24);
        assert_eq!(terminal.width, 80);
        assert_eq!(terminal.height, 24);
        assert_eq!(terminal.lines.len(), 24);
    }

    #[test]
    fn test_terminal_default() {
        let terminal = Terminal::default();
        assert_eq!(terminal.width, 80);
        assert_eq!(terminal.height, 24);
    }

    #[test]
    fn test_terminal_shell() {
        let terminal = Terminal::shell(80, 24);
        assert_eq!(terminal.width, 80);
        assert_eq!(terminal.height, 24);
    }

    #[test]
    fn test_terminal_log_viewer() {
        let terminal = Terminal::log_viewer(100, 30);
        assert_eq!(terminal.width, 100);
        assert_eq!(terminal.height, 30);
        assert!(!terminal.show_cursor);
        assert_eq!(terminal.max_scrollback, 50000);
    }

    // =========================================================================
    // Builder tests
    // =========================================================================

    #[test]
    fn test_terminal_max_scrollback() {
        let terminal = Terminal::new(80, 24).max_scrollback(5000);
        assert_eq!(terminal.max_scrollback, 5000);
    }

    #[test]
    fn test_terminal_default_fg() {
        let terminal = Terminal::new(80, 24).default_fg(Color::RED);
        assert_eq!(terminal.default_fg, Color::RED);
    }

    #[test]
    fn test_terminal_default_bg() {
        let terminal = Terminal::new(80, 24).default_bg(Color::BLUE);
        assert_eq!(terminal.default_bg, Color::BLUE);
    }

    #[test]
    fn test_terminal_show_cursor() {
        let terminal = Terminal::new(80, 24).show_cursor(false);
        assert!(!terminal.show_cursor);
    }

    #[test]
    fn test_terminal_cursor_style() {
        use super::super::types::CursorStyle;
        let terminal = Terminal::new(80, 24).cursor_style(CursorStyle::Bar);
        assert!(matches!(terminal.cursor_style, CursorStyle::Bar));
    }

    #[test]
    fn test_terminal_title() {
        let terminal = Terminal::new(80, 24).title("Test Terminal");
        assert_eq!(terminal.get_title(), Some("Test Terminal"));
    }

    // =========================================================================
    // Focus tests
    // =========================================================================

    #[test]
    fn test_terminal_focus() {
        let mut terminal = Terminal::new(80, 24);
        terminal.focus();
        assert!(terminal.is_focused());
    }

    #[test]
    fn test_terminal_blur() {
        let mut terminal = Terminal::new(80, 24);
        terminal.focus();
        terminal.blur();
        assert!(!terminal.is_focused());
    }

    #[test]
    fn test_terminal_is_focused() {
        let terminal = Terminal::new(80, 24);
        assert!(!terminal.is_focused());
    }

    // =========================================================================
    // Write tests
    // =========================================================================

    #[test]
    fn test_terminal_write_basic() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("hello");
        assert!(terminal.lines[0].cells.len() >= 5);
    }

    #[test]
    fn test_terminal_write_empty() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("");
        // Should not panic
    }

    #[test]
    fn test_terminal_write_multiple() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("hello");
        terminal.write(" ");
        terminal.write("world");
        // Should concatenate on same line
    }

    #[test]
    fn test_terminal_write_newline() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("line1\nline2");
        assert!(terminal.lines.len() >= 2);
    }

    #[test]
    fn test_terminal_write_carriage_return() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("hello\rworld");
        // Carriage return should move to start of line
    }

    #[test]
    fn test_terminal_write_tab() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("\t");
        // Tab should advance to next 8-column boundary
    }

    #[test]
    fn test_terminal_writeln() {
        let mut terminal = Terminal::new(80, 24);
        terminal.writeln("hello");
        terminal.writeln("world");
        assert!(terminal.lines.len() >= 2);
    }

    // =========================================================================
    // Clear tests
    // =========================================================================

    #[test]
    fn test_terminal_clear() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("hello world");
        terminal.clear();
        assert_eq!(terminal.cursor_row, 0);
        assert_eq!(terminal.cursor_col, 0);
    }

    #[test]
    fn test_terminal_clear_line() {
        let mut terminal = Terminal::new(80, 24);
        terminal.write("hello");
        terminal.clear_line();
        assert_eq!(terminal.cursor_col, 0);
        assert!(terminal.lines[terminal.cursor_row].cells.is_empty());
    }

    // =========================================================================
    // Scroll tests
    // =========================================================================

    #[test]
    fn test_terminal_scroll_up() {
        let mut terminal = Terminal::new(80, 24);
        for i in 0..30 {
            terminal.writeln(&format!("line {}", i));
        }
        terminal.scroll_up(5);
        assert_eq!(terminal.scroll_offset, 5);
    }

    #[test]
    fn test_terminal_scroll_down() {
        let mut terminal = Terminal::new(80, 24);
        for i in 0..30 {
            terminal.writeln(&format!("line {}", i));
        }
        terminal.scroll_up(10);
        terminal.scroll_down(5);
        assert_eq!(terminal.scroll_offset, 2);
    }

    #[test]
    fn test_terminal_scroll_to_bottom() {
        let mut terminal = Terminal::new(80, 24);
        terminal.scroll_up(10);
        terminal.scroll_to_bottom();
        assert_eq!(terminal.scroll_offset, 0);
    }

    #[test]
    fn test_terminal_scroll_to_top() {
        let mut terminal = Terminal::new(80, 24);
        for i in 0..30 {
            terminal.writeln(&format!("line {}", i));
        }
        terminal.scroll_to_top();
        assert!(terminal.scroll_offset > 0);
    }

    // =========================================================================
    // Input tests
    // =========================================================================

    #[test]
    fn test_terminal_get_input() {
        let terminal = Terminal::new(80, 24);
        assert_eq!(terminal.get_input(), "");
    }

    #[test]
    fn test_terminal_clear_input() {
        let mut terminal = Terminal::new(80, 24);
        terminal.input_buffer = "test".to_string();
        terminal.clear_input();
        assert_eq!(terminal.get_input(), "");
    }

    // =========================================================================
    // Handle key tests
    // =========================================================================

    #[test]
    fn test_terminal_handle_key_char() {
        let mut terminal = Terminal::new(80, 24);
        let key = KeyEvent::new(Key::Char('a'));
        let result = terminal.handle_key(key);
        assert!(result.is_none());
        assert_eq!(terminal.input_buffer, "a");
    }

    #[test]
    fn test_terminal_handle_key_backspace() {
        let mut terminal = Terminal::new(80, 24);
        terminal.input_buffer = "abc".to_string();
        let key = KeyEvent::new(Key::Backspace);
        let result = terminal.handle_key(key);
        assert!(result.is_none());
        assert_eq!(terminal.input_buffer, "ab");
    }

    #[test]
    fn test_terminal_handle_key_enter() {
        use super::super::types::TerminalAction;
        let mut terminal = Terminal::new(80, 24);
        terminal.input_buffer = "test".to_string();
        let key = KeyEvent::new(Key::Enter);
        let result = terminal.handle_key(key);
        assert!(matches!(result, Some(TerminalAction::Submit(_))));
        assert!(terminal.input_buffer.is_empty());
    }

    #[test]
    fn test_terminal_handle_key_up_history() {
        let mut terminal = Terminal::new(80, 24);
        terminal.history.push("cmd1".to_string());
        terminal.history.push("cmd2".to_string());
        terminal.history_pos = 2;
        let key = KeyEvent::new(Key::Up);
        terminal.handle_key(key);
        assert_eq!(terminal.input_buffer, "cmd2");
    }

    #[test]
    fn test_terminal_handle_key_down_history() {
        let mut terminal = Terminal::new(80, 24);
        terminal.history.push("cmd1".to_string());
        terminal.history_pos = 1;
        let key = KeyEvent::new(Key::Down);
        terminal.handle_key(key);
        assert_eq!(terminal.input_buffer, "");
    }

    #[test]
    fn test_terminal_handle_key_page_up() {
        let mut terminal = Terminal::new(80, 24);
        for i in 0..30 {
            terminal.writeln(&format!("line {}", i));
        }
        let key = KeyEvent::new(Key::PageUp);
        terminal.handle_key(key);
        assert!(terminal.scroll_offset > 0);
    }

    #[test]
    fn test_terminal_handle_key_page_down() {
        let mut terminal = Terminal::new(80, 24);
        terminal.scroll_up(10);
        let key = KeyEvent::new(Key::PageDown);
        terminal.handle_key(key);
        assert!(terminal.scroll_offset < 10);
    }

    #[test]
    fn test_terminal_handle_key_home_clears_input() {
        let mut terminal = Terminal::new(80, 24);
        terminal.input_buffer = "test".to_string();
        let key = KeyEvent::new(Key::Home);
        terminal.handle_key(key);
        assert!(terminal.input_buffer.is_empty());
    }

    #[test]
    fn test_terminal_handle_key_end_scroll_to_bottom() {
        let mut terminal = Terminal::new(80, 24);
        terminal.scroll_up(10);
        let key = KeyEvent::new(Key::End);
        terminal.handle_key(key);
        assert_eq!(terminal.scroll_offset, 0);
    }

    #[test]
    fn test_terminal_handle_key_escape() {
        use super::super::types::TerminalAction;
        let mut terminal = Terminal::new(80, 24);
        let key = KeyEvent::new(Key::Escape);
        let result = terminal.handle_key(key);
        assert!(matches!(result, Some(TerminalAction::Cancel)));
    }

    #[test]
    fn test_terminal_handle_key_tab() {
        use super::super::types::TerminalAction;
        let mut terminal = Terminal::new(80, 24);
        terminal.input_buffer = "test".to_string();
        let key = KeyEvent::new(Key::Tab);
        let result = terminal.handle_key(key);
        assert!(matches!(result, Some(TerminalAction::TabComplete(_))));
    }

    // =========================================================================
    // Resize tests
    // =========================================================================

    #[test]
    fn test_terminal_resize() {
        let mut terminal = Terminal::new(80, 24);
        terminal.resize(100, 30);
        assert_eq!(terminal.width, 100);
        assert_eq!(terminal.height, 30);
    }

    #[test]
    fn test_terminal_resize_adds_lines() {
        let mut terminal = Terminal::new(10, 5);
        terminal.resize(10, 10);
        assert!(terminal.lines.len() >= 10);
    }
}

use super::ansi::AnsiParser;
use super::types::{CursorStyle, TermCell, TermLine, TerminalAction};
use crate::event::{Key, KeyEvent};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Embedded terminal widget
pub struct Terminal {
    /// Terminal lines (scrollback + visible)
    lines: Vec<TermLine>,
    /// Current cursor row (relative to lines)
    cursor_row: usize,
    /// Current cursor column
    cursor_col: usize,
    /// Scroll offset (0 = at bottom, showing latest)
    scroll_offset: usize,
    /// Maximum scrollback lines
    max_scrollback: usize,
    /// Terminal width
    width: u16,
    /// Terminal height
    height: u16,
    /// ANSI parser
    parser: AnsiParser,
    /// Default foreground color
    default_fg: Color,
    /// Default background color
    default_bg: Color,
    /// Show cursor
    show_cursor: bool,
    /// Cursor style
    cursor_style: CursorStyle,
    /// Title
    title: Option<String>,
    /// Input buffer for command mode
    input_buffer: String,
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
        let mut lines = Vec::with_capacity(height as usize);
        for _ in 0..height {
            lines.push(TermLine::with_capacity(width as usize));
        }

        Self {
            lines,
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
            max_scrollback: 10000,
            width,
            height,
            parser: AnsiParser::new(),
            default_fg: Color::WHITE,
            default_bg: Color::BLACK,
            show_cursor: true,
            cursor_style: CursorStyle::Block,
            title: None,
            input_buffer: String::new(),
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
        self.default_fg = color;
        self.parser.reset_fg(color);
        self
    }

    /// Set default background color
    pub fn default_bg(mut self, color: Color) -> Self {
        self.default_bg = color;
        self.parser.reset_bg(color);
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

    /// Write text to terminal (with ANSI support)
    pub fn write(&mut self, text: &str) {
        for ch in text.chars() {
            self.write_char(ch);
        }
    }

    /// Write a single character
    fn write_char(&mut self, ch: char) {
        if ch == '\n' {
            self.newline();
            return;
        }
        if ch == '\r' {
            self.cursor_col = 0;
            return;
        }
        if ch == '\t' {
            // Tab to next 8-column boundary
            let next_tab = ((self.cursor_col / 8) + 1) * 8;
            while self.cursor_col < next_tab && self.cursor_col < self.width as usize {
                self.write_char(' ');
            }
            return;
        }

        if let Some(cell) = self.parser.parse(ch) {
            self.put_cell(cell);
        }
    }

    /// Put a cell at current cursor position
    fn put_cell(&mut self, cell: TermCell) {
        // Ensure we have enough lines
        while self.cursor_row >= self.lines.len() {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }

        // Get current line
        let line = &mut self.lines[self.cursor_row];

        // Ensure line has enough cells
        while line.cells.len() <= self.cursor_col {
            line.cells.push(TermCell::default());
        }

        // Set cell
        line.cells[self.cursor_col] = cell;

        // Advance cursor
        self.cursor_col += 1;

        // Handle line wrapping
        if self.cursor_col >= self.width as usize {
            self.cursor_col = 0;
            self.cursor_row += 1;
            if self.cursor_row >= self.lines.len() {
                let mut new_line = TermLine::with_capacity(self.width as usize);
                new_line.wrapped = true;
                self.lines.push(new_line);
            } else {
                self.lines[self.cursor_row].wrapped = true;
            }
            self.trim_scrollback();
        }
    }

    /// Move to new line
    fn newline(&mut self) {
        self.cursor_col = 0;
        self.cursor_row += 1;

        while self.cursor_row >= self.lines.len() {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }

        self.trim_scrollback();
    }

    /// Trim scrollback to max
    fn trim_scrollback(&mut self) {
        let total_lines = self.height as usize + self.max_scrollback;
        while self.lines.len() > total_lines {
            self.lines.remove(0);
            if self.cursor_row > 0 {
                self.cursor_row -= 1;
            }
        }
    }

    /// Write a line with automatic newline
    pub fn writeln(&mut self, text: &str) {
        self.write(text);
        self.write("\n");
    }

    /// Clear the terminal
    pub fn clear(&mut self) {
        self.lines.clear();
        for _ in 0..self.height {
            self.lines
                .push(TermLine::with_capacity(self.width as usize));
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.scroll_offset = 0;
    }

    /// Clear current line
    pub fn clear_line(&mut self) {
        if self.cursor_row < self.lines.len() {
            self.lines[self.cursor_row].cells.clear();
        }
        self.cursor_col = 0;
    }

    /// Scroll up
    pub fn scroll_up(&mut self, lines: usize) {
        let max_scroll = self.lines.len().saturating_sub(self.height as usize);
        self.scroll_offset = (self.scroll_offset + lines).min(max_scroll);
    }

    /// Scroll down
    pub fn scroll_down(&mut self, lines: usize) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = self.lines.len().saturating_sub(self.height as usize);
    }

    /// Get input buffer
    pub fn get_input(&self) -> &str {
        &self.input_buffer
    }

    /// Clear input buffer
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
    }

    /// Handle key event
    pub fn handle_key(&mut self, key: KeyEvent) -> Option<TerminalAction> {
        match key.key {
            Key::Char(c) => {
                self.input_buffer.push(c);
                None
            }
            Key::Backspace => {
                self.input_buffer.pop();
                None
            }
            Key::Enter => {
                let input = std::mem::take(&mut self.input_buffer);
                if !input.is_empty() {
                    self.history.push(input.clone());
                    self.history_pos = self.history.len();
                }
                Some(TerminalAction::Submit(input))
            }
            Key::Up => {
                if self.history_pos > 0 {
                    self.history_pos -= 1;
                    self.input_buffer = self
                        .history
                        .get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
                }
                None
            }
            Key::Down => {
                if self.history_pos < self.history.len() {
                    self.history_pos += 1;
                    self.input_buffer = self
                        .history
                        .get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
                }
                None
            }
            Key::PageUp => {
                self.scroll_up(self.height as usize / 2);
                None
            }
            Key::PageDown => {
                self.scroll_down(self.height as usize / 2);
                None
            }
            Key::Home => {
                self.input_buffer.clear();
                None
            }
            Key::End => {
                self.scroll_to_bottom();
                None
            }
            Key::Escape => Some(TerminalAction::Cancel),
            Key::Tab => Some(TerminalAction::TabComplete(self.input_buffer.clone())),
            _ => None,
        }
    }

    /// Resize terminal
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        // Add lines if needed
        while self.lines.len() < height as usize {
            self.lines.push(TermLine::with_capacity(width as usize));
        }
    }

    /// Get visible lines range
    fn visible_range(&self) -> std::ops::Range<usize> {
        let total = self.lines.len();
        let visible = self.height as usize;
        let end = total.saturating_sub(self.scroll_offset);
        let start = end.saturating_sub(visible);
        start..end
    }

    // Presets

    /// Create a shell-style terminal
    pub fn shell(width: u16, height: u16) -> Self {
        Self::new(width, height)
            .default_fg(Color::rgb(200, 200, 200))
            .default_bg(Color::rgb(30, 30, 30))
            .cursor_style(CursorStyle::Block)
    }

    /// Create a log viewer terminal
    pub fn log_viewer(width: u16, height: u16) -> Self {
        Self::new(width, height)
            .default_fg(Color::rgb(180, 180, 180))
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

impl View for Terminal {
    crate::impl_view_meta!("Terminal");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        // Fill background
        for y in 0..area.height {
            for x in 0..area.width {
                ctx.buffer
                    .set(area.x + x, area.y + y, Cell::new(' ').bg(self.default_bg));
            }
        }

        // Get visible lines
        let visible = self.visible_range();
        let mut render_y = 0u16;

        for line_idx in visible {
            if render_y >= area.height {
                break;
            }

            if let Some(line) = self.lines.get(line_idx) {
                for (col, cell) in line.cells.iter().enumerate() {
                    if col >= area.width as usize {
                        break;
                    }

                    let mut render_cell = Cell::new(cell.ch).fg(cell.fg).bg(cell.bg);
                    render_cell.modifier = cell.modifiers;
                    ctx.buffer
                        .set(area.x + col as u16, area.y + render_y, render_cell);
                }
            }

            render_y += 1;
        }

        // Render cursor if visible and focused
        if self.show_cursor && self.focused && self.scroll_offset == 0 {
            let cursor_screen_row = self
                .cursor_row
                .saturating_sub(self.lines.len().saturating_sub(self.height as usize));

            if cursor_screen_row < area.height as usize && self.cursor_col < area.width as usize {
                let cursor_x = area.x + self.cursor_col as u16;
                let cursor_y = area.y + cursor_screen_row as u16;

                let cursor_char = match self.cursor_style {
                    CursorStyle::Block => '█',
                    CursorStyle::Underline => '_',
                    CursorStyle::Bar => '│',
                };

                ctx.buffer.set(
                    cursor_x,
                    cursor_y,
                    Cell::new(cursor_char).fg(self.default_fg),
                );
            }
        }

        // Render input line if there's input
        if !self.input_buffer.is_empty() && self.focused {
            let input_y = area.y + area.height - 1;
            let prompt = "> ";

            // Clear input line
            for x in 0..area.width {
                ctx.buffer.set(
                    area.x + x,
                    input_y,
                    Cell::new(' ').bg(Color::rgb(40, 40, 40)),
                );
            }

            // Render prompt
            for (i, ch) in prompt.chars().enumerate() {
                if i >= area.width as usize {
                    break;
                }
                ctx.buffer.set(
                    area.x + i as u16,
                    input_y,
                    Cell::new(ch).fg(Color::CYAN).bg(Color::rgb(40, 40, 40)),
                );
            }

            // Render input
            for (i, ch) in self.input_buffer.chars().enumerate() {
                let x = prompt.len() + i;
                if x >= area.width as usize {
                    break;
                }
                ctx.buffer.set(
                    area.x + x as u16,
                    input_y,
                    Cell::new(ch).fg(Color::WHITE).bg(Color::rgb(40, 40, 40)),
                );
            }
        }

        // Render scroll indicator if scrolled
        if self.scroll_offset > 0 {
            let indicator = format!("↑{}", self.scroll_offset);
            let start_x = area.x + area.width - indicator.len() as u16 - 1;

            for (i, ch) in indicator.chars().enumerate() {
                ctx.buffer.set(
                    start_x + i as u16,
                    area.y,
                    Cell::new(ch).fg(Color::YELLOW).bg(Color::rgb(60, 60, 60)),
                );
            }
        }
    }
}

impl_styled_view!(Terminal);
impl_props_builders!(Terminal);

/// Helper function to create a terminal widget
pub fn terminal(width: u16, height: u16) -> Terminal {
    Terminal::new(width, height)
}
