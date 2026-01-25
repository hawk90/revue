//! Collapsible widget - a single expandable/collapsible section
//!
//! Similar to HTML's `<details>/<summary>` elements.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{Collapsible, collapsible};
//!
//! // Basic usage
//! let details = Collapsible::new("More Info")
//!     .content("Hidden content here\nMultiple lines supported")
//!     .expanded(false);
//!
//! // With custom icons
//! let custom = collapsible("Settings")
//!     .icons('+', '-')
//!     .content("Configuration options...")
//!     .expanded(true);
//! ```

use crate::event::Key;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps, WidgetState};
use crate::{impl_styled_view, impl_widget_builders};

/// A single collapsible/expandable section widget
///
/// Like HTML's `<details>/<summary>`, this provides a toggleable
/// section with a header and hidden content.
pub struct Collapsible {
    /// Header/summary text
    title: String,
    /// Content lines (shown when expanded)
    content: Vec<String>,
    /// Whether the content is visible
    expanded: bool,
    /// Icon when collapsed
    collapsed_icon: char,
    /// Icon when expanded
    expanded_icon: char,
    /// Header foreground color
    header_fg: Color,
    /// Header background color (optional)
    header_bg: Option<Color>,
    /// Content foreground color
    content_fg: Color,
    /// Content background color (optional)
    content_bg: Option<Color>,
    /// Show border around content
    show_border: bool,
    /// Border color
    border_color: Color,
    /// Widget state (focused, disabled, etc.)
    state: WidgetState,
    /// Widget properties (id, classes)
    props: WidgetProps,
}

impl Collapsible {
    /// Create a new collapsible section with the given title
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            content: Vec::new(),
            expanded: false,
            collapsed_icon: '▶',
            expanded_icon: '▼',
            header_fg: Color::WHITE,
            header_bg: None,
            content_fg: Color::rgb(200, 200, 200),
            content_bg: None,
            show_border: true,
            border_color: Color::rgb(80, 80, 80),
            state: WidgetState::new(),
            props: WidgetProps::new(),
        }
    }

    /// Set content text (splits by newlines)
    pub fn content(mut self, text: impl Into<String>) -> Self {
        self.content = text.into().lines().map(|s| s.to_string()).collect();
        self
    }

    /// Add a single content line
    pub fn line(mut self, line: impl Into<String>) -> Self {
        self.content.push(line.into());
        self
    }

    /// Add multiple content lines
    pub fn lines(mut self, lines: &[&str]) -> Self {
        self.content.extend(lines.iter().map(|s| s.to_string()));
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set custom icons for collapsed/expanded states
    pub fn icons(mut self, collapsed: char, expanded: char) -> Self {
        self.collapsed_icon = collapsed;
        self.expanded_icon = expanded;
        self
    }

    /// Set header colors
    pub fn header_colors(mut self, fg: Color, bg: Option<Color>) -> Self {
        self.header_fg = fg;
        self.header_bg = bg;
        self
    }

    /// Set content colors
    pub fn content_colors(mut self, fg: Color, bg: Option<Color>) -> Self {
        self.content_fg = fg;
        self.content_bg = bg;
        self
    }

    /// Show/hide border around content
    pub fn border(mut self, show: bool) -> Self {
        self.show_border = show;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }

    /// Toggle expanded state
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }

    /// Expand the content
    pub fn expand(&mut self) {
        self.expanded = true;
    }

    /// Collapse the content
    pub fn collapse(&mut self) {
        self.expanded = false;
    }

    /// Check if expanded
    pub fn is_expanded(&self) -> bool {
        self.expanded
    }

    /// Set expanded state mutably
    pub fn set_expanded(&mut self, expanded: bool) {
        self.expanded = expanded;
    }

    /// Get the current icon based on state
    fn icon(&self) -> char {
        if self.expanded {
            self.expanded_icon
        } else {
            self.collapsed_icon
        }
    }

    /// Calculate total height needed
    pub fn height(&self) -> u16 {
        if self.expanded {
            let content_height = self.content.len() as u16;
            if self.show_border {
                // header + content + bottom border
                1 + content_height + 1
            } else {
                1 + content_height
            }
        } else {
            1 // Just header
        }
    }

    /// Handle keyboard input
    ///
    /// Returns `true` if the key was handled.
    ///
    /// Supported keys:
    /// - Enter/Space: Toggle expanded state
    /// - Right/l: Expand
    /// - Left/h: Collapse
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.state.disabled {
            return false;
        }

        match key {
            Key::Enter | Key::Char(' ') => {
                self.toggle();
                true
            }
            Key::Right | Key::Char('l') => {
                self.expand();
                true
            }
            Key::Left | Key::Char('h') => {
                self.collapse();
                true
            }
            _ => false,
        }
    }
}

