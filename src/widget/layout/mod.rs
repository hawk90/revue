//! Layout widgets - Container and positioning components
//!
//! Widgets for arranging and positioning other widgets.

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
