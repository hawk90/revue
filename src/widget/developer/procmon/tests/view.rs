//! The drawn rows, columns, colors and size guards

use super::{bare, draw, highlighted_rows, monitor_with, numbered, pids_on_screen, process, row};
use crate::layout::Rect;
use crate::render::{Buffer, Modifier};
use crate::style::Color;
use crate::widget::developer::procmon::{ProcColors, ProcessSort};
use crate::widget::theme::LIGHT_GRAY;
use crate::widget::traits::{RenderContext, View};

/// The cell colors and modifier at `(x, y)`
fn style(buffer: &Buffer, x: u16, y: u16) -> (Option<Color>, Option<Color>, Modifier) {
    let cell = buffer.get(x, y).unwrap();
    (cell.fg, cell.bg, cell.modifier)
}

fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| {
        (0..buffer.width()).all(|x| {
            let cell = buffer.get(x, y).unwrap();
            cell.symbol == ' ' && cell.fg.is_none() && cell.bg.is_none()
        })
    })
}

#[test]
fn each_column_starts_at_its_offset() {
    let monitor = monitor_with(vec![process(
        42,
        "rustc",
        12.5,
        3 * 1024 * 1024,
        4.5,
        "Sleeping",
        "rustc --edition",
        "0",
    )])
    .show_cmd(true);
    let buffer = draw(&monitor, 100, 5);
    assert_eq!(
        row(&buffer, 1).trim_end(),
        "PID▲   NAME                CPU%   MEM%   MEM     STATUS    COMMAND"
    );
    assert_eq!(
        row(&buffer, 2).trim_end(),
        "    42 rustc                 12.5    4.5      3M Sleeping  rustc --edition"
    );
}

#[test]
fn zero_areas_draw_nothing() {
    let monitor = monitor_with(numbered(3));
    for (w, h) in [(0, 0), (0, 10), (10, 0)] {
        let mut buffer = Buffer::new(10, 10);
        monitor.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, w, h)));
        assert!(blank(&buffer), "{w}x{h}");
    }
}

#[test]
fn an_area_under_40x5_draws_nothing() {
    let monitor = monitor_with(numbered(3));
    for (w, h) in [(39, 20), (100, 4), (10, 2)] {
        assert!(blank(&draw(&monitor, w, h)), "{w}x{h}");
    }
    // 40x5 is enough: the header and three rows
    let buffer = draw(&monitor, 40, 5);
    assert!(row(&buffer, 1).starts_with("PID▲"));
    assert_eq!(pids_on_screen(&buffer), [100, 101, 102]);
}

#[test]
fn an_offset_area_draws_the_same_cells_shifted() {
    let monitor = monitor_with(vec![
        process(1, "init", 85.0, 2048, 12.0, "Run", "/sbin/init", "0"),
        process(2, "sh", 31.0, 10, 1.0, "Sleeping", "", "0"),
    ])
    .show_cmd(true);
    let alone = draw(&monitor, 70, 6);
    let mut placed = Buffer::new(80, 10);
    monitor.render(&mut RenderContext::new(&mut placed, Rect::new(5, 3, 70, 6)));
    for y in 0..10 {
        for x in 0..80 {
            let inside = (5..75).contains(&x) && (3..9).contains(&y);
            let want = if inside {
                *alone.get(x - 5, y - 3).unwrap()
            } else {
                *Buffer::new(1, 1).get(0, 0).unwrap()
            };
            assert_eq!(placed.get(x, y).unwrap(), &want, "({x}, {y})");
        }
    }
}

#[test]
fn the_header_fills_its_row_bold_in_the_header_colors() {
    let colors = ProcColors {
        header_bg: Color::rgb(1, 2, 3),
        header_fg: Color::rgb(4, 5, 6),
        ..ProcColors::default()
    };
    let monitor = monitor_with(numbered(1)).colors(colors).show_cmd(true);
    let buffer = draw(&monitor, 70, 5);
    for x in 0..70 {
        assert_eq!(
            buffer.get(x, 1).unwrap().bg,
            Some(Color::rgb(1, 2, 3)),
            "x={x}"
        );
    }
    let bold = (
        Some(Color::rgb(4, 5, 6)),
        Some(Color::rgb(1, 2, 3)),
        Modifier::BOLD,
    );
    for x in [0, 7, 27, 34, 41, 49, 59] {
        assert_eq!(style(&buffer, x, 1), bold, "x={x}");
    }
}

