//! Shared chart rendering functions (chart::chart_render)

use super::rows;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::chart_render::{
    calculate_chart_area, calculate_legend_position, fill_background, render_grid, render_legend,
    render_title, LegendItem,
};
use revue::widget::data::chart::{ChartGrid, Legend, LegendPosition};
use revue::widget::traits::RenderContext;

/// Run `draw` against a fresh `width` x `height` buffer.
fn draw(width: u16, height: u16, draw: impl FnOnce(&mut RenderContext, Rect)) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    draw(&mut ctx, area);
    buffer
}

#[test]
fn test_render_title() {
    let mut offset = 0;
    let buffer = draw(30, 10, |ctx, area| {
        offset = render_title(ctx, area, Some("Test Title"), Color::YELLOW);
    });
    assert_eq!(offset, 1);
    // Centered: (30 - 10) / 2 = 10
    assert_eq!(rows(&buffer)[0], format!("{:10}Test Title{:10}", "", ""));
    assert_eq!(buffer.get(10, 0).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_render_title_none() {
    let mut offset = 1;
    let buffer = draw(30, 10, |ctx, area| {
        offset = render_title(ctx, area, None, Color::WHITE);
    });
    assert_eq!(offset, 0);
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_render_grid() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::both(), 5, 5);
    });
    let rows = rows(&buffer);
    // Horizontal lines every 2 rows (drawn over the vertical ones),
    // vertical lines every 4 columns ending in '┴' on the last row
    assert_eq!(rows[0], format!("├{}", "─".repeat(19)));
    assert_eq!(rows[2], rows[0]);
    assert_eq!(rows[1], "│   │   │   │   │   ");
    assert_eq!(rows[9], "┴   ┴   ┴   ┴   ┴   ");
}

#[test]
fn test_render_grid_none() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::new(), 5, 5);
    });
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

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
fn test_fill_background() {
    let buffer = draw(10, 5, |ctx, _| {
        fill_background(ctx, Rect::new(2, 1, 3, 2), Color::BLUE);
    });
    for y in 0..5 {
        for x in 0..10 {
            let inside = (2..5).contains(&x) && (1..3).contains(&y);
            let bg = buffer.get(x, y).unwrap().bg;
            assert_eq!(bg == Some(Color::BLUE), inside, "({x}, {y})");
        }
    }
}

#[test]
fn test_calculate_chart_area() {
    let area = Rect::new(0, 0, 100, 50);
    assert_eq!(
        calculate_chart_area(area, true, 8, 2),
        Rect::new(8, 1, 91, 46)
    );
    assert_eq!(
        calculate_chart_area(area, false, 8, 2),
        Rect::new(8, 0, 91, 47)
    );
    // Too small: sizes saturate at zero
    let tiny = calculate_chart_area(Rect::new(0, 0, 5, 2), true, 8, 2);
    assert_eq!((tiny.width, tiny.height), (0, 0));
}
