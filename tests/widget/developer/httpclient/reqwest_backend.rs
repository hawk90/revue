//! ReqwestBackend against a one-shot HTTP server on 127.0.0.1 (#1487)

use revue::widget::{ContentType, HttpBackend, HttpMethod, HttpRequest, ReqwestBackend};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

/// Serve one connection with `reply` and return the raw request it read
fn serve_once(reply: Vec<u8>) -> (String, thread::JoinHandle<Vec<u8>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut seen = Vec::new();
        let mut buf = [0u8; 4096];
        // Read the head, then as much body as Content-Length says
        loop {
            let n = stream.read(&mut buf).unwrap();
            seen.extend_from_slice(&buf[..n]);
            if let Some(end) = seen.windows(4).position(|w| w == b"\r\n\r\n") {
                let head = String::from_utf8_lossy(&seen[..end]).to_lowercase();
                let len = head
                    .lines()
                    .find_map(|l| l.strip_prefix("content-length: "))
                    .map_or(0, |v| v.trim().parse::<usize>().unwrap());
                while seen.len() < end + 4 + len {
                    let n = stream.read(&mut buf).unwrap();
                    seen.extend_from_slice(&buf[..n]);
                }
                break;
            }
            if n == 0 {
                break;
            }
        }
        stream.write_all(&reply).unwrap();
        seen
    });
    (addr, server)
}

fn reply(head: &str, body: &[u8]) -> Vec<u8> {
    let mut out = format!(
        "HTTP/1.1 {head}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .into_bytes();
    out.extend_from_slice(body);
    out
}

#[test]
fn test_sends_method_url_params_headers_and_body() {
    let (addr, server) = serve_once(reply("201 Created", b"ok"));
    let request = HttpRequest::new(format!("{addr}/items"))
        .unwrap()
        .method(HttpMethod::POST)
        .header("X-Token", "t1")
        .body("payload")
        .param("q", "a b");

    let response = ReqwestBackend::new().send(&request).unwrap();
    let seen = String::from_utf8(server.join().unwrap()).unwrap();

    assert!(seen.starts_with("POST /items?q=a+b HTTP/1.1\r\n"), "{seen}");
    assert!(
        seen.to_lowercase().contains("\r\nx-token: t1\r\n"),
        "{seen}"
    );
    assert!(seen.ends_with("\r\n\r\npayload"), "{seen}");
    assert_eq!(response.status, 201);
    assert_eq!(response.status_text, "Created");
    assert_eq!(response.body, "ok");
    assert_eq!(response.size, 2);
}

#[test]
fn test_reads_status_headers_and_body() {
    let body = br#"{"a":1}"#;
    let mut raw =
        b"HTTP/1.1 404 Not Found\r\nContent-Type: application/json\r\nX-Many: 1\r\nX-Many: 2\r\n"
            .to_vec();
    raw.extend_from_slice(
        format!(
            "Content-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        )
        .as_bytes(),
    );
    raw.extend_from_slice(body);
    let (addr, server) = serve_once(raw);

    let response = ReqwestBackend::new()
        .send(&HttpRequest::new(format!("{addr}/x")).unwrap())
        .unwrap();
    let seen = String::from_utf8(server.join().unwrap()).unwrap();

    assert!(seen.starts_with("GET /x HTTP/1.1\r\n"), "{seen}");
    assert_eq!(response.status, 404);
    assert_eq!(response.status_text, "Not Found");
    assert!(!response.is_success());
    // A repeated header is one comma-separated value
    assert_eq!(
        response.headers.get("x-many").map(String::as_str),
        Some("1, 2")
    );
    // reqwest reports header names lowercase; the content type is still found
    assert_eq!(response.content_type(), ContentType::Json);
    assert_eq!(response.body, r#"{"a":1}"#);
    assert_eq!(response.size, body.len());
}

#[test]
fn test_body_that_is_not_utf8_keeps_its_byte_size() {
    let (addr, server) = serve_once(reply("200 OK", &[0xff, b'a']));
    let response = ReqwestBackend::new()
        .send(&HttpRequest::new(addr).unwrap())
        .unwrap();
    server.join().unwrap();
    assert_eq!(response.body, "\u{FFFD}a");
    assert_eq!(response.size, 2);
}

#[test]
fn test_unreachable_host_is_an_error() {
    // Take a free port, then close it: nothing listens there
    let port = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port();
    let result =
        ReqwestBackend::new().send(&HttpRequest::new(format!("http://127.0.0.1:{port}/")).unwrap());
    assert!(result.is_err());
}

#[test]
fn test_invalid_header_name_is_an_error_not_a_panic() {
    let request = HttpRequest::new("http://127.0.0.1:9/")
        .unwrap()
        .header("bad header", "v");
    let error = ReqwestBackend::new().send(&request).unwrap_err();
    assert!(!error.is_empty());
}
