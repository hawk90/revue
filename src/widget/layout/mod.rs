//! Layout widgets - Container and positioning components
//!
//! This module provides widgets for arranging and positioning other widgets.
//! Includes flexbox stacks, CSS Grid, split panes, scrolling, and more.
//!
//! # Widget Categories
//!
//! ## Basic Layout
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Stack`] | Vertical/horizontal stack | [`vstack()`][vstack], [`hstack()`][hstack] |
//! | [`Border`] | Bordered container | [`border()`] |
//! | [`Card`] | Content card with header/footer | [`card()`] |
//!
//! ## Advanced Layout
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Grid`] | CSS Grid layout | [`grid()`] |
//! | [`Splitter`] | Resizable split panes | [`hsplit()`][hsplit], [`vsplit()`][vsplit] |
//! | [`ScrollView`] | Scrollable container | [`scroll_view()`] |
//! | [`Layers`] | Z-index layering | [`layers()`] |
//! | [`Positioned`] | Absolute positioning | [`positioned()`] |
//!
//! ## Navigation
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Tabs`] | Tab container | [`tabs()`] |
//! | [`Accordion`] | Collapsible sections | [`accordion()`] |
//! | [`Collapsible`] | Single expandable section | [`collapsible()`] |
//! | [`Sidebar`] | Vertical navigation | [`sidebar()`] |
//!
//! ## Special
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Resizable`] | Resizable container | [`resizable()`] |
//! | [`Screen`] | Screen/page widget | [`screen()`] |
//!
//! # Quick Start
//!
//! ## Vertical Stack
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! vstack()
//!     .gap(1)
//!     .child(Text::new("Title"))
//!     .child(Text::new("Content"))
//!     .child(Text::new("Footer"));
//! ```
//!
//! ## Horizontal Stack
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! hstack()
//!     .gap(2)
//!     .child(Text::new("Left"))
//!     .child(Text::new("Center"))
//!     .child(Text::new("Right"));
//! ```
//!
//! ## CSS Grid
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! grid()
//!     .template_columns("1fr 2fr 1fr")
//!     .template_rows("auto 1fr auto")
//!     .child(grid_item().row(1).col(1).child(Text::new("Header")))
//!     .child(grid_item().row(2).col(1).col_span(3).child(Text::new("Content")))
//!     .child(grid_item().row(3).col(1).child(Text::new("Footer")));
//! ```
//!
//! ## Split Panes
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! hsplit()
//!     .ratio(0.3, 0.7)
//!     .pane(0, Text::new("Left Panel"))
//!     .pane(1, Text::new("Right Panel"));
//! ```
//!
//! ## Scroll View
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! scroll_view()
//!     .height(10)
//!     .child(vstack().children((0..100).map(|i| Text::new(format!("Line {}", i)))));
//! ```
//!
//! ## Tabs
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! tabs()
//!     .tab("Tab 1", Text::new("Content 1"))
//!     .tab("Tab 2", Text::new("Content 2"))
//!     .tab("Tab 3", Text::new("Content 3"));
//! ```
//!
//! [vstack]: crate::widget::vstack
//! [hstack]: crate::widget::hstack
//! [hsplit]: crate::widget::hsplit
//! [vsplit]: crate::widget::vsplit

pub mod accordion;
pub mod border;
pub mod card;
pub mod collapsible;
pub mod dock;
pub mod grid;
pub mod layer;
pub mod positioned;
pub mod resizable;
pub mod screen;
pub mod scroll;
pub mod sidebar;
pub mod splitter;
pub mod stack;
pub mod tabs;

// Re-exports for convenience
pub use accordion::{accordion, section, Accordion, AccordionSection};
pub use border::{border, draw_border, Border, BorderType};
pub use card::{card, Card, CardVariant};
pub use collapsible::{collapsible, Collapsible};
#[allow(unused_imports)]
pub use dock::{dock, dock_area, DockArea, DockManager, DockPosition, TabContent};
pub use grid::{
    grid, grid_item, grid_template, Grid, GridAlign, GridItem, GridPlacement, TrackSize,
};
pub use layer::{layers, Layers};
pub use positioned::{positioned, Anchor, Positioned};
pub use resizable::{resizable, Resizable, ResizeDirection, ResizeHandle, ResizeStyle};
pub use screen::{screen, screen_stack, Screen, ScreenStack, ScreenTransition};
pub use scroll::{scroll_view, ScrollView};
pub use sidebar::{
    sidebar, sidebar_item, sidebar_section, sidebar_section_titled, CollapseMode, FlattenedItem,
    Sidebar, SidebarItem, SidebarSection,
};
pub use splitter::{
    hsplit, pane, splitter, vsplit, HSplit, Pane, SplitOrientation, Splitter, VSplit,
};
pub use stack::{hstack, vstack, Direction, Stack};
pub use tabs::{tabs, Tab, Tabs};
