//! Skeleton drawing against a plain model of each shape, placement, and the
//! stylesheet path.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::theme::SEPARATOR_COLOR;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{hstack, vstack, Skeleton, SkeletonShape, Text};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const SHAPES: [SkeletonShape; 3] = [
    SkeletonShape::Rectangle,
    SkeletonShape::Circle,
    SkeletonShape::Paragraph,
];

/// Every cell of `buffer`, row by row.
fn cells(buffer: &Buffer) -> Vec<Cell> {
    (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).map(move |x| *buffer.get(x, y).unwrap()))
        .collect()
}

fn render(s: &Skeleton, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    s.render(&mut RenderContext::new(
        &mut buffer,
        Rect::new(0, 0, width, height),
    ));
    buffer
}

/// The shading character, written out here: frames cycle ░ ▒ ▓ ▒, and a
/// skeleton without animation always draws ░.
fn shade(animate: bool, frame: u8) -> char {
    if !animate {
        return '░';
    }
    ['░', '▒', '▓', '▒'][usize::from(frame % 4)]
}

/// What a skeleton draws into an `aw` x `ah` area at the origin: the glyph at
/// each cell, `None` where it draws nothing.
fn model(
    shape: SkeletonShape,
    width: u16,
    height: u16,
    lines: u16,
    ch: char,
    aw: u16,
    ah: u16,
) -> Vec<Vec<Option<char>>> {
    let mut grid = vec![vec![None; usize::from(aw)]; usize::from(ah)];
    let fill = if width == 0 { aw } else { width.min(aw) };
    let mut put = |x: u16, y: u16, c: char| {
        if x < aw && y < ah {
            grid[usize::from(y)][usize::from(x)] = Some(c);
        }
    };
    match shape {
        SkeletonShape::Rectangle => {
            for y in 0..height.min(ah) {
                for x in 0..fill {
                    put(x, y, ch);
                }
            }
        }
        SkeletonShape::Circle => {
            // The circle is `height` cells square (at least 1, at most the
            // area's height); `width` does not size it.
            let size = height.max(1).min(ah);
            match size {
                0 => {}
                1 => put(0, 0, '●'),
                2 => {
                    put(0, 0, '╭');
                    put(1, 0, '╮');
                    put(0, 1, '╰');
                    put(1, 1, '╯');
                }
                _ => {
                    let last = size - 1;
                    for y in 0..size {
                        for x in 0..size {
                            let c = match (x, y) {
                                (0, 0) => '╭',
                                (x, 0) if x == last => '╮',
                                (0, y) if y == last => '╰',
                                (x, y) if x == last && y == last => '╯',
                                (_, 0) => '─',
                                (_, y) if y == last => '─',
                                (0, _) => '│',
                                (x, _) if x == last => '│',
                                _ => ch,
                            };
                            put(x, y, c);
                        }
                    }
                }
            }
        }
        SkeletonShape::Paragraph => {
            let rows = lines.min(ah);
            for y in 0..rows {
                // The last row is two thirds long, odd rows four cells short
                let len = if y == rows - 1 {
                    fill * 2 / 3
                } else if y % 2 == 1 {
                    fill.saturating_sub(4)
                } else {
                    fill
                };
                for x in 0..len {
                    put(x, y, ch);
                }
            }
        }
    }
    grid
}

fn assert_matches_model(buffer: &Buffer, want: &[Vec<Option<char>>], fg: Color, what: &str) {
    for (y, row) in want.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let got = buffer.get(x as u16, y as u16).unwrap();
            match cell {
                Some(c) => {
                    assert_eq!(got.symbol, *c, "{what}: ({x}, {y})");
                    assert_eq!(got.fg, Some(fg), "{what}: fg at ({x}, {y})");
                }
                None => {
                    assert_eq!(got.symbol, ' ', "{what}: ({x}, {y}) should be empty");
                    assert_eq!(got.fg, None, "{what}: fg at ({x}, {y})");
                }
            }
            assert_eq!(got.bg, None, "{what}: bg at ({x}, {y})");
        }
    }
}

#[test]
fn zero_areas_draw_nothing() {
    for shape in SHAPES {
        for height in [0, 1, 2, 3] {
            assert_zero_areas_draw_nothing(&Skeleton::new().shape(shape).height(height));
        }
    }
}

#[test]
fn same_when_offset() {
    for shape in SHAPES {
        for (width, height) in [(0, 1), (5, 3), (12, 4)] {
            let s = Skeleton::new()
                .shape(shape)
                .width(width)
                .height(height)
                .lines(4)
                .frame(2);
            assert_same_when_offset(s, 10, 5);
        }
    }
}

#[test]
fn default_color_is_the_separator_color_and_the_builder_overrides_it() {
    for shape in SHAPES {
        let s = Skeleton::new().shape(shape).height(3);
        let want = model(shape, 0, 3, 3, '░', 10, 4);
        assert_matches_model(&render(&s, 10, 4), &want, SEPARATOR_COLOR, "default");
        let s = s.color(Color::MAGENTA);
        assert_matches_model(&render(&s, 10, 4), &want, Color::MAGENTA, "builder");
    }
}

