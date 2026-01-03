//! Text Editor Example - Demonstrates TextArea widget with full features
//!
//! A full-featured text editor showing TextArea, StatusBar, and vim-like keybindings.
//!
//! Run with: cargo run --example text_editor

use revue::prelude::*;

/// Editor mode
#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Normal,
    Insert,
    Visual,
    Command,
}

impl Mode {
    fn name(&self) -> &str {
        match self {
            Mode::Normal => "NORMAL",
            Mode::Insert => "INSERT",
            Mode::Visual => "VISUAL",
            Mode::Command => "COMMAND",
        }
    }

    fn color(&self) -> Color {
        match self {
            Mode::Normal => Color::BLUE,
            Mode::Insert => Color::GREEN,
            Mode::Visual => Color::MAGENTA,
            Mode::Command => Color::YELLOW,
        }
    }
}

/// Text editor application state
struct TextEditor {
    /// Current mode
    mode: Mode,
    /// File name
    filename: String,
    /// Editor lines
    lines: Vec<String>,
    /// Cursor row (0-indexed)
    cursor_row: usize,
    /// Cursor column (0-indexed)
    cursor_col: usize,
    /// Scroll offset
    scroll_offset: usize,
    /// Visual selection start (row, col)
    visual_start: Option<(usize, usize)>,
    /// Command input
    command_input: String,
    /// Status message
    status_message: String,
    /// Modified flag
    modified: bool,
    /// Clipboard content
    clipboard: String,
    /// Undo stack
    undo_stack: Vec<Vec<String>>,
    /// Redo stack
    redo_stack: Vec<Vec<String>>,
    /// Search query
    search_query: String,
    /// Search matches (row, col)
    search_matches: Vec<(usize, usize)>,
    /// Current search match index
    search_index: usize,
    /// Show line numbers
    show_line_numbers: bool,
    /// Word wrap enabled
    word_wrap: bool,
}

