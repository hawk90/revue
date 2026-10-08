//! Plugin system for extending Revue applications
//!
//! Plugins provide a modular way to extend app functionality with lifecycle hooks,
//! custom styles, and shared data.
//!
//! # Features
//!
//! | Feature | Description |
//!|---------|-------------|
//! | **Lifecycle Hooks** | Init, mount, tick and unmount callbacks |
//! | **Shared Data** | Per-plugin data that other plugins can read |
//! | **Custom Styles** | Add CSS variables and rules |
//!
//! # Quick Start
//!
//! ## Create a Plugin
//!
//! ```
//! use revue::plugin::{Plugin, PluginContext};
//! use std::time::Duration;
//!
//! struct TickCounter {
//!     ticks: usize,
//! }
//!
//! impl Plugin for TickCounter {
//!     fn name(&self) -> &str { "tick-counter" }
//!
//!     fn on_init(&mut self, ctx: &mut PluginContext) -> revue::Result<()> {
//!         ctx.log("initialized");
//!         Ok(())
//!     }
//!
//!     fn on_tick(&mut self, ctx: &mut PluginContext, _delta: Duration) -> revue::Result<()> {
//!         self.ticks += 1;
//!         ctx.set_data("ticks", self.ticks);
//!         Ok(())
//!     }
//! }
//! ```
//!
//! ## Register a Plugin
//!
//! ```
//! # use revue::plugin::Plugin;
//! # struct TickCounter { ticks: usize }
//! # impl Plugin for TickCounter { fn name(&self) -> &str { "tick-counter" } }
//! use revue::prelude::App;
//!
//! let app = App::builder()
//!     .plugin(TickCounter { ticks: 0 })
//!     .build();
//! assert!(app.plugins().has_plugin("tick-counter"));
//! ```
//!
//! # Lifecycle Hooks
//!
//! | Hook | When Called | Use Case |
//!|------|-------------|----------|
//! | `on_init` | After app creation | Setup, initialization |
//! | `on_mount` | When the app starts running | UI setup, subscriptions |
//! | `on_tick` | Every frame | Update logic, animation |
//! | `on_unmount` | When the app stops | Cleanup, saving |
//!
//! # Plugin Context
//!
//! The [`PluginContext`] passed to each hook gives access to the terminal
//! size, whether the app is running, logging, and data storage. Data a plugin
//! stores is kept under its name, and other plugins can read it:
//!
//! ```
//! use revue::plugin::{PerformancePlugin, PluginRegistry};
//!
//! let mut registry = PluginRegistry::new();
//! registry.register(PerformancePlugin::new());
//! registry.init()?;
//!
//! // The data PerformancePlugin stored, read from outside it
//! let fps = registry.context().get_plugin_data::<f64>("performance", "fps");
//! assert_eq!(fps, Some(&0.0));
//! # Ok::<(), revue::Error>(())
//! ```
//!
//! # Built-in Plugins
//!
//! ## LoggerPlugin
//!
//! Logs app lifecycle events:
//!
//! ```
//! use revue::plugin::LoggerPlugin;
//! use revue::prelude::App;
//!
//! let app = App::builder()
//!     .plugin(LoggerPlugin::new())
//!     .build();
//! ```
//!
//! ## PerformancePlugin
//!
//! Tracks FPS and frame time:
//!
//! ```
//! use revue::plugin::PerformancePlugin;
//! use revue::prelude::App;
//!
//! let app = App::builder()
//!     .plugin(PerformancePlugin::new())
//!     .build();
//! ```
//!
//! # Plugin Registry
//!
//! The app keeps its plugins in a [`PluginRegistry`]:
//!
//! ```
//! use revue::plugin::{LoggerPlugin, PerformancePlugin};
//! use revue::prelude::App;
//!
//! let app = App::builder()
//!     .plugin(LoggerPlugin::new())
//!     .plugin(PerformancePlugin::new())
//!     .build();
//!
//! let registry = app.plugins();
//! assert!(registry.has_plugin("logger"));
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
