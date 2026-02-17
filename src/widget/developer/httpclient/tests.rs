//! HTTP client tests
//!
//! Public API tests extracted to tests/widget/developer/httpclient.rs
//! KEEP HERE - Private implementation tests (accesses private fields: state, request, etc.)

#[cfg(test)]
mod tests {
    use crate::layout::Rect;
    use crate::prelude::RenderContext;
    use crate::render::Buffer;
    use crate::widget::developer::httpclient::builder::base64_encode;
    use crate::widget::developer::httpclient::*;
    use crate::widget::traits::View;
    use std::collections::HashMap;
    use std::time::Duration;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        assert_eq!(client.state(), RequestState::Idle);
    }

    #[test]
    fn test_request_builder() {
        let client = HttpClient::new()
            .url("https://api.example.com")
            .method(HttpMethod::POST)
            .header("Content-Type", "application/json")
            .body(r#"{"key": "value"}"#);

        assert_eq!(client.request().method, HttpMethod::POST);
        assert!(!client.request().headers.is_empty());
    }

    #[test]
    fn test_http_request_url() {
        let req = HttpRequest::new("https://api.example.com")
            .unwrap()
            .param("page", "1")
            .param("limit", "10");

        let url = req.full_url();
        assert!(url.contains("page=1"));
        assert!(url.contains("limit=10"));
    }

    #[test]
    fn test_send_request() {
        let mut client = get("https://httpbin.org/get");
        client.send();

        assert_eq!(client.state(), RequestState::Success);
        assert!(client.response().is_some());
    }

    #[test]
    fn test_render() {
        let client = get("https://example.com");

        let mut buffer = Buffer::new(80, 20);
        let area = Rect::new(0, 0, 80, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        client.render(&mut ctx);
    }

    #[test]
    fn test_method_colors() {
        assert_ne!(HttpMethod::GET.color(), HttpMethod::POST.color());
        assert_ne!(HttpMethod::DELETE.color(), HttpMethod::PUT.color());
    }

    // ==========================================================================
    // ContentType tests
    // ==========================================================================

    #[test]
    fn test_content_type_from_header_json() {
        assert_eq!(
            ContentType::from_header(Some("application/json")),
            ContentType::Json
        );
        assert_eq!(
            ContentType::from_header(Some("application/json; charset=utf-8")),
            ContentType::Json
        );
        assert_eq!(
            ContentType::from_header(Some("text/json")),
            ContentType::Json
        );
    }

    #[test]
    fn test_content_type_from_header_xml() {
        assert_eq!(
            ContentType::from_header(Some("application/xml")),
            ContentType::Xml
        );
        assert_eq!(ContentType::from_header(Some("text/xml")), ContentType::Xml);
    }

    #[test]
    fn test_content_type_from_header_html() {
        assert_eq!(
            ContentType::from_header(Some("text/html")),
            ContentType::Html
        );
        assert_eq!(
            ContentType::from_header(Some("text/html; charset=utf-8")),
            ContentType::Html
        );
    }

    #[test]
    fn test_content_type_from_header_text() {
        assert_eq!(
            ContentType::from_header(Some("text/plain")),
            ContentType::Text
        );
        assert_eq!(ContentType::from_header(None), ContentType::Text);
    }

    #[test]
    fn test_content_type_from_header_binary() {
        assert_eq!(
            ContentType::from_header(Some("application/octet-stream")),
            ContentType::Binary
        );
    }

    // ==========================================================================
    // HttpResponse tests
    // ==========================================================================

    #[test]
    fn test_response_is_success() {
        let mut response = HttpResponse::default();
        response.status = 200;
        assert!(response.is_success());

        response.status = 201;
        assert!(response.is_success());

        response.status = 299;
        assert!(response.is_success());

        response.status = 404;
        assert!(!response.is_success());

        response.status = 500;
        assert!(!response.is_success());
    }

    #[test]
    fn test_response_status_color() {
        let mut response = HttpResponse::default();

        response.status = 200;
        let green = response.status_color();

        response.status = 404;
        let red = response.status_color();

        response.status = 301;
        let yellow = response.status_color();

        response.status = 500;
        let purple = response.status_color();

        assert_ne!(green, red);
        assert_ne!(yellow, purple);
    }

    #[test]
    fn test_response_content_type() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        assert_eq!(response.content_type(), ContentType::Json);
    }

    #[test]
    fn test_response_pretty_json() {
        let mut response = HttpResponse::default();
        response.body = r#"{"name":"test","value":123}"#.to_string();

        let pretty = response.pretty_json().unwrap();
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("name"));
        assert!(pretty.contains("test"));
    }

    #[test]
    fn test_response_formatted_body_json() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        response.body = r#"{"key":"value"}"#.to_string();

        let formatted = response.formatted_body();
        assert!(formatted.contains('\n'));
    }

    #[test]
    fn test_response_formatted_body_text() {
        let mut response = HttpResponse::default();
        response
            .headers
            .insert("Content-Type".to_string(), "text/plain".to_string());
        response.body = "Hello, World!".to_string();

        let formatted = response.formatted_body();
        assert_eq!(formatted, "Hello, World!");
    }

    // ==========================================================================
    // JSON formatting tests
    // ==========================================================================

    #[test]
    fn test_format_json_simple_object() {
        let response = HttpResponse::default();
        let json = r#"{"key":"value"}"#;
        let formatted = response.format_json(json).unwrap();

        assert!(formatted.contains("{\n"));
        assert!(formatted.contains("\"key\": \"value\""));
    }

    #[test]
    fn test_format_json_nested_object() {
        let response = HttpResponse::default();
        let json = r#"{"outer":{"inner":"value"}}"#;
        let formatted = response.format_json(json).unwrap();

        assert!(formatted.contains("\"outer\": {\n"));
        assert!(formatted.contains("\"inner\": \"value\""));
    }

    #[test]
    fn test_format_json_array() {
        let response = HttpResponse::default();
        let json = r#"[1,2,3]"#;
        let formatted = response.format_json(json).unwrap();

        assert!(formatted.contains("[\n"));
        assert!(formatted.contains("1,\n"));
    }

    #[test]
    fn test_format_json_with_escaped_quotes() {
        let response = HttpResponse::default();
        let json = r#"{"message":"Hello \"World\""}"#;
        let formatted = response.format_json(json).unwrap();

        assert!(formatted.contains(r#"\"World\""#));
    }

    #[test]
    fn test_format_json_empty_returns_none() {
        let response = HttpResponse::default();
        let result = response.format_json("");
        assert!(result.is_none());
    }

    // ==========================================================================
    // RequestBuilder tests
    // ==========================================================================

    #[test]
    fn test_request_builder_get() {
        let request = RequestBuilder::get("https://api.example.com")
            .unwrap()
            .build();
        assert_eq!(request.method, HttpMethod::GET);
        assert_eq!(request.url(), "https://api.example.com");
    }

    #[test]
    fn test_request_builder_post() {
        let request = RequestBuilder::post("https://api.example.com")
            .unwrap()
            .build();
        assert_eq!(request.method, HttpMethod::POST);
    }

    #[test]
    fn test_request_builder_put() {
        let request = RequestBuilder::put("https://api.example.com")
            .unwrap()
            .build();
        assert_eq!(request.method, HttpMethod::PUT);
    }

    #[test]
    fn test_request_builder_delete() {
        let request = RequestBuilder::delete("https://api.example.com")
            .unwrap()
            .build();
        assert_eq!(request.method, HttpMethod::DELETE);
    }

    #[test]
    fn test_request_builder_patch() {
        let request = RequestBuilder::patch("https://api.example.com")
            .unwrap()
            .build();
        assert_eq!(request.method, HttpMethod::PATCH);
    }

    #[test]
    fn test_request_builder_with_header() {
        let request = RequestBuilder::get("https://api.example.com")
            .unwrap()
            .header("X-Custom", "value")
            .build();

        assert_eq!(request.headers.get("X-Custom"), Some(&"value".to_string()));
    }

    #[test]
    fn test_request_builder_with_params() {
        let request = RequestBuilder::get("https://api.example.com")
            .unwrap()
            .param("page", "1")
            .param("limit", "10")
            .build();

        let url = request.full_url();
        assert!(url.contains("page=1"));
        assert!(url.contains("limit=10"));
    }

    #[test]
    fn test_request_builder_with_body() {
        let request = RequestBuilder::post("https://api.example.com")
            .unwrap()
            .body("test body")
            .build();

        assert_eq!(request.body, "test body");
    }

    #[test]
    fn test_request_builder_json() {
        let request = RequestBuilder::post("https://api.example.com")
            .unwrap()
            .json(r#"{"key": "value"}"#)
            .build();

        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
        assert!(request.body.contains("key"));
    }

    #[test]
    fn test_request_builder_form() {
        let request = RequestBuilder::post("https://api.example.com")
            .unwrap()
            .form("key=value&other=data")
            .build();

        assert_eq!(
            request.headers.get("Content-Type"),
            Some(&"application/x-www-form-urlencoded".to_string())
        );
    }

    #[test]
    fn test_request_builder_bearer_auth() {
        let request = RequestBuilder::get("https://api.example.com")
            .unwrap()
            .bearer_auth("my_token")
            .build();

        assert_eq!(
            request.headers.get("Authorization"),
            Some(&"Bearer my_token".to_string())
        );
    }

    #[test]
    fn test_request_builder_basic_auth() {
        let request = RequestBuilder::get("https://api.example.com")
            .unwrap()
            .basic_auth("user", "pass")
            .build();

        let auth = request.headers.get("Authorization").unwrap();
        assert!(auth.starts_with("Basic "));
    }

    // ==========================================================================
    // base64 encoding tests
    // ==========================================================================

    #[test]
    fn test_base64_encode_simple() {
        assert_eq!(base64_encode(b"Hello"), "SGVsbG8=");
        assert_eq!(base64_encode(b"Hi"), "SGk=");
        assert_eq!(base64_encode(b"A"), "QQ==");
    }

    #[test]
    fn test_base64_encode_credentials() {
        let credentials = "user:pass";
        let encoded = base64_encode(credentials.as_bytes());
        assert_eq!(encoded, "dXNlcjpwYXNz");
    }

    // ==========================================================================
    // MockHttpBackend tests
    // ==========================================================================

    #[test]
    fn test_mock_backend_default_response() {
        let backend = MockHttpBackend::new();
        let request = HttpRequest::new("https://api.example.com").unwrap();

        let response = backend.send(&request).unwrap();
        assert_eq!(response.status, 200);
        assert!(response.body.contains("mock"));
    }

    #[test]
    fn test_mock_backend_custom_response() {
        let backend = MockHttpBackend::new();

        let custom_response = HttpResponse {
            status: 201,
            status_text: "Created".to_string(),
            headers: HashMap::new(),
            body: "custom body".to_string(),
            time: Duration::from_millis(10),
            size: 11,
        };

        backend.mock_response("api.example.com", custom_response);

        let request = HttpRequest::new("https://api.example.com/users").unwrap();
        let response = backend.send(&request).unwrap();

        assert_eq!(response.status, 201);
        assert_eq!(response.body, "custom body");
    }

    #[test]
    fn test_mock_backend_json_response() {
        let backend = MockHttpBackend::new();
        backend.mock_json("users", 200, r#"{"id": 1, "name": "Test"}"#);

        let request = HttpRequest::new("https://api.example.com/users").unwrap();
        let response = backend.send(&request).unwrap();

        assert_eq!(response.status, 200);
        assert_eq!(response.content_type(), ContentType::Json);
        assert!(response.body.contains("Test"));
    }

    #[test]
    fn test_mock_backend_error_response() {
        let backend = MockHttpBackend::new();
        backend.mock_error("users", 404, "User not found");

        let request = HttpRequest::new("https://api.example.com/users/999").unwrap();
        let response = backend.send(&request).unwrap();

        assert_eq!(response.status, 404);
        assert!(response.body.contains("User not found"));
    }

    #[test]
    fn test_mock_backend_wildcard_pattern() {
        let backend = MockHttpBackend::new();

        let wildcard_response = HttpResponse {
            status: 503,
            status_text: "Service Unavailable".to_string(),
            headers: HashMap::new(),
            body: "maintenance".to_string(),
            time: Duration::from_millis(1),
            size: 11,
        };

        backend.mock_response("*", wildcard_response);

        let request = HttpRequest::new("https://any.url.com/anything").unwrap();
        let response = backend.send(&request).unwrap();

        assert_eq!(response.status, 503);
    }

    #[test]
    fn test_mock_backend_most_recent_match() {
        let backend = MockHttpBackend::new();

        backend.mock_json("api", 200, r#"{"first": true}"#);
        backend.mock_json("api", 201, r#"{"second": true}"#);

        let request = HttpRequest::new("https://api.example.com").unwrap();
        let response = backend.send(&request).unwrap();

        // Most recent match should win
        assert_eq!(response.status, 201);
        assert!(response.body.contains("second"));
    }

    // ==========================================================================
    // HttpMethod tests
    // ==========================================================================

    #[test]
    fn test_http_method_names() {
        assert_eq!(HttpMethod::GET.name(), "GET");
        assert_eq!(HttpMethod::POST.name(), "POST");
        assert_eq!(HttpMethod::PUT.name(), "PUT");
        assert_eq!(HttpMethod::DELETE.name(), "DELETE");
        assert_eq!(HttpMethod::PATCH.name(), "PATCH");
        assert_eq!(HttpMethod::HEAD.name(), "HEAD");
        assert_eq!(HttpMethod::OPTIONS.name(), "OPTIONS");
    }

    #[test]
    fn test_http_method_default() {
        assert_eq!(HttpMethod::default(), HttpMethod::GET);
    }
}