impl TextEditor {
    fn new() -> Self {
        let sample_text = r#"//! Welcome to the Revue Text Editor
//!
//! This is a demonstration of the TextArea widget with vim-like keybindings.

use revue::prelude::*;

fn main() -> Result<()> {
    // Create a simple application
    let mut app = App::builder()
        .style("styles.css")
        .hot_reload(true)
        .build();

    // Create a view with styling
    let view = vstack()
        .child(Text::new("Hello, World!").fg(Color::CYAN))
        .child(Text::new("Press 'q' to quit").fg(Color::GRAY));

    // Run the application
    app.run(&view)
}

// Key bindings:
// - Normal mode: h/j/k/l to move, i to insert, v for visual
// - Insert mode: type text, Escape to return to normal
// - Visual mode: select text, y to yank, d to delete
// - Command mode: :w to save, :q to quit, :wq to save and quit

// TODO: Add syntax highlighting
// FIXME: Handle unicode properly
// NOTE: This is a demo application

fn helper_function() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}

struct Config {
    theme: String,
    tab_size: usize,
    show_whitespace: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "dark".into(),
            tab_size: 4,
            show_whitespace: false,
        }
    }
}
"#;

        let lines: Vec<String> = sample_text.lines().map(String::from).collect();

        Self {
            mode: Mode::Normal,
            filename: "untitled.rs".into(),
            lines,
            cursor_row: 0,
            cursor_col: 0,
            scroll_offset: 0,
            visual_start: None,
            command_input: String::new(),
            status_message: "Press 'i' to insert, ':' for commands, 'q' to quit".into(),
            modified: false,
            clipboard: String::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            search_query: String::new(),
            search_matches: Vec::new(),
            search_index: 0,
            show_line_numbers: true,
            word_wrap: false,
        }
    }

    fn current_line(&self) -> &str {
        self.lines
            .get(self.cursor_row)
            .map(|s| s.as_str())
            .unwrap_or("")
    }

    fn current_line_len(&self) -> usize {
        self.current_line().len()
    }

    fn save_undo(&mut self) {
        self.undo_stack.push(self.lines.clone());
        self.redo_stack.clear();
        if self.undo_stack.len() > 100 {
            self.undo_stack.remove(0);
        }
    }

    fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.lines.clone());
            self.lines = prev;
            self.status_message = "Undo".into();
            self.clamp_cursor();
        } else {
            self.status_message = "Nothing to undo".into();
        }
    }

    fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.lines.clone());
            self.lines = next;
            self.status_message = "Redo".into();
            self.clamp_cursor();
        } else {
            self.status_message = "Nothing to redo".into();
        }
    }

    fn clamp_cursor(&mut self) {
        self.cursor_row = self.cursor_row.min(self.lines.len().saturating_sub(1));
        self.cursor_col = self.cursor_col.min(self.current_line_len());
    }

    fn ensure_visible(&mut self, visible_lines: usize) {
        if self.cursor_row < self.scroll_offset {
            self.scroll_offset = self.cursor_row;
        } else if self.cursor_row >= self.scroll_offset + visible_lines {
            self.scroll_offset = self.cursor_row - visible_lines + 1;
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match self.mode {
            Mode::Normal => self.handle_normal_mode(key),
            Mode::Insert => self.handle_insert_mode(key),
            Mode::Visual => self.handle_visual_mode(key),
            Mode::Command => self.handle_command_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: &Key) -> bool {
        match key {
            // Movement
            Key::Char('h') | Key::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
                true
            }
            Key::Char('j') | Key::Down => {
                if self.cursor_row < self.lines.len().saturating_sub(1) {
                    self.cursor_row += 1;
                    self.clamp_cursor();
                }
                true
            }
            Key::Char('k') | Key::Up => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.clamp_cursor();
                }
                true
            }
            Key::Char('l') | Key::Right => {
                if self.cursor_col < self.current_line_len() {
                    self.cursor_col += 1;
                }
                true
            }
            Key::Char('0') | Key::Home => {
                self.cursor_col = 0;
                true
            }
            Key::Char('$') | Key::End => {
                self.cursor_col = self.current_line_len();
                true
            }
            Key::Char('g') => {
                self.cursor_row = 0;
                self.cursor_col = 0;
                self.status_message = "Top of file".into();
                true
            }
            Key::Char('G') => {
                self.cursor_row = self.lines.len().saturating_sub(1);
                self.cursor_col = 0;
                self.status_message = "Bottom of file".into();
                true
            }
            Key::Char('w') => {
                // Word forward
                let line = self.current_line();
                let mut col = self.cursor_col;
                // Skip current word
                while col < line.len() && !line.chars().nth(col).is_none_or(|c| c.is_whitespace())
                {
                    col += 1;
                }
                // Skip whitespace
                while col < line.len() && line.chars().nth(col).is_some_and(|c| c.is_whitespace())
                {
                    col += 1;
                }
                if col >= line.len() && self.cursor_row < self.lines.len() - 1 {
                    self.cursor_row += 1;
                    self.cursor_col = 0;
                } else {
                    self.cursor_col = col;
                }
                true
            }
            Key::Char('b') => {
                // Word backward
                if self.cursor_col == 0 && self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.cursor_col = self.current_line_len();
                } else {
                    let line = self.current_line();
                    let mut col = self.cursor_col.saturating_sub(1);
                    // Skip whitespace
                    while col > 0 && line.chars().nth(col).is_some_and(|c| c.is_whitespace()) {
                        col -= 1;
                    }
                    // Skip word
                    while col > 0
                        && !line
                            .chars()
                            .nth(col - 1)
                            .is_none_or(|c| c.is_whitespace())
                    {
                        col -= 1;
                    }
                    self.cursor_col = col;
                }
                true
            }

            // Mode switches
            Key::Char('i') => {
                self.mode = Mode::Insert;
                self.status_message = "-- INSERT --".into();
                true
            }
            Key::Char('a') => {
                self.mode = Mode::Insert;
                if self.cursor_col < self.current_line_len() {
                    self.cursor_col += 1;
                }
                self.status_message = "-- INSERT --".into();
                true
            }
            Key::Char('A') => {
                self.mode = Mode::Insert;
                self.cursor_col = self.current_line_len();
                self.status_message = "-- INSERT --".into();
                true
            }
            Key::Char('o') => {
                self.save_undo();
                self.cursor_row += 1;
                self.lines.insert(self.cursor_row, String::new());
                self.cursor_col = 0;
                self.mode = Mode::Insert;
                self.modified = true;
                self.status_message = "-- INSERT --".into();
                true
            }
            Key::Char('O') => {
                self.save_undo();
                self.lines.insert(self.cursor_row, String::new());
                self.cursor_col = 0;
                self.mode = Mode::Insert;
                self.modified = true;
                self.status_message = "-- INSERT --".into();
                true
            }
            Key::Char('v') => {
                self.mode = Mode::Visual;
                self.visual_start = Some((self.cursor_row, self.cursor_col));
                self.status_message = "-- VISUAL --".into();
                true
            }
            Key::Char(':') => {
                self.mode = Mode::Command;
                self.command_input.clear();
                self.status_message = ":".into();
                true
            }
            Key::Char('/') => {
                self.mode = Mode::Command;
                self.command_input.clear();
                self.status_message = "/".into();
                true
            }

            // Editing
            Key::Char('x') => {
                self.save_undo();
                if !self.current_line().is_empty() && self.cursor_col < self.current_line_len() {
                    let line = &mut self.lines[self.cursor_row];
                    self.clipboard = line
                        .chars()
                        .nth(self.cursor_col)
                        .map(|c| c.to_string())
                        .unwrap_or_default();
                    line.remove(self.cursor_col);
                    self.modified = true;
                    self.clamp_cursor();
                }
                true
            }
            Key::Char('d') => {
                // Delete line (dd would need multi-key handling)
                self.save_undo();
                if !self.lines.is_empty() {
                    self.clipboard = self.lines.remove(self.cursor_row);
                    if self.lines.is_empty() {
                        self.lines.push(String::new());
                    }
                    self.modified = true;
                    self.clamp_cursor();
                    self.status_message = "Line deleted".into();
                }
                true
            }
            Key::Char('y') => {
                // Yank line
                self.clipboard = self.current_line().to_string();
                self.status_message = "Line yanked".into();
                true
            }
            Key::Char('p') => {
                // Paste after
                if !self.clipboard.is_empty() {
                    self.save_undo();
                    if self.clipboard.contains('\n')
                        || !self.lines[self.cursor_row].contains(&self.clipboard)
                    {
                        self.cursor_row += 1;
                        self.lines.insert(self.cursor_row, self.clipboard.clone());
                    } else {
                        let col = self.cursor_col + 1;
                        self.lines[self.cursor_row].insert_str(col, &self.clipboard);
                        self.cursor_col = col + self.clipboard.len() - 1;
                    }
                    self.modified = true;
                    self.status_message = "Pasted".into();
                }
                true
            }
            Key::Char('P') => {
                // Paste before
                if !self.clipboard.is_empty() {
                    self.save_undo();
                    self.lines.insert(self.cursor_row, self.clipboard.clone());
                    self.modified = true;
                    self.status_message = "Pasted".into();
                }
                true
            }
            Key::Char('u') => {
                self.undo();
                true
            }
            Key::Char('r') if key == &Key::Char('r') => {
                // Ctrl+R for redo (simplified)
                self.redo();
                true
            }
            Key::Char('n') => {
                // Next search match
                if !self.search_matches.is_empty() {
                    self.search_index = (self.search_index + 1) % self.search_matches.len();
                    let (row, col) = self.search_matches[self.search_index];
                    self.cursor_row = row;
                    self.cursor_col = col;
                    self.status_message = format!(
                        "Match {}/{}",
                        self.search_index + 1,
                        self.search_matches.len()
                    );
                }
                true
            }
            Key::Char('N') => {
                // Previous search match
                if !self.search_matches.is_empty() {
                    self.search_index = if self.search_index == 0 {
                        self.search_matches.len() - 1
                    } else {
                        self.search_index - 1
                    };
                    let (row, col) = self.search_matches[self.search_index];
                    self.cursor_row = row;
                    self.cursor_col = col;
                    self.status_message = format!(
                        "Match {}/{}",
                        self.search_index + 1,
                        self.search_matches.len()
                    );
                }
                true
            }

            _ => false,
        }
    }

    fn handle_insert_mode(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.mode = Mode::Normal;
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
                self.status_message = "".into();
                true
            }
            Key::Char(c) => {
                self.save_undo();
                self.lines[self.cursor_row].insert(self.cursor_col, *c);
                self.cursor_col += 1;
                self.modified = true;
                true
            }
            Key::Enter => {
                self.save_undo();
                let rest = self.lines[self.cursor_row].split_off(self.cursor_col);
                self.cursor_row += 1;
                self.lines.insert(self.cursor_row, rest);
                self.cursor_col = 0;
                self.modified = true;
                true
            }
            Key::Backspace => {
                self.save_undo();
                if self.cursor_col > 0 {
                    self.lines[self.cursor_row].remove(self.cursor_col - 1);
                    self.cursor_col -= 1;
                    self.modified = true;
                } else if self.cursor_row > 0 {
                    let current = self.lines.remove(self.cursor_row);
                    self.cursor_row -= 1;
                    self.cursor_col = self.lines[self.cursor_row].len();
                    self.lines[self.cursor_row].push_str(&current);
                    self.modified = true;
                }
                true
            }
            Key::Delete => {
                self.save_undo();
                if self.cursor_col < self.current_line_len() {
                    self.lines[self.cursor_row].remove(self.cursor_col);
                    self.modified = true;
                } else if self.cursor_row < self.lines.len() - 1 {
                    let next = self.lines.remove(self.cursor_row + 1);
                    self.lines[self.cursor_row].push_str(&next);
                    self.modified = true;
                }
                true
            }
            Key::Tab => {
                self.save_undo();
                self.lines[self.cursor_row].insert_str(self.cursor_col, "    ");
                self.cursor_col += 4;
                self.modified = true;
                true
            }
            Key::Up => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.clamp_cursor();
                }
                true
            }
            Key::Down => {
                if self.cursor_row < self.lines.len() - 1 {
                    self.cursor_row += 1;
                    self.clamp_cursor();
                }
                true
            }
            Key::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
                true
            }
            Key::Right => {
                if self.cursor_col < self.current_line_len() {
                    self.cursor_col += 1;
                }
                true
            }
            Key::Home => {
                self.cursor_col = 0;
                true
            }
            Key::End => {
                self.cursor_col = self.current_line_len();
                true
            }
            _ => false,
        }
    }

    fn handle_visual_mode(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.mode = Mode::Normal;
                self.visual_start = None;
                self.status_message = "".into();
                true
            }
            Key::Char('h') | Key::Left => {
                if self.cursor_col > 0 {
                    self.cursor_col -= 1;
                }
                true
            }
            Key::Char('j') | Key::Down => {
                if self.cursor_row < self.lines.len() - 1 {
                    self.cursor_row += 1;
                    self.clamp_cursor();
                }
                true
            }
            Key::Char('k') | Key::Up => {
                if self.cursor_row > 0 {
                    self.cursor_row -= 1;
                    self.clamp_cursor();
                }
                true
            }
            Key::Char('l') | Key::Right => {
                if self.cursor_col < self.current_line_len() {
                    self.cursor_col += 1;
                }
                true
            }
            Key::Char('y') => {
                // Yank selection
                if let Some((start_row, start_col)) = self.visual_start {
                    let (end_row, end_col) = (self.cursor_row, self.cursor_col);
                    let (sr, sc, er, ec) = if (start_row, start_col) <= (end_row, end_col) {
                        (start_row, start_col, end_row, end_col)
                    } else {
                        (end_row, end_col, start_row, start_col)
                    };

                    if sr == er {
                        self.clipboard = self.lines[sr]
                            [sc..=ec.min(self.lines[sr].len().saturating_sub(1))]
                            .to_string();
                    } else {
                        let mut selected = String::new();
                        for row in sr..=er {
                            if row == sr {
                                selected.push_str(&self.lines[row][sc..]);
                            } else if row == er {
                                selected.push_str(
                                    &self.lines[row]
                                        [..=ec.min(self.lines[row].len().saturating_sub(1))],
                                );
                            } else {
                                selected.push_str(&self.lines[row]);
                            }
                            if row < er {
                                selected.push('\n');
                            }
                        }
                        self.clipboard = selected;
                    }
                    self.status_message = "Yanked".into();
                }
                self.mode = Mode::Normal;
                self.visual_start = None;
                true
            }
            Key::Char('d') => {
                // Delete selection
                self.save_undo();
                // Simplified: just delete lines
                if let Some((start_row, _)) = self.visual_start {
                    let (sr, er) = if start_row <= self.cursor_row {
                        (start_row, self.cursor_row)
                    } else {
                        (self.cursor_row, start_row)
                    };

                    let deleted: Vec<String> = self.lines.drain(sr..=er).collect();
                    self.clipboard = deleted.join("\n");

                    if self.lines.is_empty() {
                        self.lines.push(String::new());
                    }
                    self.cursor_row = sr.min(self.lines.len() - 1);
                    self.cursor_col = 0;
                    self.modified = true;
                    self.status_message = "Deleted".into();
                }
                self.mode = Mode::Normal;
                self.visual_start = None;
                true
            }
            _ => false,
        }
    }

    fn handle_command_mode(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.mode = Mode::Normal;
                self.command_input.clear();
                self.status_message = "".into();
                true
            }
            Key::Enter => {
                let cmd = self.command_input.clone();
                self.mode = Mode::Normal;
                self.command_input.clear();

                if self.status_message.starts_with('/') {
                    // Search
                    self.search_query = cmd.clone();
                    self.search_matches.clear();
                    for (row, line) in self.lines.iter().enumerate() {
                        let mut start = 0;
                        while let Some(pos) = line[start..].find(&cmd) {
                            self.search_matches.push((row, start + pos));
                            start += pos + 1;
                        }
                    }
                    if self.search_matches.is_empty() {
                        self.status_message = format!("Pattern not found: {}", cmd);
                    } else {
                        self.search_index = 0;
                        let (row, col) = self.search_matches[0];
                        self.cursor_row = row;
                        self.cursor_col = col;
                        self.status_message = format!("{} matches", self.search_matches.len());
                    }
                } else {
                    // Execute command
                    self.execute_command(&cmd);
                }
                true
            }
            Key::Char(c) => {
                self.command_input.push(*c);
                self.status_message = if self.status_message.starts_with('/') {
                    format!("/{}", self.command_input)
                } else {
                    format!(":{}", self.command_input)
                };
                true
            }
            Key::Backspace => {
                self.command_input.pop();
                self.status_message = if self.status_message.starts_with('/') {
                    format!("/{}", self.command_input)
                } else {
                    format!(":{}", self.command_input)
                };
                true
            }
            _ => false,
        }
    }

    fn execute_command(&mut self, cmd: &str) {
        match cmd.trim() {
            "w" => {
                self.status_message =
                    format!("\"{}\" written ({} lines)", self.filename, self.lines.len());
                self.modified = false;
            }
            "q" => {
                if self.modified {
                    self.status_message = "No write since last change (use :q! to override)".into();
                } else {
                    self.status_message = "Use Ctrl+C to quit".into();
                }
            }
            "q!" => {
                self.status_message = "Use Ctrl+C to quit".into();
            }
            "wq" | "x" => {
                self.status_message = format!("\"{}\" written, quitting...", self.filename);
                self.modified = false;
            }
            "set number" | "set nu" => {
                self.show_line_numbers = true;
                self.status_message = "Line numbers enabled".into();
            }
            "set nonumber" | "set nonu" => {
                self.show_line_numbers = false;
                self.status_message = "Line numbers disabled".into();
            }
            "set wrap" => {
                self.word_wrap = true;
                self.status_message = "Word wrap enabled".into();
            }
            "set nowrap" => {
                self.word_wrap = false;
                self.status_message = "Word wrap disabled".into();
            }
            _ if cmd.starts_with("e ") => {
                let file = cmd[2..].trim();
                self.filename = file.to_string();
                self.status_message = format!("\"{}\" opened", file);
            }
            _ => {
                self.status_message = format!("Unknown command: {}", cmd);
            }
        }
    }

    fn render_line_number(&self, line_num: usize) -> impl View {
        if self.show_line_numbers {
            let width = self.lines.len().to_string().len();
            Text::new(format!("{:>width$} ", line_num, width = width)).fg(Color::rgb(100, 100, 100))
        } else {
            Text::new("")
        }
    }

    #[allow(dead_code)]
    fn is_in_visual_selection(&self, row: usize, col: usize) -> bool {
        if self.mode != Mode::Visual {
            return false;
        }
        let Some((start_row, start_col)) = self.visual_start else {
            return false;
        };
        let (end_row, end_col) = (self.cursor_row, self.cursor_col);

        let (sr, sc, er, ec) = if (start_row, start_col) <= (end_row, end_col) {
            (start_row, start_col, end_row, end_col)
        } else {
            (end_row, end_col, start_row, start_col)
        };

        if row < sr || row > er {
            return false;
        }
        if row == sr && row == er {
            col >= sc && col <= ec
        } else if row == sr {
            col >= sc
        } else if row == er {
            col <= ec
        } else {
            true
        }
    }
}

