//! Lazy loading patterns for deferred data and UI rendering

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
