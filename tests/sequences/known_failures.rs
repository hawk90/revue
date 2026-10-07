//! Known failures of the event-sequence layers - the ratchet.
//!
//! A layer fails on any widget/sequence not listed here, and on any listed
//! one that no longer fails: fix the widget, then delete its entry. The list
//! only shrinks. `sequence` is the shrunk sequence exactly as the report
//! prints it (steps joined with a space; see `describe_all`).

/// One known failure: a widget and the minimal sequence that breaks it.
pub struct Known {
    pub layer: &'static str,
    pub widget: &'static str,
    pub sequence: &'static str,
    /// One-line root cause.
    pub reason: &'static str,
}

pub const KNOWN: &[Known] = &[
    Known {
        layer: "keys",
        widget: "DateTimePicker(range)",
        sequence: "Up",
        reason: "week/month navigation moves the stored date past min/max (RangePicker clamps, DateTimePicker does not)",
    },
    Known {
        layer: "keys",
        widget: "RangePicker",
        sequence: "'j'",
        reason: "month navigation keeps the stored day, so Jan 31 becomes Feb 31",
    },
    Known {
        layer: "keys",
        widget: "RangePicker(order)",
        sequence: "BackTab <start after end> <start after end> BackTab Up",
        reason: "navigation moves the stored start/end with the cursor and only a selection swaps them, so start can pass end - design decision",
    },
];
