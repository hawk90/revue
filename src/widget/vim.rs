//! Vim Mode system for terminal applications
//!
//! Provides vim-style modal editing with Normal, Insert, Visual,
//! and Command modes.

use crate::event::{Key, KeyEvent};
use crate::style::Color;
use std::collections::HashMap;

/// Vim mode
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum VimMode {
    /// Normal mode (navigation, commands)
    #[default]
    Normal,
    /// Insert mode (text input)
    Insert,
    /// Visual mode (selection)
    Visual,
    /// Visual Line mode
    VisualLine,
    /// Visual Block mode
    VisualBlock,
    /// Command mode (:commands)
    Command,
    /// Search mode (/search)
    Search,
    /// Replace mode (r, R)
    Replace,
}

impl VimMode {
    /// Get mode name for display
    pub fn name(&self) -> &'static str {
        match self {
            VimMode::Normal => "NORMAL",
            VimMode::Insert => "INSERT",
            VimMode::Visual => "VISUAL",
            VimMode::VisualLine => "V-LINE",
            VimMode::VisualBlock => "V-BLOCK",
            VimMode::Command => "COMMAND",
            VimMode::Search => "SEARCH",
            VimMode::Replace => "REPLACE",
        }
    }

    /// Get mode color
    pub fn color(&self) -> Color {
        match self {
            VimMode::Normal => Color::rgb(100, 150, 255),
            VimMode::Insert => Color::rgb(100, 255, 100),
            VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => {
                Color::rgb(255, 150, 100)
            }
            VimMode::Command => Color::rgb(255, 255, 100),
            VimMode::Search => Color::rgb(255, 100, 255),
            VimMode::Replace => Color::rgb(255, 100, 100),
        }
    }
}

/// Vim motion
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VimMotion {
    /// Character left (h)
    Left,
    /// Character right (l)
    Right,
    /// Line up (k)
    Up,
    /// Line down (j)
    Down,
    /// Word forward (w)
    Word,
    /// Word backward (b)
    WordBack,
    /// End of word (e)
    WordEnd,
    /// Start of line (0)
    LineStart,
    /// End of line ($)
    LineEnd,
    /// First non-blank (^)
    FirstNonBlank,
    /// Go to line (G, gg)
    GoToLine(Option<usize>),
    /// Find character (f)
    FindChar(char),
    /// Find character backward (F)
    FindCharBack(char),
    /// Till character (t)
    TillChar(char),
    /// Till character backward (T)
    TillCharBack(char),
    /// Paragraph forward (})
    ParagraphForward,
    /// Paragraph backward ({)
    ParagraphBack,
    /// Match bracket (%)
    MatchBracket,
    /// Search forward (n)
    SearchNext,
    /// Search backward (N)
    SearchPrev,
}

/// Vim action
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VimAction {
    /// Move cursor
    Move(VimMotion),
    /// Delete with motion
    Delete(Option<VimMotion>),
    /// Yank (copy) with motion
    Yank(Option<VimMotion>),
    /// Change with motion
    Change(Option<VimMotion>),
    /// Paste after
    PasteAfter,
    /// Paste before
    PasteBefore,
    /// Undo
    Undo,
    /// Redo
    Redo,
    /// Enter insert mode
    Insert,
    /// Insert at start of line
    InsertStart,
    /// Append after cursor
    Append,
    /// Append at end of line
    AppendEnd,
    /// Open line below
    OpenBelow,
    /// Open line above
    OpenAbove,
    /// Replace character
    ReplaceChar(char),
    /// Enter visual mode
    EnterVisual,
    /// Enter visual line mode
    EnterVisualLine,
    /// Enter visual block mode
    EnterVisualBlock,
    /// Enter command mode
    EnterCommand,
    /// Enter search mode
    EnterSearch,
    /// Escape to normal mode
    Escape,
    /// Repeat last action (.)
    Repeat,
    /// Join lines (J)
    JoinLines,
    /// Indent
    Indent,
    /// Outdent
    Outdent,
    /// Execute command
    ExecuteCommand(String),
    /// Nothing
    None,
}

