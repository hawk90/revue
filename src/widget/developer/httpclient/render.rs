//! HTTP Client widget rendering

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::developer::httpclient::HttpClient;
use crate::widget::theme::{DISABLED_FG, SECONDARY_TEXT, SEPARATOR_COLOR};
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
        let mut x = 0u16;
        for ch in method_name.chars() {
            let cw = crate::utils::char_width(ch) as u16;
            if x + cw > area.width {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(method.color());
            cell.modifier = Modifier::BOLD;
            ctx.set(x, 0, cell);
            x += cw;
        }

        // URL
        let url_start = x + 1;
        ctx.draw_text_clipped(
            url_start,
            0,
            self.request.url(),
            Color::WHITE,
            area.width.saturating_sub(url_start),
        );

        // Send button hint
        let hint = "[Enter: Send]";
        let hint_start = area.width.saturating_sub(hint.len() as u16);
        ctx.draw_text_clipped(
            hint_start,
            0,
            hint,
            DISABLED_FG,
            area.width.saturating_sub(hint_start),
        );

        // Separator
        for sx in 0..area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(SEPARATOR_COLOR);
            ctx.set(sx, 1, cell);
        }

        // Response area (row 2+)
        let response_y = 2u16;

        if self.state == RequestState::Sending {
            let loading = "⠋ Sending request...";
            ctx.draw_text_clipped(0, response_y, loading, Color::YELLOW, area.width);
        } else if let Some(error) = &self.error {
            let err_msg = format!("✗ Error: {}", error);
            ctx.draw_text_clipped(0, response_y, &err_msg, Color::RED, area.width);
        } else if let Some(response) = &self.response {
            // Status line
            let status_line = format!(
                "{} {} • {} • {}",
                response.status,
                response.status_text,
                Self::format_duration(response.time),
                Self::format_size(response.size)
            );

            ctx.draw_text_clipped(
                0,
                response_y,
                &status_line,
                response.status_color(),
                area.width,
            );

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
                    cell.fg = Some(if is_active { Color::WHITE } else { DISABLED_FG });
                    cell.bg = Some(if is_active {
                        self.colors.tab_active
                    } else {
                        self.colors.tab_bg
                    });
                    ctx.set(tab_x, tab_y, cell);
                    tab_x += 1;
                }

                // Space between tabs
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.colors.tab_bg);
                ctx.set(tab_x, tab_y, cell);
                tab_x += 1;
            }

            // Fill rest of tab bar
            for fx in tab_x..area.width {
                let mut cell = Cell::new(' ');
                cell.bg = Some(self.colors.tab_bg);
                ctx.set(fx, tab_y, cell);
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
                        ctx.draw_text_clipped(
                            0,
                            content_y + i as u16,
                            line,
                            SECONDARY_TEXT,
                            area.width,
                        );
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
                        let y = content_y + i as u16;
                        let half = area.width / 2;

                        // Key
                        ctx.draw_text_clipped(0, y, key, self.colors.header_key, half);
                        let kx = (crate::utils::display_width(key) as u16).min(half);

                        // Colon
                        if kx + 2 < area.width {
                            let mut cell = Cell::new(':');
                            cell.fg = Some(DISABLED_FG);
                            ctx.set(kx, y, cell);

                            // Value
                            let vx = kx + 2;
                            ctx.draw_text_clipped(
                                vx,
                                y,
                                value,
                                self.colors.header_value,
                                area.width.saturating_sub(vx),
                            );
                        }
                    }
                }
            }
        } else {
            // No response yet
            let msg = "Enter a URL and press Enter to send request";
            ctx.draw_text_clipped(0, response_y, msg, DISABLED_FG, area.width);
        }
    }
}
