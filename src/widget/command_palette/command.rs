//! Command Palette widget for quick command access
//!
//! Provides a searchable command interface similar to VSCode's Ctrl+P
//! or Sublime Text's Command Palette.

use crate::utils::{fuzzy_match, FuzzyMatch};

/// Command item
#[derive(Clone, Debug)]
pub struct Command {
    /// Command ID
    pub id: String,
    /// Display label
    pub label: String,
    /// Description
    pub description: Option<String>,
    /// Keyboard shortcut
    pub shortcut: Option<String>,
    /// Category/group
    pub category: Option<String>,
    /// Icon character
    pub icon: Option<char>,
    /// Is recently used
    pub recent: bool,
    /// Is pinned
    pub pinned: bool,
}

impl Command {
    /// Create a new command
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            shortcut: None,
            category: None,
            icon: None,
            recent: false,
            pinned: false,
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set shortcut
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set category
    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Mark as recent
    pub fn recent(mut self) -> Self {
        self.recent = true;
        self
    }

    /// Mark as pinned
    pub fn pinned(mut self) -> Self {
        self.pinned = true;
        self
    }

    /// Check if command matches query using fuzzy matching
    pub fn matches(&self, query: &str) -> bool {
        self.fuzzy_match(query).is_some()
    }

    /// Get fuzzy match result for label
    pub fn fuzzy_match(&self, query: &str) -> Option<FuzzyMatch> {
        if query.is_empty() {
            return Some(FuzzyMatch::new(0, vec![]));
        }

        // Try fuzzy match on label
        if let Some(m) = fuzzy_match(query, &self.label) {
            return Some(m);
        }

        // Try match on description
        if let Some(ref desc) = self.description {
            if let Some(m) = fuzzy_match(query, desc) {
                return Some(m);
            }
        }

        // Try match on category
        if let Some(ref cat) = self.category {
            if let Some(m) = fuzzy_match(query, cat) {
                return Some(m);
            }
        }

        None
    }

    /// Get match score (higher = better match)
    pub fn match_score(&self, query: &str) -> i32 {
        if query.is_empty() {
            return if self.pinned {
                100
            } else if self.recent {
                50
            } else {
                0
            };
        }

        let mut score = 0;

        // Use fuzzy match score
        if let Some(m) = fuzzy_match(query, &self.label) {
            score += m.score;
        }

        // Bonus for pinned/recent
        if self.pinned {
            score += 50;
        }
        if self.recent {
            score += 25;
        }

        score
    }
}
