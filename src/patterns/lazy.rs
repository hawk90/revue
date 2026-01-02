//! Lazy loading patterns for deferred data and UI rendering
//!
//! Provides utilities for loading data on-demand, pagination, and
//! progressive rendering of large datasets.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::patterns::lazy::{LazyData, LazyLoader, PagedData};
//!
//! // Lazy-loaded data
//! let data = LazyData::new(|| {
//!     // Expensive computation or API call
//!     fetch_data_from_api()
//! });
//!
//! // Access triggers loading
//! if let Some(value) = data.get() {
//!     println!("Loaded: {:?}", value);
//! }
//!
//! // Paged data for large datasets
//! let paged = PagedData::new(1000, 50, |page, size| {
//!     fetch_page(page, size)
//! });
//! ```

use std::cell::RefCell;
use std::sync::{Arc, RwLock};

/// Loading state for lazy data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoadState {
    /// Not yet loaded
    Idle,
    /// Currently loading
    Loading,
    /// Successfully loaded
    Loaded,
    /// Loading failed
    Failed,
}

/// A lazily loaded value (single-threaded)
pub struct LazyData<T, F>
where
    F: FnOnce() -> T,
{
    /// The loaded value
    value: RefCell<Option<T>>,
    /// The loader function
    loader: RefCell<Option<F>>,
    /// Current state
    state: RefCell<LoadState>,
}

impl<T, F> LazyData<T, F>
where
    F: FnOnce() -> T,
{
    /// Create new lazy data with a loader function
    pub fn new(loader: F) -> Self {
        Self {
            value: RefCell::new(None),
            loader: RefCell::new(Some(loader)),
            state: RefCell::new(LoadState::Idle),
        }
    }

    /// Get the value, loading if necessary
    pub fn get(&self) -> Option<std::cell::Ref<'_, T>> {
        self.ensure_loaded();
        let borrow = self.value.borrow();
        if borrow.is_some() {
            Some(std::cell::Ref::map(borrow, |opt| opt.as_ref().unwrap()))
        } else {
            None
        }
    }

    /// Check if data is loaded
    pub fn is_loaded(&self) -> bool {
        *self.state.borrow() == LoadState::Loaded
    }

    /// Get current state
    pub fn state(&self) -> LoadState {
        *self.state.borrow()
    }

    /// Force reload
    pub fn reload(&self)
    where
        F: Clone,
    {
        // Can't reload with FnOnce without Clone
        // This is a limitation - consider using Fn instead
    }

    /// Ensure the data is loaded
    fn ensure_loaded(&self) {
        if *self.state.borrow() != LoadState::Idle {
            return;
        }

        *self.state.borrow_mut() = LoadState::Loading;

        if let Some(loader) = self.loader.borrow_mut().take() {
            let value = loader();
            *self.value.borrow_mut() = Some(value);
            *self.state.borrow_mut() = LoadState::Loaded;
        }
    }
}

/// A lazily loaded value with reload support (single-threaded)
pub struct LazyReloadable<T, F>
where
    F: Fn() -> T,
{
    /// The loaded value
    value: RefCell<Option<T>>,
    /// The loader function
    loader: F,
    /// Current state
    state: RefCell<LoadState>,
}

