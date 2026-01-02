//! Common patterns for TUI applications
//!
//! This module provides reusable patterns discovered while building multiple TUI apps
//! (jira-revue, jenkins-tui, sshfs-tui, todo-tui). These patterns follow Clean Code and
//! Single Responsibility Principle.
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`colors`] | GitHub Dark theme color constants |
//! | [`message`] | Message display with auto-timeout |
//! | [`confirm`] | Confirmation dialog state |
//! | [`async_ops`] | Async polling patterns with mpsc channels |
//! | [`config`] | TOML config loading utilities |
//! | [`keys`] | Layered key handling pattern |
//!
//! # Example
//!
//! ```ignore
//! use revue::patterns::*;
//!
//! struct App {
//!     message: MessageState,
//!     confirm: ConfirmState,
//!     async_task: Option<AsyncTask<Vec<Item>>>,
//! }
//!
//! impl App {
//!     fn poll(&mut self) -> bool {
//!         let mut needs_redraw = false;
//!
//!         // Check message timeout
//!         needs_redraw |= self.message.check_timeout();
//!
//!         // Poll async task
//!         if let Some(task) = &mut self.async_task {
//!             if let Some(result) = task.try_recv() {
//!                 self.handle_result(result);
//!                 self.async_task = None;
//!                 needs_redraw = true;
//!             }
//!         }
//!
//!         needs_redraw
//!     }
//! }
//! ```

pub mod colors;
pub mod message;
pub mod confirm;
pub mod async_ops;
pub mod config;
pub mod keys;
pub mod search;
pub mod form;
pub mod navigation;
pub mod lazy;

// Re-export commonly used items
pub use colors::*;
pub use message::MessageState;
pub use confirm::{ConfirmAction, ConfirmState};
pub use async_ops::{AsyncTask, spinner_char, SPINNER_FRAMES};
pub use config::{AppConfig, ConfigError};
pub use search::{SearchState, SearchMode};
pub use form::{FormState, FormField, FieldType, ValidationError, Validators};
pub use navigation::{NavigationState, Route, NavigationEvent, BreadcrumbItem, build_breadcrumbs};
pub use lazy::{
    LoadState, LazyData, LazyReloadable, LazySync, PagedData, LazyList, ProgressiveLoader,
    lazy, lazy_reloadable, lazy_sync, paged, progressive,
};
