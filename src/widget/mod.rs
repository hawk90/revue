//! Widget system for Revue TUI framework.
//!
//! This module provides 92+ widgets for building terminal user interfaces.
//! Widgets are organized into categories for easy discovery.
//!
//! # Widget Categories
//!
//! ## Layout Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Stack`] | Vertical/horizontal layout | [`vstack()`], [`hstack()`] |
//! | [`Border`] | Bordered container | [`border()`] |
//! | [`Tabs`] | Tab navigation | [`tabs()`] |
//! | [`ScrollView`] | Scrollable area | [`scroll_view()`] |
//! | [`Layers`] | Overlapping widgets | [`layers()`] |
//! | [`Positioned`] | Absolute positioning | [`positioned()`] |
//! | [`Grid`] | CSS Grid layout | [`grid()`] |
//! | [`Splitter`] | Resizable panes | [`hsplit()`], [`vsplit()`] |
//! | [`Accordion`] | Collapsible sections | [`accordion()`] |
//! | [`Collapsible`] | Single expandable section | [`collapsible()`] |
//! | [`Card`] | Content container | [`card()`] |
//! | [`Screen`] | Screen navigation | [`screen()`] |
//! | [`Sidebar`] | Vertical nav rail | [`sidebar()`] |
//!
//! ## Input Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Input`] | Single-line text input | [`input()`] |
//! | [`TextArea`] | Multi-line editor | [`textarea()`] |
//! | [`Button`] | Clickable button | [`button()`] |
//! | [`Checkbox`] | Toggle checkbox | [`checkbox()`] |
//! | [`RadioGroup`] | Radio selection | [`radio_group()`] |
//! | [`Select`] | Dropdown menu | [`select()`] |
//! | [`Combobox`] | Autocomplete dropdown | [`combobox()`] |
//! | [`Switch`] | Toggle switch | [`switch()`] |
//! | [`Slider`] | Value slider | [`slider()`] |
//! | [`ColorPicker`] | Color selection | [`color_picker()`] |
//! | [`CommandPalette`] | Fuzzy command search | [`command_palette()`] |
//!
//! ## Display Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Text`] | Styled text | [`text()`] |
//! | [`RichText`] | Markup text | [`rich_text()`] |
//! | [`Progress`] | Progress bar | [`progress()`] |
//! | [`Spinner`] | Loading indicator | [`spinner()`] |
//! | [`Gauge`] | Circular gauge | [`gauge()`] |
//! | [`Badge`] | Status badge | [`badge()`] |
//! | [`Tag`] | Label tag | [`tag()`] |
//! | [`Avatar`] | User avatar | [`avatar()`] |
//! | [`GradientBox`] | Gradient background | [`gradient_box()`] |
//! | [`Skeleton`] | Loading placeholder | [`skeleton()`] |
//! | [`Divider`] | Visual separator | [`divider()`] |
//! | [`Tooltip`] | Hover tooltip | [`tooltip()`] |
//! | [`Breadcrumb`] | Navigation trail | [`breadcrumb()`] |
//! | [`Stepper`] | Step indicator | [`stepper()`] |
//! | [`Alert`] | Persistent feedback | [`alert()`] |
//! | [`Callout`] | Info highlight block | [`callout()`], [`note()`], [`tip()`] |
//! | [`EmptyState`] | No-data placeholder | [`empty_state()`] |
//! | [`StatusIndicator`] | Availability status | [`status_indicator()`], [`online()`] |
//!
//! ## Data Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Table`] | Data table | [`table()`] |
//! | [`DataGrid`] | Advanced grid | [`datagrid()`] |
//! | [`List`] | Selectable list | [`list()`] |
//! | [`Tree`] | Hierarchical tree | [`tree()`] |
//! | [`FileTree`] | File browser | [`file_tree()`] |
//! | [`Calendar`] | Date picker | [`calendar()`] |
//! | [`DateTimePicker`] | Date/time picker | [`datetime_picker()`] |
//! | [`Timeline`] | Event timeline | [`timeline()`] |
//! | [`RichLog`] | Log viewer | [`richlog()`] |
//! | [`LogViewer`] | Advanced log viewer | [`log_viewer()`] |
//! | [`CsvViewer`] | CSV/TSV data viewer | [`csv_viewer()`] |
//! | [`JsonViewer`] | JSON tree viewer | [`json_viewer()`] |
//!
//! ## Chart Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Sparkline`] | Mini line chart | [`sparkline()`] |
//! | [`BarChart`] | Bar chart | [`barchart()`] |
//! | [`Chart`] | Full charts | [`chart()`], [`line_chart()`] |
//! | [`Canvas`] | Custom drawing | [`canvas()`] |
//! | [`BrailleCanvas`] | High-res drawing | [`braille_canvas()`] |
//!
//! ## Feedback Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Modal`] | Dialog overlay | [`modal()`] |
//! | [`Toast`] | Notification popup | [`toast()`] |
//! | [`NotificationCenter`] | Notification manager | [`notification_center()`] |
//! | [`Menu`] | Dropdown menu | [`menu()`] |
//! | [`ContextMenu`] | Right-click menu | [`context_menu()`] |
//! | [`Popover`] | Anchor-positioned overlay | [`popover()`] |
//! | [`StatusBar`] | App status bar | [`statusbar()`] |
//!
//! ## Developer Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`AiStream`] | AI streaming text | [`ai_stream()`] |
//! | [`Presentation`] | Terminal slideshows | [`presentation()`] |
//! | [`Diagram`] | Mermaid diagrams | [`diagram()`] |
//! | [`VimState`] | Vim mode support | [`vim_state()`] |
//! | [`HttpClient`] | REST API client | [`http_client()`] |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Simple layout
//! let ui = vstack()
//!     .gap(1)
//!     .child(Text::new("Hello, Revue!").bold())
//!     .child(button("Click me").on_click(|| println!("Clicked!")));
//!
//! // With borders
//! let panel = Border::rounded()
//!     .title("My Panel")
//!     .child(ui);
//! ```
//!
//! # Constructor Pattern
//!
//! Revue uses a **hybrid constructor pattern** to balance ergonomics and clarity:
//!
//! ## Function-Style (Simple Widgets)
//!
//! Widgets with fewer than 5 configuration options use function-style constructors:
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Simple, single-purpose widgets
//! let btn = button("Click me");
//! let chk = checkbox("Enable");
//! let txt = text("Hello");
//! let badge = badge("New");
//! let divider = divider();
//! ```
//!
//! **Benefits**: Concise, chainable, IDE-friendly autocomplete.
//!
//! ## Builder-Style (Complex Widgets)
//!
//! Widgets with 5+ configuration options use builder-style:
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! // Complex widgets with many options
//! let grid = DataGrid::builder()
//!     .columns(vec![/* ... */])
//!     .data(&data)
//!     .selectable(true)
//!     .sortable(true)
//!     .build();
//! ```
//!
//! **Benefits**: Clear option names, handles complex configuration well.
//!
//! ## Creating New Widgets
//!
//! When adding new widgets, follow this guideline:
//!
//! | Config Options | Pattern | Example |
//! |----------------|---------|---------|
//! | < 5 | Function-style | `button()`, `text()` |
//! | >= 5 | Builder-style | `DataGrid::builder()` |
//!
//! ## Transition Period
//!
//! Some widgets may have both patterns during migration. Prefer function-style
//! for simple widgets, but `Widget::new()` remains valid for all widgets.
//!
//! # Creating Custom Widgets
//!
//! Implement the [`View`] trait to create custom widgets:
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! struct MyWidget {
//!     label: String,
//! }
//!
//! impl View for MyWidget {
//!     fn render(&self, ctx: &mut RenderContext) {
//!         Text::new(&self.label)
//!             .fg(Color::CYAN)
//!             .render(ctx);
//!     }
//! }
//! ```
//!
//! # Styling
//!
//! Most widgets support styling via builder methods:
//!
//! ```rust,ignore
//! Text::new("Styled")
//!     .fg(Color::CYAN)
//!     .bg(Color::rgb(30, 30, 46))
//!     .bold()
//!     .italic();
//!
//! Progress::new(0.75)
//!     .filled_color(Color::GREEN)
//!     .show_percentage(true);
//!
//! Border::rounded()
//!     .title("Panel")
//!     .fg(Color::BLUE);
//! ```

mod breadcrumb;
// Note: calendar moved to data/calendar
mod callout;
mod canvas;
mod command_palette;
pub mod data; // Data widgets (calendar, chart, table, timer, etc.)
mod datetime_picker;
mod debug_overlay;
mod developer;
mod display;
mod dropzone;
mod feedback;
mod filepicker;
mod form;
#[cfg(feature = "image")]
mod image;
// Input widgets (moved to input/input_widgets/)
#[path = "input/input_widgets/mod.rs"]
pub mod input_widgets;
mod layout;
mod link;
#[cfg(feature = "markdown")]
pub mod markdown;
#[cfg(feature = "markdown")]
mod markdown_presentation;
mod mermaid;
mod multi_select;
mod option_list;
mod pagination;
#[cfg(feature = "qrcode")]
mod qrcode;
mod range_picker;
#[cfg(feature = "markdown")]
pub mod slides;
mod sortable;
mod streamline;
pub mod syntax; // Syntax highlighting module
mod theme_picker;
// Note: timer moved to data/timer
pub mod traits;
mod transition;
pub mod validation;
mod zen;
#[macro_use]
mod macros;

pub use crate::utils::border::BorderChars;
// Display widgets (re-exported from display module)
pub use display::{
    avatar, avatar_icon, away_indicator, badge, battery, bigtext, busy_indicator, chip, clock,
    digits, divider, dot_badge, empty_error, empty_state, first_use, gauge, gradient_box, h1, h2,
    h3, log_entry, markup, no_results, offline, online, percentage, progress, rich_text, richlog,
    skeleton, skeleton_avatar, skeleton_paragraph, skeleton_text, span, spinner, status_indicator,
    style, tag, text, vdivider, Alignment, Avatar, AvatarShape, AvatarSize, Badge, BadgeShape,
    BadgeVariant, BigText, DigitStyle, Digits, Divider, DividerStyle, EmptyState, EmptyStateType,
    EmptyStateVariant, Gauge, GaugeStyle, GradientBox, LabelPosition, LogEntry, LogFormat,
    LogLevel, Orientation, Progress, ProgressStyle, RichLog, RichText, Skeleton, SkeletonShape,
    Span, Spinner, SpinnerStyle, Status, StatusIndicator, StatusSize, StatusStyle, Style, Tag,
    TagStyle, Text,
};
// Layout widgets (re-exported from layout module)
pub use layout::{
    accordion, border, card, collapsible, draw_border, grid, grid_item, grid_template, hsplit,
    hstack, layer, layers, pane, positioned, resizable, screen, screen_stack, scroll, scroll_view,
    section, sidebar, sidebar_item, sidebar_section, sidebar_section_titled, splitter, stack, tabs,
    vsplit, vstack, Accordion, AccordionSection, Anchor, Border, BorderType, Card, CardVariant,
    CollapseMode, Collapsible, Direction, FlattenedItem, Grid, GridAlign, GridItem, GridPlacement,
    HSplit, Layers, Pane, Positioned, Resizable, ResizeDirection, ResizeHandle, ResizeStyle,
    Screen, ScreenStack, ScreenTransition, ScrollView, Sidebar, SidebarItem, SidebarSection,
    SplitOrientation, Splitter, Stack, Tab, Tabs, TrackSize, VSplit,
};
// Input widgets (re-exported from input_widgets module)
pub use breadcrumb::{breadcrumb, crumb, Breadcrumb, BreadcrumbItem, SeparatorStyle};
pub use callout::{
    callout, danger, important, info_callout, note, tip, warning_callout, Callout, CalloutType,
    CalloutVariant,
};
pub use canvas::{
    braille_canvas, canvas, Arc, BrailleCanvas, BrailleContext, BrailleGrid, Canvas, Circle,
    ClipRegion, DrawContext, FilledCircle, FilledPolygon, FilledRectangle, Layer, Line, Points,
    Polygon, Rectangle, Shape, Transform,
};
pub use command_palette::{command_palette, Command, CommandPalette};
pub use data::calendar::{
    calendar, days_in_month, Calendar, CalendarMode, Date, DateMarker, FirstDayOfWeek,
};
pub use datetime_picker::{
    date_picker, datetime_picker, time_picker, DateTime, DateTimeFormat, DateTimeMode,
    DateTimePicker, Time, TimeField,
};
pub use debug_overlay::{
    disable_debug, enable_debug, is_debug_enabled, toggle_debug, DebugConfig, DebugEvent,
    DebugOverlay, DebugPosition, EventLog, PerfMetrics, WidgetInfo,
};
pub use dropzone::{drop_zone, DropZone, DropZoneStyle};
pub use filepicker::{
    dir_picker, file_picker, save_picker, FileFilter, FilePicker, PickerEntry, PickerMode,
    PickerResult,
};
pub use input_widgets::{
    autocomplete, button, checkbox, color_picker, combobox, currency_input, input, integer_input,
    number_input, percentage_input, percentage_slider, radio_group, rating, search_bar, select,
    selection_item, selection_list, slider, slider_range, step, stepper, switch, textarea, toggle,
    volume_slider, Autocomplete, Button, ButtonVariant, Checkbox, CheckboxStyle, ColorPalette,
    ColorPicker, ColorPickerMode, ComboOption, Combobox, Input, NumberInput, RadioGroup,
    RadioLayout, RadioStyle, Rating, RatingSize, RatingStyle, SearchBar, Select, SelectionItem,
    SelectionList, SelectionStyle, Slider, SliderOrientation, SliderStyle, Step, StepStatus,
    Stepper, StepperOrientation, StepperStyle, Suggestion, Switch, SwitchStyle, TextArea,
};
// Form widgets (re-exported from form module)
#[cfg(feature = "qrcode")]
pub use self::qrcode::{qrcode, qrcode_url, ErrorCorrection, QrCodeWidget, QrStyle};
pub use data::timer::{pomodoro, timer as timer_widget, Timer, TimerFormat, TimerState};
pub use data::timer::{stopwatch, Stopwatch};
pub use form::{
    credit_card_input, form as form_widget, form_field, masked_input, password_input, pin_input,
    rich_text_editor, Block, BlockType, EditorViewMode, ErrorDisplayStyle, Form, FormField,
    FormFieldWidget, FormattedSpan, ImageRef, InputType, MarkdownLink, MaskStyle, MaskedInput,
    RichTextEditor, TextFormat, ToolbarAction, ValidationState,
};
#[cfg(feature = "image")]
pub use image::{image_from_file, try_image_from_file, Image, ImageError, ImageResult, ScaleMode};
pub use link::{link, url_link, Link, LinkStyle};
#[cfg(feature = "markdown")]
pub use markdown::{markdown, Markdown};
#[cfg(feature = "markdown")]
pub use markdown_presentation::{markdown_presentation, MarkdownPresentation, ViewMode};
pub use mermaid::{
    diagram, edge, flowchart, node, ArrowStyle, Diagram, DiagramEdge, DiagramNode, DiagramType,
    NodeShape,
};
pub use multi_select::{multi_select, multi_select_from, MultiSelect, MultiSelectOption};
pub use option_list::{
    option_item, option_list, OptionEntry, OptionItem, OptionList,
    SeparatorStyle as OptionSeparatorStyle,
};
pub use pagination::{pagination, Pagination, PaginationStyle};
pub use range_picker::{
    analytics_range_picker, date_range_picker, range_picker, PresetRange, RangeFocus, RangePicker,
};
#[cfg(feature = "markdown")]
pub use slides::{parse_slides, SlideContent, SlideNav};
pub use sortable::{sortable_list, SortableItem, SortableList};
pub use streamline::{
    genre_stream, resource_stream, streamline, streamline_with_data, traffic_stream,
    StreamBaseline, StreamLayer, StreamOrder, Streamline,
};
pub use syntax::{HighlightSpan, Language, SyntaxHighlighter, SyntaxTheme};
pub use theme_picker::{theme_picker, ThemePicker};
pub use traits::{
    Draggable, Element, EventResult, FocusStyle, Interactive, RenderContext, StyledView, Timeout,
    View, WidgetProps, WidgetState, DISABLED_BG, DISABLED_FG,
};
pub use transition::{
    transition, transition_group, Animation, AnimationPreset, Transition as AnimationTransition,
    TransitionGroup, TransitionPhase,
};
pub use validation::{validators, Validatable, ValidationError, ValidationResult};
pub use zen::{zen, zen_dark, zen_light, ZenMode};

// Chart widgets (re-exported from chart module)
pub use data::chart::{
    area_wave, audio_waveform, barchart, boxplot, bubble_chart, candle_chart, chart,
    contribution_map, cpu_chart, donut_chart, heatmap, histogram, line_chart, memory_chart,
    network_chart, ohlc_chart, pie_chart, piechart, sawtooth_wave, scatter_chart, scatter_plot,
    scatterchart, signal_wave, sine_wave, sparkline, spectrum, square_wave, time_series,
    time_series_with_data, waveline, BarChart, BarOrientation, BinConfig, BoxGroup, BoxPlot,
    BoxStats, Candle, CandleChart, CandleStyle, CellDisplay, Chart, ChartType, ColorScale, HeatMap,
    Histogram, HistogramBin, Interpolation, LineStyle, MarkerStyle, PieChart, PieLabelStyle,
    PieSlice, PieStyle, ScatterChart, ScatterSeries, Series, Sparkline, SparklineStyle, TimeFormat,
    TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeries, TimeSeriesData, WaveStyle,
    Waveline, WhiskerStyle,
};

// Data widgets (re-exported from data module)
pub use data::{
    adv_log_entry, column, csv_viewer, datagrid, dir_entry, file_entry, file_tree, grid_column,
    grid_row, json_viewer, list, log_filter, log_parser, log_viewer, table, timeline,
    timeline_event, tree, tree_node, virtual_list, AdvLogEntry, AdvLogLevel, Column, CsvSortOrder,
    CsvViewer, DataGrid, Delimiter, EventType, FileEntry, FileTree, FileType, GridColumn, GridRow,
    JsonNode, JsonType, JsonViewer, List, LogFilter, LogParser, LogViewer, ScrollAlignment,
    ScrollMode, Search, SearchMatch, SortDirection, Table, Timeline, TimelineEvent,
    TimelineOrientation, TimelineStyle, TimestampFormat, Tree, TreeNode, VirtualList,
};

// Feedback widgets (re-exported from feedback module)
pub use feedback::{
    alert, context_menu, error_alert, error_boundary, footer, header, info_alert, key_hint, menu,
    menu_bar, menu_item, modal, notification_center, popover, status_section, statusbar,
    success_alert, toast, toast_queue, tooltip, warning_alert, Alert, AlertLevel, AlertVariant,
    ContextMenu, ErrorBoundary, KeyHint, Menu, MenuBar, MenuItem, Modal, ModalButton,
    ModalButtonStyle, Notification, NotificationCenter, NotificationLevel, NotificationPosition,
    Popover, PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger, SectionAlign,
    StackDirection, StatusBar, StatusSection, Toast, ToastEntry, ToastLevel, ToastPosition,
    ToastPriority, ToastQueue, Tooltip, TooltipArrow, TooltipPosition, TooltipStyle,
};

// Developer widgets (re-exported from developer module)
#[cfg(feature = "syntax-highlighting")]
pub use developer::TreeSitterHighlighter;
pub use developer::{
    ai_response, ai_stream, code_editor, http_client, http_delete, http_get, http_patch, http_post,
    http_put, presentation, slide, terminal, vim_state, AiStream, BracketMatch, BracketPair,
    CodeEditor, ContentType, CursorStyle, EditorConfig, HttpBackend, HttpClient, HttpMethod,
    HttpRequest, HttpResponse, IndentStyle, MockHttpBackend, Presentation, RequestBuilder,
    RequestState, ResponseView, Slide, SlideAlign, StreamCursor, StreamStatus, TermCell, TermLine,
    Terminal, TerminalAction, Transition, TypingStyle, VimAction, VimCommandResult, VimMode,
    VimMotion, VimState,
};
#[cfg(feature = "diff")]
pub use developer::{diff, diff_viewer, ChangeType, DiffColors, DiffLine, DiffMode, DiffViewer};
#[cfg(feature = "sysinfo")]
pub use developer::{
    htop, process_monitor, ProcColors, ProcessInfo, ProcessMonitor, ProcessSort, ProcessView,
};

// Re-export common widget constructors

/// Create a new text widget
pub fn text(content: impl Into<String>) -> Text {
    Text::new(content)
}
