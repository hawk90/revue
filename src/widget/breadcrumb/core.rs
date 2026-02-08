//! Breadcrumb navigation widget - core implementation

use super::types::{BreadcrumbItem, SeparatorStyle};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

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
    pub(crate) fn total_width(&self) -> u16 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event::Key;
    use crate::style::Color;

    // =========================================================================
    // Breadcrumb::new() and default tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_new() {
        let bc = Breadcrumb::new();
        assert!(bc.is_empty());
        assert_eq!(bc.len(), 0);
        assert_eq!(bc.selected(), 0);
        assert!(bc.show_home);
        assert!(bc.collapse);
    }

    #[test]
    fn test_breadcrumb_default() {
        let bc = Breadcrumb::default();
        assert!(bc.is_empty());
    }

    // =========================================================================
    // Breadcrumb::item() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_item() {
        let bc = Breadcrumb::new().item(BreadcrumbItem::new("Home"));
        assert_eq!(bc.len(), 1);
        assert_eq!(bc.selected(), 0);
    }

    #[test]
    fn test_breadcrumb_item_multiple() {
        let bc = Breadcrumb::new()
            .item(BreadcrumbItem::new("Home"))
            .item(BreadcrumbItem::new("Folder"))
            .item(BreadcrumbItem::new("File"));
        assert_eq!(bc.len(), 3);
        assert_eq!(bc.selected(), 2); // Last item selected
    }

    #[test]
    fn test_breadcrumb_item_with_icon() {
        let bc = Breadcrumb::new().item(BreadcrumbItem::new("Home").icon('🏠'));
        assert_eq!(bc.items[0].icon, Some('🏠'));
    }

    #[test]
    fn test_breadcrumb_item_not_clickable() {
        let bc = Breadcrumb::new().item(BreadcrumbItem::new("Locked").clickable(false));
        assert!(!bc.items[0].clickable);
    }

    // =========================================================================
    // Breadcrumb::push() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_push() {
        let bc = Breadcrumb::new().push("Home");
        assert_eq!(bc.len(), 1);
        assert_eq!(bc.items[0].label, "Home");
    }

    #[test]
    fn test_breadcrumb_push_multiple() {
        let bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        assert_eq!(bc.len(), 3);
    }

    #[test]
    fn test_breadcrumb_push_string() {
        let bc = Breadcrumb::new().push(String::from("Home"));
        assert_eq!(bc.items[0].label, "Home");
    }

    // =========================================================================
    // Breadcrumb::path() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_path() {
        let bc = Breadcrumb::new().path("Home/Folder/File");
        assert_eq!(bc.len(), 3);
        assert_eq!(bc.items[0].label, "Home");
        assert_eq!(bc.items[1].label, "Folder");
        assert_eq!(bc.items[2].label, "File");
    }

    #[test]
    fn test_breadcrumb_path_empty() {
        let bc = Breadcrumb::new().path("");
        assert!(bc.is_empty());
    }

    #[test]
    fn test_breadcrumb_path_with_empty_parts() {
        let bc = Breadcrumb::new().path("/Home//Folder/");
        assert_eq!(bc.len(), 2); // Empty parts filtered
    }

    // =========================================================================
    // Breadcrumb::separator() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_separator() {
        let bc = Breadcrumb::new().separator(SeparatorStyle::Arrow);
        assert_eq!(bc.separator, SeparatorStyle::Arrow);
    }

    #[test]
    fn test_breadcrumb_separator_default() {
        let bc = Breadcrumb::new();
        assert_eq!(bc.separator, SeparatorStyle::Chevron);
    }

    // =========================================================================
    // Breadcrumb color setters tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_item_color() {
        let bc = Breadcrumb::new().item_color(Color::RED);
        assert_eq!(bc.item_color, Color::RED);
    }

    #[test]
    fn test_breadcrumb_selected_color() {
        let bc = Breadcrumb::new().selected_color(Color::GREEN);
        assert_eq!(bc.selected_color, Color::GREEN);
    }

    #[test]
    fn test_breadcrumb_separator_color() {
        let bc = Breadcrumb::new().separator_color(Color::BLUE);
        assert_eq!(bc.separator_color, Color::BLUE);
    }

    // =========================================================================
    // Breadcrumb::home() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_home_true() {
        let bc = Breadcrumb::new().home(true);
        assert!(bc.show_home);
    }

    #[test]
    fn test_breadcrumb_home_false() {
        let bc = Breadcrumb::new().home(false);
        assert!(!bc.show_home);
    }

    #[test]
    fn test_breadcrumb_home_default() {
        let bc = Breadcrumb::new();
        assert!(bc.show_home);
    }

    // =========================================================================
    // Breadcrumb::home_icon() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_home_icon() {
        let bc = Breadcrumb::new().home_icon('⌂');
        assert_eq!(bc.home_icon, '⌂');
    }

    #[test]
    fn test_breadcrumb_home_icon_default() {
        let bc = Breadcrumb::new();
        assert_eq!(bc.home_icon, '🏠');
    }

    // =========================================================================
    // Breadcrumb::max_width() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_max_width() {
        let bc = Breadcrumb::new().max_width(50);
        assert_eq!(bc.max_width, 50);
    }

    #[test]
    fn test_breadcrumb_max_width_zero_no_limit() {
        let bc = Breadcrumb::new();
        assert_eq!(bc.max_width, 0);
    }

    // =========================================================================
    // Breadcrumb::collapse() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_collapse_true() {
        let bc = Breadcrumb::new().collapse(true);
        assert!(bc.collapse);
    }

    #[test]
    fn test_breadcrumb_collapse_false() {
        let bc = Breadcrumb::new().collapse(false);
        assert!(!bc.collapse);
    }

    // =========================================================================
    // Breadcrumb selection tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_selected() {
        let bc = Breadcrumb::new().push("Home").push("Folder");
        assert_eq!(bc.selected(), 1); // Last item
    }

    #[test]
    fn test_breadcrumb_set_selected() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        bc.set_selected(1);
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_selected_item() {
        let bc = Breadcrumb::new().push("Home").push("Folder");
        let item = bc.selected_item();
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "Folder");
    }

    #[test]
    fn test_breadcrumb_selected_item_none() {
        let bc = Breadcrumb::new();
        assert!(bc.selected_item().is_none());
    }

    // =========================================================================
    // Breadcrumb::select_next() / select_prev() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_select_next() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        bc.set_selected(0);
        bc.select_next();
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_select_next_at_end() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder");
        bc.select_next(); // Already at end
        assert_eq!(bc.selected(), 1); // No change (no wrap)
    }

    #[test]
    fn test_breadcrumb_select_prev() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        bc.set_selected(2);
        bc.select_prev();
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_select_prev_at_start() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder");
        bc.set_selected(0);
        bc.select_prev();
        assert_eq!(bc.selected(), 0); // No change (no wrap)
    }

    // =========================================================================
    // Breadcrumb::handle_key() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_handle_key_left() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        bc.set_selected(2);
        assert!(bc.handle_key(&Key::Left));
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_handle_key_right() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        bc.set_selected(0);
        assert!(bc.handle_key(&Key::Right));
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_handle_key_h() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder");
        bc.set_selected(1);
        assert!(bc.handle_key(&Key::Char('h')));
        assert_eq!(bc.selected(), 0);
    }

    #[test]
    fn test_breadcrumb_handle_key_l() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder");
        bc.set_selected(0);
        assert!(bc.handle_key(&Key::Char('l')));
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_handle_key_unhandled() {
        let mut bc = Breadcrumb::new().push("Home");
        assert!(!bc.handle_key(&Key::Char('x')));
        assert_eq!(bc.selected(), 0);
    }

    // =========================================================================
    // Breadcrumb::pop() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_pop() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        let item = bc.pop();
        assert!(item.is_some());
        assert_eq!(item.unwrap().label, "File");
        assert_eq!(bc.len(), 2);
    }

    #[test]
    fn test_breadcrumb_pop_empty() {
        let mut bc = Breadcrumb::new();
        assert!(bc.pop().is_none());
    }

    // =========================================================================
    // Breadcrumb::navigate_to() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_navigate_to() {
        let mut bc = Breadcrumb::new()
            .push("Home")
            .push("Folder")
            .push("Subfolder")
            .push("File");
        bc.navigate_to(1);
        assert_eq!(bc.len(), 2); // Truncated to Home + Folder
        assert_eq!(bc.selected(), 1);
    }

    #[test]
    fn test_breadcrumb_navigate_to_last() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        bc.navigate_to(2);
        assert_eq!(bc.len(), 3); // No truncation
        assert_eq!(bc.selected(), 2);
    }

    #[test]
    fn test_breadcrumb_navigate_to_out_of_bounds() {
        let mut bc = Breadcrumb::new().push("Home").push("Folder");
        bc.navigate_to(5);
        assert_eq!(bc.len(), 2); // No change
    }

    // =========================================================================
    // Breadcrumb::path_string() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_path_string() {
        let bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        assert_eq!(bc.path_string(), "Home/Folder/File");
    }

    #[test]
    fn test_breadcrumb_path_string_empty() {
        let bc = Breadcrumb::new();
        assert_eq!(bc.path_string(), "");
    }

    #[test]
    fn test_breadcrumb_path_string_single() {
        let bc = Breadcrumb::new().push("Home");
        assert_eq!(bc.path_string(), "Home");
    }

    // =========================================================================
    // Breadcrumb::len() / is_empty() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_len_empty() {
        let bc = Breadcrumb::new();
        assert_eq!(bc.len(), 0);
    }

    #[test]
    fn test_breadcrumb_len_multiple() {
        let bc = Breadcrumb::new().push("Home").push("Folder").push("File");
        assert_eq!(bc.len(), 3);
    }

    #[test]
    fn test_breadcrumb_is_empty_true() {
        let bc = Breadcrumb::new();
        assert!(bc.is_empty());
    }

    #[test]
    fn test_breadcrumb_is_empty_false() {
        let bc = Breadcrumb::new().push("Home");
        assert!(!bc.is_empty());
    }

    // =========================================================================
    // Breadcrumb::total_width() tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_total_width_empty() {
        let bc = Breadcrumb::new();
        // Home icon
        assert_eq!(bc.total_width(), 2);
    }

    #[test]
    fn test_breadcrumb_total_width_no_home() {
        let bc = Breadcrumb::new().home(false).push("Home");
        assert_eq!(bc.total_width(), 4); // "Home" = 4 chars
    }

    #[test]
    fn test_breadcrumb_total_width_with_separator() {
        let bc = Breadcrumb::new().home(false).push("Home").push("Folder");
        // Home(4) + sep(3) + Folder(6) = 13
        assert_eq!(bc.total_width(), 13);
    }

    #[test]
    fn test_breadcrumb_total_width_with_icon() {
        let bc = Breadcrumb::new()
            .home(false)
            .item(BreadcrumbItem::new("Home").icon('🏠'));
        // icon(2) + Home(4) = 6
        assert_eq!(bc.total_width(), 6);
    }

    // =========================================================================
    // Breadcrumb builder chain tests
    // =========================================================================

    #[test]
    fn test_breadcrumb_builder_chain() {
        let bc = Breadcrumb::new()
            .push("Home")
            .push("Folder")
            .separator(SeparatorStyle::Arrow)
            .item_color(Color::WHITE)
            .selected_color(Color::CYAN)
            .separator_color(Color::rgb(128, 128, 128))
            .home(false)
            .max_width(80)
            .collapse(false);

        assert_eq!(bc.len(), 2);
        assert_eq!(bc.separator, SeparatorStyle::Arrow);
        assert_eq!(bc.item_color, Color::WHITE);
        assert_eq!(bc.selected_color, Color::CYAN);
        assert!(!bc.show_home);
        assert_eq!(bc.max_width, 80);
        assert!(!bc.collapse);
    }
}