impl Default for Collapsible {
    fn default() -> Self {
        Self::new("Details")
    }
}

impl View for Collapsible {
    crate::impl_view_meta!("Collapsible");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 || area.height < 1 {
            return;
        }

        let is_focused = self.state.focused || ctx.is_focused();
        let header_fg = if self.state.disabled {
            Color::rgb(100, 100, 100)
        } else {
            self.header_fg
        };

        // Render header line
        let mut x = area.x;

        // Background for header (if set)
        if let Some(bg) = self.header_bg {
            for dx in 0..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(area.x + dx, area.y, cell);
            }
        }

        // Icon
        let mut icon_cell = Cell::new(self.icon());
        icon_cell.fg = Some(header_fg);
        if let Some(bg) = self.header_bg {
            icon_cell.bg = Some(bg);
        }
        if is_focused {
            icon_cell.modifier |= Modifier::BOLD;
        }
        ctx.buffer.set(x, area.y, icon_cell);
        x += 2; // icon + space

        // Title
        let max_title_width = (area.width.saturating_sub(3)) as usize;
        for ch in self.title.chars().take(max_title_width) {
            let mut cell = Cell::new(ch);
            cell.fg = Some(header_fg);
            if let Some(bg) = self.header_bg {
                cell.bg = Some(bg);
            }
            if is_focused {
                cell.modifier |= Modifier::BOLD;
            }
            ctx.buffer.set(x, area.y, cell);
            x += 1;
        }

        // Render content if expanded
        if self.expanded && area.height > 1 {
            let content_start_y = area.y + 1;
            let available_height = area.height.saturating_sub(1);
            let content_width = area.width.saturating_sub(2);

            // Calculate how many lines we can show
            let lines_to_show = if self.show_border {
                available_height.saturating_sub(1) as usize
            } else {
                available_height as usize
            };

            // Draw content lines with left border
            for (i, line) in self.content.iter().take(lines_to_show).enumerate() {
                let y = content_start_y + i as u16;
                if y >= area.y + area.height {
                    break;
                }

                // Background for content
                if let Some(bg) = self.content_bg {
                    for dx in 0..area.width {
                        let mut cell = Cell::new(' ');
                        cell.bg = Some(bg);
                        ctx.buffer.set(area.x + dx, y, cell);
                    }
                }

                // Left border
                if self.show_border {
                    let mut border_cell = Cell::new('│');
                    border_cell.fg = Some(self.border_color);
                    ctx.buffer.set(area.x, y, border_cell);
                }

                // Content text
                let text_x = if self.show_border {
                    area.x + 2
                } else {
                    area.x + 1
                };
                let max_content_width = content_width.saturating_sub(1) as usize;

                for (ci, ch) in line.chars().enumerate() {
                    if ci >= max_content_width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(self.content_fg);
                    if let Some(bg) = self.content_bg {
                        cell.bg = Some(bg);
                    }
                    ctx.buffer.set(text_x + ci as u16, y, cell);
                }
            }

            // Draw bottom border
            if self.show_border {
                let bottom_y = content_start_y + lines_to_show.min(self.content.len()) as u16;
                if bottom_y < area.y + area.height {
                    // Corner
                    let mut corner = Cell::new('└');
                    corner.fg = Some(self.border_color);
                    ctx.buffer.set(area.x, bottom_y, corner);

                    // Horizontal line
                    let line_width = area.width.saturating_sub(1);
                    for dx in 1..line_width {
                        let mut line_cell = Cell::new('─');
                        line_cell.fg = Some(self.border_color);
                        ctx.buffer.set(area.x + dx, bottom_y, line_cell);
                    }
                }
            }
        }
    }
}

impl_styled_view!(Collapsible);
impl_widget_builders!(Collapsible);

