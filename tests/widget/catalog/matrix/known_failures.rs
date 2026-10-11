//! Known failures of the widget matrix - the ratchet.
//!
//! A layer fails on any failure not listed here, and on any listed failure
//! that no longer happens: fix a widget, then delete its entry. The list only
//! shrinks. Case keys are `"<area> <content> <plain|focus> <direct|app>"`.

pub use super::Kind;

/// A group of known failures of one widget, of one kind, in one layer.
pub struct Known {
    pub layer: &'static str,
    pub widget: &'static str,
    pub kind: Kind,
    /// One-line root-cause guess.
    pub reason: &'static str,
    pub cases: &'static [&'static str],
}

pub const KNOWN: &[Known] = &[];
