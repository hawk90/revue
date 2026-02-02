//! Lazy loading patterns for deferred data and UI rendering
//!
//! This module provides patterns for managing data that loads asynchronously
//! or incrementally, allowing your TUI to remain responsive while data is being fetched.
//!
//! # Overview
//!
//! Lazy loading patterns help manage data that:
//! - Takes time to load from APIs or databases
//! - Is too large to load all at once
//! - Should be loaded on-demand as users navigate
//! - Needs periodic refresh/reloading
//!
//! # Patterns
//!
//! ## LazyData
//!
//! Simple wrapper for data that loads once asynchronously.
//!
//! ```rust,ignore
//! use revue::patterns::lazy::LazyData;
//!
//! struct App {
//!     items: LazyData<Vec<Item>>,
//! }
//!
//! impl App {
//!     fn load_items(&mut self) {
//!         self.items = LazyData::loading();
//!         // Start async fetch...
//!     }
//!
//!     fn on_items_loaded(&mut self, items: Vec<Item>) {
//!         self.items = LazyData::loaded(items);
//!     }
//! }
//! ```
//!
//! ## LazyList
//!
//! List data that loads in pages/chunks.
//!
//! ```rust,ignore
//! use revue::patterns::lazy::LazyList;
//!
//! struct App {
//!     list: LazyList<Item>,
//! }
//!
//! impl App {
//!     fn load_more(&mut self) {
//!         if let Some(page) = self.list.next_page_to_load() {
//!             self.fetch_page(page);
//!         }
//!     }
//! }
//! ```
//!
//! ## LazyReloadable
//!
//! Data that can be refreshed/reloaded.
//!
//! ```rust,ignore
//! use revue::patterns::lazy::LazyReloadable;
//!
//! struct App {
//!     data: LazyReloadable<Stats>,
//! }
//!
//! impl App {
//!     fn refresh(&mut self) {
//!         self.data.mark_needs_reload();
//!     }
//!
//!     fn poll(&mut self) {
//!         if self.data.needs_reload() {
//!             self.fetch_stats();
//!         }
//!     }
//! }
//! ```
//!
//! ## ProgressiveLoader
//!
//! Loads data incrementally and shows progress.
//!
//! ```rust,ignore
//! use revue::patterns::lazy::ProgressiveLoader;
//!
//! struct App {
//!     loader: ProgressiveLoader<String>,
//! }
//!
//! impl App {
//!     fn add_chunk(&mut self, chunk: String) {
//!         self.loader.add(chunk);
//!     }
//!
//!     fn progress(&self) -> f32 {
//!         self.loader.progress()
//!     }
//! }
//! ```
//!
//! # Helper Functions
//!
//! - [`lazy()`] - Create a LazyData
//! - [`paged()`] - Create a PagedData
//! - [`progressive()`] - Create a ProgressiveLoader
//! - [`lazy_reloadable()`] - Create a LazyReloadable
//!
//! # State Management
//!
//! All patterns use [`LoadState`] to track loading progress:
//!
//! ```rust,ignore
//! use revue::patterns::lazy::LoadState;
//!
//! match data.state() {
//!     LoadState::Idle => "Not loaded",
//!     LoadState::Loading => "Loading...",
//!     LoadState::Loaded => "Ready",
//!     LoadState::Failed => "Error loading",
//! };
//! ```

mod helpers;
mod lazy_data;
mod lazy_list;
mod lazy_reloadable;
mod lazy_sync;
mod paged_data;
mod progressive;
mod types;

pub use lazy_data::LazyData;
pub use lazy_list::LazyList;
pub use lazy_reloadable::LazyReloadable;
pub use lazy_sync::LazySync;
pub use paged_data::PagedData;
pub use progressive::ProgressiveLoader;
pub use types::LoadState;

pub use helpers::{lazy, lazy_reloadable, lazy_sync, paged, progressive};
