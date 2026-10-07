//! Drawing the debug overlay: panel placement, contents and border

use super::{DebugEvent, DebugOverlay, DebugPosition};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;
use crate::utils::{draw_text_overlay, truncate_to_width, truncate_with_suffix};
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

        // Fill background
        for y in rect.y..rect.y + rect.height {
            for x in rect.x..rect.x + rect.width {
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.symbol = ' ';
                    cell.bg = Some(self.config.bg_color);
                    cell.fg = Some(self.config.fg_color);
                }
            }
        }

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
            let fps_color = if self.metrics.fps() >= 30.0 {
                Color::rgb(100, 255, 100) // Green
            } else if self.metrics.fps() >= 15.0 {
                Color::rgb(255, 255, 100) // Yellow
            } else {
                Color::rgb(255, 100, 100) // Red
            };

            self.draw_text(
                buffer,
                x,
                y,
                truncate_to_width(&format!("FPS: {:.1}", self.metrics.fps()), inner_width),
                fps_color,
            );
            y += 1;

            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    truncate_to_width(
                        &format!("Frame: {:.2}ms", self.metrics.avg_frame_time_ms()),
                        inner_width,
                    ),
                    self.config.fg_color,
                );
                y += 1;
            }

            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    truncate_to_width(
                        &format!("Layout: {:.2}ms", self.metrics.avg_layout_time_ms()),
                        inner_width,
                    ),
                    self.config.fg_color,
                );
                y += 1;
            }

            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    truncate_to_width(
                        &format!("Render: {:.2}ms", self.metrics.avg_render_time_ms()),
                        inner_width,
                    ),
                    self.config.fg_color,
                );
                y += 1;
            }

            y += 1; // Spacing
        }

        // Widget tree
        if self.config.show_tree && y < max_y {
            self.draw_text(
                buffer,
                x,
                y,
                truncate_to_width("Widgets:", inner_width),
                self.config.accent_color,
            );
            y += 1;

            for widget in &self.widgets {
                if y >= max_y {
                    break;
                }
                let truncated = truncate_with_suffix(&widget.tree_line(), inner_width, "...");
                let color = if widget.focused {
                    self.config.accent_color
                } else {
                    self.config.fg_color
                };
                self.draw_text(buffer, x, y, &truncated, color);
                y += 1;
            }

            y += 1; // Spacing
        }

        // Event log
        if self.config.show_events && y < max_y {
            self.draw_text(
                buffer,
                x,
                y,
                truncate_to_width("Events:", inner_width),
                self.config.accent_color,
            );
            y += 1;

            for (_, event) in self.events.recent(5) {
                if y >= max_y {
                    break;
                }
                let text = match event {
                    DebugEvent::KeyPress(k) => format!("Key: {}", k),
                    DebugEvent::Mouse(m) => format!("Mouse: {}", m),
                    DebugEvent::StateChange(s) => format!("State: {}", s),
                    DebugEvent::Custom(c) => c.clone(),
                };
                let truncated = truncate_with_suffix(&text, inner_width, "...");
                self.draw_text(buffer, x, y, &truncated, self.config.fg_color);
                y += 1;
            }
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

    /// Draw text at position
    fn draw_text(&self, buffer: &mut Buffer, x: u16, y: u16, text: &str, color: Color) {
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
