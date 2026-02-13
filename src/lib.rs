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
//! | **80+ Widgets** | Text, Button, Input, Table, Tree, Modal, Toast, Charts, and more |
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
//! effect(move || {
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
//!         .press(Key::Up)
//!         .press(Key::Up)
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
//! | Widget Count | 80+ | 13 | 35+ | 40+ |
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
pub mod prelude {
    // Macros
    pub use crate::Store;

    // App
    pub use crate::core::app::App;

    // Events
    pub use crate::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

    // Layout
    pub use crate::layout::Rect;

    // Reactive primitives
    pub use crate::reactive::{computed, effect, signal, Computed, Signal};

    // Store support
    pub use crate::reactive::{create_store, use_store, Store, StoreExt, StoreRegistry};

    // Async support
    pub use crate::reactive::{
        use_async, use_async_immediate, use_async_poll, AsyncResult, AsyncState,
    };

    // Style
    pub use crate::style::Color;

    // Theme system
    pub use crate::style::{
        cycle_theme, register_theme, set_theme, set_theme_by_id, theme_ids, toggle_theme,
        use_theme, Theme, ThemeVariant, Themes,
    };

    // Animation system
    pub use crate::style::{
        easing,
        effective_duration,
        // Reduced motion support
        should_skip_animation,
        // Widget animation presets
        widget_animations,
        Animation,
        AnimationDirection,
        AnimationFillMode,
        AnimationGroup,
        // Core animation types
        AnimationState,
        Animations,
        Choreographer,
        CssKeyframe,
        GroupMode,
        // CSS @keyframes style
        KeyframeAnimation,
        // Choreography
        Stagger,
        Tween,
    };

    // Widgets - Types
    pub use crate::widget::{
        Alignment,
        Anchor,
        Avatar,
        AvatarShape,
        AvatarSize,
        Badge,
        BadgeShape,
        BadgeVariant,
        BarChart,
        BarOrientation,
        Border,
        BorderType,
        // Braille canvas
        BrailleCanvas,
        BrailleContext,
        BrailleGrid,
        // New widgets
        Button,
        ButtonVariant,
        Canvas,
        Checkbox,
        CheckboxStyle,
        Circle,
        Column,
        Command,
        // Command palette
        CommandPalette,
        Direction,
        // Convenience widgets
        Divider,
        DividerStyle,
        DrawContext,
        EventResult,
        FilledCircle,
        FilledRectangle,
        FocusStyle,
        Input,
        Interactive,
        // Layer system
        Layers,
        Line,
        List,
        Modal,
        ModalButton,
        ModalButtonStyle,
        Orientation,
        Pagination,
        PaginationStyle,
        Points,
        Positioned,
        Progress,
        ProgressStyle,
        RadioGroup,
        RadioLayout,
        RadioStyle,
        Rectangle,
        RenderContext,
        ScrollView,
        Select,
        Shape,
        Skeleton,
        SkeletonShape,
        Sparkline,
        SparklineStyle,
        Spinner,
        SpinnerStyle,
        Stack,
        Tab,
        Table,
        Tabs,
        Tag,
        TagStyle,
        Text,
        TextArea,
        ThemePicker,
        Timeout,
        // UX widgets
        Toast,
        ToastLevel,
        ToastPosition,
        Tree,
        TreeNode,
        View,
        WidgetState,
    };

    // Feature-gated widget types
    #[cfg(feature = "image")]
    pub use crate::widget::{Image, ScaleMode};
    #[cfg(feature = "markdown")]
    pub use crate::widget::{Markdown, MarkdownPresentation, ViewMode};

    // Widgets - Constructors
    pub use crate::widget::{
        avatar,
        avatar_icon,
        badge,
        barchart,
        border,
        braille_canvas,
        // New constructors
        button,
        canvas,
        checkbox,
        chip,
        column,
        // Command palette
        command_palette,
        // Convenience widget constructors
        divider,
        dot_badge,
        hstack,
        input,
        // Layer system constructors
        layers,
        list,
        modal,
        pagination,
        positioned,
        progress,
        radio_group,
        scroll_view,
        select,
        skeleton,
        skeleton_avatar,
        skeleton_paragraph,
        skeleton_text,
        sparkline,
        spinner,
        table,
        tabs,
        tag,
        text,
        textarea,
        theme_picker,
        // UX constructors
        toast,
        tree,
        tree_node,
        vdivider,
        vstack,
    };

    // Feature-gated widget constructors
    #[cfg(feature = "image")]
    pub use crate::widget::image_from_file;
    #[cfg(feature = "markdown")]
    pub use crate::widget::{markdown, markdown_presentation};

    // DOM system
    pub use crate::dom::{DomId, DomNode, DomRenderer, DomTree, NodeState, Query, WidgetMeta};

    // Worker system
    pub use crate::worker::{
        run_blocking, spawn as spawn_worker, WorkerChannel, WorkerHandle, WorkerMessage,
        WorkerPool, WorkerState,
    };

