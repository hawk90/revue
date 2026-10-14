//! The panel's sections: performance metrics, widget tree and event log.
//!
//! Each draws its lines from row `y`, stops before `max_y` (the bottom
//! border) and returns the row after it and its spacing.

use super::{DebugEvent, DebugOverlay};
use crate::render::Buffer;
use crate::style::Color;
use crate::utils::{truncate_to_width, truncate_with_suffix};
use crate::widget::View;

/// FPS: green from 30, yellow from 15, red below.
fn fps_color(fps: f64) -> Color {
    if fps >= 30.0 {
        Color::rgb(100, 255, 100) // Green
    } else if fps >= 15.0 {
        Color::rgb(255, 255, 100) // Yellow
    } else {
        Color::rgb(255, 100, 100) // Red
    }
}

/// One event as the log lists it.
fn event_line(event: &DebugEvent) -> String {
    match event {
        DebugEvent::KeyPress(k) => format!("Key: {}", k),
        DebugEvent::Mouse(m) => format!("Mouse: {}", m),
        DebugEvent::StateChange(s) => format!("State: {}", s),
        DebugEvent::Custom(c) => c.clone(),
    }
}

impl<V: View> DebugOverlay<V> {
    /// FPS, then the average frame, layout and render times.
    pub(super) fn draw_metrics(
        &self,
        buffer: &mut Buffer,
        x: u16,
        mut y: u16,
        max_y: u16,
        inner_width: usize,
    ) -> u16 {
        let fps = self.metrics.fps();
        self.draw_text(
            buffer,
            x,
            y,
            truncate_to_width(&format!("FPS: {:.1}", fps), inner_width),
            fps_color(fps),
        );
        y += 1;

        let times = [
            ("Frame", self.metrics.avg_frame_time_ms()),
            ("Layout", self.metrics.avg_layout_time_ms()),
            ("Render", self.metrics.avg_render_time_ms()),
        ];
        for (label, ms) in times {
            if y < max_y {
                self.draw_text(
                    buffer,
                    x,
                    y,
                    truncate_to_width(&format!("{}: {:.2}ms", label, ms), inner_width),
                    self.config.fg_color,
                );
                y += 1;
            }
        }

        y + 1 // Spacing
    }

    /// "Widgets:", then one tree line per recorded widget; the focused one
    /// in the accent color.
    pub(super) fn draw_tree(
        &self,
        buffer: &mut Buffer,
        x: u16,
        mut y: u16,
        max_y: u16,
        inner_width: usize,
    ) -> u16 {
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

        y + 1 // Spacing
    }

    /// "Events:", then the five most recent events, newest first.
    pub(super) fn draw_events(
        &self,
        buffer: &mut Buffer,
        x: u16,
        mut y: u16,
        max_y: u16,
        inner_width: usize,
    ) {
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
            let truncated = truncate_with_suffix(&event_line(event), inner_width, "...");
            self.draw_text(buffer, x, y, &truncated, self.config.fg_color);
            y += 1;
        }
    }
}
