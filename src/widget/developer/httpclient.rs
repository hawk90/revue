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
mod helpers;
mod render;
mod request;
mod response;
mod tests;
mod types;

// Public exports
pub use backend::{HttpBackend, MockHttpBackend};
pub use builder::RequestBuilder;
pub use client::HttpClient;
pub use helpers::{delete, get, http_client, patch, post, put};
pub use request::HttpRequest;
pub use response::HttpResponse;
pub use types::{ContentType, HttpMethod, RequestState, ResponseView};
