//! HttpClient exchange: send queues the request, the app takes it, runs it
//! and completes it; only the latest send's result applies (#1487)

use revue::widget::{
    HttpBackend, HttpClient, HttpResponse, MockHttpBackend, PendingRequest, RequestState,
};
use std::time::Duration;

fn response(status: u16, body: &str) -> HttpResponse {
    HttpResponse {
        status,
        status_text: "T".to_string(),
        headers: Default::default(),
        body: body.to_string(),
        time: Duration::from_millis(3),
        size: body.len(),
    }
}

fn sent(url: &str) -> (HttpClient, PendingRequest) {
    let mut client = HttpClient::new().url(url);
    client.send();
    let pending = client.take_request().unwrap();
    (client, pending)
}

#[test]
fn test_take_request_hands_out_each_send_once() {
    let mut client = HttpClient::new().url("https://a.io/x").header("K", "v");
    assert!(client.take_request().is_none());
    client.send();
    let pending = client.take_request().unwrap();
    assert_eq!(pending.request.url(), "https://a.io/x");
    assert_eq!(
        pending.request.headers.get("K").map(String::as_str),
        Some("v")
    );
    assert!(client.take_request().is_none());
    // Still waiting for the result
    assert_eq!(client.state(), RequestState::Sending);
}

#[test]
fn test_taken_request_is_the_request_as_sent() {
    let mut client = HttpClient::new().url("https://a.io/1");
    client.send();
    client.set_url("https://a.io/2");
    assert_eq!(
        client.take_request().unwrap().request.url(),
        "https://a.io/1"
    );
}

#[test]
fn test_send_before_take_replaces_the_queued_request() {
    let mut client = HttpClient::new().url("https://a.io/1");
    client.send();
    client.set_url("https://a.io/2");
    client.send();
    let pending = client.take_request().unwrap();
    assert_eq!(pending.request.url(), "https://a.io/2");
    assert!(client.take_request().is_none());
    // Both sends are in the history
    assert_eq!(client.history_for_testing().len(), 2);
    assert!(client.complete(pending.id, Ok(response(200, "two"))));
    assert_eq!(client.response().unwrap().body, "two");
}

#[test]
fn test_complete_with_a_2xx_response_succeeds() {
    let (mut client, pending) = sent("https://a.io");
    assert!(client.complete(pending.id, Ok(response(201, "made"))));
    assert_eq!(client.state(), RequestState::Success);
    assert_eq!(client.response().unwrap().body, "made");
    assert!(client.error().is_none());
}

#[test]
fn test_complete_with_a_non_2xx_response_is_an_error_that_keeps_it() {
    let (mut client, pending) = sent("https://a.io");
    assert!(client.complete(pending.id, Ok(response(404, "missing"))));
    assert_eq!(client.state(), RequestState::Error);
    assert_eq!(client.response().unwrap().status, 404);
}

#[test]
fn test_complete_with_an_error_shows_it() {
    let (mut client, pending) = sent("https://a.io");
    assert!(client.complete(pending.id, Err("connection refused".to_string())));
    assert_eq!(client.state(), RequestState::Error);
    assert_eq!(client.error(), Some("connection refused"));
    assert!(client.response().is_none());
}

#[test]
fn test_result_of_an_earlier_send_is_dropped() {
    let (mut client, first) = sent("https://a.io/1");
    client.set_url("https://a.io/2");
    client.send();
    let second = client.take_request().unwrap();

    assert!(!client.complete(first.id, Ok(response(200, "one"))));
    assert_eq!(client.state(), RequestState::Sending);
    assert!(client.response().is_none());

    assert!(client.complete(second.id, Ok(response(200, "two"))));
    assert_eq!(client.response().unwrap().body, "two");
}

#[test]
fn test_earlier_result_after_the_later_one_changes_nothing() {
    let (mut client, first) = sent("https://a.io/1");
    client.send();
    let second = client.take_request().unwrap();
    assert!(client.complete(second.id, Ok(response(200, "two"))));
    assert!(!client.complete(first.id, Err("late".to_string())));
    assert_eq!(client.state(), RequestState::Success);
    assert_eq!(client.response().unwrap().body, "two");
}

#[test]
fn test_a_request_completes_once() {
    let (mut client, pending) = sent("https://a.io");
    assert!(client.complete(pending.id, Ok(response(200, "first"))));
    assert!(!client.complete(pending.id, Ok(response(500, "again"))));
    assert_eq!(client.response().unwrap().body, "first");
}

#[test]
fn test_complete_with_nothing_sent_changes_nothing() {
    // An id from another client: this one has nothing in flight
    let (_, other) = sent("https://a.io");
    let mut client = HttpClient::new().url("https://b.io");
    assert!(!client.complete(other.id, Ok(response(200, "x"))));
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
}

