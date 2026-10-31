//! Layer 2 - `measure` against `render`.
//!
//! Every widget with one ordinary content and the edge contents, unfocused,
//! offered 80×24. Two rules:
//!
//! 1. a widget whose size follows from its content answers `measure` with a
//!    size (`None` means "I fill whatever I am given", LAY-006); the widgets
//!    that fill by design are listed in [`FILLS`] with the reason. Checked
//!    with the ordinary content only;
//! 2. the measured size fits the offer, and rendering at the measured size
//!    paints the same symbols, in reading order, as rendering at the offer -
//!    a content-sized stack that gives the widget its measured size clips
//!    nothing.
//!
//! Failures go through the same known-failures ratchet as the other layers.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{OverlayQueue, RenderContext, View};

use super::catalog::{catalog, Entry};
use super::sentinel::catch;
use super::{contents::contents, ratchet, Content, Failure, Kind};

/// The size every widget is offered.
const OFFER: (u16, u16) = (80, 24);

// Why a widget fills, for `FILLS`.
const SPLITS: &str = "divides the area it is given among its panes or tracks";
const LAYERS: &str = "places or stacks its children within the area it is given";
const PLOTS: &str = "a chart or drawing surface scaled to the area it is given";
const SCROLLS: &str = "scrolls its content inside the area it is given";
const EDITORS: &str = "an editor or terminal that fills the area it is given";
const PANELS: &str = "a full panel or screen: it lays itself out in the area it is given";
const OVERLAYS: &str = "an overlay that places itself within the area it is given";

/// Widgets for which `measure` answering `None` is right: they fill the area
/// they are given. The test fails if one of them starts to answer a size, so
/// the list stays true.
pub const FILLS: &[(&str, &str)] = &[
    ("HSplit", SPLITS),
    ("VSplit", SPLITS),
    ("Splitter", SPLITS),
    ("ScrollView", SPLITS),
    ("Grid", SPLITS),
    ("Positioned", LAYERS),
    ("Layers", LAYERS),
    ("BarChart", PLOTS),
    ("BarChartHorizontal", PLOTS),
    ("BoxPlot", PLOTS),
    ("Chart", PLOTS),
    ("Histogram", PLOTS),
    ("PieChart", PLOTS),
    ("ScatterChart", PLOTS),
    ("Streamline", PLOTS),
    ("TimeSeries", PLOTS),
    ("Waveline", PLOTS),
    ("Canvas", PLOTS),
    ("BrailleCanvas", PLOTS),
    ("Diagram", PLOTS),
    #[cfg(feature = "image")]
    ("Image", PLOTS),
    ("List", SCROLLS),
    ("VirtualList", SCROLLS),
    ("Table", SCROLLS),
    ("DataGrid", SCROLLS),
    ("Tree", SCROLLS),
    ("FileTree", SCROLLS),
    ("JsonViewer", SCROLLS),
    ("LogViewer", SCROLLS),
    ("CsvViewer", SCROLLS),
    ("RichLog", SCROLLS),
    ("OptionList", SCROLLS),
    ("SelectionList", SCROLLS),
    ("SortableList", SCROLLS),
    ("Timeline", SCROLLS),
    #[cfg(feature = "markdown")]
    ("Markdown", SCROLLS),
    ("AiStream", SCROLLS),
    #[cfg(feature = "diff")]
    ("DiffViewer", SCROLLS),
    ("HttpClient", SCROLLS),
    #[cfg(feature = "sysinfo")]
    ("ProcessMonitor", SCROLLS),
    ("TextArea", EDITORS),
    ("CodeEditor", EDITORS),
    ("RichTextEditor", EDITORS),
    ("Terminal", EDITORS),
    ("Presentation", PANELS),
    #[cfg(feature = "markdown")]
    ("MarkdownPresentation", PANELS),
    ("Inspector", PANELS),
    ("DevToolsInspector", PANELS),
    ("DevToolsState", PANELS),
    ("DevToolsStyles", PANELS),
    ("DevToolsEvents", PANELS),
    ("DevToolsProfiler", PANELS),
    ("DevToolsTimeTravel", PANELS),
    ("DebugOverlay", PANELS),
    ("FilePicker", PANELS),
    ("DropZone", PANELS),
    ("Form", PANELS),
    ("ScreenStack", PANELS),
    ("DeclarativeRouter", PANELS),
    ("Modal", OVERLAYS),
    ("Popover", OVERLAYS),
    ("Tooltip", OVERLAYS),
    ("ContextMenu", OVERLAYS),
    ("CommandPalette", OVERLAYS),
    ("Toast", OVERLAYS),
    ("ToastQueue", OVERLAYS),
    ("NotificationCenter", OVERLAYS),
];

