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

/// A content-sized widget that does not measure (#1100).
const fn no_measure(widget: &'static str) -> Known {
    Known {
        layer: "measure",
        widget,
        kind: Kind::NoMeasure,
        reason: "no measure (#1100)",
        cases: &["80x24 hello"],
    }
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
    no_measure("Avatar"),
    no_measure("GradientBox"),
    no_measure("Skeleton"),
    no_measure("Accordion"),
    no_measure("Collapsible"),
    no_measure("Resizable"),
    no_measure("Sidebar"),
    no_measure("Tabs"),
    no_measure("Autocomplete"),
    no_measure("ColorPicker"),
    no_measure("Combobox"),
    no_measure("NumberInput"),
    no_measure("SearchBar"),
    no_measure("Select"),
    no_measure("Stepper"),
    no_measure("Calendar"),
    no_measure("DateTimePicker"),
    no_measure("FormFieldWidget"),
    no_measure("MaskedInput"),
    no_measure("MultiSelect"),
    no_measure("Pagination"),
    no_measure("RangePicker"),
    no_measure("ThemePicker"),
    no_measure("RouterLink"),
    no_measure("Timer"),
    no_measure("Stopwatch"),
    no_measure("CandleChart"),
    no_measure("HeatMap"),
    no_measure("Waveline"),
    no_measure("MenuBar"),
    no_measure("Transition"),
    no_measure("TransitionGroup"),
    #[cfg(feature = "qrcode")]
    no_measure("QrCodeWidget"),
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