/// Helper function to create a Collapsible widget
pub fn collapsible(title: impl Into<String>) -> Collapsible {
    Collapsible::new(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_collapsible_new() {
        let c = Collapsible::new("Test");
        assert_eq!(c.title, "Test");
        assert!(!c.expanded);
        assert!(c.content.is_empty());
    }

    #[test]
    fn test_collapsible_builder() {
        let c = Collapsible::new("Details")
            .content("Line 1\nLine 2")
            .expanded(true)
            .icons('+', '-');

        assert!(c.expanded);
        assert_eq!(c.content.len(), 2);
        assert_eq!(c.collapsed_icon, '+');
        assert_eq!(c.expanded_icon, '-');
    }

    #[test]
    fn test_collapsible_line() {
        let c = Collapsible::new("Info")
            .line("First line")
            .line("Second line")
            .line("Third line");

        assert_eq!(c.content.len(), 3);
        assert_eq!(c.content[0], "First line");
    }

    #[test]
    fn test_collapsible_lines() {
        let c = Collapsible::new("Info").lines(&["A", "B", "C"]);

        assert_eq!(c.content.len(), 3);
    }

    #[test]
    fn test_collapsible_toggle() {
        let mut c = Collapsible::new("Test");
        assert!(!c.is_expanded());

        c.toggle();
        assert!(c.is_expanded());

        c.toggle();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_collapsible_expand_collapse() {
        let mut c = Collapsible::new("Test");

        c.expand();
        assert!(c.is_expanded());

        c.collapse();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_collapsible_height() {
        let collapsed = Collapsible::new("Test").content("A\nB\nC");
        assert_eq!(collapsed.height(), 1);

        let expanded = Collapsible::new("Test")
            .content("A\nB\nC")
            .expanded(true)
            .border(true);
        // header (1) + content (3) + bottom border (1) = 5
        assert_eq!(expanded.height(), 5);

        let no_border = Collapsible::new("Test")
            .content("A\nB")
            .expanded(true)
            .border(false);
        // header (1) + content (2) = 3
        assert_eq!(no_border.height(), 3);
    }

    #[test]
    fn test_collapsible_icon() {
        let collapsed = Collapsible::new("Test");
        assert_eq!(collapsed.icon(), '▶');

        let expanded = Collapsible::new("Test").expanded(true);
        assert_eq!(expanded.icon(), '▼');

        let custom = Collapsible::new("Test").icons('[', ']');
        assert_eq!(custom.icon(), '[');
    }

    #[test]
    fn test_collapsible_handle_key() {
        let mut c = Collapsible::new("Test");

        assert!(c.handle_key(&Key::Enter));
        assert!(c.is_expanded());

        assert!(c.handle_key(&Key::Char(' ')));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Right));
        assert!(c.is_expanded());

        assert!(c.handle_key(&Key::Left));
        assert!(!c.is_expanded());

        assert!(!c.handle_key(&Key::Up)); // Not handled
    }

    #[test]
    fn test_collapsible_handle_key_disabled() {
        let mut c = Collapsible::new("Test").disabled(true);

        assert!(!c.handle_key(&Key::Enter));
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_collapsible_render_collapsed() {
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(0, 0, 30, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Collapsible::new("Click to expand");
        c.render(&mut ctx);

        // Check icon
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▶');
        // Check title starts
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 'C');
    }

    #[test]
    fn test_collapsible_render_expanded() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Collapsible::new("Details")
            .content("Hidden content")
            .expanded(true);
        c.render(&mut ctx);

        // Check icon changed
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '▼');
        // Check border
        assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
        // Check bottom corner
        assert_eq!(buffer.get(0, 2).unwrap().symbol, '└');
    }

    #[test]
    fn test_collapsible_render_no_border() {
        let mut buffer = Buffer::new(30, 10);
        let area = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Collapsible::new("Details")
            .content("Content")
            .expanded(true)
            .border(false);
        c.render(&mut ctx);

        // No border character at content start
        assert_ne!(buffer.get(0, 1).unwrap().symbol, '│');
    }

    #[test]
    fn test_collapsible_colors() {
        let c = Collapsible::new("Test")
            .header_colors(Color::CYAN, Some(Color::rgb(30, 30, 30)))
            .content_colors(Color::YELLOW, Some(Color::BLACK))
            .border_color(Color::GREEN);

        assert_eq!(c.header_fg, Color::CYAN);
        assert_eq!(c.header_bg, Some(Color::rgb(30, 30, 30)));
        assert_eq!(c.content_fg, Color::YELLOW);
        assert_eq!(c.border_color, Color::GREEN);
    }

    #[test]
    fn test_collapsible_helper() {
        let c = collapsible("Quick").content("Fast creation");
        assert_eq!(c.title, "Quick");
    }

    #[test]
    fn test_collapsible_default() {
        let c = Collapsible::default();
        assert_eq!(c.title, "Details");
    }
}