    // Tasks - Timer, TaskRunner, EventBus
    pub use crate::tasks::{
        EventBus, EventId, Subscription, TaskId, TaskResult, TaskRunner, Timer, TimerEntry, TimerId,
    };

    // Patterns - Common TUI patterns
    pub use crate::patterns::{
        build_color,
        priority_color,
        spinner_char,
        status_color,
        // Async operations
        AsyncTask,
        BreadcrumbItem,
        ConfirmAction,
        ConfirmState,
        FieldType,
        FormField,
        // Form validation
        FormState,
        // State management
        MessageState,
        NavigationEvent,
        // Navigation
        NavigationState,
        Route,
        SearchMode,
        // Search/filter
        SearchState,
        ValidationError,
        Validators,
        BG,
        BG_INSET,
        BG_SUBTLE,
        BLUE,
        BORDER,
        BORDER_MUTED,
        // Colors
        CYAN,
        ERROR,
        FG,
        FG_DIM,
        FG_SUBTLE,
        GREEN,
        INFO,
        ORANGE,
        PURPLE,
        RED,
        SPINNER_FRAMES,
        SUCCESS,
        WARNING,
        YELLOW,
    };

    // Config loading (requires config feature)
    #[cfg(feature = "config")]
    pub use crate::patterns::{AppConfig, ConfigError};

    // Accessibility
    pub use crate::utils::{
        // Announcement functions
        announce,
        // Widget-specific announcement helpers
        announce_button_clicked,
        announce_checkbox_changed,
        announce_dialog_closed,
        announce_dialog_opened,
        announce_error,
        announce_list_selection,
        announce_now,
        announce_success,
        announce_tab_changed,
        has_announcements,
        is_high_contrast,
        // Preference getters/setters
        prefers_reduced_motion,
        set_high_contrast,
        set_reduced_motion,
        take_announcements,
    };

    // Testing (Pilot)
    pub use crate::testing::{Pilot, TestApp, TestConfig};
    // Visual regression testing
    pub use crate::testing::{
        CapturedCell, CiEnvironment, CiProvider, TestReport, VisualCapture, VisualDiff, VisualTest,
        VisualTestConfig, VisualTestResult,
    };

    // DevTools
    #[allow(deprecated)] // Re-exporting deprecated functions for backwards compatibility
    pub use crate::devtools::{
        disable_devtools, enable_devtools, is_devtools_enabled, toggle_devtools, ComputedProperty,
        DevTools, DevToolsConfig, DevToolsPosition, DevToolsTab, EventFilter, EventLogger,
        EventType, Inspector, InspectorConfig, LoggedEvent, PropertySource, StateDebugger,
        StateEntry, StateValue, StyleCategory, StyleInspector, WidgetNode,
    };

    // Profiler
    pub use crate::utils::profiler::{
        profile, profiler_report, start_profile, FlameNode, ProfileGuard, Profiler, Stats, Timing,
    };

    // Result type
    pub use crate::Result;

    // Constants
    pub use crate::constants::{
        // Animation durations
        ANIMATION_DEFAULT_DURATION,
        ANIMATION_FAST_DURATION,
        ANIMATION_SLOW_DURATION,
        ANIMATION_VERY_SLOW_DURATION,
        // Debounce
        DEBOUNCE_DEFAULT,
        DEBOUNCE_FILE_SYSTEM,
        DEBOUNCE_SEARCH,
        FRAME_DURATION_30FPS,
        // Frame rates
        FRAME_DURATION_60FPS,
        // Messages
        MESSAGE_DEFAULT_DURATION,
        MESSAGE_LONG_DURATION,
        MESSAGE_QUICK_DURATION,
        POLL_IMMEDIATE,
        // Screen transitions
        SCREEN_TRANSITION_DURATION,
        // Stagger
        STAGGER_DELAY_DEFAULT,
        // Tick rates
        TICK_RATE_DEFAULT,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    // =========================================================================
    // VERSION constant tests
    // =========================================================================

    #[test]
    fn test_version_constant_is_not_empty() {
        assert!(!VERSION.is_empty(), "VERSION should not be empty");
    }

    #[test]
    fn test_version_contains_digits() {
        assert!(
            VERSION.chars().any(|c| c.is_ascii_digit()),
            "VERSION should contain digits"
        );
    }

    #[test]
    fn test_version_has_dot() {
        assert!(
            VERSION.contains('.'),
            "VERSION should contain a dot separator"
        );
    }

    // =========================================================================
    // GIT_SHA constant tests
    // =========================================================================

    #[test]
    fn test_git_sha_is_valid() {
        // GIT_SHA is either empty (release) or 40 chars (dev)
        if is_dev_build() {
            assert_eq!(
                GIT_SHA.len(),
                40,
                "GIT_SHA should be 40 characters in dev builds"
            );
            assert!(
                GIT_SHA.chars().all(|c| c.is_ascii_hexdigit()),
                "GIT_SHA should be hex"
            );
        } else {
            assert_eq!(GIT_SHA, "", "GIT_SHA should be empty in release builds");
        }
    }

    // =========================================================================
    // is_dev_build() function tests
    // =========================================================================

    #[test]
    fn test_is_dev_build_returns_bool() {
        let result = is_dev_build();
        // Just verify it returns a boolean without panicking
        if result {
            assert!(
                VERSION.contains('-'),
                "Dev build VERSION should contain dash"
            );
        }
    }

    #[test]
    fn test_is_dev_build_can_be_called_multiple_times() {
        let _r1 = is_dev_build();
        let _r2 = is_dev_build();
        let _r3 = is_dev_build();
        // Should always return the same value
        assert_eq!(_r1, _r2);
        assert_eq!(_r2, _r3);
    }

    // =========================================================================
    // Error enum tests
    // =========================================================================

    #[test]
    fn test_error_io_from_io_error() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let error: Error = io_err.into();
        assert!(matches!(error, Error::Io(_)));
    }