/// Vim state manager
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let mut vim = VimState::new();
///
/// // Process key event
/// let action = vim.handle_key(&KeyEvent::new(Key::Char('j')));
/// match action {
///     VimAction::Move(VimMotion::Down) => { /* move cursor down */ }
///     _ => {}
/// }
/// ```
pub struct VimState {
    /// Current mode
    mode: VimMode,
    /// Pending count (for repeat)
    count: Option<usize>,
    /// Pending operator
    operator: Option<char>,
    /// Command buffer (for :commands)
    command_buffer: String,
    /// Search pattern
    search_pattern: String,
    /// Search direction (true = forward)
    search_forward: bool,
    /// Last action for repeat
    last_action: Option<VimAction>,
    /// Register (for yank/paste)
    register: String,
    /// Register name (for future named register support)
    _register_name: char,
    /// Key sequence buffer
    key_buffer: Vec<char>,
    /// Custom key mappings
    mappings: HashMap<String, VimAction>,
}

impl VimState {
    /// Create a new vim state
    pub fn new() -> Self {
        Self {
            mode: VimMode::Normal,
            count: None,
            operator: None,
            command_buffer: String::new(),
            search_pattern: String::new(),
            search_forward: true,
            last_action: None,
            register: String::new(),
            _register_name: '"',
            key_buffer: Vec::new(),
            mappings: HashMap::new(),
        }
    }

    /// Get current mode
    pub fn mode(&self) -> VimMode {
        self.mode
    }

    /// Set mode
    pub fn set_mode(&mut self, mode: VimMode) {
        self.mode = mode;
        if mode == VimMode::Normal {
            self.operator = None;
            self.count = None;
        }
    }

    /// Get count (default 1)
    pub fn count(&self) -> usize {
        self.count.unwrap_or(1)
    }

    /// Get command buffer
    pub fn command_buffer(&self) -> &str {
        &self.command_buffer
    }

    /// Get search pattern
    pub fn search_pattern(&self) -> &str {
        &self.search_pattern
    }

    /// Get register content
    pub fn register(&self) -> &str {
        &self.register
    }

    /// Set register content
    pub fn set_register(&mut self, content: impl Into<String>) {
        self.register = content.into();
    }

    /// Add a custom key mapping
    pub fn map(&mut self, keys: &str, action: VimAction) {
        self.mappings.insert(keys.to_string(), action);
    }

