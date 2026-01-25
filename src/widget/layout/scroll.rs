//! ScrollView widget for scrollable content

use crate::layout::Rect;
use crate::render::{Buffer, Cell};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// A scrollable view widget
pub struct ScrollView {
    content_height: u16,
    scroll_offset: u16,
    show_scrollbar: bool,
    scrollbar_fg: Option<Color>,
    scrollbar_bg: Option<Color>,
    /// CSS styling properties (id, classes)
    props: WidgetProps,
}

impl ScrollView {
    /// Create a new scroll view
    pub fn new() -> Self {
        Self {
            content_height: 0,
            scroll_offset: 0,
            show_scrollbar: true,
            scrollbar_fg: Some(Color::WHITE),
            scrollbar_bg: Some(Color::rgb(64, 64, 64)),
            props: WidgetProps::new(),
        }
    }

    /// Set the total content height
    pub fn content_height(mut self, height: u16) -> Self {
        self.content_height = height;
        self
    }

    /// Set the scroll offset
    pub fn scroll_offset(mut self, offset: u16) -> Self {
        self.scroll_offset = offset;
        self
    }

    /// Enable/disable scrollbar
    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    /// Set scrollbar colors
    pub fn scrollbar_style(mut self, fg: Color, bg: Color) -> Self {
        self.scrollbar_fg = Some(fg);
        self.scrollbar_bg = Some(bg);
        self
    }

    /// Get current scroll offset
    pub fn offset(&self) -> u16 {
        self.scroll_offset
    }

    /// Set scroll offset with bounds checking
    pub fn set_offset(&mut self, offset: u16, viewport_height: u16) {
        let max_offset = self.content_height.saturating_sub(viewport_height);
        self.scroll_offset = offset.min(max_offset);
    }

    /// Scroll down by lines
    pub fn scroll_down(&mut self, lines: u16, viewport_height: u16) {
        let max_offset = self.content_height.saturating_sub(viewport_height);
        self.scroll_offset = (self.scroll_offset + lines).min(max_offset);
    }

    /// Scroll up by lines
    pub fn scroll_up(&mut self, lines: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(lines);
    }

    /// Scroll to top
    pub fn scroll_to_top(&mut self) {
        self.scroll_offset = 0;
    }

    /// Scroll to bottom
    pub fn scroll_to_bottom(&mut self, viewport_height: u16) {
        let max_offset = self.content_height.saturating_sub(viewport_height);
        self.scroll_offset = max_offset;
    }

    /// Page down
    pub fn page_down(&mut self, viewport_height: u16) {
        self.scroll_down(viewport_height.saturating_sub(1), viewport_height);
    }

    /// Page up
    pub fn page_up(&mut self, viewport_height: u16) {
        self.scroll_up(viewport_height.saturating_sub(1));
    }

    /// Handle key input, returns true if scroll changed
    pub fn handle_key(&mut self, key: &crate::event::Key, viewport_height: u16) -> bool {
        use crate::event::Key;

        let old_offset = self.scroll_offset;

        match key {
            Key::Up | Key::Char('k') => {
                self.scroll_up(1);
            }
            Key::Down | Key::Char('j') => {
                self.scroll_down(1, viewport_height);
            }
            Key::PageUp => {
                self.page_up(viewport_height);
            }
            Key::PageDown => {
                self.page_down(viewport_height);
            }
            Key::Home => {
                self.scroll_to_top();
            }
            Key::End => {
                self.scroll_to_bottom(viewport_height);
            }
            _ => {}
        }

        old_offset != self.scroll_offset
    }

    /// Check if content is scrollable
    pub fn is_scrollable(&self, viewport_height: u16) -> bool {
        self.content_height > viewport_height
    }

    /// Get scroll percentage (0.0 - 1.0)
    pub fn scroll_percentage(&self, viewport_height: u16) -> f32 {
        let max_offset = self.content_height.saturating_sub(viewport_height);
        if max_offset == 0 {
            0.0
        } else {
            self.scroll_offset as f32 / max_offset as f32
        }
    }

    /// Render scrollbar
    pub fn render_scrollbar(&self, ctx: &mut RenderContext) {
        if !self.show_scrollbar {
            return;
        }

        let area = ctx.area;
        if area.width < 1 || area.height < 1 {
            return;
        }

        let viewport_height = area.height;
        if self.content_height <= viewport_height {
            return; // No scrollbar needed
        }

        let scrollbar_x = area.x + area.width - 1;

        // Calculate scrollbar thumb position and size
        let thumb_height = ((viewport_height as f32 / self.content_height as f32)
            * viewport_height as f32)
            .max(1.0) as u16;
        let thumb_height = thumb_height.max(1).min(viewport_height);

        let max_offset = self.content_height.saturating_sub(viewport_height);
        let scroll_ratio = if max_offset > 0 {
            self.scroll_offset as f32 / max_offset as f32
        } else {
            0.0
        };

        let thumb_position = ((viewport_height - thumb_height) as f32 * scroll_ratio) as u16;

        // Draw scrollbar track
        for y in 0..viewport_height {
            let mut cell = Cell::new('│');
            cell.fg = self.scrollbar_bg;
            ctx.buffer.set(scrollbar_x, area.y + y, cell);
        }

        // Draw scrollbar thumb
        for y in thumb_position..(thumb_position + thumb_height).min(viewport_height) {
            let mut cell = Cell::new('█');
            cell.fg = self.scrollbar_fg;
            ctx.buffer.set(scrollbar_x, area.y + y, cell);
        }
    }

