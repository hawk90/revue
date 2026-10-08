//! Filtering logic for Combobox

use super::super::Combobox;
use crate::utils::{fuzzy_match, FilterMode, FuzzyMatch};

/// A row of the open dropdown.
pub(super) enum DisplayRow<'a> {
    /// The name of the group whose options follow
    Header(&'a str),
    /// The option at this position in `filtered`
    Option(usize),
}

impl Combobox {
    // ─────────────────────────────────────────────────────────────────────────
    // Filtering
    // ─────────────────────────────────────────────────────────────────────────

    /// Update filtered options based on input
    pub fn update_filter(&mut self) {
        if self.input.is_empty() {
            // Show all options when input is empty
            self.filtered = (0..self.options.len()).collect();
            self.group_filtered();
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
        matches.sort_by_key(|b| std::cmp::Reverse(b.1));

        self.filtered = matches.into_iter().map(|(i, _)| i).collect();
        self.group_filtered();
        self.selected_idx = 0;
        self.scroll_offset = 0;
    }

    /// Keep each group's options together: ungrouped options first, then
    /// the groups in the order they first appear. The sort is stable, so
    /// within a group the options keep their filter order.
    fn group_filtered(&mut self) {
        let mut groups: Vec<&str> = Vec::new();
        for opt in &self.options {
            if let Some(g) = opt.group.as_deref() {
                if !groups.contains(&g) {
                    groups.push(g);
                }
            }
        }
        if groups.is_empty() {
            return;
        }
        let rank = |i: &usize| match self.options[*i].group.as_deref() {
            None => 0,
            Some(g) => 1 + groups.iter().position(|x| *x == g).unwrap_or(0),
        };
        let mut filtered = std::mem::take(&mut self.filtered);
        filtered.sort_by_key(rank);
        self.filtered = filtered;
    }

    /// The dropdown's rows: a header before the first option of each group,
    /// then the options, as positions in `filtered`.
    pub(super) fn display_rows(&self) -> Vec<DisplayRow<'_>> {
        let mut rows = Vec::with_capacity(self.filtered.len());
        let mut current: Option<&str> = None;
        for (pos, &i) in self.filtered.iter().enumerate() {
            let group = self.options[i].group.as_deref();
            if group.is_some() && group != current {
                rows.push(DisplayRow::Header(group.unwrap_or_default()));
            }
            current = group;
            rows.push(DisplayRow::Option(pos));
        }
        rows
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
