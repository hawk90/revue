//! A builder that named a color outranks the stylesheet — including when the
//! color it named happens to be the widget's own default.
//!
//! `color_or(builder, initial)` compares the two and treats "equal" as "the
//! builder said nothing". That is fine when `initial` is a sentinel nobody sets
//! on purpose — `Color::default()` is fully transparent — but these widgets
//! default to real colors like `DARK_GRAY`, so `.color(DARK_GRAY)` was
//! indistinguishable from silence and a stylesheet won against a builder that
//! had spoken. The same shape `Text.align` had before it became an `Option`.
//!
//! Both directions are asserted. Without the silent-builder half, "always take
//! the builder's value" would pass.

use revue::prelude::*;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::theme::DARK_GRAY as HANDLE_GRAY;
use revue::widget::theme::{DARK_GRAY, LIGHT_GRAY, SEPARATOR_COLOR};
use revue::widget::theme::{DISABLED_FG, SECONDARY_TEXT, SUBTLE_GRAY};
#[cfg(feature = "markdown")]
use revue::widget::MarkdownPresentation;
#[cfg(feature = "qrcode")]
use revue::widget::QrCodeWidget;
use revue::widget::{
    crumb, pane, text, AiStream, Autocomplete, Breadcrumb, Collapsible, Divider, Gauge, Menu,
    MenuBar, Pagination, Rating, Resizable, SearchBar, Skeleton, Slider, SortableList, Splitter,
    StatusBar, Switch, Tag, Terminal, VirtualList, ZenMode,
};
use revue::widget::{Presentation, Slide};

const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

fn draw<V: View>(css: &str, view: &V) -> PipelineHarness {
    let mut h = PipelineHarness::with_css(css, 40, 6).dom_from_render(true);
    h.draw(view);
    h
}

fn any_bg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.bg) == Some(want)))
}

fn any_fg(h: &PipelineHarness, want: Color) -> bool {
    let buffer = h.buffer();
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).and_then(|c| c.fg) == Some(want)))
}

macro_rules! case {
    ($named:ident, $silent:ident, $build_named:expr, $build_silent:expr) => {
        struct $named;
        impl View for $named {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build_named).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($named)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
        struct $silent;
        impl View for $silent {
            fn render(&self, ctx: &mut RenderContext) {
                vstack().child($build_silent).render(ctx);
            }
            fn widget_type(&self) -> &'static str {
                stringify!($silent)
            }
            fn id(&self) -> Option<&str> {
                Some("root")
            }
        }
    };
}

case!(
    DividerNamed,
    DividerSilent,
    Divider::new().color(DARK_GRAY).element_id("w"),
    Divider::new().element_id("w")
);

case!(
    SkeletonNamed,
    SkeletonSilent,
    Skeleton::new().color(SEPARATOR_COLOR).element_id("w"),
    Skeleton::new().element_id("w")
);

case!(
    TagNamed,
    TagSilent,
    Tag::new("v1").color(DARK_GRAY).element_id("w"),
    Tag::new("v1").element_id("w")
);

case!(
    CrumbNamed,
    CrumbSilent,
    Breadcrumb::new()
        .item(crumb("home"))
        .item_color(LIGHT_GRAY)
        .element_id("w"),
    Breadcrumb::new().item(crumb("home")).element_id("w")
);

macro_rules! both_directions {
    ($named_test:ident, $silent_test:ident, $named:ident, $silent:ident, $default:expr) => {
        #[test]
        fn $named_test() {
            let h = draw("#w { color: #ff0000; }", &$named);
            assert!(
                !any_fg(&h, RED),
                "a stylesheet outranked a builder that named the color"
            );
            assert!(any_fg(&h, $default), "the builder's own color went missing");
        }

        #[test]
        fn $silent_test() {
            let h = draw("#w { color: #ff0000; }", &$silent);
            assert!(
                any_fg(&h, RED),
                "a silent builder stopped deferring to the stylesheet"
            );
        }
    };
}

both_directions!(
    a_named_divider_color_beats_css,
    a_silent_divider_defers_to_css,
    DividerNamed,
    DividerSilent,
    DARK_GRAY
);

