//! Status bar widget for header/footer displays
//!
//! Provides configurable status bars with sections for displaying
//! application state, key hints, and other information.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

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
    fn render_section(
        &self,
        ctx: &mut RenderContext,
        section: &StatusSection,
        x: u16,
        y: u16,
    ) -> u16 {
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
    use crate::layout::Rect;
    use crate::render::Buffer;

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

        let bar = StatusBar::new().left_text("Left").right_text("Right");

        bar.render(&mut ctx);
        // Smoke test
    }

    #[test]
    fn test_status_bar_update() {
        let mut bar = StatusBar::new().left_text("Original");

        bar.update_left(0, "Updated");
        assert_eq!(bar.left[0].content, "Updated");
    }

    // =========================================================================
    // StatusBarPosition enum tests
    // =========================================================================

    #[test]
    fn test_status_bar_position_default() {
        let pos = StatusBarPosition::default();
        assert_eq!(pos, StatusBarPosition::Bottom);
    }

    #[test]
    fn test_status_bar_position_clone() {
        let pos = StatusBarPosition::Top;
        let cloned = pos.clone();
        assert_eq!(pos, cloned);
    }

    #[test]
    fn test_status_bar_position_copy() {
        let pos1 = StatusBarPosition::Bottom;
        let pos2 = pos1;
        assert_eq!(pos1, StatusBarPosition::Bottom);
        assert_eq!(pos2, StatusBarPosition::Bottom);
    }

    #[test]
    fn test_status_bar_position_partial_eq() {
        assert_eq!(StatusBarPosition::Top, StatusBarPosition::Top);
        assert_ne!(StatusBarPosition::Top, StatusBarPosition::Bottom);
    }

    #[test]
    fn test_status_bar_position_debug() {
        let pos = StatusBarPosition::Top;
        assert!(format!("{:?}", pos).contains("Top"));
    }

    // =========================================================================
    // SectionAlign enum tests
    // =========================================================================

    #[test]
    fn test_section_align_default() {
        let align = SectionAlign::default();
        assert_eq!(align, SectionAlign::Left);
    }

    #[test]
    fn test_section_align_clone() {
        let align = SectionAlign::Center;
        let cloned = align.clone();
        assert_eq!(align, cloned);
    }

    #[test]
    fn test_section_align_copy() {
        let align1 = SectionAlign::Right;
        let align2 = align1;
        assert_eq!(align1, SectionAlign::Right);
        assert_eq!(align2, SectionAlign::Right);
    }

    #[test]
    fn test_section_align_partial_eq() {
        assert_eq!(SectionAlign::Left, SectionAlign::Left);
        assert_ne!(SectionAlign::Left, SectionAlign::Center);
    }

    #[test]
    fn test_section_align_debug() {
        let align = SectionAlign::Center;
        assert!(format!("{:?}", align).contains("Center"));
    }

    // =========================================================================
    // StatusSection tests
    // =========================================================================

    #[test]
    fn test_status_section_new() {
        let section = StatusSection::new("Test");
        assert_eq!(section.content, "Test");
        assert!(section.fg.is_none());
        assert!(section.bg.is_none());
        assert!(!section.bold);
        assert_eq!(section.min_width, 0);
        assert_eq!(section.priority, 0);
    }

    #[test]
    fn test_status_section_fg() {
        let section = StatusSection::new("Test").fg(Color::RED);
        assert_eq!(section.fg, Some(Color::RED));
    }

    #[test]
    fn test_status_section_bg() {
        let section = StatusSection::new("Test").bg(Color::BLUE);
        assert_eq!(section.bg, Some(Color::BLUE));
    }

    #[test]
    fn test_status_section_bold() {
        let section = StatusSection::new("Test").bold();
        assert!(section.bold);
    }

    #[test]
    fn test_status_section_min_width() {
        let section = StatusSection::new("Test").min_width(20);
        assert_eq!(section.min_width, 20);
    }

    #[test]
    fn test_status_section_priority() {
        let section = StatusSection::new("Test").priority(5);
        assert_eq!(section.priority, 5);
    }

    #[test]
    fn test_status_section_width_content() {
        let section = StatusSection::new("Hello World");
        assert_eq!(section.width(), 11);
    }

    #[test]
    fn test_status_section_width_min_width() {
        let section = StatusSection::new("Hi").min_width(10);
        assert_eq!(section.width(), 10);
    }

    #[test]
    fn test_status_section_clone() {
        let section1 = StatusSection::new("Test").fg(Color::RED).bold().priority(3);
        let section2 = section1.clone();
        assert_eq!(section1.content, section2.content);
        assert_eq!(section1.fg, section2.fg);
        assert_eq!(section1.bold, section2.bold);
        assert_eq!(section1.priority, section2.priority);
    }

    #[test]
    fn test_status_section_builder_chain() {
        let section = StatusSection::new("Chained")
            .fg(Color::CYAN)
            .bg(Color::BLACK)
            .bold()
            .min_width(15)
            .priority(10);
        assert_eq!(section.content, "Chained");
        assert_eq!(section.fg, Some(Color::CYAN));
        assert_eq!(section.bg, Some(Color::BLACK));
        assert!(section.bold);
        assert_eq!(section.min_width, 15);
        assert_eq!(section.priority, 10);
    }

    // =========================================================================
    // KeyHint tests
    // =========================================================================

    #[test]
    fn test_key_hint_new() {
        let hint = KeyHint::new("Ctrl+S", "Save");
        assert_eq!(hint.key, "Ctrl+S");
        assert_eq!(hint.description, "Save");
    }

    #[test]
    fn test_key_hint_clone() {
        let hint1 = KeyHint::new("Ctrl+Q", "Quit");
        let hint2 = hint1.clone();
        assert_eq!(hint1.key, hint2.key);
        assert_eq!(hint1.description, hint2.description);
    }

    // =========================================================================
    // StatusBar builder tests
    // =========================================================================

    #[test]
    fn test_status_bar_new() {
        let bar = StatusBar::new();
        assert!(bar.left.is_empty());
        assert!(bar.center.is_empty());
        assert!(bar.right.is_empty());
        assert!(bar.key_hints.is_empty());
        assert_eq!(bar.position, StatusBarPosition::Bottom);
        assert_eq!(bar.height, 1);
        assert!(bar.separator.is_none());
    }

    #[test]
    fn test_status_bar_position_builder() {
        let bar = StatusBar::new().position(StatusBarPosition::Top);
        assert_eq!(bar.position, StatusBarPosition::Top);
    }

    #[test]
    fn test_status_bar_header() {
        let bar = StatusBar::new().header();
        assert_eq!(bar.position, StatusBarPosition::Top);
    }

    #[test]
    fn test_status_bar_footer() {
        let bar = StatusBar::new().footer();
        assert_eq!(bar.position, StatusBarPosition::Bottom);
    }

    #[test]
    fn test_status_bar_left() {
        let section = StatusSection::new("Left Text");
        let bar = StatusBar::new().left(section.clone());
        assert_eq!(bar.left.len(), 1);
        assert_eq!(bar.left[0].content, "Left Text");
    }

    #[test]
    fn test_status_bar_center() {
        let section = StatusSection::new("Center");
        let bar = StatusBar::new().center(section);
        assert_eq!(bar.center.len(), 1);
        assert_eq!(bar.center[0].content, "Center");
    }

    #[test]
    fn test_status_bar_right() {
        let section = StatusSection::new("Right");
        let bar = StatusBar::new().right(section);
        assert_eq!(bar.right.len(), 1);
        assert_eq!(bar.right[0].content, "Right");
    }

    #[test]
    fn test_status_bar_left_text() {
        let bar = StatusBar::new().left_text("File");
        assert_eq!(bar.left.len(), 1);
        assert_eq!(bar.left[0].content, "File");
    }

    #[test]
    fn test_status_bar_center_text() {
        let bar = StatusBar::new().center_text("Line 1");
        assert_eq!(bar.center.len(), 1);
        assert_eq!(bar.center[0].content, "Line 1");
    }

    #[test]
    fn test_status_bar_right_text() {
        let bar = StatusBar::new().right_text("UTF-8");
        assert_eq!(bar.right.len(), 1);
        assert_eq!(bar.right[0].content, "UTF-8");
    }

    #[test]
    fn test_status_bar_bg() {
        let bar = StatusBar::new().bg(Color::BLUE);
        assert_eq!(bar.bg, Color::BLUE);
    }

    #[test]
    fn test_status_bar_fg() {
        let bar = StatusBar::new().fg(Color::YELLOW);
        assert_eq!(bar.fg, Color::YELLOW);
    }

    #[test]
    fn test_status_bar_key() {
        let bar = StatusBar::new().key("Ctrl+S", "Save");
        assert_eq!(bar.key_hints.len(), 1);
        assert_eq!(bar.key_hints[0].key, "Ctrl+S");
        assert_eq!(bar.key_hints[0].description, "Save");
    }

    #[test]
    fn test_status_bar_keys() {
        let hints = vec![
            KeyHint::new("Ctrl+S", "Save"),
            KeyHint::new("Ctrl+Q", "Quit"),
        ];
        let bar = StatusBar::new().keys(hints);
        assert_eq!(bar.key_hints.len(), 2);
    }

    #[test]
    fn test_status_bar_separator() {
        let bar = StatusBar::new().separator('|');
        assert_eq!(bar.separator, Some('|'));
    }

    #[test]
    fn test_status_bar_height() {
        let bar = StatusBar::new().height(2);
        assert_eq!(bar.height, 2);
    }

    #[test]
    fn test_status_bar_height_minimum() {
        let bar = StatusBar::new().height(0);
        assert_eq!(bar.height, 1); // Minimum is 1
    }

    // =========================================================================
    // StatusBar update method tests
    // =========================================================================

    #[test]
    fn test_status_bar_update_center() {
        let mut bar = StatusBar::new().center_text("Original");
        bar.update_center(0, "Updated");
        assert_eq!(bar.center[0].content, "Updated");
    }

    #[test]
    fn test_status_bar_update_right() {
        let mut bar = StatusBar::new().right_text("Original");
        bar.update_right(0, "Updated");
        assert_eq!(bar.right[0].content, "Updated");
    }

    #[test]
    fn test_status_bar_update_invalid_index() {
        let mut bar = StatusBar::new().left_text("Test");
        bar.update_left(5, "Won't update");
        assert_eq!(bar.left[0].content, "Test"); // Unchanged
    }

    #[test]
    fn test_status_bar_clear() {
        let mut bar = StatusBar::new()
            .left_text("L")
            .center_text("C")
            .right_text("R")
            .key("Ctrl+S", "Save");

        bar.clear();
        assert!(bar.left.is_empty());
        assert!(bar.center.is_empty());
        assert!(bar.right.is_empty());
        assert!(bar.key_hints.is_empty());
    }

    // =========================================================================
    // StatusBar Default trait tests
    // =========================================================================

    #[test]
    fn test_status_bar_default() {
        let bar = StatusBar::default();
        assert!(bar.left.is_empty());
        assert!(bar.center.is_empty());
        assert!(bar.right.is_empty());
        assert_eq!(bar.position, StatusBarPosition::Bottom);
    }

    // =========================================================================
    // StatusBar builder chain tests
    // =========================================================================

    #[test]
    fn test_status_bar_builder_chain() {
        let bar = StatusBar::new()
            .header()
            .bg(Color::BLUE)
            .fg(Color::WHITE)
            .separator('|')
            .height(2)
            .left_text("File.txt")
            .center_text("Line 1")
            .right_text("UTF-8")
            .key("Ctrl+S", "Save")
            .key("Ctrl+Q", "Quit");

        assert_eq!(bar.position, StatusBarPosition::Top);
        assert_eq!(bar.bg, Color::BLUE);
        assert_eq!(bar.fg, Color::WHITE);
        assert_eq!(bar.separator, Some('|'));
        assert_eq!(bar.height, 2);
        assert_eq!(bar.left.len(), 1);
        assert_eq!(bar.center.len(), 1);
        assert_eq!(bar.right.len(), 1);
        assert_eq!(bar.key_hints.len(), 2);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_statusbar_helper() {
        let bar = statusbar();
        assert!(bar.left.is_empty());
        assert_eq!(bar.position, StatusBarPosition::Bottom);
    }

    #[test]
    fn test_header_helper() {
        let bar = header();
        assert_eq!(bar.position, StatusBarPosition::Top);
    }

    #[test]
    fn test_footer_helper() {
        let bar = footer();
        assert_eq!(bar.position, StatusBarPosition::Bottom);
    }

    #[test]
    fn test_section_helper() {
        let s = section("Content");
        assert_eq!(s.content, "Content");
    }

    #[test]
    fn test_key_hint_helper() {
        let hint = key_hint("Ctrl+C", "Copy");
        assert_eq!(hint.key, "Ctrl+C");
        assert_eq!(hint.description, "Copy");
    }

    // =========================================================================
    // StatusBar render_y tests
    // =========================================================================

    #[test]
    fn test_render_y_top() {
        let bar = StatusBar::new().position(StatusBarPosition::Top);
        assert_eq!(bar.render_y(24), 0);
    }

    #[test]
    fn test_render_y_bottom() {
        let bar = StatusBar::new().position(StatusBarPosition::Bottom);
        assert_eq!(bar.render_y(24), 23);
    }

    #[test]
    fn test_render_y_bottom_with_height() {
        let bar = StatusBar::new()
            .position(StatusBarPosition::Bottom)
            .height(2);
        assert_eq!(bar.render_y(24), 22);
    }
}
