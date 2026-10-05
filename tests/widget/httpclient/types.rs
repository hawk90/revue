//! HTTP client value types
//!
//! HttpMethod names and default, two distinct-color checks, and
//! ContentType::from_header for the JSON/XML/HTML/text/binary/None cases
//! are covered by the in-source tests in
//! src/widget/developer/httpclient/tests.rs.

use revue::style::Color;
use revue::widget::{ContentType, HttpClient, HttpMethod, RequestState, ResponseView};

#[test]
fn test_http_method_colors() {
    let expected = [
        (HttpMethod::GET, Color::rgb(97, 175, 239)),
        (HttpMethod::POST, Color::rgb(152, 195, 121)),
        (HttpMethod::PUT, Color::rgb(229, 192, 123)),
        (HttpMethod::DELETE, Color::rgb(224, 108, 117)),
        (HttpMethod::PATCH, Color::rgb(198, 120, 221)),
        (HttpMethod::HEAD, Color::rgb(86, 182, 194)),
        (HttpMethod::OPTIONS, Color::rgb(171, 178, 191)),
    ];
    for (method, color) in expected {
        assert_eq!(method.color(), color, "{method:?}");
    }
}

#[test]
fn test_request_state_default_and_variants() {
    assert_eq!(RequestState::default(), RequestState::Idle);
    let variants = [
        RequestState::Idle,
        RequestState::Sending,
        RequestState::Success,
        RequestState::Error,
    ];
    for (i, a) in variants.iter().enumerate() {
        for (j, b) in variants.iter().enumerate() {
            assert_eq!(i == j, a == b, "{a:?} vs {b:?}");
        }
    }
}

#[test]
fn test_content_type_default() {
    assert_eq!(ContentType::default(), ContentType::Text);
}

#[test]
fn test_content_type_from_unknown_header() {
    assert_eq!(
        ContentType::from_header(Some("application/unknown")),
        ContentType::Text
    );
    assert_eq!(ContentType::from_header(Some("")), ContentType::Text);
}

#[test]
fn test_response_view_default_and_variants() {
    assert_eq!(ResponseView::default(), ResponseView::Body);
    assert_ne!(ResponseView::Body, ResponseView::Headers);
    assert_ne!(ResponseView::Body, ResponseView::Raw);
    assert_ne!(ResponseView::Headers, ResponseView::Raw);
}

#[test]
fn test_http_colors_default() {
    // HttpColors is not exported; read the defaults off a new client.
    let client = HttpClient::new();
    let colors = client.colors_for_testing();
    assert_eq!(colors.url_bg, Color::rgb(30, 30, 40));
    assert_eq!(colors.method_bg, Color::rgb(40, 40, 60));
    assert_eq!(colors.header_key, Color::rgb(97, 175, 239));
    assert_eq!(colors.header_value, Color::rgb(171, 178, 191));
    assert_eq!(colors.tab_bg, Color::rgb(40, 40, 50));
    assert_eq!(colors.tab_active, Color::rgb(60, 60, 80));
}
