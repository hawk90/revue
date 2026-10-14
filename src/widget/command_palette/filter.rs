//! The command list, the query and the filtered view of the commands.

use super::{command::Command, CommandPalette};
use crate::utils::fuzzy_match;

impl CommandPalette {
    /// Get current query
    pub fn get_query(&self) -> &str {
        &self.query
    }

    /// Set query
    pub fn set_query(&mut self, query: impl Into<String>) {
        self.query = query.into();
        self.update_filter();
    }

    /// Handle text input
    pub fn input(&mut self, ch: char) {
        self.query.push(ch);
        self.update_filter();
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        self.query.pop();
        self.update_filter();
    }

    /// Clear query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.update_filter();
    }

    /// Update filtered list
    pub(super) fn update_filter(&mut self) {
        self.filtered = self
            .commands
            .iter()
            .enumerate()
            .filter(|(_, cmd)| cmd.matches(&self.query))
            .map(|(idx, cmd)| (idx, cmd.match_score(&self.query)))
            .collect::<Vec<_>>()
            .into_iter()
            .map(|(idx, _score)| idx)
            .collect();

        // Sort by score
        self.filtered.sort_by(|&a, &b| {
            let score_a = self.commands[a].match_score(&self.query);
            let score_b = self.commands[b].match_score(&self.query);
            score_b.cmp(&score_a)
        });

        // Reset selection
        self.selection.set_len(self.filtered.len());
        self.selection.first();
    }

    /// Add command dynamically
    pub fn add_command(&mut self, command: Command) {
        self.commands.push(command);
        self.update_filter();
    }

    /// Remove command by ID
    pub fn remove_command(&mut self, id: &str) {
        self.commands.retain(|c| c.id != id);
        self.update_filter();
    }

    /// Clear all commands
    pub fn clear_commands(&mut self) {
        self.commands.clear();
        self.filtered.clear();
    }

    /// Mark command as recent
    pub fn mark_recent(&mut self, id: &str) {
        if let Some(cmd) = self.commands.iter_mut().find(|c| c.id == id) {
            cmd.recent = true;
        }
    }

    /// Highlight matching characters in label using fuzzy match indices
    pub fn highlight_match(&self, label: &str) -> Vec<(char, bool)> {
        if self.query.is_empty() {
            return label.chars().map(|c| (c, false)).collect();
        }

        // Get fuzzy match to find matched character indices
        if let Some(m) = fuzzy_match(&self.query, label) {
            label
                .chars()
                .enumerate()
                .map(|(i, c)| (c, m.indices.contains(&i)))
                .collect()
        } else {
            label.chars().map(|c| (c, false)).collect()
        }
    }
}
