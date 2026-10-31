//! The options the widget matrix's one configuration per widget does not
//! reach: each widget's other styles, sizes and states answer `measure` with
//! the size they paint (#1100).

use revue::layout::Rect;
use revue::prelude::*;
use revue::render::Buffer;
use revue::widget::*;

const OFFER: (u16, u16) = (80, 24);

/// Every cell a view paints at `w`×`h`, overlays going to their queue as they
/// do in the runtime, as `(x, y, symbol)`; blank cells without a background
/// are left out.
fn cells(view: &dyn View, w: u16, h: u16) -> Vec<(u16, u16, char)> {
    let mut buffer = Buffer::new(w, h);
    let mut overlays = OverlayQueue::new();
    let mut ctx =
        RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)).with_overlay_queue(&mut overlays);
    view.render(&mut ctx);
    let mut out = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let cell = buffer.get(x, y).unwrap();
            if cell.symbol != ' ' || cell.bg.is_some() {
                out.push((x, y, cell.symbol));
            }
        }
    }
    out
}

/// The view measures `want` at the offer, paints nothing outside it, and
/// paints the same cells there as with all the offered room.
#[track_caller]
fn assert_measures(view: impl View, want: (u16, u16)) {
    assert_eq!(view.measure(OFFER.0, OFFER.1), Some(want), "measured size");
    let full = cells(&view, OFFER.0, OFFER.1);
    let outside: Vec<_> = full
        .iter()
        .filter(|&&(x, y, _)| x >= want.0 || y >= want.1)
        .collect();
    assert!(outside.is_empty(), "painted outside {want:?}: {outside:?}");
    assert_eq!(
        cells(&view, want.0, want.1),
        full,
        "cells at the measured size"
    );
}

/// For a view with parts anchored to the edges of its area (a border, a block
/// at the bottom): the view measures `want` and paints the same symbols, in
/// reading order, there as with all the offered room. Box-drawing lines grow
/// with the area and are not compared.
#[track_caller]
fn assert_measures_anchored(view: impl View, want: (u16, u16)) {
    assert_eq!(view.measure(OFFER.0, OFFER.1), Some(want), "measured size");
    let symbols = |w, h| -> String {
        cells(&view, w, h)
            .into_iter()
            .map(|(_, _, ch)| ch)
            .filter(|ch| *ch != ' ' && !('\u{2500}'..='\u{257F}').contains(ch))
            .collect()
    };
    assert_eq!(symbols(want.0, want.1), symbols(OFFER.0, OFFER.1));
}

/// What a view answers never exceeds a smaller offer, and a zero offer is safe.
#[track_caller]
fn assert_clamps(view: impl View) {
    for (w, h) in [(0, 0), (0, 5), (5, 0), (1, 1), (3, 2)] {
        if let Some((mw, mh)) = view.measure(w, h) {
            assert!(
                mw <= w && mh <= h,
                "measured {mw}x{mh} for an offer of {w}x{h}"
            );
        }
    }
}

#[test]
fn avatar_takes_its_shape_and_a_column_for_the_status() {
    assert_measures(Avatar::new("John Doe").small(), (1, 1));
    assert_measures(Avatar::new("John Doe").small().online(), (2, 1));
    assert_measures(Avatar::new("John Doe"), (4, 1));
    assert_measures(Avatar::new("John Doe").online(), (5, 1));
    assert_measures(Avatar::new("John Doe").large(), (5, 3));
    assert_measures(Avatar::new("John Doe").large().online(), (6, 3));
    assert_clamps(Avatar::new("John Doe").large().online());
}

#[test]
fn avatar_leaves_the_rest_of_a_row_to_its_neighbour() {
    let row = hstack()
        .child(Avatar::new("John Doe"))
        .child(Text::new("John Doe"));
    let mut buffer = Buffer::new(40, 1);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 1));
    row.render(&mut ctx);
    let line: String = (0..14).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert_eq!(line, "◖JD◗John Doe  ");
}

#[test]
fn skeleton_follows_its_shape() {
    let plain = || Skeleton::new().no_animate();
    assert_measures(plain().width(5).height(2), (5, 2));
    assert_measures(plain().circle().height(1), (1, 1));
    assert_measures(plain().circle().height(3), (3, 3));
    assert_measures(plain().paragraph().width(10).lines(3), (10, 3));
    // Without a width it stretches, one axis only.
    assert_eq!(plain().measure(30, 24), Some((30, 1)));
    assert_eq!(plain().fills(), Fill::WIDTH);
    assert_eq!(plain().width(5).fills(), Fill::NONE);
    assert_eq!(plain().circle().fills(), Fill::NONE);
    assert_clamps(plain().circle().height(9));
}

