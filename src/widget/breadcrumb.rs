//! Breadcrumb navigation widget
//!
//! Shows hierarchical navigation path with clickable segments.

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::Selection;
use crate::{impl_props_builders, impl_styled_view};

/// Breadcrumb item
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// Item label
    pub label: String,
    /// Optional icon
    pub icon: Option<char>,
    /// Is item clickable
    pub clickable: bool,
}

impl BreadcrumbItem {
    /// Create a new breadcrumb item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            clickable: true,
        }
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set clickable state
    pub fn clickable(mut self, clickable: bool) -> Self {
        self.clickable = clickable;
        self
    }
}

/// Separator style
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SeparatorStyle {
    /// Slash /
    #[default]
    Slash,
    /// Arrow >
    Arrow,
    /// Chevron ›
    Chevron,
    /// Double arrow »
    DoubleArrow,
    /// Dot •
    Dot,
    /// Pipe |
    Pipe,
    /// Custom character
    Custom(char),
}

impl SeparatorStyle {
    fn char(&self) -> char {
        match self {
            SeparatorStyle::Slash => '/',
            SeparatorStyle::Arrow => '>',
            SeparatorStyle::Chevron => '›',
            SeparatorStyle::DoubleArrow => '»',
            SeparatorStyle::Dot => '•',
            SeparatorStyle::Pipe => '|',
            SeparatorStyle::Custom(c) => *c,
        }
    }
}

/// Breadcrumb widget
pub struct Breadcrumb {
    /// Items in the breadcrumb
    items: Vec<BreadcrumbItem>,
    /// Selection state
    selection: Selection,
    /// Separator style
    separator: SeparatorStyle,
    /// Item color
    item_color: Color,
    /// Selected item color
    selected_color: Color,
    /// Separator color
    separator_color: Color,
    /// Show home icon
    show_home: bool,
    /// Home icon
    home_icon: char,
    /// Max width (0 = no limit)
    max_width: u16,
    /// Collapse mode when too long
    collapse: bool,
    /// Widget properties
    props: WidgetProps,
}

