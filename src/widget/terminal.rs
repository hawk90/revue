//! Terminal widget for embedded terminal emulator
//!
//! Provides an embedded terminal with ANSI color support and scrollback.

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::event::{KeyEvent, Key};
use crate::{impl_styled_view, impl_props_builders};

/// Terminal cell with character and styling
#[derive(Clone, Debug)]
pub struct TermCell {
    /// Character
    pub ch: char,
    /// Foreground color
    pub fg: Color,
    /// Background color
    pub bg: Color,
    /// Text modifiers
    pub modifiers: Modifier,
}

impl Default for TermCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: Color::WHITE,
            bg: Color::BLACK,
            modifiers: Modifier::empty(),
        }
    }
}

impl TermCell {
    /// Create a new terminal cell
    pub fn new(ch: char) -> Self {
        Self {
            ch,
            ..Default::default()
        }
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set modifiers
    pub fn modifiers(mut self, modifiers: Modifier) -> Self {
        self.modifiers = modifiers;
        self
    }
}

/// ANSI parser state
#[derive(Clone, Debug, Default)]
struct AnsiParser {
    /// Current state
    state: ParserState,
    /// CSI parameters
    params: Vec<u16>,
    /// Current parameter being built
    current_param: Option<u16>,
    /// Current foreground
    fg: Color,
    /// Current background
    bg: Color,
    /// Current modifiers
    modifiers: Modifier,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum ParserState {
    #[default]
    Normal,
    Escape,
    Csi,
    OscStart,
    Osc,
}

impl AnsiParser {
    fn new() -> Self {
        Self::default()
    }

    fn reset_attrs(&mut self) {
        self.fg = Color::WHITE;
        self.bg = Color::BLACK;
        self.modifiers = Modifier::empty();
    }

    /// Parse a character and return cell if printable
    fn parse(&mut self, ch: char) -> Option<TermCell> {
        match self.state {
            ParserState::Normal => {
                if ch == '\x1b' {
                    self.state = ParserState::Escape;
                    None
                } else if ch.is_control() && ch != '\n' && ch != '\r' && ch != '\t' {
                    None
                } else {
                    Some(TermCell {
                        ch,
                        fg: self.fg,
                        bg: self.bg,
                        modifiers: self.modifiers,
                    })
                }
            }
            ParserState::Escape => {
                match ch {
                    '[' => {
                        self.state = ParserState::Csi;
                        self.params.clear();
                        self.current_param = None;
                        None
                    }
                    ']' => {
                        self.state = ParserState::OscStart;
                        None
                    }
                    _ => {
                        self.state = ParserState::Normal;
                        None
                    }
                }
            }
            ParserState::Csi => {
                if ch.is_ascii_digit() {
                    let digit = ch.to_digit(10).unwrap() as u16;
                    self.current_param = Some(
                        self.current_param.unwrap_or(0).saturating_mul(10).saturating_add(digit)
                    );
                    None
                } else if ch == ';' {
                    self.params.push(self.current_param.unwrap_or(0));
                    self.current_param = None;
                    None
                } else {
                    // End of CSI sequence
                    if let Some(p) = self.current_param {
                        self.params.push(p);
                    }
                    self.handle_csi(ch);
                    self.state = ParserState::Normal;
                    None
                }
            }
            ParserState::OscStart => {
                self.state = ParserState::Osc;
                None
            }
            ParserState::Osc => {
                // OSC sequences end with BEL or ST
                if ch == '\x07' || ch == '\\' {
                    self.state = ParserState::Normal;
                }
                None
            }
        }
    }

    fn handle_csi(&mut self, cmd: char) {
        match cmd {
            'm' => self.handle_sgr(),
            // Other CSI commands can be added here
            _ => {}
        }
    }

