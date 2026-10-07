//! Known failures of the fault-injection layers - the ratchet.
//!
//! A layer fails on any failure not listed here, and on any listed failure
//! that no longer happens: fix the library, then delete the entry. The list
//! only shrinks. Case keys are layer-specific; each layer documents its own.

/// A group of known failures of one layer sharing one root cause.
pub struct Known {
    pub layer: &'static str,
    /// One-line root cause.
    pub reason: &'static str,
    pub cases: &'static [&'static str],
}

pub const KNOWN: &[Known] = &[
    Known {
        layer: "output",
        reason: "CrosstermBackend marks mouse capture on only after the whole enabling write succeeds, so a failed init/enable_mouse that already sent ?1000h is never undone by restore",
        cases: &[
            "backend fail-write#3 bail",
            "backend fail-write#4 bail",
            "backend fail-write#5 bail",
            "backend fail-write#6 bail",
            "backend fail-write#47 bail",
            "backend fail-flush#1 bail",
            "backend fail-flush#7 bail",
            "backend fail-write#47 continue",
            "backend fail-flush#7 continue",
        ],
    },
    Known {
        layer: "events",
        reason: "Buffer::resize ignores the 16384-per-side / 10 M-cell limits Buffer::new enforces, so a huge Resize allocates billions of cells",
        cases: &["resize 65535x65535"],
    },
    Known {
        layer: "parsers",
        reason: "var() substitution is bounded in depth but not in size: a value that refers to itself several times expands exponentially",
        cases: &["declaration hang"],
    },
    Known {
        layer: "parsers",
        reason: "Terminal widget: a tab near the right edge wraps the cursor back to column 0 inside its own fill loop, which then never ends",
        cases: &["terminal hang"],
    },
    Known {
        layer: "parsers",
        reason: "SyntaxHighlighter slices the line with a char index as if it were a byte index",
        cases: &["markdown soup", "syntax corpus", "syntax soup", "syntax unicode"],
    },
    Known {
        layer: "parsers",
        reason: "Diagram node definitions slice between an opening and a closing bracket without checking that the closing one comes after",
        cases: &["mermaid soup"],
    },
    Known {
        layer: "parsers",
        reason: "parse_duration hands negative / non-finite / huge seconds to Duration::from_secs_f64, which panics",
        cases: &["transition soup"],
    },
];
