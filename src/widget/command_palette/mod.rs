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

mod builders;
pub mod command;
mod filter;
mod input;
mod state;
mod view;

pub use command::*;

use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Command palette widget for quick command access
///
/// Provides a searchable command interface similar to VSCode's Ctrl+P
/// or Sublime Text's Command Palette.
#[derive(Clone)]
pub struct CommandPalette {
    /// All commands
    pub commands: Vec<Command>,
    /// Current search query
    pub query: String,
    /// Filtered command indices
    pub filtered: Vec<usize>,
    /// Selection state for filtered list (uses Selection utility)
    pub selection: Selection,
    /// Visible state
    pub visible: bool,
    /// Width
    pub width: u16,
    /// Max visible items
    pub max_visible: u16,
    /// Placeholder text
    pub placeholder: String,
    /// Title
    pub title: Option<String>,
    /// Show descriptions
    pub show_descriptions: bool,
    /// Show shortcuts
    pub show_shortcuts: bool,
    /// Show icons
    pub show_icons: bool,
    /// Colors
    pub bg_color: Color,
    /// Border color
    pub border_color: Color,
    /// Selected item background
    pub selected_bg: Color,
    /// Match highlight color
    pub match_color: Color,
    /// Widget properties
    pub props: WidgetProps,
}

impl_styled_view!(CommandPalette);
impl_props_builders!(CommandPalette);

/// Helper to create a command palette
pub fn command_palette() -> CommandPalette {
    CommandPalette::new()
}

#[cfg(test)]
mod tests;
