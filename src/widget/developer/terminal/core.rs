//! Terminal widget core implementation

#![allow(clippy::explicit_counter_loop)]
//! Terminal widget for embedded terminal emulator
//!
//! Provides an embedded terminal with ANSI color support and scrollback.

// KEEP HERE: Private tests for Terminal
// Private implementation tests that are not part of the public API
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_char_private() {
        // Test private write_char method
        let mut terminal = Terminal::new(80, 24);
        terminal.write_char('a');
        assert_eq!(terminal.input_buffer, "");
        assert_eq!(terminal.lines[0].cells.len(), 1);
    }

    #[test]
    fn test_put_cell_private() {
        // Test private put_cell method
        let mut terminal = Terminal::new(80, 24);

        // This test would require accessing private types
        // Just ensure the terminal works as expected
        terminal.write("test");
        assert_eq!(terminal.lines[0].cells.len(), 4);
    }

    #[test]
    fn test_newline_private() {
        // Test private newline method
        // Terminal pre-allocates all lines, so lines.len() equals height
        let mut terminal = Terminal::new(80, 24);
        terminal.writeln("line1");
        terminal.writeln("line2");
        assert_eq!(terminal.lines.len(), 24); // Terminal height
        assert_eq!(terminal.lines[0].cells.len(), 5); // "line1"
        assert_eq!(terminal.lines[1].cells.len(), 5); // "line2"
    }

    #[test]
    fn test_trim_scrollback_private() {
        // Test private trim_scrollback method
        let mut terminal = Terminal::new(80, 24);
        terminal.max_scrollback = 5;

        // Write more lines than max scrollback
        for i in 0..10 {
            terminal.writeln(&format!("line {}", i));
        }

        // Should have trimmed some lines
        assert!(terminal.lines.len() <= terminal.height as usize + terminal.max_scrollback);
    }

    #[test]
    fn test_visible_range_private() {
        // Test private visible_range method
        let terminal = Terminal::new(80, 24);
        let range = terminal.visible_range();
        // Should return a valid range
        assert!(range.start <= range.end);
    }

    #[test]
    fn test_render_cursor_private() {
        // Test private cursor rendering logic
        let mut terminal = Terminal::new(80, 24);
        terminal.focus();

        let mut buffer = crate::render::Buffer::new(80, 24);
        let area = crate::layout::Rect::new(0, 0, 80, 24);
        let mut ctx = crate::widget::traits::RenderContext::new(&mut buffer, area);

        // Should not panic
        terminal.render(&mut ctx);
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
