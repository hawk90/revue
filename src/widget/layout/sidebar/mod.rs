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

mod helpers;
mod render;
mod state;
pub mod types;

pub use types::{CollapseMode, FlattenedItem, SidebarItem, SidebarSection};

use crate::style::Color;
use crate::widget::traits::{WidgetProps, DISABLED_FG};
use crate::{impl_props_builders, impl_styled_view};

/// Sidebar widget for vertical navigation
#[derive(Clone, Debug)]
pub struct Sidebar {
    /// Navigation sections
    pub(crate) sections: Vec<SidebarSection>,
    /// Currently selected item ID
    pub(crate) selected: Option<String>,
    /// Currently hovered item index in flattened list
    pub(crate) hovered: usize,
    /// Collapse mode
    pub(crate) collapse_mode: CollapseMode,
    /// Auto-collapse width threshold
    pub(crate) collapse_threshold: u16,
    /// Expanded width
    pub(crate) expanded_width: u16,
    /// Collapsed width (icon-only)
    pub(crate) collapsed_width: u16,
    /// Header content (rendered at top)
    pub(crate) header: Option<String>,
    /// Footer content (rendered at bottom)
    pub(crate) footer: Option<String>,
    /// Scroll offset
    pub(crate) scroll: usize,
    // Styling
    pub(crate) fg: Option<Color>,
    pub(crate) bg: Option<Color>,
    pub(crate) selected_fg: Option<Color>,
    pub(crate) selected_bg: Option<Color>,
    pub(crate) hover_fg: Option<Color>,
    pub(crate) hover_bg: Option<Color>,
    pub(crate) disabled_fg: Option<Color>,
    pub(crate) section_fg: Option<Color>,
    pub(crate) badge_fg: Option<Color>,
    pub(crate) badge_bg: Option<Color>,
    pub(crate) border_fg: Option<Color>,
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
            section_fg: Some(Color::rgb(128, 128, 128)),
            badge_fg: Some(Color::WHITE),
            badge_bg: Some(Color::RED),
            border_fg: Some(Color::rgb(60, 60, 80)),
            props: WidgetProps::new(),
        }
    }

    /// Add a section
    pub fn section(mut self, section: SidebarSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Add multiple sections
    pub fn sections(mut self, sections: Vec<SidebarSection>) -> Self {
        self.sections = sections;
        self
    }

    /// Add items without a section title
    pub fn items(mut self, items: Vec<SidebarItem>) -> Self {
        self.sections.push(SidebarSection::new(items));
        self
    }

    /// Set selected item by ID
    pub fn selected(mut self, id: impl Into<String>) -> Self {
        self.selected = Some(id.into());
        self
    }

    /// Set collapse mode
    pub fn collapse_mode(mut self, mode: CollapseMode) -> Self {
        self.collapse_mode = mode;
        self
    }

    /// Set collapse threshold for auto mode
    pub fn collapse_threshold(mut self, width: u16) -> Self {
        self.collapse_threshold = width;
        self
    }

    /// Set expanded width
    pub fn expanded_width(mut self, width: u16) -> Self {
        self.expanded_width = width;
        self
    }

    /// Set collapsed width
    pub fn collapsed_width(mut self, width: u16) -> Self {
        self.collapsed_width = width;
        self
    }

    /// Set header text
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Set footer text
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set selected item style
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set hover style
    pub fn hover_style(mut self, fg: Color, bg: Color) -> Self {
        self.hover_fg = Some(fg);
        self.hover_bg = Some(bg);
        self
    }

    /// Set disabled color
    pub fn disabled_color(mut self, color: Color) -> Self {
        self.disabled_fg = Some(color);
        self
    }

    /// Set section title color
    pub fn section_color(mut self, color: Color) -> Self {
        self.section_fg = Some(color);
        self
    }

    /// Set badge style
    pub fn badge_style(mut self, fg: Color, bg: Color) -> Self {
        self.badge_fg = Some(fg);
        self.badge_bg = Some(bg);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_fg = Some(color);
        self
    }

    /// Toggle sidebar collapse mode
    pub fn toggle_collapse(&mut self) {
        self.collapse_mode = match self.collapse_mode {
            CollapseMode::Expanded => CollapseMode::Collapsed,
            CollapseMode::Collapsed => CollapseMode::Expanded,
            CollapseMode::Auto => CollapseMode::Collapsed,
        };
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::widget::traits::View for Sidebar {
    fn render(&self, ctx: &mut crate::widget::traits::RenderContext) {
        self.render_sidebar(ctx);
    }

    crate::impl_view_meta!("Sidebar");
}

impl_styled_view!(Sidebar);
impl_props_builders!(Sidebar);

// Re-export helper functions
pub use helpers::{sidebar, sidebar_item, sidebar_section, sidebar_section_titled};

// Import state methods
use state::SidebarState;

// State getters and navigation methods
impl Sidebar {
    /// Get selected item ID
    pub fn selected_id(&self) -> Option<&str> {
        SidebarState::selected_id(self)
    }

    /// Get hovered index
    pub fn hovered_index(&self) -> usize {
        SidebarState::hovered_index(self)
    }

    /// Check if sidebar is collapsed
    pub fn is_collapsed(&self) -> bool {
        SidebarState::is_collapsed(self)
    }

    /// Get current width based on collapse state
    pub fn current_width(&self) -> u16 {
        SidebarState::current_width(self, self.expanded_width, self.collapsed_width)
    }

    /// Get flattened list of visible items
    pub fn visible_items(&self) -> Vec<FlattenedItem> {
        SidebarState::visible_items(self)
    }

    /// Get total item count (excluding sections)
    pub fn item_count(&self) -> usize {
        SidebarState::item_count(self)
    }

    /// Move hover down
    pub fn hover_down(&mut self) {
        SidebarState::hover_down(self);
    }

    /// Move hover up
    pub fn hover_up(&mut self) {
        SidebarState::hover_up(self);
    }

    /// Select the currently hovered item
    pub fn select_hovered(&mut self) {
        SidebarState::select_hovered(self);
    }

    /// Toggle expansion of hovered item
    pub fn toggle_hovered(&mut self) {
        SidebarState::toggle_hovered(self);
    }

    /// Toggle item expansion by ID
    pub fn toggle_item(&mut self, id: &str) {
        SidebarState::toggle_item(self, id);
    }

    /// Expand all items
    pub fn expand_all(&mut self) {
        SidebarState::expand_all(self);
    }

    /// Collapse all items
    pub fn collapse_all(&mut self) {
        SidebarState::collapse_all(self);
    }
}
