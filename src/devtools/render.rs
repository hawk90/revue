//! Drawing the DevTools panel: background, border, tab bar and the active tab

use super::{DevTools, DevToolsTab};
use crate::layout::Rect;
use crate::render::Buffer;

impl DevTools {
    /// Render devtools panel
    pub fn render(&self, buffer: &mut Buffer, area: Rect) {
        if let Some(panel) = self.panel_rect(area) {
            // The inspected widget's outline goes under the panel
            if self.config.active_tab == DevToolsTab::Inspector {
                self.inspector.render_bounds(buffer);
            }
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

        // Tab bar: needs a row between the top and bottom borders
        if area.height < 3 {
            return;
        }
        let tab_area = Rect::new(area.x + 1, area.y + 1, area.width - 2, 1);
        self.render_tabs(buffer, tab_area);

        // Content area: below the tab separator, above the bottom border
        if area.height < 5 {
            return;
        }
        let content_area = Rect::new(area.x + 1, area.y + 3, area.width - 2, area.height - 4);

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

        // Separator after tabs, if it fits above the bottom border
        if area.height < 4 {
            return;
        }
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
    fn test_render_panel_stays_inside_its_area() {
        let area_x = 2;
        let area_y = 2;
        for tab in DevToolsTab::all() {
            for h in 0..=4u16 {
                for w in 0..=4u16 {
                    let mut devtools = DevTools::new();
                    devtools.set_tab(*tab);
                    let mut buffer = Buffer::new(10, 10);
                    for y in 0..10 {
                        for x in 0..10 {
                            buffer.get_mut(x, y).unwrap().symbol = '#';
                        }
                    }

                    devtools.render_panel(&mut buffer, Rect::new(area_x, area_y, w, h));

                    for y in 0..10 {
                        for x in 0..10 {
                            let inside = (area_x..area_x + w).contains(&x)
                                && (area_y..area_y + h).contains(&y);
                            if !inside {
                                assert_eq!(
                                    buffer.get(x, y).unwrap().symbol,
                                    '#',
                                    "{tab:?} {w}x{h} panel wrote outside its area at ({x}, {y})"
                                );
                            }
                        }
                    }
                }
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

    /// A panel on `tab` with `items` rows of `text` in every tab, as the
    /// widget matrix builds it.
    fn populated(tab: DevToolsTab, text: &str, items: usize) -> DevTools {
        use crate::devtools::{EventType, RenderEvent, StateEntry, StateValue};
        let mut d = DevTools::new();
        d.set_visible(true);
        d.set_tab(tab);
        let root = d.inspector_mut().add_root(text);
        for i in 0..items {
            let item = format!("{text}{i}");
            d.inspector_mut().add_child(root, item.clone());
            d.state_mut().add(StateEntry::new(
                item.clone(),
                StateValue::String(text.into()),
            ));
            d.events_mut().log(EventType::KeyPress, item);
        }
        d.styles_mut().set_widget(text, Some(text.to_string()));
        d.styles_mut().add_class(text);
        let profiler = d.profiler_mut();
        profiler.start_recording();
        for i in 0..items {
            profiler.start_frame();
            profiler.record_render(RenderEvent::new(
                format!("{text}{i}"),
                std::time::Duration::from_micros(250),
            ));
            profiler.end_frame();
        }
        d
    }

    // Each tab advances a row cursor (`y += n`) past its header rows; in a
    // panel at the bottom of the coordinate space that used to overflow.
    #[test]
    fn test_render_tabs_at_the_bottom_of_the_coordinate_space() {
        for tab in DevToolsTab::all() {
            for h in 1..=8u16 {
                for w in [2, 30] {
                    let devtools = populated(*tab, "hello", 3);
                    let mut buffer = Buffer::new(4, 16);
                    devtools.render_panel(&mut buffer, Rect::new(1, u16::MAX - h, w, h));
                }
            }
        }
    }

    // Long rows - a long widget name, class list, component name, event or
    // action - are cut at the panel's right edge, by display width.
    #[test]
    fn test_render_tabs_clip_long_rows_to_the_area() {
        use crate::devtools::{Action, ProfilerView, SnapshotValue, TimeTravelView};
        let (area_x, area_y, w, h) = (2u16, 1u16, 40u16, 10u16);
        let texts = ["abc ".repeat(250), "가나다 ".repeat(100), "👩‍👩‍👧‍👦".repeat(60)];
        for text in &texts {
            for items in [3, 200] {
                for tab in DevToolsTab::all() {
                    let mut devtools = populated(*tab, text, items);
                    for i in 0..3 {
                        let state = [(format!("{text}{i}"), SnapshotValue::String(text.clone()))]
                            .into_iter()
                            .collect();
                        devtools
                            .time_travel_mut()
                            .record_action(Action::new(format!("{text}{i}")), state);
                    }
                    for view in 0..4 {
                        devtools.profiler_mut().set_view(ProfilerView::all()[view]);
                        devtools
                            .time_travel_mut()
                            .set_view(TimeTravelView::all()[view]);
                        let mut buffer = Buffer::new(w + 4, h + 2);
                        for y in 0..h + 2 {
                            for x in 0..w + 4 {
                                buffer.get_mut(x, y).unwrap().symbol = '#';
                            }
                        }
                        devtools.render_panel(&mut buffer, Rect::new(area_x, area_y, w, h));
                        for y in 0..h + 2 {
                            for x in 0..w + 4 {
                                let inside = (area_x..area_x + w).contains(&x)
                                    && (area_y..area_y + h).contains(&y);
                                let cell = buffer.get(x, y).unwrap();
                                assert!(
                                    inside || (cell.symbol == '#' && cell.bg.is_none()),
                                    "{tab:?} view {view} {items} items: wrote outside at ({x}, {y})"
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
