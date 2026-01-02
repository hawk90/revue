//! Accordion widget for collapsible sections
//!
//! A vertically stacked list of collapsible content panels.

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::layout::Rect;
use crate::utils::border::render_border;
use crate::{impl_styled_view, impl_props_builders};

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
    /// Selected section index
    selected: usize,
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
    /// Scroll offset
    scroll_offset: u16,
    /// Widget properties
    props: WidgetProps,
}

impl Accordion {
    /// Create a new accordion
    pub fn new() -> Self {
        Self {
            sections: Vec::new(),
            selected: 0,
            multi_expand: false,
            header_bg: Color::rgb(50, 50, 50),
            header_fg: Color::WHITE,
            selected_bg: Color::rgb(60, 90, 140),
            content_bg: Color::rgb(30, 30, 30),
            content_fg: Color::rgb(200, 200, 200),
            border_color: None,
            show_dividers: true,
            scroll_offset: 0,
            props: WidgetProps::new(),
        }
    }

    /// Add a section
    pub fn section(mut self, section: AccordionSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Add multiple sections
    pub fn sections(mut self, sections: Vec<AccordionSection>) -> Self {
        self.sections.extend(sections);
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

    /// Select next section
    pub fn select_next(&mut self) {
        if !self.sections.is_empty() {
            self.selected = (self.selected + 1) % self.sections.len();
            self.ensure_visible();
        }
    }

    /// Select previous section
    pub fn select_prev(&mut self) {
        if !self.sections.is_empty() {
            self.selected = self.selected.checked_sub(1).unwrap_or(self.sections.len() - 1);
            self.ensure_visible();
        }
    }

    /// Toggle selected section
    pub fn toggle_selected(&mut self) {
        if let Some(section) = self.sections.get_mut(self.selected) {
            if self.multi_expand {
                section.expanded = !section.expanded;
            } else {
                let was_expanded = section.expanded;
                // Collapse all others
                for s in &mut self.sections {
                    s.expanded = false;
                }
                // Toggle selected
                self.sections[self.selected].expanded = !was_expanded;
            }
        }
    }

    /// Expand selected section
    pub fn expand_selected(&mut self) {
        if self.selected >= self.sections.len() {
            return;
        }
        if !self.multi_expand {
            for s in &mut self.sections {
                s.expanded = false;
            }
        }
        self.sections[self.selected].expanded = true;
    }

    /// Collapse selected section
    pub fn collapse_selected(&mut self) {
        if let Some(section) = self.sections.get_mut(self.selected) {
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
        self.selected
    }

    /// Set selected section
    pub fn set_selected(&mut self, index: usize) {
        if index < self.sections.len() {
            self.selected = index;
        }
    }

    /// Get section count
    pub fn len(&self) -> usize {
        self.sections.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.sections.is_empty()
    }

    /// Ensure selected section is visible
    fn ensure_visible(&mut self) {
        // Calculate position of selected header
        let mut y = 0u16;
        for (i, section) in self.sections.iter().enumerate() {
            if i == self.selected {
                break;
            }
            y += section.height();
            if self.show_dividers && i < self.sections.len() - 1 {
                y += 1;
            }
        }

        if y < self.scroll_offset {
            self.scroll_offset = y;
        }
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
    }

    /// Remove section by index
    pub fn remove_section(&mut self, index: usize) -> Option<AccordionSection> {
        if index < self.sections.len() {
            let section = self.sections.remove(index);
            if self.selected >= self.sections.len() && !self.sections.is_empty() {
                self.selected = self.sections.len() - 1;
            }
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
            Rect::new(area.x + 1, area.y + 1, area.width.saturating_sub(2), area.height.saturating_sub(2))
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

            let is_selected = section_idx == self.selected;

            // Render header
            let header_bg = if is_selected { self.selected_bg } else { self.header_bg };

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
        let s = AccordionSection::new("Multi")
            .content("Line 1\nLine 2\nLine 3");

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
    fn test_accordion_new() {
        let acc = Accordion::new();
        assert!(acc.is_empty());
        assert_eq!(acc.selected(), 0);
    }

    #[test]
    fn test_accordion_sections() {
        let acc = Accordion::new()
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"));

        assert_eq!(acc.len(), 2);
    }

    #[test]
    fn test_accordion_selection() {
        let mut acc = Accordion::new()
            .section(AccordionSection::new("A"))
            .section(AccordionSection::new("B"))
            .section(AccordionSection::new("C"));

        assert_eq!(acc.selected(), 0);

        acc.select_next();
        assert_eq!(acc.selected(), 1);

        acc.select_next();
        assert_eq!(acc.selected(), 2);

        acc.select_next();
        assert_eq!(acc.selected(), 0); // Wrap

        acc.select_prev();
        assert_eq!(acc.selected(), 2); // Wrap back
    }

    #[test]
    fn test_accordion_toggle() {
        let mut acc = Accordion::new()
            .section(AccordionSection::new("A").line("Content"));

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
    fn test_accordion_render() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let acc = Accordion::new()
            .section(AccordionSection::new("Section 1").line("Content 1").expanded(true))
            .section(AccordionSection::new("Section 2").line("Content 2"));

        acc.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_accordion_with_border() {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let acc = Accordion::new()
            .border(Color::WHITE)
            .section(AccordionSection::new("Test"));

        acc.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    }

    #[test]
    fn test_accordion_add_remove() {
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
        let acc = accordion()
            .section(section("Test").line("Content"));

        assert_eq!(acc.len(), 1);
    }

    #[test]
    fn test_section_icons() {
        let s = AccordionSection::new("Test")
            .icons('+', '-');

        assert_eq!(s.collapsed_icon, '+');
        assert_eq!(s.expanded_icon, '-');
        assert_eq!(s.icon(), '+');
    }
}
