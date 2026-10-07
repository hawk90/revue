//! Shared machinery for the widget matrix: the inputs (areas, contents,
//! focus, render paths), the two invariants every case is checked against,
//! and the known-failures ratchet.
//!
//! The invariants:
//!
//! 1. rendering does not panic;
//! 2. on the direct path, nothing is written outside the area the widget was
//!    given (the buffer is pre-filled with a sentinel cell and every cell
//!    outside the area must still be that sentinel afterwards).
//!
//! The ratchet: a layer fails if it sees a failure that is not listed in
//! [`known_failures::KNOWN`], **or** if a listed failure no longer happens.
//! The list can only shrink.

pub mod catalog;
pub mod contents;
pub mod known_failures;
pub mod pairwise;
pub mod sentinel;
pub mod sizes;

use std::collections::BTreeSet;
use std::fmt;

use revue::layout::Rect;
use revue::testing::PipelineHarness;
use revue::widget::View;

use catalog::Entry;
use sentinel::{catch, render_checked};

// ─── Inputs ─────────────────────────────────────────────────────────────────

/// What a factory feeds the widget: a text and an item list derived from it.
pub struct Content {
    /// Short stable label used in case keys.
    pub label: &'static str,
    /// Text for widgets that take a string.
    pub text: String,
    /// Items for list-like widgets (options, rows, data points, ...).
    pub items: Vec<String>,
}

impl Content {
    /// A text content whose items are three copies of the text.
    pub fn text(label: &'static str, text: impl Into<String>) -> Self {
        let text = text.into();
        let items = vec![text.clone(), format!("{text}2"), format!("{text}3")];
        Self { label, text, items }
    }

    /// An ordinary text with `n` ordinary items.
    pub fn items(label: &'static str, n: usize) -> Self {
        Self {
            label,
            text: "Hello".to_string(),
            items: (0..n).map(|i| format!("item {i}")).collect(),
        }
    }

    /// The everyday content: "Hello" with a few items.
    pub fn ordinary() -> Self {
        Self::items("hello", 3)
    }
}

/// Where the widget is rendered: its area and the buffer around it.
#[derive(Clone, Copy)]
pub struct Placement {
    pub label: &'static str,
    pub buffer: (u16, u16),
    pub area: Rect,
}

impl Placement {
    /// A `w`×`h` area with a 2-cell margin left/right and 1 row above/below,
    /// so a write past any edge lands in the sentinel margin.
    pub fn sized(label: &'static str, w: u16, h: u16) -> Self {
        Self {
            label,
            buffer: (w + 4, h + 2),
            area: Rect::new(2, 1, w, h),
        }
    }

    /// A 5×2 area pressed against `u16::MAX` on the x axis.
    ///
    /// `Buffer` caps a side at 16384 cells, so the area lies *outside* a small
    /// buffer: nothing should land in the buffer at all, and a coordinate that
    /// wraps around on overflow shows up as a write near the origin.
    pub fn far_x() -> Self {
        Self {
            label: "5x2@65530,1",
            buffer: (16, 4),
            area: Rect::new(u16::MAX - 5, 1, 5, 2),
        }
    }

    /// A 2×5 area pressed against `u16::MAX` on the y axis (see [`far_x`](Self::far_x)).
    pub fn far_y() -> Self {
        Self {
            label: "2x5@1,65530",
            buffer: (4, 16),
            area: Rect::new(1, u16::MAX - 5, 2, 5),
        }
    }

    fn is_offset(&self) -> bool {
        self.area.x > 1000 || self.area.y > 1000
    }
}

/// Which render path drew the widget.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Path {
    /// `RenderContext` over a bare buffer + `View::render`.
    Direct,
    /// The real `App::draw` via `PipelineHarness` with the 3.0 defaults
    /// (`dom_from_render` and `css_layout` on).
    App,
}

/// One case of one widget.
pub struct Case<'a> {
    pub placement: Placement,
    pub content: &'a Content,
    pub focused: bool,
    pub path: Path,
}

impl Case<'_> {
    /// Stable key used in the known-failures list.
    pub fn key(&self) -> String {
        format!(
            "{} {} {} {}",
            self.placement.label,
            self.content.label,
            if self.focused { "focus" } else { "plain" },
            match self.path {
                Path::Direct => "direct",
                Path::App => "app",
            }
        )
    }
}

// ─── Running a case ─────────────────────────────────────────────────────────

/// How a case failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Panic,
    OutOfArea,
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Kind::Panic => "panic",
            Kind::OutOfArea => "out-of-area",
        })
    }
}

/// One observed failure.
pub struct Failure {
    pub widget: &'static str,
    pub case: String,
    pub kind: Kind,
    pub detail: String,
}

fn render_direct(view: &dyn View, case: &Case) -> Result<Option<String>, String> {
    render_checked(
        view,
        case.placement.buffer,
        case.placement.area,
        case.focused,
    )
}

fn render_app(view: Box<dyn View>, case: &Case) -> Result<(), String> {
    let area = case.placement.area;
    catch(|| {
        let mut harness = PipelineHarness::new(area.width, area.height)
            .dom_from_render(true)
            .css_layout(true);
        harness.draw(&view);
    })
}