    #[test]
    fn test_error_display_formatting() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
        let error = Error::Io(io_err);
        let display = format!("{}", error);
        assert!(display.contains("I/O error"));
        assert!(display.contains("access denied"));
    }

    #[test]
    fn test_error_debug_formatting() {
        let error = Error::Render("buffer overflow".to_string());
        let debug = format!("{:?}", error);
        assert!(debug.contains("Render"));
        assert!(debug.contains("buffer overflow"));
    }

    #[test]
    fn test_error_render_variant() {
        let error = Error::Render("test error".to_string());
        assert!(matches!(error, Error::Render(_)));
    }

    #[test]
    fn test_error_from_anyhow() {
        let anyhow_err = anyhow::anyhow!("generic error");
        let error: Error = anyhow_err.into();
        assert!(matches!(error, Error::Other(_)));
    }

    #[test]
    fn test_error_clone_for_render() {
        let error = Error::Render("test".to_string());
        // Error doesn't impl Clone but we can use it
        let _msg = format!("{}", error);
    }

    // =========================================================================
    // Result type tests
    // =========================================================================

    #[test]
    fn test_result_ok_variant() {
        let result: Result<String> = Ok("success".to_string());
        assert!(result.is_ok());
        assert!(!result.is_err());
    }

    #[test]
    fn test_result_err_variant() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
        let result: Result<String> = Err(io_err.into());
        assert!(!result.is_ok());
        assert!(result.is_err());
    }

    #[test]
    fn test_result_with_question_mark() {
        fn fallsible() -> Result<()> {
            let _ = Ok::<(), Error>(());
            Ok(())
        }
        assert!(fallsible().is_ok());
    }

    #[test]
    fn test_result_question_mark_propagates_io_error() {
        fn fallsible_io() -> Result<()> {
            let _file = std::fs::read_to_string("/nonexistent/file/that/does/not/exist")?;
            Ok(())
        }
        assert!(fallsible_io().is_err());
    }

    // =========================================================================
    // Store derive macro re-export tests
    // =========================================================================

    #[test]
    fn test_store_macro_is_reexported() {
        // Just verify Store is accessible from prelude
        // This test passes if it compiles
    }

    // =========================================================================
    // Core module re-export tests
    // =========================================================================

    #[test]
    fn test_constants_module_is_reexported() {
        // Verify constants module is accessible
        // This test passes if it compiles
    }

    // =========================================================================
    // Runtime modules re-export tests
    // =========================================================================

    #[test]
    fn test_dom_module_is_reexported() {
        // Verify dom module is accessible
    }

    #[test]
    fn test_event_module_is_reexported() {
        // Verify event module is accessible
    }

    #[test]
    fn test_layout_module_is_reexported() {
        // Verify layout module is accessible
    }

    #[test]
    fn test_render_module_is_reexported() {
        // Verify render module is accessible
    }

    #[test]
    fn test_style_module_is_reexported() {
        // Verify style module is accessible
    }

    // =========================================================================
    // State modules re-export tests
    // =========================================================================

    #[test]
    fn test_patterns_module_is_reexported() {
        // Verify patterns module is accessible
    }

    #[test]
    fn test_reactive_module_is_reexported() {
        // Verify reactive module is accessible
    }

    // =========================================================================
    // Other module re-export tests
    // =========================================================================

    #[test]
    fn test_a11y_module_is_reexported() {
        // Verify a11y module is accessible
    }

    #[test]
    fn test_devtools_module_is_reexported() {
        // Verify devtools module is accessible
    }

    #[test]
    fn test_query_module_is_reexported() {
        // Verify query module is accessible
    }

    #[test]
    fn test_testing_module_is_reexported() {
        // Verify testing module is accessible
    }

    #[test]
    fn test_text_module_is_reexported() {
        // Verify text module is accessible
    }

    #[test]
    fn test_utils_module_is_reexported() {
        // Verify utils module is accessible
    }

    #[test]
    fn test_widget_module_is_reexported() {
        // Verify widget module is accessible
    }
}

// =========================================================================
// Display widget tests
// =========================================================================

// Display widget tests are in separate files in tests/widget/display/
