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
#![allow(dead_code)] // Allow during development

pub mod app;
pub mod constants;
pub mod devtools;
pub mod dom;
pub mod event;
pub mod layout;
pub mod patterns;
pub mod plugin;
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
    pub use crate::event::{Key, KeyEvent, Event, MouseEvent, MouseEventKind, MouseButton};

    // Layout
    pub use crate::layout::Rect;

    // Reactive primitives
    pub use crate::reactive::{signal, computed, effect, Signal, Computed};

    // Async support
    pub use crate::reactive::{
        AsyncState, AsyncResult, use_async, use_async_poll, use_async_immediate,
    };

    // Style
    pub use crate::style::Color;

    // Theme system
    pub use crate::style::{
        Theme, ThemeVariant, Themes,
        use_theme, set_theme, set_theme_by_id, toggle_theme, cycle_theme,
        theme_ids, register_theme,
    };

    // Animation system
    pub use crate::style::{
        // Core animation types
        AnimationState, Tween, Animation, Animations, easing,
        // CSS @keyframes style
        KeyframeAnimation, CssKeyframe, AnimationDirection, AnimationFillMode,
        // Choreography
        Stagger, AnimationGroup, GroupMode, Choreographer,
        // Widget animation presets
        widget_animations,
        // Reduced motion support
        should_skip_animation, effective_duration,
    };

    // Widgets - Types
    pub use crate::widget::{
        View, RenderContext, Timeout, WidgetState, Interactive, EventResult, FocusStyle,
        Stack, Direction,
        Text, Alignment,
        Border, BorderType,
        Input,
        List,
        Progress, ProgressStyle,
        Spinner, SpinnerStyle,
        Table, Column,
        Select,
        ThemePicker,
        Modal, ModalButton, ModalButtonStyle,
        Tabs, Tab,
        Tree, TreeNode,
        ScrollView,
        Markdown,
        Image, ScaleMode,
        // New widgets
        Button, ButtonVariant,
        Checkbox, CheckboxStyle,
        RadioGroup, RadioStyle, RadioLayout,
        Sparkline, SparklineStyle,
        // Layer system
        Layers,
        Positioned, Anchor,
        Canvas, DrawContext,
        BarChart, BarOrientation,
        // Braille canvas
        BrailleCanvas, BrailleContext, BrailleGrid,
        Shape, Line, Circle, FilledCircle, Rectangle, FilledRectangle, Points,
        // UX widgets
        Toast, ToastLevel, ToastPosition,
        TextArea,
        // Command palette
        CommandPalette, Command,
        // Convenience widgets
        Divider, Orientation, DividerStyle,
        Badge, BadgeVariant, BadgeShape,
        Avatar, AvatarSize, AvatarShape,
        Tag, TagStyle,
        Skeleton, SkeletonShape,
        Pagination, PaginationStyle,
    };

    // Widgets - Constructors
    pub use crate::widget::{
        vstack, hstack,
        text,
        border,
        input,
        list,
        progress,
        spinner,
        table, column,
        select,
        theme_picker,
        modal,
        tabs,
        tree, tree_node,
        scroll_view,
        markdown,
        image_from_file,
        // New constructors
        button,
        checkbox,
        radio_group,
        sparkline,
        // Layer system constructors
        layers,
        positioned,
        canvas,
        barchart,
        braille_canvas,
        // UX constructors
        toast,
        textarea,
        // Command palette
        command_palette,
        // Convenience widget constructors
        divider, vdivider,
        badge, dot_badge,
        avatar, avatar_icon,
        tag, chip,
        skeleton, skeleton_text, skeleton_avatar, skeleton_paragraph,
        pagination,
    };

    // DOM system
    pub use crate::dom::{
        DomRenderer, DomTree, DomNode, DomId, WidgetMeta, NodeState, Query,
    };

    // Worker system
    pub use crate::worker::{
        WorkerPool, WorkerHandle, WorkerState, WorkerChannel, WorkerMessage,
        run_blocking, spawn as spawn_worker,
    };

    // Tasks - Timer, TaskRunner, EventBus
    pub use crate::tasks::{
        Timer, TimerId, TimerEntry,
        TaskRunner, TaskId, TaskResult,
        EventBus, EventId, Subscription,
    };

    // Patterns - Common TUI patterns
    pub use crate::patterns::{
        // Colors
        CYAN, GREEN, YELLOW, RED, BLUE, PURPLE, ORANGE,
        FG, FG_DIM, FG_SUBTLE,
        BG, BG_SUBTLE, BG_INSET,
        BORDER, BORDER_MUTED,
        SUCCESS, ERROR, WARNING, INFO,
        status_color, build_color, priority_color,
        // State management
        MessageState,
        ConfirmAction, ConfirmState,
        // Async operations
        AsyncTask, spinner_char, SPINNER_FRAMES,
        // Config loading
        AppConfig, ConfigError,
        // Search/filter
        SearchState, SearchMode,
        // Form validation
        FormState, FormField, FieldType, ValidationError, Validators,
        // Navigation
        NavigationState, Route, NavigationEvent, BreadcrumbItem,
    };

    // Accessibility
    pub use crate::utils::{
        // Announcement functions
        announce, announce_now, take_announcements, has_announcements,
        // Preference getters/setters
        prefers_reduced_motion, set_reduced_motion,
        is_high_contrast, set_high_contrast,
        // Widget-specific announcement helpers
        announce_button_clicked, announce_checkbox_changed,
        announce_list_selection, announce_tab_changed,
        announce_error, announce_success,
        announce_dialog_opened, announce_dialog_closed,
    };

    // Testing (Pilot)
    pub use crate::testing::{Pilot, TestApp, TestConfig};
    // Visual regression testing
    pub use crate::testing::{
        VisualTest, VisualTestConfig, VisualTestResult,
        VisualCapture, VisualDiff, CapturedCell,
        CiEnvironment, CiProvider, TestReport,
    };

    // DevTools
    pub use crate::devtools::{
        DevTools, DevToolsConfig, DevToolsPosition, DevToolsTab,
        Inspector, WidgetNode, InspectorConfig,
        StateDebugger, StateEntry, StateValue,
        StyleInspector, ComputedProperty, PropertySource, StyleCategory,
        EventLogger, LoggedEvent, EventType, EventFilter,
        enable_devtools, disable_devtools, is_devtools_enabled, toggle_devtools,
    };

    // Profiler
    pub use crate::utils::profiler::{
        Profiler, ProfileGuard, Stats, Timing, FlameNode,
        profile, start_profile, profiler_report,
    };

    // Result type
    pub use crate::Result;

    // Constants
    pub use crate::constants::{
        // Frame rates
        FRAME_DURATION_60FPS, FRAME_DURATION_30FPS,
        // Animation durations
        ANIMATION_DEFAULT_DURATION, ANIMATION_FAST_DURATION,
        ANIMATION_SLOW_DURATION, ANIMATION_VERY_SLOW_DURATION,
        // Debounce
        DEBOUNCE_DEFAULT, DEBOUNCE_SEARCH, DEBOUNCE_FILE_SYSTEM,
        // Tick rates
        TICK_RATE_DEFAULT, POLL_IMMEDIATE,
        // Screen transitions
        SCREEN_TRANSITION_DURATION,
        // Stagger
        STAGGER_DELAY_DEFAULT,
        // Messages
        MESSAGE_DEFAULT_DURATION, MESSAGE_QUICK_DURATION, MESSAGE_LONG_DURATION,
    };
}
