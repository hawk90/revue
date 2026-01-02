//! Tabs widget for tabbed navigation

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_styled_view, impl_props_builders};

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
    selected: usize,
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
            selected: 0,
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
        self
    }

    /// Add a tab
    pub fn tab(mut self, label: impl Into<String>) -> Self {
        self.tabs.push(Tab::new(label));
        self
    }

    /// Set selected tab index
    pub fn selected(mut self, index: usize) -> Self {
        self.selected = index.min(self.tabs.len().saturating_sub(1));
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
        self.selected
    }

    /// Get selected tab label
    pub fn selected_label(&self) -> Option<&str> {
        self.tabs.get(self.selected).map(|t| t.label.as_str())
    }

    /// Select next tab
    pub fn select_next(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = (self.selected + 1) % self.tabs.len();
        }
    }

    /// Select previous tab
    pub fn select_prev(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = self.selected
                .checked_sub(1)
                .unwrap_or(self.tabs.len() - 1);
        }
    }

    /// Select first tab
    pub fn select_first(&mut self) {
        self.selected = 0;
    }

    /// Select last tab
    pub fn select_last(&mut self) {
        if !self.tabs.is_empty() {
            self.selected = self.tabs.len() - 1;
        }
    }

    /// Select tab by index
    pub fn select(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.selected = index;
        }
    }

    /// Handle key input, returns true if selection changed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Left | Key::Char('h') => {
                let old = self.selected;
                self.select_prev();
                old != self.selected
            }
            Key::Right | Key::Char('l') => {
                let old = self.selected;
                self.select_next();
                old != self.selected
            }
            Key::Home => {
                let old = self.selected;
                self.select_first();
                old != self.selected
            }
            Key::End => {
                let old = self.selected;
                self.select_last();
                old != self.selected
            }
            Key::Char(c) if c.is_ascii_digit() => {
                let index = (*c as usize) - ('1' as usize);
                if index < self.tabs.len() {
                    let old = self.selected;
                    self.selected = index;
                    old != self.selected
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
            let is_active = i == self.selected;
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
    use crate::render::Buffer;
    use crate::layout::Rect;
    

    #[test]
    fn test_tabs_new() {
        let t = Tabs::new();
        assert!(t.is_empty());
        assert_eq!(t.selected_index(), 0);
    }

    #[test]
    fn test_tabs_builder() {
        let t = Tabs::new()
            .tab("Home")
            .tab("Settings")
            .tab("Help");

        assert_eq!(t.len(), 3);
        assert_eq!(t.selected_label(), Some("Home"));
    }

    #[test]
    fn test_tabs_from_vec() {
        let t = Tabs::new()
            .tabs(vec!["A", "B", "C"]);

        assert_eq!(t.len(), 3);
    }

    #[test]
    fn test_tabs_navigation() {
        let mut t = Tabs::new()
            .tabs(vec!["One", "Two", "Three"]);

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

        let mut t = Tabs::new()
            .tabs(vec!["A", "B", "C"]);

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

        let t = Tabs::new()
            .tab("Files")
            .tab("Edit");

        t.render(&mut ctx);

        // Check first tab label
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'F');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'i');
    }

    #[test]
    fn test_tabs_selected_label() {
        let t = Tabs::new()
            .tabs(vec!["Alpha", "Beta"]);

        assert_eq!(t.selected_label(), Some("Alpha"));
    }

    #[test]
    fn test_tabs_helper() {
        let t = tabs()
            .tab("Test");

        assert_eq!(t.len(), 1);
    }

    #[test]
    fn test_tab_item() {
        let tab = Tab::new("My Tab");
        assert_eq!(tab.label, "My Tab");
    }
}
