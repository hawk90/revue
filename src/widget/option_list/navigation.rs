//! Querying, selecting and moving the highlight between options

use super::{OptionEntry, OptionItem, OptionList};
use crate::event::Key;

impl OptionList {
    /// Handle a key, returning whether it is one the list handles.
    ///
    /// The keys are those of [`List`](crate::widget::List), plus Enter and
    /// Space to select: `Up`/`k` and `Down`/`j` move the highlight to the
    /// option before or after it, `Home` and `End` to the first or last
    /// option, and `Enter` or `Space` select the highlighted option.
    pub fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Up | Key::Char('k') => self.highlight_previous(),
            Key::Down | Key::Char('j') => self.highlight_next(),
            Key::Home => self.highlight_first(),
            Key::End => self.highlight_last(),
            Key::Enter | Key::Char(' ') => {
                self.select_highlighted();
            }
            _ => return false,
        }
        true
    }

    /// Get option count (excluding separators and groups)
    pub fn option_count(&self) -> usize {
        self.entries
            .iter()
            .filter(|e| matches!(e, OptionEntry::Option(_)))
            .count()
    }

    /// Get highlighted option
    pub fn get_highlighted(&self) -> Option<&OptionItem> {
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == self.highlighted {
                    return Some(item);
                }
                option_idx += 1;
            }
        }
        None
    }

    /// Get selected option
    pub fn get_selected(&self) -> Option<&OptionItem> {
        let selected = self.selected?;
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == selected {
                    return Some(item);
                }
                option_idx += 1;
            }
        }
        None
    }

    /// Get selected value
    pub fn get_selected_value(&self) -> Option<&str> {
        self.get_selected()
            .and_then(|item| item.value.as_deref().or(Some(&item.text)))
    }

    /// Select highlighted option
    pub fn select_highlighted(&mut self) -> bool {
        if let Some(item) = self.get_highlighted() {
            if !item.disabled {
                self.selected = Some(self.highlighted);
                return true;
            }
        }
        false
    }

    /// Select by index
    pub fn select(&mut self, index: usize) {
        let mut option_idx = 0;
        for entry in &self.entries {
            if let OptionEntry::Option(item) = entry {
                if option_idx == index && !item.disabled {
                    self.selected = Some(index);
                    self.highlighted = index;
                    return;
                }
                option_idx += 1;
            }
        }
    }

    /// Clear selection
    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    /// The options in order, by option index: whether each is enabled
    fn enabled(&self) -> Vec<bool> {
        self.entries
            .iter()
            .filter_map(|entry| match entry {
                OptionEntry::Option(item) => Some(!item.disabled),
                _ => None,
            })
            .collect()
    }

    /// Move the highlight to the first enabled option among `candidates`
    /// (option indices, in the order to try). With none it stays: a
    /// disabled option is never highlighted by moving (#1553).
    fn highlight_first_of(&mut self, mut candidates: impl Iterator<Item = usize>) {
        let enabled = self.enabled();
        if let Some(index) = candidates.find(|&i| enabled.get(i).copied().unwrap_or(false)) {
            self.highlighted = index;
        }
    }

    /// Move the highlight off a disabled option onto the first enabled one,
    /// as a list starts with its first enabled option highlighted
    pub(super) fn settle_highlight(&mut self) {
        if self.get_highlighted().is_some_and(|item| item.disabled) {
            self.highlight_first();
        }
    }

    /// Move highlight to previous option, skipping disabled ones
    pub fn highlight_previous(&mut self) {
        self.highlight_first_of((0..self.highlighted).rev());
    }

    /// Move highlight to next option, skipping disabled ones
    pub fn highlight_next(&mut self) {
        let from = self.highlighted.saturating_add(1);
        self.highlight_first_of(from..self.option_count());
    }

    /// Move to the first enabled option
    pub fn highlight_first(&mut self) {
        self.highlight_first_of(0..self.option_count());
    }

    /// Move to the last enabled option
    pub fn highlight_last(&mut self) {
        self.highlight_first_of((0..self.option_count()).rev());
    }
}
