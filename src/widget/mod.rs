//! Widget system for Revue TUI framework.
//!
//! This module provides 70+ widgets for building terminal user interfaces.
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

pub mod traits;
mod stack;
mod text;
mod list;
mod input;
mod border;
mod progress;
mod spinner;
mod table;
mod select;
mod theme_picker;
mod modal;
mod tabs;
mod tree;
mod scroll;
mod markdown;
mod image;
mod button;
mod checkbox;
mod radio;
mod sparkline;
mod layer;
mod positioned;
mod canvas;
mod barchart;
mod toast;
mod textarea;
mod chart;
mod calendar;
mod tooltip;
mod grid;
mod switch;
mod gauge;
mod notification;
mod slider;
mod command_palette;
mod accordion;
mod color_picker;
mod breadcrumb;
mod stepper;
mod screen;
mod menu;
mod statusbar;
mod splitter;
mod filetree;
mod timeline;
mod datagrid;
mod richlog;
mod richtext;
mod divider;
mod badge;
mod avatar;
mod tag;
mod skeleton;
mod pagination;
mod virtuallist;
mod terminal;
mod contextmenu;
mod autocomplete;
mod rating;
mod digits;
mod link;
mod masked_input;
mod selection_list;
mod option_list;
mod timer;
mod filepicker;
mod heatmap;
mod candlechart;
mod timeseries;
mod waveline;
mod streamline;
mod syntax;
mod qrcode;
mod diff;
mod aistream;
mod presentation;
mod procmon;
mod debug_overlay;
mod mermaid;
mod vim;
mod httpclient;
mod dropzone;
mod sortable;
mod resizable;
#[macro_use]
mod macros;

