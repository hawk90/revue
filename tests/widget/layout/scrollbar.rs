//! The standalone scrollbar: its cells for any content, offset and length,
//! and the column (or row) it always takes (#954)

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{Color, Style, VisualStyle};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Fill, Scrollbar};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn draw(bar: &Scrollbar, w: u16, h: u16) -> Buffer {
    let mut b = Buffer::new(w, h);
    bar.render(&mut RenderContext::new(&mut b, Rect::new(0, 0, w, h)));
    b
}

fn column(b: &Buffer) -> String {
    (0..b.height())
        .map(|y| b.get(0, y).unwrap().symbol)
        .collect()
}

#[test]
fn test_scrollbar_draws_the_thumb_at_the_offset() {
    // 40 rows, 10 shown, a 10-row bar: a 2-row thumb (a quarter), moving
    // from the top at offset 0 to the bottom at the last page and past it
    let bar = |offset| {
        Scrollbar::vertical()
            .content_length(40)
            .viewport_length(10)
            .offset(offset)
    };
    assert_eq!(column(&draw(&bar(0), 1, 10)), "██││││││││");
    assert_eq!(column(&draw(&bar(15), 1, 10)), "││││██││││");
    assert_eq!(column(&draw(&bar(30), 1, 10)), "││││││││██");
    assert_eq!(column(&draw(&bar(1000), 1, 10)), "││││││││██");
}

#[test]
fn test_scrollbar_keeps_its_column_with_nothing_to_scroll() {
    // Everything fits: the track alone, in the same column, so the content
    // beside it keeps its width
    let bar = Scrollbar::vertical().content_length(5).viewport_length(10);
    assert_eq!(column(&draw(&bar, 1, 4)), "││││");
    assert_eq!(bar.measure(30, 8), Some((1, 8)));
    assert_eq!(bar.fills(), Fill::HEIGHT);
    let empty = Scrollbar::vertical();
    assert_eq!(column(&draw(&empty, 1, 3)), "│││");
    assert_eq!(empty.measure(0, 8), Some((0, 8)));
}

#[test]
fn test_scrollbar_viewport_defaults_to_its_length() {
    // Without viewport_length the bar is taken to be as tall as the view:
    // 20 rows, 5 shown, a 5-row bar gives a 1-row thumb
    let bar = Scrollbar::vertical().content_length(20).offset(15);
    assert_eq!(column(&draw(&bar, 1, 5)), "││││█");
}

#[test]
fn test_horizontal_scrollbar_runs_along_a_row() {
    let bar = Scrollbar::horizontal()
        .content_length(30)
        .viewport_length(10)
        .offset(10);
    let b = draw(&bar, 9, 1);
    let row: String = (0..9).map(|x| b.get(x, 0).unwrap().symbol).collect();
    assert_eq!(row, "───███───");
    assert_eq!(bar.measure(9, 4), Some((9, 1)));
    assert_eq!(bar.fills(), Fill::WIDTH);
}

#[test]
fn test_scrollbar_colors_from_builders_then_the_stylesheet() {
    let bar = Scrollbar::vertical().content_length(4).viewport_length(2);
    let b = draw(&bar, 1, 2);
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(b.get(0, 1).unwrap().fg, Some(Color::rgb(64, 64, 64)));
    // The stylesheet's color and background
    let style = Style {
        visual: VisualStyle {
            color: Color::CYAN,
            background: Color::BLUE,
            ..VisualStyle::default()
        },
        ..Style::default()
    };
    let mut b = Buffer::new(1, 2);
    bar.render(&mut RenderContext::with_style(
        &mut b,
        Rect::new(0, 0, 1, 2),
        &style,
    ));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(b.get(0, 1).unwrap().fg, Some(Color::BLUE));
    // The builders outrank it
    let bar = bar.thumb_color(Color::RED).track_color(Color::GREEN);
    let mut b = Buffer::new(1, 2);
    bar.render(&mut RenderContext::with_style(
        &mut b,
        Rect::new(0, 0, 1, 2),
        &style,
    ));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(b.get(0, 1).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_scrollbar_zero_areas_and_offset() {
    let bar = Scrollbar::vertical()
        .content_length(50)
        .viewport_length(5)
        .offset(20);
    assert_zero_areas_draw_nothing(&bar);
    assert_same_when_offset(bar, 1, 8);
}

proptest! {
    /// Any content, shown length, offset and bar length: the bar is a track
    /// with at most one run of thumb, which is the shown share of the track
    /// (at least a cell) and sits at the top at offset 0 and at the bottom
    /// at the last page; with nothing to scroll there is no thumb.
    #[test]
    fn prop_scrollbar_thumb_is_one_run(
        content in 0usize..500,
        viewport in 0usize..60,
        offset in 0usize..600,
        h in 1u16..40,
    ) {
        let bar = Scrollbar::vertical().content_length(content).viewport_length(viewport).offset(offset);
        let col: Vec<char> = column(&draw(&bar, 1, h)).chars().collect();
        prop_assert!(col.iter().all(|&c| c == '│' || c == '█'));
        let first = col.iter().position(|&c| c == '█');
        let last = col.iter().rposition(|&c| c == '█');
        if content <= viewport {
            prop_assert!(first.is_none());
            return Ok(());
        }
        let (first, last) = (first.unwrap(), last.unwrap());
        prop_assert!(col[first..=last].iter().all(|&c| c == '█'));
        let len = last - first + 1;
        let share = viewport as f64 / content as f64 * f64::from(h);
        prop_assert!(len as f64 <= share.max(1.0) + 1e-9 && len as f64 > share - 1.0, "{} vs {}", len, share);
        if offset == 0 {
            prop_assert_eq!(first, 0);
        }
        if offset >= content - viewport {
            prop_assert_eq!(last, usize::from(h) - 1);
        }
    }
}
