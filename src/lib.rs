//! # Revue
//!
//! A Vue-style TUI framework for Rust with CSS styling, reactive state management,
//! and a rich set of widgets for building beautiful terminal user interfaces.
//!
//! ## Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | **CSS Styling** | External CSS files with variables, selectors, transitions, and hot reload |
//! | **Flexbox Layout** | Custom TUI-optimized layout engine for flexible layouts |
//! | **Reactive State** | Vue-inspired Signal/Computed/Effect pattern |
//! | **100+ Widgets** | Text, Button, Input, Table, Tree, Modal, Toast, Charts, and more |
//! | **Markdown & Images** | Built-in markdown rendering with Kitty image protocol support |
//! | **Developer Tools** | Hot reload, widget inspector, snapshot testing (Pilot) |
//! | **Theming** | Built-in themes: Dracula, Nord, Monokai, Gruvbox, Catppuccin |
//!
//! ## Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! fn main() -> Result<()> {
//!     let mut app = App::builder().build();
//!     let counter = Counter::new();
//!
//!     app.run_with_handler(counter, |key, state| {
//!         state.handle_key(&key.key)
//!     })
//! }
//!
//! struct Counter { value: i32 }
//!
//! impl Counter {
//!     fn new() -> Self { Self { value: 0 } }
//!
//!     fn handle_key(&mut self, key: &Key) -> bool {
//!         match key {
//!             Key::Up => { self.value += 1; true }
//!             Key::Down => { self.value -= 1; true }
//!             _ => false,
//!         }
//!     }
//! }
//!
//! impl View for Counter {
//!     fn render(&self, ctx: &mut RenderContext) {
//!         vstack()
//!             .child(Text::new(format!("Count: {}", self.value)))
//!             .child(Text::muted("[↑/↓] to change, [q] to quit"))
//!             .render(ctx);
//!     }
//! }
//! ```
//!
//! ## CSS Styling
//!
//! Revue supports CSS for styling widgets:
//!
//! ```css
//! /* styles.css */
//! :root {
//!     --primary: #bd93f9;
//!     --bg: #282a36;
//! }
//!
//! .button {
//!     background: var(--primary);
//!     color: var(--bg);
//!     transition: background 0.3s ease;
//! }
//!
//! .button:hover {
//!     background: #ff79c6;
//! }
//! ```
//!
//! ## Reactive State
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! let count = signal(0);
//! let doubled = computed(move || count.get() * 2);
//!
//! let _logger = effect(move || {
//!     println!("Count changed to: {}", count.get());
//! });
//!
//! count.set(5); // Triggers effect, doubled is now 10
//! ```
//!
//! ## Widget Gallery
//!
//! ### Layout
//! - [`widget::vstack()`] / [`widget::hstack()`] - Vertical/horizontal stack layout
//! - [`widget::Border`] - Bordered container with title support
//! - [`widget::Tabs`] - Tab navigation
//! - [`widget::ScrollView`] - Scrollable content area
//! - [`widget::Layers`] - Overlapping widgets (for modals, toasts)
//!
//! ### Input
//! - [`widget::Input`] - Single-line text input
//! - [`widget::TextArea`] - Multi-line text editor
//! - [`widget::Button`] - Clickable button with variants
//! - [`widget::Checkbox`] - Toggle checkbox
//! - [`widget::RadioGroup`] - Radio button group
//! - [`widget::Select`] - Dropdown selection
//!
//! ### Display
//! - [`widget::Text`] - Styled text display
//! - [`widget::Progress`] - Progress bar
//! - [`widget::Spinner`] - Loading spinner
//! - [`widget::Badge`] / [`widget::Tag`] - Labels and tags
//! - [`widget::Avatar`] - User avatar display
//! - [`widget::Skeleton`] - Loading placeholder
//!
//! ### Data
//! - [`widget::Table`] - Data table with columns
//! - [`widget::List`] - Selectable list
//! - [`widget::Tree`] - Hierarchical tree view
//! - [`widget::Sparkline`] - Inline mini chart
//! - [`widget::BarChart`] - Bar chart visualization
//! - [`widget::Canvas`] / [`widget::BrailleCanvas`] - Custom drawing
//!
//! ### Feedback
//! - [`widget::Modal`] - Dialog overlay
//! - [`widget::Toast`] - Notification popup
//! - [`widget::CommandPalette`] - Fuzzy command search (Ctrl+P)
//!
//! ## Module Overview
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`core::app`] | Application lifecycle and event loop |
//! | [`dom`] | Virtual DOM and rendering tree |
//! | [`event`] | Keyboard/mouse events and keymaps |
//! | [`layout`] | Flexbox layout engine |
//! | [`reactive`] | Signal/Computed/Effect primitives |
//! | [`render`] | Terminal rendering and buffer |
//! | [`style`] | CSS parsing and theming |
//! | [`testing`] | Pilot testing framework |
//! | [`widget`] | All widget implementations |
//! | [`worker`] | Background task execution |
//!
//! ## Testing with Pilot
//!
//! ```rust,ignore
//! use revue::testing::{Pilot, TestApp};
//!
//! #[test]
//! fn test_counter() {
//!     let mut app = TestApp::new(Counter::new());
//!     let mut pilot = Pilot::new(&mut app);
//!
//!     pilot
//!         .press_key(Key::Up)
//!         .press_key(Key::Up)
//!         .assert_contains("2");
//! }
//! ```
//!
//! ## Themes
//!
//! Built-in themes available via [`style::themes`]:
//!
//! - **Dracula** - Dark purple theme
//! - **Nord** - Arctic blue theme
//! - **Monokai** - Classic dark theme
//! - **Gruvbox** - Retro groove theme
//! - **Catppuccin** - Pastel dark theme
//!
//! ## Comparison with Other Frameworks
//!
//! | Feature | Revue | Ratatui | Textual | Cursive |
//! |---------|-------|---------|---------|---------|
//! | Language | Rust | Rust | Python | Rust |
//! | CSS Styling | ✅ | ❌ | ✅ | ❌ |
//! | Reactive State | ✅ | ❌ | ✅ | ❌ |
//! | Hot Reload | ✅ | ❌ | ✅ | ❌ |
//! | Widget Count | 100+ | ~14 | ~37 | ~40 |
//! | Snapshot Testing | ✅ | ❌ | ❌ | ❌ |

