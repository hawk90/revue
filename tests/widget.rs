//! Widget integration tests - split into modules by widget type

#[path = "widget/breadcrumb/mod.rs"]
mod breadcrumb;
#[path = "widget/callout/mod.rs"]
mod callout;
#[path = "widget/canvas/mod.rs"]
mod canvas;
#[path = "widget/command_palette.rs"]
mod command_palette;
#[path = "widget/command_palette_unit/mod.rs"]
pub mod command_palette_unit;
#[path = "widget/data/mod.rs"]
pub mod data;
#[path = "widget/datetime_picker.rs"]
mod datetime_picker;
#[path = "widget/debug_overlay.rs"]
mod debug_overlay;
#[path = "widget/developer/mod.rs"]
mod developer;
#[path = "widget/display/mod.rs"]
mod display;
#[path = "widget/dropzone/mod.rs"]
mod dropzone;
#[path = "widget/edge_cases/mod.rs"]
mod edge_cases;
#[path = "widget/feedback/mod.rs"]
mod feedback;
#[path = "widget/filepicker/mod.rs"]
mod filepicker;
#[path = "widget/form/mod.rs"]
mod form;
#[cfg(feature = "image")]
#[path = "widget/image.rs"]
mod image;
#[path = "widget/input/mod.rs"]
mod input;
#[path = "widget/layout/mod.rs"]
mod layout;
#[path = "widget/link.rs"]
mod link;
#[path = "widget/macros.rs"]
mod macros;
#[cfg(feature = "markdown")]
#[path = "widget/markdown/mod.rs"]
mod markdown;
#[cfg(feature = "markdown")]
#[path = "widget/markdown_presentation.rs"]
mod markdown_presentation;
#[path = "widget/mermaid/mod.rs"]
mod mermaid;
#[path = "widget/multi_select/mod.rs"]
mod multi_select;
#[path = "widget/option_list.rs"]
mod option_list;
#[path = "widget/pagination.rs"]
mod pagination;
#[cfg(feature = "qrcode")]
#[path = "widget/qrcode.rs"]
mod qrcode;
#[path = "widget/range_picker/mod.rs"]
mod range_picker;
#[cfg(feature = "markdown")]
#[path = "widget/slides.rs"]
mod slides;
#[path = "widget/sortable/mod.rs"]
mod sortable;
#[path = "widget/streamline/mod.rs"]
mod streamline;
#[path = "widget/syntax.rs"]
mod syntax;
#[path = "widget/theme_picker.rs"]
mod theme_picker;
#[path = "widget/transition/mod.rs"]
mod transition;
#[path = "widget/validation.rs"]
mod validation;
#[path = "widget/zen.rs"]
mod zen;

// Shared infrastructure tests
#[path = "widget/traits/focus_handlers.rs"]
mod focus_handlers_tests;
#[path = "widget/traits/theme.rs"]
mod theme_constants_tests;
#[path = "widget/traits/mod.rs"]
mod traits;
