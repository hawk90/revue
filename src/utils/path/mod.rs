//! Path manipulation utilities
//!
//! Provides utilities for displaying and manipulating file paths
//! in user-friendly formats.
//!
//! # Example
//!
//! ```
//! use revue::utils::path::{abbreviate_path, home_dir, home_relative, shorten_path};
//!
//! // Replace the home directory with ~
//! if let Some(home) = home_dir() {
//!     assert!(home_relative(home.join("file.txt")).starts_with("~/"));
//! }
//!
//! // Shorten to fit width
//! let short = shorten_path("/very/long/path/to/file.txt", 20);
//! assert_eq!(short, ".../path/to/file.txt");
//!
//! // Abbreviate middle directories
//! let abbr = abbreviate_path("/Users/john/Documents/Projects/rust/src/main.rs");
//! assert_eq!(abbr, "/U/j/D/P/r/src/main.rs"); // the last two stay whole
//! ```

mod component;
mod display;
mod error;
mod format;
mod home;
mod util;
mod validate;

#[cfg(test)]
mod tests;

// Re-export all public items for backward compatibility
pub use component::{extension, filename, is_hidden, parent, stem};
pub use display::PathDisplay;
pub use error::PathError;
pub use format::{abbreviate_path, abbreviate_path_keep, relative_to, shorten_path};
pub use home::{expand_home, home_dir, home_relative};
pub use util::{common_prefix, join_paths, normalize_separators};
pub use validate::{
    validate_characters, validate_no_traversal, validate_relative_only, validate_within_base,
};
