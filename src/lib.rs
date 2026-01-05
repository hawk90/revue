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
//! | **Flexbox Layout** | Powered by [taffy](https://github.com/DioxusLabs/taffy) for flexible layouts |
//! | **Reactive State** | Vue-inspired Signal/Computed/Effect pattern |
//! | **40+ Widgets** | Text, Button, Input, Table, Tree, Modal, Toast, Charts, and more |
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
//! - [`widget::Markdown`] - Markdown rendering with syntax highlighting
//! - [`widget::Image`] - Terminal images (Kitty protocol)
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
//! | [`app`] | Application lifecycle and event loop |
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
//! | Widget Count | 40+ | 13 | 35+ | 40+ |
//! | Snapshot Testing | ✅ | ❌ | ❌ | ❌ |

#![warn(missing_docs)]

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

pub mod app;
pub mod constants;
pub mod devtools;
pub mod dom;
pub mod event;
pub mod layout;
pub mod patterns;
pub mod plugin;
pub mod query;
pub mod reactive;
pub mod render;
pub mod style;
pub mod tasks;
pub mod testing;
pub mod text;
pub mod utils;
pub mod widget;
pub mod worker;

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

    /// Generic error with custom message.
    ///
    /// Used for errors that don't fit other categories.
    #[error("{0}")]
    Other(String),
}

/// Result type alias for Revue operations.
///
/// Shorthand for `std::result::Result<T, revue::Error>`.
pub type Result<T> = std::result::Result<T, Error>;

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
/// - [`app::App`] - Application builder and runner
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
/// All 40+ widgets and their constructors are included.
/// See [`widget`] module documentation for the full list.
///
/// ## Testing
/// - [`testing::Pilot`], [`testing::TestApp`], [`testing::TestConfig`] - Testing utilities
///
/// ## Workers
/// - [`worker::WorkerPool`], [`worker::WorkerHandle`] - Background tasks
pub mod prelude {
    // App
    pub use crate::app::App;

    // Events
    pub use crate::event::{Event, Key, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

    // Layout
    pub use crate::layout::Rect;

    // Reactive primitives
    pub use crate::reactive::{computed, effect, signal, Computed, Signal};

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
