//! Vim Mode system for terminal applications
//!
//! Provides vim-style modal editing with Normal, Insert, Visual,
//! and Command modes.

mod key_handling;
mod types;

pub use types::{VimAction, VimCommandResult, VimMode, VimMotion};

use std::collections::HashMap;

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

/// Create a new vim state
pub fn vim_state() -> VimState {
    VimState::new()
}
