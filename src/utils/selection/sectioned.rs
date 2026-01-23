//! List selection with viewport scrolling - SectionedSelection implementation

use super::types::SectionedSelection;

impl Default for SectionedSelection {
    fn default() -> Self {
        Self::new()
    }
}

impl SectionedSelection {
    /// Create a new sectioned selection
    pub fn new() -> Self {
        Self {
            section: 0,
            item: 0,
            collapsed: std::collections::HashMap::new(),
        }
    }

    /// Move to next item (wraps to next section if at end)
    ///
    /// # Arguments
    ///
    /// * `section_sizes` - Number of items in each section
    ///
    /// # Example
    ///
    /// ```ignore
    /// sel.next(&[5, 3, 2]); // 3 sections with 5, 3, 2 items
    /// ```
    pub fn next(&mut self, section_sizes: &[usize]) {
        if section_sizes.is_empty() {
            return;
        }

        let current_section_size = section_sizes.get(self.section).copied().unwrap_or(0);

        if current_section_size == 0 || self.item >= current_section_size - 1 {
            // Move to next section
            self.section = (self.section + 1) % section_sizes.len();
            self.item = 0;
        } else {
            // Move to next item in current section
            self.item += 1;
        }
    }

    /// Move to previous item (wraps to previous section if at start)
    pub fn prev(&mut self, section_sizes: &[usize]) {
        if section_sizes.is_empty() {
            return;
        }

        if self.item == 0 {
            // Move to previous section
            if self.section == 0 {
                self.section = section_sizes.len() - 1;
            } else {
                self.section -= 1;
            }
            let prev_section_size = section_sizes.get(self.section).copied().unwrap_or(0);
            self.item = prev_section_size.saturating_sub(1);
        } else {
            // Move to previous item in current section
            self.item -= 1;
        }
    }

    /// Jump to next section
    pub fn next_section(&mut self, section_count: usize) {
        if section_count > 0 {
            self.section = (self.section + 1) % section_count;
            self.item = 0;
        }
    }

    /// Jump to previous section
    pub fn prev_section(&mut self, section_count: usize) {
        if section_count > 0 {
            if self.section == 0 {
                self.section = section_count - 1;
            } else {
                self.section -= 1;
            }
            self.item = 0;
        }
    }

    /// Toggle current section's collapsed state
    pub fn toggle_section(&mut self) {
        let collapsed = self.collapsed.entry(self.section).or_insert(false);
        *collapsed = !*collapsed;
    }

    /// Check if a section is collapsed
    pub fn is_section_collapsed(&self, section: usize) -> bool {
        self.collapsed.get(&section).copied().unwrap_or(false)
    }

    /// Collapse a specific section
    pub fn collapse_section(&mut self, section: usize) {
        self.collapsed.insert(section, true);
    }

    /// Expand a specific section
    pub fn expand_section(&mut self, section: usize) {
        self.collapsed.insert(section, false);
    }

    /// Expand all sections
    pub fn expand_all(&mut self) {
        self.collapsed.clear();
    }

    /// Collapse all sections
    pub fn collapse_all(&mut self, section_count: usize) {
        for i in 0..section_count {
            self.collapsed.insert(i, true);
        }
    }

    /// Get current selection as (section, item)
    pub fn get(&self) -> (usize, usize) {
        (self.section, self.item)
    }

    /// Set selection to specific section and item
    pub fn set(&mut self, section: usize, item: usize) {
        self.section = section;
        self.item = item;
    }

    /// Reset to first section, first item
    pub fn reset(&mut self) {
        self.section = 0;
        self.item = 0;
    }
}