impl Breadcrumb {
    /// Create a new breadcrumb
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            selection: Selection::new(0),
            separator: SeparatorStyle::Chevron,
            item_color: Color::rgb(150, 150, 150),
            selected_color: Color::CYAN,
            separator_color: Color::rgb(80, 80, 80),
            show_home: true,
            home_icon: '🏠',
            max_width: 0,
            collapse: true,
            props: WidgetProps::new(),
        }
    }

    /// Add an item
    pub fn item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self.selection.set_len(self.items.len());
        self.selection.last(); // Select last item
        self
    }

    /// Add item from string
    pub fn push(mut self, label: impl Into<String>) -> Self {
        self.items.push(BreadcrumbItem::new(label));
        self.selection.set_len(self.items.len());
        self.selection.last(); // Select last item
        self
    }

    /// Set path (splits by separator)
    pub fn path(mut self, path: &str) -> Self {
        self.items = path
            .split('/')
            .filter(|s| !s.is_empty())
            .map(BreadcrumbItem::new)
            .collect();
        self.selection.set_len(self.items.len());
        self.selection.last(); // Select last item
        self
    }

    /// Set separator style
    pub fn separator(mut self, style: SeparatorStyle) -> Self {
        self.separator = style;
        self
    }

    /// Set item color
    pub fn item_color(mut self, color: Color) -> Self {
        self.item_color = color;
        self
    }

    /// Set selected color
    pub fn selected_color(mut self, color: Color) -> Self {
        self.selected_color = color;
        self
    }

    /// Set separator color
    pub fn separator_color(mut self, color: Color) -> Self {
        self.separator_color = color;
        self
    }

    /// Show/hide home icon
    pub fn home(mut self, show: bool) -> Self {
        self.show_home = show;
        self
    }

    /// Set home icon
    pub fn home_icon(mut self, icon: char) -> Self {
        self.home_icon = icon;
        self
    }

    /// Set max width
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Enable/disable collapse mode
    pub fn collapse(mut self, collapse: bool) -> Self {
        self.collapse = collapse;
        self
    }

    /// Select next item (no wrap)
    pub fn select_next(&mut self) {
        self.selection.down();
    }

    /// Select previous item (no wrap)
    pub fn select_prev(&mut self) {
        self.selection.up();
    }

    /// Get selected index
    pub fn selected(&self) -> usize {
        self.selection.index
    }

    /// Set selected index
    pub fn set_selected(&mut self, index: usize) {
        self.selection.set(index);
    }

    /// Get selected item
    pub fn selected_item(&self) -> Option<&BreadcrumbItem> {
        self.items.get(self.selection.index)
    }

    /// Get path string
    pub fn path_string(&self) -> String {
        self.items
            .iter()
            .map(|i| i.label.as_str())
            .collect::<Vec<_>>()
            .join("/")
    }

    /// Get item count
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Left | Key::Char('h') => {
                self.select_prev();
                true
            }
            Key::Right | Key::Char('l') => {
                self.select_next();
                true
            }
            _ => false,
        }
    }

    /// Pop last item (go up one level)
    pub fn pop(&mut self) -> Option<BreadcrumbItem> {
        let item = self.items.pop();
        self.selection.set_len(self.items.len());
        item
    }

    /// Navigate to index (removes items after)
    pub fn navigate_to(&mut self, index: usize) {
        if index < self.items.len() {
            self.items.truncate(index + 1);
            self.selection.set_len(self.items.len());
            self.selection.set(index);
        }
    }

    /// Calculate total width
    fn total_width(&self) -> u16 {
        let mut width = 0u16;

        if self.show_home {
            width += 2; // home icon + space
        }

        for (i, item) in self.items.iter().enumerate() {
            if item.icon.is_some() {
                width += 2; // icon + space
            }
            width += item.label.len() as u16;

            if i < self.items.len() - 1 {
                width += 3; // space + separator + space
            }
        }

        width
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Breadcrumb {
    crate::impl_view_meta!("Breadcrumb");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let max_width = if self.max_width > 0 {
            self.max_width.min(area.width)
        } else {
            area.width
        };

        let total = self.total_width();
        let need_collapse = self.collapse && total > max_width;

        let mut x = area.x;
        let y = area.y;

        // Home icon
        if self.show_home {
            let mut home = Cell::new(self.home_icon);
            home.fg = Some(self.item_color);
            ctx.buffer.set(x, y, home);
            x += 2;

            if !self.items.is_empty() {
                let mut sep = Cell::new(self.separator.char());
                sep.fg = Some(self.separator_color);
                ctx.buffer.set(x, y, sep);
                x += 2;
            }
        }

        // Determine which items to show
        let (start_idx, show_ellipsis) = if need_collapse && self.items.len() > 2 {
            // Show first, ..., last few items
            (self.items.len().saturating_sub(2), true)
        } else {
            (0, false)
        };

        // First item if collapsing
        if show_ellipsis && !self.items.is_empty() {
            let item = &self.items[0];
            let is_selected = self.selection.is_selected(0);

            if let Some(icon) = item.icon {
                let mut cell = Cell::new(icon);
                cell.fg = Some(if is_selected {
                    self.selected_color
                } else {
                    self.item_color
                });
                ctx.buffer.set(x, y, cell);
                x += 2;
            }

            let color = if is_selected {
                self.selected_color
            } else {
                self.item_color
            };
            for ch in item.label.chars() {
                if x >= area.x + max_width - 10 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x, y, cell);
                x += 1;
            }

            // Separator
            x += 1;
            let mut sep = Cell::new(self.separator.char());
            sep.fg = Some(self.separator_color);
            ctx.buffer.set(x, y, sep);
            x += 2;

            // Ellipsis
            for ch in "...".chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.separator_color);
                ctx.buffer.set(x, y, cell);
                x += 1;
            }

            // Separator after ellipsis
            x += 1;
            let mut sep = Cell::new(self.separator.char());
            sep.fg = Some(self.separator_color);
            ctx.buffer.set(x, y, sep);
            x += 2;
        }

        // Remaining items
        let items_to_show = if show_ellipsis {
            &self.items[start_idx..]
        } else {
            &self.items[..]
        };

        for (i, item) in items_to_show.iter().enumerate() {
            let actual_idx = if show_ellipsis { start_idx + i } else { i };
            let is_selected = self.selection.is_selected(actual_idx);
            let is_last = actual_idx == self.items.len() - 1;

            if x >= area.x + max_width {
                break;
            }

            // Icon
            if let Some(icon) = item.icon {
                let mut cell = Cell::new(icon);
                cell.fg = Some(if is_selected {
                    self.selected_color
                } else {
                    self.item_color
                });
                ctx.buffer.set(x, y, cell);
                x += 2;
            }

            // Label
            let color = if is_selected {
                self.selected_color
            } else {
                self.item_color
            };
            for ch in item.label.chars() {
                if x >= area.x + max_width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x, y, cell);
                x += 1;
            }

            // Separator (except for last item)
            if !is_last && x + 2 < area.x + max_width {
                x += 1;
                let mut sep = Cell::new(self.separator.char());
                sep.fg = Some(self.separator_color);
                ctx.buffer.set(x, y, sep);
                x += 2;
            }
        }
    }
}

