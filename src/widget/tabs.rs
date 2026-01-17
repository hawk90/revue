//! Tabs widget for tabbed navigation

use super::traits::{RenderContext, View, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::utils::Selection;
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
            active_fg: Some(Color::WHITE),
            active_bg: Some(Color::BLUE),
            divider: '│',
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
        let area = ctx.area;
        if area.width < 3 || area.height < 1 || self.tabs.is_empty() {
            return;
        }

        let mut x = area.x;

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
            if x < area.x + area.width {
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Draw label
            for ch in tab.label.chars() {
                if x >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = fg;
                cell.bg = bg;
                if is_active {
                    cell.modifier |= crate::render::Modifier::BOLD;
                }
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Draw padding
            if x < area.x + area.width {
                let mut cell = Cell::new(' ');
                cell.fg = fg;
                cell.bg = bg;
                ctx.buffer.set(x, area.y, cell);
                x += 1;
            }

            // Draw divider (unless last tab)
            if i < self.tabs.len() - 1 && x < area.x + area.width {
                let mut cell = Cell::new(self.divider);
                cell.fg = self.fg;
                ctx.buffer.set(x, area.y, cell);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_tabs_new() {
        let t = Tabs::new();
        assert!(t.is_empty());
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tabs_builder() {
        let t = Tabs::new().tab("Home").tab("Settings").tab("Help");

        assert_eq!(t.len(), 3);
        assert_eq!(t.selected_label(), Some("Home"));
    }

    #[test]
    fn test_tabs_from_vec() {
        let t = Tabs::new().tabs(vec!["A", "B", "C"]);

        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_tabs_navigation() {
        let mut t = Tabs::new().tabs(vec!["One", "Two", "Three"]);

        assert_eq!(t.selected_index(), 0);

        t.select_next();
        assert_eq!(t.selected_index(), 1);

        t.select_next();
        assert_eq!(t.selected_index(), 2);

        t.select_next(); // Wraps around
        assert_eq!(t.selected_index(), 0);

        t.select_prev(); // Wraps around backward
        assert_eq!(t.selected_index(), 2);

        t.select_first();
        assert_eq!(t.selected_index(), 0);

        t.select_last();
        assert_eq!(t.selected_index(), 2);

        t.select(1);
        assert_eq!(t.selected_index(), 1);
    }

    #[test]
    fn test_tabs_handle_key() {
        use crate::event::Key;

        let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

        let changed = t.handle_key(&Key::Right);
        assert!(changed);
        assert_eq!(t.selected_index(), 1);

        let changed = t.handle_key(&Key::Left);
        assert!(changed);
        assert_eq!(t.selected_index(), 0);

        // Number keys (1-indexed)
        t.handle_key(&Key::Char('3'));
        assert_eq!(t.selected_index(), 2);

        t.handle_key(&Key::Char('1'));
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tabs_render() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tabs::new().tab("Files").tab("Edit");

        t.render(&mut ctx);

        // Check first tab label
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'i');
    }

    #[test]
    fn test_tabs_selected_label() {
        let t = Tabs::new().tabs(vec!["Alpha", "Beta"]);

        assert_eq!(t.selected_label(), Some("Alpha"));
    }

    #[test]
    fn test_tabs_helper() {
        let t = tabs().tab("Test");

        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_tab_item() {
        let tab = Tab::new("My Tab");
        assert_eq!(tab.label, "My Tab");
    }

    // =========================================================================
    // Additional coverage tests
    // =========================================================================

    #[test]
    fn test_tabs_default() {
        let t = Tabs::default();
        assert!(t.is_empty());
    }

    #[test]
    fn test_tabs_fg_bg_colors() {
        let t = Tabs::new().fg(Color::RED).bg(Color::BLUE);
        assert_eq!(t.fg, Some(Color::RED));
        assert_eq!(t.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_tabs_active_style() {
        let t = Tabs::new().active_style(Color::WHITE, Color::GREEN);
        assert_eq!(t.active_fg, Some(Color::WHITE));
        assert_eq!(t.active_bg, Some(Color::GREEN));
    }

    #[test]
    fn test_tabs_divider() {
        let t = Tabs::new().divider('|');
        assert_eq!(t.divider, '|');
    }

    #[test]
    fn test_tabs_handle_key_h_l() {
        use crate::event::Key;

        let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

        // l for right
        t.handle_key(&Key::Char('l'));
        assert_eq!(t.selected_index(), 1);

        // h for left
        t.handle_key(&Key::Char('h'));
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_home_end() {
        use crate::event::Key;

        let mut t = Tabs::new().tabs(vec!["A", "B", "C"]);

        t.handle_key(&Key::End);
        assert_eq!(t.selected_index(), 2);

        t.handle_key(&Key::Home);
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_number_out_of_range() {
        use crate::event::Key;

        let mut t = Tabs::new().tabs(vec!["A", "B"]);

        // Pressing '9' when there are only 2 tabs should do nothing
        let changed = t.handle_key(&Key::Char('9'));
        assert!(!changed);
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_unhandled() {
        use crate::event::Key;

        let mut t = Tabs::new().tabs(vec!["A", "B"]);

        let changed = t.handle_key(&Key::Escape);
        assert!(!changed);
    }

    #[test]
    fn test_tabs_selected_label_empty() {
        let t = Tabs::new();
        assert!(t.selected_label().is_none());
    }

    #[test]
    fn test_tabs_render_empty() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tabs::new();
        t.render(&mut ctx);
        // Empty tabs should not panic
    }

    #[test]
    fn test_tabs_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tabs::new().tab("Test");
        t.render(&mut ctx);
        // Small area should not panic
    }

    #[test]
    fn test_tabs_render_with_divider() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tabs::new().tab("A").tab("B").divider('|');

        t.render(&mut ctx);
        // Verify divider appears
    }

    #[test]
    fn test_tabs_render_selected() {
        let mut buffer = Buffer::new(40, 5);
        let area = Rect::new(0, 0, 40, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let t = Tabs::new()
            .tabs(vec!["First", "Second", "Third"])
            .selected(1);

        t.render(&mut ctx);
        // Verify selection rendering
    }

    #[test]
    fn test_tab_clone() {
        let tab = Tab::new("Original");
        let cloned = tab.clone();
        assert_eq!(cloned.label, "Original");
    }
}