both_directions!(
    a_named_skeleton_color_beats_css,
    a_silent_skeleton_defers_to_css,
    SkeletonNamed,
    SkeletonSilent,
    SEPARATOR_COLOR
);

/// `Tag::color` reads the `color` property but paints it as the *fill* -
/// `text_color` is the foreground - so the rule is the same and the assertion
/// is against the background.
#[test]
fn a_named_tag_color_beats_css() {
    let h = draw("#w { color: #ff0000; }", &TagNamed);
    assert!(
        !any_bg(&h, RED),
        "a stylesheet outranked a builder that named the color"
    );
    assert!(
        any_bg(&h, DARK_GRAY),
        "the builder's own color went missing"
    );
}

#[test]
fn a_silent_tag_defers_to_css() {
    let h = draw("#w { color: #ff0000; }", &TagSilent);
    assert!(
        any_bg(&h, RED),
        "a silent builder stopped deferring to the stylesheet"
    );
}

both_directions!(
    a_named_crumb_color_beats_css,
    a_silent_crumb_defers_to_css,
    CrumbNamed,
    CrumbSilent,
    LIGHT_GRAY
);

// ---------------------------------------------------------------------------
// Batch 2
// ---------------------------------------------------------------------------

case!(
    GaugeNamed,
    GaugeSilent,
    // `value` is normalised 0..1, and the warning/critical thresholds are unset
    // by default, so this paints the ordinary fill.
    Gauge::new()
        .value(0.5)
        .fill_color(Color::GREEN)
        .element_id("w"),
    Gauge::new().value(0.5).element_id("w")
);

case!(
    StatusBarNamed,
    StatusBarSilent,
    StatusBar::new()
        .key("q", "quit")
        .fg(Color::WHITE)
        .element_id("w"),
    StatusBar::new().key("q", "quit").element_id("w")
);

case!(
    MenuBarNamed,
    MenuBarSilent,
    // Two menus: the first is selected and keeps its highlight, so the second
    // is the one `fg` paints.
    MenuBar::new()
        .menu(Menu::new("File"))
        .menu(Menu::new("Edit"))
        .fg(Color::WHITE)
        .element_id("w"),
    MenuBar::new()
        .menu(Menu::new("File"))
        .menu(Menu::new("Edit"))
        .element_id("w")
);

both_directions!(
    a_named_gauge_fill_beats_css,
    a_silent_gauge_defers_to_css,
    GaugeNamed,
    GaugeSilent,
    Color::GREEN
);

both_directions!(
    a_named_status_bar_fg_beats_css,
    a_silent_status_bar_defers_to_css,
    StatusBarNamed,
    StatusBarSilent,
    Color::WHITE
);

both_directions!(
    a_named_menu_bar_fg_beats_css,
    a_silent_menu_bar_defers_to_css,
    MenuBarNamed,
    MenuBarSilent,
    Color::WHITE
);

// ---------------------------------------------------------------------------
// Batch 3
// ---------------------------------------------------------------------------

case!(
    SwitchNamed,
    SwitchSilent,
    Switch::new()
        .checked(true)
        .on_color(Color::GREEN)
        .element_id("w"),
    Switch::new().checked(true).element_id("w")
);

case!(
    RatingNamed,
    RatingSilent,
    Rating::new()
        .max_value(5)
        .value(3.0)
        .filled_color(Color::rgb(255, 200, 0))
        .element_id("w"),
    Rating::new().max_value(5).value(3.0).element_id("w")
);

case!(
    SliderNamed,
    SliderSilent,
    Slider::new()
        .range(0.0, 100.0)
        .value(50.0)
        .fill_color(Color::CYAN)
        .element_id("w"),
    Slider::new().range(0.0, 100.0).value(50.0).element_id("w")
);

both_directions!(
    a_named_switch_on_color_beats_css,
    a_silent_switch_defers_to_css,
    SwitchNamed,
    SwitchSilent,
    Color::GREEN
);

both_directions!(
    a_named_rating_fill_beats_css,
    a_silent_rating_defers_to_css,
    RatingNamed,
    RatingSilent,
    Color::rgb(255, 200, 0)
);

