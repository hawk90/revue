//! Status bar widget for header/footer displays
//!
//! Provides configurable status bars with sections for displaying
//! application state, key hints, and other information.

use super::traits::{View, RenderContext, WidgetProps};
use crate::{impl_styled_view, impl_props_builders};
use crate::render::{Cell, Modifier};
use crate::style::Color;

/// Status bar position
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusBarPosition {
    /// At the top of the area
    Top,
    /// At the bottom of the area
    #[default]
    Bottom,
}

/// Section alignment
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SectionAlign {
    /// Left-aligned section (default)
    #[default]
    Left,
    /// Center-aligned section
    Center,
    /// Right-aligned section
    Right,
}

/// A section in the status bar
#[derive(Clone)]
pub struct StatusSection {
    /// Section content
    pub content: String,
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Bold text
    pub bold: bool,
    /// Minimum width
    pub min_width: u16,
    /// Priority (higher = more important, kept when space is limited)
    pub priority: u8,
}

impl StatusSection {
    /// Create a new section
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            fg: None,
            bg: None,
            bold: false,
            min_width: 0,
            priority: 0,
        }
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

    /// Set bold
    pub fn bold(mut self) -> Self {
        self.bold = true;
        self
    }

    /// Set minimum width
    pub fn min_width(mut self, width: u16) -> Self {
        self.min_width = width;
        self
    }

    /// Set priority
    pub fn priority(mut self, priority: u8) -> Self {
        self.priority = priority;
        self
    }

    /// Get display width
    pub fn width(&self) -> u16 {
        self.content.chars().count().max(self.min_width as usize) as u16
    }
}

/// Key hint for display in status bar
#[derive(Clone)]
pub struct KeyHint {
    /// Key combination
    pub key: String,
    /// Description
    pub description: String,
}

impl KeyHint {
    /// Create a new key hint
    pub fn new(key: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            description: description.into(),
        }
    }
}

/// Status bar widget
pub struct StatusBar {
    /// Left-aligned sections
    left: Vec<StatusSection>,
    /// Center-aligned sections
    center: Vec<StatusSection>,
    /// Right-aligned sections
    right: Vec<StatusSection>,
    /// Position
    position: StatusBarPosition,
    /// Background color
    bg: Color,
    /// Default foreground color
    fg: Color,
    /// Key hints
    key_hints: Vec<KeyHint>,
    /// Key hint foreground
    key_fg: Color,
    /// Key hint background
    key_bg: Color,
    /// Separator between sections
    separator: Option<char>,
    /// Height (usually 1)
    height: u16,
    /// Widget props for CSS integration
    props: WidgetProps,
}

impl StatusBar {
    /// Create a new status bar
    pub fn new() -> Self {
        Self {
            left: Vec::new(),
            center: Vec::new(),
            right: Vec::new(),
            position: StatusBarPosition::Bottom,
            bg: Color::rgb(40, 40, 40),
            fg: Color::WHITE,
            key_hints: Vec::new(),
            key_fg: Color::BLACK,
            key_bg: Color::rgb(200, 200, 200),
            separator: None,
            height: 1,
            props: WidgetProps::new(),
        }
    }

    /// Set position
    pub fn position(mut self, position: StatusBarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set as header (top position)
    pub fn header(mut self) -> Self {
        self.position = StatusBarPosition::Top;
        self
    }

    /// Set as footer (bottom position)
    pub fn footer(mut self) -> Self {
        self.position = StatusBarPosition::Bottom;
        self
    }

    /// Add left section
    pub fn left(mut self, section: StatusSection) -> Self {
        self.left.push(section);
        self
    }

    /// Add center section
    pub fn center(mut self, section: StatusSection) -> Self {
        self.center.push(section);
        self
    }

    /// Add right section
    pub fn right(mut self, section: StatusSection) -> Self {
        self.right.push(section);
        self
    }

    /// Add left text
    pub fn left_text(self, text: impl Into<String>) -> Self {
        self.left(StatusSection::new(text))
    }

    /// Add center text
    pub fn center_text(self, text: impl Into<String>) -> Self {
        self.center(StatusSection::new(text))
    }

    /// Add right text
    pub fn right_text(self, text: impl Into<String>) -> Self {
        self.right(StatusSection::new(text))
    }

    /// Set background color
    pub fn bg(mut self, color: Color) -> Self {
        self.bg = color;
        self
    }

    /// Set foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = color;
        self
    }

