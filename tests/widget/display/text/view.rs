//! Text placement, and a model of where each alignment puts the characters

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::{Color, FontWeight, Style as Css, TextDecoration};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{vstack, Alignment, RichText, Text};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const ALIGNMENTS: [Alignment; 4] = [
    Alignment::Left,
    Alignment::Center,
    Alignment::Right,
    Alignment::Justify,
];

/// `text` drawn into a one-row area `width` wide, optionally under `css`
fn draw(text: &Text, width: u16, css: Option<&Css>) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    let area = Rect::new(0, 0, width, 1);
    let mut ctx = match css {
        Some(css) => RenderContext::with_style(&mut buffer, area, css),
        None => RenderContext::new(&mut buffer, area),
    };
    text.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, 0).unwrap().symbol)
        .collect()
}

#[test]
fn test_text_zero_areas_and_offset() {
    for align in ALIGNMENTS {
        let text = Text::new("Hi there you")
            .fg(Color::RED)
            .bg(Color::BLUE)
            .bold()
            .align(align);
        assert_zero_areas_draw_nothing(&text);
        assert_same_when_offset(text, 16, 1);
    }
}

#[test]
fn test_text_hidden_draws_nothing_in_any_alignment() {
    let mut css = Css::default();
    css.visual.visible = false;
    for align in ALIGNMENTS {
        let buffer = draw(&Text::new("ab cd").align(align), 9, Some(&css));
        assert_eq!(row(&buffer), "         ", "{align:?}");
    }
}

#[test]
fn test_text_takes_the_stylesheet_colors_under_the_builders() {
    let mut css = Css::default();
    css.visual.color = Color::MAGENTA;
    css.visual.background = Color::rgb(1, 2, 3);
    for align in [Alignment::Left, Alignment::Center, Alignment::Right] {
        let plain = draw(&Text::new("ab").align(align), 6, Some(&css));
        let x = (0..6)
            .find(|&x| plain.get(x, 0).unwrap().symbol == 'a')
            .unwrap();
        let cell = plain.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::MAGENTA), Some(Color::rgb(1, 2, 3))),
            "{align:?}"
        );

        let builder = Text::new("ab").fg(Color::RED).bg(Color::BLUE).align(align);
        let cell = *draw(&builder, 6, Some(&css)).get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::RED), Some(Color::BLUE)),
            "{align:?}"
        );
    }
}

#[test]
fn test_text_justify_paints_the_words_with_the_builder_style() {
    let text = Text::new("ab cd ef")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .bold()
        .italic()
        .underline()
        .dim()
        .reverse()
        .align(Alignment::Justify);
    let buffer = draw(&text, 12, None);
    assert_eq!(row(&buffer), "ab   cd   ef");
    for x in [0, 1, 5, 6, 10, 11] {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(
            (cell.fg, cell.bg),
            (Some(Color::RED), Some(Color::BLUE)),
            "{x}"
        );
        assert_eq!(
            cell.modifier,
            Modifier::BOLD
                | Modifier::ITALIC
                | Modifier::UNDERLINE
                | Modifier::DIM
                | Modifier::REVERSE,
            "{x}"
        );
    }
}

#[test]
fn test_text_justify_takes_the_stylesheet_attributes() {
    let mut css = Css::default();
    css.visual.font_weight = FontWeight::Bold;
    css.visual.text_decoration = TextDecoration {
        underline: true,
        line_through: true,
    };
    let text = Text::new("ab cd").align(Alignment::Justify);
    let buffer = draw(&text, 9, Some(&css));
    assert_eq!(row(&buffer), "ab     cd");
    assert_eq!(
        buffer.get(8, 0).unwrap().modifier,
        Modifier::BOLD | Modifier::UNDERLINE | Modifier::CROSSED_OUT
    );
}

#[test]
fn test_text_justify_falls_back_to_left() {
    // One word, and words that fill the width, are drawn left-aligned
    let one = Text::new("  word  ").align(Alignment::Justify);
    assert_eq!(row(&draw(&one, 10, None)), "  word    ");
    let full = Text::new("ab cd").align(Alignment::Justify);
    assert_eq!(row(&draw(&full, 4, None)), "ab c");
    assert_eq!(row(&draw(&full, 5, None)), "ab cd");
}

