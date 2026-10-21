//! HttpClient on the screen: placement, the size gate, each row's text and
//! colors, and models of the URL bar, the body and a header row
//!
//! The screen after `send()` is left out where it would compare the status
//! line: it shows the measured elapsed time.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use revue::widget::theme::{DISABLED_FG, SECONDARY_TEXT, SEPARATOR_COLOR};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{http_get, HttpClient, HttpColors, HttpMethod, HttpResponse, ResponseView};
use std::time::Duration;

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const METHODS: [HttpMethod; 7] = [
    HttpMethod::GET,
    HttpMethod::POST,
    HttpMethod::PUT,
    HttpMethod::DELETE,
    HttpMethod::PATCH,
    HttpMethod::HEAD,
    HttpMethod::OPTIONS,
];

fn draw(client: &HttpClient, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    client.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .filter(|&c| c != '\0')
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn cell(buffer: &Buffer, x: u16, y: u16) -> Cell {
    *buffer.get(x, y).unwrap()
}

fn response(
    status: u16,
    text: &str,
    headers: &[(&str, &str)],
    body: &str,
    ms: u64,
) -> HttpResponse {
    HttpResponse {
        status,
        status_text: text.to_string(),
        headers: headers
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
        body: body.to_string(),
        time: Duration::from_millis(ms),
        size: body.len(),
    }
}

fn with(resp: HttpResponse, view: ResponseView) -> HttpClient {
    let mut client = http_get("https://example.com");
    client.set_response(resp);
    client.set_view(view);
    client
}

// =========================================================================
// Placement and the size gate
// =========================================================================

#[test]
fn test_http_client_zero_areas_and_offset() {
    let mut error = http_get("https://example.com");
    error.set_error("boom");
    let clients = [
        HttpClient::new(),
        http_get("https://example.com/한글"),
        error,
        with(
            response(200, "OK", &[("K", "v")], "a\nb", 3),
            ResponseView::Body,
        ),
        with(
            response(404, "NF", &[("K", "v")], "a\nb", 3),
            ResponseView::Headers,
        ),
        with(response(500, "E", &[], "raw", 3), ResponseView::Raw),
    ];
    for client in clients {
        assert_zero_areas_draw_nothing(&client);
        assert_same_when_offset(client.clone(), 50, 12);
    }
}

#[test]
fn test_http_client_draws_nothing_below_forty_by_ten() {
    let client = with(response(200, "OK", &[], "body", 1), ResponseView::Body);
    for (w, h) in [(39, 10), (40, 9), (39, 9), (1, 1)] {
        let buffer = draw(&client, w, h);
        assert!(
            (0..h).all(|y| (0..w).all(|x| cell(&buffer, x, y) == Cell::default())),
            "{w}x{h}"
        );
    }
    let buffer = draw(&client, 40, 10);
    assert!(row(&buffer, 0).starts_with("GET https://example.com"));
    assert_eq!(row(&buffer, 4), "body");
}

// =========================================================================
// URL bar
// =========================================================================

#[test]
fn test_http_client_method_badge_per_method() {
    let colors = HttpColors::default();
    for method in METHODS {
        let buffer = draw(&http_get("https://e.x").method(method), 60, 10);
        let name = method.name();
        assert_eq!(
            row(&buffer, 0),
            format!(
                "{name} https://e.x{}[Enter: Send]",
                " ".repeat(60 - 13 - name.len() - 1 - 11)
            )
        );
        for x in 0..name.len() as u16 {
            let c = cell(&buffer, x, 0);
            assert_eq!(c.fg, Some(method.color()), "{name}");
            assert_eq!(c.bg, Some(colors.method_bg), "{name}");
            assert!(c.modifier.contains(Modifier::BOLD), "{name}");
        }
        // The gap after the badge and the URL are on url_bg, not bold.
        let gap = cell(&buffer, name.len() as u16, 0);
        assert_eq!((gap.symbol, gap.bg), (' ', Some(colors.url_bg)));
        let url = cell(&buffer, name.len() as u16 + 1, 0);
        assert_eq!(url.symbol, 'h');
        assert_eq!(url.fg, Some(Color::WHITE));
        assert!(!url.modifier.contains(Modifier::BOLD));
    }
}

#[test]
fn test_http_client_send_hint_covers_a_long_url() {
    let url = format!("https://example.com/{}", "x".repeat(80));
    let buffer = draw(&http_get(url.as_str()), 50, 10);
    // 50 - 13 = 37 columns before the hint: "GET " and 33 URL characters.
    assert_eq!(row(&buffer, 0), format!("GET {}[Enter: Send]", &url[..33]));
    for x in 37..50 {
        assert_eq!(cell(&buffer, x, 0).fg, Some(DISABLED_FG));
        assert_eq!(cell(&buffer, x, 0).bg, Some(HttpColors::default().url_bg));
    }
}

#[test]
fn test_http_client_empty_url() {
    let buffer = draw(&HttpClient::new(), 40, 10);
    assert_eq!(
        row(&buffer, 0),
        format!("GET{}[Enter: Send]", " ".repeat(24))
    );
    // The whole bar is on url_bg past the badge, even with nothing in it.
    for x in 3..27 {
        assert_eq!(cell(&buffer, x, 0).bg, Some(HttpColors::default().url_bg));
    }
}

#[test]
fn test_http_client_wide_url_keeps_its_continuation_cells() {
    let buffer = draw(&http_get("https://한글.kr"), 40, 10);
    assert_eq!(cell(&buffer, 12, 0).symbol, '한');
    assert!(cell(&buffer, 13, 0).is_continuation());
    assert_eq!(cell(&buffer, 14, 0).symbol, '글');
    assert!(cell(&buffer, 15, 0).is_continuation());
    assert_eq!(cell(&buffer, 16, 0).symbol, '.');
}

#[test]
fn test_http_client_separator_spans_the_width() {
    let buffer = draw(&HttpClient::new(), 57, 10);
    for x in 0..57 {
        assert_eq!(cell(&buffer, x, 1).symbol, '─');
        assert_eq!(cell(&buffer, x, 1).fg, Some(SEPARATOR_COLOR));
    }
}

// =========================================================================
// Row 2: placeholder, error, status line
// =========================================================================

#[test]
fn test_http_client_error_row_is_red_and_wins_over_an_older_response() {
    let mut client = with(response(200, "OK", &[], "body", 1), ResponseView::Body);
    client.set_error("Connection refused");
    let buffer = draw(&client, 60, 10);
    assert_eq!(row(&buffer, 2), "✗ Error: Connection refused");
    assert_eq!(cell(&buffer, 0, 2).fg, Some(Color::RED));
    // Nothing of the response below it: no tabs, no body.
    assert!((3..10).all(|y| row(&buffer, y).is_empty()));
}

#[test]
fn test_http_client_status_line_text_and_color_per_class() {
    for (status, ms, size, line) in [
        (200, 0, 0, "200 OK • 0ms • 0"),
        (204, 999, 1023, "204 OK • 999ms • 1023"),
        (301, 1000, 1024, "301 OK • 1.00s • 1K"),
        (404, 1500, 1536, "404 OK • 1.50s • 1.5K"),
        (503, 61_234, 3 * 1024 * 1024, "503 OK • 61.23s • 3M"),
        (99, 7, 5, "99 OK • 7ms • 5"),
    ] {
        let mut resp = response(status, "OK", &[], "", ms);
        resp.size = size;
        let color = resp.status_color();
        let buffer = draw(&with(resp, ResponseView::Body), 60, 10);
        assert_eq!(row(&buffer, 2), line);
        for x in 0..line.chars().count() as u16 {
            assert_eq!(cell(&buffer, x, 2).fg, Some(color), "{line} @{x}");
        }
    }
}

// =========================================================================
// Tabs and content
// =========================================================================

#[test]
fn test_http_client_tab_bar_marks_the_active_view() {
    let colors = HttpColors {
        tab_bg: Color::BLUE,
        tab_active: Color::RED,
        ..HttpColors::default()
    };
    // "Body Headers Raw ": the tabs start at 0, 5 and 13.
    let spans = [(0u16, 4u16), (5, 12), (13, 16)];
    for (i, view) in [ResponseView::Body, ResponseView::Headers, ResponseView::Raw]
        .into_iter()
        .enumerate()
    {
        let client = with(response(200, "OK", &[], "", 1), view).colors(colors.clone());
        let buffer = draw(&client, 45, 10);
        assert_eq!(row(&buffer, 3), "Body Headers Raw");
        for x in 0..45 {
            let c = cell(&buffer, x, 3);
            let active = spans
                .iter()
                .position(|&(a, b)| (a..b).contains(&x))
                .map(|t| t == i);
            match active {
                Some(true) => {
                    assert_eq!(
                        (c.fg, c.bg),
                        (Some(Color::WHITE), Some(Color::RED)),
                        "{view:?} @{x}"
                    )
                }
                Some(false) => {
                    assert_eq!(
                        (c.fg, c.bg),
                        (Some(DISABLED_FG), Some(Color::BLUE)),
                        "{view:?} @{x}"
                    )
                }
                None => assert_eq!((c.symbol, c.bg), (' ', Some(Color::BLUE)), "{view:?} @{x}"),
            }
        }
    }
}

#[test]
fn test_http_client_body_fills_the_rows_under_the_tabs() {
    let body: Vec<String> = (0..20).map(|i| format!("line {i}")).collect();
    let client = with(
        response(200, "OK", &[], &body.join("\n"), 1),
        ResponseView::Raw,
    );
    let buffer = draw(&client, 40, 12);
    // Rows 4..12: eight lines, the ninth is cut by the area.
    for y in 4..12 {
        assert_eq!(row(&buffer, y), format!("line {}", y - 4));
        assert_eq!(cell(&buffer, 0, y).fg, Some(SECONDARY_TEXT));
    }
}

#[test]
fn test_http_client_body_line_is_clipped_to_the_width() {
    let line = "z".repeat(70);
    let client = with(response(200, "OK", &[], &line, 1), ResponseView::Raw);
    let buffer = draw(&client, 40, 10);
    assert_eq!(row(&buffer, 4), "z".repeat(40));
}

#[test]
fn test_http_client_empty_body_and_no_headers_draw_no_content() {
    for view in [ResponseView::Body, ResponseView::Headers, ResponseView::Raw] {
        let buffer = draw(&with(response(200, "OK", &[], "", 1), view), 40, 10);
        assert_eq!(row(&buffer, 3), "Body Headers Raw");
        assert!((4..10).all(|y| row(&buffer, y).is_empty()), "{view:?}");
    }
}

#[test]
fn test_http_client_header_row_layout_and_colors() {
    let colors = HttpColors {
        header_key: Color::GREEN,
        header_value: Color::YELLOW,
        ..HttpColors::default()
    };
    let client = with(
        response(200, "OK", &[("Key", "value")], "", 1),
        ResponseView::Headers,
    )
    .colors(colors);
    let buffer = draw(&client, 40, 10);
    assert_eq!(row(&buffer, 4), "Key: value");
    for x in 0..3 {
        assert_eq!(cell(&buffer, x, 4).fg, Some(Color::GREEN));
    }
    assert_eq!(cell(&buffer, 3, 4).fg, Some(DISABLED_FG));
    for x in 5..10 {
        assert_eq!(cell(&buffer, x, 4).fg, Some(Color::YELLOW));
    }
}

#[test]
fn test_http_client_long_header_key_is_cut_at_half_the_width() {
    let key = "K".repeat(30);
    let value = "v".repeat(40);
    let client = with(
        response(200, "OK", &[(&key, &value)], "", 1),
        ResponseView::Headers,
    );
    let buffer = draw(&client, 41, 10);
    // half = 20: 20 key cells, the colon at 20, the value from 22 to the edge.
    assert_eq!(
        row(&buffer, 4),
        format!("{}: {}", "K".repeat(20), "v".repeat(19))
    );
}

#[test]
fn test_http_client_wide_header_key_moves_the_colon() {
    let client = with(
        response(200, "OK", &[("키", "값")], "", 1),
        ResponseView::Headers,
    );
    let buffer = draw(&client, 40, 10);
    assert_eq!(cell(&buffer, 0, 4).symbol, '키');
    assert!(cell(&buffer, 1, 4).is_continuation());
    assert_eq!(cell(&buffer, 2, 4).symbol, ':');
    assert_eq!(cell(&buffer, 4, 4).symbol, '값');
}

// =========================================================================
// Models
// =========================================================================

fn method() -> impl Strategy<Value = HttpMethod> {
    prop::sample::select(METHODS.to_vec())
}

proptest! {
    /// Row 0 is the method name, a space and the URL, with "[Enter: Send]"
    /// written over the last 13 columns.
    #[test]
    fn prop_url_bar_matches_the_model(
        m in method(),
        path in "[a-z0-9/._-]{0,90}",
        width in 40u16..110,
    ) {
        let url = format!("https://h.io/{path}");
        let buffer = draw(&http_get(url.as_str()).method(m), width, 10);
        let w = width as usize;
        let mut model: Vec<char> = vec![' '; w];
        for (i, ch) in format!("{} {url}", m.name()).chars().take(w).enumerate() {
            model[i] = ch;
        }
        for (i, ch) in "[Enter: Send]".chars().enumerate() {
            model[w - 13 + i] = ch;
        }
        let got: String = (0..width).map(|x| cell(&buffer, x, 0).symbol).collect();
        prop_assert_eq!(got, model.into_iter().collect::<String>());
    }

    /// The content rows show the body's lines from the scroll offset on, as
    /// many as fit under the tabs, each cut at the width. The offset stops
    /// where the last line is on the bottom row (#973). Body and Raw draw
    /// the same for a body that is not JSON.
    #[test]
    fn prop_body_rows_match_the_model(
        lines in prop::collection::vec("[a-z ]{0,60}", 0..30),
        scroll_pick in 0usize..31,
        width in 40u16..80,
        height in 10u16..30,
        raw in any::<bool>(),
    ) {
        let mut client = with(
            response(200, "OK", &[("Content-Type", "text/plain")], &lines.join("\n"), 1),
            if raw { ResponseView::Raw } else { ResponseView::Body },
        );
        client.scroll_down(scroll_pick);
        let buffer = draw(&client, width, height);
        let joined = lines.join("\n");
        let body: Vec<&str> = joined.lines().collect();
        // Before a draw the offset can put the last line on top; the draw
        // then keeps the last page full under the four header rows
        let scroll = scroll_pick
            .min(body.len().saturating_sub(1))
            .min(body.len().saturating_sub((height - 4) as usize));
        for y in 4..height {
            let want = body
                .get(scroll + (y - 4) as usize)
                .map(|l| l.chars().take(width as usize).collect::<String>())
                .unwrap_or_default();
            prop_assert_eq!(row(&buffer, y), want.trim_end(), "row {}", y);
        }
    }

    /// One header row: the key cut at half the width, ": ", then the value
    /// cut at the right edge.
    #[test]
    fn prop_header_row_matches_the_model(
        key in "[A-Za-z-]{1,60}",
        value in "[a-z0-9 ]{0,80}",
        width in 40u16..100,
    ) {
        let client = with(response(200, "OK", &[(&key, &value)], "", 1), ResponseView::Headers);
        let buffer = draw(&client, width, 10);
        let half = (width / 2) as usize;
        let k: String = key.chars().take(half).collect();
        let v: String = value.chars().take(width as usize - k.len() - 2).collect();
        let want = format!("{k}: {v}");
        prop_assert_eq!(row(&buffer, 4), want.trim_end());
    }
}

/// `scroll_down` used to store any offset, so the content rows went blank
/// past the last line (#973).
#[test]
fn body_scroll_stops_at_the_last_line() {
    let body: Vec<String> = (0..10).map(|i| format!("l{i}")).collect();
    let mut c = with(
        response(200, "OK", &[], &body.join("\n"), 1),
        ResponseView::Body,
    );
    c.scroll_down(100);
    assert!(c.body_scroll_for_testing() <= 9);
    // 8 content rows under the tabs: the last page is full
    let buffer = draw(&c, 60, 12);
    let shown: Vec<String> = (4..12).map(|y| row(&buffer, y)).collect();
    assert_eq!(shown, body[2..]);
}

/// The Headers view used the body's offset, so after scrolling the body it
/// started past the first headers, or showed nothing (#973).
#[test]
fn headers_view_is_not_scrolled_by_the_body() {
    let headers: Vec<(String, String)> =
        (0..10).map(|i| (format!("X-{i:02}"), "v".into())).collect();
    let pairs: Vec<(&str, &str)> = headers
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();
    let body: Vec<String> = (0..20).map(|i| format!("l{i}")).collect();
    let mut c = with(
        response(200, "OK", &pairs, &body.join("\n"), 1),
        ResponseView::Body,
    );
    c.scroll_down(3);
    c.set_view(ResponseView::Headers);
    // The headers come in map order (#1083): check the offset and that the
    // eight content rows are all headers
    assert_eq!(c.body_scroll_for_testing(), 0);
    let buffer = draw(&c, 60, 12);
    assert!((4..12).all(|y| row(&buffer, y).starts_with("X-")));
}