impl<T, F> LazyReloadable<T, F>
where
    F: Fn() -> T,
{
    /// Create new lazy reloadable data
    pub fn new(loader: F) -> Self {
        Self {
            value: RefCell::new(None),
            loader,
            state: RefCell::new(LoadState::Idle),
        }
    }

    /// Get the value, loading if necessary
    pub fn get(&self) -> std::cell::Ref<'_, T> {
        self.ensure_loaded();
        std::cell::Ref::map(self.value.borrow(), |opt| opt.as_ref().unwrap())
    }

    /// Check if data is loaded
    pub fn is_loaded(&self) -> bool {
        *self.state.borrow() == LoadState::Loaded
    }

    /// Get current state
    pub fn state(&self) -> LoadState {
        *self.state.borrow()
    }

    /// Force reload
    pub fn reload(&self) {
        *self.state.borrow_mut() = LoadState::Loading;
        let value = (self.loader)();
        *self.value.borrow_mut() = Some(value);
        *self.state.borrow_mut() = LoadState::Loaded;
    }

    /// Invalidate cached value (will reload on next access)
    pub fn invalidate(&self) {
        *self.value.borrow_mut() = None;
        *self.state.borrow_mut() = LoadState::Idle;
    }

    /// Ensure the data is loaded
    fn ensure_loaded(&self) {
        if *self.state.borrow() == LoadState::Loaded {
            return;
        }

        *self.state.borrow_mut() = LoadState::Loading;
        let value = (self.loader)();
        *self.value.borrow_mut() = Some(value);
        *self.state.borrow_mut() = LoadState::Loaded;
    }
}

/// Thread-safe lazy data
pub struct LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    /// The loaded value
    value: Arc<RwLock<Option<T>>>,
    /// The loader function
    loader: Arc<RwLock<Option<F>>>,
    /// Current state
    state: Arc<RwLock<LoadState>>,
}

impl<T, F> LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    /// Create new thread-safe lazy data
    pub fn new(loader: F) -> Self {
        Self {
            value: Arc::new(RwLock::new(None)),
            loader: Arc::new(RwLock::new(Some(loader))),
            state: Arc::new(RwLock::new(LoadState::Idle)),
        }
    }

    /// Get the value, loading if necessary
    /// Returns a clone of the value
    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.ensure_loaded();
        self.value
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Get a read guard to the value without cloning (zero-copy access)
    ///
    /// Returns `None` if the value hasn't been loaded yet.
    /// Use `ensure_loaded()` or `get()` first if you need guaranteed access.
    ///
    /// # Example
    /// ```ignore
    /// let data = LazySync::new(|| vec![1, 2, 3]);
    /// data.get(); // trigger loading
    /// if let Some(guard) = data.read() {
    ///     println!("Length: {}", guard.as_ref().map(|v| v.len()).unwrap_or(0));
    /// }
    /// ```
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Option<T>> {
        self.ensure_loaded();
        self.value
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Try to get a read guard without blocking
    ///
    /// Returns `None` if the lock is currently held by a writer.
    pub fn try_read(&self) -> Option<std::sync::RwLockReadGuard<'_, Option<T>>> {
        self.value.try_read().ok()
    }

    /// Check if data is loaded
    pub fn is_loaded(&self) -> bool {
        *self
            .state
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            == LoadState::Loaded
    }

    /// Get current state
    pub fn state(&self) -> LoadState {
        *self
            .state
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// Ensure the data is loaded
    fn ensure_loaded(&self) {
        {
            let state = self
                .state
                .read()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if *state != LoadState::Idle {
                return;
            }
        }

        {
            let mut state = self
                .state
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            if *state != LoadState::Idle {
                return; // Double-check after acquiring write lock
            }
            *state = LoadState::Loading;
        }

        if let Some(loader) = self
            .loader
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take()
        {
            let value = loader();
            *self
                .value
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner()) = Some(value);
            *self
                .state
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner()) = LoadState::Loaded;
        }
    }
}

impl<T, F> Clone for LazySync<T, F>
where
    F: FnOnce() -> T + Send,
    T: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            loader: Arc::clone(&self.loader),
            state: Arc::clone(&self.state),
        }
    }
}

/// Paged data source for large datasets
pub struct PagedData<T, F>
where
    F: Fn(usize, usize) -> Vec<T>,
{
    /// Total item count
    total: usize,
    /// Page size
    page_size: usize,
    /// Loaded pages (page_index -> items)
    pages: RefCell<Vec<Option<Vec<T>>>>,
    /// Page loader function
    loader: F,
    /// Currently loading pages
    loading: RefCell<Vec<bool>>,
}

