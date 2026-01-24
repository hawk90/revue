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

#[cfg(test)]
mod tests {
//! Sidebar tests

use super::types::{CollapseMode, FlattenedItem, SidebarItem, SidebarSection};
use super::Sidebar;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

// Re-export helpers from parent module
pub use super::sidebar;
pub use super::sidebar_item;
pub use super::sidebar_section;
pub use super::sidebar_section_titled;

// =========================================================================
// SidebarItem Tests
// =========================================================================

#[test]
fn test_sidebar_item_new() {
    let item = SidebarItem::new("home", "Home");
    assert_eq!(item.id, "home");
    assert_eq!(item.label, "Home");
    assert!(item.icon.is_none());
    assert!(!item.disabled);
    assert!(item.badge.is_none());
    assert!(item.children.is_empty());
    assert!(!item.expanded);
}

#[test]
fn test_sidebar_item_icon() {
    let item = SidebarItem::new("home", "Home").icon('🏠');
    assert_eq!(item.icon, Some('🏠'));
}

#[test]
fn test_sidebar_item_disabled() {
    let item = SidebarItem::new("home", "Home").disabled(true);
    assert!(item.disabled);
}

#[test]
fn test_sidebar_item_badge() {
    let item = SidebarItem::new("inbox", "Inbox").badge("5");
    assert_eq!(item.badge, Some("5".to_string()));
}

#[test]
fn test_sidebar_item_children() {
    let children = vec![
        SidebarItem::new("child1", "Child 1"),
        SidebarItem::new("child2", "Child 2"),
    ];
    let item = SidebarItem::new("parent", "Parent").children(children);
    assert_eq!(item.children.len(), 2);
}

#[test]
fn test_sidebar_item_expanded() {
    let item = SidebarItem::new("folder", "Folder").expanded(true);
    assert!(item.expanded);
}

#[test]
fn test_sidebar_item_has_children() {
    let item =
        SidebarItem::new("folder", "Folder").children(vec![SidebarItem::new("file", "File")]);
    assert!(item.has_children());

    let empty_item = SidebarItem::new("file", "File");
    assert!(!empty_item.has_children());
}

#[test]
fn test_sidebar_item_builder_chain() {
    let item = SidebarItem::new("nav", "Navigation")
        .icon('📁')
        .disabled(false)
        .badge("3")
        .expanded(true);

    assert_eq!(item.icon, Some('📁'));
    assert!(!item.disabled);
    assert_eq!(item.badge, Some("3".to_string()));
    assert!(item.expanded);
}

// =========================================================================
// SidebarSection Tests
// =========================================================================

#[test]
fn test_sidebar_section_new() {
    let items = vec![SidebarItem::new("home", "Home")];
    let section = SidebarSection::new(items);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 1);
}

#[test]
fn test_sidebar_section_titled() {
    let items = vec![SidebarItem::new("home", "Home")];
    let section = SidebarSection::titled("Main", items);
    assert_eq!(section.title, Some("Main".to_string()));
    assert_eq!(section.items.len(), 1);
}

// =========================================================================
// CollapseMode Tests
// =========================================================================

#[test]
fn test_collapse_mode_default() {
    assert_eq!(CollapseMode::default(), CollapseMode::Expanded);
}

#[test]
fn test_collapse_mode_equality() {
    assert_eq!(CollapseMode::Expanded, CollapseMode::Expanded);
    assert_ne!(CollapseMode::Expanded, CollapseMode::Collapsed);
}

// =========================================================================
// Sidebar Creation Tests
// =========================================================================