/// Space-separated lowercase words, with runs of spaces and spaces at the ends
fn content() -> impl Strategy<Value = String> {
    (
        " {0,2}",
        prop::collection::vec(("[a-z]{1,5}", " {1,3}"), 0..6),
        " {0,2}",
    )
        .prop_map(|(lead, words, trail)| {
            let mut s = lead;
            for (i, (w, sep)) in words.iter().enumerate() {
                if i > 0 {
                    s.push_str(sep);
                }
                s.push_str(w);
            }
            s.push_str(&trail);
            s
        })
}

/// Where a plain model puts the ASCII `content` in a row `width` wide
fn model(content: &str, align: Alignment, width: usize) -> String {
    let mut row = vec![' '; width];
    let mut put = |x: usize, s: &str| {
        for (i, ch) in s.chars().enumerate() {
            if x + i < width {
                row[x + i] = ch;
            }
        }
    };
    let left = content.len();
    match align {
        Alignment::Left => put(0, content),
        Alignment::Center => put(width.saturating_sub(left) / 2, content),
        Alignment::Right => put(width.saturating_sub(left), content),
        Alignment::Justify => {
            let words: Vec<&str> = content.split_whitespace().collect();
            let letters: usize = words.iter().map(|w| w.len()).sum();
            if words.len() < 2 || letters >= width {
                put(0, content);
            } else {
                // The spaces go evenly between the words; the first gaps
                // take one more while the remainder lasts.
                let gaps = words.len() - 1;
                let (base, extra) = ((width - letters) / gaps, (width - letters) % gaps);
                let mut x = 0;
                for (i, w) in words.iter().enumerate() {
                    put(x, w);
                    x += w.len() + base + usize::from(i < extra);
                }
            }
        }
    }
    row.into_iter().collect()
}

fn alignment() -> impl Strategy<Value = Alignment> {
    prop::sample::select(ALIGNMENTS.to_vec())
}

proptest! {
    /// Every alignment puts the characters where the plain model does,
    /// including justify's even spacing and its fallback to the left when
    /// there is one word or the words fill the row.
    #[test]
    fn prop_text_alignment_follows_the_model(
        content in content(),
        align in alignment(),
        width in 1u16..30,
    ) {
        let buffer = draw(&Text::new(content.clone()).align(align), width, None);
        prop_assert_eq!(row(&buffer), model(&content, align, width as usize));
    }

    /// A single-line text measures one row as wide as its characters
    /// (2 columns for 한 and 日), capped by what is offered.
    #[test]
    fn prop_text_measure_is_its_width_capped(
        chars in prop::collection::vec(prop::sample::select(vec!['a', 'Z', ' ', '.', '한', '日']), 0..20),
        max_w in 0u16..50,
        max_h in 0u16..3,
    ) {
        let content: String = chars.iter().collect();
        let width: u16 = chars.iter().map(|c| if c.is_ascii() { 1 } else { 2 }).sum();
        let text = Text::new(content);
        prop_assert_eq!(text.measure(max_w, max_h), Some((width.min(max_w), 1.min(max_h))));
    }
}

// =========================================================================
// Text with several lines (#1068), and zero-width chars (#1330)
// =========================================================================

/// The rows of a `w`x`h` render, blanks kept, continuations left out.
fn rows(view: &dyn View, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    view.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)));
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| buffer.get(x, y).unwrap())
                .filter(|cell| !cell.is_continuation())
                .map(|cell| cell.symbol)
                .collect()
        })
        .collect()
}

#[test]
fn test_text_measures_each_line() {
    assert_eq!(Text::new("ab\ncd").measure(10, 10), Some((2, 2)));
    assert_eq!(Text::new("a\nlonger\nb").measure(10, 10), Some((6, 3)));
    // Columns, not chars; a trailing newline is an empty last row.
    assert_eq!(Text::new("한글\nab").measure(10, 10), Some((4, 2)));
    assert_eq!(Text::new("ab\n").measure(10, 10), Some((2, 2)));
    assert_eq!(Text::new("\n\n").measure(10, 10), Some((0, 3)));
    // Capped by the offer on both axes.
    assert_eq!(Text::new("abcd\ncd\nef").measure(3, 2), Some((3, 2)));
    assert_eq!(Text::new("ab\ncd").measure(0, 0), Some((0, 0)));
    // A control char takes no column.
    assert_eq!(Text::new("a\tb\u{1}\nc").measure(10, 10), Some((2, 2)));
}

