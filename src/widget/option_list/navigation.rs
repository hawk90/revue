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

    /// Move highlight to previous option
    pub fn highlight_previous(&mut self) {
        if self.highlighted > 0 {
            self.highlighted -= 1;

            // Skip disabled items
            while self.highlighted > 0 {
                if let Some(item) = self.get_highlighted() {
                    if !item.disabled {
                        break;
                    }
                }
                self.highlighted -= 1;
            }
        }
    }

    /// Move highlight to next option
    pub fn highlight_next(&mut self) {
        let max = self.option_count().saturating_sub(1);
        if self.highlighted < max {
            self.highlighted += 1;

            // Skip disabled items
            while self.highlighted < max {
                if let Some(item) = self.get_highlighted() {
                    if !item.disabled {
                        break;
                    }
                }
                self.highlighted += 1;
            }
        }
    }

    /// Move to first option
    pub fn highlight_first(&mut self) {
        self.highlighted = 0;

        // Skip disabled items
        while self.highlighted < self.option_count().saturating_sub(1) {
            if let Some(item) = self.get_highlighted() {
                if !item.disabled {
                    break;
                }
            }
            self.highlighted += 1;
        }
    }

    /// Move to last option
    pub fn highlight_last(&mut self) {
        self.highlighted = self.option_count().saturating_sub(1);

        // Skip disabled items
        while self.highlighted > 0 {
            if let Some(item) = self.get_highlighted() {
                if !item.disabled {
                    break;
                }
            }
            self.highlighted -= 1;
        }
    }
}