    /// Handle key event in normal mode
    fn handle_normal(&mut self, key: &KeyEvent) -> VimAction {
        // Handle digits for count
        if let Key::Char(ch) = key.key {
            if ch.is_ascii_digit() {
                let digit = ch.to_digit(10).unwrap() as usize;
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                return VimAction::None;
            }
        }

        // Handle operator pending
        if let Some(op) = self.operator {
            if let Key::Char(ch) = key.key {
                let motion = self.char_to_motion(ch);
                if motion.is_some() {
                    self.operator = None;
                    return match op {
                        'd' => VimAction::Delete(motion),
                        'y' => VimAction::Yank(motion),
                        'c' => VimAction::Change(motion),
                        _ => VimAction::None,
                    };
                }
            }
        }

        match key.key {
            // Mode changes
            Key::Char('i') => {
                self.set_mode(VimMode::Insert);
                VimAction::Insert
            }
            Key::Char('I') => {
                self.set_mode(VimMode::Insert);
                VimAction::InsertStart
            }
            Key::Char('a') => {
                self.set_mode(VimMode::Insert);
                VimAction::Append
            }
            Key::Char('A') => {
                self.set_mode(VimMode::Insert);
                VimAction::AppendEnd
            }
            Key::Char('o') => {
                self.set_mode(VimMode::Insert);
                VimAction::OpenBelow
            }
            Key::Char('O') => {
                self.set_mode(VimMode::Insert);
                VimAction::OpenAbove
            }
            Key::Char('v') => {
                self.set_mode(VimMode::Visual);
                VimAction::EnterVisual
            }
            Key::Char('V') => {
                self.set_mode(VimMode::VisualLine);
                VimAction::EnterVisualLine
            }
            Key::Char(':') => {
                self.set_mode(VimMode::Command);
                self.command_buffer.clear();
                VimAction::EnterCommand
            }
            Key::Char('/') => {
                self.set_mode(VimMode::Search);
                self.search_pattern.clear();
                self.search_forward = true;
                VimAction::EnterSearch
            }
            Key::Char('?') => {
                self.set_mode(VimMode::Search);
                self.search_pattern.clear();
                self.search_forward = false;
                VimAction::EnterSearch
            }

            // Motions
            Key::Char('h') | Key::Left => VimAction::Move(VimMotion::Left),
            Key::Char('j') | Key::Down => VimAction::Move(VimMotion::Down),
            Key::Char('k') | Key::Up => VimAction::Move(VimMotion::Up),
            Key::Char('l') | Key::Right => VimAction::Move(VimMotion::Right),
            Key::Char('w') => VimAction::Move(VimMotion::Word),
            Key::Char('b') => VimAction::Move(VimMotion::WordBack),
            Key::Char('e') => VimAction::Move(VimMotion::WordEnd),
            Key::Char('0') => VimAction::Move(VimMotion::LineStart),
            Key::Char('$') => VimAction::Move(VimMotion::LineEnd),
            Key::Char('^') => VimAction::Move(VimMotion::FirstNonBlank),
            Key::Char('G') => VimAction::Move(VimMotion::GoToLine(self.count)),
            Key::Char('g') => {
                self.key_buffer.push('g');
                VimAction::None
            }
            Key::Char('{') => VimAction::Move(VimMotion::ParagraphBack),
            Key::Char('}') => VimAction::Move(VimMotion::ParagraphForward),
            Key::Char('%') => VimAction::Move(VimMotion::MatchBracket),
            Key::Char('n') => VimAction::Move(VimMotion::SearchNext),
            Key::Char('N') => VimAction::Move(VimMotion::SearchPrev),

            // Operators
            Key::Char('d') => {
                self.operator = Some('d');
                VimAction::None
            }
            Key::Char('y') => {
                self.operator = Some('y');
                VimAction::None
            }
            Key::Char('c') => {
                self.operator = Some('c');
                VimAction::None
            }

            // Actions
            Key::Char('x') => VimAction::Delete(Some(VimMotion::Right)),
            Key::Char('X') => VimAction::Delete(Some(VimMotion::Left)),
            Key::Char('p') => VimAction::PasteAfter,
            Key::Char('P') => VimAction::PasteBefore,
            Key::Char('u') => VimAction::Undo,
            Key::Char('r') if key.ctrl => VimAction::Redo,
            Key::Char('.') => VimAction::Repeat,
            Key::Char('J') => VimAction::JoinLines,
            Key::Char('>') => VimAction::Indent,
            Key::Char('<') => VimAction::Outdent,

            Key::Escape => {
                self.count = None;
                self.operator = None;
                VimAction::Escape
            }

            _ => VimAction::None,
        }
    }

