//! Shared chart legend tests: placement and drawing

use super::super::rows;
use super::draw;
use revue::layout::Rect;
use revue::style::Color;
use revue::widget::data::chart::chart_render::{
    calculate_legend_position, render_legend, LegendItem,
};
use revue::widget::data::chart::{Legend, LegendPosition};

#[test]
fn test_calculate_legend_position() {
    let area = Rect::new(0, 0, 100, 50);
    let pos = |p| calculate_legend_position(p, area, 20, 5);
    assert_eq!(pos(LegendPosition::TopLeft), Some((1, 1)));
    assert_eq!(pos(LegendPosition::TopCenter), Some((40, 1)));
    assert_eq!(pos(LegendPosition::TopRight), Some((79, 1)));
    assert_eq!(pos(LegendPosition::BottomLeft), Some((1, 44)));
    assert_eq!(pos(LegendPosition::BottomCenter), Some((40, 44)));
    assert_eq!(pos(LegendPosition::BottomRight), Some((79, 44)));
    assert_eq!(pos(LegendPosition::Left), Some((1, 22)));
    assert_eq!(pos(LegendPosition::Right), Some((79, 22)));
    assert_eq!(pos(LegendPosition::None), None);
}

#[test]
fn test_calculate_legend_position_larger_than_area() {
    // A legend bigger than the area is pinned to the top/left edge
    let area = Rect::new(0, 0, 10, 4);
    let pos = |p| calculate_legend_position(p, area, 20, 8);
    assert_eq!(pos(LegendPosition::TopCenter), Some((0, 1)));
    assert_eq!(pos(LegendPosition::BottomCenter), Some((0, 0)));
    assert_eq!(pos(LegendPosition::Left), Some((1, 0)));
    assert_eq!(pos(LegendPosition::Right), Some((0, 0)));
}

#[test]
fn test_render_legend_larger_than_area() {
    let items = [LegendItem {
        label: "A label much wider than the area",
        color: Color::RED,
    }];
    for legend in [Legend::top_center(), Legend::left(), Legend::right()] {
        let buffer = draw(12, 3, |ctx, area| {
            render_legend(ctx, area, &legend, &items);
        });
        assert!(rows(&buffer).join("").contains('┌'));
    }
}