#[test]
fn pagination_measures_every_style() {
    assert_measures(
        Pagination::new(5).current(3).no_arrows().no_edges(),
        (11, 1),
    );
    assert_measures(Pagination::new(5).current(1), (19, 1));
    assert_measures(Pagination::new(5).current(1).no_edges(), (15, 1));
    // The window, an ellipsis either side and both ends of a long range
    assert_measures(Pagination::new(100).current(50), (40, 1));
    assert_measures(Pagination::new(5).current(3).simple(), (15, 1));
    assert_measures(Pagination::new(10).current(3).compact(), (4, 1));
    assert_measures(Pagination::new(5).current(3).dots(), (9, 1));
    assert_clamps(Pagination::new(100).current(50));
}

#[test]
fn stepper_measures_both_orientations() {
    let steps = || {
        vec![
            Step::new("Cart").description("Your items"),
            Step::new("Pay"),
        ]
    };
    let horizontal = Stepper::new().steps(steps()).descriptions(false);
    assert_eq!(horizontal.fills(), Fill::WIDTH);
    assert_measures(horizontal, (80, 2));
    assert_measures(Stepper::new().steps(steps()).descriptions(true), (80, 3));
    // No step has a description: no row for them.
    assert_measures(
        Stepper::new()
            .steps(vec![Step::new("Cart")])
            .descriptions(true),
        (80, 2),
    );

    // Icon column, then the longest title or description; a connector row
    // between the steps.
    assert_measures(
        Stepper::new().steps(steps()).vertical().descriptions(true),
        (13, 4),
    );
    assert_measures(
        Stepper::new().steps(steps()).vertical().descriptions(false),
        (7, 3),
    );
    assert_eq!(Stepper::new().vertical().fills(), Fill::NONE);
    assert_eq!(Stepper::new().measure(80, 24), Some((0, 0)));
    assert_clamps(Stepper::new().steps(steps()).vertical());
}

#[test]
fn color_picker_measures_every_mode() {
    assert_measures_anchored(ColorPicker::new(), (16, 4));
    assert_measures_anchored(ColorPicker::new().preview(false), (16, 4));
    // A tall palette: the grid keeps its last two rows free with or without
    // the preview, and none of its rows is cut.
    let tall = || ColorPicker::new().palette(ColorPalette::Extended);
    assert_measures_anchored(tall().preview(false), (32, 18));
    assert_measures_anchored(ColorPicker::new().border(Color::WHITE), (18, 6));
    let rgb = ColorPicker::new().mode(ColorPickerMode::Rgb);
    assert_eq!(rgb.fills(), Fill::WIDTH);
    assert_measures_anchored(rgb, (80, 5));
    assert_measures_anchored(ColorPicker::new().mode(ColorPickerMode::Hsl), (80, 5));
    assert_measures_anchored(ColorPicker::new().mode(ColorPickerMode::Hex), (16, 5));
    assert_clamps(ColorPicker::new());
}

#[test]
fn calendars_keep_their_size_from_month_to_month() {
    // February 2026 starts on a Sunday and takes four weeks, August six.
    for month in [2, 8] {
        assert_measures(Calendar::new(2026, month), (22, 9));
        assert_measures(Calendar::new(2026, month).week_numbers(true), (26, 9));
        assert_measures_anchored(Calendar::new(2026, month).border(Color::WHITE), (24, 11));
    }
    assert_clamps(Calendar::new(2026, 8));

    // The month name is centered over the area it is given.
    assert_measures_anchored(
        DateTimePicker::new().format(DateTimeFormat::DateOnly),
        (22, 8),
    );
    assert_measures_anchored(
        DateTimePicker::new().format(DateTimeFormat::TimeOnly),
        (22, 5),
    );
    assert_measures_anchored(DateTimePicker::new(), (22, 13));
    assert_clamps(DateTimePicker::new());

    assert_measures(RangePicker::new(), (64, 11));
    assert_measures(RangePicker::new().with_presets(false), (57, 11));
    assert_clamps(RangePicker::new());
}

#[test]
fn open_lists_drawn_in_place_are_measured() {
    let mut menu = MenuBar::new()
        .menu(Menu::new("File").items(vec![MenuItem::new("Open"), MenuItem::new("Save")]));
    assert_eq!(menu.fills(), Fill::WIDTH);
    assert_eq!(menu.measure(80, 24), Some((80, 1)));
    menu.open_menu(0);
    assert_measures(menu, (80, 5));

    let mut picker = ThemePicker::new();
    assert_eq!(picker.measure(80, 24), Some((35, 1)));
    picker.open();
    let (w, h) = picker.measure(80, 24).unwrap();
    assert_eq!(w, 35);
    assert!(h >= 3, "the list and its borders: {h}");
    assert_measures(picker, (w, h));
    assert_eq!(
        ThemePicker::new().compact(true).measure(80, 24),
        Some((10, 1))
    );
}

