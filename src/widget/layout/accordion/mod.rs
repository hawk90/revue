//! Accordion widget for collapsible sections
//!
//! A vertically stacked list of collapsible content panels.

use crate::layout::Rect;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::border::render_border;
use crate::utils::Selection;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Accordion section
#[derive(Clone)]
pub struct AccordionSection {
    /// Section title
    pub title: String,
    /// Section content lines
    pub content: Vec<String>,
    /// Is section expanded
    pub expanded: bool,
    /// Custom icon when collapsed
    pub collapsed_icon: char,
    /// Custom icon when expanded
    pub expanded_icon: char,
}

impl AccordionSection {
    /// Create a new section
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: Vec::new(),
            expanded: false,
            collapsed_icon: '▶',
            expanded_icon: '▼',
        }
    }

    /// Add content line
    pub fn line(mut self, line: impl Into<String>) -> Self {
        self.content.push(line.into());
        self
    }

    /// Add multiple content lines
    pub fn lines(mut self, lines: &[&str]) -> Self {
        self.content.extend(lines.iter().map(|s| s.to_string()));
        self
    }

    /// Set content text (splits by newline)
    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.content = text.into().lines().map(|s| s.to_string()).collect();
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set custom icons
    pub fn icons(mut self, collapsed: char, expanded: char) -> Self {
        self.collapsed_icon = collapsed;
        self.expanded_icon = expanded;
        self
    }

    /// Get current icon
    fn icon(&self) -> char {
        if self.expanded {
            self.expanded_icon
        } else {
            self.collapsed_icon
        }
    }

    /// Get total height (header + content if expanded)
    #[cfg(test)]
    fn height(&self) -> u16 {
        if self.expanded {
            1 + self.content.len() as u16
        } else {
            1
        }
    }
}

/// Accordion widget
pub struct Accordion {
    /// Sections
    sections: Vec<AccordionSection>,
    /// Selection state
    selection: Selection,
    /// Allow multiple expanded sections
    multi_expand: bool,
    /// Header background color
    header_bg: Color,
    /// Header foreground color
    header_fg: Color,
    /// Selected header background
    selected_bg: Color,
    /// Content background color
    content_bg: Color,
    /// Content foreground color
    content_fg: Color,
    /// Border color
    border_color: Option<Color>,
    /// Show dividers between sections
    show_dividers: bool,
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

impl View for Accordion {
    crate::impl_view_meta!("Accordion");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 {
            return;
        }

        let content_area = if self.border_color.is_some() {
            Rect::new(
                area.x + 1,
                area.y + 1,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            )
        } else {
            area
        };

        // Draw border if set
        if let Some(border_color) = self.border_color {
            render_border(ctx, area, border_color);
        }

        let mut y = content_area.y;
        let max_y = content_area.y + content_area.height;

        for (section_idx, section) in self.sections.iter().enumerate() {
            if y >= max_y {
                break;
            }

            let is_selected = self.selection.is_selected(section_idx);

            // Render header
            let header_bg = if is_selected {
                self.selected_bg
            } else {
                self.header_bg
            };

            // Fill header background
            for x in content_area.x..content_area.x + content_area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(header_bg);
                ctx.buffer.set(x, y, cell);
            }

            // Icon
            let mut icon_cell = Cell::new(section.icon());
            icon_cell.fg = Some(self.header_fg);
            icon_cell.bg = Some(header_bg);
            ctx.buffer.set(content_area.x + 1, y, icon_cell);

            // Title
            let title_x = content_area.x + 3;
            let max_title_width = (content_area.width.saturating_sub(4)) as usize;
            for (i, ch) in section.title.chars().take(max_title_width).enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.header_fg);
                cell.bg = Some(header_bg);
                if is_selected {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(title_x + i as u16, y, cell);
            }

            y += 1;

            // Render content if expanded
            if section.expanded {
                for line in &section.content {
                    if y >= max_y {
                        break;
                    }

                    // Fill content background
                    for x in content_area.x..content_area.x + content_area.width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(self.content_bg);
                        ctx.buffer.set(x, y, cell);
                    }

                    // Content with indent
                    let content_x = content_area.x + 3;
                    let max_content_width = (content_area.width.saturating_sub(4)) as usize;
                    for (i, ch) in line.chars().take(max_content_width).enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(self.content_fg);
                        cell.bg = Some(self.content_bg);
                        ctx.buffer.set(content_x + i as u16, y, cell);
                    }

                    y += 1;
                }
            }

            // Divider
            if self.show_dividers && section_idx < self.sections.len() - 1 && y < max_y {
                for x in content_area.x..content_area.x + content_area.width {
                    let mut cell = Cell::new('─');
                    cell.fg = Some(Color::rgb(60, 60, 60));
                    ctx.buffer.set(x, y, cell);
                }
                y += 1;
            }
        }
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
    use crate::render::Buffer;

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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline
