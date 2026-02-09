//! Helper context for rendering devtools panels

use crate::devtools::DevToolsConfig;
use crate::render::Buffer;
use crate::style::Color;

/// Helper context for rendering devtools panels
pub struct RenderCtx<'a> {
    pub buffer: &'a mut Buffer,
    pub x: u16,
    #[allow(dead_code)]
    pub width: u16,
    #[allow(dead_code)]
    pub config: &'a DevToolsConfig,
}

impl<'a> RenderCtx<'a> {
    pub fn new(buffer: &'a mut Buffer, x: u16, width: u16, config: &'a DevToolsConfig) -> Self {
        Self {
            buffer,
            x,
            width,
            config,
        }
    }

    pub fn draw_text(&mut self, y: u16, text: &str, color: Color) {
        use crate::devtools::helpers::draw_text_overlay;
        draw_text_overlay(self.buffer, self.x, y, text, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_ctx_new() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 10, 80, &config);

        assert_eq!(ctx.x, 10);
        assert_eq!(ctx.width, 80);
    }

    #[test]
    fn test_render_ctx_fields_are_accessible() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 5, 100, &config);

        // Test public fields are accessible
        let _x = ctx.x;
        let _width = ctx.width;
        let _config = ctx.config;
    }

    #[test]
    fn test_render_ctx_draw_text() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let mut ctx = RenderCtx::new(&mut buffer, 5, 80, &config);

        // Should not panic
        ctx.draw_text(0, "Test text", Color::rgb(255, 0, 0));
        ctx.draw_text(1, "", Color::default());
        ctx.draw_text(2, "Unicode: 你好", Color::rgb(0, 255, 0));
    }

    // =========================================================================
    // Additional devtools style helper tests
    // =========================================================================

    #[test]
    fn test_render_ctx_with_zero_x() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 0, 80, &config);
        assert_eq!(ctx.x, 0);
    }

    #[test]
    fn test_render_ctx_with_zero_width() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 10, 0, &config);
        assert_eq!(ctx.width, 0);
    }

    #[test]
    fn test_render_ctx_with_large_values() {
        let mut buffer = Buffer::new(1000, 1000);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 500, 500, &config);
        assert_eq!(ctx.x, 500);
        assert_eq!(ctx.width, 500);
    }

    #[test]
    fn test_render_ctx_draw_text_multiple_times() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let mut ctx = RenderCtx::new(&mut buffer, 0, 80, &config);

        for i in 0..10 {
            ctx.draw_text(i, &format!("Line {}", i), Color::WHITE);
        }
    }

    #[test]
    fn test_render_ctx_draw_text_with_special_chars() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let mut ctx = RenderCtx::new(&mut buffer, 0, 80, &config);

        ctx.draw_text(0, "Special: @#$%^&*()", Color::YELLOW);
        ctx.draw_text(1, "Tabs\t\tseparated", Color::CYAN);
        ctx.draw_text(2, "New\nLines", Color::MAGENTA);
    }

    #[test]
    fn test_render_ctx_with_custom_config() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 10, 80, &config);
        assert_eq!(ctx.width, 80);
    }

    #[test]
    fn test_render_ctx_draw_text_with_all_colors() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let mut ctx = RenderCtx::new(&mut buffer, 0, 80, &config);

        ctx.draw_text(0, "Red", Color::RED);
        ctx.draw_text(1, "Green", Color::GREEN);
        ctx.draw_text(2, "Blue", Color::BLUE);
        ctx.draw_text(3, "Yellow", Color::YELLOW);
        ctx.draw_text(4, "Cyan", Color::CYAN);
        ctx.draw_text(5, "Magenta", Color::MAGENTA);
        ctx.draw_text(6, "White", Color::WHITE);
        ctx.draw_text(7, "Black", Color::BLACK);
    }

    #[test]
    fn test_render_ctx_draw_text_out_of_bounds() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let mut ctx = RenderCtx::new(&mut buffer, 0, 80, &config);

        // Should handle out of bounds gracefully
        ctx.draw_text(100, "Out of bounds", Color::WHITE);
    }

    #[test]
    fn test_render_ctx_with_minimal_buffer() {
        let mut buffer = Buffer::new(1, 1);
        let config = DevToolsConfig::default();
        let mut ctx = RenderCtx::new(&mut buffer, 0, 1, &config);
        assert_eq!(ctx.width, 1);
        ctx.draw_text(0, "X", Color::WHITE);
    }

    #[test]
    fn test_render_ctx_x_at_right_edge() {
        let mut buffer = Buffer::new(80, 24);
        let config = DevToolsConfig::default();
        let ctx = RenderCtx::new(&mut buffer, 79, 1, &config);
        assert_eq!(ctx.x, 79);
    }
}