    /// Handle key event in insert mode
    fn handle_insert(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                VimAction::Escape
            }
            _ => VimAction::None, // Let the widget handle insert keys
        }
    }

    /// Handle key event in visual mode
    fn handle_visual(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                VimAction::Escape
            }
            Key::Char('d') | Key::Char('x') => {
                self.set_mode(VimMode::Normal);
                VimAction::Delete(None)
            }
            Key::Char('y') => {
                self.set_mode(VimMode::Normal);
                VimAction::Yank(None)
            }
            Key::Char('c') => {
                self.set_mode(VimMode::Insert);
                VimAction::Change(None)
            }
            // Movement in visual mode
            Key::Char('h') | Key::Left => VimAction::Move(VimMotion::Left),
            Key::Char('j') | Key::Down => VimAction::Move(VimMotion::Down),
            Key::Char('k') | Key::Up => VimAction::Move(VimMotion::Up),
            Key::Char('l') | Key::Right => VimAction::Move(VimMotion::Right),
            Key::Char('w') => VimAction::Move(VimMotion::Word),
            Key::Char('b') => VimAction::Move(VimMotion::WordBack),
            _ => VimAction::None,
        }
    }

    /// Handle key event in command mode
    fn handle_command(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                self.command_buffer.clear();
                VimAction::Escape
            }
            Key::Enter => {
                let cmd = self.command_buffer.clone();
                self.set_mode(VimMode::Normal);
                self.command_buffer.clear();
                VimAction::ExecuteCommand(cmd)
            }
            Key::Backspace => {
                self.command_buffer.pop();
                if self.command_buffer.is_empty() {
                    self.set_mode(VimMode::Normal);
                }
                VimAction::None
            }
            Key::Char(ch) => {
                self.command_buffer.push(ch);
                VimAction::None
            }
            _ => VimAction::None,
        }
    }

    /// Handle key event in search mode
    fn handle_search(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            Key::Escape => {
                self.set_mode(VimMode::Normal);
                self.search_pattern.clear();
                VimAction::Escape
            }
            Key::Enter => {
                self.set_mode(VimMode::Normal);
                VimAction::Move(if self.search_forward {
                    VimMotion::SearchNext
                } else {
                    VimMotion::SearchPrev
                })
            }
            Key::Backspace => {
                self.search_pattern.pop();
                if self.search_pattern.is_empty() {
                    self.set_mode(VimMode::Normal);
                }
                VimAction::None
            }
            Key::Char(ch) => {
                self.search_pattern.push(ch);
                VimAction::None
            }
            _ => VimAction::None,
        }
    }

    /// Convert character to motion
    fn char_to_motion(&self, ch: char) -> Option<VimMotion> {
        match ch {
            'h' => Some(VimMotion::Left),
            'j' => Some(VimMotion::Down),
            'k' => Some(VimMotion::Up),
            'l' => Some(VimMotion::Right),
            'w' => Some(VimMotion::Word),
            'b' => Some(VimMotion::WordBack),
            'e' => Some(VimMotion::WordEnd),
            '0' => Some(VimMotion::LineStart),
            '$' => Some(VimMotion::LineEnd),
            '^' => Some(VimMotion::FirstNonBlank),
            'G' => Some(VimMotion::GoToLine(None)),
            '{' => Some(VimMotion::ParagraphBack),
            '}' => Some(VimMotion::ParagraphForward),
            '%' => Some(VimMotion::MatchBracket),
            // Same key repeats = line
            'd' | 'y' | 'c' => Some(VimMotion::Down),
            _ => None,
        }
    }

    /// Handle a key event
    pub fn handle_key(&mut self, key: &KeyEvent) -> VimAction {
        // Check for 'gg' sequence
        if !self.key_buffer.is_empty() {
            if let Key::Char(ch) = key.key {
                if self.key_buffer == ['g'] && ch == 'g' {
                    self.key_buffer.clear();
                    return VimAction::Move(VimMotion::GoToLine(Some(1)));
                }
            }
            self.key_buffer.clear();
        }

        let action = match self.mode {
            VimMode::Normal => self.handle_normal(key),
            VimMode::Insert => self.handle_insert(key),
            VimMode::Visual | VimMode::VisualLine | VimMode::VisualBlock => self.handle_visual(key),
            VimMode::Command => self.handle_command(key),
            VimMode::Search => self.handle_search(key),
            VimMode::Replace => {
                if let Key::Char(ch) = key.key {
                    self.set_mode(VimMode::Normal);
                    VimAction::ReplaceChar(ch)
                } else if key.key == Key::Escape {
                    self.set_mode(VimMode::Normal);
                    VimAction::Escape
                } else {
                    VimAction::None
                }
            }
        };

        // Save for repeat
        if action != VimAction::None && action != VimAction::Escape {
            if matches!(
                action,
                VimAction::Delete(_)
                    | VimAction::Yank(_)
                    | VimAction::Change(_)
                    | VimAction::Insert
                    | VimAction::Append
                    | VimAction::OpenBelow
                    | VimAction::OpenAbove
            ) {
                self.last_action = Some(action.clone());
            }
        }

        // Reset count after action
        if action != VimAction::None {
            self.count = None;
        }

        action
    }

    /// Parse and execute a command
    pub fn execute_command(&mut self, cmd: &str) -> VimCommandResult {
        let cmd = cmd.trim();

        match cmd {
            "w" | "write" => VimCommandResult::Write,
            "q" | "quit" => VimCommandResult::Quit,
            "wq" | "x" => VimCommandResult::WriteQuit,
            "q!" => VimCommandResult::ForceQuit,
            "e" | "edit" => VimCommandResult::Edit(None),
            _ if cmd.starts_with("e ") || cmd.starts_with("edit ") => {
                let file = cmd.split_whitespace().nth(1).map(|s| s.to_string());
                VimCommandResult::Edit(file)
            }
            _ if cmd.starts_with("set ") => {
                let option = cmd[4..].trim();
                VimCommandResult::Set(option.to_string())
            }
            _ if cmd.chars().all(|c| c.is_ascii_digit()) => {
                let line: usize = cmd.parse().unwrap_or(1);
                VimCommandResult::GoToLine(line)
            }
            _ => VimCommandResult::Unknown(cmd.to_string()),
        }
    }
}

