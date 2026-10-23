//! Which suggestions match the typed value

use crate::utils::{fuzzy_match, FilterMode};

use super::Autocomplete;

impl Autocomplete {
    /// Update filtered suggestions
    pub(super) fn update_filter(&mut self) {
        if self.get_value().chars().count() < self.min_chars {
            self.filtered.clear();
            self.dropdown_visible = false;
            return;
        }

        let query = self.get_value();
        self.filtered = self
            .suggestions
            .iter()
            .enumerate()
            .filter_map(|(idx, suggestion)| {
                let matches = match self.filter_mode {
                    FilterMode::Fuzzy => fuzzy_match(query, &suggestion.label).is_some(),
                    FilterMode::Prefix => suggestion
                        .label
                        .to_lowercase()
                        .starts_with(&query.to_lowercase()),
                    FilterMode::Contains => suggestion
                        .label
                        .to_lowercase()
                        .contains(&query.to_lowercase()),
                    FilterMode::Exact => suggestion.label.to_lowercase() == query.to_lowercase(),
                    FilterMode::None => true,
                };
                if matches {
                    Some(idx)
                } else {
                    None
                }
            })
            .take(self.max_suggestions)
            .collect();

        self.dropdown_visible = !self.filtered.is_empty();
        self.selection.set_len(self.filtered.len());
        self.selection.first();
    }
}
