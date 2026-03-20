//! Accordion widget for collapsible sections
//!
//! A vertically stacked list of collapsible content panels.

mod render;
mod types;

pub use types::AccordionSection;

use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Accordion widget
pub struct Accordion {
    /// Sections
    pub(crate) sections: Vec<AccordionSection>,
    /// Selection state
    pub(crate) selection: Selection,
    /// Allow multiple expanded sections
    pub(crate) multi_expand: bool,
    /// Header background color
    pub(crate) header_bg: Color,
    /// Header foreground color
    pub(crate) header_fg: Color,
    /// Selected header background
    pub(crate) selected_bg: Color,
    /// Content background color
    pub(crate) content_bg: Color,
    /// Content foreground color
    pub(crate) content_fg: Color,
    /// Border color
    pub(crate) border_color: Option<Color>,
    /// Show dividers between sections
    pub(crate) show_dividers: bool,
    /// Minimum width constraint (0 = no constraint)
    min_width: u16,
    /// Minimum height constraint (0 = no constraint)
    min_height: u16,
    /// Maximum width constraint (0 = no constraint)
    max_width: u16,
    /// Maximum height constraint (0 = no constraint)
    max_height: u16,
    /// Widget properties
    props: WidgetProps,
}

impl Accordion {
    /// Create a new accordion
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            selection: Selection::new(0),
            multi_expand: false,
            header_bg: Color::rgb(50, 50, 50),
            header_fg: Color::WHITE,
            selected_bg: Color::rgb(60, 90, 140),
            content_bg: Color::rgb(30, 30, 30),
            content_fg: Color::rgb(200, 200, 200),
            border_color: None,
            show_dividers: true,
            min_width: 0,
            min_height: 0,
            max_width: 0,
            max_height: 0,
            props: WidgetProps::new(),
        }
    }

    /// Add a section
    pub fn section(mut self, section: AccordionSection) -> Self {
        self.sections.push(section);
        self.selection.set_len(self.sections.len());
        self
    }

    /// Add multiple sections
    pub fn sections(mut self, sections: Vec<AccordionSection>) -> Self {
        self.sections.extend(sections);
        self.selection.set_len(self.sections.len());
        self
    }

    /// Allow multiple sections to be expanded
    pub fn multi_expand(mut self, allow: bool) -> Self {
        self.multi_expand = allow;
        self
    }

    /// Set header colors
    pub fn header_colors(mut self, fg: Color, bg: Color) -> Self {
        self.header_fg = fg;
        self.header_bg = bg;
        self
    }

    /// Set selected header background
    pub fn selected_bg(mut self, color: Color) -> Self {
        self.selected_bg = color;
        self
    }

    /// Set content colors
    pub fn content_colors(mut self, fg: Color, bg: Color) -> Self {
        self.content_fg = fg;
        self.content_bg = bg;
        self
    }

    /// Set border color
    pub fn border(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Show/hide dividers
    pub fn dividers(mut self, show: bool) -> Self {
        self.show_dividers = show;
        self
    }

    /// Select next section (wraps around)
    pub fn select_next(&mut self) {
        self.selection.next();
    }

    /// Select previous section (wraps around)
    pub fn select_prev(&mut self) {
        self.selection.prev();
    }

    /// Toggle selected section
    pub fn toggle_selected(&mut self) {
        if let Some(section) = self.sections.get_mut(self.selection.index) {
            if self.multi_expand {
                section.expanded = !section.expanded;
            } else {
                let was_expanded = section.expanded;
                // Collapse all others
                for s in &mut self.sections {
                    s.expanded = false;
                }
                // Toggle selected
                self.sections[self.selection.index].expanded = !was_expanded;
            }
        }
    }

    /// Expand selected section
    pub fn expand_selected(&mut self) {
        if self.selection.index >= self.sections.len() {
            return;
        }
        if !self.multi_expand {
            for s in &mut self.sections {
                s.expanded = false;
            }
        }
        self.sections[self.selection.index].expanded = true;
    }

    /// Collapse selected section
    pub fn collapse_selected(&mut self) {
        if let Some(section) = self.sections.get_mut(self.selection.index) {
            section.expanded = false;
        }
    }

    /// Expand all sections
    pub fn expand_all(&mut self) {
        for section in &mut self.sections {
            section.expanded = true;
        }
    }

    /// Collapse all sections
    pub fn collapse_all(&mut self) {
        for section in &mut self.sections {
            section.expanded = false;
        }
    }

    /// Get selected section index
    pub fn selected(&self) -> usize {
        self.selection.index
    }

    /// Set selected section
    pub fn set_selected(&mut self, index: usize) {
        self.selection.set(index);
    }

    /// Get section count
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
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

    /// Handle key input
    pub fn handle_key(&mut self, key: &crate::event::Key) -> bool {
        use crate::event::Key;

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Enter | Key::Char(' ') => {
                self.toggle_selected();
                true
            }
            Key::Right | Key::Char('l') => {
                self.expand_selected();
                true
            }
            Key::Left | Key::Char('h') => {
                self.collapse_selected();
                true
            }
            _ => false,
        }
    }

    /// Add section dynamically
    pub fn add_section(&mut self, section: AccordionSection) {
        self.sections.push(section);
        self.selection.set_len(self.sections.len());
    }

    /// Remove section by index
    pub fn remove_section(&mut self, index: usize) -> Option<AccordionSection> {
        if index < self.sections.len() {
            let section = self.sections.remove(index);
            self.selection.set_len(self.sections.len());
            Some(section)
        } else {
            None
        }
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

impl_styled_view!(Accordion);
impl_props_builders!(Accordion);

/// Helper to create an accordion
pub fn accordion() -> Accordion {
    Accordion::new()
}

/// Helper to create a section
pub fn section(title: impl Into<String>) -> AccordionSection {
    AccordionSection::new(title)
}

// KEEP HERE - Private implementation tests (all tests access private fields: title, expanded, content, height, etc.)

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::traits::{RenderContext, View};

    #[test]
    fn test_section_new() {
        let s = AccordionSection::new("Title");
        assert_eq!(s.title, "Title");
        assert!(!s.expanded);
    }

    #[test]
    fn test_section_builder() {
        let s = AccordionSection::new("FAQ")
            .line("Question 1")
            .line("Answer 1")
            .expanded(true);

        assert_eq!(s.content.len(), 2);
        assert!(s.expanded);
    }

    #[test]
    fn test_section_content() {
        let s = AccordionSection::new("Multi").content(
            "Line 1
Line 2
Line 3",
        );

        assert_eq!(s.content.len(), 3);
    }

    #[test]
    fn test_section_height() {
        let collapsed = AccordionSection::new("A");
        assert_eq!(collapsed.height(), 1);

        let expanded = AccordionSection::new("B")
            .line("1")
            .line("2")
            .expanded(true);
        assert_eq!(expanded.height(), 3);
    }

    #[test]
    fn test_accordion_toggle() {
        let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

        assert!(!acc.sections[0].expanded);

        acc.toggle_selected();
        assert!(acc.sections[0].expanded);

        acc.toggle_selected();
        assert!(!acc.sections[0].expanded);
    }

    #[test]
    fn test_accordion_single_expand() {
        let mut acc = Accordion::new()
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        acc.expand_selected();
        assert!(acc.sections[0].expanded);
        assert!(!acc.sections[1].expanded);

        acc.select_next();
        acc.expand_selected();
        assert!(!acc.sections[0].expanded);
        assert!(acc.sections[1].expanded);
    }

    #[test]
    fn test_accordion_multi_expand() {
        let mut acc = Accordion::new()
            .multi_expand(true)
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        acc.expand_selected();
        acc.select_next();
        acc.expand_selected();

        assert!(acc.sections[0].expanded);
        assert!(acc.sections[1].expanded);
    }

    #[test]
    fn test_accordion_expand_collapse_all() {
        let mut acc = Accordion::new()
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        acc.expand_all();
        assert!(acc.sections.iter().all(|s| s.expanded));

        acc.collapse_all();
        assert!(acc.sections.iter().all(|s| !s.expanded));
    }

    #[test]
    fn test_accordion_handle_key() {
        use crate::event::Key;

        let mut acc = Accordion::new()
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        assert!(acc.handle_key(&Key::Down));
        assert_eq!(acc.selected(), 1);

        assert!(acc.handle_key(&Key::Up));
        assert_eq!(acc.selected(), 0);

        assert!(acc.handle_key(&Key::Enter));
        assert!(acc.sections[0].expanded);
    }

    #[test]
    fn test_accordion_add_remove_title() {
        let mut acc = Accordion::new();

        acc.add_section(AccordionSection::new("A"));
        acc.add_section(AccordionSection::new("B"));
        assert_eq!(acc.len(), 2);

        acc.remove_section(0);
        assert_eq!(acc.len(), 1);
        assert_eq!(acc.sections[0].title, "B");
    }

    #[test]
    fn test_helpers() {
        let acc = accordion().section(section("Test").line("Content"));

        assert_eq!(acc.len(), 1);
    }

    #[test]
    fn test_section_icons() {
        let s = AccordionSection::new("Test").icons('+', '-');

        assert_eq!(s.collapsed_icon, '+');
        assert_eq!(s.expanded_icon, '-');
        assert_eq!(s.icon(), '+');
    }

    #[test]
    fn test_accordion_collapse_selected() {
        let mut acc = Accordion::new().section(AccordionSection::new("A").expanded(true));

        assert!(acc.sections[0].expanded);
        acc.collapse_selected();
        assert!(!acc.sections[0].expanded);
    }

    #[test]
    fn test_accordion_collapse_selected_empty() {
        let mut acc = Accordion::new();
        // Should not panic on empty
        acc.collapse_selected();
    }

    #[test]
    fn test_accordion_expand_selected_empty() {
        let mut acc = Accordion::new();
        // Should not panic on empty
        acc.expand_selected();
    }

    #[test]
    fn test_accordion_toggle_multi_expand() {
        let mut acc = Accordion::new()
            .multi_expand(true)
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        acc.toggle_selected();
        assert!(acc.sections[0].expanded);

        acc.toggle_selected();
        assert!(!acc.sections[0].expanded);
    }

    #[test]
    fn test_accordion_sections_batch() {
        let sections = vec![
            AccordionSection::new("A"),
            AccordionSection::new("B"),
            AccordionSection::new("C"),
        ];
        let acc = Accordion::new().sections(sections);
        assert_eq!(acc.len(), 3);
    }

    #[test]
    fn test_accordion_handle_key_j_k() {
        use crate::event::Key;

        let mut acc = Accordion::new()
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        // j for down
        assert!(acc.handle_key(&Key::Char('j')));
        assert_eq!(acc.selected(), 1);

        // k for up
        assert!(acc.handle_key(&Key::Char('k')));
        assert_eq!(acc.selected(), 0);
    }

    #[test]
    fn test_accordion_handle_key_space() {
        use crate::event::Key;

        let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

        assert!(acc.handle_key(&Key::Char(' ')));
        assert!(acc.sections[0].expanded);
    }

    #[test]
    fn test_accordion_handle_key_l_h() {
        use crate::event::Key;

        let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

        // l for expand
        assert!(acc.handle_key(&Key::Char('l')));
        assert!(acc.sections[0].expanded);

        // h for collapse
        assert!(acc.handle_key(&Key::Char('h')));
        assert!(!acc.sections[0].expanded);
    }

    #[test]
    fn test_accordion_handle_key_unhandled() {
        use crate::event::Key;

        let mut acc = Accordion::new().section(AccordionSection::new("A"));

        let changed = acc.handle_key(&Key::Tab);
        assert!(!changed);
    }

    #[test]
    fn test_accordion_colors() {
        let acc = Accordion::new()
            .header_colors(Color::WHITE, Color::RED)
            .content_colors(Color::BLACK, Color::GREEN)
            .selected_bg(Color::BLUE);

        assert_eq!(acc.header_fg, Color::WHITE);
        assert_eq!(acc.header_bg, Color::RED);
        assert_eq!(acc.content_fg, Color::BLACK);
        assert_eq!(acc.content_bg, Color::GREEN);
        assert_eq!(acc.selected_bg, Color::BLUE);
    }

    #[test]
    fn test_accordion_dividers() {
        let acc = Accordion::new().dividers(false);
        assert!(!acc.show_dividers);
    }

    #[test]
    fn test_accordion_render_small_area() {
        let mut buffer = Buffer::new(2, 1);
        let area = Rect::new(0, 0, 2, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let acc = Accordion::new().section(AccordionSection::new("Test"));

        acc.render(&mut ctx);
        // Small area should not panic
    }

    #[test]
    fn test_accordion_render_content_overflow() {
        let mut buffer = Buffer::new(40, 3);
        let area = Rect::new(0, 0, 40, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let acc = Accordion::new().section(
            AccordionSection::new("Section 1")
                .line("Line 1")
                .line("Line 2")
                .line("Line 3")
                .line("Line 4")
                .line("Line 5")
                .expanded(true),
        );

        acc.render(&mut ctx);
        // Should not panic when content exceeds area
    }

    #[test]
    fn test_section_lines() {
        let s = AccordionSection::new("Test").lines(&["Line 1", "Line 2", "Line 3"]);
        assert_eq!(s.content.len(), 3);
    }

    #[test]
    fn test_section_icon_expanded() {
        let s = AccordionSection::new("Test").expanded(true);
        assert_eq!(s.icon(), '▼');
    }

    #[test]
    fn test_section_clone() {
        let s = AccordionSection::new("Test").line("Content");
        let cloned = s.clone();
        assert_eq!(cloned.title, "Test");
        assert_eq!(cloned.content.len(), 1);
    }
}