both_directions!(
    a_named_slider_fill_beats_css,
    a_silent_slider_defers_to_css,
    SliderNamed,
    SliderSilent,
    Color::CYAN
);

// ---------------------------------------------------------------------------
// Batch 4
// ---------------------------------------------------------------------------

/// `text_color` paints the *typed* query; an empty bar shows the placeholder in
/// its own color, so the fixture has to type something.
fn typed_search_bar(named: bool) -> SearchBar {
    let mut bar = if named {
        SearchBar::new().colors(Color::BLACK, DARK_GRAY, Color::WHITE)
    } else {
        SearchBar::new()
    };
    bar.set_query("term");
    bar.element_id("w")
}

case!(
    SearchBarNamed,
    SearchBarSilent,
    typed_search_bar(true),
    typed_search_bar(false)
);

case!(
    AutocompleteNamed,
    AutocompleteSilent,
    Autocomplete::new()
        .suggestions(["alpha", "beta"])
        .value("al")
        .input_style(Color::WHITE, Color::BLACK)
        .element_id("w"),
    Autocomplete::new()
        .suggestions(["alpha", "beta"])
        .value("al")
        .element_id("w")
);

case!(
    CollapsibleNamed,
    CollapsibleSilent,
    Collapsible::new("Details")
        .header_colors(Color::WHITE, None)
        .element_id("w"),
    Collapsible::new("Details").element_id("w")
);

case!(
    SplitterNamed,
    SplitterSilent,
    Splitter::new()
        .pane(pane("a"))
        .pane(pane("b"))
        .color(HANDLE_GRAY)
        .element_id("w"),
    Splitter::new()
        .pane(pane("a"))
        .pane(pane("b"))
        .element_id("w")
);

both_directions!(
    a_named_search_bar_text_beats_css,
    a_silent_search_bar_defers_to_css,
    SearchBarNamed,
    SearchBarSilent,
    Color::WHITE
);

both_directions!(
    a_named_autocomplete_input_beats_css,
    a_silent_autocomplete_defers_to_css,
    AutocompleteNamed,
    AutocompleteSilent,
    Color::WHITE
);

both_directions!(
    a_named_collapsible_header_beats_css,
    a_silent_collapsible_defers_to_css,
    CollapsibleNamed,
    CollapsibleSilent,
    Color::WHITE
);

both_directions!(
    a_named_splitter_color_beats_css,
    a_silent_splitter_defers_to_css,
    SplitterNamed,
    SplitterSilent,
    HANDLE_GRAY
);

// ---------------------------------------------------------------------------
// Batch 5
// ---------------------------------------------------------------------------

case!(
    ResizableNamed,
    ResizableSilent,
    Resizable::new(20, 4)
        .handle_color(DISABLED_FG)
        .element_id("w"),
    Resizable::new(20, 4).element_id("w")
);

case!(
    VirtualListNamed,
    VirtualListSilent,
    VirtualList::new(vec!["a".to_string(), "b".to_string()])
        .item_fg(Color::WHITE)
        .element_id("w"),
    VirtualList::new(vec!["a".to_string(), "b".to_string()]).element_id("w")
);

case!(
    SortableNamed,
    SortableSilent,
    SortableList::new(["a", "b"])
        .item_color(SECONDARY_TEXT)
        .element_id("w"),
    SortableList::new(["a", "b"]).element_id("w")
);

case!(
    PaginationNamed,
    PaginationSilent,
    Pagination::new(5)
        .inactive_color(SUBTLE_GRAY)
        .element_id("w"),
    Pagination::new(5).element_id("w")
);

both_directions!(
    a_named_resizable_handle_beats_css,
    a_silent_resizable_defers_to_css,
    ResizableNamed,
    ResizableSilent,
    DISABLED_FG
);

both_directions!(
    a_named_virtual_list_item_beats_css,
    a_silent_virtual_list_defers_to_css,
    VirtualListNamed,
    VirtualListSilent,
    Color::WHITE
);

both_directions!(
    a_named_sortable_item_beats_css,
    a_silent_sortable_defers_to_css,
    SortableNamed,
    SortableSilent,
    SECONDARY_TEXT
);