pub use traits::{View, Element, RenderContext, Timeout, WidgetState, WidgetProps, DISABLED_FG, DISABLED_BG, Interactive, EventResult, StyledView, FocusStyle, Draggable};
pub use dropzone::{DropZone, DropZoneStyle, drop_zone};
pub use sortable::{SortableList, SortableItem, sortable_list};
pub use resizable::{Resizable, ResizeHandle, ResizeDirection, ResizeStyle, resizable};
pub use stack::{Stack, vstack, hstack, Direction};
pub use text::{Text, Alignment};
pub use list::{List, list};
pub use input::{Input, input};
pub use border::{Border, BorderType, border};
pub use progress::{Progress, ProgressStyle, progress};
pub use spinner::{Spinner, SpinnerStyle, spinner};
pub use table::{Table, Column, table, column};
pub use select::{Select, select};
pub use theme_picker::{ThemePicker, theme_picker};
pub use modal::{Modal, ModalButton, ModalButtonStyle, modal};
pub use tabs::{Tabs, Tab, tabs};
pub use tree::{Tree, TreeNode, tree, tree_node};
pub use scroll::{ScrollView, scroll_view};
pub use markdown::{Markdown, markdown};
pub use image::{Image, ImageError, ImageResult, ScaleMode, image_from_file, try_image_from_file};
pub use button::{Button, ButtonVariant, button};
pub use checkbox::{Checkbox, CheckboxStyle, checkbox};
pub use radio::{RadioGroup, RadioStyle, RadioLayout, radio_group};
pub use sparkline::{Sparkline, SparklineStyle, sparkline};
pub use layer::{Layers, layers};
pub use positioned::{Positioned, Anchor, positioned};
pub use canvas::{
    Canvas, DrawContext, canvas,
    BrailleCanvas, BrailleContext, BrailleGrid, braille_canvas,
    Shape, Line, Circle, FilledCircle, Rectangle, FilledRectangle, Points,
};
pub use barchart::{BarChart, BarOrientation, barchart};
pub use toast::{Toast, ToastLevel, ToastPosition, toast};
pub use textarea::{TextArea, textarea};
pub use chart::{
    Chart, ChartType, Series, Axis, AxisFormat,
    Marker, LineStyle, LegendPosition,
    chart, line_chart, scatter_plot,
};
pub use calendar::{
    Calendar, CalendarMode, FirstDayOfWeek, Date, DateMarker,
    calendar,
};
pub use tooltip::{
    Tooltip, TooltipPosition, TooltipArrow, TooltipStyle,
    tooltip,
};
pub use grid::{
    Grid, GridItem, GridPlacement, GridAlign, TrackSize,
    grid, grid_item, grid_template,
};
pub use switch::{Switch, SwitchStyle, switch, toggle};
pub use gauge::{Gauge, GaugeStyle, LabelPosition, gauge, percentage, battery};
pub use notification::{
    NotificationCenter, Notification, NotificationLevel, NotificationPosition,
    notification_center,
};
pub use slider::{
    Slider, SliderOrientation, SliderStyle,
    slider, slider_range, percentage_slider, volume_slider,
};
pub use command_palette::{CommandPalette, Command, command_palette};
pub use accordion::{Accordion, AccordionSection, accordion, section};
pub use color_picker::{ColorPicker, ColorPickerMode, ColorPalette, color_picker};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem, SeparatorStyle, breadcrumb, crumb};
pub use stepper::{Stepper, Step, StepStatus, StepperOrientation, StepperStyle, stepper, step};
pub use screen::{Screen, ScreenStack, ScreenTransition, screen, screen_stack};
pub use menu::{Menu, MenuItem, MenuBar, ContextMenu, menu, menu_item, menu_bar, context_menu};
pub use statusbar::{StatusBar, StatusSection, KeyHint, SectionAlign, statusbar, header, footer, section as status_section, key_hint};
pub use splitter::{Splitter, Pane, SplitOrientation, HSplit, VSplit, splitter, pane, hsplit, vsplit};
pub use filetree::{FileTree, FileEntry, FileType, file_tree, file_entry, dir_entry};
pub use timeline::{Timeline, TimelineEvent, TimelineOrientation, TimelineStyle, EventType, timeline, timeline_event};
pub use datagrid::{DataGrid, GridColumn, GridRow, SortDirection, datagrid, grid_column, grid_row};
pub use richlog::{RichLog, LogEntry, LogLevel, richlog, log_entry};
pub use richtext::{RichText, Span, Style, rich_text, markup, span, style};
pub use divider::{Divider, Orientation, DividerStyle, divider, vdivider};
pub use badge::{Badge, BadgeVariant, BadgeShape, badge, dot_badge};
pub use avatar::{Avatar, AvatarSize, AvatarShape, avatar, avatar_icon};
pub use tag::{Tag, TagStyle, tag, chip};
pub use skeleton::{Skeleton, SkeletonShape, skeleton, skeleton_text, skeleton_avatar, skeleton_paragraph};
pub use pagination::{Pagination, PaginationStyle, pagination};
pub use virtuallist::{VirtualList, virtual_list, ScrollMode, ScrollAlignment};
pub use terminal::{Terminal, TermCell, TermLine, TerminalAction, CursorStyle, terminal};
// Note: contextmenu module provides alternative implementation
// The primary ContextMenu is exported from menu module
pub use autocomplete::{Autocomplete, Suggestion, FilterMode, autocomplete};
pub use rating::{Rating, RatingStyle, RatingSize, rating};
pub use digits::{Digits, DigitStyle, digits, clock, timer};
pub use link::{Link, LinkStyle, link, url_link};
pub use masked_input::{MaskedInput, MaskStyle, ValidationState, masked_input, password_input, pin_input, credit_card_input};
pub use selection_list::{SelectionList, SelectionItem, SelectionStyle, selection_list, selection_item};
pub use option_list::{OptionList, OptionEntry, OptionItem, SeparatorStyle as OptionSeparatorStyle, option_list, option_item};
pub use timer::{Timer, TimerState, TimerFormat, timer as timer_widget, pomodoro};
pub use timer::{Stopwatch, stopwatch};
pub use filepicker::{FilePicker, PickerMode, FileFilter, PickerEntry, PickerResult, file_picker, save_picker, dir_picker};
pub use heatmap::{HeatMap, ColorScale, CellDisplay, heatmap, contribution_map};
pub use candlechart::{CandleChart, Candle, ChartStyle as CandleStyle, candle_chart, ohlc_chart};
pub use timeseries::{
    TimeSeries, TimeSeriesData, TimePoint, TimeLineStyle,
    TimeFormat, TimeRange, TimeMarker, MarkerStyle,
    time_series, time_series_with_data, cpu_chart, memory_chart, network_chart,
};
pub use waveline::{
    Waveline, WaveStyle, Interpolation,
    waveline, audio_waveform, signal_wave, area_wave, spectrum,
    sine_wave, square_wave, sawtooth_wave,
};
pub use streamline::{
    Streamline, StreamLayer, StreamBaseline, StreamOrder,
    streamline, streamline_with_data, genre_stream, traffic_stream, resource_stream,
};
pub use syntax::{Language, SyntaxHighlighter, SyntaxTheme, HighlightSpan};
pub use qrcode::{QrCodeWidget, QrStyle, ErrorCorrection, qrcode, qrcode_url};
pub use diff::{DiffViewer, DiffMode, DiffLine, ChangeType, DiffColors, diff_viewer, diff};
pub use aistream::{AiStream, TypingStyle, StreamCursor, StreamStatus, ai_stream, ai_response};
pub use presentation::{Presentation, Slide, Transition, SlideAlign, presentation, slide};
pub use procmon::{ProcessMonitor, ProcessInfo, ProcessSort, ProcessView, ProcColors, process_monitor, htop};
pub use debug_overlay::{
    DebugOverlay, DebugConfig, DebugPosition, DebugEvent,
    PerfMetrics, EventLog, WidgetInfo,
    enable_debug, disable_debug, is_debug_enabled, toggle_debug,
};
pub use mermaid::{Diagram, DiagramNode, DiagramEdge, DiagramType, NodeShape, ArrowStyle, diagram, flowchart, node, edge};
pub use vim::{VimState, VimMode, VimMotion, VimAction, VimCommandResult, vim_state};
pub use httpclient::{HttpClient, HttpMethod, HttpRequest, HttpResponse, ResponseView, RequestState, http_client, get as http_get, post as http_post};

// Re-export common widget constructors

/// Create a new text widget
pub fn text(content: impl Into<String>) -> Text {
    Text::new(content)
}
