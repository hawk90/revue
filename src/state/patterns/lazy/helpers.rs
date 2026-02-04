//! Helper functions for lazy loading

use super::{LazyData, LazyReloadable, LazySync, PagedData, ProgressiveLoader};

/// Create a lazy data loader
pub fn lazy<T, F>(loader: F) -> LazyData<T, F>
where
    F: FnOnce() -> T,
{
    LazyData::new(loader)
}

/// Create a reloadable lazy loader
pub fn lazy_reloadable<T, F>(loader: F) -> LazyReloadable<T, F>
where
    F: Fn() -> T,
{
    LazyReloadable::new(loader)
}

/// Create a thread-safe lazy loader
pub fn lazy_sync<T, F>(loader: F) -> LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    LazySync::new(loader)
}

/// Create a paged data source
pub fn paged<T, F>(total: usize, page_size: usize, loader: F) -> PagedData<T, F>
where
    F: Fn(usize, usize) -> Vec<T>,
{
    PagedData::new(total, page_size, loader)
}

/// Create a progressive loader
pub fn progressive<T: Clone>(items: Vec<T>, chunk_size: usize) -> ProgressiveLoader<T> {
    ProgressiveLoader::new(items, chunk_size)
}
