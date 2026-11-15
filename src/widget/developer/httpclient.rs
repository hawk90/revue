//! HTTP Client widget for REST API testing
//!
//! A Postman-like widget for making HTTP requests and viewing responses.
//!
//! # Features
//!
//! - Multiple HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
//! - Request headers and body
//! - Query parameters with URL builder
//! - Response body with JSON/XML formatting
//! - Loading and error states
//! - Request history navigation
//!
//! # Sending
//!
//! The widget does not do network I/O. [`HttpClient::send`] queues the
//! request; the app takes it with [`HttpClient::take_request`], runs it off
//! the UI thread with an [`HttpBackend`] (with the `http` feature,
//! `ReqwestBackend`), and applies the result on the UI thread with
//! [`HttpClient::complete`]. A later send replaces an earlier one, whose
//! result is then dropped.
//!
//! ```rust
//! use revue::widget::{HttpBackend, HttpClient, HttpMethod, MockHttpBackend, RequestState};
//!
//! let mut client = HttpClient::new()
//!     .url("https://api.example.com/users")
//!     .method(HttpMethod::GET)
//!     .header("Authorization", "Bearer token");
//!
//! client.send();
//! assert_eq!(client.state(), RequestState::Sending);
//!
//! // In the app: run it on a worker (a mock here), then complete it
//! let backend = MockHttpBackend::new();
//! backend.mock_json("/users", 200, r#"[{"id": 1}]"#);
//! let pending = client.take_request().unwrap();
//! let result = backend.send(&pending.request);
//! client.complete(pending.id, result);
//!
//! assert_eq!(client.response().unwrap().status, 200);
//! ```

mod backend;
mod builder;
mod client;
mod controls;
mod exchange;
mod helpers;
mod render;
mod request;
#[cfg(feature = "http")]
mod reqwest_backend;
mod response;
// The file's own `#[cfg(test)] mod tests` wrapper was removed (it made the
// path `tests::tests`), so the gate has to live on the declaration instead -
// otherwise the imports compile into normal builds with nothing using them.
mod testing;
#[cfg(test)]
mod tests;
mod types;

// Public exports
pub use backend::{HttpBackend, MockHttpBackend};
pub use builder::RequestBuilder;
pub use helpers::{delete, get, http_client, patch, post, put};
#[cfg(feature = "http")]
pub use reqwest_backend::ReqwestBackend;
pub use response::HttpResponse;
pub use types::{
    ContentType, HttpColors, HttpMethod, PendingRequest, RequestId, RequestState, ResponseView,
};

use crate::utils::ScrollState;
use std::collections::HashMap;

use crate::widget::traits::WidgetProps;

// Both structs live here, in the module root, so that their fields stay
// private and every child file (builders, exchange, controls, render, the
// backend that matches on the URL) still sees them.

/// HTTP Client widget
///
/// Shows a request and its response. It does not send anything itself:
/// [`send`](Self::send) queues the request, the app runs what
/// [`take_request`](Self::take_request) hands out, and
/// [`complete`](Self::complete) shows the result.
#[derive(Clone)]
pub struct HttpClient {
    /// Current request
    request: HttpRequest,
    /// Last response
    response: Option<HttpResponse>,
    /// Request state
    state: RequestState,
    /// Error message
    error: Option<String>,
    /// Response view mode
    view: ResponseView,
    /// Colors
    colors: HttpColors,
    /// URL cursor position
    url_cursor: usize,
    /// Where the shown view (body, headers or raw) is scrolled to, in lines
    scroll: ScrollState,
    /// Request history
    history: Vec<HttpRequest>,
    /// History index
    history_index: usize,
    /// Show headers panel
    show_headers: bool,
    /// The last URL given was refused, so `request.url` is an older one
    url_rejected: bool,
    /// Sent but not yet taken by the app
    pending: Option<PendingRequest>,
    /// The latest request sent and not yet completed: only its result applies
    in_flight: Option<RequestId>,
    /// The id of the latest request sent
    next_request_id: u64,
    /// Widget properties
    pub props: WidgetProps,
}

/// HTTP request builder
#[derive(Clone, Debug, Default)]
pub struct HttpRequest {
    /// HTTP method
    pub method: HttpMethod,
    /// Request URL (private: use url() getter for access)
    url: String,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: String,
    /// Query parameters
    pub params: HashMap<String, String>,
}
