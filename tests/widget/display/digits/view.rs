//! Digits against plain models: the formatted string, the Block glyph rows
//! on the screen with their colors, the label row, placement and the
//! stylesheet path.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{DigitStyle, Digits};

use super::block_lines;
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const STYLES: [DigitStyle; 4] = [
    DigitStyle::Block,
    DigitStyle::Thin,
    DigitStyle::Ascii,
    DigitStyle::Braille,
];

fn render(d: &Digits, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    d.render(&mut RenderContext::new(
        &mut buffer,
        Rect::new(0, 0, width, height),
    ));
    buffer
}

fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

/// Group the integer digits of `int` by three from the right.
fn grouped(int: &str, sep: char) -> String {
    let n = int.chars().count();
    let mut out = String::new();
    for (i, c) in int.chars().enumerate() {
        if i > 0 && (n - i).is_multiple_of(3) {
            out.push(sep);
        }
        out.push(c);
    }
    out
}

/// What `format_value` shows for a plain number `sign int [. frac]`:
/// zero padding goes after the sign; blank padding goes in front of the
/// grouped number.
fn format_model(
    negative: bool,
    int: &str,
    frac: Option<&str>,
    sep: Option<char>,
    min_width: Option<usize>,
    zeros: bool,
) -> String {
    let sign = if negative { "-" } else { "" };
    let tail = frac.map(|f| format!(".{f}")).unwrap_or_default();
    let raw_len = sign.len() + int.len() + tail.len();
    let int = match (zeros, min_width) {
        (true, Some(w)) if raw_len < w => format!("{}{int}", "0".repeat(w - raw_len)),
        _ => int.to_string(),
    };
    let int = match sep {
        Some(s) => grouped(&int, s),
        None => int,
    };
    let shown = format!("{sign}{int}{tail}");
    match (zeros, min_width) {
        (false, Some(w)) if shown.chars().count() < w => {
            format!("{}{shown}", " ".repeat(w - shown.chars().count()))
        }
        _ => shown,
    }
}

#[test]
fn zero_areas_draw_nothing() {
    for style in STYLES {
        assert_zero_areas_draw_nothing(&Digits::new("-12:34.5").style(style));
        assert_zero_areas_draw_nothing(&Digits::new(42).style(style).prefix("$").fg(Color::RED));
    }
    assert_zero_areas_draw_nothing(&Digits::new(""));
}

#[test]
fn same_when_offset() {
    for style in STYLES {
        assert_same_when_offset(Digits::new("12:34").style(style), 24, 6);
        assert_same_when_offset(
            Digits::new(-7)
                .style(style)
                .suffix(" kg")
                .fg(Color::GREEN)
                .bg(Color::BLUE),
            12,
            7,
        );
    }
}

#[test]
fn empty_value_draws_nothing_and_measures_its_height() {
    for style in STYLES {
        let d = Digits::new("").style(style);
        assert!(d.render_lines().iter().all(|l| l.is_empty()));
        let buffer = render(&d, 10, 6);
        assert!(
            (0..6).all(|y| row(&buffer, y).trim().is_empty()),
            "{style:?}"
        );
        assert_eq!(d.measure(10, 10), Some((0, d.height() as u16)));
    }
    // An empty value with a label still draws the label
    let d = Digits::new("").suffix("%");
    assert_eq!(row(&render(&d, 6, 6), 5), "%     ");
}

#[test]
fn rows_below_the_glyphs_are_left_alone_and_the_area_clips() {
    // A five-row display in a three-row area draws its top three rows
    let d = Digits::new(88);
    let buffer = render(&d, 20, 3);
    for (y, line) in block_lines("88").iter().take(3).enumerate() {
        assert_eq!(row(&buffer, y as u16).trim_end(), line.trim_end());
    }
    // A narrow area cuts the rows, never wraps them
    let buffer = render(&d, 5, 6);
    assert_eq!(row(&buffer, 0), "███ █");
    assert_eq!(row(&buffer, 5).trim(), "");
}

