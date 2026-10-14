//! Moving between pages and asking where the pagination is

use super::Pagination;

impl Pagination {
    /// Go to next page
    pub fn next_page(&mut self) -> bool {
        if self.current < self.total {
            self.current += 1;
            true
        } else {
            false
        }
    }

    /// Go to previous page
    pub fn prev_page(&mut self) -> bool {
        if self.current > 1 {
            self.current -= 1;
            true
        } else {
            false
        }
    }

    /// Go to first page
    pub fn first(&mut self) {
        self.current = 1;
    }

    /// Go to last page
    pub fn last(&mut self) {
        // Pages count from 1; with no pages, stay on page 1 like `new(0)`
        self.current = self.total.max(1);
    }

    /// Go to specific page
    pub fn goto(&mut self, page: u16) {
        self.current = page.min(self.total).max(1);
    }

    /// Get current page
    pub fn get_current(&self) -> u16 {
        self.current
    }

    /// Get total pages
    pub fn get_total(&self) -> u16 {
        self.total
    }

    /// Set total pages
    pub fn set_total(&mut self, total: u16) {
        self.total = total;
        if self.current > total {
            self.current = total.max(1);
        }
    }

    /// Check if on first page
    pub fn is_first(&self) -> bool {
        self.current == 1
    }

    /// Check if on last page
    pub fn is_last(&self) -> bool {
        self.current == self.total
    }
}
