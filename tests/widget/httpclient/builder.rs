//! RequestBuilder tests
//!
//! Each constructor's method, a single header, params in full_url(),
//! body, json, form, bearer_auth and the "Basic " prefix of basic_auth
//! are covered by the in-source tests in src/widget/developer/httpclient/
//! tests.rs, as are the "Hello"/"user:pass" base64 vectors.

use revue::widget::{HttpMethod, RequestBuilder};

fn authorization(builder: RequestBuilder) -> String {
    builder
        .build()
        .headers
        .get("Authorization")
        .cloned()
        .expect("Authorization header")
}

#[test]
fn test_request_builder_keeps_url() {
    let request = RequestBuilder::get(String::from("https://api.test.com"))
        .unwrap()
        .build();
    assert_eq!(request.url(), "https://api.test.com");
    assert!(request.headers.is_empty());
    assert!(request.params.is_empty());
    assert_eq!(request.body, "");
}

#[test]
fn test_request_builder_rejects_invalid_urls() {
    for url in ["", "ftp://example.com", "file:///etc/passwd"] {
        assert!(RequestBuilder::get(url).is_none(), "get {url}");
        assert!(RequestBuilder::post(url).is_none(), "post {url}");
        assert!(RequestBuilder::put(url).is_none(), "put {url}");
        assert!(RequestBuilder::delete(url).is_none(), "delete {url}");
        assert!(RequestBuilder::patch(url).is_none(), "patch {url}");
    }
}

#[test]
fn test_request_builder_multiple_headers() {
    let request = RequestBuilder::get("https://example.com")
        .unwrap()
        .header("Accept", "application/json")
        .header("User-Agent", "TestClient")
        .build();
    assert_eq!(request.headers.len(), 2);
    assert_eq!(
        request.headers.get("Accept").map(String::as_str),
        Some("application/json")
    );
    assert_eq!(
        request.headers.get("User-Agent").map(String::as_str),
        Some("TestClient")
    );
}

#[test]
fn test_request_builder_params() {
    let request = RequestBuilder::get("https://example.com")
        .unwrap()
        .param("page", "1")
        .param("limit", "10")
        .build();
    assert_eq!(request.params.len(), 2);
    assert_eq!(request.params.get("page").map(String::as_str), Some("1"));
    assert_eq!(request.params.get("limit").map(String::as_str), Some("10"));
}

#[test]
fn test_request_builder_json_and_form_bodies() {
    let json = RequestBuilder::post("https://example.com")
        .unwrap()
        .json("{\"test\":true}")
        .build();
    assert_eq!(json.body, "{\"test\":true}");

    let form = RequestBuilder::post("https://example.com")
        .unwrap()
        .form("key=value&foo=bar")
        .build();
    assert_eq!(form.body, "key=value&foo=bar");
}

#[test]
fn test_request_builder_basic_auth_encodes_credentials() {
    let get = || RequestBuilder::get("https://example.com").unwrap();
    assert_eq!(
        authorization(get().basic_auth("user", "pass")),
        "Basic dXNlcjpwYXNz"
    );
    assert_eq!(
        authorization(get().basic_auth("user", "")),
        "Basic dXNlcjo="
    );
    assert_eq!(
        authorization(get().basic_auth("Aladdin", "open sesame")),
        "Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=="
    );
    assert_eq!(
        authorization(get().basic_auth("사용자", "비밀")),
        "Basic 7IKs7Jqp7J6QOuu5hOuwgA=="
    );
}

#[test]
fn test_request_builder_basic_auth_padding() {
    // "u:" is 2 bytes, "ab:" 3 and "abc:" 4: one, no and two padding chars.
    let get = || RequestBuilder::get("https://example.com").unwrap();
    assert_eq!(authorization(get().basic_auth("u", "")), "Basic dTo=");
    assert_eq!(authorization(get().basic_auth("ab", "")), "Basic YWI6");
    assert_eq!(authorization(get().basic_auth("abc", "")), "Basic YWJjOg==");
}

#[test]
fn test_request_builder_later_auth_replaces_earlier() {
    let builder = RequestBuilder::get("https://example.com")
        .unwrap()
        .basic_auth("user", "pass")
        .bearer_auth("token");
    assert_eq!(authorization(builder), "Bearer token");
}

#[test]
fn test_request_builder_build_complex() {
    let request = RequestBuilder::post("https://api.example.com/data")
        .unwrap()
        .header("X-Custom", "value")
        .param("version", "v1")
        .bearer_auth("token")
        .json("{\"key\":\"value\"}")
        .build();

    assert_eq!(request.method, HttpMethod::POST);
    assert_eq!(request.url(), "https://api.example.com/data");
    assert_eq!(
        request.headers.get("X-Custom").map(String::as_str),
        Some("value")
    );
    assert_eq!(
        request.headers.get("Authorization").map(String::as_str),
        Some("Bearer token")
    );
    assert_eq!(
        request.headers.get("Content-Type").map(String::as_str),
        Some("application/json")
    );
    assert_eq!(request.body, "{\"key\":\"value\"}");
    assert_eq!(
        request.full_url(),
        "https://api.example.com/data?version=v1"
    );
}