#![warn(missing_docs)]

/// The version of the library, including the git commit hash in development builds.
///
/// In production releases (from crates.io), this will be a semver like "2.33.4".
///
/// In development builds, this will be in the format "2.33.4-SHA" where SHA is the
/// short git commit hash, allowing precise identification of the exact code version.
pub const VERSION: &str = env!("REVUE_VERSION");

/// The full git commit hash of this build.
///
/// Empty in release builds, contains the 40-character commit SHA in development builds.
pub const GIT_SHA: &str = env!("GIT_SHA");

/// Whether this is a development build (with commit hash in version) or a release build.
///
/// Returns `true` for development builds (version includes SHA), `false` for releases.
pub fn is_dev_build() -> bool {
    env!("REVUE_IS_DEV") == "true"
}

// Internal logging macros - no-op when tracing feature is disabled
#[cfg(feature = "tracing")]
macro_rules! log_debug {
    ($($arg:tt)*) => { tracing::debug!($($arg)*) }
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_debug {
    ($($arg:tt)*) => { { let _ = ($($arg)*,); } }
}
pub(crate) use log_debug;

#[cfg(feature = "tracing")]
macro_rules! log_warn {
    ($($arg:tt)*) => { tracing::warn!($($arg)*) }
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_warn {
    ($($arg:tt)*) => { { let _ = ($($arg)*,); } }
}
pub(crate) use log_warn;

#[cfg(feature = "tracing")]
macro_rules! log_error {
    ($($arg:tt)*) => { tracing::error!($($arg)*) }
}
#[cfg(not(feature = "tracing"))]
macro_rules! log_error {
    ($($arg:tt)*) => { { let _ = ($($arg)*,); } }
}
pub(crate) use log_error;

// Core modules
pub mod core;
pub use core::constants; // Re-export from core

// Runtime systems
pub mod runtime;
pub use runtime::{dom, event, layout, render, style};

// State management
pub mod state;
pub use state::{patterns, plugin, reactive, tasks, worker};

// Other modules (keep at root for now)
pub mod a11y;
pub mod devtools;
pub mod query;
pub mod testing;
pub mod text;
pub mod utils;
pub mod widget;

// Re-export derive macros
pub use revue_macros::Store;

/// Error type for Revue operations.
///
/// This enum covers all error cases that can occur when using Revue,
/// including CSS parsing errors, I/O errors, and general runtime errors.
///
/// # Example
///
/// ```rust,ignore
/// use revue::{Error, Result};
///
/// fn load_styles() -> Result<()> {
///     // Operations that might fail...
///     Ok(())
/// }
/// ```
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// CSS parsing error.
    ///
    /// Occurs when parsing invalid CSS syntax or unsupported properties.
    #[error("CSS error: {0}")]
    Css(#[from] style::ParseError),

    /// I/O error.
    ///
    /// Occurs during file operations (loading CSS, images, etc.).
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Layout error.
    ///
    /// Occurs during layout computation.
    #[error("Layout error: {0}")]
    Layout(#[from] layout::LayoutError),

    /// Rendering error.
    ///
    /// Occurs during buffer operations or rendering.
    #[error("Render error: {0}")]
    Render(String),

    /// Generic error with custom message.
    ///
    /// Used for errors that don't fit other categories.
    /// This variant preserves the underlying error source for better debugging.
    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type alias for Revue operations.
///
/// Shorthand for `std::result::Result<T, revue::Error>`.
///
/// # Example
///
/// ```rust,ignore
/// use revue::Result;
///
/// fn load_config() -> Result<String> {
///     let content = std::fs::read_to_string("config.css")?;
///     Ok(content)
/// }
/// ```
pub type Result<T> = std::result::Result<T, Error>;

// ─────────────────────────────────────────────────────────────────────────
// Error Handling Guidelines
// ─────────────────────────────────────────────────────────────────────────

/// # Error Handling Guidelines
///
/// Revue follows these error handling patterns to provide clear, actionable error messages.
///
/// ## Public APIs
///
/// **Always return `Result<T>`** for public APIs that can fail:
///
/// ```rust,ignore
/// use revue::Result;
///
/// pub fn parse_css(input: &str) -> Result<Stylesheet> {
///     // Parse and return Ok(stylesheet) or Err(Error)
/// }
/// ```
///
/// ## Internal Code
///
/// Choose the appropriate error handling strategy:
///
/// | Situation | Use | Example |
/// |-----------|-----|---------|
/// | Recoverable error | `Result<T, E>` | File not found, invalid input |
/// | Optional value | `Option<T>` | Missing config, lookup by ID |
/// | Truly invariant | `expect(msg)` | Logic errors, "never happens" |
/// | Never fails | `unwrap()` // With comment explaining why | Static constants, known-good values |
///
/// ## Error Type Hierarchy
///
/// Revue uses `thiserror` for error definitions:
///
/// ```rust,ignore
/// #[derive(Debug, thiserror::Error)]
/// pub enum Error {
///     #[error("CSS error: {0}")]
///     Css(#[from] style::ParseError),
///
///     #[error("I/O error: {0}")]
///     Io(#[from] std::io::Error),
///
///     #[error("Layout error: {0}")]
///     Layout(#[from] layout::LayoutError),
///
///     #[error("Render error: {0}")]
///     Render(String),
///
///     #[error("Unexpected error: {0}")]
///     Other(#[from] anyhow::Error),
/// }
/// ```
///
/// ## Error Context
///
/// **Good: Use anyhow for context**
///
/// ```rust,ignore
/// use anyhow::Context;
///
/// let file = std::fs::read_to_string(path)
///     .context("Failed to read config")?;
/// ```
///
/// **Bad: Lose context**
///
/// ```rust,ignore
/// let file = std::fs::read_to_string(path)?;  // Error doesn't mention config
/// ```
///
/// ## Converting Errors
///
/// Use `?` operator for automatic conversion:
///
/// ```rust,ignore
/// use revue::Result;
///
/// fn load_stylesheet(path: &str) -> Result<Stylesheet> {
///     let content = std::fs::read_to_string(path)?;  // io::Error → Error::Io
///     Ok(parse_css(&content)?)
/// }
/// ```
///
/// Prelude module for convenient imports.
///
/// Import everything you need with a single line:
///
/// ```rust,ignore
/// use revue::prelude::*;
/// ```
///
/// # Included Items
///
/// ## Core Types
/// - [`core::app::App`] - Application builder and runner
/// - [`widget::View`], [`widget::RenderContext`] - Widget rendering trait
/// - [`Result`] - Error handling
///
/// ## Events
/// - [`event::Key`], [`event::KeyEvent`], [`event::Event`] - Input handling
///
/// ## Reactive
/// - [`reactive::signal`], [`reactive::computed`], [`reactive::effect`] - State primitives
/// - [`reactive::Signal`], [`reactive::Computed`] - Reactive types
///
/// ## Layout
/// - [`widget::vstack`], [`widget::hstack`] - Stack layouts
/// - [`layout::Rect`] - Rectangle geometry
///
/// ## Widgets
/// All 85+ widgets and their constructors are included.
/// See [`widget`] module documentation for the full list.
///
/// ## Testing
/// - [`testing::Pilot`], [`testing::TestApp`], [`testing::TestConfig`] - Testing utilities
///
/// ## Workers
/// - [`worker::WorkerPool`], [`worker::WorkerHandle`] - Background tasks
pub mod prelude;

// Tests extracted to tests/lib_tests.rs
// All tests use only public constants, functions, and types from revue