    /// Add key hint
    pub fn key(mut self, key: impl Into<String>, description: impl Into<String>) -> Self {
        self.key_hints.push(KeyHint::new(key, description));
        self
    }

    /// Add multiple key hints
    pub fn keys(mut self, hints: Vec<KeyHint>) -> Self {
        self.key_hints.extend(hints);
        self
    }

    /// Set separator character
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = Some(sep);
        self
    }

    /// Set height
    pub fn height(mut self, height: u16) -> Self {
        self.height = height.max(1);
        self
    }

    /// Update a left section by index
    pub fn update_left(&mut self, index: usize, content: impl Into<String>) {
        if let Some(section) = self.left.get_mut(index) {
            section.content = content.into();
        }
    }

    /// Update a center section by index
    pub fn update_center(&mut self, index: usize, content: impl Into<String>) {
        if let Some(section) = self.center.get_mut(index) {
            section.content = content.into();
        }
    }

    /// Update a right section by index
    pub fn update_right(&mut self, index: usize, content: impl Into<String>) {
        if let Some(section) = self.right.get_mut(index) {
            section.content = content.into();
        }
    }

    /// Clear all sections
    pub fn clear(&mut self) {
        self.left.clear();
        self.center.clear();
        self.right.clear();
        self.key_hints.clear();
    }

    /// Get render Y position
    fn render_y(&self, area_height: u16) -> u16 {
        match self.position {
            StatusBarPosition::Top => 0,
            StatusBarPosition::Bottom => area_height.saturating_sub(self.height),
        }
    }
}

impl Default for StatusBar {
    fn default() -> Self {
        Self::new()
    }
}

impl View for StatusBar {
    crate::impl_view_meta!("StatusBar");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let y = area.y + self.render_y(area.height);

        if y >= area.y + area.height {
            return;
        }

        // Fill background
        for row in 0..self.height {
            if y + row >= area.y + area.height {
                break;
            }
            for x in area.x..area.x + area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.bg);
                ctx.buffer.set(x, y + row, cell);
            }
        }

        // Calculate section widths
        let left_width: u16 = self.left.iter().map(|s| s.width() + 1).sum();
        let center_width: u16 = self.center.iter().map(|s| s.width() + 1).sum();
        let right_width: u16 = self.right.iter().map(|s| s.width() + 1).sum();

        // Render left sections
        let mut x = area.x;
        for section in &self.left {
            x = self.render_section(ctx, section, x, y);
            if self.separator.is_some() && x < area.x + area.width {
                x += 1;
            }
        }

        // Render center sections
        let center_start = area.x + (area.width.saturating_sub(center_width)) / 2;
        let mut x = center_start.max(x + 1);
        for section in &self.center {
            x = self.render_section(ctx, section, x, y);
            if self.separator.is_some() && x < area.x + area.width {
                x += 1;
            }
        }

        // Render right sections
        let mut x = area.x + area.width - right_width;
        for section in &self.right {
            x = self.render_section(ctx, section, x, y);
            if self.separator.is_some() && x < area.x + area.width {
                x += 1;
            }
        }

        // Render key hints on second row if height > 1
        if self.height > 1 && !self.key_hints.is_empty() {
            self.render_key_hints(ctx, area.x, y + 1, area.width);
        } else if self.height == 1 && !self.key_hints.is_empty() {
            // Render key hints in remaining space
            let hints_start = area.x + left_width + 2;
            let hints_end = area.x + area.width - right_width - 2;
            if hints_start < hints_end {
                self.render_key_hints_inline(ctx, hints_start, y, hints_end - hints_start);
            }
        }
    }
}

impl_styled_view!(StatusBar);
impl_props_builders!(StatusBar);

