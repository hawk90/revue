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

        let thumb_position = ((viewport_height - thumb_height) as f32 * scroll_ratio)
            .max(0.0)
            .min((viewport_height - thumb_height) as f32) as u16;

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
    use crate::render::Buffer;

    // =========================================================================
    // ScrollView::new and default tests
    // =========================================================================

    #[test]
    fn test_scroll_view_new() {
        let sv = ScrollView::new();
        assert_eq!(sv.offset(), 0);
        assert!(sv.show_scrollbar);
        assert_eq!(sv.content_height, 0);
    }

    #[test]
    fn test_scroll_view_new_default_colors() {
        let sv = ScrollView::new();
        assert_eq!(sv.scrollbar_fg, Some(Color::WHITE));
        assert_eq!(sv.scrollbar_bg, Some(Color::rgb(64, 64, 64)));
    }

    #[test]
    fn test_scroll_view_default() {
        let sv = ScrollView::default();
        assert_eq!(sv.offset(), 0);
        assert!(sv.show_scrollbar);
    }

    // =========================================================================
    // ScrollView builder tests
    // =========================================================================

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
    fn test_scroll_view_content_height() {
        let sv = ScrollView::new().content_height(50);
        assert_eq!(sv.content_height, 50);
    }

    #[test]
    fn test_scroll_view_scroll_offset() {
        let sv = ScrollView::new().scroll_offset(25);
        assert_eq!(sv.offset(), 25);
    }

    #[test]
    fn test_scroll_view_show_scrollbar_true() {
        let sv = ScrollView::new().show_scrollbar(true);
        assert!(sv.show_scrollbar);
    }

    #[test]
    fn test_scroll_view_show_scrollbar_false() {
        let sv = ScrollView::new().show_scrollbar(false);
        assert!(!sv.show_scrollbar);
    }

    #[test]
    fn test_scroll_view_scrollbar_style() {
        let sv = ScrollView::new().scrollbar_style(Color::RED, Color::BLUE);
        assert_eq!(sv.scrollbar_fg, Some(Color::RED));
        assert_eq!(sv.scrollbar_bg, Some(Color::BLUE));
    }

    #[test]
    fn test_scroll_view_builder_chain() {
        let sv = ScrollView::new()
            .content_height(100)
            .scroll_offset(10)
            .show_scrollbar(true)
            .scrollbar_style(Color::CYAN, Color::BLACK);

        assert_eq!(sv.content_height, 100);
        assert_eq!(sv.offset(), 10);
        assert!(sv.show_scrollbar);
        assert_eq!(sv.scrollbar_fg, Some(Color::CYAN));
        assert_eq!(sv.scrollbar_bg, Some(Color::BLACK));
    }

    // =========================================================================
    // ScrollView offset tests
    // =========================================================================

    #[test]
    fn test_scroll_view_offset() {
        let sv = ScrollView::new().scroll_offset(42);
        assert_eq!(sv.offset(), 42);
    }

    #[test]
    fn test_scroll_set_offset() {
        let mut sv = ScrollView::new().content_height(100);
        sv.set_offset(50, 20);
        assert_eq!(sv.offset(), 50);
    }

    #[test]
    fn test_scroll_set_offset_clamped() {
        let mut sv = ScrollView::new().content_height(50);
        sv.set_offset(100, 20);
        // Max offset is 50 - 20 = 30
        assert_eq!(sv.offset(), 30);
    }

    #[test]
    fn test_scroll_set_offset_zero_viewport() {
        let mut sv = ScrollView::new().content_height(100);
        sv.set_offset(50, 0);
        // Max offset is 100 - 0 = 100
        assert_eq!(sv.offset(), 50);
    }

    // =========================================================================
    // ScrollView scroll_up tests
    // =========================================================================

    #[test]
    fn test_scroll_up() {
        let mut sv = ScrollView::new().scroll_offset(10);
        sv.scroll_up(5);
        assert_eq!(sv.offset(), 5);
    }

    #[test]
    fn test_scroll_up_clamps_to_zero() {
        let mut sv = ScrollView::new().scroll_offset(5);
        sv.scroll_up(10);
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_scroll_up_already_zero() {
        let mut sv = ScrollView::new();
        sv.scroll_up(10);
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_scroll_up_zero_lines() {
        let mut sv = ScrollView::new().scroll_offset(10);
        sv.scroll_up(0);
        assert_eq!(sv.offset(), 10);
    }

    // =========================================================================
    // ScrollView scroll_down tests
    // =========================================================================

    #[test]
    fn test_scroll_down() {
        let mut sv = ScrollView::new().content_height(100);
        sv.scroll_down(5, 20);
        assert_eq!(sv.offset(), 5);
    }

    #[test]
    fn test_scroll_down_clamps_to_max() {
        let mut sv = ScrollView::new().content_height(50);
        sv.scroll_down(100, 20);
        // Max offset = 50 - 20 = 30
        assert_eq!(sv.offset(), 30);
    }

    #[test]
    fn test_scroll_down_zero_viewport() {
        let mut sv = ScrollView::new().content_height(100);
        sv.scroll_down(50, 0);
        // Max offset = 100 - 0 = 100
        assert_eq!(sv.offset(), 50);
    }

    #[test]
    fn test_scroll_down_zero_lines() {
        let mut sv = ScrollView::new().content_height(100).scroll_offset(10);
        sv.scroll_down(0, 20);
        assert_eq!(sv.offset(), 10);
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

    // =========================================================================
    // ScrollView scroll_to_edges tests
    // =========================================================================

    #[test]
    fn test_scroll_to_top() {
        let mut sv = ScrollView::new().scroll_offset(50);
        sv.scroll_to_top();
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_scroll_to_top_already_zero() {
        let mut sv = ScrollView::new();
        sv.scroll_to_top();
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_scroll_to_bottom() {
        let mut sv = ScrollView::new().content_height(100);
        sv.scroll_to_bottom(20);
        assert_eq!(sv.offset(), 80); // 100 - 20
    }

    #[test]
    fn test_scroll_to_bottom_zero_viewport() {
        let mut sv = ScrollView::new().content_height(50);
        sv.scroll_to_bottom(0);
        assert_eq!(sv.offset(), 50); // Full content height
    }

    #[test]
    fn test_scroll_to_bottom_viewport_larger() {
        let mut sv = ScrollView::new().content_height(50);
        sv.scroll_to_bottom(100);
        assert_eq!(sv.offset(), 0); // Can't scroll if viewport is larger
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

    // =========================================================================
    // ScrollView page navigation tests
    // =========================================================================

    #[test]
    fn test_page_down() {
        let mut sv = ScrollView::new().content_height(100);
        sv.page_down(20);
        assert_eq!(sv.offset(), 19); // viewport - 1
    }

    #[test]
    fn test_page_down_clamps() {
        let mut sv = ScrollView::new().content_height(50);
        sv.page_down(20);
        // Max offset = 50 - 20 = 30, page_down(20) = min(30, 20-1) = 19, clamped to 30... wait
        // page_down(viewport - 1) = page_down(19)
        // scroll_down(19, 20) = (0 + 19).min(30) = 19
        assert_eq!(sv.offset(), 19);
    }

    #[test]
    fn test_page_down_small_viewport() {
        let mut sv = ScrollView::new().content_height(100);
        sv.page_down(3);
        assert_eq!(sv.offset(), 2); // 3 - 1 = 2
    }

    #[test]
    fn test_page_down_single_line_viewport() {
        let mut sv = ScrollView::new().content_height(100);
        sv.page_down(1);
        assert_eq!(sv.offset(), 0); // 1 - 1 = 0, can't scroll
    }

    #[test]
    fn test_page_up() {
        let mut sv = ScrollView::new().content_height(100).scroll_offset(50);
        sv.page_up(20);
        // page_up(20) = scroll_up(viewport - 1) = scroll_up(19)
        // offset 50 - 19 = 31
        assert_eq!(sv.offset(), 31);
    }

    #[test]
    fn test_page_up_clamps() {
        let mut sv = ScrollView::new().content_height(100).scroll_offset(5);
        sv.page_up(20);
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_page_navigation() {
        let mut sv = ScrollView::new().content_height(100);

        sv.page_down(20);
        assert_eq!(sv.offset(), 19); // viewport - 1

        sv.page_up(20);
        assert_eq!(sv.offset(), 0);
    }

    // =========================================================================
    // ScrollView handle_key tests
    // =========================================================================

    #[test]
    fn test_handle_key_down() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);
        let changed = sv.handle_key(&Key::Down, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 1);
    }

    #[test]
    fn test_handle_key_up() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100).scroll_offset(5);
        let changed = sv.handle_key(&Key::Up, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 4);
    }

    #[test]
    fn test_handle_key_j_vim() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);
        let changed = sv.handle_key(&Key::Char('j'), 20);
        assert!(changed);
        assert_eq!(sv.offset(), 1);
    }

    #[test]
    fn test_handle_key_k_vim() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100).scroll_offset(5);
        let changed = sv.handle_key(&Key::Char('k'), 20);
        assert!(changed);
        assert_eq!(sv.offset(), 4);
    }

    #[test]
    fn test_handle_key_page_down() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);
        let changed = sv.handle_key(&Key::PageDown, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 19);
    }

    #[test]
    fn test_handle_key_page_up() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100).scroll_offset(50);
        let changed = sv.handle_key(&Key::PageUp, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 31);
    }

    #[test]
    fn test_handle_key_home() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100).scroll_offset(50);
        let changed = sv.handle_key(&Key::Home, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_handle_key_end() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);
        let changed = sv.handle_key(&Key::End, 20);
        assert!(changed);
        assert_eq!(sv.offset(), 80);
    }

    #[test]
    fn test_handle_key_unhandled() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);
        let changed = sv.handle_key(&Key::Left, 20);
        assert!(!changed);
        assert_eq!(sv.offset(), 0);
    }

    #[test]
    fn test_handle_key_at_bounds() {
        use crate::event::Key;

        let mut sv = ScrollView::new().content_height(100);

        let changed = sv.handle_key(&Key::Up, 20);
        assert!(!changed); // Already at top

        sv.scroll_to_bottom(20);
        let changed = sv.handle_key(&Key::Down, 20);
        assert!(!changed); // Already at bottom
    }

    // =========================================================================
    // ScrollView is_scrollable tests
    // =========================================================================

    #[test]
    fn test_is_scrollable() {
        let sv = ScrollView::new().content_height(50);

        assert!(sv.is_scrollable(20)); // 50 > 20
        assert!(!sv.is_scrollable(50)); // 50 == 50
        assert!(!sv.is_scrollable(100)); // 50 < 100
    }

    #[test]
    fn test_is_scrollable_empty() {
        let sv = ScrollView::new();
        assert!(!sv.is_scrollable(20));
    }

    #[test]
    fn test_is_scrollable_equal() {
        let sv = ScrollView::new().content_height(50);
        assert!(!sv.is_scrollable(50));
    }

    // =========================================================================
    // ScrollView scroll_percentage tests
    // =========================================================================

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
    fn test_scroll_percentage_no_scroll() {
        let sv = ScrollView::new().content_height(50);
        assert_eq!(sv.scroll_percentage(50), 0.0); // Can't scroll
    }

    #[test]
    fn test_scroll_percentage_quarter() {
        let mut sv = ScrollView::new().content_height(100);
        sv.set_offset(20, 20); // 20 / 80 = 0.25
        assert!((sv.scroll_percentage(20) - 0.25).abs() < 0.01);
    }

    #[test]
    fn test_scroll_percentage_three_quarters() {
        let mut sv = ScrollView::new().content_height(100);
        sv.set_offset(60, 20); // 60 / 80 = 0.75
        assert!((sv.scroll_percentage(20) - 0.75).abs() < 0.01);
    }

    // =========================================================================
    // ScrollView content_area tests
    // =========================================================================

    #[test]
    fn test_content_area() {
        let sv = ScrollView::new().content_height(100);
        let area = Rect::new(0, 0, 80, 24);

        let content_area = sv.content_area(area);
        assert_eq!(content_area.width, 79); // -1 for scrollbar
        assert_eq!(content_area.height, 24);
        assert_eq!(content_area.x, 0);
        assert_eq!(content_area.y, 0);
    }

    #[test]
    fn test_content_area_no_scrollbar_needed() {
        let sv = ScrollView::new().content_height(10);
        let area = Rect::new(0, 0, 80, 24);

        let content_area = sv.content_area(area);
        assert_eq!(content_area.width, 80); // No scrollbar needed
    }

    #[test]
    fn test_content_area_scrollbar_disabled() {
        let sv = ScrollView::new().content_height(100).show_scrollbar(false);
        let area = Rect::new(0, 0, 80, 24);

        let content_area = sv.content_area(area);
        assert_eq!(content_area.width, 80); // Scrollbar disabled
    }

    #[test]
    fn test_content_area_offset() {
        let sv = ScrollView::new().content_height(100);
        let area = Rect::new(5, 10, 80, 24);

        let content_area = sv.content_area(area);
        assert_eq!(content_area.x, 5);
        assert_eq!(content_area.y, 10);
    }

    // =========================================================================
    // ScrollView create_content_buffer tests
    // =========================================================================

    #[test]
    fn test_create_content_buffer() {
        let sv = ScrollView::new().content_height(50);
        let buffer = sv.create_content_buffer(20);

        assert_eq!(buffer.width(), 20);
        assert_eq!(buffer.height(), 50);
    }

    #[test]
    fn test_create_content_buffer_large() {
        let sv = ScrollView::new().content_height(1000);
        let buffer = sv.create_content_buffer(200);

        assert_eq!(buffer.width(), 200);
        assert_eq!(buffer.height(), 1000);
    }

    // =========================================================================
    // Helper function tests
    // =========================================================================

    #[test]
    fn test_scroll_helper() {
        let sv = scroll_view().content_height(50);
        assert_eq!(sv.content_height, 50);
    }

    // =========================================================================
    // Render tests
    // =========================================================================

    #[test]
    fn test_render_scrollbar_with_content() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new()
            .content_height(50)
            .scrollbar_style(Color::RED, Color::BLUE);
        sv.render_scrollbar(&mut ctx);

        // Check scrollbar thumb at right edge (thumb overwrites track)
        // At offset 0, thumb is at the top
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '█');
        assert_eq!(buffer.get(19, 0).unwrap().fg, Some(Color::RED));
    }

    #[test]
    fn test_render_scrollbar_no_scrollbar_needed() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new()
            .content_height(5) // Less than viewport
            .show_scrollbar(true);
        sv.render_scrollbar(&mut ctx);

        // No scrollbar should be drawn
        assert_eq!(buffer.get(19, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_render_scrollbar_disabled() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new().content_height(50).show_scrollbar(false);
        sv.render_scrollbar(&mut ctx);

        // No scrollbar should be drawn
        assert_eq!(buffer.get(19, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_render_scrollbar_small_area() {
        let mut buffer = Buffer::new(2, 10);
        let area = Rect::new(0, 0, 2, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new().content_height(50);
        sv.render_scrollbar(&mut ctx);

        // Should render at minimum width (thumb overwrites track)
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '█');
    }

    #[test]
    fn test_render_scrollbar_thumb_position() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new().content_height(100).scroll_offset(50); // Halfway (50/80 = 0.625)
        sv.render_scrollbar(&mut ctx);

        // Thumb position: (10 - 1) * 0.625 = 5.625, rounds to 5
        // Thumb height: (10/100) * 10 = 1, so thumb is at y=5
        // Check thumb character (thumb overwrites track)
        let cell = buffer.get(19, 5).unwrap();
        assert_eq!(cell.symbol, '█');
        assert_eq!(cell.fg, Some(Color::WHITE));
    }

    #[test]
    fn test_render_scrollbar_at_top() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new().content_height(100).scroll_offset(0);
        sv.render_scrollbar(&mut ctx);

        // Thumb should be at top
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '█');
    }

    #[test]
    fn test_render_scrollbar_at_bottom() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let sv = ScrollView::new().content_height(100).scroll_offset(90);
        sv.render_scrollbar(&mut ctx);

        // Thumb should be near bottom
        // Max offset = 90 (100 - 10), scroll_ratio = 90/90 = 1.0
        // thumb_position = (10 - 1) * 1.0 = 9
        // So thumb at y=9
        assert_eq!(buffer.get(19, 9).unwrap().symbol, '█');
    }

    // =========================================================================
    // render_content tests
    // =========================================================================

    #[test]
    fn test_render_content_basic() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Create content buffer - use full width, render_content will use content_area
        let mut content = Buffer::new(20, 10);
        for y in 0..10 {
            for x in 0..20 {
                let mut cell = Cell::new((b'0' + (y % 10) as u8) as char);
                cell.fg = Some(Color::WHITE);
                content.set(x, y, cell);
            }
        }

        let sv = ScrollView::new()
            .content_height(10) // No scrollbar needed
            .scroll_offset(5);
        sv.render_content(&mut ctx, &content);

        // Since content_height (10) == viewport height (10), no scrollbar
        // content_area.width = 20, and we're at offset 5
        // Row 5 of content (which has '5') is rendered to y=0 of target
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '5');
        assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
    }

    #[test]
    fn test_render_content_with_scrollbar() {
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Content buffer full width (before scrollbar adjustment)
        let mut content = Buffer::new(20, 50);
        for y in 0..50 {
            for x in 0..20 {
                content.set(x, y, Cell::new('X'));
            }
        }

        let sv = ScrollView::new().content_height(50);
        sv.render_content(&mut ctx, &content);

        // Check scrollbar was rendered (thumb at top since offset=0)
        assert_eq!(buffer.get(19, 0).unwrap().symbol, '█');
        // Check content - content_area.width = 19 (20 - 1 for scrollbar)
        // So we should see content at x=0..18
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'X');
        assert_eq!(buffer.get(18, 0).unwrap().symbol, 'X');
    }
}
