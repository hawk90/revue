//! HTTP client type tests

use revue::widget::developer::httpclient::types::*;
use revue::style::Color;

// =========================================================================
// HttpMethod enum tests
// =========================================================================

#[test]
fn test_http_method_default() {
    assert_eq!(HttpMethod::default(), HttpMethod::GET);
}

#[test]
fn test_http_method_clone() {
    let method = HttpMethod::POST;
    assert_eq!(method, method.clone());
}

#[test]
fn test_http_method_copy() {
    let method1 = HttpMethod::GET;
    let method2 = method1;
    assert_eq!(method1, HttpMethod::GET);
    assert_eq!(method2, HttpMethod::GET);
}

#[test]
fn test_http_method_equality() {
    assert_eq!(HttpMethod::GET, HttpMethod::GET);
    assert_eq!(HttpMethod::POST, HttpMethod::POST);
    assert_ne!(HttpMethod::GET, HttpMethod::POST);
}

#[test]
fn test_http_method_debug() {
    let debug_str = format!("{:?}", HttpMethod::GET);
    assert!(debug_str.contains("GET"));
}

#[test]
fn test_http_method_name_get() {
    assert_eq!(HttpMethod::GET.name(), "GET");
}

#[test]
fn test_http_method_name_post() {
    assert_eq!(HttpMethod::POST.name(), "POST");
}

#[test]
fn test_http_method_name_put() {
    assert_eq!(HttpMethod::PUT.name(), "PUT");
}

#[test]
fn test_http_method_name_delete() {
    assert_eq!(HttpMethod::DELETE.name(), "DELETE");
}

#[test]
fn test_http_method_name_patch() {
    assert_eq!(HttpMethod::PATCH.name(), "PATCH");
}

#[test]
fn test_http_method_name_head() {
    assert_eq!(HttpMethod::HEAD.name(), "HEAD");
}

#[test]
fn test_http_method_name_options() {
    assert_eq!(HttpMethod::OPTIONS.name(), "OPTIONS");
}

#[test]
fn test_http_method_color_get() {
    let color = HttpMethod::GET.color();
    assert_eq!(color, Color::rgb(97, 175, 239));
}

#[test]
fn test_http_method_color_post() {
    let color = HttpMethod::POST.color();
    assert_eq!(color, Color::rgb(152, 195, 121));
}

#[test]
fn test_http_method_color_put() {
    let color = HttpMethod::PUT.color();
    assert_eq!(color, Color::rgb(229, 192, 123));
}

#[test]
fn test_http_method_color_delete() {
    let color = HttpMethod::DELETE.color();
    assert_eq!(color, Color::rgb(224, 108, 117));
}

#[test]
fn test_http_method_color_patch() {
    let color = HttpMethod::PATCH.color();
    assert_eq!(color, Color::rgb(198, 120, 221));
}

#[test]
fn test_http_method_color_head() {
    let color = HttpMethod::HEAD.color();
    assert_eq!(color, Color::rgb(86, 182, 194));
}

#[test]
fn test_http_method_color_options() {
    let color = HttpMethod::OPTIONS.color();
    assert_eq!(color, Color::rgb(171, 178, 191));
}

// =========================================================================
// RequestState enum tests
// =========================================================================

#[test]
fn test_request_state_default() {
    assert_eq!(RequestState::default(), RequestState::Idle);
}

#[test]
fn test_request_state_clone() {
    let state = RequestState::Sending;
    assert_eq!(state, state.clone());
}

#[test]
fn test_request_state_copy() {
    let state1 = RequestState::Success;
    let state2 = state1;
    assert_eq!(state1, RequestState::Success);
    assert_eq!(state2, RequestState::Success);
}

#[test]
fn test_request_state_equality() {
    assert_eq!(RequestState::Idle, RequestState::Idle);
    assert_eq!(RequestState::Success, RequestState::Success);
    assert_ne!(RequestState::Idle, RequestState::Error);
}

#[test]
fn test_request_state_debug() {
    let debug_str = format!("{:?}", RequestState::Error);
    assert!(debug_str.contains("Error"));
}

// =========================================================================
// ContentType enum tests
// =========================================================================

#[test]
fn test_content_type_default() {
    assert_eq!(ContentType::default(), ContentType::Text);
}

