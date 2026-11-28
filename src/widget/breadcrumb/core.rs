//! Breadcrumb navigation widget - core implementation

use super::types::{BreadcrumbItem, SeparatorStyle};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::theme::{DARK_GRAY, LIGHT_GRAY};
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Breadcrumb widget
#[derive(Clone)]
pub struct Breadcrumb {
    /// Items in the breadcrumb
    items: Vec<BreadcrumbItem>,
    /// Selection state
    selection: Selection,
    /// Separator style
    separator: SeparatorStyle,
    /// Item color
    /// The color the builder named, if it named one.
    ///
    /// `None` is not the same as `Some(LIGHT_GRAY)`. While this was a plain
    /// `Color`, naming the default explicitly was indistinguishable from
    /// saying nothing, so a stylesheet outranked a builder that had spoken.
    item_color: Option<Color>,
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
            item_color: None,
            selected_color: Color::CYAN,
            separator_color: DARK_GRAY,
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
        self.item_color = Some(color);
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

    /// Enable/disable collapse mode: a trail wider than the area keeps its
    /// first item and last two, with one `…` for the items left out. When
    /// that is still too wide only the last item is kept after the `…`, and
    /// then the first item is shortened (ending with `…`) so the last item,
    /// where the user is, is drawn whole. The last item is cut at the edge
    /// only when even a one-column first item leaves it too little room.
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
            width += icon_columns(self.home_icon);
            if !self.items.is_empty() {
                width += 2; // separator + space
            }
        }

        for (i, item) in self.items.iter().enumerate() {
            if let Some(icon) = item.icon {
                width += icon_columns(icon);
            }
            width += crate::utils::display_width(&item.label) as u16;

            if i < self.items.len() - 1 {
                width += 3; // space + separator + space
            }
        }

        width
    }
}

/// The columns an icon takes with the space after it: two for a narrow
/// icon, three for a wide one (🏠, 📁)
fn icon_columns(icon: char) -> u16 {
    crate::utils::unicode::char_width(icon) as u16 + 1
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Breadcrumb {
    crate::impl_view_meta!("Breadcrumb");

    /// One row, as wide as the whole trail, capped at
    /// [`max_width`](Breadcrumb::max_width) when one is set.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let mut width = self.total_width();
        if self.max_width > 0 {
            width = width.min(self.max_width);
        }
        Some((width.min(max_width), 1.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        // The trail's own items take `color`; the selected one and the separator keep theirs.
        let item_color = self.item_color.unwrap_or_else(|| ctx.css_color(LIGHT_GRAY));
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

        let mut x: u16 = 0;

        // Home icon
        if self.show_home {
            let mut home = Cell::new(self.home_icon);
            home.fg = Some(item_color);
            ctx.set(x, 0, home);
            x += icon_columns(self.home_icon);

            if !self.items.is_empty() {
                let mut sep = Cell::new(self.separator.char());
                sep.fg = Some(self.separator_color);
                ctx.set(x, 0, sep);
                x += 2;
            }
        }

        // The entries drawn after the home icon: items, and `None` for the
        // one `…` cell standing for the items left out
        let (start, first_label) = if need_collapse {
            self.collapse_plan(usize::from(max_width))
        } else {
            (1, None)
        };
        let mut entries: Vec<Option<(usize, std::borrow::Cow<'_, str>)>> = Vec::new();
        if let Some(first) = self.items.first() {
            let label =
                first_label.map_or(std::borrow::Cow::Borrowed(first.label.as_str()), Into::into);
            entries.push(Some((0, label)));
            if start > 1 {
                entries.push(None);
            }
            for (i, item) in self.items.iter().enumerate().skip(start) {
                entries.push(Some((i, item.label.as_str().into())));
            }
        }

        let count = entries.len();
        for (k, entry) in entries.into_iter().enumerate() {
            if x >= max_width {
                break;
            }
            match entry {
                // The marker for the items left out, one column like Pagination's
                None => {
                    let mut ellipsis = Cell::new('…');
                    ellipsis.fg = Some(self.separator_color);
                    ctx.set(x, 0, ellipsis);
                    x += 1;
                }
                Some((idx, label)) => {
                    let item = &self.items[idx];
                    let is_selected = self.selection.is_selected(idx);
                    if let Some(icon) = item.icon {
                        let mut cell = Cell::new(icon);
                        cell.fg = Some(if is_selected {
                            self.selected_color
                        } else {
                            item_color
                        });
                        ctx.set(x, 0, cell);
                        x += icon_columns(icon);
                    }
                    let clip_width = max_width.saturating_sub(x);
                    if is_selected {
                        ctx.draw_text_clipped_bold(x, 0, &label, self.selected_color, clip_width);
                    } else {
                        ctx.draw_text_clipped(x, 0, &label, item_color, clip_width);
                    }
                    x += (crate::utils::display_width(&label) as u16).min(clip_width);
                }
            }

            // Separator (except after the last entry)
            if k + 1 < count && x.saturating_add(2) < max_width {
                x += 1;
                let mut sep = Cell::new(self.separator.char());
                sep.fg = Some(self.separator_color);
                ctx.set(x, 0, sep);
                x += 2;
            }
        }
    }
}

impl Breadcrumb {
    /// The columns an item takes: its icon with the space after it, and its
    /// label
    fn item_columns(item: &BreadcrumbItem) -> usize {
        item.icon.map_or(0, |icon| usize::from(icon_columns(icon)))
            + crate::utils::display_width(&item.label)
    }

    /// The width of the trail drawn as the first item, then (when `start`
    /// is past 1) one `…`, then the items from `start` - with the first
    /// item's label taking `first_label` columns
    fn trail_columns(&self, start: usize, first_label: usize) -> usize {
        let home = if self.show_home {
            usize::from(icon_columns(self.home_icon)) + 2
        } else {
            0
        };
        let first = self.items[0]
            .icon
            .map_or(0, |icon| usize::from(icon_columns(icon)));
        let ellipsis = if start > 1 { 4 } else { 0 }; // " › " and "…"
        let rest: usize = self.items[start..]
            .iter()
            .map(|item| 3 + Self::item_columns(item))
            .sum();
        home + first + first_label + ellipsis + rest
    }

    /// How a trail wider than `max_width` collapses: the index of the first
    /// item drawn after the first item and `…`, and the first item's label
    /// when it has to be shortened.
    ///
    /// The first item and the last two are kept; when they do not fit, only
    /// the last one. When the first item, `…` and the last item still do
    /// not fit, the first item is shortened (ending with `…`), so the last
    /// item - where the user is - stays whole. Only when even a one-column
    /// first item leaves too little room is the last item cut at the edge.
    fn collapse_plan(&self, max_width: usize) -> (usize, Option<String>) {
        let n = self.items.len();
        if n < 2 {
            return (1, None);
        }
        // Keeping the last two hides something only past three items
        let first = crate::utils::display_width(&self.items[0].label);
        if n > 3 && self.trail_columns(n - 2, first) <= max_width {
            return (n - 2, None);
        }
        // The last one alone; the first label gets the room the rest leaves
        // (all of it when it fits, unchanged)
        let start = if n > 2 { n - 1 } else { 1 };
        let room = max_width
            .saturating_sub(self.trail_columns(start, 0))
            .max(1);
        let label = crate::utils::unicode::truncate_with_ellipsis(&self.items[0].label, room);
        (start, (label != self.items[0].label).then_some(label))
    }
}

impl_styled_view!(Breadcrumb);
impl_props_builders!(Breadcrumb);
