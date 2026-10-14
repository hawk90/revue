//! HTTP Client widget rendering

use std::time::Duration;

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::utils::format_size_compact;
use crate::widget::theme::{DISABLED_FG, SECONDARY_TEXT, SEPARATOR_COLOR};
use crate::widget::traits::{RenderContext, View};

use super::response::HttpResponse;
use super::types::RequestState;
use super::types::ResponseView;
use super::HttpClient;

impl View for HttpClient {
    crate::impl_view_meta!("HttpClient");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 40 || area.height < 10 {
            return;
        }

        self.draw_url_bar(ctx);

        // Response area (row 2+)
        let response_y = 2u16;

        if self.state == RequestState::Sending {
            let loading = "⠋ Sending request...";
            ctx.draw_text_clipped(0, response_y, loading, Color::YELLOW, area.width);
        } else if let Some(error) = &self.error {
            let err_msg = format!("✗ Error: {}", error);
            ctx.draw_text_clipped(0, response_y, &err_msg, Color::RED, area.width);
        } else if let Some(response) = &self.response {
            self.draw_response(ctx, response, response_y);
        } else {
            // No response yet
            let msg = "Enter a URL and press Enter to send request";
            ctx.draw_text_clipped(0, response_y, msg, DISABLED_FG, area.width);
        }
    }
}

impl HttpClient {
    /// Rows 0-1: the method badge, the URL and the send hint on url_bg, then
    /// a separator.
    fn draw_url_bar(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // The method badge and the status class each carry their own color -
        // blue GET against red DELETE, green 2xx against red 4xx - and that is
        // how a request is read. The URL is the base a `color` rule reaches.
        let url_fg = ctx.css_color(Color::WHITE);

        // URL bar (row 0-1): the whole bar sits on url_bg, the method badge
        // on method_bg.
        let url_bg = self.colors.url_bg;
        for bx in 0..area.width {
            let mut cell = Cell::new(' ');
            cell.bg = Some(url_bg);
            ctx.set(bx, 0, cell);
        }

        let x = self.draw_method_badge(ctx);

        // URL
        let url_start = x + 1;
        ctx.draw_text_clipped_bg(
            url_start,
            0,
            self.request.url(),
            url_fg,
            url_bg,
            area.width.saturating_sub(url_start),
        );

        // Send button hint
        let hint = "[Enter: Send]";
        let hint_start = area.width.saturating_sub(hint.len() as u16);
        ctx.draw_text_clipped_bg(
            hint_start,
            0,
            hint,
            DISABLED_FG,
            url_bg,
            area.width.saturating_sub(hint_start),
        );

        // Separator
        for sx in 0..area.width {
            let mut cell = Cell::new('─');
            cell.fg = Some(SEPARATOR_COLOR);
            ctx.set(sx, 1, cell);
        }
    }

    /// The method name in bold, in the method's color on method_bg, from
    /// column 0. Returns the column after it.
    fn draw_method_badge(&self, ctx: &mut RenderContext) -> u16 {
        let area = ctx.area;

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
            cell.bg = Some(self.colors.method_bg);
            cell.modifier = Modifier::BOLD;
            ctx.set(x, 0, cell);
            x += cw;
        }
        x
    }

    /// The status line, the view tabs under it and the content of the
    /// selected view below them.
    fn draw_response(&self, ctx: &mut RenderContext, response: &HttpResponse, response_y: u16) {
        let area = ctx.area;

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

        let tab_y = response_y + 1;
        self.draw_tabs(ctx, tab_y);

        // Content
        let content_y = tab_y + 1;
        let content_height = area.height.saturating_sub(content_y);

        match self.view {
            ResponseView::Body | ResponseView::Raw => {
                self.draw_body(ctx, response, content_y, content_height)
            }
            ResponseView::Headers => self.draw_headers(ctx, response, content_y, content_height),
        }
    }

    /// The "Body Headers Raw" bar: the active view on tab_active, the rest of
    /// the row on tab_bg.
    fn draw_tabs(&self, ctx: &mut RenderContext, tab_y: u16) {
        let area = ctx.area;

        // Tabs
        let tabs = ["Body", "Headers", "Raw"];
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
    }

    /// Body and Raw: the body's lines from `body_scroll` on.
    fn draw_body(
        &self,
        ctx: &mut RenderContext,
        response: &HttpResponse,
        content_y: u16,
        content_height: u16,
    ) {
        let area = ctx.area;
        for (i, line) in response
            .body
            .lines()
            .skip(self.body_scroll)
            .take(content_height as usize)
            .enumerate()
        {
            ctx.draw_text_clipped(0, content_y + i as u16, line, SECONDARY_TEXT, area.width);
        }
    }

    /// Headers: one `key: value` row per header from `body_scroll` on, the
    /// key clipped to half the width.
    fn draw_headers(
        &self,
        ctx: &mut RenderContext,
        response: &HttpResponse,
        content_y: u16,
        content_height: u16,
    ) {
        let area = ctx.area;
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

    /// Format duration
    fn format_duration(d: Duration) -> String {
        let ms = d.as_millis();
        if ms < 1000 {
            format!("{}ms", ms)
        } else {
            format!("{:.2}s", d.as_secs_f64())
        }
    }

    /// Format size
    fn format_size(bytes: usize) -> String {
        format_size_compact(bytes as u64)
    }
}

// Without these, a widget reports a `WidgetMeta` it has no way to fill in:
// it takes part in the DOM and no rule can select it.
crate::impl_props_builders!(HttpClient);
