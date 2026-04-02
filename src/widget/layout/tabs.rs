//! Tabs widget for tabbed navigation

use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Tab item
#[derive(Clone)]
pub struct Tab {
    /// Tab label
    pub label: String,
}

impl Tab {
    /// Create a new tab
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

/// Tabs widget for tabbed navigation
pub struct Tabs {
    tabs: Vec<Tab>,
    selection: Selection,
    fg: Option<Color>,
    bg: Option<Color>,
    active_fg: Option<Color>,
    active_bg: Option<Color>,
    divider: char,
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    props: WidgetProps,
}

impl Tabs {
    /// Create a new tabs widget
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            selection: Selection::new(0),
            fg: None,
            bg: None,
            active_fg: Some(Color::rgb(220, 220, 220)),
            active_bg: Some(Color::BLUE),
            divider: '│',
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }

    /// Set tabs
    pub fn tabs(mut self, tabs: Vec<impl Into<String>>) -> Self {
        self.tabs = tabs.into_iter().map(|t| Tab::new(t)).collect();
        self.selection.set_len(self.tabs.len());
        self
    }

    /// Add a tab
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(Tab::new(label));
        self.selection.set_len(self.tabs.len());
        self
    }

    /// Set selected tab index
    pub fn selected(mut self, index: usize) -> Self {
        self.selection.set(index);
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

    /// Set active tab colors
    pub fn active_style(mut self, fg: Color, bg: Color) -> Self {
        self.active_fg = Some(fg);
        self.active_bg = Some(bg);
        self
    }

    /// Set divider character
    pub fn divider(mut self, ch: char) -> Self {
        self.divider = ch;
        self
    }

    /// Set minimum width constraint
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set minimum height constraint
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set maximum width constraint (0 = no limit)
    pub fn max_width(mut self, width: u16) -> Self {
        self.max_width = width;
        self
    }

    /// Set maximum height constraint (0 = no limit)
    pub fn max_height(mut self, height: u16) -> Self {
        self.max_height = height;
        self
    }

    /// Set both min width and height
    pub fn min_size(self, width: u16, height: u16) -> Self {
        self.min_width(width).min_height(height)
    }

    /// Set both max width and height (0 = no limit)
    pub fn max_size(self, width: u16, height: u16) -> Self {
        self.max_width(width).max_height(height)
    }

    /// Set all size constraints at once
    pub fn constrain(self, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Self {
        self.min_width(min_w)
            .min_height(min_h)
            .max_width(max_w)
            .max_height(max_h)
    }

    /// Apply size constraints to the available area
    fn apply_constraints(&self, area: Rect) -> Rect {
        let eff_max_w = if self.max_width > 0 {
            self.max_width.max(self.min_width)
        } else {
            u16::MAX
        };
        let eff_max_h = if self.max_height > 0 {
            self.max_height.max(self.min_height)
        } else {
            u16::MAX
        };
        let width = area.width.clamp(self.min_width, eff_max_w);
        let height = area.height.clamp(self.min_height, eff_max_h);

        Rect::new(area.x, area.y, width, height)
    }

    /// Get selected tab index
    pub fn selected_index(&self) -> usize {
        self.selection.index
    }

    /// Get selected tab label
    pub fn selected_label(&self) -> Option<&str> {
        self.tabs
            .get(self.selection.index)
            .map(|t| t.label.as_str())
    }

    /// Select next tab (wraps around)
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous tab (wraps around)
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Select first tab
    pub fn select_first(&mut self) {
        self.selection.first();
    }

    /// Select last tab
    pub fn select_last(&mut self) {
        self.selection.last();
    }

    /// Select tab by index
    pub fn select(&mut self, index: usize) {
        self.selection.set(index);
    }

    /// Handle key input, returns true if selection changed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Left | Key::Char('h') => {
                let old = self.selection.index;
                self.select_prev();
                old != self.selection.index
            }
            Key::Right | Key::Char('l') => {
                let old = self.selection.index;
                self.select_next();
                old != self.selection.index
            }
            Key::Home => {
                let old = self.selection.index;
                self.select_first();
                old != self.selection.index
            }
            Key::End => {
                let old = self.selection.index;
                self.select_last();
                old != self.selection.index
            }
            Key::Char(c) if c.is_ascii_digit() => {
                let index = (*c as usize) - ('1' as usize);
                if index < self.tabs.len() {
                    let old = self.selection.index;
                    self.selection.index = index;
                    old != self.selection.index
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Get number of tabs
    pub fn len(&self) -> usize {
        self.tabs.len()
    }

    /// Check if tabs is empty
    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Tabs {
    fn render(&self, ctx: &mut RenderContext) {
        let area = self.apply_constraints(ctx.area);
        if area.width < 3 || area.height < 1 || self.tabs.is_empty() {
            return;
        }

        let mut x: u16 = 0;

        for (i, tab) in self.tabs.iter().enumerate() {
            let is_active = i == self.selection.index;
            let (fg, bg) = if is_active {
                (self.active_fg, self.active_bg)
            } else {
                (self.fg, self.bg)
            };

            // Draw padding
            let mut cell = Cell::new(' ');
            cell.fg = fg;
            cell.bg = bg;
            if x < area.width {
                ctx.set(x, 0, cell);
                x += 1;
            }

            // Draw label
            let clip = area.width.saturating_sub(x);
            if let Some(f) = fg {
                if is_active {
                    if let Some(b) = bg {
                        ctx.draw_text_clipped_bg_bold(x, 0, &tab.label, f, b, clip);
                    } else {
                        ctx.draw_text_clipped_bold(x, 0, &tab.label, f, clip);
                    }
                } else if let Some(b) = bg {
                    ctx.draw_text_clipped_bg(x, 0, &tab.label, f, b, clip);
                } else {
                    ctx.draw_text_clipped(x, 0, &tab.label, f, clip);
                }
                x += (crate::utils::display_width(&tab.label) as u16).min(clip);
            } else {
                // fg is None: preserve terminal default color
                for ch in tab.label.chars() {
                    let cw = crate::utils::char_width(ch) as u16;
                    if x + cw > area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = fg;
                    cell.bg = bg;
                    if is_active {
                        cell.modifier |= crate::render::Modifier::BOLD;
                    }
                    ctx.set(x, 0, cell);
                    x += cw;
                }
            }

            // Draw padding
            if x < area.width {
                let mut cell = Cell::new(' ');
                cell.fg = fg;
                cell.bg = bg;
                ctx.set(x, 0, cell);
                x += 1;
            }

            // Draw divider (unless last tab)
            if i < self.tabs.len() - 1 && x < area.width {
                let mut cell = Cell::new(self.divider);
                cell.fg = self.fg;
                ctx.set(x, 0, cell);
                x += 1;
            }
        }
    }

    crate::impl_view_meta!("Tabs");
}

/// Helper function to create tabs
pub fn tabs() -> Tabs {
    Tabs::new()
}

impl_styled_view!(Tabs);
impl_props_builders!(Tabs);

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;

    #[test]
    fn test_tabs_new_empty() {
        let t = Tabs::new();
        assert!(t.is_empty());
        assert_eq!(t.len(), 0);
        assert_eq!(t.selected_index(), 0);
        assert!(t.selected_label().is_none());
    }

    #[test]
    fn test_tabs_add_tabs() {
        let t = Tabs::new().tab("One").tab("Two").tab("Three");
        assert_eq!(t.len(), 3);
        assert_eq!(t.selected_label(), Some("One"));
    }

    #[test]
    fn test_tabs_from_vec() {
        let t = Tabs::new().tabs(vec!["A", "B", "C"]);
        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_tabs_selected() {
        let t = Tabs::new().tab("A").tab("B").tab("C").selected(2);
        assert_eq!(t.selected_index(), 2);
        assert_eq!(t.selected_label(), Some("C"));
    }

    #[test]
    fn test_tabs_navigation() {
        let mut t = Tabs::new().tab("A").tab("B").tab("C");
        assert_eq!(t.selected_index(), 0);

        t.select_next();
        assert_eq!(t.selected_index(), 1);

        t.select_next();
        assert_eq!(t.selected_index(), 2);

        t.select_next(); // wraps
        assert_eq!(t.selected_index(), 0);

        t.select_prev(); // wraps back
        assert_eq!(t.selected_index(), 2);
    }

    #[test]
    fn test_tabs_select_first_last() {
        let mut t = Tabs::new().tab("A").tab("B").tab("C").selected(1);
        t.select_first();
        assert_eq!(t.selected_index(), 0);

        t.select_last();
        assert_eq!(t.selected_index(), 2);
    }

    #[test]
    fn test_tabs_handle_key() {
        use crate::event::Key;

        let mut t = Tabs::new().tab("A").tab("B").tab("C");

        assert!(t.handle_key(&Key::Right));
        assert_eq!(t.selected_index(), 1);

        assert!(t.handle_key(&Key::Left));
        assert_eq!(t.selected_index(), 0);

        assert!(t.handle_key(&Key::End));
        assert_eq!(t.selected_index(), 2);

        assert!(t.handle_key(&Key::Home));
        assert_eq!(t.selected_index(), 0);

        // Digit key selects tab (1-indexed)
        assert!(t.handle_key(&Key::Char('2')));
        assert_eq!(t.selected_index(), 1);

        // Unknown key returns false
        assert!(!t.handle_key(&Key::Char('x')));
    }

    #[test]
    fn test_tabs_render_empty_no_panic() {
        let mut buf = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buf, area);
        let t = Tabs::new();
        t.render(&mut ctx);
    }

    #[test]
    fn test_tabs_render_small_area_no_panic() {
        let mut buf = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buf, area);
        let t = Tabs::new().tab("Tab1").tab("Tab2");
        t.render(&mut ctx); // Width < 3, should return early
    }

    #[test]
    fn test_tabs_default() {
        let t = Tabs::default();
        assert!(t.is_empty());
    }

    #[test]
    fn test_tabs_helper_fn() {
        let t = tabs().tab("A");
        assert_eq!(t.len(), 1);
    }
}