    fn handle_sgr(&mut self) {
        if self.params.is_empty() {
            self.reset_attrs();
            return;
        }

        let mut i = 0;
        while i < self.params.len() {
            match self.params[i] {
                0 => self.reset_attrs(),
                1 => self.modifiers |= Modifier::BOLD,
                2 => self.modifiers |= Modifier::DIM,
                3 => self.modifiers |= Modifier::ITALIC,
                4 => self.modifiers |= Modifier::UNDERLINE,
                5 | 6 => {} // Blink not supported
                7 => {} // Reverse not supported
                8 => {} // Hidden not supported
                9 => self.modifiers |= Modifier::CROSSED_OUT,
                22 => self.modifiers &= !(Modifier::BOLD | Modifier::DIM),
                23 => self.modifiers &= !Modifier::ITALIC,
                24 => self.modifiers &= !Modifier::UNDERLINE,
                25 => {} // Blink not supported
                27 => {} // Reverse not supported
                28 => {} // Hidden not supported
                29 => self.modifiers &= !Modifier::CROSSED_OUT,
                // Standard foreground colors
                30 => self.fg = Color::BLACK,
                31 => self.fg = Color::RED,
                32 => self.fg = Color::GREEN,
                33 => self.fg = Color::YELLOW,
                34 => self.fg = Color::BLUE,
                35 => self.fg = Color::MAGENTA,
                36 => self.fg = Color::CYAN,
                37 => self.fg = Color::WHITE,
                38 => {
                    // Extended foreground color
                    if i + 2 < self.params.len() && self.params[i + 1] == 5 {
                        // 256 color
                        self.fg = color_256(self.params[i + 2]);
                        i += 2;
                    } else if i + 4 < self.params.len() && self.params[i + 1] == 2 {
                        // RGB color
                        self.fg = Color::rgb(
                            self.params[i + 2] as u8,
                            self.params[i + 3] as u8,
                            self.params[i + 4] as u8,
                        );
                        i += 4;
                    }
                }
                39 => self.fg = Color::WHITE,
                // Standard background colors
                40 => self.bg = Color::BLACK,
                41 => self.bg = Color::RED,
                42 => self.bg = Color::GREEN,
                43 => self.bg = Color::YELLOW,
                44 => self.bg = Color::BLUE,
                45 => self.bg = Color::MAGENTA,
                46 => self.bg = Color::CYAN,
                47 => self.bg = Color::WHITE,
                48 => {
                    // Extended background color
                    if i + 2 < self.params.len() && self.params[i + 1] == 5 {
                        // 256 color
                        self.bg = color_256(self.params[i + 2]);
                        i += 2;
                    } else if i + 4 < self.params.len() && self.params[i + 1] == 2 {
                        // RGB color
                        self.bg = Color::rgb(
                            self.params[i + 2] as u8,
                            self.params[i + 3] as u8,
                            self.params[i + 4] as u8,
                        );
                        i += 4;
                    }
                }
                49 => self.bg = Color::BLACK,
                // Bright foreground colors
                90 => self.fg = Color::rgb(128, 128, 128),
                91 => self.fg = Color::rgb(255, 85, 85),
                92 => self.fg = Color::rgb(85, 255, 85),
                93 => self.fg = Color::rgb(255, 255, 85),
                94 => self.fg = Color::rgb(85, 85, 255),
                95 => self.fg = Color::rgb(255, 85, 255),
                96 => self.fg = Color::rgb(85, 255, 255),
                97 => self.fg = Color::WHITE,
                // Bright background colors
                100 => self.bg = Color::rgb(128, 128, 128),
                101 => self.bg = Color::rgb(255, 85, 85),
                102 => self.bg = Color::rgb(85, 255, 85),
                103 => self.bg = Color::rgb(255, 255, 85),
                104 => self.bg = Color::rgb(85, 85, 255),
                105 => self.bg = Color::rgb(255, 85, 255),
                106 => self.bg = Color::rgb(85, 255, 255),
                107 => self.bg = Color::WHITE,
                _ => {}
            }
            i += 1;
        }
    }
}

/// Convert 256-color code to RGB
fn color_256(code: u16) -> Color {
    match code {
        // Standard colors (0-15)
        0 => Color::BLACK,
        1 => Color::RED,
        2 => Color::GREEN,
        3 => Color::YELLOW,
        4 => Color::BLUE,
        5 => Color::MAGENTA,
        6 => Color::CYAN,
        7 => Color::WHITE,
        8 => Color::rgb(128, 128, 128),
        9 => Color::rgb(255, 85, 85),
        10 => Color::rgb(85, 255, 85),
        11 => Color::rgb(255, 255, 85),
        12 => Color::rgb(85, 85, 255),
        13 => Color::rgb(255, 85, 255),
        14 => Color::rgb(85, 255, 255),
        15 => Color::rgb(255, 255, 255),
        // 216 colors (16-231)
        16..=231 => {
            let n = code - 16;
            let r = (n / 36) as u8;
            let g = ((n % 36) / 6) as u8;
            let b = (n % 6) as u8;
            let to_val = |v: u8| if v == 0 { 0 } else { 55 + 40 * v };
            Color::rgb(to_val(r), to_val(g), to_val(b))
        }
        // Grayscale (232-255)
        232..=255 => {
            let gray = ((code - 232) * 10 + 8) as u8;
            Color::rgb(gray, gray, gray)
        }
        _ => Color::WHITE,
    }
}

/// Terminal line containing cells
#[derive(Clone, Debug)]
pub struct TermLine {
    /// Cells in this line
    pub cells: Vec<TermCell>,
    /// Whether line is wrapped from previous
    pub wrapped: bool,
}

impl TermLine {
    /// Create empty line
    pub fn new() -> Self {
        Self {
            cells: Vec::new(),
            wrapped: false,
        }
    }

