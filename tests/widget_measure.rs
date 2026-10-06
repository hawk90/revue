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
    Badge, Button, Checkbox, CheckboxStyle, Input, RadioGroup, RadioLayout, RadioStyle, Rating,
    RatingSize, Slider, Switch, SwitchStyle, Tag,
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