#[test]
fn test_text_lines_each_get_a_row_in_a_stack() {
    let stack = vstack().child(Text::new("ab\ncd")).child(Text::new("ef"));
    assert_eq!(rows(&stack, 3, 3), ["ab ", "cd ", "ef "]);
}

#[test]
fn test_text_aligns_each_line_on_its_own() {
    let text = |align| Text::new("ab\ncdef\n").align(align);
    assert_eq!(
        rows(&text(Alignment::Center), 8, 3),
        ["   ab   ", "  cdef  ", "        "]
    );
    assert_eq!(
        rows(&text(Alignment::Right), 8, 3),
        ["      ab", "    cdef", "        "]
    );
    assert_eq!(
        rows(&text(Alignment::Left), 8, 3),
        ["ab      ", "cdef    ", "        "]
    );
    // A line wider than the area starts at its left edge, without pushing
    // the next one; rows past the area are cut.
    assert_eq!(
        rows(&Text::new("abcdef\nxy").align(Alignment::Right), 4, 2),
        ["abcd", "  xy"]
    );
    assert_eq!(
        rows(&Text::new("ab\ncd\nef").align(Alignment::Center), 4, 2),
        [" ab ", " cd "]
    );
}

#[test]
fn test_text_justifies_each_line_on_its_own() {
    let text = Text::new("a b\nc d e\nword").align(Alignment::Justify);
    assert_eq!(rows(&text, 7, 3), ["a     b", "c  d  e", "word   "]);
}

#[test]
fn test_text_justify_takes_the_stylesheet_colors() {
    let mut css = Css::default();
    css.visual.color = Color::MAGENTA;
    css.visual.background = Color::rgb(1, 2, 3);
    let text = Text::new("ab cd").align(Alignment::Justify);
    let buffer = draw(&text, 9, Some(&css));
    // The words and the gap between them, up to the last column
    for x in 0..9 {
        let cell = buffer.get(x, 0).unwrap();
        assert_eq!(cell.bg, Some(Color::rgb(1, 2, 3)), "bg at {x}");
    }
    for x in [0, 1, 7, 8] {
        assert_eq!(
            buffer.get(x, 0).unwrap().fg,
            Some(Color::MAGENTA),
            "fg at {x}"
        );
    }
    // The builder still outranks the stylesheet.
    let built = Text::new("ab cd")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .align(Alignment::Justify);
    let cell = *draw(&built, 9, Some(&css)).get(0, 0).unwrap();
    assert_eq!((cell.fg, cell.bg), (Some(Color::RED), Some(Color::BLUE)));
}

#[test]
fn test_text_justify_lays_out_in_columns() {
    let text = Text::new("한 a").align(Alignment::Justify);
    let buffer = draw(&text, 6, None);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 'a');
    // A control or zero-width char inside a word takes no cell.
    let buffer = draw(
        &Text::new("a\u{1}b\u{200b}c d").align(Alignment::Justify),
        6,
        None,
    );
    assert_eq!(row(&buffer), "abc  d");
}

#[test]
fn test_zero_width_chars_take_no_cell() {
    // At the end of the text it used to land past the measured width, in the
    // middle in the next char's cell.
    let end = Text::new("e\u{301}");
    assert_eq!(end.measure(10, 1), Some((1, 1)));
    assert_eq!(rows(&end, 3, 1), ["e  "]);
    assert_eq!(rows(&Text::new("a\u{200b}b"), 3, 1), ["ab "]);
    assert_eq!(rows(&Text::new("\u{301}a"), 2, 1), ["a "]);
    assert_eq!(rows(&Text::new("\u{200b}"), 2, 1), ["  "]);

    let rich = RichText::new().text("e\u{301}").text("\u{200b}x");
    assert_eq!(rich.measure(10, 1), Some((2, 1)));
    assert_eq!(rows(&rich, 3, 1), ["ex "]);
    // Beside a wide glyph, at the right edge of the area
    assert_eq!(rows(&Text::new("한\u{301}a"), 3, 1), ["한a"]);
}
