//! Splitter divider placement and rendering

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{pane, splitter, Splitter, SplitterStyle, View};

fn render(s: &Splitter, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    s.render(&mut ctx);
    buffer
}

fn row(buffer: &Buffer, y: u16, w: u16) -> String {
    (0..w)
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .collect()
}

#[test]
fn test_splitter_panes_and_dividers_fill_the_area() {
    let s = splitter().pane(pane("a")).pane(pane("b")).pane(pane("c"));
    let area = Rect::new(3, 0, 50, 10);
    let areas = s.pane_areas(area);
    assert_eq!(areas.len(), 3);
    // Each pane starts one column after the previous pane's divider ...
    for pair in areas.windows(2) {
        let (prev, next) = (&pair[0].1, &pair[1].1);
        assert_eq!(prev.x + prev.width + 1, next.x);
    }
    // ... and the last pane reaches the right edge of the area.
    let last = areas[2].1;
    assert_eq!(last.x + last.width, area.x + area.width);
}

#[test]
fn test_splitter_vertical_last_pane_reaches_bottom() {
    let s = splitter().vertical().pane(pane("top")).pane(pane("bottom"));
    let areas = s.pane_areas(Rect::new(0, 2, 10, 21));
    assert_eq!(areas[0].1, Rect::new(0, 2, 10, 10));
    assert_eq!(areas[1].1, Rect::new(0, 13, 10, 10));
}

#[test]
fn test_splitter_renders_horizontal_divider() {
    let s = splitter().pane(pane("a")).pane(pane("b"));
    let buffer = render(&s, 21, 2);
    let expected = format!("{}│{}", " ".repeat(10), " ".repeat(10));
    assert_eq!(row(&buffer, 0, 21), expected);
    assert_eq!(row(&buffer, 1, 21), expected);
}

#[test]
fn test_splitter_renders_vertical_divider() {
    let s = splitter().vertical().pane(pane("a")).pane(pane("b"));
    let buffer = render(&s, 4, 11);
    assert_eq!(row(&buffer, 4, 4), "    ");
    assert_eq!(row(&buffer, 5, 4), "────");
    assert_eq!(row(&buffer, 6, 4), "    ");
}

#[test]
fn test_splitter_divider_styles() {
    let cases = [
        (SplitterStyle::Double, '║'),
        (SplitterStyle::Thick, '┃'),
        (SplitterStyle::Hidden, ' '),
    ];
    for (style, ch) in cases {
        let s = splitter().style(style).pane(pane("a")).pane(pane("b"));
        let buffer = render(&s, 21, 1);
        assert_eq!(buffer.get(10, 0).unwrap().symbol, ch, "{style:?}");
    }
}