#[test]
fn the_sort_arrow_marks_the_sort_column_and_its_direction() {
    let cases = [
        (
            ProcessSort::Pid,
            false,
            "PID▼   NAME                CPU%   MEM%   MEM     STATUS",
        ),
        (
            ProcessSort::Name,
            true,
            "PID    NAME▲               CPU%   MEM%   MEM     STATUS",
        ),
        (
            ProcessSort::Cpu,
            false,
            "PID    NAME                CPU%▼  MEM%   MEM     STATUS",
        ),
        // MEM% and MEM both sort by memory
        (
            ProcessSort::Memory,
            true,
            "PID    NAME                CPU%   MEM%▲  MEM▲    STATUS",
        ),
        (
            ProcessSort::Status,
            false,
            "PID    NAME                CPU%   MEM%   MEM     STATUS▼",
        ),
    ];
    for (sort, asc, header) in cases {
        let monitor = bare().sort_by(sort).ascending(asc);
        assert_eq!(
            row(&draw(&monitor, 70, 5), 1).trim_end(),
            header,
            "{sort:?}"
        );
    }
}

#[test]
fn cpu_turns_medium_above_30_and_high_above_80() {
    let colors = ProcColors::default();
    let cases = [
        (0.0, colors.low_cpu),
        (30.0, colors.low_cpu),
        (30.1, colors.medium_cpu),
        (80.0, colors.medium_cpu),
        (80.1, colors.high_cpu),
        (250.0, colors.high_cpu),
    ];
    for (cpu, want) in cases {
        let monitor = monitor_with(vec![process(1, "a", cpu, 0, 0.0, "Run", "", "0")]);
        let buffer = draw(&monitor, 60, 5);
        // The column is right-aligned in six cells from x=27
        for x in 27..33 {
            assert_eq!(buffer.get(x, 2).unwrap().fg, Some(want), "cpu={cpu} x={x}");
        }
    }
}

#[test]
fn mem_percent_turns_high_above_10_and_is_the_row_color_otherwise() {
    let row_fg = Color::rgb(9, 9, 9);
    let colors = ProcColors {
        name: Some(row_fg),
        ..ProcColors::default()
    };
    for (percent, want) in [(10.0, row_fg), (10.1, colors.high_mem), (0.0, row_fg)] {
        let monitor = monitor_with(vec![process(1, "a", 0.0, 0, percent, "Run", "", "0")])
            .colors(colors.clone());
        let buffer = draw(&monitor, 60, 5);
        for x in 34..40 {
            assert_eq!(buffer.get(x, 2).unwrap().fg, Some(want), "{percent}% x={x}");
        }
    }
}

#[test]
fn row_columns_have_their_colors() {
    let colors = ProcColors {
        pid: Color::rgb(1, 1, 1),
        ..ProcColors::default()
    };
    let monitor = monitor_with(vec![process(4321, "abc", 0.0, 2048, 0.0, "Run", "c", "0")])
        .colors(colors)
        .show_cmd(true);
    let buffer = draw(&monitor, 70, 5);
    let fg = |x| buffer.get(x, 2).unwrap().fg;
    // PID, name and memory; the name and memory default to white
    assert_eq!(fg(5), Some(Color::rgb(1, 1, 1)));
    assert_eq!(fg(7), Some(Color::WHITE));
    assert_eq!(fg(47), Some(Color::WHITE));
    // Status and command are light gray
    assert_eq!(fg(49), Some(LIGHT_GRAY));
    assert_eq!(fg(59), Some(LIGHT_GRAY));
    // ProcColors::name sets the name and memory columns
    let named = monitor_with(vec![process(4321, "abc", 0.0, 2048, 0.0, "Run", "c", "0")]).colors(
        ProcColors {
            name: Some(Color::rgb(7, 7, 7)),
            ..ProcColors::default()
        },
    );
    let buffer = draw(&named, 70, 5);
    assert_eq!(buffer.get(7, 2).unwrap().fg, Some(Color::rgb(7, 7, 7)));
    assert_eq!(buffer.get(47, 2).unwrap().fg, Some(Color::rgb(7, 7, 7)));
}

#[test]
fn the_selected_row_has_the_selected_background_across_the_area() {
    let mut monitor = monitor_with(numbered(3));
    monitor.select_next();
    let buffer = draw(&monitor, 64, 6);
    assert_eq!(highlighted_rows(&monitor, &buffer), [3]);
    let selected_bg = monitor.colors.selected_bg;
    for x in 0..64 {
        assert_eq!(buffer.get(x, 3).unwrap().bg, Some(selected_bg), "x={x}");
        assert_eq!(buffer.get(x, 2).unwrap().bg, None, "x={x}");
    }
}

