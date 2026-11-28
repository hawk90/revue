//! Shared chart axis tests: grid lines, tick labels and axis titles

use super::super::rows;
use super::draw;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::chart::chart_render::{
    render_axis_title, render_grid, render_x_axis_labels, render_y_axis_labels,
};
use revue::widget::data::chart::{Axis, AxisFormat, ChartGrid};
use revue::widget::traits::RenderContext;

#[test]
fn test_render_grid() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::both(), 5, 5);
    });
    let lines = rows(&buffer);
    // Tick i of 5 on column i * 19 / 5 and row i * 9 / 5, the first and
    // last on the edges (#1622): horizontal lines on rows 0, 1, 3, 5, 7
    // and 9 (drawn over the vertical ones), vertical lines on columns 0,
    // 3, 7, 11, 15 and 19
    let line = format!("├{}", "─".repeat(19));
    for (y, row) in lines.iter().enumerate() {
        if [0, 1, 3, 5, 7, 9].contains(&y) {
            assert_eq!(*row, line, "row {y}");
        } else {
            assert_eq!(row, "│  │   │   │   │   │", "row {y}");
        }
    }
    // Vertical lines alone end in '┴' on the last row
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::x_only(), 5, 5);
    });
    assert_eq!(rows(&buffer)[9], "┴  ┴   ┴   ┴   ┴   ┴");
}

#[test]
fn test_render_grid_none() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::new(), 5, 5);
    });
    assert!(rows(&buffer).iter().all(|r| r.trim().is_empty()));
}

#[test]
fn test_render_grid_only_x() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = ChartGrid::x_only();
    render_grid(&mut ctx, area, &grid, 5, 0);

    // Should only have vertical lines
    let mut found_vertical = false;
    for y in 0..10 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '│' {
                    found_vertical = true;
                    break;
                }
            }
        }
    }
    assert!(found_vertical);
}

#[test]
fn test_render_grid_only_y() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let grid = ChartGrid::y_only();
    render_grid(&mut ctx, area, &grid, 0, 5);

    // Should only have horizontal lines
    let mut found_horizontal = false;
    for y in 0..10 {
        for x in 0..20 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '─' || cell.symbol == '├' {
                    found_horizontal = true;
                    break;
                }
            }
        }
    }
    assert!(found_horizontal);
}

/// An x axis title of 65536 columns (32768 wide glyphs) starts at the
/// title area's left edge and fills its last row with whole glyphs, inside
/// an area at an offset (#1596)
#[test]
fn test_render_axis_title_wider_than_u16_fills_the_row() {
    let title = "가".repeat(32_768);
    let mut buffer = Buffer::new(40, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(4, 1, 30, 4));
    render_axis_title(
        &mut ctx,
        Rect::new(3, 0, 20, 3),
        Some(&title),
        Color::GREEN,
        true,
    );
    for y in 0..6 {
        for x in 0..40 {
            let cell = buffer.get(x, y).unwrap();
            if y == 3 && (7..27).contains(&x) {
                if (x - 7) % 2 == 0 {
                    assert_eq!(
                        (cell.symbol, cell.fg),
                        ('가', Some(Color::GREEN)),
                        "({x}, {y})"
                    );
                } else {
                    assert!(cell.is_continuation(), "({x}, {y})");
                }
            } else {
                assert_eq!(cell.symbol, ' ', "({x}, {y})");
            }
        }
    }
}

/// A huge tick count draws one grid line per row and column, and returns
/// instead of looping once per tick (#1616)
#[test]
fn test_render_grid_caps_huge_tick_counts_at_the_cells() {
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::x_only(), usize::MAX, usize::MAX);
    });
    let lines = rows(&buffer);
    for (y, line) in lines[..9].iter().enumerate() {
        assert_eq!(*line, "│".repeat(20), "row {y}");
    }
    assert_eq!(lines[9], "┴".repeat(20));
    let buffer = draw(20, 10, |ctx, area| {
        render_grid(ctx, area, &ChartGrid::y_only(), usize::MAX, usize::MAX);
    });
    for (y, row) in rows(&buffer).iter().enumerate() {
        assert_eq!(*row, format!("├{}", "─".repeat(19)), "row {y}");
    }
}

/// Y labels on rows 1..=5 of a 10x6 area, `ticks` ticks over 0..10
fn y_labels(ticks: usize, format: AxisFormat, label_width: u16) -> Vec<String> {
    let axis = Axis::new().ticks(ticks).format(format).color(Color::RED);
    let buffer = draw(10, 6, |ctx, area| {
        render_y_axis_labels(ctx, area, &axis, 0.0, 10.0, label_width);
    });
    for y in 0..6 {
        for x in 0..10 {
            let cell = buffer.get(x, y).unwrap();
            if cell.symbol != ' ' {
                assert_eq!(cell.fg, Some(Color::RED), "({x}, {y})");
            }
        }
    }
    rows(&buffer)
}

