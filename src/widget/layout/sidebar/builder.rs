//! Sidebar builders: content, collapse settings, colors and size constraints

use super::types::{CollapseMode, SidebarItem, SidebarSection};
use super::Sidebar;
use crate::style::Color;

impl Sidebar {
    /// Add a section
    pub fn section(mut self, section: SidebarSection) -> Self {
        self.sections.push(section);
        self
    }

    /// Add multiple sections
    pub fn sections(mut self, sections: Vec<SidebarSection>) -> Self {
        self.sections = sections;
        self
    }

    /// Add items without a section title
    pub fn items(mut self, items: Vec<SidebarItem>) -> Self {
        self.sections.push(SidebarSection::new(items));
        self
    }

    /// Set selected item by ID
    pub fn selected(mut self, id: impl Into<String>) -> Self {
        self.selected = Some(id.into());
        self
    }

    /// Set collapse mode
    pub fn collapse_mode(mut self, mode: CollapseMode) -> Self {
        self.collapse_mode = mode;
        self
    }

    /// Set collapse threshold for auto mode
    pub fn collapse_threshold(mut self, width: u16) -> Self {
        self.collapse_threshold = width;
        self
    }

    /// Set expanded width
    pub fn expanded_width(mut self, width: u16) -> Self {
        self.expanded_width = width;
        self
    }

    /// Set collapsed width
    pub fn collapsed_width(mut self, width: u16) -> Self {
        self.collapsed_width = width;
        self
    }

    /// Set header text
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Set footer text
    pub fn footer(mut self, footer: impl Into<String>) -> Self {
        self.footer = Some(footer.into());
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

    /// Set selected item style
    pub fn selected_style(mut self, fg: Color, bg: Color) -> Self {
        self.selected_fg = Some(fg);
        self.selected_bg = Some(bg);
        self
    }

    /// Set hover style
    pub fn hover_style(mut self, fg: Color, bg: Color) -> Self {
        self.hover_fg = Some(fg);
        self.hover_bg = Some(bg);
        self
    }

    /// Set disabled color
    pub fn disabled_color(mut self, color: Color) -> Self {
        self.disabled_fg = Some(color);
        self
    }

    /// Set section title color
    pub fn section_color(mut self, color: Color) -> Self {
        self.section_fg = Some(color);
        self
    }

    /// Set badge style
    pub fn badge_style(mut self, fg: Color, bg: Color) -> Self {
        self.badge_fg = Some(fg);
        self.badge_bg = Some(bg);
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_fg = Some(color);
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
}
