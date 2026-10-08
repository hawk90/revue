//! Lazy loading patterns for deferred data and UI rendering
//!
//! This module provides patterns for data that is expensive to produce, so
//! that it is produced only when - and as far as - it is needed.
//!
//! The loaders run synchronously, on the thread that first reads the value.
//! For work that must not block the UI, load in the background with
//! [`AsyncTask`](crate::patterns::AsyncTask) instead.
//!
//! # Overview
//!
//! Lazy loading patterns help manage data that:
//! - Is expensive to compute or read
//! - Is too large to load all at once
//! - Should be loaded on-demand as users navigate
//! - Needs periodic refresh/reloading
//!
//! # Patterns
//!
//! ## LazyData
//!
//! A value loaded once, on first access.
//!
//! ```
//! use revue::patterns::lazy::{LazyData, LoadState};
//!
//! let items = LazyData::new(|| vec!["one", "two"]);
//! assert_eq!(items.state(), LoadState::Idle);
//!
//! // The loader runs here
//! assert_eq!(items.get().unwrap().len(), 2);
//! assert!(items.is_loaded());
//! ```
//!
//! ## PagedData
//!
//! Large data loaded a page at a time, as the pages are read.
//!
//! ```
//! use revue::patterns::lazy::PagedData;
//!
//! // 1000 rows, 50 per page; the loader gets the page number and size
//! let rows = PagedData::new(1000, 50, |page, size| {
//!     (page * size..(page + 1) * size).map(|i| format!("row {}", i)).collect()
//! });
//!
//! assert_eq!(&*rows.get(120).unwrap(), "row 120");
//! assert!(rows.is_page_loaded(2));
//! assert!(!rows.is_page_loaded(0));
//! ```
//!
//! ## LazyList
//!
//! A fixed-size list whose items are filled in as they arrive.
//!
//! ```
//! use revue::patterns::lazy::LazyList;
//!
//! let list: LazyList<String> = LazyList::new(100);
//! list.set(3, "fourth".to_string());
//!
//! assert_eq!(list.get(3).as_deref(), Some("fourth"));
//! assert!(!list.is_loaded(0));
//! assert_eq!(list.loaded_count(), 1);
//! ```
//!
//! ## LazyReloadable
//!
//! Data that can be refreshed/reloaded.
//!
//! ```
//! use revue::patterns::lazy::LazyReloadable;
//! use std::cell::Cell;
//!
//! let fetches = Cell::new(0);
//! let stats = LazyReloadable::new(|| {
//!     fetches.set(fetches.get() + 1);
//!     fetches.get()
//! });
//!
//! assert_eq!(*stats.get(), 1);
//!
//! // Drop the cached value; the next read loads it again
//! stats.invalidate();
//! assert_eq!(*stats.get(), 2);
//!
//! // Or load it again right away
//! stats.reload();
//! assert_eq!(*stats.get(), 3);
//! ```
//!
//! ## ProgressiveLoader
//!
//! Hands out items a chunk at a time and reports progress.
//!
//! ```
//! use revue::patterns::lazy::ProgressiveLoader;
//!
//! let loader = ProgressiveLoader::new((1..=10).collect::<Vec<_>>(), 4);
//!
//! // e.g. one chunk per tick
//! assert_eq!(loader.load_next(), vec![1, 2, 3, 4]);
//! assert_eq!(loader.progress(), 0.4);
//! loader.load_next();
//! loader.load_next();
//! assert!(loader.is_complete());
//! ```
//!
//! # Helper Functions
//!
//! - [`lazy()`][helpers::lazy] - Create a LazyData
//! - [`paged()`][helpers::paged] - Create a PagedData
//! - [`progressive()`][helpers::progressive] - Create a ProgressiveLoader
//! - [`lazy_reloadable()`][helpers::lazy_reloadable] - Create a LazyReloadable
//!
//! # State Management
//!
//! The single-value patterns report a [`LoadState`]:
//!
//! ```
//! use revue::patterns::lazy::{LazyData, LoadState};
//!
//! let data = LazyData::new(|| 42);
//!
//! let status = match data.state() {
//!     LoadState::Idle => "Not loaded",
//!     LoadState::Loading => "Loading...",
//!     LoadState::Loaded => "Ready",
//!     LoadState::Failed => "Error loading",
//! };
//! assert_eq!(status, "Not loaded");
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
