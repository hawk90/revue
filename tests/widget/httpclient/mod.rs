//! HttpClient widget tests
//!
//! No test touches the network: HttpClient::send() builds a mock response
//! and MockHttpBackend only answers from its registered mocks.

pub mod backend;
pub mod builder;
pub mod client;
pub mod helpers;
pub mod rendering_scroll_history;
pub mod request;
pub mod response;
pub mod types;