#[test]
fn test_sidebar_new() {
    let sb = Sidebar::new();
    assert!(sb.selected_id().is_none());
    assert_eq!(sb.hovered_index(), 0);
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_default() {
    let sb = Sidebar::default();
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_helper() {
    let sb = sidebar();
    assert!(sb.selected_id().is_none());
}

#[test]
fn test_sidebar_item_helper() {
    let item = sidebar_item("test", "Test");
    assert_eq!(item.id, "test");
    assert_eq!(item.label, "Test");
}

#[test]
fn test_sidebar_section_helper() {
    let section = sidebar_section(vec![sidebar_item("a", "A")]);
    assert!(section.title.is_none());
    assert_eq!(section.items.len(), 1);
}

#[test]
fn test_sidebar_section_titled_helper() {
    let section = sidebar_section_titled("Title", vec![sidebar_item("a", "A")]);
    assert_eq!(section.title, Some("Title".to_string()));
}

// =========================================================================
// Sidebar Builder Tests
// =========================================================================

#[test]
fn test_sidebar_section_builder() {
    let sb = Sidebar::new().section(SidebarSection::new(vec![SidebarItem::new("home", "Home")]));
    assert_eq!(sb.item_count(), 1);
}

#[test]
fn test_sidebar_sections_builder() {
    let sb = Sidebar::new().sections(vec![
        SidebarSection::new(vec![SidebarItem::new("a", "A")]),
        SidebarSection::new(vec![SidebarItem::new("b", "B")]),
    ]);
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_items_builder() {
    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home"),
        SidebarItem::new("settings", "Settings"),
    ]);
    assert_eq!(sb.item_count(), 2);
}

#[test]
fn test_sidebar_selected() {
    let sb = Sidebar::new()
        .items(vec![SidebarItem::new("home", "Home")])
        .selected("home");
    assert_eq!(sb.selected_id(), Some("home"));
}

#[test]
fn test_sidebar_collapse_mode() {
    let sb = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert!(sb.is_collapsed());
}

#[test]
fn test_sidebar_collapse_threshold() {
    let sb = Sidebar::new().collapse_threshold(30);
    assert_eq!(sb.collapse_threshold, 30);
}

#[test]
fn test_sidebar_expanded_width() {
    let sb = Sidebar::new().expanded_width(32);
    assert_eq!(sb.expanded_width, 32);
}

#[test]
fn test_sidebar_collapsed_width() {
    let sb = Sidebar::new().collapsed_width(6);
    assert_eq!(sb.collapsed_width, 6);
}

#[test]
fn test_sidebar_header() {
    let sb = Sidebar::new().header("App Name");
    assert_eq!(sb.header, Some("App Name".to_string()));
}

#[test]
fn test_sidebar_footer() {
    let sb = Sidebar::new().footer("v1.0.0");
    assert_eq!(sb.footer, Some("v1.0.0".to_string()));
}

#[test]
fn test_sidebar_fg() {
    let sb = Sidebar::new().fg(Color::WHITE);
    assert_eq!(sb.fg, Some(Color::WHITE));
}

#[test]
fn test_sidebar_bg() {
    let sb = Sidebar::new().bg(Color::BLACK);
    assert_eq!(sb.bg, Some(Color::BLACK));
}

#[test]
fn test_sidebar_selected_style() {
    let sb = Sidebar::new().selected_style(Color::WHITE, Color::BLUE);
    assert_eq!(sb.selected_fg, Some(Color::WHITE));
    assert_eq!(sb.selected_bg, Some(Color::BLUE));
}

#[test]
fn test_sidebar_hover_style() {
    let sb = Sidebar::new().hover_style(Color::YELLOW, Color::CYAN);
    assert_eq!(sb.hover_fg, Some(Color::YELLOW));
    assert_eq!(sb.hover_bg, Some(Color::CYAN));
}

#[test]
fn test_sidebar_disabled_color() {
    let sb = Sidebar::new().disabled_color(Color::rgb(128, 128, 128));
    assert_eq!(sb.disabled_fg, Some(Color::rgb(128, 128, 128)));
}

#[test]
fn test_sidebar_section_color() {
    let sb = Sidebar::new().section_color(Color::CYAN);
    assert_eq!(sb.section_fg, Some(Color::CYAN));
}

#[test]
fn test_sidebar_badge_style() {
    let sb = Sidebar::new().badge_style(Color::WHITE, Color::RED);
    assert_eq!(sb.badge_fg, Some(Color::WHITE));
    assert_eq!(sb.badge_bg, Some(Color::RED));
}

#[test]
fn test_sidebar_border_color() {
    let sb = Sidebar::new().border_color(Color::MAGENTA);
    assert_eq!(sb.border_fg, Some(Color::MAGENTA));
}

// =========================================================================
// State Getter Tests
// =========================================================================

#[test]
fn test_sidebar_current_width_expanded() {
    let sb = Sidebar::new()
        .expanded_width(30)
        .collapse_mode(CollapseMode::Expanded);
    assert_eq!(sb.current_width(), 30);
}

#[test]
fn test_sidebar_current_width_collapsed() {
    let sb = Sidebar::new()
        .collapsed_width(5)
        .collapse_mode(CollapseMode::Collapsed);
    assert_eq!(sb.current_width(), 5);
}

#[test]
fn test_sidebar_current_width_auto() {
    let sb = Sidebar::new()
        .expanded_width(25)
        .collapse_mode(CollapseMode::Auto);
    // Auto mode returns expanded_width (actual collapse determined at render)
    assert_eq!(sb.current_width(), 25);
}

#[test]
fn test_sidebar_visible_items_empty() {
    let sb = Sidebar::new();
    assert!(sb.visible_items().is_empty());
}

#[test]
fn test_sidebar_visible_items_flat() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    let items = sb.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_visible_items_with_section_title() {
    let sb = Sidebar::new().section(SidebarSection::titled(
        "Section",
        vec![SidebarItem::new("a", "A")],
    ));
    let items = sb.visible_items();
    assert_eq!(items.len(), 2); // Section header + item
}

#[test]
fn test_sidebar_visible_items_nested_collapsed() {
    let sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
    let items = sb.visible_items();
    // Parent not expanded, so child is hidden
    assert_eq!(items.len(), 1);
}

#[test]
fn test_sidebar_visible_items_nested_expanded() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("parent", "Parent")
        .expanded(true)
        .children(vec![SidebarItem::new("child", "Child")])]);
    let items = sb.visible_items();
    // Parent expanded, so child is visible
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_item_count() {
    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Main",
            vec![SidebarItem::new("a", "A")],
        ))
        .items(vec![SidebarItem::new("b", "B")]);
    // item_count excludes sections
    assert_eq!(sb.item_count(), 2);
}

