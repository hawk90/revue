// The items of `revue::prelude`. Its docs are on `pub mod prelude;` in lib.rs.

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
    cycle_theme, register_theme, set_theme, set_theme_by_id, theme_ids, toggle_theme, use_theme,
    Theme, ThemeVariant, Themes,
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
    // CSS @keyframes parser types
    KeyframeBlock,
    KeyframesDefinition,
    // Choreography
    Stagger,
    Tween,
};

// Widgets - Types
pub use crate::widget::Fill;
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
    BigText,
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
    // Layout containers that `widget::` exported but the prelude did not
    Card,
    CardVariant,
    Checkbox,
    CheckboxStyle,
    Circle,
    Column,
    Command,
    // Command palette
    CommandPalette,
    DigitStyle,
    Digits,
    Direction,
    // Convenience widgets
    Divider,
    DividerStyle,
    DrawContext,
    EmptyState,
    EmptyStateType,
    EmptyStateVariant,
    EventResult,
    FilledCircle,
    FilledRectangle,
    FocusStyle,
    Gauge,
    GaugeStyle,
    Input,
    Interactive,
    LabelPosition,
    // Layer system
    Layers,
    Line,
    List,
    LogEntry,
    LogFormat,
    LogLevel,
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
    RichLog,
    RichText,
    ScrollView,
    Select,
    Shape,
    Skeleton,
    SkeletonShape,
    Span,
    Sparkline,
    SparklineStyle,
    Spinner,
    SpinnerStyle,
    Stack,
    Status,
    StatusIndicator,
    StatusSize,
    StatusStyle,
    Style,
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
    away_indicator,
    badge,
    barchart,
    battery,
    bigtext,
    border,
    braille_canvas,
    busy_indicator,
    // New constructors
    button,
    canvas,
    checkbox,
    chip,
    clock,
    column,
    // Command palette
    command_palette,
    digits,
    // Convenience widget constructors
    divider,
    dot_badge,
    empty_error,
    empty_state,
    first_use,
    gauge,
    h1,
    h2,
    h3,
    hstack,
    input,
    // Layer system constructors
    layers,
    list,
    log_entry,
    markup,
    modal,
    no_results,
    offline,
    online,
    pagination,
    percentage,
    positioned,
    progress,
    radio_group,
    rich_text,
    richlog,
    scroll_view,
    select,
    skeleton,
    skeleton_avatar,
    skeleton_paragraph,
    skeleton_text,
    span,
    sparkline,
    spinner,
    status_indicator,
    style,
    table,
    tabs,
    tag,
    text,
    textarea,
    theme_picker,
    timer_widget as timer,
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

// Render system
pub use crate::runtime::render::{Buffer, Modifier};

// Worker system
pub use crate::worker::{
    run_blocking, spawn as spawn_worker, WorkerChannel, WorkerHandle, WorkerMessage, WorkerPool,
    WorkerState,
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

// Figlet fonts (for BigText widget)
pub use crate::utils::figlet::FigletFont;

// Testing (Pilot)
pub use crate::testing::{Pilot, TestApp, TestConfig};
// Visual regression testing
pub use crate::testing::{
    CapturedCell, CiEnvironment, CiProvider, TestReport, VisualCapture, VisualDiff, VisualTest,
    VisualTestConfig, VisualTestResult,
};

// DevTools
pub use crate::devtools::{
    ComputedProperty, DevTools, DevToolsConfig, DevToolsPosition, DevToolsTab, EventFilter,
    EventLogger, EventType, Inspector, InspectorConfig, LoggedEvent, PropertySource, StateDebugger,
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