#[test]
fn stylesheet_color_reaches_every_cell_and_the_builder_outranks_it() {
    for shape in SHAPES {
        let draw = |s: Skeleton| {
            let mut h =
                PipelineHarness::with_css("#s { color: #00ff00; }", 10, 4).dom_from_render(true);
            h.draw(&s.element_id("s"));
            h.buffer().clone()
        };
        let want = model(shape, 0, 3, 3, '░', 10, 4);
        let s = Skeleton::new().shape(shape).height(3);
        assert_matches_model(&draw(s.clone()), &want, Color::rgb(0, 255, 0), "css");
        // Naming the default explicitly still outranks the stylesheet
        let named = s.color(SEPARATOR_COLOR);
        assert_matches_model(&draw(named), &want, SEPARATOR_COLOR, "named");
    }
}

#[test]
fn shapes_at_their_smallest() {
    // One-cell area: a rectangle cell, a dot, and a paragraph's one line,
    // which is also its last line (two thirds of one column: nothing)
    let one = |shape| render(&Skeleton::new().shape(shape).no_animate(), 1, 1);
    assert_eq!(one(SkeletonShape::Rectangle).get(0, 0).unwrap().symbol, '░');
    assert_eq!(one(SkeletonShape::Circle).get(0, 0).unwrap().symbol, '●');
    assert_eq!(one(SkeletonShape::Paragraph).get(0, 0).unwrap().symbol, ' ');
    // Zero lines and zero height draw nothing
    let empty = |s: Skeleton| cells(&render(&s, 6, 3)) == cells(&Buffer::new(6, 3));
    assert!(empty(Skeleton::new().paragraph().lines(0)));
    assert!(empty(Skeleton::new().height(0)));
}

fn shape_strategy() -> impl Strategy<Value = SkeletonShape> {
    prop::sample::select(SHAPES.to_vec())
}

proptest! {
    /// Every shape, size, line count, frame and area draws what the model
    /// draws: the glyphs, the color on every drawn cell and nothing else.
    #[test]
    fn draws_the_model(
        shape in shape_strategy(),
        width in prop_oneof![Just(0u16), 1u16..30],
        height in 0u16..9,
        lines in 0u16..9,
        frame in any::<u8>(),
        animate in any::<bool>(),
        named in any::<bool>(),
        aw in 0u16..16,
        ah in 0u16..10,
    ) {
        let mut s = Skeleton::new()
            .shape(shape)
            .width(width)
            .height(height)
            .lines(lines)
            .frame(frame);
        if !animate {
            s = s.no_animate();
        }
        let fg = if named {
            s = s.color(Color::YELLOW);
            Color::YELLOW
        } else {
            SEPARATOR_COLOR
        };
        let want = model(shape, width, height, lines, shade(animate, frame), aw, ah);
        let what = format!("{shape:?} w{width} h{height} l{lines} in {aw}x{ah}");
        // A wider buffer than the area: nothing may spill past the area
        let mut buffer = Buffer::new(aw + 3, ah + 2);
        s.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, aw, ah)));
        let mut wide = want.clone();
        for row in &mut wide {
            row.extend([None; 3]);
        }
        wide.extend(vec![vec![None; usize::from(aw) + 3]; 2]);
        assert_matches_model(&buffer, &wide, fg, &what);
    }
}

// =========================================================================
// #1065
// =========================================================================

#[test]
fn a_circle_in_a_zero_height_area_draws_nothing() {
    for height in [0, 1, 2, 3] {
        let mut buffer = Buffer::new(10, 10);
        Skeleton::new()
            .circle()
            .height(height)
            .render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 0)));
        assert!(
            (0..10).all(|y| (0..10).all(|x| buffer.get(x, y).unwrap().symbol == ' ')),
            "height {height}"
        );
    }
}

#[test]
fn stacks_size_a_skeleton_by_its_width_and_height() {
    let row = |buffer: &Buffer, y: u16| -> String {
        (0..buffer.width())
            .map(|x| buffer.get(x, y).unwrap().symbol)
            .collect()
    };
    let draw = |view: &dyn View, w: u16, h: u16| {
        let mut buffer = Buffer::new(w, h);
        view.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)));
        buffer
    };

    let column = vstack()
        .child(Skeleton::new().height(1).no_animate())
        .child(Skeleton::new().height(1).no_animate())
        .child(Text::new("loaded"));
    let buffer = draw(&column, 10, 6);
    assert_eq!(row(&buffer, 0), "░░░░░░░░░░");
    assert_eq!(row(&buffer, 1), "░░░░░░░░░░");
    assert_eq!(row(&buffer, 2), "loaded    ");

    let line = hstack()
        .child(Skeleton::new().width(3).no_animate())
        .child(Text::new("name"));
    assert_eq!(row(&draw(&line, 12, 1), 0), "░░░name     ");
}
