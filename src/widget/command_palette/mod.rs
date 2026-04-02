//! Command palette widget - Quick command search and execution
//!
//! This module provides a VS Code-style command palette for searching and executing
//! commands with fuzzy matching, keyboard navigation, and categorization.
//!
//! # Features
//!
//! | Feature | Description |
//!|---------|-------------|
//! | **Fuzzy Search** | Substring matching on command IDs, labels, descriptions |
//! | **Keyboard Navigation** | Arrow keys, Enter, Escape |
//! | **Categories** | Group related commands |
//! | **Shortcuts** | Display keyboard shortcuts |
//! | **Icons** | Visual indicators |
//! | **Pinning** | Pin frequently used commands |
//! | **Recent Commands** | Track recently used |
//!
//! # Quick Start
//!
//! ## Basic Command Palette
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let palette = command_palette()
//!     .command(Command::new("save", "Save File")
//!         .description("Save the current file")
//!         .shortcut("Ctrl+S"))
//!     .command(Command::new("open", "Open File")
//!         .description("Open a file")
//!         .shortcut("Ctrl+O"))
//!     .width(60)
//!     .max_visible(10);
//! ```
//!
//! ## Command Execution
//!
//! ```rust,ignore
//! use revue::widget::CommandPalette;
//!
//! let mut palette = CommandPalette::new();
//!
//! // Add commands
//! palette.add_command(Command::new("quit", "Quit").action(|| {
//!     println!("Quitting...");
//! }));
//!
//! // Show palette
//! palette.show();
//!
//! // Handle user input...
//!
//! // Check for execution
//! if let Some(cmd_id) = palette.execute() {
//!     println!("Executed: {}", cmd_id);
//! }
//! ```
//!
//! # Command Options
//!
//! ```rust,ignore
//! use revue::widget::Command;
//!
//! Command::new("format", "Format Code")
//!     .description("Format the current document")
//!     .shortcut("Shift+Alt+F")
//!     .category("Formatting")
//!     .icon("🎨")
//!     .pinned()     // Always show at top
//!     .recent()     // Mark as recently used
//!     .action(|| {
//!         // Execute command
//!     });
//! ```
//!
//! # Command Matching
//!
//! The command palette uses fuzzy matching to find commands:
//!
//! - **ID matching**: `sf` matches `save_file`
//! - **Label matching**: `sav` matches `Save File`
//! - **Description matching**: `disk` matches commands with "disk" in description
//! - **Scoring**: Exact matches > Prefix matches > Substring matches
//!
//! ```rust,ignore
//! let cmd = Command::new("save_file", "Save File").description("Save to disk");
//!
//! assert!(cmd.matches("save"));        // Label match
//! assert!(cmd.matches("file"));       // Label match
//! assert!(cmd.matches("sf"));          // Fuzzy: S_ave F_ile
//! assert!(cmd.matches("disk"));        // Description match
//! ```
//!
//! # Palette Styling
//!
//! ```rust,ignore
//! command_palette()
//!     .width(80)
//!     .max_visible(15)
//!     .placeholder("Type a command...")
//!     .prompt(">")
//!     .show_icons(true)
//!     .show_shortcuts(true);
//! ```

pub mod command;
pub mod core;
pub mod default;
pub mod helper;
pub mod impls;
pub mod styled;
pub mod view;