impl<T, F> PagedData<T, F>
where
    F: Fn(usize, usize) -> Vec<T>,
{
    /// Create new paged data source
    pub fn new(total: usize, page_size: usize, loader: F) -> Self {
        let num_pages = (total + page_size - 1) / page_size;
        let pages: Vec<Option<Vec<T>>> = (0..num_pages).map(|_| None).collect();
        let loading: Vec<bool> = vec![false; num_pages];
        Self {
            total,
            page_size,
            pages: RefCell::new(pages),
            loader,
            loading: RefCell::new(loading),
        }
    }

    /// Get total item count
    pub fn total(&self) -> usize {
        self.total
    }

    /// Get page size
    pub fn page_size(&self) -> usize {
        self.page_size
    }

    /// Get number of pages
    pub fn page_count(&self) -> usize {
        (self.total + self.page_size - 1) / self.page_size
    }

    /// Check if a page is loaded
    pub fn is_page_loaded(&self, page: usize) -> bool {
        self.pages
            .borrow()
            .get(page)
            .map(|p| p.is_some())
            .unwrap_or(false)
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<std::cell::Ref<'_, T>> {
        if index >= self.total {
            return None;
        }

        let page = index / self.page_size;
        let offset = index % self.page_size;

        self.ensure_page_loaded(page);

        let pages = self.pages.borrow();
        if let Some(Some(items)) = pages.get(page) {
            if offset < items.len() {
                return Some(std::cell::Ref::map(pages, |p| {
                    &p[page].as_ref().unwrap()[offset]
                }));
            }
        }
        None
    }

    /// Get a range of items (loads pages as needed)
    pub fn get_range(&self, start: usize, end: usize) -> Vec<std::cell::Ref<'_, T>> {
        let mut result = Vec::new();
        for i in start..end.min(self.total) {
            if let Some(item) = self.get(i) {
                result.push(item);
            }
        }
        result
    }

    /// Prefetch pages for upcoming scroll
    pub fn prefetch(&self, start_index: usize, count: usize) {
        let start_page = start_index / self.page_size;
        let end_page = (start_index + count) / self.page_size;

        for page in start_page..=end_page.min(self.page_count().saturating_sub(1)) {
            self.ensure_page_loaded(page);
        }
    }

    /// Invalidate a page (force reload on next access)
    pub fn invalidate_page(&self, page: usize) {
        if let Some(slot) = self.pages.borrow_mut().get_mut(page) {
            *slot = None;
        }
    }

    /// Invalidate all pages
    pub fn invalidate_all(&self) {
        for slot in self.pages.borrow_mut().iter_mut() {
            *slot = None;
        }
    }

    /// Ensure a page is loaded
    fn ensure_page_loaded(&self, page: usize) {
        if page >= self.page_count() {
            return;
        }

        {
            let pages = self.pages.borrow();
            if pages.get(page).map(|p| p.is_some()).unwrap_or(false) {
                return;
            }
        }

        // Mark as loading
        {
            let mut loading = self.loading.borrow_mut();
            if loading.get(page).copied().unwrap_or(false) {
                return; // Already loading
            }
            if let Some(slot) = loading.get_mut(page) {
                *slot = true;
            }
        }

        // Load the page
        let items = (self.loader)(page, self.page_size);

        // Store the page
        if let Some(slot) = self.pages.borrow_mut().get_mut(page) {
            *slot = Some(items);
        }

        // Clear loading flag
        if let Some(slot) = self.loading.borrow_mut().get_mut(page) {
            *slot = false;
        }
    }
}

/// Lazy list that loads items on demand
pub struct LazyList<T> {
    /// Loaded items (sparse)
    items: RefCell<Vec<Option<T>>>,
    /// Total capacity
    capacity: usize,
}

