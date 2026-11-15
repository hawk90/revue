//! HttpClient widget tests
//!
//! Only `reqwest_backend` touches the network, and only a server on
//! 127.0.0.1 that the test starts itself. HttpClient::send() queues the
//! request for the app, and MockHttpBackend answers from its mocks.

pub mod backend;
pub mod builder;
pub mod client;
pub mod exchange;
pub mod helpers;
pub mod model;
pub mod rendering_scroll_history;
pub mod request;
#[cfg(feature = "http")]
pub mod reqwest_backend;
pub mod response;
pub mod types;
pub mod view;