pub use command::*;
pub use core::CommandPalette;
pub use helper::command_palette;

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_palette() -> CommandPalette {
        CommandPalette::new()
            .command(Command::new("save", "Save File").shortcut("Ctrl+S"))
            .command(Command::new("open", "Open File").shortcut("Ctrl+O"))
            .command(
                Command::new("format", "Format Code")
                    .description("Format document")
                    .category("Editing"),
            )
            .command(Command::new("quit", "Quit Application"))
    }

    #[test]
    fn test_command_new() {
        let cmd = Command::new("test", "Test Command");
        assert_eq!(cmd.id, "test");
        assert_eq!(cmd.label, "Test Command");
        assert!(cmd.description.is_none());
        assert!(!cmd.recent);
        assert!(!cmd.pinned);
    }

    #[test]
    fn test_command_builder() {
        let cmd = Command::new("fmt", "Format")
            .description("Format code")
            .shortcut("Ctrl+F")
            .category("Edit")
            .icon('F')
            .recent()
            .pinned();
        assert_eq!(cmd.description, Some("Format code".into()));
        assert_eq!(cmd.shortcut, Some("Ctrl+F".into()));
        assert_eq!(cmd.category, Some("Edit".into()));
        assert_eq!(cmd.icon, Some('F'));
        assert!(cmd.recent);
        assert!(cmd.pinned);
    }

    #[test]
    fn test_command_fuzzy_match() {
        let cmd = Command::new("save_file", "Save File");
        assert!(cmd.matches("save"));
        assert!(cmd.matches("file"));
        assert!(cmd.matches("sf"));
        assert!(cmd.matches("")); // Empty matches all
    }

    #[test]
    fn test_command_match_score() {
        let pinned = Command::new("a", "A").pinned();
        let recent = Command::new("b", "B").recent();
        let normal = Command::new("c", "C");
        assert!(pinned.match_score("") > recent.match_score(""));
        assert!(recent.match_score("") > normal.match_score(""));
    }

    #[test]
    fn test_palette_new() {
        let p = CommandPalette::new();
        assert!(!p.is_visible());
        assert!(p.commands.is_empty());
    }

    #[test]
    fn test_palette_add_commands() {
        let p = sample_palette();
        assert_eq!(p.commands.len(), 4);
    }

    #[test]
    fn test_palette_show_hide() {
        let mut p = sample_palette();
        p.show();
        assert!(p.is_visible());
        p.hide();
        assert!(!p.is_visible());
        p.toggle();
        assert!(p.is_visible());
        p.toggle();
        assert!(!p.is_visible());
    }

    #[test]
    fn test_palette_filter() {
        let mut p = sample_palette();
        p.show();
        assert_eq!(p.filtered.len(), 4); // All match empty query

        p.set_query("save");
        assert!(p.filtered.len() < 4);
        assert_eq!(p.selected_id(), Some("save"));
    }

    #[test]
    fn test_palette_input_backspace() {
        let mut p = sample_palette();
        p.show();
        p.input('s');
        assert_eq!(p.get_query(), "s");
        p.input('a');
        assert_eq!(p.get_query(), "sa");
        p.backspace();
        assert_eq!(p.get_query(), "s");
        p.clear_query();
        assert_eq!(p.get_query(), "");
    }

    #[test]
    fn test_palette_navigation() {
        let mut p = sample_palette();
        p.show();
        let first = p.selected_id().map(|s| s.to_string());
        p.select_next();
        let second = p.selected_id().map(|s| s.to_string());
        assert_ne!(first, second);
        p.select_prev();
        assert_eq!(p.selected_id().map(|s| s.to_string()), first);
    }

    #[test]
    fn test_palette_execute() {
        let mut p = sample_palette();
        p.show();
        let id = p.execute();
        assert!(id.is_some());
        assert!(!p.is_visible()); // Hidden after execute
    }

    #[test]
    fn test_palette_handle_key() {
        use crate::event::Key;
        let mut p = sample_palette();
        p.show();
        assert!(p.handle_key(&Key::Down));
        assert!(p.handle_key(&Key::Escape));
        assert!(!p.is_visible());

        // Not visible = no key handling
        assert!(!p.handle_key(&Key::Down));
    }

    #[test]
    fn test_palette_add_remove_command() {
        let mut p = CommandPalette::new();
        p.add_command(Command::new("a", "A"));
        p.add_command(Command::new("b", "B"));
        assert_eq!(p.commands.len(), 2);

        p.remove_command("a");
        assert_eq!(p.commands.len(), 1);
        assert_eq!(p.commands[0].id, "b");

        p.clear_commands();
        assert!(p.commands.is_empty());
    }

    #[test]
    fn test_palette_mark_recent() {
        let mut p = sample_palette();
        assert!(!p.commands[0].recent);
        p.mark_recent("save");
        assert!(p.commands[0].recent);
    }

    #[test]
    fn test_palette_highlight_match() {
        let mut p = sample_palette();
        p.set_query("save");
        let highlights = p.highlight_match("Save File");
        // Some chars should be highlighted
        assert!(highlights.iter().any(|(_, matched)| *matched));
    }
}
