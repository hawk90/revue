//! Custom Event Dispatching System
//!
//! Provides a type-safe, extensible event system for user-defined events.
//!
//! # Features
//!
//! - Custom event type registration
//! - Type-safe event dispatching
//! - Event bubbling/capturing support
//! - Event metadata (timestamp, source)
//! - Event cancellation
//! - Priority levels
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::event::custom::*;
//!
//! // Define a custom event
//! struct ChatMessage {
//!     user: String,
//!     text: String,
//! }
//!
//! impl CustomEvent for ChatMessage {
//!     fn event_type() -> &'static str { "chat_message" }
//! }
//!
//! // Create dispatcher and register handler
//! let mut dispatcher = EventDispatcher::new();
//!
//! dispatcher.on::<ChatMessage>(|event, ctx| {
//!     println!("{}: {}", event.user, event.text);
//!     EventResponse::Handled
//! });
//!
//! // Dispatch event
//! dispatcher.dispatch(ChatMessage {
//!     user: "Alice".to_string(),
//!     text: "Hello!".to_string(),
//! });
//! ```

mod bus;
mod dispatcher;
mod events;
mod handler;
mod response;
mod result;
mod types;

#[cfg(test)]
mod tests;

// Re-exports
pub use bus::{CustomEventBus, EventRecord};
pub use dispatcher::EventDispatcher;
pub use events::{AppEvent, ErrorEvent, NavigateEvent, StateChangeEvent};
pub use handler::{CustomHandlerId, HandlerOptions};
pub use response::EventResponse;
pub use result::DispatchResult;
pub use types::{CustomEvent, DispatchPhase, EventEnvelope, EventId, EventMeta, EventPriority};