impl<T: Clone> LazyList<T> {
    /// Create a new lazy list with capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            items: RefCell::new(vec![None; capacity]),
            capacity,
        }
    }

    /// Get capacity
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Get item at index
    pub fn get(&self, index: usize) -> Option<T> {
        self.items.borrow().get(index).and_then(|opt| opt.clone())
    }

    /// Set item at index
    pub fn set(&self, index: usize, value: T) {
        if let Some(slot) = self.items.borrow_mut().get_mut(index) {
            *slot = Some(value);
        }
    }

    /// Check if item is loaded
    pub fn is_loaded(&self, index: usize) -> bool {
        self.items
            .borrow()
            .get(index)
            .map(|opt| opt.is_some())
            .unwrap_or(false)
    }

    /// Get number of loaded items
    pub fn loaded_count(&self) -> usize {
        self.items
            .borrow()
            .iter()
            .filter(|opt| opt.is_some())
            .count()
    }

    /// Clear item at index
    pub fn clear(&self, index: usize) {
        if let Some(slot) = self.items.borrow_mut().get_mut(index) {
            *slot = None;
        }
    }

    /// Clear all items
    pub fn clear_all(&self) {
        for slot in self.items.borrow_mut().iter_mut() {
            *slot = None;
        }
    }
}

/// Progressive loader for chunked loading
pub struct ProgressiveLoader<T> {
    /// All items to load
    items: Vec<T>,
    /// Number of items loaded so far
    loaded: RefCell<usize>,
    /// Chunk size
    chunk_size: usize,
}

impl<T: Clone> ProgressiveLoader<T> {
    /// Create a new progressive loader
    pub fn new(items: Vec<T>, chunk_size: usize) -> Self {
        Self {
            items,
            loaded: RefCell::new(0),
            chunk_size: chunk_size.max(1),
        }
    }

    /// Get total item count
    pub fn total(&self) -> usize {
        self.items.len()
    }

    /// Get number of items loaded
    pub fn loaded_count(&self) -> usize {
        *self.loaded.borrow()
    }

    /// Check if all items are loaded
    pub fn is_complete(&self) -> bool {
        *self.loaded.borrow() >= self.items.len()
    }

    /// Get loading progress (0.0 - 1.0)
    pub fn progress(&self) -> f32 {
        if self.items.is_empty() {
            1.0
        } else {
            *self.loaded.borrow() as f32 / self.items.len() as f32
        }
    }

    /// Load next chunk, returns the newly loaded items
    pub fn load_next(&self) -> Vec<T> {
        let start = *self.loaded.borrow();
        let end = (start + self.chunk_size).min(self.items.len());

        if start >= self.items.len() {
            return Vec::new();
        }

        let chunk: Vec<T> = self.items[start..end].to_vec();
        *self.loaded.borrow_mut() = end;
        chunk
    }

    /// Get all loaded items
    pub fn loaded_items(&self) -> Vec<T> {
        let count = *self.loaded.borrow();
        self.items[..count].to_vec()
    }

