//! CSS specificity calculation
//!
//! Specificity determines which CSS rule takes precedence when multiple rules match.
//! Higher specificity rules override lower specificity ones.

use std::cmp::Ordering;

/// CSS specificity (a, b, c)
/// - a: ID selectors count
/// - b: class, attribute, pseudo-class count
/// - c: type, pseudo-element count
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Specificity {
    /// Inline style flag (highest priority)
    pub inline: bool,
    /// !important flag
    pub important: bool,
    /// ID count
    pub ids: usize,
    /// Class/attribute/pseudo-class count
    pub classes: usize,
    /// Type/pseudo-element count
    pub types: usize,
    /// Source order (later = higher priority for equal specificity)
    pub order: usize,
}

impl Specificity {
    /// Create from specificity tuple
    pub fn new(ids: usize, classes: usize, types: usize, order: usize) -> Self {
        Self {
            inline: false,
            important: false,
            ids,
            classes,
            types,
            order,
        }
    }

    /// Create inline style specificity
    pub fn inline() -> Self {
        Self {
            inline: true,
            ..Default::default()
        }
    }

    /// Mark as !important
    pub fn important(mut self) -> Self {
        self.important = true;
        self
    }
}

impl Ord for Specificity {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. !important wins
        match (self.important, other.important) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }

        // 2. Inline styles win
        match (self.inline, other.inline) {
            (true, false) => return Ordering::Greater,
            (false, true) => return Ordering::Less,
            _ => {}
        }

        // 3. Compare (a, b, c)
        match self.ids.cmp(&other.ids) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self.classes.cmp(&other.classes) {
            Ordering::Equal => {}
            ord => return ord,
        }

        match self.types.cmp(&other.types) {
            Ordering::Equal => {}
            ord => return ord,
        }

        // 4. Source order (later wins)
        self.order.cmp(&other.order)
    }
}

impl PartialOrd for Specificity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