    /// Get the visible area for content (excludes scrollbar)
    pub fn content_area(&self, area: Rect) -> Rect {
        if self.show_scrollbar && self.content_height > area.height {
            Rect {
                x: area.x,
                y: area.y,
                width: area.width.saturating_sub(1),
                height: area.height,
            }
        } else {
            area
        }
    }

    /// Create a clipped buffer for scrolled content
    pub fn create_content_buffer(&self, width: u16) -> Buffer {
        Buffer::new(width, self.content_height)
    }

    /// Render scrolled content from a pre-rendered buffer
    pub fn render_content(&self, ctx: &mut RenderContext, content_buffer: &Buffer) {
        let area = self.content_area(ctx.area);
        let viewport_height = area.height;

        for y in 0..viewport_height {
            let content_y = self.scroll_offset + y;
            if content_y >= self.content_height {
                break;
            }

            for x in 0..area.width {
                if let Some(cell) = content_buffer.get(x, content_y) {
                    ctx.buffer.set(area.x + x, area.y + y, *cell);
                }
            }
        }

        self.render_scrollbar(ctx);
    }
}

impl Default for ScrollView {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ScrollView {
    crate::impl_view_meta!("ScrollView");

    fn render(&self, ctx: &mut RenderContext) {
        // ScrollView alone just renders the scrollbar
        // Content should be rendered via render_content method
        self.render_scrollbar(ctx);
    }
}

impl_styled_view!(ScrollView);
impl_props_builders!(ScrollView);

/// Helper function to create a scroll view
pub fn scroll_view() -> ScrollView {
    ScrollView::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scroll_view_new() {
        let sv = ScrollView::new();
        assert_eq!(sv.offset(), 0);
        assert!(sv.show_scrollbar);
    }

    #[test]
    fn test_scroll_view_builder() {
        let sv = ScrollView::new()
            .content_height(100)
            .scroll_offset(10)
            .show_scrollbar(false);

        assert_eq!(sv.content_height, 100);
        assert_eq!(sv.offset(), 10);
        assert!(!sv.show_scrollbar);
    }

    #[test]
    fn test_scroll_down_up() {
        let mut sv = ScrollView::new().content_height(100);

        sv.scroll_down(5, 20);
        assert_eq!(sv.offset(), 5);

        sv.scroll_up(3);
        assert_eq!(sv.offset(), 2);

        sv.scroll_up(10); // Should clamp to 0
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_scroll_bounds() {
        let mut sv = ScrollView::new().content_height(50);

        // Viewport of 20, content of 50 means max offset is 30
        sv.scroll_down(100, 20);
        assert_eq!(sv.offset(), 30);

        sv.set_offset(50, 20);
        assert_eq!(sv.offset(), 30); // Clamped to max
    }

    #[test]
    fn test_scroll_to_edges() {
        let mut sv = ScrollView::new().content_height(100);

        sv.scroll_down(50, 20);
        sv.scroll_to_top();
        assert_eq!(sv.offset(), 0);

        sv.scroll_to_bottom(20);
        assert_eq!(sv.offset(), 80); // 100 - 20
    }

    #[test]
    fn test_page_navigation() {
        let mut sv = ScrollView::new().content_height(100);

        sv.page_down(20);
        assert_eq!(sv.offset(), 19); // viewport - 1

        sv.page_up(20);
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_handle_key() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);

        let changed = sv.handle_key(&Key::Down, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 1);

        let changed = sv.handle_key(&Key::Up, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 0);

        // No change when already at top
        let changed = sv.handle_key(&Key::Up, 20);
        assert!(!changed);
    }

    #[test]
    fn test_is_scrollable() {
        let sv = ScrollView::new().content_height(50);

        assert!(sv.is_scrollable(20)); // 50 > 20
        assert!(!sv.is_scrollable(50)); // 50 == 50
        assert!(!sv.is_scrollable(100)); // 50 < 100
    }

    #[test]
    fn test_scroll_percentage() {
        let mut sv = ScrollView::new().content_height(100);

        assert_eq!(sv.scroll_percentage(20), 0.0);

        sv.scroll_to_bottom(20);
        assert_eq!(sv.scroll_percentage(20), 1.0);

        sv.set_offset(40, 20);
        assert!((sv.scroll_percentage(20) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_content_area() {
        let sv = ScrollView::new().content_height(100);
        let area = Rect::new(0, 0, 80, 24);

        let content_area = sv.content_area(area);
        assert_eq!(content_area.width, 79); // -1 for scrollbar

        let sv_small = ScrollView::new().content_height(10);
        let content_area = sv_small.content_area(area);
        assert_eq!(content_area.width, 80); // No scrollbar needed
    }

    #[test]
    fn test_scroll_helper() {
        let sv = scroll_view().content_height(50);
        assert_eq!(sv.content_height, 50);
    }
}
