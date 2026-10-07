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

pub const KNOWN: &[Known] = &[
    // ── sizes ──
    Known {
        layer: "sizes",
        widget: "DevToolsEvents",
        kind: Kind::Panic,
        reason: "src/devtools/events/core.rs:289: row cursor `y += 2` overflows at the far-y offset [not covered by #767]",
        cases: &[
            "2x5@1,65530 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "DevToolsStyles",
        kind: Kind::Panic,
        reason: "src/devtools/style/view.rs:39: row cursor `y += 1` overflows at the far-y offset [not covered by #767]",
        cases: &[
            "2x5@1,65530 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "DevToolsTimeTravel",
        kind: Kind::Panic,
        reason: "src/devtools/time_travel/debugger/render.rs:31: row cursor `y += 2` overflows at the far-y offset [not covered by #767]",
        cases: &[
            "2x5@1,65530 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "Inspector",
        kind: Kind::OutOfArea,
        reason: "paints the inspected widget's highlight at its absolute bounds, outside the inspector's own area (overlay by design, but unclipped)",
        cases: &[
            "0x0 hello plain direct",
            "1x1 hello plain direct",
            "2x1 hello plain direct",
            "1x2 hello plain direct",
            "3x3 hello plain direct",
            "80x24 hello plain direct",
            "1000x1 hello plain direct",
            "1x1000 hello plain direct",
            "5x2@65530,1 hello plain direct",
        ],
    },
    // ── contents ──
    Known {
        layer: "contents",
        widget: "DevToolsProfiler",
        kind: Kind::OutOfArea,
        reason: "long text rows are not clipped to the panel's right edge [not covered by #767]",
        cases: &[
            "40x10 long plain direct",
            "40x10 items200 plain direct",
        ],
    },
    Known {
        layer: "contents",
        widget: "DevToolsStyles",
        kind: Kind::OutOfArea,
        reason: "long text rows are not clipped to the panel's right edge [not covered by #767]",
        cases: &[
            "40x10 long plain direct",
        ],
    },
    Known {
        layer: "contents",
        widget: "Inspector",
        kind: Kind::OutOfArea,
        reason: "paints the inspected widget's highlight at its absolute bounds, outside the inspector's own area (overlay by design, but unclipped)",
        cases: &[
            "40x10 empty plain direct",
            "40x10 hangul plain direct",
            "40x10 emoji plain direct",
            "40x10 combining plain direct",
            "40x10 rtl plain direct",
            "40x10 zero-width plain direct",
            "40x10 control plain direct",
            "40x10 long plain direct",
            "40x10 items0 plain direct",
            "40x10 items1 plain direct",
            "40x10 items200 plain direct",
        ],
    },
    // ── pairwise ──
    Known {
        layer: "pairwise",
        widget: "DevToolsEvents",
        kind: Kind::Panic,
        reason: "src/devtools/events/core.rs:324: event details truncated by bytes (`&details[..n]`) panic inside a multibyte char [not covered by #767]",
        cases: &[
            "80x24 emoji plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "DevToolsProfiler",
        kind: Kind::OutOfArea,
        reason: "long text rows are not clipped to the panel's right edge [not covered by #767]",
        cases: &[
            "80x24 long plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "DevToolsStyles",
        kind: Kind::OutOfArea,
        reason: "long text rows are not clipped to the panel's right edge [not covered by #767]",
        cases: &[
            "80x24 long plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "Inspector",
        kind: Kind::OutOfArea,
        reason: "paints the inspected widget's highlight at its absolute bounds, outside the inspector's own area (overlay by design, but unclipped)",
        cases: &[
            "0x0 empty plain direct",
            "1x1 long focus direct",
            "2x1 hangul plain direct",
            "1x2 emoji focus direct",
            "3x3 empty plain direct",
            "80x24 empty plain direct",
            "5x2@65530,1 empty plain direct",
            "5x2@65530,1 hangul focus direct",
            "0x0 emoji plain direct",
            "0x0 long plain direct",
            "1x1 empty plain direct",
            "1x1 hangul plain direct",
            "2x1 emoji plain direct",
            "2x1 long plain direct",
            "1x2 empty plain direct",
            "1x2 hangul plain direct",
            "3x3 emoji plain direct",
            "3x3 long plain direct",
            "80x24 emoji plain direct",
            "80x24 long plain direct",
            "5x2@65530,1 emoji plain direct",
            "5x2@65530,1 long plain direct",
        ],
    },
];
