//! `View::measure` must agree with what `render` paints.
//!
//! A content-sized stack (`Stack::content_sized`) gives each child exactly the
//! size it measures, so a widget that answers too small is clipped and one that
//! answers too large pushes its siblings away. Each test renders the widget
//! into a roomy area, takes the bounding box of everything it painted, and
//! checks it against `measure` - then renders again into exactly the measured
//! size and checks nothing was lost.
//!
//! Widgets that stretch along a row (inputs, bars) answer with the whole width
//! they are offered; for those only the rows are compared.

use revue::layout::Rect;
use revue::prelude::*;
use revue::render::{Buffer, Cell};
use revue::widget::traits::RenderContext;
use revue::widget::{
    card, Badge, BigText, Breadcrumb, Button, CardVariant, Checkbox, CheckboxStyle, DebugOverlay,
    DigitStyle, Digits, Divider, ErrorBoundary, Gauge, GaugeStyle, Input, Link, Progress,
    RadioGroup, RadioLayout, RadioStyle, Rating, RatingSize, RichText, Slider, Sparkline, Spinner,
    Status, StatusIndicator, StatusSize, StatusStyle, Switch, SwitchStyle, Tag, ZenMode,
};

const W: u16 = 60;
const H: u16 = 20;

fn render_into<V: View>(view: &V, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

/// Width and height of the box holding every cell render touched.
fn painted_box(buffer: &Buffer, width: u16, height: u16) -> (u16, u16) {
    let (mut w, mut h) = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if buffer.get(x, y).is_some_and(|c| *c != Cell::default()) {
                w = w.max(x + 1);
                h = h.max(y + 1);
            }
        }
    }
    (w, h)
}

