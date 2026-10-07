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
];
