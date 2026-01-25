//! HTTP Client widget rendering

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::developer::httpclient::HttpClient;
use crate::widget::traits::{RenderContext, View};

use super::types::RequestState;
use super::types::ResponseView;

impl View for HttpClient {
    crate::impl_view_meta!("HttpClient");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 40 || area.height < 10 {
            return;
        }

        // URL bar (row 0-1)
        // Method badge
        let method = self.request.method;
        let method_name = method.name();
        for (i, ch) in method_name.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(method.color());
            cell.modifier = Modifier::BOLD;
            ctx.buffer.set(area.x + i as u16, area.y, cell);
        }

        // URL
        let url_start = method_name.len() as u16 + 1;
        for (i, ch) in self.request.url.chars().enumerate() {
            if url_start + i as u16 >= area.width - 1 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::WHITE);
            ctx.buffer.set(area.x + url_start + i as u16, area.y, cell);
        }

        // Send button hint
        let hint = "[Enter: Send]";
        let hint_start = area.width.saturating_sub(hint.len() as u16);
        for (i, ch) in hint.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(100, 100, 100));
            ctx.buffer.set(area.x + hint_start + i as u16, area.y, cell);
        }

        // Separator
        for x in 0..area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(Color::rgb(60, 60, 60));
            ctx.buffer.set(area.x + x, area.y + 1, cell);
        }

        // Response area (row 2+)
        let response_y = 2u16;

        if self.state == RequestState::Sending {
            let loading = "⠋ Sending request...";
            for (i, ch) in loading.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::YELLOW);
                ctx.buffer.set(area.x + i as u16, area.y + response_y, cell);
            }
        } else if let Some(error) = &self.error {
            let err_msg = format!("✗ Error: {}", error);
            for (i, ch) in err_msg.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::RED);
                ctx.buffer.set(area.x + i as u16, area.y + response_y, cell);
            }
        } else if let Some(response) = &self.response {
            // Status line
            let status_line = format!(
                "{} {} • {} • {}",
                response.status,
                response.status_text,
                Self::format_duration(response.time),
                Self::format_size(response.size)
            );

            for (x, ch) in status_line.chars().enumerate() {
                let x = x as u16;
                if x >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(response.status_color());
                ctx.buffer.set(area.x + x, area.y + response_y, cell);
            }

            // Tabs
            let tabs = ["Body", "Headers", "Raw"];
            let tab_y = response_y + 1;
            let mut tab_x = 0u16;
            for (i, tab) in tabs.iter().enumerate() {
                let is_active = matches!(
                    (i, self.view),
                    (0, ResponseView::Body) | (1, ResponseView::Headers) | (2, ResponseView::Raw)
                );

                for ch in tab.chars() {
                    if tab_x >= area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if is_active {
                        Color::WHITE
                    } else {
                        Color::rgb(100, 100, 100)
                    });
                    cell.bg = Some(if is_active {
                        self.colors.tab_active
                    } else {
                        self.colors.tab_bg
                    });
                    ctx.buffer.set(area.x + tab_x, area.y + tab_y, cell);
                    tab_x += 1;
                }

                // Space between tabs
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.colors.tab_bg);
                ctx.buffer.set(area.x + tab_x, area.y + tab_y, cell);
                tab_x += 1;
            }

            // Fill rest of tab bar
            for x in tab_x..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.colors.tab_bg);
                ctx.buffer.set(area.x + x, area.y + tab_y, cell);
            }

            // Content
            let content_y = tab_y + 1;
            let content_height = area.height.saturating_sub(content_y);

            match self.view {
                ResponseView::Body | ResponseView::Raw => {
                    for (i, line) in response
                        .body
                        .lines()
                        .skip(self.body_scroll)
                        .take(content_height as usize)
                        .enumerate()
                    {
                        for (j, ch) in line.chars().enumerate() {
                            if j as u16 >= area.width {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(Color::rgb(200, 200, 200));
                            ctx.buffer
                                .set(area.x + j as u16, area.y + content_y + i as u16, cell);
                        }
                    }
                }
                ResponseView::Headers => {
                    for (i, (key, value)) in response
                        .headers
                        .iter()
                        .skip(self.body_scroll)
                        .take(content_height as usize)
                        .enumerate()
                    {
                        let y = area.y + content_y + i as u16;

                        // Key
                        for (j, ch) in key.chars().enumerate() {
                            if j as u16 >= area.width / 2 {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(self.colors.header_key);
                            ctx.buffer.set(area.x + j as u16, y, cell);
                        }

                        // Colon
                        let colon_x = key.len() as u16;
                        if colon_x + 2 < area.width {
                            let mut cell = Cell::new(':');
                            cell.fg = Some(Color::rgb(100, 100, 100));
                            ctx.buffer.set(area.x + colon_x, y, cell);

                            // Value
                            for (j, ch) in value.chars().enumerate() {
                                if colon_x + 2 + j as u16 >= area.width {
                                    break;
                                }
                                let mut cell = Cell::new(ch);
                                cell.fg = Some(self.colors.header_value);
                                ctx.buffer.set(area.x + colon_x + 2 + j as u16, y, cell);
                            }
                        }
                    }
                }
            }
        } else {
            // No response yet
            let msg = "Enter a URL and press Enter to send request";
            for (i, ch) in msg.chars().enumerate() {
                if i as u16 >= area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::rgb(100, 100, 100));
                ctx.buffer.set(area.x + i as u16, area.y + response_y, cell);
            }
        }
    }
}
