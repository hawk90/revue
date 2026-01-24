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
