//! Drawing the DevTools panel: background, border, tab bar and the active tab

use super::{DevTools, DevToolsTab};
use crate::layout::Rect;
use crate::render::Buffer;

impl DevTools {
    /// Render devtools panel
    pub fn render(&self, buffer: &mut Buffer, area: Rect) {
        if let Some(panel) = self.panel_rect(area) {
            self.render_panel(buffer, panel);
        }
    }

    fn render_panel(&self, buffer: &mut Buffer, area: Rect) {
        // Fill background
        for y in area.y..area.y + area.height {
            for x in area.x..area.x + area.width {
                if let Some(cell) = buffer.get_mut(x, y) {
                    cell.symbol = ' ';
                    cell.bg = Some(self.config.bg_color);
                    cell.fg = Some(self.config.fg_color);
                }
            }
        }

        // Too small for a border; the background is all there is room for
        if area.width < 2 || area.height < 2 {
            return;
        }

        // Draw border
        self.draw_border(buffer, area);

        // Tab bar
        let tab_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, 1);
        self.render_tabs(buffer, tab_area);

        // Content area
        let content_area = Rect::new(
            area.x + 1,
            area.y + 3,
            area.width - 2,
            area.height.saturating_sub(4),
        );

        match self.config.active_tab {
            DevToolsTab::Inspector => {
                self.inspector
                    .render_content(buffer, content_area, &self.config)
            }
            DevToolsTab::State => self
                .state
                .render_content(buffer, content_area, &self.config),
            DevToolsTab::Styles => self
                .styles
                .render_content(buffer, content_area, &self.config),
            DevToolsTab::Events => self
                .events
                .render_content(buffer, content_area, &self.config),
            DevToolsTab::Profiler => {
                self.profiler
                    .render_content(buffer, content_area, &self.config)
            }
            DevToolsTab::TimeTravel => {
                self.time_travel
                    .render_content(buffer, content_area, &self.config)
            }
        }
    }

    fn render_tabs(&self, buffer: &mut Buffer, area: Rect) {
        let mut x = area.x;

        for tab in DevToolsTab::all() {
            let label = format!(" {} ", tab.label());
            let is_active = *tab == self.config.active_tab;

            let (fg, bg) = if is_active {
                (self.config.bg_color, self.config.accent_color)
            } else {
                (self.config.fg_color, self.config.bg_color)
            };

            for ch in label.chars() {
                if x < area.x + area.width {
                    if let Some(cell) = buffer.get_mut(x, area.y) {
                        cell.symbol = ch;
                        cell.fg = Some(fg);
                        cell.bg = Some(bg);
                    }
                    x += 1;
                }
            }

            x += 1; // Gap between tabs
        }
    }

    fn draw_border(&self, buffer: &mut Buffer, area: Rect) {
        let color = self.config.accent_color;

        // Corners and edges
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get_mut(x, area.y) {
                cell.symbol = if x == area.x {
                    '┌'
                } else if x == area.x + area.width - 1 {
                    '┐'
                } else {
                    '─'
                };
                cell.fg = Some(color);
            }
            if let Some(cell) = buffer.get_mut(x, area.y + area.height - 1) {
                cell.symbol = if x == area.x {
                    '└'
                } else if x == area.x + area.width - 1 {
                    '┘'
                } else {
                    '─'
                };
                cell.fg = Some(color);
            }
        }

        for y in area.y + 1..area.y + area.height - 1 {
            if let Some(cell) = buffer.get_mut(area.x, y) {
                cell.symbol = '│';
                cell.fg = Some(color);
            }
            if let Some(cell) = buffer.get_mut(area.x + area.width - 1, y) {
                cell.symbol = '│';
                cell.fg = Some(color);
            }
        }

        // Separator after tabs
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get_mut(x, area.y + 2) {
                cell.symbol = if x == area.x {
                    '├'
                } else if x == area.x + area.width - 1 {
                    '┤'
                } else {
                    '─'
                };
                cell.fg = Some(color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::devtools::DevToolsPosition;

    #[test]
    fn test_render_panel_in_tiny_areas() {
        for tab in DevToolsTab::all() {
            for (w, h) in [(0, 0), (1, 5), (5, 0), (1, 1), (2, 1), (2, 2), (5, 3)] {
                let mut devtools = DevTools::new().position(DevToolsPosition::Left).size(w);
                devtools.set_visible(true);
                devtools.set_tab(*tab);
                let mut buffer = Buffer::new(10, 10);
                devtools.render(&mut buffer, Rect::new(0, 0, 10, h));
            }
        }
    }

    #[test]
    fn test_render_zero_size_panel() {
        let mut devtools = DevTools::new().size(0);
        devtools.set_visible(true);
        let mut buffer = Buffer::new(10, 10);
        devtools.render(&mut buffer, Rect::new(0, 0, 10, 10));
        devtools.render(&mut buffer, Rect::new(0, 0, 0, 0));
    }
}
