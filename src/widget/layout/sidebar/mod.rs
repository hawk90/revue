//! Sidebar Navigation widget
//!
//! A vertical navigation rail for application navigation.
//!
//! Features:
//! - Vertical navigation rail
//! - Collapsible (icon-only mode)
//! - Nested/hierarchical items
//! - Active item highlighting
//! - Icon + label format
//! - Section dividers
//! - Header/footer slots
//! - Keyboard navigation

mod builder;
mod helpers;
mod render;
mod state;
#[cfg(test)]
mod tests;
pub mod types;

pub use types::{CollapseMode, FlattenedItem, SidebarItem, SidebarSection};

use crate::style::Color;
use crate::widget::theme::PLACEHOLDER_FG;
use crate::widget::traits::{WidgetProps, DISABLED_FG};
use crate::{impl_props_builders, impl_styled_view};

/// Sidebar widget for vertical navigation
#[derive(Clone, Debug)]
pub struct Sidebar {
    /// Navigation sections
    sections: Vec<SidebarSection>,
    /// Currently selected item ID
    selected: Option<String>,
    /// Currently hovered item index in flattened list
    hovered: usize,
    /// Collapse mode
    collapse_mode: CollapseMode,
    /// Auto-collapse width threshold
    collapse_threshold: u16,
    /// Expanded width
    expanded_width: u16,
    /// Collapsed width (icon-only)
    collapsed_width: u16,
    /// Header content (rendered at top)
    header: Option<String>,
    /// Footer content (rendered at bottom)
    footer: Option<String>,
    /// Scroll offset
    scroll: usize,
    // Styling
    fg: Option<Color>,
    bg: Option<Color>,
    selected_fg: Option<Color>,
    selected_bg: Option<Color>,
    hover_fg: Option<Color>,
    hover_bg: Option<Color>,
    disabled_fg: Option<Color>,
    section_fg: Option<Color>,
    badge_fg: Option<Color>,
    badge_bg: Option<Color>,
    border_fg: Option<Color>,
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    /// Widget props
    pub props: WidgetProps,
}

impl Sidebar {
    /// Create a new sidebar
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            selected: None,
            hovered: 0,
            collapse_mode: CollapseMode::Expanded,
            collapse_threshold: 20,
            expanded_width: 24,
            collapsed_width: 4,
            header: None,
            footer: None,
            scroll: 0,
            fg: None,
            bg: Some(Color::rgb(30, 30, 40)),
            selected_fg: Some(Color::WHITE),
            selected_bg: Some(Color::BLUE),
            hover_fg: Some(Color::WHITE),
            hover_bg: Some(Color::rgb(50, 50, 70)),
            disabled_fg: Some(DISABLED_FG),
            section_fg: Some(PLACEHOLDER_FG),
            badge_fg: Some(Color::WHITE),
            badge_bg: Some(Color::RED),
            border_fg: Some(Color::rgb(60, 60, 80)),
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::widget::traits::View for Sidebar {
    fn render(&self, ctx: &mut crate::widget::traits::RenderContext) {
        let area = crate::widget::layout::constraints::constrain(
            ctx.area,
            self.min_width,
            self.min_height,
            self.max_width,
            self.max_height,
        );
        crate::widget::layout::constraints::within(ctx, area, |ctx| self.render_sidebar(ctx));
    }

    crate::impl_view_meta!("Sidebar");
}

impl_styled_view!(Sidebar);
impl_props_builders!(Sidebar);

// Re-export helper functions
pub use helpers::{sidebar, sidebar_item, sidebar_section, sidebar_section_titled};
