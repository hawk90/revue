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
