//! Built-in Vim and Emacs keymap presets

use super::{KeymapConfig, Mode};

/// Vim-style keymap preset
pub fn vim_preset() -> KeymapConfig {
    let mut config = KeymapConfig::new();

    // Normal mode
    config.bind(Mode::Normal, "h", "move_left");
    config.bind(Mode::Normal, "j", "move_down");
    config.bind(Mode::Normal, "k", "move_up");
    config.bind(Mode::Normal, "l", "move_right");
    config.bind(Mode::Normal, "i", "enter_insert");
    config.bind(Mode::Normal, "a", "append");
    config.bind(Mode::Normal, "A", "append_end");
    config.bind(Mode::Normal, "o", "open_below");
    config.bind(Mode::Normal, "O", "open_above");
    config.bind(Mode::Normal, "v", "enter_visual");
    config.bind(Mode::Normal, ":", "enter_command");
    config.bind(Mode::Normal, "/", "search_forward");
    config.bind(Mode::Normal, "?", "search_backward");
    config.bind(Mode::Normal, "n", "search_next");
    config.bind(Mode::Normal, "N", "search_prev");
    config.bind(Mode::Normal, "g g", "goto_first");
    config.bind(Mode::Normal, "G", "goto_last");
    config.bind(Mode::Normal, "Ctrl-u", "page_up");
    config.bind(Mode::Normal, "Ctrl-d", "page_down");
    config.bind(Mode::Normal, "d d", "delete_line");
    config.bind(Mode::Normal, "y y", "yank_line");
    config.bind(Mode::Normal, "p", "paste_after");
    config.bind(Mode::Normal, "P", "paste_before");
    config.bind(Mode::Normal, "u", "undo");
    config.bind(Mode::Normal, "Ctrl-r", "redo");

    // Insert mode
    config.bind(Mode::Insert, "Escape", "exit_insert");
    config.bind(Mode::Insert, "Ctrl-c", "exit_insert");

    // Visual mode
    config.bind(Mode::Visual, "Escape", "exit_visual");
    config.bind(Mode::Visual, "h", "extend_left");
    config.bind(Mode::Visual, "j", "extend_down");
    config.bind(Mode::Visual, "k", "extend_up");
    config.bind(Mode::Visual, "l", "extend_right");
    config.bind(Mode::Visual, "y", "yank_selection");
    config.bind(Mode::Visual, "d", "delete_selection");

    // Command mode
    config.bind(Mode::Command, "Escape", "exit_command");
    config.bind(Mode::Command, "Enter", "execute_command");

    // Global
    config.bind_global("Ctrl-c", "quit");
    config.bind_global("Ctrl-z", "suspend");

    config
}

/// Emacs-style keymap preset
pub fn emacs_preset() -> KeymapConfig {
    let mut config = KeymapConfig::new();

    // Navigation
    config.bind(Mode::Normal, "Ctrl-p", "move_up");
    config.bind(Mode::Normal, "Ctrl-n", "move_down");
    config.bind(Mode::Normal, "Ctrl-b", "move_left");
    config.bind(Mode::Normal, "Ctrl-f", "move_right");
    config.bind(Mode::Normal, "Ctrl-a", "line_start");
    config.bind(Mode::Normal, "Ctrl-e", "line_end");
    config.bind(Mode::Normal, "Alt-<", "goto_first");
    config.bind(Mode::Normal, "Alt->", "goto_last");
    config.bind(Mode::Normal, "Ctrl-v", "page_down");
    config.bind(Mode::Normal, "Alt-v", "page_up");

    // Editing
    config.bind(Mode::Normal, "Ctrl-d", "delete_char");
    config.bind(Mode::Normal, "Ctrl-k", "kill_line");
    config.bind(Mode::Normal, "Ctrl-y", "yank");
    config.bind(Mode::Normal, "Ctrl-w", "cut_region");
    config.bind(Mode::Normal, "Alt-w", "copy_region");

    // Search
    config.bind(Mode::Normal, "Ctrl-s", "search_forward");
    config.bind(Mode::Normal, "Ctrl-r", "search_backward");

    // Undo
    config.bind(Mode::Normal, "Ctrl-/", "undo");
    config.bind(Mode::Normal, "Ctrl-x u", "undo");

    // File operations
    config.bind(Mode::Normal, "Ctrl-x Ctrl-s", "save");
    config.bind(Mode::Normal, "Ctrl-x Ctrl-c", "quit");
    config.bind(Mode::Normal, "Ctrl-x Ctrl-f", "open_file");

    config
}