#[test]
fn stylesheet_color_reaches_the_glyphs_and_the_builder_outranks_it() {
    let draw = |d: Digits| {
        let mut h =
            PipelineHarness::with_css("#d { color: #00ff00; }", 12, 6).dom_from_render(true);
        h.draw(&d.element_id("d"));
        h.buffer().clone()
    };
    let green = Color::rgb(0, 255, 0);
    let buffer = draw(Digits::new(42));
    for y in 0..5 {
        for x in 0..8 {
            assert_eq!(buffer.get(x, y).unwrap().fg, Some(green), "({x}, {y})");
        }
    }
    let buffer = draw(Digits::new(42).fg(Color::RED));
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn measure_matches_the_painted_box() {
    for style in STYLES {
        for value in ["7", "-12.5", "12:34:56", "1 0"] {
            for label in [None, Some(("$", "")), Some(("", " units long"))] {
                let mut d = Digits::new(value).style(style);
                if let Some((p, s)) = label {
                    d = d.prefix(p).suffix(s);
                }
                let (w, h) = d.measure(200, 20).unwrap();
                let buffer = render(&d, 60, 10);
                let ink_w = (0..10)
                    .map(|y| row(&buffer, y).trim_end().chars().count())
                    .max()
                    .unwrap();
                let ink_h = (0..10)
                    .filter(|&y| !row(&buffer, y).trim().is_empty())
                    .max()
                    .map_or(0, |y| y + 1);
                assert_eq!(
                    (w as usize, h),
                    (ink_w, ink_h),
                    "{style:?} {value:?} {label:?}"
                );
                // A small limit caps the size
                let (sw, sh) = d.measure(3, 2).unwrap();
                assert!(sw <= 3 && sh <= 2);
            }
        }
    }
}

fn number() -> impl Strategy<Value = (bool, String, Option<String>)> {
    (
        any::<bool>(),
        "[1-9][0-9]{0,9}|0",
        prop::option::of("[0-9]{1,4}"),
    )
}

proptest! {
    /// `format_value` against the model, for whole and decimal numbers of
    /// either sign with any separator, width and padding choice.
    #[test]
    fn formats_like_the_model(
        (negative, int, frac) in number(),
        // A separator or a width, not both: with both, zero padding counts
        // the digits and blank padding counts the separators too (#1066),
        // so the model does not pick a side.
        (sep, min_width) in prop_oneof![
            Just((None, None)),
            prop::sample::select(vec![',', '.', ' ', '_']).prop_map(|s| (Some(s), None)),
            (0usize..16).prop_map(|w| (None, Some(w))),
        ],
        zeros in prop::option::of(any::<bool>()),
    ) {
        let value = format!(
            "{}{int}{}",
            if negative { "-" } else { "" },
            frac.as_deref().map(|f| format!(".{f}")).unwrap_or_default()
        );
        let mut d = Digits::new(&value);
        if let Some(s) = sep {
            d = d.separator(s);
        }
        if let Some(w) = min_width {
            d = d.min_width(w);
        }
        if let Some(z) = zeros {
            d = d.leading_zeros(z);
        }
        let want = format_model(
            negative,
            &int,
            frac.as_deref(),
            sep,
            min_width,
            zeros != Some(false),
        );
        prop_assert_eq!(d.format_value(), want);
    }

    /// The Block rows on the screen are the model's rows for the shown
    /// string, in the builder's colors, cut to the area; the label row
    /// follows them.
    #[test]
    fn draws_the_block_model(
        shown in "[0-9:. -]{0,8}",
        fg in prop::option::of(Just(Color::RED)),
        bg in prop::option::of(Just(Color::BLUE)),
        label in prop::option::of("[a-z$%]{1,4}"),
        aw in 0u16..40,
        ah in 0u16..8,
    ) {
        let mut d = Digits::new(&shown);
        if let Some(c) = fg {
            d = d.fg(c);
        }
        if let Some(c) = bg {
            d = d.bg(c);
        }
        if let Some(l) = &label {
            d = d.suffix(l.as_str());
        }
        let mut buffer = Buffer::new(aw + 2, ah + 2);
        d.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, aw, ah)));
        let lines = block_lines(&shown);
        for y in 0..ah + 2 {
            let want: Vec<char> = if y < ah && y < 5 {
                lines[usize::from(y)].chars().collect()
            } else if y < ah && y == 5 {
                label
                    .as_ref()
                    .map(|l| format!("{shown}{l}").chars().collect())
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
            for x in 0..aw + 2 {
                let cell = buffer.get(x, y).unwrap();
                let inside = x < aw && usize::from(x) < want.len();
                let symbol = if inside { want[usize::from(x)] } else { ' ' };
                prop_assert_eq!(cell.symbol, symbol, "({}, {})", x, y);
                if inside && y < 5 {
                    prop_assert_eq!(cell.fg, fg, "fg ({}, {})", x, y);
                    prop_assert_eq!(cell.bg, bg, "bg ({}, {})", x, y);
                } else if inside {
                    // The label takes the fg; its bg is left out (#1066:
                    // the label ignores `bg`).
                    prop_assert_eq!(cell.fg, fg, "label fg ({}, {})", x, y);
                } else {
                    prop_assert_eq!((cell.fg, cell.bg), (None, None), "({}, {})", x, y);
                }
            }
        }
    }

    /// Every style draws each character as its glyph width plus one blank
    /// column, in `height` rows, and `measure` covers the widest row.
    #[test]
    fn every_style_lays_glyphs_side_by_side(
        style in prop::sample::select(STYLES.to_vec()),
        shown in "[0-9:. x-]{0,10}",
    ) {
        let d = Digits::new(&shown).style(style);
        let lines = d.render_lines();
        prop_assert_eq!(lines.len(), d.height());
        let narrow = |c: char| matches!(c, ':' | '.');
        let want: usize = shown
            .chars()
            .map(|c| if narrow(c) { 2 } else { d.digit_width() + 1 })
            .sum();
        for line in &lines {
            prop_assert_eq!(line.chars().count(), want);
        }
        let widest = lines.iter().map(|l| l.trim_end().chars().count()).max().unwrap_or(0);
        prop_assert_eq!(d.measure(500, 50), Some((widest as u16, d.height() as u16)));
    }
}

