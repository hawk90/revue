//! A widget's `height()` / `width()` helper must agree with what `render` paints.
//!
//! Layout code sizes a widget from these helpers, so a helper that promises
//! more rows than the widget draws leaves blank rows on screen, and one that
//! promises fewer clips the widget. Each test here renders the widget into an
//! area exactly as big as the helper says and checks that the painted extent
//! matches - neither short of it nor beyond it.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::utils::{figlet_with_font, FigletFont};
use revue::widget::traits::RenderContext;
use revue::widget::{
    card, Alert, AlertVariant, BigText, Callout, CalloutVariant, CardVariant, DigitStyle, Digits,
    EmptyState, EmptyStateVariant, StatusBar, StatusSection, Text, View,
};

const W: u16 = 40;

fn render_into<V: View>(view: &V, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

fn is_ink(symbol: char) -> bool {
    symbol != ' ' && symbol != '\0'
}

/// Rows from the top down to the last one carrying a visible glyph.
///
/// The full-height accent bar some variants draw down column 0 stretches to
/// whatever area it is given, so it says nothing about the widget's own
/// height and is not counted.
fn painted_rows(buffer: &Buffer, width: u16, height: u16) -> u16 {
    let mut last = 0;
    for y in 0..height {
        for x in 0..width {
            let sym = buffer.get(x, y).map(|c| c.symbol).unwrap_or(' ');
            if x == 0 && sym == '┃' {
                continue;
            }
            if is_ink(sym) {
                last = y + 1;
                break;
            }
        }
    }
    last
}

fn row_text(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .filter(|&c| c != '\0')
        .collect()
}

fn screen_text(buffer: &Buffer, width: u16, height: u16) -> String {
    (0..height)
        .map(|y| row_text(buffer, y, width))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Render at exactly `height` rows; every promised row must be used, and
/// everything listed in `must_show` must be on screen.
fn assert_height_agrees<V: View>(label: &str, view: &V, height: u16, must_show: &[&str]) {
    let buffer = render_into(view, W, height);
    let painted = painted_rows(&buffer, W, height);
    let screen = screen_text(&buffer, W, height);
    assert_eq!(
        painted, height,
        "{label}: height() says {height} rows but render painted {painted}:\n{screen}"
    );
    for text in must_show {
        assert!(
            screen.contains(text),
            "{label}: {text:?} was clipped at height() = {height}:\n{screen}"
        );
    }
}

// ==================== Alert ====================

#[test]
fn alert_height_matches_what_each_variant_paints() {
    for variant in [
        AlertVariant::Filled,
        AlertVariant::Outlined,
        AlertVariant::Minimal,
    ] {
        let plain = Alert::new("Saved").variant(variant);
        assert_height_agrees(&format!("{variant:?}"), &plain, plain.height(), &["Saved"]);

        let titled = Alert::new("Saved").title("Done").variant(variant);
        assert_height_agrees(
            &format!("{variant:?} + title"),
            &titled,
            titled.height(),
            &["Done", "Saved"],
        );
    }
}

#[test]
fn outlined_alert_is_one_row_per_line_of_text() {
    // Outlined is documented as "only left border accent": no top or bottom
    // border, so a message alone is one row and a title adds one.
    assert_eq!(Alert::new("m").variant(AlertVariant::Outlined).height(), 1);
    assert_eq!(
        Alert::new("m")
            .title("t")
            .variant(AlertVariant::Outlined)
            .height(),
        2
    );
}

// ==================== Callout ====================

#[test]
fn callout_height_matches_what_each_variant_paints() {
    for variant in [
        CalloutVariant::Filled,
        CalloutVariant::LeftBorder,
        CalloutVariant::Minimal,
    ] {
        let one = Callout::new("Body text").title("Heads up").variant(variant);
        assert_height_agrees(
            &format!("{variant:?}"),
            &one,
            one.height(),
            &["Heads up", "Body text"],
        );

        let three = Callout::new("one\ntwo\nthree")
            .title("Heads up")
            .variant(variant);
        assert_height_agrees(
            &format!("{variant:?} x3"),
            &three,
            three.height(),
            &["Heads up", "one", "two", "three"],
        );

        let collapsed = Callout::new("Body text")
            .title("Heads up")
            .variant(variant)
            .collapsible(true)
            .expanded(false);
        assert_height_agrees(
            &format!("{variant:?} collapsed"),
            &collapsed,
            collapsed.height(),
            &["Heads up"],
        );
    }
}

#[test]
fn filled_callout_has_no_top_or_bottom_border_rows() {
    // Filled is "filled background with accent border": the accent is the bar
    // down the left edge, so the rows are the title and the content only.
    assert_eq!(
        Callout::new("x").variant(CalloutVariant::Filled).height(),
        2
    );
    assert_eq!(
        Callout::new("a\nb\nc")
            .variant(CalloutVariant::Filled)
            .height(),
        4
    );
}

// ==================== Card ====================

const BORDER_GLYPHS: &[char] = &[
    '┌', '┐', '└', '┘', '╭', '╮', '╰', '╯', '│', '├', '┤', '═', '║', '╔', '╗', '╚', '╝', '┃', '┏',
    '┓', '┗', '┛',
];

fn assert_no_border(label: &str, buffer: &Buffer, width: u16, height: u16) {
    let screen = screen_text(buffer, width, height);
    for y in 0..height {
        for x in 0..width {
            let sym = buffer.get(x, y).unwrap().symbol;
            assert!(
                !BORDER_GLYPHS.contains(&sym),
                "{label}: border glyph {sym:?} at ({x}, {y}):\n{screen}"
            );
        }
    }
    // A full-width row of `─` with corners would be a top/bottom border.
    for y in [0, height - 1] {
        let row = row_text(buffer, y, width);
        assert!(
            !row.chars().all(|c| c == '─'),
            "{label}: row {y} is a horizontal border:\n{screen}"
        );
    }
}

#[test]
fn flat_card_draws_no_border_when_set_with_variant() {
    let c = card()
        .title("Flat")
        .subtitle("No border, minimal style")
        .body(Text::new("Content goes here"))
        .variant(CardVariant::Flat);
    let buffer = render_into(&c, 30, 6);
    assert_no_border("variant(Flat)", &buffer, 30, 6);
    let screen = screen_text(&buffer, 30, 6);
    // With no border the title sits on the first row.
    assert!(
        row_text(&buffer, 0, 30).trim_start().starts_with("Flat"),
        "{screen}"
    );
    assert!(screen.contains("Content goes here"), "{screen}");
}

#[test]
fn flat_card_draws_no_border_when_set_with_flat() {
    let c = card()
        .title("Flat")
        .body(Text::new("Content goes here"))
        .flat();
    let buffer = render_into(&c, 30, 5);
    assert_no_border("flat()", &buffer, 30, 5);
}

#[test]
fn flat_card_still_takes_an_explicit_border_set_after_it() {
    // Choosing Flat drops the border; asking for one afterwards still wins.
    let c = card()
        .title("T")
        .variant(CardVariant::Flat)
        .border_style(revue::widget::BorderType::Rounded);
    let buffer = render_into(&c, 20, 4);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
}

// ==================== EmptyState ====================

#[test]
fn empty_state_height_matches_what_each_variant_paints() {
    for variant in [
        EmptyStateVariant::Full,
        EmptyStateVariant::Compact,
        EmptyStateVariant::Minimal,
    ] {
        for icon in [true, false] {
            for desc in [false, true] {
                for action in [false, true] {
                    if variant == EmptyStateVariant::Minimal && (desc || action) {
                        // Minimal is the title alone by design.
                        continue;
                    }
                    let mut e = EmptyState::new("Nothing here").variant(variant).icon(icon);
                    let mut show = vec!["Nothing here"];
                    if desc {
                        e = e.description("Try again later");
                        show.push("Try again later");
                    }
                    if action {
                        e = e.action("Retry");
                        show.push("Retry");
                    }
                    assert_height_agrees(
                        &format!("{variant:?} icon={icon} desc={desc} action={action}"),
                        &e,
                        e.height(),
                        &show,
                    );
                }
            }
        }
    }
}

// ==================== BigText ====================

#[test]
fn bigtext_figlet_paints_within_its_height() {
    // The Figlet fonts pad every glyph to the font's full height, so trailing
    // rows can be blank; what matters is that nothing spills past `height()`.
    for tier in 1..=6u8 {
        let b = BigText::new("Hi 42", tier).force_figlet(true);
        let h = b.height();
        // Room to spare, so a short helper cannot hide clipping.
        let buffer = render_into(&b, 80, h + 6);
        let painted = painted_rows(&buffer, 80, h + 6);
        assert!(
            painted > 0 && painted <= h,
            "tier {tier}: height() = {h}, painted {painted}:\n{}",
            screen_text(&buffer, 80, h + 6)
        );
    }
}

#[test]
fn figlet_fonts_render_as_many_lines_as_font_height_says() {
    for font in [
        FigletFont::Block,
        FigletFont::Slant,
        FigletFont::Banner,
        FigletFont::Small,
        FigletFont::Mini,
    ] {
        let lines = figlet_with_font("Hi 42", font).lines().count();
        assert_eq!(
            lines,
            revue::utils::font_height(font),
            "{font:?} renders {lines} lines"
        );
    }
}

// ==================== Digits ====================

#[test]
fn digits_height_matches_the_glyph_rows_it_paints() {
    for style in [
        DigitStyle::Block,
        DigitStyle::Thin,
        DigitStyle::Ascii,
        DigitStyle::Braille,
    ] {
        let d = Digits::new(8080).style(style);
        let h = d.height() as u16;
        let buffer = render_into(&d, 60, h);
        let screen = screen_text(&buffer, 60, h);
        for y in 0..h {
            assert!(
                row_text(&buffer, y, 60).chars().any(is_ink),
                "{style:?}: row {y} of {h} is blank:\n{screen}"
            );
        }
    }
}

// ==================== StatusBar ====================

#[test]
fn status_section_width_counts_terminal_columns() {
    assert_eq!(StatusSection::new("main").width(), 4);
    // Hangul is two columns per character.
    assert_eq!(StatusSection::new("한글").width(), 4);
    assert_eq!(StatusSection::new("한").min_width(5).width(), 5);
}

#[test]
fn status_bar_paints_wide_text_at_the_width_it_reports() {
    let bar = StatusBar::new()
        .left(StatusSection::new("한글"))
        .left(StatusSection::new("X"));
    let buffer = render_into(&bar, 20, 1);
    let row = row_text(&buffer, 0, 20);
    assert!(row.starts_with("한글"), "{row:?}");
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '글');
    // The next section starts after the four columns the first one takes.
    let x = (0..20)
        .find(|&x| buffer.get(x, 0).unwrap().symbol == 'X')
        .expect("second section painted");
    assert!(x >= 4, "second section overlaps the first at column {x}");

    let right = StatusBar::new().right(StatusSection::new("한글"));
    let buffer = render_into(&right, 20, 1);
    let row = row_text(&buffer, 0, 20);
    let x = (0..20)
        .find(|&x| buffer.get(x, 0).unwrap().symbol == '한')
        .expect("right section painted");
    assert_eq!(buffer.get(x + 2, 0).unwrap().symbol, '글', "{row:?}");
    assert!(x + 4 <= 20, "right section runs off the edge: {row:?}");
}

// ==================== measure ====================
//
// `View::measure` is how a content-sized stack sizes these widgets, so it must
// answer with the same helpers the tests above hold to what `render` paints.

#[test]
fn measure_answers_with_the_height_helpers() {
    const H: u16 = 30;
    for variant in [
        AlertVariant::Filled,
        AlertVariant::Outlined,
        AlertVariant::Minimal,
    ] {
        let a = Alert::new("Saved").title("Done").variant(variant);
        assert_eq!(a.measure(W, H), Some((W, a.height())), "{variant:?}");
    }
    let mut dismissed = Alert::new("Saved");
    dismissed.dismiss();
    assert_eq!(dismissed.measure(W, H), Some((0, 0)));

    for variant in [
        CalloutVariant::Filled,
        CalloutVariant::LeftBorder,
        CalloutVariant::Minimal,
    ] {
        let c = Callout::new("one\ntwo").title("T").variant(variant);
        assert_eq!(c.measure(W, H), Some((W, c.height())), "{variant:?}");
    }

    for variant in [
        EmptyStateVariant::Full,
        EmptyStateVariant::Compact,
        EmptyStateVariant::Minimal,
    ] {
        let e = EmptyState::new("Nothing")
            .description("Try later")
            .action("Retry")
            .variant(variant);
        assert_eq!(e.measure(W, H), Some((W, e.height())), "{variant:?}");
    }

    for tier in 1..=6u8 {
        let b = BigText::new("Hi 42", tier).force_figlet(true);
        assert_eq!(
            b.measure(80, H).map(|m| m.1),
            Some(b.height()),
            "tier {tier}"
        );
    }

    for style in [
        DigitStyle::Block,
        DigitStyle::Thin,
        DigitStyle::Ascii,
        DigitStyle::Braille,
    ] {
        let d = Digits::new(8080).style(style);
        assert_eq!(
            d.measure(80, H).map(|m| m.1),
            Some(d.height() as u16),
            "{style:?}"
        );
    }

    let bar = StatusBar::new().left(StatusSection::new("main"));
    assert_eq!(bar.measure(W, H), Some((W, 1)));
    assert_eq!(bar.height(2).measure(W, H), Some((W, 2)));
}

#[test]
fn measure_never_exceeds_what_it_is_offered() {
    let alert = Alert::new("Saved").title("Done");
    assert_eq!(alert.measure(10, 2), Some((10, 2)));
    let digits = Digits::new(8080);
    let (w, h) = digits.measure(4, 2).unwrap();
    assert!(w <= 4 && h <= 2);
    let big = BigText::new("Hi", 1).force_figlet(true);
    let (w, h) = big.measure(3, 1).unwrap();
    assert!(w <= 3 && h <= 1);
}
