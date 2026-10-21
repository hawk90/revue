//! Accordion widget for collapsible sections
//!
//! A vertically stacked list of collapsible content panels.

mod render;
mod section;

pub use section::AccordionSection;

use crate::style::Color;
use crate::utils::{ScrollState, Selection};
use crate::widget::theme::{EDITOR_BG, SECONDARY_TEXT};
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

/// Accordion widget
#[derive(Clone)]
pub struct Accordion {
    /// Sections
    sections: Vec<AccordionSection>,
    /// Selection state
    selection: Selection,
    /// Where the sections are scrolled to, in rows: a header, the content
    /// lines of an expanded section, and a divider
    scroll: ScrollState,
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
            scroll: ScrollState::new(),
            multi_expand: false,
            header_bg: Color::rgb(50, 50, 50),
            header_fg: Color::WHITE,
            selected_bg: Color::rgb(60, 90, 140),
            content_bg: EDITOR_BG,
            content_fg: SECONDARY_TEXT,
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
            // The selection stays on its section (#975); removing the
            // selected one selects the section now in its place
            if index < self.selection.index {
                self.selection.index -= 1;
            }
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

#[cfg(test)]
mod tests;