#[test]
fn open_lists_in_an_overlay_take_no_space() {
    let mut combobox = Combobox::new().options(vec!["apple", "pear"]);
    combobox.open_dropdown();
    assert_eq!(combobox.measure(80, 24).map(|(_, h)| h), Some(1));
    let options = vec!["apple".to_string(), "pear".to_string()];
    let mut select = Select::new().options(options);
    select.toggle();
    assert_eq!(select.measure(80, 24).map(|(_, h)| h), Some(1));
}

#[test]
fn collapsed_and_constrained_layouts() {
    assert_measures(Collapsible::new("T").content("a\nb"), (80, 1));
    assert_measures(
        Collapsible::new("T").content("a\nb").expanded(true),
        (80, 4),
    );
    assert_measures(
        Collapsible::new("T")
            .content("a")
            .expanded(true)
            .border(false),
        (80, 2),
    );
    assert_eq!(Collapsible::new("T").fills(), Fill::WIDTH);

    let section = |title| AccordionSection::new(title).content("line").expanded(true);
    assert_measures(
        Accordion::new().section(section("A")).section(section("B")),
        (80, 5),
    );
    assert_measures_anchored(
        Accordion::new().section(section("A")).border(Color::WHITE),
        (80, 4),
    );
    assert_eq!(
        Accordion::new()
            .section(section("A"))
            .max_width(20)
            .measure(80, 24),
        Some((20, 2))
    );

    let bar = Sidebar::new().items(vec![SidebarItem::new("a", "A")]);
    assert_eq!(bar.fills(), Fill::HEIGHT);
    assert_eq!(bar.measure(80, 24), Some((24, 24)));
    let collapsed = Sidebar::new().collapse_mode(CollapseMode::Collapsed);
    assert_eq!(collapsed.measure(80, 24), Some((4, 24)));

    assert_measures(Tabs::new().tabs(vec!["One", "Two"]), (11, 1));
    assert_eq!(Tabs::new().measure(80, 24), Some((0, 1)));
    assert_clamps(Tabs::new().tabs(vec!["One", "Two"]));
}

#[test]
fn fields_measure_their_own_width() {
    let number = NumberInput::new();
    assert_eq!(number.fills(), Fill::WIDTH);
    assert_eq!(number.measure(80, 24), Some((80, 1)));
    let sized = NumberInput::new().width(12);
    assert_eq!(sized.fills(), Fill::NONE);
    assert_eq!(sized.measure(80, 24), Some((12, 1)));
    assert_eq!(sized.measure(5, 24), Some((5, 1)));

    assert_eq!(SearchBar::new().measure(80, 24), Some((40, 1)));
    let mut focused = SearchBar::new();
    focused.focus();
    assert_measures(focused, (40, 2));

    let pin = || MaskedInput::new().value("abc");
    // The field's default 20 columns between its brackets
    assert_measures(pin(), (22, 1));
    assert_measures(pin().label("Pin"), (22, 2));
    assert_measures(pin().label("A label wider than the field"), (28, 2));
}

#[test]
fn waveline_fills_unless_it_has_a_height() {
    assert_eq!(Waveline::new(vec![0.5]).measure(80, 24), None);
    assert_eq!(
        Waveline::new(vec![0.5]).height(4).measure(80, 24),
        Some((80, 4))
    );
    assert_eq!(
        Waveline::new(vec![0.5]).height(40).measure(80, 24),
        Some((80, 24))
    );
}

#[test]
fn a_hidden_transition_takes_no_room() {
    let mut transition = AnimationTransition::new("hello");
    assert_eq!(transition.measure(80, 24), Some((5, 1)));
    // No leave animation: hiding is immediate.
    transition.hide();
    assert_eq!(transition.phase(), TransitionPhase::Hidden);
    assert_eq!(transition.measure(80, 24), Some((0, 0)));
}

#[cfg(feature = "qrcode")]
#[test]
fn qr_code_takes_cells_per_module_by_style() {
    let side = |style| {
        QrCodeWidget::new("hi")
            .style(style)
            .quiet_zone(0)
            .measure(200, 200)
            .unwrap()
    };
    let (w, h) = side(QrStyle::HalfBlock);
    assert_eq!(w, 21);
    assert_eq!(h, 11);
    assert_eq!(side(QrStyle::FullBlock), (42, 21));
    assert_eq!(side(QrStyle::Ascii), (42, 21));
    assert_eq!(side(QrStyle::Braille), (11, 6));
    assert_measures(QrCodeWidget::new("hi").quiet_zone(0), (21, 11));
}