impl StatusBar {
    fn render_section(&self, ctx: &mut RenderContext, section: &StatusSection, x: u16, y: u16) -> u16 {
        let fg = section.fg.unwrap_or(self.fg);
        let bg = section.bg.unwrap_or(self.bg);

        let mut current_x = x;
        for ch in section.content.chars() {
            if current_x >= ctx.area.x + ctx.area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(fg);
            cell.bg = Some(bg);
            if section.bold {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(current_x, y, cell);
            current_x += 1;
        }

        // Pad to min_width
        while current_x < x + section.min_width && current_x < ctx.area.x + ctx.area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(bg);
            ctx.buffer.set(current_x, y, cell);
            current_x += 1;
        }

        current_x
    }

    fn render_key_hints(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        let mut current_x = x;

        for hint in &self.key_hints {
            if current_x >= x + width {
                break;
            }

            // Render key
            for ch in hint.key.chars() {
                if current_x >= x + width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.key_fg);
                cell.bg = Some(self.key_bg);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(current_x, y, cell);
                current_x += 1;
            }

            // Render description
            let desc = format!(" {} ", hint.description);
            for ch in desc.chars() {
                if current_x >= x + width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.fg);
                cell.bg = Some(self.bg);
                ctx.buffer.set(current_x, y, cell);
                current_x += 1;
            }
        }
    }

    fn render_key_hints_inline(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16) {
        let mut current_x = x;

        for hint in &self.key_hints {
            let hint_width = hint.key.len() + hint.description.len() + 3;
            if current_x + hint_width as u16 > x + width {
                break;
            }

            // Render key
            for ch in hint.key.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.key_fg);
                cell.bg = Some(self.key_bg);
                ctx.buffer.set(current_x, y, cell);
                current_x += 1;
            }

            // Space
            current_x += 1;

            // Render description
            for ch in hint.description.chars() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(self.fg);
                cell.bg = Some(self.bg);
                ctx.buffer.set(current_x, y, cell);
                current_x += 1;
            }

            // Separator
            current_x += 2;
        }
    }
}

// Helper functions

/// Create a new status bar
pub fn statusbar() -> StatusBar {
    StatusBar::new()
}

/// Create a header status bar (positioned at top)
pub fn header() -> StatusBar {
    StatusBar::new().header()
}

/// Create a footer status bar (positioned at bottom)
pub fn footer() -> StatusBar {
    StatusBar::new().footer()
}

/// Create a status bar section with content
pub fn section(content: impl Into<String>) -> StatusSection {
    StatusSection::new(content)
}

/// Create a key hint with key and description
pub fn key_hint(key: impl Into<String>, description: impl Into<String>) -> KeyHint {
    KeyHint::new(key, description)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Buffer;
    use crate::layout::Rect;

    #[test]
    fn test_status_section() {
        let section = StatusSection::new("Hello")
            .fg(Color::WHITE)
            .bold()
            .min_width(10);

        assert_eq!(section.content, "Hello");
        assert_eq!(section.width(), 10);
        assert!(section.bold);
    }

    #[test]
    fn test_status_bar() {
        let bar = StatusBar::new()
            .left_text("File.txt")
            .center_text("Line 1, Col 1")
            .right_text("UTF-8");

        assert_eq!(bar.left.len(), 1);
        assert_eq!(bar.center.len(), 1);
        assert_eq!(bar.right.len(), 1);
    }

    #[test]
    fn test_status_bar_with_keys() {
        let bar = StatusBar::new()
            .key("^X", "Exit")
            .key("^S", "Save")
            .key("^O", "Open");

        assert_eq!(bar.key_hints.len(), 3);
    }

    #[test]
    fn test_status_bar_position() {
        let top = StatusBar::new().header();
        assert_eq!(top.position, StatusBarPosition::Top);
        assert_eq!(top.render_y(24), 0);

        let bottom = StatusBar::new().footer();
        assert_eq!(bottom.position, StatusBarPosition::Bottom);
        assert_eq!(bottom.render_y(24), 23);
    }

    #[test]
    fn test_status_bar_render() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bar = StatusBar::new()
            .left_text("Left")
            .right_text("Right");

        bar.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_status_bar_update() {
        let mut bar = StatusBar::new()
            .left_text("Original");

        bar.update_left(0, "Updated");
        assert_eq!(bar.left[0].content, "Updated");
    }
}
