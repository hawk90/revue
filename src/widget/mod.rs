//! Widget system for Revue TUI framework.
//!
//! This module provides 85+ widgets for building terminal user interfaces.
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
//! | [`Markdown`] | Markdown renderer | [`markdown()`] |
//! | [`Image`] | Terminal images | [`image_from_file()`] |
//! | [`Progress`] | Progress bar | [`progress()`] |
//! | [`Spinner`] | Loading indicator | [`spinner()`] |
//! | [`Gauge`] | Circular gauge | [`gauge()`] |
//! | [`Badge`] | Status badge | [`badge()`] |
//! | [`Tag`] | Label tag | [`tag()`] |
//! | [`Avatar`] | User avatar | [`avatar()`] |
//! | [`Skeleton`] | Loading placeholder | [`skeleton()`] |
//! | [`Divider`] | Visual separator | [`divider()`] |
//! | [`Tooltip`] | Hover tooltip | [`tooltip()`] |
//! | [`Breadcrumb`] | Navigation trail | [`breadcrumb()`] |
//! | [`Stepper`] | Step indicator | [`stepper()`] |
//! | [`Alert`] | Persistent feedback | [`alert()`] |
//! | [`Callout`] | Info highlight block | [`callout()`], [`note()`], [`tip()`] |
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
//! | [`StatusBar`] | App status bar | [`statusbar()`] |
//!
//! ## Developer Widgets
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`QrCodeWidget`] | QR code generator | [`qrcode()`] |
//! | [`DiffViewer`] | Side-by-side diff | [`diff_viewer()`] |
//! | [`AiStream`] | AI streaming text | [`ai_stream()`] |
//! | [`Presentation`] | Terminal slideshows | [`presentation()`] |
//! | [`ProcessMonitor`] | htop-style monitor | [`process_monitor()`] |
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

mod accordion;
mod aistream;
mod alert;
mod autocomplete;
mod avatar;
mod badge;
mod barchart;
mod bigtext;
mod border;
mod boxplot;
mod breadcrumb;
mod button;
mod calendar;
mod callout;
mod candlechart;
mod canvas;
mod card;
mod chart;
mod chart_common;
mod checkbox;
mod collapsible;
mod color_picker;
mod command_palette;
mod datagrid;
mod datetime_picker;
mod debug_overlay;
#[cfg(feature = "diff")]
mod diff;
mod digits;
mod divider;
mod dropzone;
mod filepicker;
mod filetree;
mod gauge;
mod grid;
mod heatmap;
mod histogram;
mod httpclient;
#[cfg(feature = "image")]
mod image;
mod input;
mod layer;
mod link;
mod list;
#[cfg(feature = "markdown")]
mod markdown;
#[cfg(feature = "markdown")]
mod markdown_presentation;
mod masked_input;
mod menu;
mod mermaid;
mod modal;
mod multi_select;
mod notification;
mod number_input;
mod option_list;
mod pagination;
mod piechart;
mod positioned;
mod presentation;
#[cfg(feature = "sysinfo")]
mod procmon;
mod progress;
#[cfg(feature = "qrcode")]
mod qrcode;
mod radio;
mod range_picker;
mod rating;
mod resizable;
mod richlog;
mod richtext;
mod scatterchart;
mod screen;
mod scroll;
mod search_bar;
mod select;
mod selection_list;
mod skeleton;
mod slider;
#[cfg(feature = "markdown")]
pub mod slides;
mod sortable;
mod sparkline;
mod spinner;
mod splitter;
mod stack;
mod status_indicator;
mod statusbar;
mod stepper;
mod streamline;
mod switch;
mod syntax;
mod table;
mod tabs;
mod tag;
mod terminal;
mod text;
mod textarea;
mod theme_picker;
mod timeline;
mod timer;
mod timeseries;
mod toast;
mod tooltip;
pub mod traits;
mod tree;
mod vim;
mod virtuallist;
mod waveline;
mod zen;
#[macro_use]
mod macros;