impl View for TextEditor {
    fn render(&self, ctx: &mut RenderContext) {
        let visible_lines = 20; // Approximate visible lines

        // Header
        let modified_indicator = if self.modified { " [+]" } else { "" };
        let header = hstack()
            .child(
                Text::new(format!(" {} ", self.filename))
                    .fg(Color::WHITE)
                    .bg(Color::rgb(50, 50, 50)),
            )
            .child(Text::new(modified_indicator).fg(Color::YELLOW))
            .child(
                Text::new(format!("  {} lines  ", self.lines.len())).fg(Color::rgb(100, 100, 100)),
            );

        // Editor content
        let mut content = vstack();
        let start = self.scroll_offset;
        let end = (start + visible_lines).min(self.lines.len());

        for row in start..end {
            let line = &self.lines[row];
            let line_num = self.render_line_number(row + 1);

            // Build line with cursor and selection highlighting
            let mut line_content = String::new();
            let is_cursor_row = row == self.cursor_row;

            for ch in line.chars() {
                line_content.push(ch);
            }

            // Show cursor position
            let text = if is_cursor_row && self.mode == Mode::Normal {
                // Highlight cursor position
                let before = &line[..self.cursor_col.min(line.len())];
                let cursor_char = line.chars().nth(self.cursor_col).unwrap_or(' ');
                let after = if self.cursor_col < line.len() {
                    &line[self.cursor_col + 1..]
                } else {
                    ""
                };

                hstack()
                    .child(line_num)
                    .child(Text::new(before))
                    .child(
                        Text::new(cursor_char.to_string())
                            .fg(Color::BLACK)
                            .bg(Color::WHITE),
                    )
                    .child(Text::new(after))
            } else if is_cursor_row && self.mode == Mode::Insert {
                // Insert mode cursor (line)
                let before = &line[..self.cursor_col.min(line.len())];
                let after = &line[self.cursor_col.min(line.len())..];

                hstack()
                    .child(line_num)
                    .child(Text::new(before))
                    .child(Text::new("|").fg(Color::CYAN))
                    .child(Text::new(after))
            } else {
                hstack().child(line_num).child(Text::new(line))
            };

            content = content.child(text);
        }

        // Status line
        let mode_indicator = Text::new(format!(" {} ", self.mode.name()))
            .fg(Color::BLACK)
            .bg(self.mode.color())
            .bold();

        let file_info = Text::new(format!(" {} ", self.filename));

        let position = Text::new(format!(" {}:{} ", self.cursor_row + 1, self.cursor_col + 1));

        let percent = if self.lines.is_empty() {
            "Top".to_string()
        } else {
            let pct = (self.cursor_row * 100) / self.lines.len().max(1);
            if pct == 0 {
                "Top".to_string()
            } else if pct >= 99 {
                "Bot".to_string()
            } else {
                format!("{}%", pct)
            }
        };

        let status_line = hstack()
            .child(mode_indicator)
            .child(file_info)
            .child(Text::new(&self.status_message).fg(Color::rgb(180, 180, 180)))
            .child(Text::new(format!(" {} ", percent)).fg(Color::rgb(100, 100, 100)))
            .child(position);

        // Help line
        let help = if self.mode == Mode::Normal {
            "i: Insert | v: Visual | :: Command | /: Search | hjkl: Move | u: Undo | q: Quit"
        } else if self.mode == Mode::Insert {
            "Esc: Normal mode | Arrow keys: Move | Enter: New line | Backspace: Delete"
        } else if self.mode == Mode::Visual {
            "Esc: Cancel | hjkl: Select | y: Yank | d: Delete"
        } else {
            "Enter: Execute | Esc: Cancel"
        };

        let help_line = Text::new(help).fg(Color::rgb(80, 80, 80));

        // Main layout
        vstack()
            .child(header)
            .child(Border::single().child(content))
            .child(status_line)
            .child(help_line)
            .render(ctx);
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let editor = TextEditor::new();

    app.run_with_handler(editor, |key_event, editor| {
        editor.ensure_visible(18);
        editor.handle_key(&key_event.key)
    })
}
