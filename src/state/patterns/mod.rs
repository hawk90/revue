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
//! | [`UndoStack`] | Generic undo/redo stack | Text editing, state history |
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
//! ```
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
//! ```
//! use revue::patterns::{spinner_char, AsyncTask};
//!
//! fn fetch_items() -> Vec<String> {
//!     vec!["one".into()] // Expensive operation
//! }
//!
//! struct App {
//!     items: Vec<String>,
//!     task: Option<AsyncTask<Vec<String>>>,
//!     frame: usize,
//! }
//!
//! impl App {
//!     fn start_loading(&mut self) {
//!         self.task = Some(AsyncTask::spawn(fetch_items));
//!     }
//!
//!     fn poll(&mut self) -> bool {
//!         let Some(task) = &mut self.task else { return false };
//!         match task.try_recv() {
//!             Some(result) => {
//!                 self.items = result;
//!                 self.task = None;
//!             }
//!             // Still loading: advance the spinner
//!             None => self.frame += 1,
//!         }
//!         true
//!     }
//!
//!     fn status(&self) -> &str {
//!         if self.task.is_some() { spinner_char(self.frame) } else { "✓" }
//!     }
//! }
//!
//! let mut app = App { items: Vec::new(), task: None, frame: 0 };
//! app.start_loading();
//! while app.task.is_some() {
//!     app.poll();
//! #   std::thread::sleep(std::time::Duration::from_millis(1));
//! }
//! assert_eq!(app.status(), "✓");
//! ```
//!
//! ## Form with Validation
//!
//! ```
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
//!                 .field("username", |f| f.label("Username").required())
//!                 .field("password", |f| f.password().required().min_length(8))
//!                 .build(),
//!         }
//!     }
//!
//!     /// The first error of each invalid field, as (field, message)
//!     fn submit(&self) -> Result<(), Vec<(String, String)>> {
//!         if self.form.submit() { Ok(()) } else { Err(self.form.errors()) }
//!     }
//! }
//!
//! let app = App::new();
//! app.form.set_value("username", "alice");
//! app.form.set_value("password", "short");
//! assert_eq!(app.submit().unwrap_err().len(), 1);
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
pub mod undo;

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
pub use undo::UndoStack;