// =========================================================================
// Navigation Tests
// =========================================================================

#[test]
fn test_sidebar_hover_down() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B"),
        SidebarItem::new("c", "C"),
    ]);
    assert_eq!(sb.hovered_index(), 0);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 1);
    sb.hover_down();
    assert_eq!(sb.hovered_index(), 2);
}

#[test]
fn test_sidebar_hover_down_at_end() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hover_down();
    sb.hover_down();
    sb.hover_down(); // Should stay at last
    assert_eq!(sb.hovered_index(), 1);
}

#[test]
fn test_sidebar_hover_down_skips_disabled() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B").disabled(true),
        SidebarItem::new("c", "C"),
    ]);
    sb.hover_down();
    // Should skip disabled item B and go to C
    assert_eq!(sb.hovered_index(), 2);
}

#[test]
fn test_sidebar_hover_up() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A"),
        SidebarItem::new("b", "B"),
        SidebarItem::new("c", "C"),
    ]);
    sb.hovered = 2;
    sb.hover_up();
    assert_eq!(sb.hovered_index(), 1);
    sb.hover_up();
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_hover_up_at_start() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hover_up(); // Already at 0
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_select_hovered() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A"), SidebarItem::new("b", "B")]);
    sb.hovered = 1;
    sb.select_hovered();
    assert_eq!(sb.selected_id(), Some("b"));
}

#[test]
fn test_sidebar_select_hovered_disabled() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A").disabled(true)]);
    sb.select_hovered();
    // Should not select disabled item
    assert!(sb.selected_id().is_none());
}

#[test]
fn test_sidebar_toggle_hovered() {
    let mut sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("parent", "Parent")
                .children(vec![SidebarItem::new("child", "Child")])]);
    assert_eq!(sb.visible_items().len(), 1);
    sb.toggle_hovered();
    assert_eq!(sb.visible_items().len(), 2);
    sb.toggle_hovered();
    assert_eq!(sb.visible_items().len(), 1);
}

#[test]
fn test_sidebar_toggle_item() {
    let mut sb =
        Sidebar::new()
            .items(vec![SidebarItem::new("folder", "Folder")
                .children(vec![SidebarItem::new("file", "File")])]);
    sb.toggle_item("folder");
    let items = sb.visible_items();
    assert_eq!(items.len(), 2);
}

#[test]
fn test_sidebar_expand_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A").children(vec![SidebarItem::new("a1", "A1")]),
        SidebarItem::new("b", "B").children(vec![SidebarItem::new("b1", "B1")]),
    ]);
    sb.expand_all();
    assert_eq!(sb.visible_items().len(), 4);
}

#[test]
fn test_sidebar_collapse_all() {
    let mut sb = Sidebar::new().items(vec![
        SidebarItem::new("a", "A")
            .expanded(true)
            .children(vec![SidebarItem::new("a1", "A1")]),
        SidebarItem::new("b", "B")
            .expanded(true)
            .children(vec![SidebarItem::new("b1", "B1")]),
    ]);
    assert_eq!(sb.visible_items().len(), 4);
    sb.collapse_all();
    assert_eq!(sb.visible_items().len(), 2);
}