/// Ten ticks on five rows: one label per row, the first tick of each row,
/// from the top (10) to the bottom (0) (#1615)
#[test]
fn test_render_y_axis_labels_draw_one_label_per_row() {
    let rows = y_labels(10, AxisFormat::Integer, 4);
    assert_eq!(
        rows,
        [
            "          ",
            "10        ",
            "7         ",
            "5         ",
            "2         ",
            "0         "
        ]
    );
    // Five ticks on five rows: all of them
    let rows = y_labels(4, AxisFormat::Fixed(1), 4);
    assert_eq!(
        rows[1..],
        [
            "10.0      ",
            "7.5       ",
            "5.0       ",
            "2.5       ",
            "0.0       "
        ]
    );
}

/// A huge tick count labels each row once, and returns instead of
/// formatting every tick (#1616)
#[test]
fn test_render_y_axis_labels_cap_huge_tick_counts_at_the_rows() {
    let rows = y_labels(usize::MAX, AxisFormat::Custom("v".into()), 4);
    assert_eq!(rows[0], " ".repeat(10));
    for (y, row) in rows.iter().enumerate().skip(1) {
        assert_eq!(*row, format!("v{}", " ".repeat(9)), "row {y}");
    }
}

/// A label wider than its columns is cut with `…`, wide glyphs whole
#[test]
fn test_render_y_axis_labels_cut_wide_labels_with_an_ellipsis() {
    let rows = y_labels(1, AxisFormat::Fixed(1), 3);
    assert_eq!(rows[1], "10…       ");
    assert_eq!(rows[5], "0.0       ");
    let rows = y_labels(1, AxisFormat::Custom("가나{}".into()), 4);
    // (`rows` shows the right half of a wide glyph as '\0')
    assert_eq!(rows[1], "가\0…       ");
    assert_eq!(rows[5], "가\0…       ");
    let rows = y_labels(1, AxisFormat::Custom("가나{}".into()), 5);
    assert_eq!(rows[5], concat!("가\0나\0", "0     "));
}

/// A huge tick count labels every other column, from the first one past
/// the offset, and returns instead of formatting every tick (#1615, #1616)
#[test]
fn test_render_x_axis_labels_draw_one_label_per_two_columns() {
    let axis = Axis::new()
        .ticks(usize::MAX)
        .format(AxisFormat::Custom("x".into()))
        .color(Color::RED);
    let buffer = draw(20, 3, |ctx, area| {
        render_x_axis_labels(ctx, area, &axis, 0.0, 1.0, 0, 4);
    });
    let rows = rows(&buffer);
    assert_eq!(rows[2], "    x x x x x x x x ");
    for x in (4..20).step_by(2) {
        assert_eq!(buffer.get(x, 2).unwrap().fg, Some(Color::RED), "x={x}");
    }
    assert!(rows[0].trim().is_empty() && rows[1].trim().is_empty());
}

/// The X labels span the chart area `calculate_chart_area` leaves after
/// the offset, x_max on its last column, each centered on its tick and cut
/// with `…` to the columns between labeled ticks less one (#1622, #1623)
#[test]
fn test_render_x_axis_labels_reach_the_last_column_cut_to_their_slot() {
    let axis = Axis::new()
        .ticks(3)
        .format(AxisFormat::Fixed(2))
        .color(Color::RED);
    let buffer = draw(20, 3, |ctx, area| {
        render_x_axis_labels(ctx, area, &axis, 0.0, 30.0, 0, 4);
    });
    // The chart is columns 4..18: ticks on 4, 8, 13 and 18
    let row = &rows(&buffer)[2];
    assert_eq!(row, "   0.… 10…  20…  30…");
    for (x, ch) in (0u16..).zip(row.chars()) {
        let fg = buffer.get(x, 2).unwrap().fg;
        assert_eq!(fg, (ch != ' ').then_some(Color::RED), "x={x}");
    }
}

/// Grid lines of a chart area at an offset: the first on its left and top
/// edges, the last on its right and bottom ones, nothing outside (#1622)
#[test]
fn test_render_grid_at_an_offset_reaches_the_last_row_and_column() {
    let buffer = draw(14, 8, |ctx, _| {
        render_grid(ctx, Rect::new(2, 1, 10, 5), &ChartGrid::x_only(), 2, 2);
    });
    let lines = rows(&buffer);
    assert_eq!(lines[0], " ".repeat(14));
    for y in 1..5 {
        assert_eq!(lines[y], "  │   │    │  ", "row {y}");
    }
    assert_eq!(lines[5], "  ┴   ┴    ┴  ");
    assert_eq!(lines[6], " ".repeat(14));
    let buffer = draw(14, 8, |ctx, _| {
        render_grid(ctx, Rect::new(2, 1, 10, 5), &ChartGrid::y_only(), 2, 2);
    });
    let lines = rows(&buffer);
    for (y, line) in lines.iter().enumerate() {
        let want = if [1, 3, 5].contains(&y) {
            format!("  ├{}  ", "─".repeat(9))
        } else {
            " ".repeat(14)
        };
        assert_eq!(*line, want, "row {y}");
    }
}