    /// Create line with capacity
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cells: Vec::with_capacity(capacity),
            wrapped: false,
        }
    }
}

impl Default for TermLine {
    fn default() -> Self {
        Self::new()
    }
}

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

/// Cursor display style
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CursorStyle {
    /// Block cursor █
    #[default]
    Block,
    /// Underline cursor _
    Underline,
    /// Vertical bar cursor |
    Bar,
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
        self.parser.fg = color;
        self
    }

    /// Set default background color
    pub fn default_bg(mut self, color: Color) -> Self {
        self.default_bg = color;
        self.parser.bg = color;
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
            while self.cursor_col < next_tab as usize && self.cursor_col < self.width as usize {
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
            self.lines.push(TermLine::with_capacity(self.width as usize));
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
            self.lines.push(TermLine::with_capacity(self.width as usize));
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
            self.lines.push(TermLine::with_capacity(self.width as usize));
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
                    self.input_buffer = self.history.get(self.history_pos)
                        .cloned()
                        .unwrap_or_default();
                }
                None
            }
            Key::Down => {
                if self.history_pos < self.history.len() {
                    self.history_pos += 1;
                    self.input_buffer = self.history.get(self.history_pos)
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
            Key::Escape => {
                Some(TerminalAction::Cancel)
            }
            Key::Tab => {
                Some(TerminalAction::TabComplete(self.input_buffer.clone()))
            }
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

/// Actions from terminal input
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalAction {
    /// User submitted command
    Submit(String),
    /// User cancelled
    Cancel,
    /// Tab completion requested
    TabComplete(String),
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
                ctx.buffer.set(
                    area.x + x,
                    area.y + y,
                    Cell::new(' ').bg(self.default_bg),
                );
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
                    ctx.buffer.set(area.x + col as u16, area.y + render_y, render_cell);
                }
            }

            render_y += 1;
        }

        // Render cursor if visible and focused
        if self.show_cursor && self.focused && self.scroll_offset == 0 {
            let cursor_screen_row = self.cursor_row.saturating_sub(
                self.lines.len().saturating_sub(self.height as usize)
            );

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

    #[test]
    fn test_terminal_new() {
        let term = Terminal::new(80, 24);
        assert_eq!(term.width, 80);
        assert_eq!(term.height, 24);
    }

    #[test]
    fn test_terminal_write() {
        let mut term = Terminal::new(80, 24);
        term.write("Hello, World!");

        assert_eq!(term.cursor_col, 13);
        assert_eq!(term.lines[0].cells.len(), 13);
    }

    #[test]
    fn test_terminal_writeln() {
        let mut term = Terminal::new(80, 24);
        term.writeln("Line 1");
        term.writeln("Line 2");

        assert_eq!(term.cursor_row, 2);
    }

    #[test]
    fn test_terminal_ansi_colors() {
        let mut term = Terminal::new(80, 24);
        term.write("\x1b[31mRed\x1b[0m Normal");

        // First 3 cells should be red
        assert_eq!(term.lines[0].cells[0].fg, Color::RED);
        assert_eq!(term.lines[0].cells[1].fg, Color::RED);
        assert_eq!(term.lines[0].cells[2].fg, Color::RED);
        // Rest should be white
        assert_eq!(term.lines[0].cells[4].fg, Color::WHITE);
    }

    #[test]
    fn test_terminal_scroll() {
        let mut term = Terminal::new(80, 5);
        for i in 0..10 {
            term.writeln(&format!("Line {}", i));
        }

        term.scroll_up(3);
        assert_eq!(term.scroll_offset, 3);

        term.scroll_down(2);
        assert_eq!(term.scroll_offset, 1);

        term.scroll_to_bottom();
        assert_eq!(term.scroll_offset, 0);
    }

    #[test]
    fn test_terminal_clear() {
        let mut term = Terminal::new(80, 24);
        term.writeln("Some text");
        term.clear();

        assert_eq!(term.cursor_row, 0);
        assert_eq!(term.cursor_col, 0);
    }

    #[test]
    fn test_terminal_input() {
        let mut term = Terminal::new(80, 24);

        term.handle_key(KeyEvent::new(Key::Char('h')));
        term.handle_key(KeyEvent::new(Key::Char('i')));

        assert_eq!(term.get_input(), "hi");

        let action = term.handle_key(KeyEvent::new(Key::Enter));
        assert_eq!(action, Some(TerminalAction::Submit("hi".to_string())));
        assert_eq!(term.get_input(), "");
    }

    #[test]
    fn test_terminal_history() {
        let mut term = Terminal::new(80, 24);

        term.handle_key(KeyEvent::new(Key::Char('a')));
        term.handle_key(KeyEvent::new(Key::Enter));

        term.handle_key(KeyEvent::new(Key::Char('b')));
        term.handle_key(KeyEvent::new(Key::Enter));

        term.handle_key(KeyEvent::new(Key::Up));
        assert_eq!(term.get_input(), "b");

        term.handle_key(KeyEvent::new(Key::Up));
        assert_eq!(term.get_input(), "a");
    }

    #[test]
    fn test_terminal_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut term = Terminal::new(80, 24);
        term.write("Test");
        term.render(&mut ctx);
    }

    #[test]
    fn test_terminal_256_color() {
        let mut term = Terminal::new(80, 24);
        term.write("\x1b[38;5;196mRed256\x1b[0m");

        // Color 196 is bright red (should be an RGB color)
        let cell = &term.lines[0].cells[0];
        // Check it's not white (default) - 256 color mode sets a custom color
        assert_ne!(cell.fg, Color::WHITE);
    }

    #[test]
    fn test_terminal_rgb_color() {
        let mut term = Terminal::new(80, 24);
        term.write("\x1b[38;2;255;128;64mOrange\x1b[0m");

        let cell = &term.lines[0].cells[0];
        assert_eq!(cell.fg, Color::rgb(255, 128, 64));
    }

    #[test]
    fn test_terminal_presets() {
        let shell = Terminal::shell(80, 24);
        assert!(matches!(shell.cursor_style, CursorStyle::Block));

        let log = Terminal::log_viewer(80, 24);
        assert!(!log.show_cursor);
    }

    #[test]
    fn test_terminal_helper() {
        let term = terminal(120, 40);
        assert_eq!(term.width, 120);
        assert_eq!(term.height, 40);
    }
}
