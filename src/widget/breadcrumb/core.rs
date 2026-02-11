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

    /// Get items (for testing)
    #[doc(hidden)]
    pub fn items(&self) -> &[BreadcrumbItem] {
        &self.items
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
