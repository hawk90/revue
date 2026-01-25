//! Filtering logic for Combobox

use super::super::Combobox;
use crate::utils::{fuzzy_match, FilterMode, FuzzyMatch};

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Filtering
    // ─────────────────────────────────────────────────────────────────────────

    /// Update filtered options based on input
    pub fn update_filter(&mut self) {
        if self.input.is_empty() {
            // Show all options when input is empty
            self.filtered = (0..self.options.len()).collect();
            self.selected_idx = 0;
            self.scroll_offset = 0;
            return;
        }

        let query = self.input.to_lowercase();
        let mut matches: Vec<(usize, i32)> = Vec::new();

        for (i, opt) in self.options.iter().enumerate() {
            let label_lower = opt.label.to_lowercase();

            let score = match self.filter_mode {
                FilterMode::Fuzzy => fuzzy_match(&self.input, &opt.label).map(|m| m.score),
                FilterMode::Prefix => {
                    if label_lower.starts_with(&query) {
                        Some(100 - (opt.label.len() as i32))
                    } else {
                        None
                    }
                }
                FilterMode::Exact => {
                    if label_lower == query {
                        Some(100)
                    } else {
                        None
                    }
                }
                FilterMode::Contains => {
                    if label_lower.contains(&query) {
                        // Score based on position (earlier = higher)
                        label_lower
                            .find(&query)
                            .map(|pos| 100 - (pos as i32) - (opt.label.len() as i32))
                    } else {
                        None
                    }
                }
                FilterMode::None => Some(0), // No filtering, include all with neutral score
            };

            if let Some(s) = score {
                matches.push((i, s));
            }
        }

        // Sort by score descending
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Get fuzzy match for an option (for highlighting)
    pub fn get_match(&self, option: &str) -> Option<FuzzyMatch> {
        if self.input.is_empty() || self.filter_mode != FilterMode::Fuzzy {
            None
        } else {
            fuzzy_match(&self.input, option)
        }
    }
}
