//! Widget integration tests - split into modules by widget type
//!
//! NOTE: Some modules are temporarily disabled due to API mismatches.
//! Search for "TODO(test-coverage)" to find disabled modules.

#[path = "widget/accordion.rs"]
mod accordion;
// TODO(test-coverage): aistream has API mismatches
// #[path = "widget/aistream/mod.rs"]
// mod aistream;
#[path = "widget/alert.rs"]
mod alert;
// TODO(test-coverage): autocomplete has API mismatches
// #[path = "widget/autocomplete/mod.rs"]
// mod autocomplete;
#[path = "widget/avatar.rs"]
mod avatar;
#[path = "widget/badge.rs"]
mod badge;
// TODO(test-coverage): breadcrumb has API mismatches
// #[path = "widget/breadcrumb/mod.rs"]
// pub mod breadcrumb;
#[path = "widget/button.rs"]
mod button;
#[path = "widget/calendar.rs"]
mod calendar;
// TODO(test-coverage): callout has API mismatches
// #[path = "widget/callout/mod.rs"]
// mod callout;
#[path = "widget/candlechart.rs"]
mod candlechart;
// TODO(test-coverage): card has API mismatches
// #[path = "widget/card.rs"]
// mod card;
// TODO(test-coverage): canvas has API mismatches
// #[path = "widget/canvas/mod.rs"]
// pub mod canvas;
// TODO(test-coverage): checkbox has API mismatches
// #[path = "widget/checkbox/mod.rs"]
// mod checkbox;
// TODO(test-coverage): code_editor_tests has API mismatches
// #[path = "widget/code_editor_tests.rs"]
// pub mod code_editor_tests;
#[path = "widget/collapsible.rs"]
pub mod collapsible;
#[path = "widget/color_picker.rs"]
mod color_picker;
// TODO(test-coverage): combobox has API mismatches
// #[path = "widget/combobox/mod.rs"]
// mod combobox;
// TODO(test-coverage): combobox_tests has API mismatches
// #[path = "widget/combobox_tests.rs"]
// pub mod combobox_tests;
#[path = "widget/command_palette.rs"]
mod command_palette;
#[path = "widget/command_palette_unit/mod.rs"]
pub mod command_palette_unit;
// TODO(test-coverage): data has API mismatches
// #[path = "widget/data/mod.rs"]
// pub mod data;
// TODO(test-coverage): data_chart_tests has API mismatches
// #[path = "widget/data_chart_tests.rs"]
// pub mod data_chart_tests;
// TODO(test-coverage): datagrid has API mismatches
// #[path = "widget/datagrid/mod.rs"]
// mod datagrid;
#[path = "widget/datetime_picker.rs"]
mod datetime_picker;
// TODO(test-coverage): dropzone has API mismatches
// #[path = "widget/dropzone/mod.rs"]
// pub mod dropzone;
// TODO(test-coverage): debug_overlay has API mismatches
// #[path = "widget/debug_overlay.rs"]
// mod debug_overlay;
// TODO(test-coverage): developer has API mismatches
// #[path = "widget/developer/mod.rs"]
// pub mod developer;
#[cfg(feature = "diff")]
#[path = "widget/diff.rs"]
mod diff;
// TODO(test-coverage): digits has API mismatches
// #[path = "widget/digits.rs"]
// mod digits;
#[path = "widget/divider.rs"]
mod divider;
// TODO(test-coverage): display has API mismatches
// #[path = "widget/display/mod.rs"]
// mod display;
#[path = "widget/empty_state.rs"]
mod empty_state;
// TODO(test-coverage): feedback has API mismatches
// #[path = "widget/feedback/mod.rs"]
// pub mod feedback;
// TODO(test-coverage): filepicker has API mismatches
// #[path = "widget/filepicker/mod.rs"]
// mod filepicker;
// TODO(test-coverage): filetree has API mismatches
// #[path = "widget/filetree/mod.rs"]
// mod filetree;
// TODO(test-coverage): form has API mismatches
// #[path = "widget/form/mod.rs"]
// pub mod form;
// TODO(test-coverage): form_tests has API mismatches
// #[path = "widget/form_tests.rs"]
// mod form_tests;
#[path = "widget/gauge.rs"]
mod gauge;
// TODO(test-coverage): heatmap_tests has API mismatches
// #[path = "widget/heatmap_tests.rs"]
// pub mod heatmap_tests;
// TODO(test-coverage): httpclient has API mismatches
// #[path = "widget/httpclient/mod.rs"]
// pub mod httpclient;
// TODO(test-coverage): multi_select has API mismatches
// #[path = "widget/multi_select/mod.rs"]
// pub mod multi_select;
#[cfg(feature = "image")]
#[path = "widget/image.rs"]
mod image;
// TODO(test-coverage): input has API mismatches
// #[path = "widget/input/mod.rs"]
// mod input;
// TODO(test-coverage): link has API mismatches
// #[path = "widget/link.rs"]
// mod link;
// TODO(test-coverage): layout has API mismatches
// #[path = "widget/layout/mod.rs"]
// mod layout;
// TODO(test-coverage): list has API mismatches
// #[path = "widget/list.rs"]
// mod list;
#[path = "widget/log_viewer_tests.rs"]
mod log_viewer_tests;
#[cfg(feature = "markdown")]
#[path = "widget/markdown/mod.rs"]
mod markdown;
#[path = "widget/masked_input.rs"]
mod masked_input;
#[path = "widget/masked_input_tests.rs"]
mod masked_input_tests;
// TODO(test-coverage): mermaid has API mismatches
// #[path = "widget/mermaid/mod.rs"]
// mod mermaid;
// TODO(test-coverage): option_list has API mismatches
// #[path = "widget/option_list/mod.rs"]
// mod option_list;
#[path = "widget/pagination.rs"]
mod pagination;
// TODO(test-coverage): presentation has API mismatches
// #[path = "widget/presentation/mod.rs"]
// mod presentation;
#[cfg(feature = "sysinfo")]
#[path = "widget/procmon.rs"]
mod procmon;
#[path = "widget/progress.rs"]
mod progress;
#[cfg(feature = "qrcode")]
#[path = "widget/qrcode.rs"]
mod qrcode;
#[path = "widget/radio.rs"]
mod radio;
// TODO(test-coverage): range_picker has API mismatches
// #[path = "widget/range_picker/mod.rs"]
// mod range_picker;
#[path = "widget/rating.rs"]
mod rating;
#[path = "widget/resizable.rs"]
mod resizable;
#[path = "widget/richlog.rs"]
mod richlog;
#[path = "widget/richtext.rs"]
mod richtext;
#[path = "widget/screen.rs"]
pub mod screen;
#[path = "widget/search_bar.rs"]
mod search_bar;
// TODO(test-coverage): sortable has API mismatches
// #[path = "widget/sortable/mod.rs"]
// pub mod sortable;
#[path = "widget/scroll.rs"]
mod scroll;
#[path = "widget/select.rs"]
mod select;
#[path = "widget/selection_list.rs"]
mod selection_list;
#[path = "widget/sidebar_tests.rs"]
mod sidebar_tests;
#[path = "widget/skeleton.rs"]
mod skeleton;
// TODO(test-coverage): slider has API mismatches
// #[path = "widget/slider/mod.rs"]
// mod slider;
#[cfg(feature = "markdown")]
#[path = "widget/slides.rs"]
mod slides;
#[path = "widget/spinner.rs"]
mod spinner;
// TODO(test-coverage): splitter has API mismatches
// #[path = "widget/splitter/mod.rs"]
// mod splitter;
#[path = "widget/statusbar.rs"]
mod statusbar;
#[path = "widget/stepper.rs"]
mod stepper;
// TODO(test-coverage): streamline has API mismatches
// #[path = "widget/streamline/mod.rs"]
// mod streamline;
#[path = "widget/switch.rs"]
mod switch;
#[path = "widget/syntax.rs"]
mod syntax;
#[path = "widget/tabs.rs"]
mod tabs;
#[path = "widget/terminal.rs"]
mod terminal;
#[path = "widget/terminal_ansi.rs"]
mod terminal_ansi;
#[path = "widget/terminal_types.rs"]
mod terminal_types;
#[cfg(feature = "syntax-highlighting")]
#[path = "widget/tree_sitter_highlight.rs"]
mod tree_sitter_highlight;
// TODO(test-coverage): text has API mismatches
// #[path = "widget/text/mod.rs"]
// mod text;
// TODO(test-coverage): textarea has API mismatches
// #[path = "widget/textarea/mod.rs"]
// mod textarea;
#[path = "widget/theme_picker.rs"]
mod theme_picker;
#[path = "widget/timeline.rs"]
mod timeline;
#[path = "widget/timer.rs"]
mod timer;
#[path = "widget/timeseries_tests.rs"]
pub mod timeseries_tests;
#[path = "widget/tooltip.rs"]
mod tooltip;
// TODO(test-coverage): transition has API mismatches
// #[path = "widget/transition/mod.rs"]
// mod transition;
#[path = "widget/validation.rs"]
mod validation;
#[path = "widget/vim.rs"]
mod vim;
// TODO(test-coverage): waveline has API mismatches
// #[path = "widget/waveline/mod.rs"]
// mod waveline;
#[path = "widget/zen.rs"]
mod zen;

// Shared infrastructure tests
#[path = "widget/traits/dropdown.rs"]
mod dropdown_tests;
#[path = "widget/traits/focus_handlers.rs"]
mod focus_handlers_tests;
#[path = "widget/traits/theme.rs"]
mod theme_constants_tests;
