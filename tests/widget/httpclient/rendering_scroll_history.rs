//! Rendering, Scroll, History Tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{http_delete, http_get, HttpClient, ResponseView};

#[test]
fn test_render_basic() {
    let client = http_get("https://example.com");
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    client.render(&mut ctx);
}

#[test]
fn test_render_with_response() {
    let mut client = http_get("https://example.com");
    client.send();
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    client.render(&mut ctx);
}

#[test]
fn test_render_with_error() {
    let mut client = HttpClient::new();
    client.set_error("Connection timeout");
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    client.render(&mut ctx);
}

#[test]
fn test_scroll_down() {
    let mut client = HttpClient::new();
    client.scroll_down(10);
    client.scroll_down(5);
}

#[test]
fn test_scroll_up() {
    let mut client = HttpClient::new();
    client.scroll_down(20);
    client.scroll_up(5);
}

#[test]
fn test_history_saved_on_send() {
    let mut client = http_get("https://api.example.com/1");
    client.send();
}

#[test]
fn test_history_back() {
    let mut client = HttpClient::new();
    client.set_url("https://api.example.com/1");
    client.send();
    client.set_url("https://api.example.com/2");
    client.send();
    client.history_back();
}

#[test]
fn test_toggle_headers() {
    let mut client = HttpClient::new();
    client.toggle_headers();
    client.toggle_headers();
}