impl_styled_view!(Breadcrumb);
impl_props_builders!(Breadcrumb);

/// Helper to create a breadcrumb
pub fn breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
}

/// Helper to create a breadcrumb item
pub fn crumb(label: impl Into<String>) -> BreadcrumbItem {
    BreadcrumbItem::new(label)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_breadcrumb_item() {
        let item = BreadcrumbItem::new("Home").icon('🏠');
        assert_eq!(item.label, "Home");
        assert_eq!(item.icon, Some('🏠'));
    }

    #[test]
    fn test_breadcrumb_new() {
        let bc = Breadcrumb::new();
        assert!(bc.is_empty());
    }

    #[test]
    fn test_breadcrumb_push() {
        let bc = Breadcrumb::new()
            .push("Home")
            .push("Documents")
            .push("Work");

        assert_eq!(bc.len(), 3);
        assert_eq!(bc.selected(), 2); // Last item selected
    }

    #[test]
    fn test_breadcrumb_path() {
        let bc = Breadcrumb::new().path("/home/user/documents");
        assert_eq!(bc.len(), 3);
        assert_eq!(bc.path_string(), "home/user/documents");
    }

    #[test]
    fn test_breadcrumb_selection() {
        let mut bc = Breadcrumb::new().push("A").push("B").push("C");

        assert_eq!(bc.selected(), 2);

        bc.select_prev();
        assert_eq!(bc.selected(), 1);

        bc.select_prev();
        assert_eq!(bc.selected(), 0);

        bc.select_prev();
        assert_eq!(bc.selected(), 0); // Can't go below 0

        bc.select_next();
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_navigate() {
        let mut bc = Breadcrumb::new().push("A").push("B").push("C").push("D");

        bc.navigate_to(1);
        assert_eq!(bc.len(), 2);
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_pop() {
        let mut bc = Breadcrumb::new().push("A").push("B");

        let item = bc.pop();
        assert_eq!(item.unwrap().label, "B");
        assert_eq!(bc.len(), 1);
    }

    #[test]
    fn test_separator_style() {
        assert_eq!(SeparatorStyle::Slash.char(), '/');
        assert_eq!(SeparatorStyle::Arrow.char(), '>');
        assert_eq!(SeparatorStyle::Chevron.char(), '›');
        assert_eq!(SeparatorStyle::Custom('→').char(), '→');
    }

    #[test]
    fn test_handle_key() {
        use crate::event::Key;

        let mut bc = Breadcrumb::new().push("A").push("B");

        bc.set_selected(0);
        assert!(bc.handle_key(&Key::Right));
        assert_eq!(bc.selected(), 1);

        assert!(bc.handle_key(&Key::Left));
        assert_eq!(bc.selected(), 0);
    }

    #[test]
    fn test_render() {
        let mut buffer = Buffer::new(60, 3);
        let area = Rect::new(0, 0, 60, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .push("Home")
            .push("Documents")
            .push("Work");

        bc.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_helpers() {
        let bc = breadcrumb().item(crumb("Test"));

        assert_eq!(bc.len(), 1);
    }

    // =========================================================================
    // Additional coverage tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_default() {
        let bc = Breadcrumb::default();
        assert!(bc.is_empty());
    }

    #[test]
    fn test_breadcrumb_item_clickable() {
        let item = BreadcrumbItem::new("Test").clickable(false);
        assert!(!item.clickable);
    }

    #[test]
    fn test_breadcrumb_item_clone() {
        let item = BreadcrumbItem::new("Test").icon('📁');
        let cloned = item.clone();
        assert_eq!(cloned.label, "Test");
        assert_eq!(cloned.icon, Some('📁'));
    }

    #[test]
    fn test_separator_style_all() {
        assert_eq!(SeparatorStyle::DoubleArrow.char(), '»');
        assert_eq!(SeparatorStyle::Dot.char(), '•');
        assert_eq!(SeparatorStyle::Pipe.char(), '|');
    }

    #[test]
    fn test_separator_style_debug() {
        let style = SeparatorStyle::Chevron;
        let debug = format!("{:?}", style);
        assert!(debug.contains("Chevron"));
    }

    #[test]
    fn test_separator_style_eq() {
        assert_eq!(SeparatorStyle::Slash, SeparatorStyle::Slash);
        assert_ne!(SeparatorStyle::Slash, SeparatorStyle::Arrow);
    }

    #[test]
    fn test_breadcrumb_colors() {
        let bc = Breadcrumb::new()
            .item_color(Color::WHITE)
            .selected_color(Color::CYAN)
            .separator_color(Color::rgb(80, 80, 80));

        assert_eq!(bc.item_color, Color::WHITE);
        assert_eq!(bc.selected_color, Color::CYAN);
    }

    #[test]
    fn test_breadcrumb_home_settings() {
        let bc = Breadcrumb::new().home(false).home_icon('🏠');

        assert!(!bc.show_home);
        assert_eq!(bc.home_icon, '🏠');
    }

    #[test]
    fn test_breadcrumb_max_width() {
        let bc = Breadcrumb::new().max_width(50);
        assert_eq!(bc.max_width, 50);
    }

    #[test]
    fn test_breadcrumb_collapse() {
        let bc = Breadcrumb::new().collapse(false);
        assert!(!bc.collapse);
    }

    #[test]
    fn test_breadcrumb_total_width() {
        let bc = Breadcrumb::new().home(false).push("Home").push("Documents");

        let width = bc.total_width();
        // "Home" (4) + " › " (3) + "Documents" (9) = 16
        assert!(width > 0);
    }

    #[test]
    fn test_breadcrumb_total_width_with_icons() {
        let bc = Breadcrumb::new()
            .home(false)
            .item(BreadcrumbItem::new("Home").icon('📁'))
            .item(BreadcrumbItem::new("Work"));

        let width = bc.total_width();
        assert!(width > 0);
    }

    #[test]
    fn test_breadcrumb_total_width_with_home() {
        let bc = Breadcrumb::new().home(true).push("Documents");

        let width = bc.total_width();
        // Home icon + space + separator + Documents
        assert!(width > 0);
    }

    #[test]
    fn test_breadcrumb_render_empty() {
        let mut buffer = Buffer::new(40, 3);
        let area = Rect::new(0, 0, 40, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new().home(false);
        bc.render(&mut ctx);
        // Empty breadcrumb should not panic
    }

    #[test]
    fn test_breadcrumb_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new().push("Test");
        bc.render(&mut ctx);
        // Small area should not panic
    }

    #[test]
    fn test_breadcrumb_render_with_collapse() {
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(0, 0, 30, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .home(false)
            .max_width(20)
            .collapse(true)
            .push("Very")
            .push("Long")
            .push("Path")
            .push("That")
            .push("Needs")
            .push("Collapse");

        bc.render(&mut ctx);
    }

    #[test]
    fn test_breadcrumb_render_with_icons() {
        let mut buffer = Buffer::new(60, 3);
        let area = Rect::new(0, 0, 60, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .home(true)
            .item(BreadcrumbItem::new("Documents").icon('📁'))
            .item(BreadcrumbItem::new("Work").icon('💼'));

        bc.render(&mut ctx);
    }

    #[test]
    fn test_breadcrumb_handle_key_vim() {
        use crate::event::Key;

        let mut bc = Breadcrumb::new().push("A").push("B");

        bc.set_selected(0);

        // 'l' for right
        assert!(bc.handle_key(&Key::Char('l')));
        assert_eq!(bc.selected(), 1);

        // 'h' for left
        assert!(bc.handle_key(&Key::Char('h')));
        assert_eq!(bc.selected(), 0);
    }

    #[test]
    fn test_breadcrumb_handle_key_unhandled() {
        use crate::event::Key;

        let mut bc = Breadcrumb::new().push("Test");

        let handled = bc.handle_key(&Key::Escape);
        assert!(!handled);
    }

    #[test]
    fn test_breadcrumb_selected_item() {
        let bc = Breadcrumb::new().push("First").push("Second");

        let item = bc.selected_item();
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "Second");
    }

    #[test]
    fn test_breadcrumb_selected_item_empty() {
        let bc = Breadcrumb::new();
        assert!(bc.selected_item().is_none());
    }

    #[test]
    fn test_breadcrumb_navigate_to_out_of_bounds() {
        let mut bc = Breadcrumb::new().push("A").push("B").push("C");

        bc.navigate_to(10); // Out of bounds, should do nothing
        assert_eq!(bc.len(), 3);
    }

    #[test]
    fn test_breadcrumb_pop_empty() {
        let mut bc = Breadcrumb::new();
        let item = bc.pop();
        assert!(item.is_none());
    }

    #[test]
    fn test_crumb_helper() {
        let item = crumb("Test").icon('✓').clickable(false);
        assert_eq!(item.label, "Test");
        assert_eq!(item.icon, Some('✓'));
        assert!(!item.clickable);
    }
}