#[test]
fn test_render_legend() {
    let items = [
        LegendItem {
            label: "Series A",
            color: Color::RED,
        },
        LegendItem {
            label: "Series B",
            color: Color::GREEN,
        },
    ];
    let buffer = draw(40, 20, |ctx, area| {
        render_legend(ctx, area, &Legend::top_right(), &items);
    });
    let rows = rows(&buffer);
    // 12 wide (label + 4), 4 tall, at x = 40 - 13
    let pad = " ".repeat(27);
    assert_eq!(rows[1], format!("{pad}┌──────────┐ "));
    assert_eq!(rows[2], format!("{pad}│■ Series A│ "));
    assert_eq!(rows[3], format!("{pad}│■ Series B│ "));
    assert_eq!(rows[4], format!("{pad}└──────────┘ "));
    assert_eq!(buffer.get(28, 2).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(28, 3).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_render_legend_hidden_or_empty() {
    let items = [LegendItem {
        label: "A",
        color: Color::RED,
    }];
    let hidden = draw(40, 20, |ctx, area| {
        render_legend(ctx, area, &Legend::none(), &items);
    });
    assert!(rows(&hidden).iter().all(|r| r.trim().is_empty()));
    let empty = draw(40, 20, |ctx, area| {
        render_legend(ctx, area, &Legend::top_right(), &[]);
    });
    assert!(rows(&empty).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_render_horizontal_legend_lays_out_wide_labels_in_columns() {
    // "매출" takes four columns: the next item starts after them (#974)
    let items = [
        LegendItem {
            label: "매출",
            color: Color::RED,
        },
        LegendItem {
            label: "B",
            color: Color::BLUE,
        },
    ];
    let buffer = draw(30, 3, |ctx, area| {
        revue::widget::data::chart::chart_render::render_horizontal_legend(
            ctx,
            area,
            &Legend::top_left(),
            &items,
        );
    });
    let row: String = (0..30)
        .map(|x| buffer.get(x, 1).unwrap())
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect();
    assert!(row.contains("● 매출  ● B"), "{row:?}");
}

/// The rows of `buffer` from `area`, as text
fn area_rows(buffer: &revue::render::Buffer, area: Rect) -> Vec<String> {
    (area.y..area.bottom())
        .map(|y| {
            (area.x..area.right())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

/// A legend of more entries, or a wider label, than u16 holds is clamped to
/// it and clipped to the area instead of overflowing (#1552)
#[test]
fn test_render_legend_counts_near_u16_max() {
    let area = Rect::new(2, 1, 12, 5);
    let run = |items: &[LegendItem<'_>]| {
        let mut buffer = revue::render::Buffer::new(16, 8);
        let mut ctx = revue::widget::traits::RenderContext::new(&mut buffer, area);
        render_legend(&mut ctx, Rect::new(0, 0, 12, 5), &Legend::top_left(), items);
        buffer
    };

    // 65534 entries: 65536 rows with the border
    let items: Vec<LegendItem<'_>> = (0..65_534)
        .map(|_| LegendItem {
            label: "s",
            color: Color::RED,
        })
        .collect();
    let buffer = run(&items);
    assert_eq!(
        area_rows(&buffer, Rect::new(2, 1, 12, 5)),
        [
            "            ",
            " ┌───┐      ",
            " │■ s│      ",
            " │■ s│      ",
            " │■ s│      "
        ]
    );
    for y in 3..6 {
        assert_eq!(buffer.get(4, y).unwrap().fg, Some(Color::RED), "y={y}");
        assert_eq!(buffer.get(6, y).unwrap().fg, Some(Color::WHITE), "y={y}");
    }

    // A 65532-byte label: 65536 columns with the marker and border
    let label = "x".repeat(65_532);
    let buffer = run(&[LegendItem {
        label: &label,
        color: Color::RED,
    }]);
    assert_eq!(
        area_rows(&buffer, Rect::new(2, 1, 12, 5)),
        [
            "            ",
            " ┌──────────",
            " │■ xxxxxxx ",
            " └──────────",
            "            "
        ]
    );
}

/// The one-line legend's width adds a gap per entry: past 32768 entries
/// that overflowed u16 (#1552)
#[test]
fn test_render_horizontal_legend_counts_near_u16_max() {
    let items: Vec<LegendItem<'_>> = (0..32_769)
        .map(|_| LegendItem {
            label: "s",
            color: Color::BLUE,
        })
        .collect();
    let buffer = draw(20, 3, |ctx, area| {
        revue::widget::data::chart::chart_render::render_horizontal_legend(
            ctx,
            area,
            &Legend::top_center(),
            &items,
        );
    });
    assert_eq!(rows(&buffer)[1], "● s  ● s  ● s  ● s  ");
    for x in [0, 5, 10, 15] {
        assert_eq!(buffer.get(x, 1).unwrap().fg, Some(Color::BLUE), "x={x}");
    }
}

/// The box is sized by the labels' terminal columns, not their bytes, and
/// a label cut at the area's edge leaves out a wide glyph that would end
/// on the area's last column (#1564)
#[test]
fn test_render_legend_wide_labels() {
    let run = |label: &str, w: u16| {
        let items = [LegendItem {
            label,
            color: Color::RED,
        }];
        draw(w, 4, |ctx, area| {
            render_legend(ctx, area, &Legend::top_left(), &items);
        })
    };
    // "매출" is 4 columns: the box is 8 wide
    let buffer = run("매출", 12);
    assert_eq!(
        rows(&buffer),
        [
            "            ",
            " ┌──────┐   ",
            " │■ 매\0출\0│   ",
            " └──────┘   "
        ]
    );
    for x in 4..8 {
        assert_eq!(buffer.get(x, 2).unwrap().fg, Some(Color::WHITE), "x={x}");
    }
    // Cut at the 8-column area: "ab" ends at column 5, "매" would take 6
    // and 7, the area's last column, which stays as the box drew it
    let buffer = run("ab매출", 8);
    assert_eq!(
        rows(&buffer),
        ["        ", " ┌──────", " │■ ab  ", " └──────"]
    );
}