both_directions!(
    a_named_pagination_inactive_beats_css,
    a_silent_pagination_defers_to_css,
    PaginationNamed,
    PaginationSilent,
    SUBTLE_GRAY
);

// ---------------------------------------------------------------------------
// Batch 6
// ---------------------------------------------------------------------------

fn zen(named: bool) -> ZenMode {
    let mut z = if named {
        ZenMode::new(text("focus")).bg(Color::rgb(15, 15, 25))
    } else {
        ZenMode::new(text("focus"))
    }
    .element_id("w");
    z.enable();
    z
}

case!(ZenNamed, ZenSilent, zen(true), zen(false));

#[cfg(feature = "qrcode")]
case!(
    QrNamed,
    QrSilent,
    QrCodeWidget::new("hi").fg(Color::BLACK).element_id("w"),
    QrCodeWidget::new("hi").element_id("w")
);

fn stream(named: bool) -> AiStream {
    let mut s = if named {
        AiStream::new().fg(Color::WHITE)
    } else {
        AiStream::new()
    }
    .content("streamed")
    .element_id("w");
    s.complete();
    s
}

case!(StreamNamed, StreamSilent, stream(true), stream(false));

fn term(named: bool) -> Terminal {
    let mut t = if named {
        Terminal::new(30, 4).default_fg(Color::WHITE)
    } else {
        Terminal::new(30, 4)
    }
    .element_id("w");
    t.writeln("boot");
    t
}

case!(TerminalNamed, TerminalSilent, term(true), term(false));

#[test]
fn a_named_zen_background_beats_css() {
    let h = draw("#w { background: #ff0000; }", &ZenNamed);
    assert!(!any_bg(&h, RED));
    assert!(any_bg(&h, Color::rgb(15, 15, 25)));
}

#[test]
fn a_silent_zen_defers_to_css() {
    let h = draw("#w { background: #ff0000; }", &ZenSilent);
    assert!(any_bg(&h, RED));
}

#[cfg(feature = "qrcode")]
both_directions!(
    a_named_qr_fg_beats_css,
    a_silent_qr_defers_to_css,
    QrNamed,
    QrSilent,
    Color::BLACK
);

both_directions!(
    a_named_stream_fg_beats_css,
    a_silent_stream_defers_to_css,
    StreamNamed,
    StreamSilent,
    Color::WHITE
);

both_directions!(
    a_named_terminal_fg_beats_css,
    a_silent_terminal_defers_to_css,
    TerminalNamed,
    TerminalSilent,
    Color::WHITE
);

// ---------------------------------------------------------------------------
// Batch 6b
// ---------------------------------------------------------------------------

case!(
    DeckNamed,
    DeckSilent,
    Presentation::new()
        .slide(Slide::new("Deck").line("body"))
        .bg(Color::rgb(20, 20, 30))
        .element_id("w"),
    Presentation::new()
        .slide(Slide::new("Deck").line("body"))
        .element_id("w")
);

#[cfg(feature = "markdown")]
case!(
    MdDeckNamed,
    MdDeckSilent,
    MarkdownPresentation::new("# Title\n\nbody\n")
        .bg(Color::rgb(20, 20, 30))
        .element_id("w"),
    MarkdownPresentation::new("# Title\n\nbody\n").element_id("w")
);

#[test]
fn a_named_presentation_background_beats_css() {
    let h = draw("#w { background: #ff0000; }", &DeckNamed);
    assert!(!any_bg(&h, RED));
    assert!(any_bg(&h, Color::rgb(20, 20, 30)));
}

#[test]
fn a_silent_presentation_defers_to_css() {
    let h = draw("#w { background: #ff0000; }", &DeckSilent);
    assert!(any_bg(&h, RED));
}

#[cfg(feature = "markdown")]
#[test]
fn a_named_markdown_deck_background_beats_css() {
    let h = draw("#w { background: #ff0000; }", &MdDeckNamed);
    assert!(!any_bg(&h, RED));
    assert!(any_bg(&h, Color::rgb(20, 20, 30)));
}

#[cfg(feature = "markdown")]
#[test]
fn a_silent_markdown_deck_defers_to_css() {
    let h = draw("#w { background: #ff0000; }", &MdDeckSilent);
    assert!(any_bg(&h, RED));
}
