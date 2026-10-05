//! HttpClient rendering

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{
    http_delete, http_get, HttpClient, HttpColors, HttpResponse, RenderContext, ResponseView, View,
};
use std::collections::HashMap;
use std::time::Duration;

fn rows(client: &HttpClient, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    client.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                // Skip the continuation cell after a wide character.
                .filter(|&c| c != '\0')
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn with_body(lines: usize) -> HttpClient {
    let body: Vec<String> = (0..lines).map(|i| format!("line {i}")).collect();
    let body = body.join("\n");
    let mut client = http_get("https://example.com");
    client.set_response(HttpResponse {
        status: 200,
        status_text: "OK".to_string(),
        headers: [("X-Key".to_string(), "x-value".to_string())]
            .into_iter()
            .collect::<HashMap<_, _>>(),
        size: body.len(),
        body,
        time: Duration::from_millis(12),
    });
    client
}

#[test]
fn test_render_url_bar_and_placeholder() {
    let rows = rows(&http_delete("https://example.com/item"), 80, 20);
    assert!(
        rows[0].starts_with("DELETE https://example.com/item"),
        "{rows:?}"
    );
    assert!(rows[0].ends_with("[Enter: Send]"), "{rows:?}");
    assert!(rows[1].chars().all(|c| c == '─'), "{rows:?}");
    assert_eq!(rows[2], "Enter a URL and press Enter to send request");
}

#[test]
fn test_render_too_small_draws_nothing() {
    let rows = rows(&http_get("https://example.com"), 39, 20);
    assert!(rows.iter().all(String::is_empty), "{rows:?}");
}

#[test]
fn test_render_with_response() {
    let mut client = http_get("https://example.com");
    client.send();
    let rows = rows(&client, 80, 20);
    assert!(rows[2].starts_with("200 OK • "), "{rows:?}");
    assert!(rows[3].starts_with("Body Headers Raw"), "{rows:?}");
    assert_eq!(rows[4], "{");
    assert_eq!(rows[5], "  \"status\": \"success\",");
}

#[test]
fn test_render_with_error() {
    let mut client = HttpClient::new();
    client.set_error("Connection timeout");
    let rows = rows(&client, 80, 20);
    assert_eq!(rows[2], "✗ Error: Connection timeout");
}

#[test]
fn test_render_headers_view() {
    let mut client = with_body(3);
    client.set_view(ResponseView::Headers);
    let rows = rows(&client, 80, 20);
    assert_eq!(rows[4], "X-Key: x-value");
}

#[test]
fn test_render_scrolled_body() {
    let mut client = with_body(30);
    let first = rows(&client, 80, 10);
    assert_eq!(first[4], "line 0");
    assert_eq!(first[9], "line 5");

    client.scroll_down(10);
    let scrolled = rows(&client, 80, 10);
    assert_eq!(scrolled[4], "line 10");

    client.scroll_up(3);
    assert_eq!(rows(&client, 80, 10)[4], "line 7");
}

#[test]
fn test_render_after_history_back_shows_recalled_url() {
    let mut client = HttpClient::new();
    client.set_url("https://api.example.com/1");
    client.send();
    client.set_url("https://api.example.com/2");
    client.send();
    client.history_back();
    client.history_back();
    let rows = rows(&client, 80, 20);
    assert!(
        rows[0].starts_with("GET https://api.example.com/1"),
        "{rows:?}"
    );
}

#[test]
fn test_render_uses_colors_builder() {
    let colors = HttpColors {
        tab_active: Color::RED,
        tab_bg: Color::BLUE,
        header_key: Color::GREEN,
        header_value: Color::YELLOW,
        ..HttpColors::default()
    };
    let mut client = with_body(1).colors(colors);
    client.set_view(ResponseView::Headers);

    let mut buffer = Buffer::new(80, 20);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 80, 20));
    client.render(&mut ctx);

    // Tab bar on row 3: "Body Headers Raw", Headers active.
    assert_eq!(buffer.get(0, 3).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(5, 3).unwrap().symbol, 'H');
    assert_eq!(buffer.get(5, 3).unwrap().bg, Some(Color::RED));
    // Header row 4: "X-Key: x-value".
    assert_eq!(buffer.get(0, 4).unwrap().fg, Some(Color::GREEN));
    assert_eq!(buffer.get(7, 4).unwrap().symbol, 'x');
    assert_eq!(buffer.get(7, 4).unwrap().fg, Some(Color::YELLOW));
}

#[test]
#[ignore = "BUG: HttpColors::url_bg and method_bg are never read by HttpClient::render"]
fn test_render_uses_url_and_method_backgrounds() {
    let colors = HttpColors {
        url_bg: Color::MAGENTA,
        method_bg: Color::CYAN,
        ..HttpColors::default()
    };
    let client = with_body(1).colors(colors);

    let mut buffer = Buffer::new(80, 20);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 80, 20));
    client.render(&mut ctx);

    // Row 0 is "GET https://example.com ...": the badge takes method_bg,
    // the URL takes url_bg.
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'G');
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::CYAN));
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'h');
    assert_eq!(buffer.get(4, 0).unwrap().bg, Some(Color::MAGENTA));
}
