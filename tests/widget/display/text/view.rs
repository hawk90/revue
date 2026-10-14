//! Text placement, and a model of where each alignment puts the characters

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::{Color, FontWeight, Style as Css, TextDecoration};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Alignment, Text};

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
    // The stylesheet `color` and `background` are not checked here: the
    // justified path paints only the builder's colors (draft-rt-1).
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