/// A style with its colon, dot and minus rows.
type Marks = (
    DigitStyle,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
);

#[test]
fn each_style_draws_its_own_colon_dot_and_minus() {
    // (style, colon rows, dot rows, minus rows), written out here
    let cases: [Marks; 4] = [
        (
            DigitStyle::Block,
            &[" ", "█", " ", "█", " "],
            &[" ", " ", " ", " ", "█"],
            &["   ", "   ", "███", "   ", "   "],
        ),
        (
            DigitStyle::Thin,
            &[" ", "●", " ", "●", " "],
            &[" ", " ", " ", " ", "●"],
            &["   ", "   ", "───", "   ", "   "],
        ),
        (
            DigitStyle::Ascii,
            &[" ", "o", " ", "o", " "],
            &[" ", " ", " ", " ", "o"],
            &["   ", "   ", "---", "   ", "   "],
        ),
        (
            DigitStyle::Braille,
            &[" ", "⠂", "⠂", " "],
            &[" ", " ", " ", "⠂"],
            &["  ", "⠤⠤", "  ", "  "],
        ),
    ];
    for (style, colon, dot, minus) in cases {
        for (c, want) in [(":", colon), (".", dot), ("-", minus)] {
            let lines = Digits::new(c).style(style).render_lines();
            let want: Vec<String> = want.iter().map(|r| format!("{r} ")).collect();
            assert_eq!(lines, want, "{style:?} {c:?}");
        }
    }
}
