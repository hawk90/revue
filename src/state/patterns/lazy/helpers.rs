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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lazy_function() {
        let lazy_data = lazy(|| 42);
        // Just verify it creates a LazyData (can't inspect private fields)
        let _ = lazy_data;
    }

    #[test]
    fn test_lazy_reloadable_function() {
        let lazy_data = lazy_reloadable(|| vec![1, 2, 3]);
        // Just verify it creates a LazyReloadable (can't inspect private fields)
        let _ = lazy_data;
    }

    #[test]
    fn test_lazy_sync_function() {
        let lazy_data = lazy_sync(|| vec![1, 2, 3]);
        // Just verify it creates a LazySync (can't inspect private fields)
        let _ = lazy_data;
    }

    #[test]
    fn test_paged_function() {
        let paged = paged(100, 10, |start, count| (start..start + count).collect());
        // Just verify it creates a PagedData (can't inspect private fields)
        let _ = paged;
    }

    #[test]
    fn test_progressive_function() {
        let items = vec![1, 2, 3, 4, 5];
        let progressive = progressive(items, 2);
        // Just verify it creates a ProgressiveLoader (can't inspect private fields)
        let _ = progressive;
    }

    #[test]
    fn test_lazy_with_closure() {
        let lazy_data = lazy(|| vec![1, 2, 3]);
        let _ = lazy_data;
    }

    #[test]
    fn test_lazy_reloadable_with_closure() {
        let counter = std::cell::RefCell::new(0);
        let lazy_data = lazy_reloadable(|| {
            let mut count = counter.borrow_mut();
            *count += 1;
            *count
        });
        let _ = lazy_data;
    }

    #[test]
    fn test_paged_with_loader() {
        let paged = paged(50, 10, |offset, limit| vec![(offset, limit)]);
        let _ = paged;
    }

    #[test]
    fn test_progressive_with_items() {
        let items = vec!["a", "b", "c", "d", "e"];
        let progressive = progressive(items.clone(), 2);
        let _ = progressive;
    }

    #[test]
    fn test_progressive_empty_items() {
        let items: Vec<i32> = vec![];
        let progressive = progressive(items, 10);
        let _ = progressive;
    }
}
