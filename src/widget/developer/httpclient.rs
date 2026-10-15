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
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{HttpClient, HttpMethod};
//!
//! let mut client = HttpClient::new()
//!     .url("https://api.example.com/users")
//!     .method(HttpMethod::GET)
//!     .header("Authorization", "Bearer token");
//!
//! // Send request
//! client.send();
//!
//! // Check response
//! if let Some(response) = client.response() {
//!     println!("Status: {}", response.status);
//!     println!("Body: {}", response.body);
//! }
//! ```

mod backend;
mod builder;
mod client;
mod controls;
mod exchange;
mod helpers;
mod render;
mod request;
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
pub use response::HttpResponse;
pub use types::{ContentType, HttpColors, HttpMethod, RequestState, ResponseView};

use std::collections::HashMap;

use crate::widget::traits::WidgetProps;

// Both structs live here, in the module root, so that their fields stay
// private and every child file (builders, exchange, controls, render, the
// backend that matches on the URL) still sees them.

/// HTTP Client widget
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let mut client = HttpClient::new()
///     .url("https://api.example.com/users")
///     .method(HttpMethod::GET);
///
/// // Send request (async)
/// client.send();
///
/// // Check response
/// if let Some(response) = client.response() {
///     println!("Status: {}", response.status);
/// }
/// ```
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
    /// Body scroll
    body_scroll: usize,
    /// Request history
    history: Vec<HttpRequest>,
    /// History index
    history_index: usize,
    /// Show headers panel
    show_headers: bool,
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