impl Default for VimState {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of executing a vim command
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VimCommandResult {
    /// Write file
    Write,
    /// Quit
    Quit,
    /// Write and quit
    WriteQuit,
    /// Force quit without saving
    ForceQuit,
    /// Edit file
    Edit(Option<String>),
    /// Set option
    Set(String),
    /// Go to line number
    GoToLine(usize),
    /// Unknown command
    Unknown(String),
}

/// Create a new vim state
pub fn vim_state() -> VimState {
    VimState::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vim_state_creation() {
        let vim = VimState::new();
        assert_eq!(vim.mode(), VimMode::Normal);
    }

    #[test]
    fn test_mode_switch() {
        let mut vim = VimState::new();

        let action = vim.handle_key(&KeyEvent::new(Key::Char('i')));
        assert_eq!(action, VimAction::Insert);
        assert_eq!(vim.mode(), VimMode::Insert);

        let action = vim.handle_key(&KeyEvent::new(Key::Escape));
        assert_eq!(action, VimAction::Escape);
        assert_eq!(vim.mode(), VimMode::Normal);
    }

    #[test]
    fn test_motions() {
        let mut vim = VimState::new();

        assert_eq!(
            vim.handle_key(&KeyEvent::new(Key::Char('j'))),
            VimAction::Move(VimMotion::Down)
        );

        assert_eq!(
            vim.handle_key(&KeyEvent::new(Key::Char('w'))),
            VimAction::Move(VimMotion::Word)
        );
    }

    #[test]
    fn test_count() {
        let mut vim = VimState::new();

        vim.handle_key(&KeyEvent::new(Key::Char('5')));
        assert_eq!(vim.count(), 5);

        vim.handle_key(&KeyEvent::new(Key::Char('j')));
        assert_eq!(vim.count(), 1); // Reset after action
    }

    #[test]
    fn test_operator_motion() {
        let mut vim = VimState::new();

        vim.handle_key(&KeyEvent::new(Key::Char('d')));
        let action = vim.handle_key(&KeyEvent::new(Key::Char('w')));
        assert_eq!(action, VimAction::Delete(Some(VimMotion::Word)));
    }

    #[test]
    fn test_command_mode() {
        let mut vim = VimState::new();

        vim.handle_key(&KeyEvent::new(Key::Char(':')));
        assert_eq!(vim.mode(), VimMode::Command);

        vim.handle_key(&KeyEvent::new(Key::Char('w')));
        vim.handle_key(&KeyEvent::new(Key::Char('q')));
        assert_eq!(vim.command_buffer(), "wq");

        let action = vim.handle_key(&KeyEvent::new(Key::Enter));
        assert_eq!(action, VimAction::ExecuteCommand("wq".to_string()));
    }

    #[test]
    fn test_execute_command() {
        let mut vim = VimState::new();

        assert_eq!(vim.execute_command("w"), VimCommandResult::Write);
        assert_eq!(vim.execute_command("q"), VimCommandResult::Quit);
        assert_eq!(vim.execute_command("wq"), VimCommandResult::WriteQuit);
        assert_eq!(vim.execute_command("42"), VimCommandResult::GoToLine(42));
    }

    #[test]
    fn test_gg_sequence() {
        let mut vim = VimState::new();

        vim.handle_key(&KeyEvent::new(Key::Char('g')));
        let action = vim.handle_key(&KeyEvent::new(Key::Char('g')));
        assert_eq!(action, VimAction::Move(VimMotion::GoToLine(Some(1))));
    }
}