#[test]
fn test_splitter_active_divider_uses_active_color() {
    let mut s = splitter()
        .color(Color::RED)
        .active_color(Color::GREEN)
        .pane(pane("a"))
        .pane(pane("b"))
        .pane(pane("c"));
    s.start_resize(1);
    let areas = s.pane_areas(Rect::new(0, 0, 32, 1));
    let first = areas[0].1.x + areas[0].1.width;
    let second = areas[1].1.x + areas[1].1.width;

    let buffer = render(&s, 32, 1);
    assert_eq!(buffer.get(first, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(second, 0).unwrap().fg, Some(Color::GREEN));

    s.stop_resize();
    let buffer = render(&s, 32, 1);
    assert_eq!(buffer.get(second, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_splitter_single_pane_draws_no_divider() {
    let s = splitter().pane(pane("only"));
    let buffer = render(&s, 10, 1);
    assert_eq!(row(&buffer, 0, 10), " ".repeat(10));
}

#[test]
fn test_splitter_zero_areas_and_offset() {
    for vertical in [false, true] {
        let mut s = splitter()
            .color(Color::BLUE)
            .pane(pane("a").min_size(0))
            .pane(pane("b").collapsible())
            .pane(pane("c").min_size(0).ratio(0.2));
        if vertical {
            s = s.vertical();
        }
        s.start_resize(1);
        crate::traits::placement::assert_zero_areas_draw_nothing(&s);
        crate::traits::placement::assert_same_when_offset(s, 15, 9);
    }
}

#[test]
fn test_splitter_idle_divider_is_dark_gray_by_default() {
    let s = splitter().pane(pane("a")).pane(pane("b"));
    let buffer = render(&s, 21, 1);
    assert_eq!(
        buffer.get(10, 0).unwrap().fg,
        Some(revue::widget::theme::DARK_GRAY)
    );
}

mod props {
    use super::*;
    use proptest::prelude::*;
    use revue::widget::SplitOrientation;

    const STYLES: [(SplitterStyle, char, char); 4] = [
        (SplitterStyle::Line, '│', '─'),
        (SplitterStyle::Double, '║', '═'),
        (SplitterStyle::Thick, '┃', '━'),
        (SplitterStyle::Hidden, ' ', ' '),
    ];

    proptest! {
        /// The splitter draws exactly its dividers: the full line one cell
        /// past every visible pane but the last (when that cell is inside
        /// the area), in the style's glyph, the divider being resized in
        /// the active color and the others in the idle color; no other
        /// cell is touched, in the area or outside it.
        #[test]
        fn prop_splitter_draws_its_dividers_and_nothing_else(
            n in 0usize..6,
            ratios in prop::collection::vec(0.0f32..=1.0, 6),
            mins in prop::collection::vec(0u16..8, 6),
            collapsed in prop::collection::vec(prop::bool::weighted(0.25), 6),
            vertical in any::<bool>(),
            style in 0usize..4,
            color in prop::option::of(Just(Color::RED)),
            active in prop::option::of(0usize..6),
            (x, y, w, h) in (0u16..4, 0u16..4, 0u16..40, 0u16..16),
        ) {
            let mut s = splitter().style(STYLES[style].0).active_color(Color::GREEN);
            if vertical {
                s = s.orientation(SplitOrientation::Vertical);
            }
            if let Some(c) = color {
                s = s.color(c);
            }
            for i in 0..n {
                s = s.pane(pane(format!("p{i}")).ratio(ratios[i]).min_size(mins[i]).collapsible());
            }
            for (i, &c) in collapsed.iter().enumerate().take(n) {
                if c {
                    s.toggle_pane(i);
                }
            }
            if let Some(d) = active {
                s.start_resize(d);
            }
            let area = Rect::new(x, y, w, h);
            let areas = s.pane_areas(area);
            let dividers = areas.len().saturating_sub(1);
            let active = active.filter(|&d| d < dividers);

            let mut buffer = Buffer::new(x + w + 2, y + h + 2);
            let mut ctx = RenderContext::new(&mut buffer, area);
            s.render(&mut ctx);

            // Expected: the divider index at each line along the split
            let mut line = std::collections::HashMap::new();
            for (i, (_, r)) in areas.iter().take(dividers).enumerate() {
                let at = if vertical { r.y + r.height } else { r.x + r.width };
                line.insert(at, i);
            }
            let glyph = if vertical { STYLES[style].2 } else { STYLES[style].1 };
            let idle = color.unwrap_or(revue::widget::theme::DARK_GRAY);
            for yy in 0..y + h + 2 {
                for xx in 0..x + w + 2 {
                    let cell = buffer.get(xx, yy).unwrap();
                    let inside = (x..x + w).contains(&xx) && (y..y + h).contains(&yy);
                    let along = if vertical { yy } else { xx };
                    match line.get(&along) {
                        Some(&i) if inside => {
                            let fg = if Some(i) == active { Color::GREEN } else { idle };
                            prop_assert_eq!((cell.symbol, cell.fg), (glyph, Some(fg)), "({}, {})", xx, yy);
                        }
                        _ => prop_assert_eq!(cell, &revue::render::Cell::default(), "({}, {})", xx, yy),
                    }
                }
            }
        }
    }
}
