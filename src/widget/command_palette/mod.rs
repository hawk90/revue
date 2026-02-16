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