fn screen(buffer: &Buffer, width: u16, height: u16) -> String {
    (0..height)
        .map(|y| {
            (0..width)
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .filter(|&c| c != '\0')
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Rendering into exactly `size` paints what rendering into `W` x `H` did
/// inside that box.
fn assert_nothing_clipped<V: View>(label: &str, view: &V, size: (u16, u16)) {
    let big = render_into(view, W, H);
    let exact = render_into(view, size.0, size.1);
    for y in 0..size.1 {
        for x in 0..size.0 {
            assert_eq!(
                exact.get(x, y),
                big.get(x, y),
                "{label}: cell ({x}, {y}) differs when given exactly {size:?}:\n{}\n--- given {W}x{H}:\n{}",
                screen(&exact, size.0, size.1),
                screen(&big, W, H),
            );
        }
    }
}

/// A widget with a fixed natural size: `measure` is the painted box.
fn assert_measures_paint<V: View>(label: &str, view: &V) {
    let size = view
        .measure(W, H)
        .unwrap_or_else(|| panic!("{label}: measure returned None"));
    let big = render_into(view, W, H);
    assert_eq!(
        size,
        painted_box(&big, W, H),
        "{label}: measure disagrees with the painted box:\n{}",
        screen(&big, W, H)
    );
    assert_nothing_clipped(label, view, size);
    assert_clamped(label, view);
}

/// A widget that stretches along the row: the full width, `rows` tall.
fn assert_stretches<V: View>(label: &str, view: &V, rows: u16) {
    assert_eq!(view.measure(W, H), Some((W, rows)), "{label}");
    assert_eq!(view.measure(W / 2, H), Some((W / 2, rows)), "{label}");
    let big = render_into(view, W, H);
    assert_eq!(
        painted_box(&big, W, H).1,
        rows,
        "{label}: painted rows disagree with measure:\n{}",
        screen(&big, W, H)
    );
    assert_nothing_clipped(label, view, (W, rows));
    assert_clamped(label, view);
}

/// Never more than offered.
fn assert_clamped<V: View>(label: &str, view: &V) {
    for (mw, mh) in [(0, 0), (1, 1), (3, 0), (0, 3), (2, 2)] {
        if let Some((w, h)) = view.measure(mw, mh) {
            assert!(
                w <= mw && h <= mh,
                "{label}: measure({mw}, {mh}) = ({w}, {h})"
            );
        }
    }
}

// ==================== Button, Badge, Tag ====================

#[test]
fn button_measures_its_label_and_padding() {
    assert_measures_paint("plain", &Button::new("OK"));
    assert_measures_paint("primary", &Button::primary("Save changes"));
    assert_measures_paint("focused", &Button::new("Go").focused(true));
    assert_measures_paint("icon", &Button::new("Save").icon('✓'));
    assert_measures_paint("wide icon", &Button::new("Save").icon('💾'));
    assert_measures_paint("wide label", &Button::new("저장"));
    assert_measures_paint("min width", &Button::new("OK").width(12));
    assert_eq!(Button::new("OK").measure(W, H), Some((6, 1)));
    assert_eq!(Button::new("저장").measure(W, H), Some((8, 1)));
}

#[test]
fn badge_measures_its_text_and_padding() {
    assert_measures_paint("default", &Badge::new("v1.0"));
    assert_measures_paint("pill", &Badge::new("new").pill());
    assert_measures_paint("square", &Badge::new("3").square().error());
    assert_measures_paint("max width", &Badge::new("a long badge").max_width(6));
    assert_measures_paint("dot", &Badge::dot());
    assert_eq!(Badge::new("5").measure(W, H), Some((3, 1)));
    assert_eq!(Badge::new("한글").measure(W, H), Some((6, 1)));
}

#[test]
fn tag_measures_its_content_and_edges() {
    assert_measures_paint("plain", &Tag::new("rust"));
    assert_measures_paint("outlined", &Tag::new("TUI").outlined());
    assert_measures_paint("closable", &Tag::new("Framework").closable());
    assert_measures_paint("icon", &Tag::new("ok").icon('✓'));
    assert_measures_paint("wide", &Tag::new("한글"));
    assert_eq!(Tag::new("rust").measure(W, H), Some((6, 1)));
    assert_eq!(Tag::new("한글").measure(W, H), Some((6, 1)));
}

/// The columns `measure` counts for a wide glyph are the ones render gives it:
/// the glyph, then its continuation cell.
#[test]
fn wide_labels_take_two_columns_each() {
    fn row(view: &impl View) -> Vec<char> {
        let buffer = render_into(view, W, 1);
        (0..W).map(|x| buffer.get(x, 0).unwrap().symbol).collect()
    }
    let badge = row(&Badge::new("한글"));
    assert_eq!(&badge[..6], [' ', '한', '\0', '글', '\0', ' ']);
    let tag = row(&Tag::new("한글"));
    assert_eq!(&tag[..6], [' ', '한', '\0', '글', '\0', ' ']);
    let button = row(&Button::new("Save").icon('💾'));
    assert_eq!(&button[2..6], ['💾', '\0', ' ', 'S']);
}

// ==================== Input, Checkbox, Switch, Radio, Rating, Slider ====================

#[test]
fn input_takes_one_row_of_whatever_width_it_is_offered() {
    assert_stretches("placeholder", &Input::new().placeholder("Search..."), 1);
    assert_stretches("value", &Input::new().value("hello"), 1);
}

#[test]
fn checkbox_measures_its_box_and_label() {
    assert_measures_paint("square", &Checkbox::new("Remember me"));
    assert_measures_paint("checked", &Checkbox::new("On").checked(true));
    assert_measures_paint("focused", &Checkbox::new("Focus").focused(true));
    assert_measures_paint(
        "unicode",
        &Checkbox::new("Unicode").style(CheckboxStyle::Unicode),
    );
    assert_measures_paint("wide", &Checkbox::new("동의"));
    assert_eq!(Checkbox::new("abc").measure(W, H), Some((7, 1)));
    // Focus adds the `> ` marker in front.
    assert_eq!(
        Checkbox::new("abc").focused(true).measure(W, H),
        Some((9, 1))
    );
}

#[test]
fn switch_measures_track_label_and_focus_bracket() {
    assert_measures_paint("bare", &Switch::new());
    assert_measures_paint("on", &Switch::new().on(true));
    assert_measures_paint("label left", &Switch::new().label("Wi-Fi"));
    assert_measures_paint("label right", &Switch::new().label("Wi-Fi").label_right());
    assert_measures_paint("focused", &Switch::new().focused(true));
    assert_measures_paint("focused label", &Switch::new().label("Dark").focused(true));
    assert_measures_paint(
        "focused label right",
        &Switch::new().label("Dark").label_right().focused(true),
    );
    assert_measures_paint("text", &Switch::new().style(SwitchStyle::Text));
    assert_measures_paint("ios", &Switch::new().style(SwitchStyle::IOS).width(8));
    assert_measures_paint("block", &Switch::new().style(SwitchStyle::Block));
    assert_measures_paint("material", &Switch::new().style(SwitchStyle::Material));
    assert_measures_paint("wide label", &Switch::new().label("알림"));
    assert_eq!(Switch::new().measure(W, H), Some((6, 1)));
    assert_eq!(Switch::new().label("Wi-Fi").measure(W, H), Some((12, 1)));
}

#[test]
fn radio_group_measures_its_options() {
    let options = ["Small", "Medium", "Large"];
    assert_measures_paint("vertical", &RadioGroup::new(options));
    assert_measures_paint("vertical gap", &RadioGroup::new(options).gap(1));
    assert_measures_paint(
        "horizontal",
        &RadioGroup::new(options).layout(RadioLayout::Horizontal),
    );
    assert_measures_paint("focused", &RadioGroup::new(options).focused(true));
    assert_measures_paint(
        "unicode",
        &RadioGroup::new(options).style(RadioStyle::Unicode),
    );
    assert_measures_paint("wide", &RadioGroup::new(["작게", "크게"]));
    assert_eq!(RadioGroup::new(options).measure(W, H), Some((10, 3)));
    assert_eq!(
        RadioGroup::new(Vec::<String>::new()).measure(W, H),
        Some((0, 0))
    );
}

#[test]
fn rating_measures_label_stars_and_value() {
    assert_measures_paint("plain", &Rating::new().value(3.0));
    assert_measures_paint("label", &Rating::new().value(2.0).label("Quality"));
    assert_measures_paint("value", &Rating::new().value(4.5).show_value(true));
    assert_measures_paint("large", &Rating::new().size(RatingSize::Large));
    assert_measures_paint("ten", &Rating::new().max_value(10).show_value(true));
    // Five stars two apart, the last with no trailing gap.
    assert_eq!(Rating::new().measure(W, H), Some((9, 1)));
}

#[test]
fn slider_measures_its_fixed_track() {
    assert_measures_paint("plain", &Slider::new().value(30.0));
    assert_measures_paint("label", &Slider::new().label("Vol").value(50.0));
    assert_measures_paint("value", &Slider::new().show_value(true).value(42.0));
    assert_measures_paint("short", &Slider::new().length(8));
    assert_measures_paint("ticks", &Slider::new().ticks(5));
    assert_measures_paint("vertical", &Slider::new().vertical().length(6));
    assert_measures_paint(
        "vertical value",
        &Slider::new().vertical().length(6).show_value(true),
    );
    assert_eq!(Slider::new().show_value(false).measure(W, H), Some((20, 1)));
    // The value, after a space: "0".
    assert_eq!(Slider::new().measure(W, H), Some((22, 1)));
}

// ==================== Progress, Spinner, Divider, Gauge, Sparkline ====================

#[test]
fn progress_takes_one_row_of_whatever_width_it_is_offered() {
    assert_stretches("half", &Progress::new(0.5), 1);
    assert_stretches("percentage", &Progress::new(0.25).show_percentage(true), 1);
}

#[test]
fn sparkline_takes_one_row_of_whatever_width_it_is_offered() {
    assert_stretches("data", &Sparkline::new([1.0, 4.0, 2.0, 8.0]), 1);
}

#[test]
fn spinner_measures_its_glyph_and_label() {
    assert_measures_paint("bare", &Spinner::new());
    assert_measures_paint("label", &Spinner::new().label("Loading..."));
    assert_measures_paint("wide label", &Spinner::new().label("로딩"));
    assert_eq!(Spinner::new().measure(W, H), Some((1, 1)));
    assert_eq!(Spinner::new().label("Wait").measure(W, H), Some((6, 1)));
}

#[test]
fn divider_runs_the_length_it_is_offered() {
    assert_stretches("horizontal", &Divider::new(), 1);
    assert_stretches("label", &Divider::new().label("Section"), 1);
    assert_measures_paint("fixed", &Divider::new().length(10));
    assert_measures_paint("fixed + margin", &Divider::new().margin(2).length(10));

    // Vertical: one column, the full height.
    let v = Divider::vertical();
    assert_eq!(v.measure(W, H), Some((1, H)));
    assert_eq!(painted_box(&render_into(&v, W, H), W, H), (1, H));
    assert_nothing_clipped("vertical", &v, (1, H));
    assert_measures_paint("vertical fixed", &Divider::vertical().length(4));
    assert_clamped("vertical", &v);
}

#[test]
fn gauge_measures_the_styles_drawn_at_a_set_size() {
    assert_measures_paint("bar", &Gauge::new().value(0.4));
    assert_measures_paint("bar + title", &Gauge::new().value(0.4).title("CPU"));
    assert_measures_paint("long title", &Gauge::new().width(4).title("Memory usage"));
    assert_measures_paint("battery", &Gauge::new().style(GaugeStyle::Battery));
    assert_measures_paint("dots", &Gauge::new().style(GaugeStyle::Dots).value(0.5));
    assert_measures_paint("vertical", &Gauge::new().style(GaugeStyle::Vertical));
    assert_measures_paint("thermometer", &Gauge::new().style(GaugeStyle::Thermometer));
    assert_eq!(Gauge::new().measure(W, H), Some((20, 1)));

    // Segments are two columns apart; render fits `width / 2` of them, so
    // the last one needs its gap column too.
    let segments = Gauge::new().style(GaugeStyle::Segments).segments(5);
    assert_eq!(segments.measure(W, H), Some((10, 1)));
    assert_eq!(painted_box(&render_into(&segments, W, H), W, H), (9, 1));
    assert_nothing_clipped("segments", &segments, (10, 1));

    for style in [GaugeStyle::Arc, GaugeStyle::Circle] {
        assert_eq!(Gauge::new().style(style).measure(W, H), None, "{style:?}");
    }
}

// ==================== Digits, BigText, Card ====================

#[test]
fn digits_measure_their_glyphs() {
    for style in [
        DigitStyle::Block,
        DigitStyle::Thin,
        DigitStyle::Ascii,
        DigitStyle::Braille,
    ] {
        let d = Digits::new(1234).style(style);
        let (w, h) = d.measure(W, H).unwrap();
        // Trailing blank columns paint as default cells, so compare the ink.
        let buffer = render_into(&d, W, H);
        assert_eq!((w, h), painted_box(&buffer, W, H), "{style:?}");
        assert_nothing_clipped(&format!("{style:?}"), &d, (w, h));
    }
    assert_measures_paint("suffix", &Digits::new(42).suffix(" ms"));
}

#[test]
fn figlet_bigtext_measures_its_art() {
    for tier in 1..=6u8 {
        assert_measures_paint(
            &format!("tier {tier}"),
            &BigText::new("Hi 42", tier).force_figlet(true),
        );
    }
    assert_eq!(BigText::new("", 1).measure(W, H), Some((0, 0)));
}

/// Cards fill the width they are offered, so only the rows are checked: at
/// the measured height every part shows, and one row less loses the body.
fn assert_card_rows<V: View>(label: &str, view: &V, rows: u16, must_show: &[&str]) {
    assert_eq!(view.measure(W, H), Some((W, rows)), "{label}");
    let buffer = render_into(view, W, rows);
    let text = screen(&buffer, W, rows);
    for s in must_show {
        assert!(
            text.contains(s),
            "{label}: {s:?} missing at {rows} rows:\n{text}"
        );
    }
    let last = screen(&buffer, W, rows)
        .lines()
        .last()
        .unwrap_or("")
        .to_string();
    assert!(!last.trim().is_empty(), "{label}: last row blank:\n{text}");
}

#[test]
fn card_measures_its_rows() {
    let body = || Text::new("hello");
    assert_card_rows(
        "title + body",
        &card().title("T").body(body()),
        5,
        &["T", "hello"],
    );
    assert_card_rows(
        "subtitle",
        &card().title("T").subtitle("sub").body(body()),
        6,
        &["T", "sub", "hello"],
    );
    assert_card_rows(
        "footer",
        &card().title("T").body(body()).footer(Text::new("foot")),
        7,
        &["T", "hello", "foot"],
    );
    assert_card_rows(
        "body only",
        &card().body(vstack().child(body()).child(body())),
        4,
        &["hello"],
    );
    assert_card_rows(
        "flat",
        &card().title("T").body(body()).flat(),
        3,
        &["T", "hello"],
    );
    assert_card_rows(
        "elevated",
        &card()
            .title("T")
            .body(body())
            .variant(CardVariant::Elevated),
        6,
        &["T", "hello"],
    );
    assert_card_rows(
        "collapsed",
        &card()
            .title("T")
            .body(body())
            .collapsible(true)
            .expanded(false),
        3,
        &["T"],
    );

    // One row less and the body has no room.
    let c = card().title("T").body(body());
    let short = screen(&render_into(&c, W, 4), W, 4);
    assert!(!short.contains("hello"), "{short}");

    // A body that fills makes the card fill.
    struct Fill;
    impl View for Fill {
        fn render(&self, _: &mut RenderContext) {}
    }
    assert_eq!(card().title("T").body(Fill).measure(W, H), None);
    assert_eq!(card().title("T").measure(W, H), Some((W, 3)));
}

// ==================== Link, Breadcrumb, RichText, StatusIndicator ====================

#[test]
fn link_measures_the_text_it_shows() {
    assert_measures_paint("url", &Link::new("https://example.com"));
    assert_measures_paint("text", &Link::new("https://example.com").text("docs"));
    assert_eq!(Link::new("u").text("docs").measure(W, H), Some((4, 1)));
}

#[test]
fn breadcrumb_measures_its_trail() {
    let trail = Breadcrumb::new().push("src").push("widget").push("mod.rs");
    assert_measures_paint("trail", &trail);
    assert_measures_paint("home", &Breadcrumb::new().home(true).push("docs"));
    assert_measures_paint("wide", &Breadcrumb::new().push("문서").push("설정"));
    // "src › widget › mod.rs": 15 columns of names, 3 per separator.
    let no_home = Breadcrumb::new()
        .home(false)
        .push("src")
        .push("widget")
        .push("mod.rs");
    assert_eq!(no_home.measure(W, H), Some((21, 1)));
    // The home icon and its separator add 4.
    assert_eq!(trail.measure(W, H), Some((25, 1)));
}

#[test]
fn rich_text_measures_its_lines() {
    assert_measures_paint("one line", &RichText::new().text("plain ").text("more"));
    assert_measures_paint("two lines", &RichText::new().text("first\nsecond line"));
    assert_eq!(
        RichText::new().text("ab\ncdef").text("\ng").measure(W, H),
        Some((4, 3))
    );
}

#[test]
fn status_indicator_measures_its_width() {
    for style in [
        StatusStyle::Dot,
        StatusStyle::DotWithLabel,
        StatusStyle::LabelOnly,
        StatusStyle::Badge,
    ] {
        for size in [StatusSize::Small, StatusSize::Medium, StatusSize::Large] {
            let s = StatusIndicator::new(Status::Online)
                .indicator_style(style)
                .size(size);
            assert_measures_paint(&format!("{style:?} {size:?}"), &s);
        }
    }
}

// ==================== Wrappers ====================

#[test]
fn wrappers_that_draw_nothing_of_their_own_forward_measure() {
    let inner = || Text::new("ok");

    assert_eq!(
        ErrorBoundary::new().child(inner()).measure(W, H),
        Some((2, 1))
    );
    assert_eq!(ErrorBoundary::new().measure(W, H), None);

    let mut zen = ZenMode::new(inner());
    assert_eq!(zen.measure(W, H), Some((2, 1)));
    // On, it paints its background over the whole area.
    zen.enable();
    assert_eq!(zen.measure(W, H), None);

    assert_eq!(
        DebugOverlay::wrap(inner()).visible(false).measure(W, H),
        Some((2, 1))
    );
    assert_eq!(
        DebugOverlay::wrap(inner()).visible(true).measure(W, H),
        None
    );

    let boxed: Box<dyn View> = Box::new(inner());
    assert_eq!(boxed.measure(W, H), Some((2, 1)));
}

#[test]
fn an_error_boundary_measures_its_fallback_after_a_panic() {
    struct Boom;
    impl View for Boom {
        fn render(&self, _: &mut RenderContext) {
            panic!("boom");
        }
        fn measure(&self, _: u16, _: u16) -> Option<(u16, u16)> {
            Some((9, 9))
        }
    }
    let boundary = ErrorBoundary::new()
        .child(Boom)
        .fallback(Text::new("failed"));
    assert_eq!(boundary.measure(W, H), Some((9, 9)));
    render_into(&boundary, W, H);
    assert_eq!(boundary.measure(W, H), Some((6, 1)));
}
