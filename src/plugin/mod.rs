//! Plugin system for extending Revue applications
//!
//! Plugins provide a modular way to extend app functionality with lifecycle hooks,
//! custom styles, and shared state.
//!
//! # Features
//!
//! | Feature | Description |
//!|---------|-------------|
//! | **Lifecycle Hooks** | Init, tick, event, shutdown callbacks |
//! | **Shared State** | Global state accessible to all plugins |
//! | **Custom Styles** | Add CSS variables and rules |
//! | **Event Handling** | Subscribe to and emit events |
//! | **Async Support** | Async lifecycle operations |
//!
//! # Quick Start
//!
//! ## Create a Plugin
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
//!     fn on_init(&mut self, ctx: &mut PluginContext) -> revue::Result<()> {
//!         println!("[{}] Plugin initialized", self.name());
//!         Ok(())
//!     }
//!
//!     fn on_tick(&mut self, _ctx: &mut PluginContext, delta: Duration) -> revue::Result<()> {
//!         self.event_count += 1;
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Register a Plugin
//!
//! ```rust,ignore
//! use revue::App;
//!
//! let app = App::builder()
//!     .plugin(LoggerPlugin { event_count: 0 })
//!     .build();
//! ```
//!
//! # Lifecycle Hooks
//!
//! | Hook | When Called | Use Case |
//!|------|-------------|----------|
//! | `on_init` | After app creation | Setup, initialization |
//! | `on_mount` | When view is mounted | UI setup, subscriptions |
//! | `on_tick` | Every frame | Update logic, animation |
//! | `on_event` | On input event | Event handling |
//! | `on_shutdown` | Before app exit | Cleanup, saving |
//!
//! # Plugin Context
//!
//! The [`PluginContext`] provides access to:
//!
//! ```rust,ignore
//! impl Plugin for MyPlugin {
//!     fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()> {
//!         // Access app state
//!         let state = ctx.state();
//!
//!         // Emit events
//!         ctx.emit("my_event", data);
//!
//!         // Subscribe to events
//!         ctx.subscribe("other_event", |data| {
//!             println!("Got event: {:?}", data);
//!         });
//!
//!         Ok(())
//!     }
//! }
//! ```
//!
//! # Built-in Plugins
//!
//! ## LoggerPlugin
//!
//! Logs app lifecycle events and errors:
//!
//! ```rust,ignore
//! use revue::plugin::LoggerPlugin;
//!
//! let app = App::builder()
//!     .plugin(LoggerPlugin::new())
//!     .build();
//! ```
//!
//! ## PerformancePlugin
//!
//! Tracks FPS, frame time, and memory usage:
//!
//! ```rust,ignore
//! use revue::plugin::PerformancePlugin;
//!
//! let app = App::builder()
//!     .plugin(PerformancePlugin::new())
//!     .build();
//! ```
//!
//! # Plugin Registry
//!
//! Access and manage plugins:
//!
//! ```rust,ignore
//! use revue::plugin::PluginRegistry;
//!
//! // Get plugin by name
//! if let Some(logger) = registry.get("logger") {
//!     // Use plugin...
//! }
//!
//! // List all plugins
//! for name in registry.plugin_names() {
//!     println!("{}", name);
//! }
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
