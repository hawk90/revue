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
        widget: "CommandPalette",
        kind: Kind::Panic,
        reason: "src/widget/command_palette/view.rs:72: title clipping `x + width - 2` underflows when the palette is narrower than 2 columns",
        cases: &[
            "1x1000 hello plain direct",
        ],
    },
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
            "3x3 hello plain direct",
            "80x24 hello plain direct",
            "1000x1 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "Inspector",
        kind: Kind::Panic,
        reason: "src/core/app/inspector.rs:268: title `title_x + i` overflows near u16::MAX",
        cases: &[
            "5x2@65530,1 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "Inspector",
        kind: Kind::Panic,
        reason: "src/core/app/inspector.rs:278: `panel_width - 3` underflows when the panel is narrower than 3 columns",
        cases: &[
            "0x0 hello plain direct",
            "1x1 hello plain direct",
            "2x1 hello plain direct",
            "1x2 hello plain direct",
            "1x1000 hello plain direct",
            "2x5@1,65530 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "RichLog",
        kind: Kind::Panic,
        reason: "src/widget/display/richlog/render.rs:141: scrollbar math `area.height as usize - 1` underflows at height 0",
        cases: &[
            "0x0 hello plain direct",
        ],
    },
    Known {
        layer: "sizes",
        widget: "Splitter",
        kind: Kind::Panic,
        reason: "src/widget/layout/splitter/mod.rs:151: pane_areas: `area.x + offset` overflows near u16::MAX",
        cases: &[
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
    Known {
        layer: "contents",
        widget: "LogViewer",
        kind: Kind::Panic,
        reason: "src/widget/data/log_viewer/parser.rs:292: timestamp sniffing slices `&s[..8]` by bytes and panics inside a multibyte char (in `load`, before any render) [not covered by #767]",
        cases: &[
            "40x10 hangul plain direct",
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
            "3x3 empty plain direct",
            "80x24 empty plain direct",
            "3x3 emoji plain direct",
            "3x3 long plain direct",
            "80x24 emoji plain direct",
            "80x24 long plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "Inspector",
        kind: Kind::Panic,
        reason: "src/core/app/inspector.rs:268: title `title_x + i` overflows near u16::MAX",
        cases: &[
            "5x2@65530,1 empty plain direct",
            "5x2@65530,1 hangul focus direct",
            "5x2@65530,1 emoji plain direct",
            "5x2@65530,1 long plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "Inspector",
        kind: Kind::Panic,
        reason: "src/core/app/inspector.rs:278: `panel_width - 3` underflows when the panel is narrower than 3 columns",
        cases: &[
            "0x0 empty plain direct",
            "0x0 hangul focus app",
            "1x1 emoji plain app",
            "1x1 long focus direct",
            "2x1 empty focus app",
            "2x1 hangul plain direct",
            "1x2 emoji focus direct",
            "1x2 long plain app",
            "0x0 emoji plain direct",
            "0x0 long plain direct",
            "1x1 empty plain direct",
            "1x1 hangul plain direct",
            "2x1 emoji plain direct",
            "2x1 long plain direct",
            "1x2 empty plain direct",
            "1x2 hangul plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "LogViewer",
        kind: Kind::Panic,
        reason: "src/widget/data/log_viewer/parser.rs:292: timestamp sniffing slices `&s[..8]` by bytes and panics inside a multibyte char (in `load`, before any render) [not covered by #767]",
        cases: &[
            "0x0 hangul focus app",
            "2x1 hangul plain direct",
            "3x3 hangul focus app",
            "80x24 hangul focus app",
            "5x2@65530,1 hangul focus direct",
            "1x1 hangul plain direct",
            "1x2 hangul plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "RichLog",
        kind: Kind::Panic,
        reason: "src/widget/display/richlog/render.rs:141: scrollbar math `area.height as usize - 1` underflows at height 0",
        cases: &[
            "0x0 empty plain direct",
            "0x0 emoji plain direct",
            "0x0 long plain direct",
        ],
    },
    Known {
        layer: "pairwise",
        widget: "Splitter",
        kind: Kind::Panic,
        reason: "src/widget/layout/splitter/mod.rs:151: pane_areas: `area.x + offset` overflows near u16::MAX",
        cases: &[
            "5x2@65530,1 empty plain direct",
            "5x2@65530,1 hangul focus direct",
            "5x2@65530,1 emoji plain direct",
            "5x2@65530,1 long plain direct",
        ],
    },
];
