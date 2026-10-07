//! Drawing the time-travel tab: header, view tabs, timeline, diff, actions and state

use super::TimeTravelDebugger;
use crate::devtools::helpers::draw_text_overlay_clipped;
use crate::devtools::time_travel::TimeTravelView;
use crate::devtools::DevToolsConfig;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::Color;

impl TimeTravelDebugger {
    /// Render time travel debugger content
    pub fn render_content(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        // The row cursors saturate and every row is drawn only above `max_y`,
        // so a panel at the bottom of the coordinate space cannot overflow
        // them.
        let mut y = area.y;
        let max_y = area.y.saturating_add(area.height);
        if y >= max_y {
            return;
        }

        // Header with status
        let status = if self.paused { "⏸ PAUSED" } else { "● REC" };
        let header = format!(
            "{} | {} snapshots | pos {}/{}",
            status,
            self.snapshots.len(),
            self.position + 1,
            self.snapshots.len()
        );
        Self::draw_text(buffer, area, area.x, y, &header, config.accent_color);
        y = y.saturating_add(1);
        if y >= max_y {
            return;
        }

        // View tabs
        self.render_view_tabs(buffer, area.x, y, area.width, config);
        y = y.saturating_add(2);

        if y >= max_y {
            return;
        }

        // Content based on view
        let content_area = Rect::new(area.x, y, area.width, max_y.saturating_sub(y));

        match self.view {
            TimeTravelView::Timeline => self.render_timeline(buffer, content_area, config),
            TimeTravelView::Diff => self.render_diff(buffer, content_area, config),
            TimeTravelView::Actions => self.render_actions(buffer, content_area, config),
            TimeTravelView::State => self.render_state(buffer, content_area, config),
        }
    }

    fn render_view_tabs(
        &self,
        buffer: &mut Buffer,
        x: u16,
        y: u16,
        width: u16,
        config: &DevToolsConfig,
    ) {
        let mut px = x;
        for view in TimeTravelView::all() {
            let label = format!(" {} ", view.label());
            let is_active = *view == self.view;

            let (fg, bg) = if is_active {
                (config.bg_color, config.accent_color)
            } else {
                (config.fg_color, config.bg_color)
            };

            for ch in label.chars() {
                if px < x.saturating_add(width) {
                    if let Some(cell) = buffer.get_mut(px, y) {
                        cell.symbol = ch;
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                    }
                    px = px.saturating_add(1);
                }
            }
            px = px.saturating_add(1);
        }
    }

    fn render_timeline(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y.saturating_add(area.height);

        if self.snapshots.is_empty() {
            Self::draw_text(
                buffer,
                area,
                area.x,
                y,
                "No snapshots recorded",
                config.fg_color,
            );
            return;
        }

        // Draw timeline slider
        let slider_width = area.width.saturating_sub(4);
        if slider_width > 0 && self.snapshots.len() > 1 {
            let progress = self.position as f32 / (self.snapshots.len() - 1) as f32;
            let filled = (slider_width as f32 * progress) as u16;

            Self::draw_text(buffer, area, area.x, y, "[", config.fg_color);
            for i in 0..slider_width {
                let ch = if i == filled { '●' } else { '─' };
                let color = if i <= filled {
                    config.accent_color
                } else {
                    config.fg_color
                };
                if let Some(cell) = buffer.get_mut(area.x.saturating_add(1 + i), y) {
                    cell.symbol = ch;
                    cell.fg = Some(color);
                }
            }
            let end_x = area.x.saturating_add(1 + slider_width);
            Self::draw_text(buffer, area, end_x, y, "]", config.fg_color);
            y = y.saturating_add(2);
        }

        // List recent snapshots
        let start = self.scroll;
        for (i, snapshot) in self.snapshots.iter().enumerate().skip(start) {
            if y >= max_y {
                break;
            }

            let is_current = i == self.position;
            let marker = if is_current { "▸" } else { " " };
            let action_name = snapshot
                .action
                .as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("snapshot");
            let label = snapshot.label.as_deref().unwrap_or("");

            let line = if label.is_empty() {
                format!("{} #{}: {}", marker, snapshot.id, action_name)
            } else {
                format!("{} #{}: {} ({})", marker, snapshot.id, action_name, label)
            };

            let color = if is_current {
                config.accent_color
            } else {
                config.fg_color
            };
            Self::draw_text(buffer, area, area.x, y, &line, color);
            y = y.saturating_add(1);
        }
    }