pub use accordion::{accordion, section, Accordion, AccordionSection};
pub use aistream::{ai_response, ai_stream, AiStream, StreamCursor, StreamStatus, TypingStyle};
pub use alert::{
    alert, error_alert, info_alert, success_alert, warning_alert, Alert, AlertLevel, AlertVariant,
};
pub use autocomplete::{autocomplete, Autocomplete, FilterMode, Suggestion};
pub use avatar::{avatar, avatar_icon, Avatar, AvatarShape, AvatarSize};
pub use badge::{badge, dot_badge, Badge, BadgeShape, BadgeVariant};
pub use barchart::{barchart, BarChart, BarOrientation};
pub use bigtext::{bigtext, h1, h2, h3, BigText};
pub use border::{border, Border, BorderType};
pub use boxplot::{boxplot, BoxGroup, BoxPlot, BoxStats, WhiskerStyle};
pub use breadcrumb::{breadcrumb, crumb, Breadcrumb, BreadcrumbItem, SeparatorStyle};
pub use button::{button, Button, ButtonVariant};
pub use calendar::{
    calendar, days_in_month, Calendar, CalendarMode, Date, DateMarker, FirstDayOfWeek,
};
pub use callout::{
    callout, danger, important, info_callout, note, tip, warning_callout, Callout, CalloutType,
    CalloutVariant,
};
pub use candlechart::{candle_chart, ohlc_chart, Candle, CandleChart, ChartStyle as CandleStyle};
pub use canvas::{
    braille_canvas, canvas, BrailleCanvas, BrailleContext, BrailleGrid, Canvas, Circle,
    DrawContext, FilledCircle, FilledRectangle, Line, Points, Rectangle, Shape,
};
pub use card::{card, Card, CardBorder, CardVariant};
pub use chart::{chart, line_chart, scatter_plot, Chart, ChartType, LineStyle, Series};
pub use chart_common::{
    Axis, AxisFormat, ChartGrid, ChartOrientation, ChartTooltip, ChartTooltipFormat,
    ChartTooltipPosition, ColorScheme, GridStyle, Legend, LegendOrientation, LegendPosition,
    Marker,
};
pub use checkbox::{checkbox, Checkbox, CheckboxStyle};
pub use collapsible::{collapsible, Collapsible};
pub use color_picker::{color_picker, ColorPalette, ColorPicker, ColorPickerMode};
pub use command_palette::{command_palette, Command, CommandPalette};
pub use datagrid::{datagrid, grid_column, grid_row, DataGrid, GridColumn, GridRow, SortDirection};
pub use datetime_picker::{
    date_picker, datetime_picker, time_picker, DateTime, DateTimeFormat, DateTimeMode,
    DateTimePicker, Time, TimeField,
};
pub use debug_overlay::{
    disable_debug, enable_debug, is_debug_enabled, toggle_debug, DebugConfig, DebugEvent,
    DebugOverlay, DebugPosition, EventLog, PerfMetrics, WidgetInfo,
};
#[cfg(feature = "diff")]
pub use diff::{diff, diff_viewer, ChangeType, DiffColors, DiffLine, DiffMode, DiffViewer};
pub use digits::{clock, digits, timer, DigitStyle, Digits};
pub use divider::{divider, vdivider, Divider, DividerStyle, Orientation};
pub use dropzone::{drop_zone, DropZone, DropZoneStyle};
pub use filepicker::{
    dir_picker, file_picker, save_picker, FileFilter, FilePicker, PickerEntry, PickerMode,
    PickerResult,
};
pub use filetree::{dir_entry, file_entry, file_tree, FileEntry, FileTree, FileType};
pub use gauge::{battery, gauge, percentage, Gauge, GaugeStyle, LabelPosition};
pub use grid::{
    grid, grid_item, grid_template, Grid, GridAlign, GridItem, GridPlacement, TrackSize,
};
pub use heatmap::{contribution_map, heatmap, CellDisplay, ColorScale, HeatMap};
pub use histogram::{histogram, BinConfig, Histogram, HistogramBin};
pub use httpclient::{
    get as http_get, http_client, post as http_post, HttpClient, HttpMethod, HttpRequest,
    HttpResponse, RequestState, ResponseView,
};
#[cfg(feature = "image")]
pub use image::{image_from_file, try_image_from_file, Image, ImageError, ImageResult, ScaleMode};
pub use input::{input, Input};
pub use layer::{layers, Layers};
pub use link::{link, url_link, Link, LinkStyle};
pub use list::{list, List};
#[cfg(feature = "markdown")]
pub use markdown::{markdown, Markdown};
#[cfg(feature = "markdown")]
pub use markdown_presentation::{markdown_presentation, MarkdownPresentation, ViewMode};
pub use masked_input::{
    credit_card_input, masked_input, password_input, pin_input, MaskStyle, MaskedInput,
    ValidationState,
};
pub use menu::{context_menu, menu, menu_bar, menu_item, ContextMenu, Menu, MenuBar, MenuItem};
pub use mermaid::{
    diagram, edge, flowchart, node, ArrowStyle, Diagram, DiagramEdge, DiagramNode, DiagramType,
    NodeShape,
};
pub use modal::{modal, Modal, ModalButton, ModalButtonStyle};
pub use multi_select::{multi_select, multi_select_from, MultiSelect, MultiSelectOption};
pub use notification::{
    notification_center, Notification, NotificationCenter, NotificationLevel, NotificationPosition,
};
pub use number_input::{
    currency_input, integer_input, number_input, percentage_input, NumberInput,
};
pub use option_list::{
    option_item, option_list, OptionEntry, OptionItem, OptionList,
    SeparatorStyle as OptionSeparatorStyle,
};
pub use pagination::{pagination, Pagination, PaginationStyle};
pub use piechart::{donut_chart, pie_chart, PieChart, PieLabelStyle, PieSlice, PieStyle};
pub use positioned::{positioned, Anchor, Positioned};
pub use presentation::{presentation, slide, Presentation, Slide, SlideAlign, Transition};
#[cfg(feature = "sysinfo")]
pub use procmon::{
    htop, process_monitor, ProcColors, ProcessInfo, ProcessMonitor, ProcessSort, ProcessView,
};
pub use progress::{progress, Progress, ProgressStyle};
#[cfg(feature = "qrcode")]
pub use qrcode::{qrcode, qrcode_url, ErrorCorrection, QrCodeWidget, QrStyle};
pub use radio::{radio_group, RadioGroup, RadioLayout, RadioStyle};
pub use range_picker::{
    analytics_range_picker, date_range_picker, range_picker, PresetRange, RangeFocus, RangePicker,
};
pub use rating::{rating, Rating, RatingSize, RatingStyle};
pub use resizable::{resizable, Resizable, ResizeDirection, ResizeHandle, ResizeStyle};
pub use richlog::{log_entry, richlog, LogEntry, LogLevel, RichLog};
pub use richtext::{markup, rich_text, span, style, RichText, Span, Style};
pub use scatterchart::{bubble_chart, scatter_chart, ScatterChart, ScatterSeries};
pub use screen::{screen, screen_stack, Screen, ScreenStack, ScreenTransition};
pub use scroll::{scroll_view, ScrollView};
pub use search_bar::{search_bar, SearchBar};
pub use select::{select, Select};
pub use selection_list::{
    selection_item, selection_list, SelectionItem, SelectionList, SelectionStyle,
};
pub use skeleton::{
    skeleton, skeleton_avatar, skeleton_paragraph, skeleton_text, Skeleton, SkeletonShape,
};
pub use slider::{
    percentage_slider, slider, slider_range, volume_slider, Slider, SliderOrientation, SliderStyle,
};
#[cfg(feature = "markdown")]
pub use slides::{parse_slides, SlideContent, SlideNav};
pub use sortable::{sortable_list, SortableItem, SortableList};
pub use sparkline::{sparkline, Sparkline, SparklineStyle};
pub use spinner::{spinner, Spinner, SpinnerStyle};
pub use splitter::{
    hsplit, pane, splitter, vsplit, HSplit, Pane, SplitOrientation, Splitter, VSplit,
};
pub use stack::{hstack, vstack, Direction, Stack};
pub use status_indicator::{
    away_indicator, busy_indicator, offline, online, status_indicator, Status, StatusIndicator,
    StatusSize, StatusStyle,
};
pub use statusbar::{
    footer, header, key_hint, section as status_section, statusbar, KeyHint, SectionAlign,
    StatusBar, StatusSection,
};
pub use stepper::{step, stepper, Step, StepStatus, Stepper, StepperOrientation, StepperStyle};
pub use streamline::{
    genre_stream, resource_stream, streamline, streamline_with_data, traffic_stream,
    StreamBaseline, StreamLayer, StreamOrder, Streamline,
};
pub use switch::{switch, toggle, Switch, SwitchStyle};
pub use syntax::{HighlightSpan, Language, SyntaxHighlighter, SyntaxTheme};
pub use table::{column, table, Column, Table};
pub use tabs::{tabs, Tab, Tabs};
pub use tag::{chip, tag, Tag, TagStyle};
pub use terminal::{terminal, CursorStyle, TermCell, TermLine, Terminal, TerminalAction};
pub use text::{Alignment, Text};
pub use textarea::{textarea, TextArea};
pub use theme_picker::{theme_picker, ThemePicker};
pub use timeline::{
    timeline, timeline_event, EventType, Timeline, TimelineEvent, TimelineOrientation,
    TimelineStyle,
};
pub use timer::{pomodoro, timer as timer_widget, Timer, TimerFormat, TimerState};
pub use timer::{stopwatch, Stopwatch};
pub use timeseries::{
    cpu_chart, memory_chart, network_chart, time_series, time_series_with_data, MarkerStyle,
    TimeFormat, TimeLineStyle, TimeMarker, TimePoint, TimeRange, TimeSeries, TimeSeriesData,
};
pub use toast::{toast, Toast, ToastLevel, ToastPosition};
pub use tooltip::{tooltip, Tooltip, TooltipArrow, TooltipPosition, TooltipStyle};
pub use traits::{
    Draggable, Element, EventResult, FocusStyle, Interactive, RenderContext, StyledView, Timeout,
    View, WidgetProps, WidgetState, DISABLED_BG, DISABLED_FG,
};
pub use tree::{tree, tree_node, Tree, TreeNode};
pub use vim::{vim_state, VimAction, VimCommandResult, VimMode, VimMotion, VimState};
pub use virtuallist::{virtual_list, ScrollAlignment, ScrollMode, VirtualList};
pub use waveline::{
    area_wave, audio_waveform, sawtooth_wave, signal_wave, sine_wave, spectrum, square_wave,
    waveline, Interpolation, WaveStyle, Waveline,
};
pub use zen::{zen, zen_dark, zen_light, ZenMode};

// Re-export common widget constructors

/// Create a new text widget
pub fn text(content: impl Into<String>) -> Text {
    Text::new(content)
}
