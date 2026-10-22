//! Drawing the debug overlay: panel placement, background and border; the
//! sections inside are in `sections.rs`

use super::{DebugOverlay, DebugPosition};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::utils::{draw_text_overlay, truncate_to_width};
use crate::widget::{RenderContext, View};

impl<V: View> DebugOverlay<V> {
    /// Calculate panel rectangle
    fn panel_rect(&self, area: Rect) -> Rect {
        let width = self.config.width.min(area.width);
        let height = self.config.max_height.min(area.height);

        let (x, y) = match self.config.position {
            DebugPosition::TopLeft => (area.x, area.y),
            DebugPosition::TopRight => (area.x + area.width - width, area.y),
            DebugPosition::BottomLeft => (area.x, area.y + area.height - height),
            DebugPosition::BottomRight => {
                (area.x + area.width - width, area.y + area.height - height)
            }
        };

        Rect::new(x, y, width, height)
    }

    /// Render debug panel
    fn render_panel(&self, buffer: &mut Buffer, rect: Rect) {
        // A bordered panel needs at least 2x2 cells
        if rect.width < 2 || rect.height < 2 {
            return;
        }
        let inner_width = (rect.width - 2) as usize;

        self.fill_panel(buffer, rect);

        // Content goes between the top border (which carries the title) and
        // the bottom border
        let mut y = rect.y + 1;
        let x = rect.x + 1;
        let max_y = rect.y + rect.height - 1;

        // Separator
        if y < max_y {
            self.draw_text(buffer, x, y, &"-".repeat(inner_width), self.config.fg_color);
            y += 1;
        }

        // Performance metrics
        if self.config.show_metrics && y < max_y {
            y = self.draw_metrics(buffer, x, y, max_y, inner_width);
        }

        // Widget tree
        if self.config.show_tree && y < max_y {
            y = self.draw_tree(buffer, x, y, max_y, inner_width);
        }

        // Event log
        if self.config.show_events && y < max_y {
            self.draw_events(buffer, x, y, max_y, inner_width);
        }

        // Border, then the title on top of it
        self.draw_border(buffer, rect);
        self.draw_text(
            buffer,
            x,
            rect.y,
            truncate_to_width(" Debug ", inner_width),
            self.config.accent_color,
        );
    }

    /// Fill background. A wide glyph of the view that the panel's edge cuts
    /// in half is blanked: the terminal erases it (#974).
    fn fill_panel(&self, buffer: &mut Buffer, rect: Rect) {
        for y in rect.y..rect.y + rect.height {
            let right = rect.x + rect.width;
            if rect.x > 0 && buffer.get(rect.x, y).is_some_and(|c| c.is_continuation()) {
                if let Some(cell) = buffer.get_mut(rect.x - 1, y) {
                    cell.symbol = ' ';
                }
            }
            if buffer.get(right, y).is_some_and(|c| c.is_continuation()) {
                if let Some(cell) = buffer.get_mut(right, y) {
                    cell.symbol = ' ';
                }
            }
            for x in rect.x..rect.x + rect.width {
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.symbol = ' ';
                    cell.bg = Some(self.config.bg_color);
                    cell.fg = Some(self.config.fg_color);
                }
            }
        }
    }

    /// Draw text at position
    pub(super) fn draw_text(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
        draw_text_overlay(buffer, x, y, text, color);
    }

    /// Draw border around rect
    fn draw_border(&self, buffer: &mut Buffer, rect: Rect) {
        let border_color = self.config.accent_color;

        // Top and bottom
        for x in rect.x..rect.x + rect.width {
            if let Some(cell) = buffer.get_mut(x, rect.y) {
                cell.symbol = if x == rect.x {
                    '┌'
                } else if x == rect.x + rect.width - 1 {
                    '┐'
                } else {
                    '─'
                };
                cell.fg = Some(border_color);
            }
            if let Some(cell) = buffer.get_mut(x, rect.y + rect.height - 1) {
                cell.symbol = if x == rect.x {
                    '└'
                } else if x == rect.x + rect.width - 1 {
                    '┘'
                } else {
                    '─'
                };
                cell.fg = Some(border_color);
            }
        }

        // Left and right
        for y in rect.y + 1..rect.y + rect.height - 1 {
            if let Some(cell) = buffer.get_mut(rect.x, y) {
                cell.symbol = '│';
                cell.fg = Some(border_color);
            }
            if let Some(cell) = buffer.get_mut(rect.x + rect.width - 1, y) {
                cell.symbol = '│';
                cell.fg = Some(border_color);
            }
        }
    }
}

impl<V: View> View for DebugOverlay<V> {
    /// Hidden, it is just the wrapped view and measures as that. Shown, its
    /// panel is placed within the whole area, so it fills.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        if self.visible {
            None
        } else {
            self.inner.measure(max_width, max_height)
        }
    }

    /// Hidden, what the wrapped view fills; shown, everything.
    fn fills(&self) -> crate::widget::Fill {
        if self.visible {
            crate::widget::Fill::BOTH
        } else {
            self.inner.fills()
        }
    }

    fn render(&self, ctx: &mut RenderContext) {
        // Render inner view
        self.inner.render(ctx);

        // Render debug overlay
        if self.visible {
            let panel_rect = self.panel_rect(ctx.area);
            self.render_panel(ctx.buffer, panel_rect);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::Text;

    #[test]
    fn test_panel_rect_positions() {
        let text = Text::new("test");
        let overlay = DebugOverlay::wrap(text)
            .width(20)
            .position(DebugPosition::TopLeft);

        let area = Rect::new(0, 0, 80, 24);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 0);
        assert_eq!(panel.y, 0);
        assert_eq!(panel.width, 20);
    }

    // =========================================================================
    // panel_rect tests for all positions
    // =========================================================================

    #[test]
    fn test_panel_rect_top_right() {
        let text = Text::new("test");
        let overlay = DebugOverlay::wrap(text)
            .width(20)
            .position(DebugPosition::TopRight);

        let area = Rect::new(0, 0, 100, 50);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 80);
        assert_eq!(panel.y, 0);
    }

    #[test]
    fn test_panel_rect_bottom_left() {
        let text = Text::new("test");
        let mut overlay = DebugOverlay::wrap(text).width(30);
        overlay.config.max_height = 15;
        overlay.config.position = DebugPosition::BottomLeft;

        let area = Rect::new(0, 0, 100, 50);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 0);
        assert_eq!(panel.y, 35);
    }

    #[test]
    fn test_panel_rect_bottom_right() {
        let text = Text::new("test");
        let mut overlay = DebugOverlay::wrap(text).width(25);
        overlay.config.max_height = 10;
        overlay.config.position = DebugPosition::BottomRight;

        let area = Rect::new(0, 0, 100, 50);
        let panel = overlay.panel_rect(area);

        assert_eq!(panel.x, 75);
        assert_eq!(panel.y, 40);
    }
}