#[test]
fn test_content_type_clone() {
    let ct = ContentType::Json;
    assert_eq!(ct, ct.clone());
}

#[test]
fn test_content_type_copy() {
    let ct1 = ContentType::Xml;
    let ct2 = ct1;
    assert_eq!(ct1, ContentType::Xml);
    assert_eq!(ct2, ContentType::Xml);
}

#[test]
fn test_content_type_equality() {
    assert_eq!(ContentType::Json, ContentType::Json);
    assert_eq!(ContentType::Html, ContentType::Html);
    assert_ne!(ContentType::Json, ContentType::Xml);
}

#[test]
fn test_content_type_debug() {
    let debug_str = format!("{:?}", ContentType::Binary);
    assert!(debug_str.contains("Binary"));
}

#[test]
fn test_content_type_from_header_json() {
    let ct = ContentType::from_header(Some("application/json"));
    assert_eq!(ct, ContentType::Json);
}

#[test]
fn test_content_type_from_header_json_text() {
    let ct = ContentType::from_header(Some("text/json"));
    assert_eq!(ct, ContentType::Json);
}

#[test]
fn test_content_type_from_header_xml() {
    let ct = ContentType::from_header(Some("application/xml"));
    assert_eq!(ct, ContentType::Xml);
}

#[test]
fn test_content_type_from_header_xml_text() {
    let ct = ContentType::from_header(Some("text/xml"));
    assert_eq!(ct, ContentType::Xml);
}

#[test]
fn test_content_type_from_header_html() {
    let ct = ContentType::from_header(Some("text/html"));
    assert_eq!(ct, ContentType::Html);
}

#[test]
fn test_content_type_from_header_text() {
    let ct = ContentType::from_header(Some("text/plain"));
    assert_eq!(ct, ContentType::Text);
}

#[test]
fn test_content_type_from_header_binary() {
    let ct = ContentType::from_header(Some("application/octet-stream"));
    assert_eq!(ct, ContentType::Binary);
}

#[test]
fn test_content_type_from_header_none() {
    let ct = ContentType::from_header(None);
    assert_eq!(ct, ContentType::Text);
}

#[test]
fn test_content_type_from_header_unknown() {
    let ct = ContentType::from_header(Some("application/unknown"));
    assert_eq!(ct, ContentType::Text);
}

// =========================================================================
// ResponseView enum tests
// =========================================================================

#[test]
fn test_response_view_default() {
    assert_eq!(ResponseView::default(), ResponseView::Body);
}

#[test]
fn test_response_view_clone() {
    let view = ResponseView::Headers;
    assert_eq!(view, view.clone());
}

#[test]
fn test_response_view_copy() {
    let view1 = ResponseView::Raw;
    let view2 = view1;
    assert_eq!(view1, ResponseView::Raw);
    assert_eq!(view2, ResponseView::Raw);
}

#[test]
fn test_response_view_equality() {
    assert_eq!(ResponseView::Body, ResponseView::Body);
    assert_eq!(ResponseView::Headers, ResponseView::Headers);
    assert_ne!(ResponseView::Body, ResponseView::Raw);
}

#[test]
fn test_response_view_debug() {
    let debug_str = format!("{:?}", ResponseView::Headers);
    assert!(debug_str.contains("Headers"));
}

// =========================================================================
// HttpColors tests
// =========================================================================

#[test]
fn test_http_colors_default() {
    let colors = HttpColors::default();
    assert_eq!(colors.url_bg, Color::rgb(30, 30, 40));
    assert_eq!(colors.method_bg, Color::rgb(40, 40, 60));
    assert_eq!(colors.header_key, Color::rgb(97, 175, 239));
    assert_eq!(colors.header_value, Color::rgb(171, 178, 191));
    assert_eq!(colors.tab_bg, Color::rgb(40, 40, 50));
    assert_eq!(colors.tab_active, Color::rgb(60, 60, 80));
}

#[test]
fn test_http_colors_clone() {
    let colors1 = HttpColors::default();
    let colors2 = colors1.clone();
    assert_eq!(colors1.url_bg, colors2.url_bg);
    assert_eq!(colors1.tab_active, colors2.tab_active);
}

#[test]
fn test_http_colors_debug() {
    let colors = HttpColors::default();
    let debug_str = format!("{:?}", colors);
    assert!(debug_str.contains("HttpColors"));
}
