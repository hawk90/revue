//! Common patterns for TUI applications
//!
//! This module provides reusable patterns discovered while building multiple TUI apps
//! (jira-revue, jenkins-tui, sshfs-tui, todo-tui). These patterns follow Clean Code and
//! Single Responsibility Principle.
//!
#![allow(rustdoc::broken_intra_doc_links)]
//!
//! # Pattern Categories
//!
//! ## State Management Patterns
//!
//! | Pattern | Description | Use Case |
//! |---------|-------------|----------|
//! | [`MessageState`] | Auto-expiring messages | Toast notifications, status updates |
//! | [`ConfirmState`] | Confirmation dialogs | Destructive actions, confirmations |
//! | [`SearchState`] | Search/filter input | List filtering, query input |
//!
//! ## Async Patterns
//!
//! | Pattern | Description | Use Case |
//! |---------|-------------|----------|
//! | [`AsyncTask`] | Async polling with mpsc | API calls, file I/O, background work |
//! | [`ProgressiveLoader`] | Progressive data loading | Large datasets, pagination |
//!
//! ## Data Loading Patterns
//!
//! | Pattern | Description | Use Case |
//! |---------|-------------|----------|
//! | [`LazyData`] | On-demand data loading | Expensive computations, caching |
//! | [`LazySync`] | Synchronized lazy loading | Thread-safe lazy initialization |
//! | [`LazyReloadable`] | Reloadable data | Hot-reload, refreshable content |
//! | [`PagedData`] | Paginated data | Large lists, API pagination |
//!
//! ## Configuration Patterns
//!
//! | Pattern | Description | Use Case |
//! |---------|-------------|----------|
//! | [`AppConfig`] | TOML config loading | Application configuration |
//!
//! ## Interaction Patterns
//!
//! | Pattern | Description | Use Case |
//! |---------|-------------|----------|
//! | [`KeyHandler`] | Layered key handling | Modal UI, context-sensitive keys |
//! | [`FormState`] | Form field management | Multi-field input forms |
//! | [`NavigationState`] | Navigation stack | Breadcrumbs, route history |
//!
//! ## UI Patterns
//!
//! | Pattern | Description | Use Case |
//! |---------|-------------|----------|
//! | [`colors`] | Color constants | Themed UIs |
//!
//! # Examples
//!
//! ## Message with Auto-Timeout
//!
//! ```ignore
//! use revue::patterns::MessageState;
//!
//! struct App {
//!     message: MessageState,
//! }
//!
//! impl App {
//!     fn show_status(&mut self, msg: String) {
//!         self.message.set(msg);  // Auto-clears after 3 seconds
//!     }
//!
//!     fn poll(&mut self) -> bool {
//!         self.message.check_timeout()  // Returns true if expired
//!     }
//! }
//! ```
//!
//! ## Async Task with Spinner
//!
//! ```ignore
//! use revue::patterns::{AsyncTask, spinner_char};
//!
//! struct App {
//!     task: Option<AsyncTask<Vec<Item>>>,
//! }
//!
//! impl App {
//!     fn start_loading(&mut self) {
//!         self.task = Some(AsyncTask::new(async {
//!             // Expensive operation
//!             fetch_items().await
//!         }));
//!     }
//!
//!     fn poll(&mut self) -> bool {
//!         if let Some(task) = &mut self.task {
//!             match task.try_recv() {
//!                 Some(result) => {
//!                     self.items = result;
//!                     self.task = None;
//!                     true
//!                 }
//!                 None => {
//!                     // Show spinner
//!                     let frame = spinner_char();
//!                     self.draw_spinner(frame);
//!                     false
//!                 }
//!             }
//!         } else {
//!             false
//!         }
//!     }
//! }
//! ```
//!
//! ## Form with Validation
//!
//! ```ignore
//! use revue::patterns::FormState;
//!
//! struct App {
//!     form: FormState,
//! }
//!
//! impl App {
//!     fn new() -> Self {
//!         Self {
//!             form: FormState::new()
//!                 .field("username", FieldType::Text)
//!                 .field("password", FieldType::Password)
//!                 .field("remember", FieldType::Checkbox),
//!         }
//!     }
//!
//!     fn submit(&self) -> Result<(), Vec<ValidationError>> {
//!         self.form.validate()
//!     }
//! }
//! ```

pub mod async_ops;
pub mod colors;
#[cfg(feature = "config")]
pub mod config;
pub mod confirm;
pub mod form;
pub mod keys;
pub mod lazy;
pub mod message;
pub mod navigation;
pub mod search;

// Re-export commonly used items
pub use async_ops::{spinner_char, AsyncTask, SPINNER_FRAMES};
pub use colors::*;
#[cfg(feature = "config")]
pub use config::{AppConfig, ConfigError};
pub use confirm::{ConfirmAction, ConfirmState};
pub use form::{FieldType, FormField, FormState, ValidationError, Validators};
pub use lazy::{
    lazy, lazy_reloadable, lazy_sync, paged, progressive, LazyData, LazyList, LazyReloadable,
    LazySync, LoadState, PagedData, ProgressiveLoader,
};
pub use message::MessageState;
pub use navigation::{build_breadcrumbs, BreadcrumbItem, NavigationEvent, NavigationState, Route};
pub use search::{SearchMode, SearchState};
