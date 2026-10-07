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
        widget: "DataGrid",
        sequence: "Ctrl+End <drop rows>",
        reason: "recompute_cache() does not clamp the selection/scroll after the rows shrink",
    },
    Known {
        layer: "keys",
        widget: "DateTimePicker",
        sequence: "'j'",
        reason: "month navigation keeps the selected day, so Jan 31 becomes Feb 31",
    },
    Known {
        layer: "keys",
        widget: "DateTimePicker(range)",
        sequence: "Up",
        reason: "week/month navigation moves the stored date past min/max (RangePicker clamps, DateTimePicker does not)",
    },
    Known {
        layer: "keys",
        widget: "Pagination",
        sequence: "<total 0> End",
        reason: "last()/goto() set page 0 when there are no pages (set_total keeps 1)",
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
    Known {
        layer: "keys",
        widget: "ScrollView",
        sequence: "Ctrl+End [80x24]",
        reason: "the offset is clamped only against the viewport of the last scroll call, not the one rendered",
    },
    Known {
        layer: "keys",
        widget: "Slider",
        sequence: "<set NaN>",
        reason: "set_value(NaN) stores NaN, outside the range for good",
    },
    Known {
        layer: "keys",
        widget: "TextArea",
        sequence: "<set content> Enter PageUp <add cursor below> Delete",
        reason: "edits move only the primary cursor; secondary cursors are left past the text",
    },
    Known {
        layer: "mouse",
        widget: "DataGrid",
        sequence: "Ctrl+End <drop rows>",
        reason: "recompute_cache() does not clamp the selection/scroll after the rows shrink",
    },
    Known {
        layer: "mouse",
        widget: "ScrollView",
        sequence: "ScrollDown@(start,start) ScrollDown@(start,start) ScrollDown@(start,start) ScrollDown@(start,start) [7x40]",
        reason: "the offset is clamped only against the viewport of the last scroll call, not the one rendered",
    },
];
