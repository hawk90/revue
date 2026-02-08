//! Tabs widget for tabbed navigation

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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Tab tests
    // =========================================================================

    #[test]
    fn test_tab_new() {
        let tab = Tab::new("My Tab");
        assert_eq!(tab.label, "My Tab");
    }

    #[test]
    fn test_tab_new_from_string() {
        let tab = Tab::new(String::from("Owned"));
        assert_eq!(tab.label, "Owned");
    }

    #[test]
    fn test_tab_new_from_str() {
        let tab = Tab::new("str");
        assert_eq!(tab.label, "str");
    }

    #[test]
    fn test_tab_clone() {
        let tab = Tab::new("Original");
        let cloned = tab.clone();
        assert_eq!(cloned.label, "Original");
    }

    #[test]
    fn test_tab_empty_label() {
        let tab = Tab::new("");
        assert_eq!(tab.label, "");
    }

    #[test]
    fn test_tab_unicode_label() {
        let tab = Tab::new("タブ");
        assert_eq!(tab.label, "タブ");
    }

    // =========================================================================
    // Tabs::new and default tests
    // =========================================================================

    #[test]
    fn test_tabs_new() {
        let tabs = Tabs::new();
        assert!(tabs.is_empty());
        assert_eq!(tabs.len(), 0);
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_default() {
        let tabs = Tabs::default();
        assert!(tabs.is_empty());
    }

    #[test]
    fn test_tabs_helper() {
        let tabs = tabs();
        assert!(tabs.is_empty());
    }

    // =========================================================================
    // Tabs builder tests
    // =========================================================================

    #[test]
    fn test_tabs_with_single_tab() {
        let tabs = Tabs::new().tab("Home");
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs.selected_label(), Some("Home"));
    }

    #[test]
    fn test_tabs_with_multiple_tabs() {
        let tabs = Tabs::new().tab("Tab1").tab("Tab2").tab("Tab3");
        assert_eq!(tabs.len(), 3);
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_builder() {
        let tabs = Tabs::new().tabs(vec!["A", "B", "C"]);
        assert_eq!(tabs.len(), 3);
    }

    #[test]
    fn test_tabs_builder_empty_vec() {
        let tabs = Tabs::new().tabs(vec![""; 0]);
        assert!(tabs.is_empty());
    }

    // =========================================================================
    // Tabs selection tests
    // =========================================================================

    #[test]
    fn test_tabs_selected() {
        let tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(1);
        assert_eq!(tabs.selected_index(), 1);
        assert_eq!(tabs.selected_label(), Some("Tab2"));
    }

    #[test]
    fn test_tabs_select_next() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").tab("Tab3").selected(0);

        tabs.select_next();
        assert_eq!(tabs.selected_index(), 1);

        tabs.select_next();
        assert_eq!(tabs.selected_index(), 2);

        // Wrap around
        tabs.select_next();
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_select_prev() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").tab("Tab3").selected(0);

        tabs.select_prev();
        assert_eq!(tabs.selected_index(), 2); // Wrap around

        tabs.select_prev();
        assert_eq!(tabs.selected_index(), 1);
    }

    #[test]
    fn test_tabs_select_first() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").tab("Tab3").selected(2);

        tabs.select_first();
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_select_last() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(0);

        tabs.select_last();
        assert_eq!(tabs.selected_index(), 1);
    }

    #[test]
    fn test_tabs_select_by_index() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").tab("Tab3");

        tabs.select(2);
        assert_eq!(tabs.selected_index(), 2);
        assert_eq!(tabs.selected_label(), Some("Tab3"));
    }

    #[test]
    fn test_tabs_selected_label_none() {
        let tabs = Tabs::new();
        assert_eq!(tabs.selected_label(), None);
    }

    // =========================================================================
    // Tabs handle_key tests
    // =========================================================================

    #[test]
    fn test_tabs_handle_key_left() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(1);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::Left);
        assert!(handled);
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_right() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(0);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::Right);
        assert!(handled);
        assert_eq!(tabs.selected_index(), 1);
    }

    #[test]
    fn test_tabs_handle_key_h() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(1);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::Char('h'));
        assert!(handled);
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_l() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(0);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::Char('l'));
        assert!(handled);
        assert_eq!(tabs.selected_index(), 1);
    }

    #[test]
    fn test_tabs_handle_key_home() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(1);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::Home);
        assert!(handled);
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_end() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(0);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::End);
        assert!(handled);
        assert_eq!(tabs.selected_index(), 1);
    }

    #[test]
    fn test_tabs_handle_key_digit() {
        let mut tabs = Tabs::new().tab("Tab1").tab("Tab2").selected(1);

        use crate::event::Key;
        // Index 2 (key '3') is out of range for 2 tabs
        let handled = tabs.handle_key(&Key::Char('3'));
        assert!(!handled); // Index 2 out of range

        // Index 1 (key '2') should select second tab (but already selected)
        let handled = tabs.handle_key(&Key::Char('2'));
        assert!(!handled); // No change because already at index 1

        // Index 0 (key '1') should select first tab
        let handled = tabs.handle_key(&Key::Char('1'));
        assert!(handled);
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_handle_key_unhandled() {
        let mut tabs = Tabs::new().tab("Tab1").selected(0);

        use crate::event::Key;
        let handled = tabs.handle_key(&Key::Up);
        assert!(!handled);
        assert_eq!(tabs.selected_index(), 0);
    }

    // =========================================================================
    // Tabs style tests
    // =========================================================================

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
    fn test_tabs_divider_unicode() {
        let t = Tabs::new().divider('┃');
        assert_eq!(t.divider, '┃');
    }

    // =========================================================================
    // Tabs query methods
    // =========================================================================

    #[test]
    fn test_tabs_len() {
        let tabs = Tabs::new().tab("A").tab("B").tab("C");
        assert_eq!(tabs.len(), 3);
    }

    #[test]
    fn test_tabs_is_empty_true() {
        let tabs = Tabs::new();
        assert!(tabs.is_empty());
    }

    #[test]
    fn test_tabs_is_empty_false() {
        let tabs = Tabs::new().tab("A");
        assert!(!tabs.is_empty());
    }

    // =========================================================================
    // Tabs edge cases
    // =========================================================================

    #[test]
    fn test_tabs_empty_selected_label() {
        let tabs = Tabs::new();
        assert_eq!(tabs.selected_label(), None);
    }

    #[test]
    fn test_tabs_single_tab() {
        let mut tabs = Tabs::new().tab("Only");
        assert_eq!(tabs.len(), 1);
        assert_eq!(tabs.selected_label(), Some("Only"));
        // Next and prev should wrap to same tab
        tabs.select_next();
        assert_eq!(tabs.selected_index(), 0);
        tabs.select_prev();
        assert_eq!(tabs.selected_index(), 0);
    }

    #[test]
    fn test_tabs_chain_builder() {
        let tabs = Tabs::new()
            .tab("A")
            .tab("B")
            .selected(1)
            .fg(Color::CYAN)
            .bg(Color::BLACK)
            .active_style(Color::WHITE, Color::BLUE)
            .divider('│');

        assert_eq!(tabs.len(), 2);
        assert_eq!(tabs.selected_index(), 1);
    }
}
