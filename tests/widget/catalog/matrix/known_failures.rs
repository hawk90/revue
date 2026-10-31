//! Known failures of the widget matrix - the ratchet.
//!
//! A layer fails on any failure not listed here, and on any listed failure
//! that no longer happens: fix a widget, then delete its entry. The list only
//! shrinks. Case keys are `"<area> <content> <plain|focus> <direct|app>"`,
//! and `"<offer> <content>"` in the `measure` layer.

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

/// A text drawn through `RichText` that its measured size clips.
const fn clipped(
    widget: &'static str,
    reason: &'static str,
    cases: &'static [&'static str],
) -> Known {
    Known {
        layer: "measure",
        widget,
        kind: Kind::Clipped,
        reason,
        cases,
    }
}

const ZERO_WIDTH: &str = "a zero-width char lands in the next cell (#1330)";
const NEWLINES: &str = "a text with newlines measures as one row (#1068)";

pub const KNOWN: &[Known] = &[
    clipped("HeatMap", ZERO_WIDTH, &["80x24 combining"]),
    clipped("Stopwatch", ZERO_WIDTH, &["80x24 combining"]),
    clipped("Link", ZERO_WIDTH, &["80x24 combining"]),
    clipped("Link", NEWLINES, &["80x24 control"]),
    clipped("RichText", ZERO_WIDTH, &["80x24 combining"]),
    clipped("Text", ZERO_WIDTH, &["80x24 combining"]),
    clipped("Text", NEWLINES, &["80x24 control"]),
    clipped("Border", NEWLINES, &["80x24 control"]),
    clipped("Card", NEWLINES, &["80x24 control"]),
    clipped("HStack", NEWLINES, &["80x24 control"]),
    clipped("ErrorBoundary", ZERO_WIDTH, &["80x24 combining"]),
    clipped("ErrorBoundary", NEWLINES, &["80x24 control"]),
    clipped("ZenMode", ZERO_WIDTH, &["80x24 combining"]),
    clipped("ZenMode", NEWLINES, &["80x24 control"]),
];
