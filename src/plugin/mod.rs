//! Plugin system for extending Revue applications
//!
//! Plugins provide a modular way to extend app functionality with lifecycle hooks,
//! custom styles, and shared state.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::plugin::{Plugin, PluginContext};
//! use std::time::Duration;
//!
//! struct LoggerPlugin {
//!     event_count: usize,
//! }
//!
//! impl Plugin for LoggerPlugin {
//!     fn name(&self) -> &str { "logger" }
//!
//!     fn on_init(&mut self, _ctx: &mut PluginContext) -> revue::Result<()> {
//!         println!("[Logger] Plugin initialized");
//!         Ok(())
//!     }
//!
//!     fn on_tick(&mut self, _ctx: &mut PluginContext, delta: Duration) -> revue::Result<()> {
//!         self.event_count += 1;
//!         Ok(())
//!     }
//! }
//!
//! // Usage
//! let app = App::builder()
//!     .plugin(LoggerPlugin { event_count: 0 })
//!     .build();
//! ```

mod context;
mod registry;
mod traits;

pub use context::PluginContext;
pub use registry::PluginRegistry;
pub use traits::Plugin;

// Built-in plugins
mod builtin;
pub use builtin::{LoggerPlugin, PerformancePlugin};