    /// Reset loading progress
    pub fn reset(&self) {
        *self.loaded.borrow_mut() = 0;
    }
}

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
    use std::rc::Rc;

    #[test]
    fn test_lazy_data() {
        let data = LazyData::new(|| 42);
        assert!(!data.is_loaded());
        assert_eq!(data.state(), LoadState::Idle);

        let value = data.get().unwrap();
        assert_eq!(*value, 42);
        assert!(data.is_loaded());
        assert_eq!(data.state(), LoadState::Loaded);
    }

    #[test]
    fn test_lazy_reloadable() {
        use std::cell::Cell;
        let counter = Rc::new(Cell::new(0));
        let counter_clone = counter.clone();

        let data = LazyReloadable::new(move || {
            counter_clone.set(counter_clone.get() + 1);
            counter_clone.get()
        });

        assert_eq!(*data.get(), 1);
        assert_eq!(*data.get(), 1); // Same value, not reloaded

        data.reload();
        assert_eq!(*data.get(), 2); // Reloaded
    }

    #[test]
    fn test_lazy_reloadable_invalidate() {
        use std::cell::Cell;
        let counter = Rc::new(Cell::new(0));
        let counter_clone = counter.clone();

        let data = LazyReloadable::new(move || {
            counter_clone.set(counter_clone.get() + 1);
            counter_clone.get()
        });

        assert_eq!(*data.get(), 1);
        data.invalidate();
        assert!(!data.is_loaded());
        assert_eq!(*data.get(), 2); // Reloaded on access
    }

    #[test]
    fn test_lazy_sync() {
        let data: LazySync<i32, _> = LazySync::new(|| 42);
        assert!(!data.is_loaded());

        let value = data.get().unwrap();
        assert_eq!(value, 42);
        assert!(data.is_loaded());
    }

    #[test]
    fn test_lazy_sync_zero_copy() {
        let data: LazySync<Vec<i32>, _> = LazySync::new(|| vec![1, 2, 3, 4, 5]);

        // Zero-copy read access
        {
            let guard = data.read();
            assert!(guard.is_some());
            let vec = guard.as_ref().unwrap();
            assert_eq!(vec.len(), 5);
            assert_eq!(vec[0], 1);
        }

        // try_read should work when no writers
        {
            let guard = data.try_read();
            assert!(guard.is_some());
        }
    }

    #[test]
    fn test_paged_data() {
        let data = PagedData::new(100, 10, |page, _size| {
            (page * 10..(page + 1) * 10).collect::<Vec<_>>()
        });

        assert_eq!(data.total(), 100);
        assert_eq!(data.page_size(), 10);
        assert_eq!(data.page_count(), 10);

        // Access item triggers page load
        assert!(!data.is_page_loaded(0));
        let item = data.get(5);
        assert!(item.is_some());
        assert_eq!(*item.unwrap(), 5);
        assert!(data.is_page_loaded(0));

        // Access item on another page
        let item = data.get(25);
        assert_eq!(*item.unwrap(), 25);
        assert!(data.is_page_loaded(2));
    }

    #[test]
    fn test_paged_data_prefetch() {
        let data = PagedData::new(100, 10, |page, _size| {
            (page * 10..(page + 1) * 10).collect::<Vec<_>>()
        });

        data.prefetch(15, 20); // Should load pages 1 and 2
        assert!(data.is_page_loaded(1));
        assert!(data.is_page_loaded(2));
    }

    #[test]
    fn test_lazy_list() {
        let list = LazyList::<i32>::new(10);
        assert_eq!(list.capacity(), 10);
        assert!(!list.is_loaded(5));

        list.set(5, 42);
        assert!(list.is_loaded(5));
        assert_eq!(list.get(5), Some(42));

        list.clear(5);
        assert!(!list.is_loaded(5));
    }

    #[test]
    fn test_progressive_loader() {
        let items: Vec<i32> = (0..25).collect();
        let loader = ProgressiveLoader::new(items, 10);

        assert_eq!(loader.total(), 25);
        assert_eq!(loader.loaded_count(), 0);
        assert!(!loader.is_complete());

        let chunk1 = loader.load_next();
        assert_eq!(chunk1.len(), 10);
        assert_eq!(loader.loaded_count(), 10);
        assert!((loader.progress() - 0.4).abs() < 0.01);

        let chunk2 = loader.load_next();
        assert_eq!(chunk2.len(), 10);
        assert_eq!(loader.loaded_count(), 20);

        let chunk3 = loader.load_next();
        assert_eq!(chunk3.len(), 5);
        assert!(loader.is_complete());

        loader.reset();
        assert_eq!(loader.loaded_count(), 0);
    }

    #[test]
    fn test_helper_functions() {
        let data = lazy(|| "hello");
        assert_eq!(*data.get().unwrap(), "hello");

        let data = lazy_reloadable(|| 42);
        assert_eq!(*data.get(), 42);

        let data: LazySync<Vec<i32>, _> = lazy_sync(|| vec![1, 2, 3]);
        assert_eq!(data.get().unwrap(), vec![1, 2, 3]);
    }
}