#[test]
fn test_clear_drops_the_request_in_flight() {
    let (mut client, pending) = sent("https://a.io");
    client.send();
    client.clear();
    assert!(client.take_request().is_none());
    assert!(!client.complete(pending.id, Ok(response(200, "x"))));
    assert_eq!(client.state(), RequestState::Idle);
    assert!(client.response().is_none());
}

#[test]
fn test_send_without_a_url_is_refused() {
    let mut client = HttpClient::new();
    client.send();
    assert_eq!(client.state(), RequestState::Error);
    assert!(client.error().unwrap().starts_with("Invalid URL"));
    assert!(client.take_request().is_none());
    assert!(client.history_for_testing().is_empty());
}

#[test]
fn test_send_after_a_refused_url_does_not_send_the_previous_one() {
    let mut client = HttpClient::new().url("https://ok.io");
    client.set_url("ftp://refused");
    // The request still holds the previous URL ...
    assert_eq!(client.request().url(), "https://ok.io");
    client.send();
    // ... but that is not what the user asked to send
    assert_eq!(client.state(), RequestState::Error);
    assert!(client.take_request().is_none());
    assert!(client.history_for_testing().is_empty());
}

#[test]
fn test_refused_send_drops_the_request_in_flight() {
    let (mut client, first) = sent("https://ok.io");
    client.set_url("javascript:alert(1)");
    client.send();
    assert!(!client.complete(first.id, Ok(response(200, "x"))));
    assert_eq!(client.state(), RequestState::Error);
    assert!(client.error().unwrap().starts_with("Invalid URL"));
}

#[test]
fn test_a_valid_url_after_a_refused_one_can_be_sent() {
    let mut client = HttpClient::new();
    client.set_url("ftp://refused");
    client.set_url("https://ok.io");
    client.send();
    assert_eq!(
        client.take_request().unwrap().request.url(),
        "https://ok.io"
    );

    let mut built = HttpClient::new().url("ftp://refused").url("https://ok.io");
    built.send();
    assert_eq!(built.take_request().unwrap().request.url(), "https://ok.io");
}

/// A valid URL left the refusal of the one before it on screen and the
/// state at Error (#1083): it drops the refusal and the state is the
/// exchange's again, for the setter and the builder alike
#[test]
fn test_a_valid_url_drops_the_refusal_before_it() {
    let refused_then_ok = |c: &mut HttpClient| {
        c.set_url("ftp://refused");
        assert_eq!(c.state(), RequestState::Error);
        c.set_url("https://ok.io");
        assert_eq!(c.error(), None);
        c.state()
    };
    // Nothing sent: idle
    assert_eq!(refused_then_ok(&mut HttpClient::new()), RequestState::Idle);
    let built = HttpClient::new().url("ftp://refused").url("https://ok.io");
    assert_eq!((built.error(), built.state()), (None, RequestState::Idle));
    assert_eq!(built.url_cursor_for_testing(), 13);
    // A request in flight: still sending, and its result applies
    let (mut c, pending) = sent("https://a.io");
    assert_eq!(refused_then_ok(&mut c), RequestState::Sending);
    assert!(c.complete(pending.id, Ok(response(200, "x"))));
    // A response shown: its state, a 2xx or not
    assert_eq!(refused_then_ok(&mut c), RequestState::Success);
    c.set_response(response(500, "x"));
    assert_eq!(refused_then_ok(&mut c), RequestState::Error);
    // Another error stays: only the refusal goes
    let mut c = HttpClient::new();
    c.set_url("ftp://refused");
    c.set_error("down");
    c.set_url("https://ok.io");
    assert_eq!((c.error(), c.state()), (Some("down"), RequestState::Error));
    // A refused send is a refusal too
    let mut c = HttpClient::new();
    c.send();
    c.set_url("https://ok.io");
    assert_eq!((c.error(), c.state()), (None, RequestState::Idle));
}

/// A response applied after an error replaces it (#1083)
#[test]
fn test_a_response_after_an_error_clears_it() {
    let (mut c, pending) = sent("https://a.io");
    c.set_error("timeout");
    assert!(c.complete(pending.id, Ok(response(201, "x"))));
    assert_eq!((c.error(), c.state()), (None, RequestState::Success));
}

#[test]
fn test_mock_backend_runs_the_taken_request() {
    let backend = MockHttpBackend::new();
    backend.mock_json("/users", 200, r#"[{"id":1}]"#);
    backend.mock_error("/admin", 403, "no");

    let (mut client, pending) = sent("https://api.io/users");
    assert!(client.complete(pending.id, backend.send(&pending.request)));
    assert_eq!(client.state(), RequestState::Success);
    assert_eq!(client.response().unwrap().body, r#"[{"id":1}]"#);

    client.set_url("https://api.io/admin");
    client.send();
    let pending = client.take_request().unwrap();
    assert!(client.complete(pending.id, backend.send(&pending.request)));
    assert_eq!(client.state(), RequestState::Error);
    assert_eq!(client.response().unwrap().status, 403);
}