#[test]
fn test_sidebar_toggle_collapse() {
    let mut sb = Sidebar::new().collapse_mode(CollapseMode::Expanded);
    assert!(!sb.is_collapsed());
    sb.toggle_collapse();
    assert!(sb.is_collapsed());
    sb.toggle_collapse();
    assert!(!sb.is_collapsed());
}

#[test]
fn test_sidebar_toggle_collapse_from_auto() {
    let mut sb = Sidebar::new().collapse_mode(CollapseMode::Auto);
    sb.toggle_collapse();
    assert!(sb.is_collapsed());
}

// =========================================================================
// FlattenedItem Tests
// =========================================================================

#[test]
fn test_flattened_item_section() {
    let flat = FlattenedItem::Section(Some("Title".to_string()));
    if let FlattenedItem::Section(title) = flat {
        assert_eq!(title, Some("Title".to_string()));
    } else {
        panic!("Expected Section");
    }
}

#[test]
fn test_flattened_item_item() {
    let flat = FlattenedItem::Item {
        item: SidebarItem::new("test", "Test"),
        depth: 2,
    };
    if let FlattenedItem::Item { item, depth } = flat {
        assert_eq!(item.id, "test");
        assert_eq!(depth, 2);
    } else {
        panic!("Expected Item");
    }
}

// =========================================================================
// Render Tests
// =========================================================================

#[test]
fn test_sidebar_render_basic() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("home", "Home").icon('🏠'),
        SidebarItem::new("settings", "Settings").icon('⚙'),
    ]);
    sb.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_sidebar_render_with_header() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .header("My App")
        .items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_footer() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .footer("v1.0.0")
        .items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_collapsed() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Collapsed)
        .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_sections() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .section(SidebarSection::titled(
            "Main",
            vec![SidebarItem::new("home", "Home")],
        ))
        .section(SidebarSection::titled(
            "Settings",
            vec![SidebarItem::new("prefs", "Preferences")],
        ));
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_nested() {
    let mut buffer = Buffer::new(30, 15);
    let area = Rect::new(0, 0, 30, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("folder", "Folder")
        .expanded(true)
        .children(vec![SidebarItem::new("file", "File")])]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_badges() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("inbox", "Inbox").badge("5")]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_disabled() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![
        SidebarItem::new("active", "Active"),
        SidebarItem::new("disabled", "Disabled").disabled(true),
    ]);
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_with_selected() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .items(vec![
            SidebarItem::new("a", "Item A"),
            SidebarItem::new("b", "Item B"),
        ])
        .selected("b");
    sb.render(&mut ctx);
}

#[test]
fn test_sidebar_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new().items(vec![SidebarItem::new("home", "Home")]);
    sb.render(&mut ctx); // Should handle gracefully
}

#[test]
fn test_sidebar_render_auto_collapse() {
    let mut buffer = Buffer::new(15, 10);
    let area = Rect::new(0, 0, 15, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let sb = Sidebar::new()
        .collapse_mode(CollapseMode::Auto)
        .collapse_threshold(20)
        .items(vec![SidebarItem::new("home", "Home").icon('🏠')]);
    // Width (15) < threshold (20), so should render collapsed
    sb.render(&mut ctx);
}

// =========================================================================
// Edge Cases
// =========================================================================

#[test]
fn test_sidebar_empty() {
    let sb = Sidebar::new();
    assert_eq!(sb.item_count(), 0);
    assert!(sb.visible_items().is_empty());
}

#[test]
fn test_sidebar_navigation_on_empty() {
    let mut sb = Sidebar::new();
    sb.hover_up();
    sb.hover_down();
    sb.select_hovered();
    // Should not panic
    assert_eq!(sb.hovered_index(), 0);
}

#[test]
fn test_sidebar_toggle_nonexistent_item() {
    let mut sb = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    sb.toggle_item("nonexistent");
    // Should not panic
}

#[test]
fn test_sidebar_deeply_nested() {
    let sb = Sidebar::new().items(vec![SidebarItem::new("l1", "Level 1")
        .expanded(true)
        .children(vec![SidebarItem::new("l2", "Level 2")
            .expanded(true)
            .children(vec![SidebarItem::new("l3", "Level 3")
                .expanded(true)
                .children(vec![SidebarItem::new("l4", "Level 4")])])])]);

    let items = sb.visible_items();
    assert_eq!(items.len(), 4);

    // Check depths
    if let FlattenedItem::Item { depth, .. } = &items[3] {
        assert_eq!(*depth, 3);
    }
}

}