/// Build and render one case; `None` if both invariants hold.
pub fn run_case(entry: &Entry, case: &Case) -> Option<Failure> {
    let fail = |kind, detail| {
        Some(Failure {
            widget: entry.name,
            case: case.key(),
            kind,
            detail,
        })
    };
    let view = match catch(|| (entry.build)(case.content, case.focused)) {
        Ok(view) => view,
        Err(msg) => return fail(Kind::Panic, format!("while building: {msg}")),
    };
    match case.path {
        Path::Direct => match render_direct(&*view, case) {
            Ok(None) => None,
            Ok(Some(detail)) => fail(Kind::OutOfArea, detail),
            Err(msg) => fail(Kind::Panic, msg),
        },
        Path::App => {
            assert!(
                !case.placement.is_offset(),
                "the app path cannot place an area at an offset"
            );
            render_app(view, case)
                .err()
                .and_then(|m| fail(Kind::Panic, m))
        }
    }
}

// ─── The ratchet ────────────────────────────────────────────────────────────

/// Compare what a layer observed with the known-failures list for that layer
/// and panic with a readable report on any difference.
pub fn ratchet(layer: &str, cases_run: usize, failures: Vec<Failure>) {
    let known: BTreeSet<(&str, String, Kind)> = known_failures::KNOWN
        .iter()
        .filter(|k| k.layer == layer)
        .flat_map(|k| {
            k.cases
                .iter()
                .map(move |c| (k.widget, c.to_string(), k.kind))
        })
        .collect();
    let seen: BTreeSet<(&str, String, Kind)> = failures
        .iter()
        .map(|f| (f.widget, f.case.clone(), f.kind))
        .collect();

    let new: Vec<&Failure> = failures
        .iter()
        .filter(|f| !known.contains(&(f.widget, f.case.clone(), f.kind)))
        .collect();
    let fixed: Vec<_> = known.difference(&seen).collect();

    // `WIDGET_MATRIX_DUMP=<dir>` writes every failure of the layer as TSV,
    // known or not - handy when triaging.
    if let Ok(dir) = std::env::var("WIDGET_MATRIX_DUMP") {
        let tsv: String = failures
            .iter()
            .map(|f| {
                format!(
                    "{layer}\t{}\t{}\t{}\t{}\n",
                    f.widget, f.kind, f.case, f.detail
                )
            })
            .collect();
        let _ = std::fs::write(format!("{dir}/{layer}.tsv"), tsv);
    }

    eprintln!(
        "widget matrix [{layer}]: {cases_run} cases, {} failing ({} known)",
        failures.len(),
        failures.len() - new.len()
    );
    if new.is_empty() && fixed.is_empty() {
        return;
    }

    let mut report = String::new();
    if !new.is_empty() {
        report.push_str(&format!(
            "\n{} NEW failure(s) in layer `{layer}` (fix the widget, or list them in tests/matrix/known_failures.rs):\n\n",
            new.len()
        ));
        report.push_str(&format!(
            "  {:<22} {:<12} {:<32} {}\n",
            "widget", "kind", "case", "detail"
        ));
        for f in &new {
            report.push_str(&format!(
                "  {:<22} {:<12} {:<32} {}\n",
                f.widget,
                f.kind.to_string(),
                f.case,
                f.detail
            ));
        }
        report.push_str("\nSuggested entries:\n");
        let mut groups: Vec<(&str, Kind, Vec<&str>)> = Vec::new();
        for f in &new {
            match groups
                .iter_mut()
                .find(|(w, k, _)| *w == f.widget && *k == f.kind)
            {
                Some(g) => g.2.push(&f.case),
                None => groups.push((f.widget, f.kind, vec![&f.case])),
            }
        }
        for (widget, kind, cases) in groups {
            report.push_str(&format!(
                "    Known {{ layer: {layer:?}, widget: {widget:?}, kind: Kind::{kind:?}, reason: \"TODO\", cases: &{cases:?} }},\n"
            ));
        }
    }
    if !fixed.is_empty() {
        report.push_str(&format!(
            "\n{} known failure(s) in layer `{layer}` no longer happen - remove them from tests/matrix/known_failures.rs:\n\n",
            fixed.len()
        ));
        for (widget, case, kind) in fixed {
            let reason = known_failures::KNOWN
                .iter()
                .find(|k| {
                    k.layer == layer
                        && k.widget == *widget
                        && k.kind == *kind
                        && k.cases.contains(&case.as_str())
                })
                .map_or("", |k| k.reason);
            report.push_str(&format!(
                "  {widget:<22} {:<12} {case:<32} ({reason})\n",
                kind.to_string()
            ));
        }
    }
    panic!("{report}");
}

/// Run every case for every catalog entry and apply the ratchet.
pub fn run_layer(layer: &str, cases: &[Case<'_>]) {
    let catalog = catalog::catalog();
    let mut failures = Vec::new();
    let mut count = 0;
    for entry in &catalog {
        for case in cases {
            count += 1;
            failures.extend(run_case(entry, case));
        }
    }
    ratchet(layer, count, failures);
}