#[test]
fn a_long_name_is_cut_at_19_columns_and_a_long_status_at_8() {
    let monitor = monitor_with(vec![process(
        1,
        "abcdefghijklmnopqrstuvwxyz",
        0.0,
        0,
        0.0,
        "UninterruptibleDiskSleep",
        "",
        "0",
    )]);
    let line = row(&draw(&monitor, 60, 5), 2);
    assert_eq!(&line[7..27], "abcdefghijklmnopqrs ");
    assert_eq!(&line[49..58], "Uninterr ");
}

#[test]
fn the_status_column_is_drawn_only_wider_than_55() {
    let monitor = monitor_with(vec![process(1, "a", 0.0, 0, 0.0, "Run", "", "0")]);
    assert_eq!(&row(&draw(&monitor, 55, 5), 2)[49..55], "      ");
    assert_eq!(&row(&draw(&monitor, 56, 5), 2)[49..56], "Run    ");
}

#[test]
fn show_cmd_falls_back_to_the_name_and_stops_at_the_area() {
    let monitor = monitor_with(vec![
        process(1, "named", 0.0, 0, 0.0, "Run", "", "0"),
        process(
            2,
            "b",
            0.0,
            0,
            0.0,
            "Run",
            "/usr/bin/a-long-command --flag",
            "0",
        ),
    ])
    .show_cmd(true);
    let buffer = draw(&monitor, 70, 5);
    // The header has the 3-byte arrow; count columns, not bytes
    let header: String = row(&buffer, 1).chars().skip(59).collect();
    assert_eq!(header.trim_end(), "COMMAND");
    assert_eq!(row(&buffer, 2)[59..].trim_end(), "named");
    assert_eq!(&row(&buffer, 3)[59..], "/usr/bin/a-");
}

#[test]
fn rows_stop_at_the_bottom_and_a_selection_below_scrolls_into_view() {
    let mut monitor = monitor_with(numbered(10));
    // 5 rows fit under the stats bar and the header
    assert_eq!(
        pids_on_screen(&draw(&monitor, 60, 7)),
        [100, 101, 102, 103, 104]
    );
    monitor.page_down(7);
    let buffer = draw(&monitor, 60, 7);
    assert_eq!(pids_on_screen(&buffer), [103, 104, 105, 106, 107]);
    assert_eq!(highlighted_rows(&monitor, &buffer), [6]);
}

#[test]
fn an_empty_list_draws_the_header_and_no_rows() {
    let monitor = monitor_with(Vec::new());
    let buffer = draw(&monitor, 60, 6);
    assert!(row(&buffer, 1).starts_with("PID▲"));
    for y in 2..6 {
        assert_eq!(row(&buffer, y).trim(), "", "y={y}");
        assert_eq!(buffer.get(0, y).unwrap().bg, None);
    }
}

#[test]
fn the_stats_bar_is_light_gray_and_counts_the_processes() {
    let monitor = monitor_with(numbered(3));
    let buffer = draw(&monitor, 80, 6);
    let stats = row(&buffer, 0);
    assert!(stats.starts_with("CPU:"), "{stats}");
    assert!(stats.trim_end().ends_with("Processes: 3"), "{stats}");
    for x in 0..stats.trim_end().chars().count() as u16 {
        assert_eq!(
            style(&buffer, x, 0),
            (Some(LIGHT_GRAY), None, Modifier::empty()),
            "x={x}"
        );
    }
}

/// `render` worked the first row out from the selection each frame and
/// dropped it, so moving up inside the page scrolled the view (#973).
#[test]
fn moving_up_inside_the_page_keeps_the_page() {
    let mut monitor = monitor_with(numbered(10)); // PIDs 100..109
    monitor.page_down(7);
    // 5 rows fit in 7: PIDs 103..107, 107 highlighted at the bottom
    assert_eq!(
        pids_on_screen(&draw(&monitor, 60, 7)),
        [103, 104, 105, 106, 107]
    );
    monitor.select_prev();
    let buffer = draw(&monitor, 60, 7);
    assert_eq!(pids_on_screen(&buffer), [103, 104, 105, 106, 107]);
    assert_eq!(highlighted_rows(&monitor, &buffer), [5]);
}
