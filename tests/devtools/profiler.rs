//! The render profiler's Ranked and Counts views keep a 20-column name
//! column, so a wide (Hangul) name leaves the next column where an ASCII one
//! does.

use revue::devtools::{DevToolsConfig, Profiler, ProfilerView, RenderEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use std::time::Duration;

/// The area the views draw into, at an offset inside a 70x12 buffer.
const AREA: Rect = Rect {
    x: 5,
    y: 1,
    width: 60,
    height: 10,
};

/// Rows from `AREA.y + 2` on, ordered by both total time and count:
/// - 10 Hangul chars, exactly 20 columns, 3 renders;
/// - 11 Hangul chars, cut to 9 chars + "…" (19 columns), 2 renders;
/// - an ASCII name, 1 render.
const NAMES: [&str; 3] = ["렌더렌더렌더렌더렌더", "렌더렌더렌더렌더렌더렌", "abc"];

fn draw(view: ProfilerView) -> Buffer {
    let mut profiler = Profiler::new();
    for (i, name) in NAMES.iter().enumerate() {
        for _ in 0..(3 - i) {
            profiler.record_render(RenderEvent::new(*name, Duration::from_millis(1)));
        }
    }
    profiler.set_view(view);
    let mut buffer = Buffer::new(70, 12);
    profiler.render_content(&mut buffer, AREA, &DevToolsConfig::default());
    buffer
}

/// The symbols of `row` from `AREA.x + from` to `AREA.x + to` (exclusive).
fn symbols(buffer: &Buffer, row: u16, from: u16, to: u16) -> String {
    (from..to)
        .map(|x| buffer.get(AREA.x + x, AREA.y + 2 + row).unwrap().symbol)
        .collect()
}

/// The name cells: the Hangul rows fill their cells in pairs; the cut row
/// ends in "…", a padding space and the separator; the ASCII row is padded
/// with spaces.
fn assert_names(buffer: &Buffer) {
    for (row, chars) in [(0u16, 10u16), (1, 9)] {
        for i in 0..chars {
            let cell = buffer.get(AREA.x + i * 2, AREA.y + 2 + row).unwrap();
            assert_eq!(
                cell.symbol,
                if i % 2 == 0 { '렌' } else { '더' },
                "row {row}"
            );
        }
    }
    assert_eq!(symbols(buffer, 1, 18, 21), "…  ");
    assert_eq!(symbols(buffer, 2, 0, 21), format!("{:<21}", "abc"));
}

#[test]
fn counts_view_draws_the_count_at_the_same_column_for_wide_names() {
    let buffer = draw(ProfilerView::Counts);
    assert_names(&buffer);
    let accent = DevToolsConfig::default().accent_color;
    for (row, count) in [(0u16, '3'), (1, '2'), (2, '1')] {
        // " {:>6}" after the 20-column name: columns 20..=26, the count last
        assert_eq!(symbols(&buffer, row, 20, 26), "      ", "row {row}");
        assert_eq!(
            symbols(&buffer, row, 26, 27),
            count.to_string(),
            "row {row}"
        );
        // The bar starts at column 28, after one plain cell
        let gap = buffer.get(AREA.x + 27, AREA.y + 2 + row).unwrap();
        assert_eq!((gap.symbol, gap.bg), (' ', None), "row {row}");
        let bar = buffer.get(AREA.x + 28, AREA.y + 2 + row).unwrap();
        assert_eq!((bar.symbol, bar.bg), (' ', Some(accent)), "row {row}");
    }
}

#[test]
fn ranked_view_starts_the_stats_at_the_same_column_for_wide_names() {
    let buffer = draw(ProfilerView::Ranked);
    assert_names(&buffer);
    for (row, total) in [(0u16, "3.00"), (1, "2.00"), (2, "1.00")] {
        // " {:>6.2}ms total" after the 20-column name
        let expected = format!("   {total}ms total");
        assert_eq!(symbols(&buffer, row, 20, 35), expected, "row {row}");
    }
}