    fn render_diff(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let mut y = area.y;
        let max_y = area.y.saturating_add(area.height);

        let diff = match self.current_diff() {
            Some(d) => d,
            None => {
                Self::draw_text(
                    buffer,
                    area,
                    area.x,
                    y,
                    "No previous snapshot to compare",
                    config.fg_color,
                );
                return;
            }
        };

        if diff.is_empty() {
            Self::draw_text(buffer, area, area.x, y, "No changes", config.fg_color);
            return;
        }

        // Added
        let added_color = Color::rgb(100, 200, 100);
        for (key, value) in &diff.added {
            if y >= max_y {
                break;
            }
            let line = format!("+ {}: {}", key, value.display());
            Self::draw_text(buffer, area, area.x, y, &line, added_color);
            y = y.saturating_add(1);
        }

        // Removed
        let removed_color = Color::rgb(200, 100, 100);
        for (key, value) in &diff.removed {
            if y >= max_y {
                break;
            }
            let line = format!("- {}: {}", key, value.display());
            Self::draw_text(buffer, area, area.x, y, &line, removed_color);
            y = y.saturating_add(1);
        }

        // Changed
        let changed_color = Color::rgb(200, 200, 100);
        for (key, (old, new)) in &diff.changed {
            if y >= max_y {
                break;
            }
            let line = format!("~ {}: {} → {}", key, old.display(), new.display());
            Self::draw_text(buffer, area, area.x, y, &line, changed_color);
            y = y.saturating_add(1);
        }
    }

    fn render_actions(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let y_start = area.y;
        let max_y = area.y.saturating_add(area.height);

        let actions: Vec<_> = self
            .snapshots
            .iter()
            .filter_map(|s| s.action.as_ref().map(|a| (s.id, a)))
            .collect();

        if actions.is_empty() {
            Self::draw_text(
                buffer,
                area,
                area.x,
                y_start,
                "No actions recorded",
                config.fg_color,
            );
            return;
        }

        // A bounded range: `y_start..` would overflow stepping past u16::MAX.
        for (y, (id, action)) in (y_start..max_y).zip(actions.iter().skip(self.scroll)) {
            let source = action.source.as_deref().unwrap_or("");
            let line = if source.is_empty() {
                format!("#{}: {}", id, action.name)
            } else {
                format!("#{}: {} ({})", id, action.name, source)
            };

            Self::draw_text(buffer, area, area.x, y, &line, config.fg_color);
        }
    }

    fn render_state(&self, buffer: &mut Buffer, area: Rect, config: &DevToolsConfig) {
        let y_start = area.y;
        let max_y = area.y.saturating_add(area.height);

        let snapshot = match self.current() {
            Some(s) => s,
            None => {
                Self::draw_text(
                    buffer,
                    area,
                    area.x,
                    y_start,
                    "No snapshot selected",
                    config.fg_color,
                );
                return;
            }
        };

        if snapshot.state.is_empty() {
            Self::draw_text(
                buffer,
                area,
                area.x,
                y_start,
                "Empty state",
                config.fg_color,
            );
            return;
        }

        let mut entries: Vec<_> = snapshot.state.iter().collect();
        entries.sort_by_key(|a| a.0);

        for (y, (key, value)) in (y_start..max_y).zip(entries.iter().skip(self.scroll)) {
            let line = format!("{}: {} ({})", key, value.display(), value.type_name());
            Self::draw_text(buffer, area, area.x, y, &line, config.fg_color);
        }
    }

    /// Draw `text` at `(x, y)`, cut by display width at `area`'s right edge.
    fn draw_text(buffer: &mut Buffer, area: Rect, x: u16, y: u16, text: &str, color: Color) {
        let max_x = area.x.saturating_add(area.width);
        draw_text_overlay_clipped(buffer, x, y, max_x, text, color, None);
    }
}