/// Whether a symbol is content rather than blank or frame: a border or rule
/// grows with the area it is given, so it is not compared.
fn is_content(symbol: char) -> bool {
    symbol != ' ' && !('\u{2500}'..='\u{257F}').contains(&symbol)
}

/// The symbols a widget paints into a `w`×`h` buffer, in reading order,
/// without blanks, frame lines and wide-glyph continuations.
fn painted(view: &dyn View, w: u16, h: u16) -> Result<Vec<char>, String> {
    let mut buffer = Buffer::new(w, h);
    catch(|| {
        // As the runtime does: an open dropdown goes to the overlay queue and
        // takes no space in the widget's own area.
        let mut overlays = OverlayQueue::new();
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h))
            .with_overlay_queue(&mut overlays);
        view.render(&mut ctx);
    })?;
    let mut symbols = Vec::new();
    for y in 0..h {
        for x in 0..w {
            match buffer.get(x, y) {
                Some(cell) if is_content(cell.symbol) && !cell.is_continuation() => {
                    symbols.push(cell.symbol)
                }
                _ => {}
            }
        }
    }
    Ok(symbols)
}

/// Check one widget with one content. Rule 1 is checked with the ordinary
/// content only - whether a widget measures does not depend on its text - so
/// a widget that does not measure is reported once.
fn check(entry: &Entry, content: &Content, ordinary: bool) -> Option<(Kind, String)> {
    let view = match catch(|| (entry.build)(content, false)) {
        Ok(view) => view,
        Err(msg) => return Some((Kind::Panic, format!("while building: {msg}"))),
    };
    let measured = match catch(|| view.measure(OFFER.0, OFFER.1)) {
        Ok(measured) => measured,
        Err(msg) => return Some((Kind::Panic, format!("while measuring: {msg}"))),
    };
    let fills = FILLS.iter().any(|(name, _)| *name == entry.name);
    let (w, h) = match (measured, fills) {
        (_, true) if !ordinary => return None,
        (None, _) if !ordinary => return None,
        (None, true) => return None,
        (None, false) => return Some((Kind::NoMeasure, "measure answered None".into())),
        (Some(size), true) => {
            return Some((
                Kind::NoMeasure,
                format!("listed in FILLS but measured {size:?} - remove it from FILLS"),
            ))
        }
        (Some(size), false) => size,
    };
    if w > OFFER.0 || h > OFFER.1 {
        return Some((
            Kind::Clipped,
            format!("measured {w}x{h}, larger than the offer"),
        ));
    }
    let full = match painted(&*view, OFFER.0, OFFER.1) {
        Ok(symbols) => symbols,
        Err(msg) => return Some((Kind::Panic, msg)),
    };
    let fitted = match painted(&*view, w, h) {
        Ok(symbols) => symbols,
        Err(msg) => return Some((Kind::Panic, msg)),
    };
    if fitted != full {
        let show = |s: &[char]| s.iter().take(200).collect::<String>();
        return Some((
            Kind::Clipped,
            format!(
                "at the measured {w}x{h}: {:?}, at {}x{}: {:?}",
                show(&fitted),
                OFFER.0,
                OFFER.1,
                show(&full)
            ),
        ));
    }
    None
}

#[test]
fn every_widget_measures_what_it_paints() {
    let mut all = vec![Content::ordinary()];
    all.extend(contents());
    let mut failures = Vec::new();
    let mut count = 0;
    for entry in &catalog() {
        for (i, content) in all.iter().enumerate() {
            count += 1;
            if let Some((kind, detail)) = check(entry, content, i == 0) {
                failures.push(Failure {
                    widget: entry.name,
                    case: format!("{}x{} {}", OFFER.0, OFFER.1, content.label),
                    kind,
                    detail,
                });
            }
        }
    }
    ratchet("measure", count, failures);
}

#[test]
fn every_fills_entry_is_in_the_catalog() {
    let names: Vec<&str> = catalog().iter().map(|e| e.name).collect();
    let stale: Vec<&str> = FILLS
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| !names.contains(name))
        .collect();
    assert!(stale.is_empty(), "FILLS names no catalog entry: {stale:?}");
}
