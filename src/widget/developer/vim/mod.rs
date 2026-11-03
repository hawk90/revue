//! Vim Mode system for terminal applications
//!
//! Provides vim-style modal editing with Normal, Insert, Visual,
//! and Command modes.

mod key_handling;
mod normal;
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
    /// The count that went with the action just returned, for [`count`](Self::count)
    action_count: Option<usize>,
    /// Pending operator
    operator: Option<char>,
    /// The count typed before the pending operator: vim multiplies it with
    /// the one typed after (`2d3w` deletes six words, #1363)
    operator_count: Option<usize>,
    /// `f`, `F`, `t` or `T`, waiting for the char to find
    pending_find: Option<char>,
    /// Whether replace mode is `r`'s, for one char, rather than `R`'s
    replace_once: bool,
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
            action_count: None,
            operator: None,
            operator_count: None,
            pending_find: None,
            replace_once: true,
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
        self.operator = None;
        self.operator_count = None;
        self.pending_find = None;
        self.count = None;
        self.action_count = None;
        self.key_buffer.clear();
    }

    /// The count being typed, or the one typed before the action
    /// [`handle_key`](Self::handle_key) just returned, until the next key
    /// (default 1). `3j` returns `Move(Down)` and leaves this at 3.
    pub fn count(&self) -> usize {
        // With an operator pending, the counts before and after it multiply
        let pending = match (self.operator_count, self.count) {
            (None, None) => None,
            (before, after) => Some(before.unwrap_or(1).saturating_mul(after.unwrap_or(1))),
        };
        pending.or(self.action_count).unwrap_or(1)
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

    /// Parse and execute a command. An empty one (`:` then Enter) is
    /// `Unknown("")`, which does nothing, as in vim; a line number past
    /// `usize` goes to the last line.
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
            _ if !cmd.is_empty() && cmd.chars().all(|c| c.is_ascii_digit()) => {
                // All digits: only too large fails to parse
                VimCommandResult::GoToLine(cmd.parse().unwrap_or(usize::MAX))
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
