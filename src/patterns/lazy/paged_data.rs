//! Paged data source for large datasets

use std::cell::RefCell;

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
        let num_pages = total.div_ceil(page_size);
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
        self.total.div_ceil(self.page_size)
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
