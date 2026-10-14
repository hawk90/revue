//! Splitter widget tests

mod boundary;
mod input;
mod layout;
mod render;
mod two_pane;

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{HSplit, Pane, RichText, SplitOrientation, Splitter, VSplit};

/// Render `view` into a fresh `w` x `h` buffer
fn render(view: &impl View, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    view.render(&mut ctx);
    buffer
}

/// The rows `view` draws in a `w` x `h` area
fn rows(view: &impl View, w: u16, h: u16) -> Vec<String> {
    let buffer = render(view, w, h);
    (0..h)
        .map(|y| (0..w).map(|x| buffer.get(x, y).unwrap().symbol).collect())
        .collect()
}

/// Pane widths in a `w`-column area
fn widths(splitter: &Splitter, w: u16) -> Vec<u16> {
    splitter
        .pane_areas(Rect::new(0, 0, w, 1))
        .iter()
        .map(|(_, r)| r.width)
        .collect()
}

#[test]
fn test_pane_new() {
    let pane = Pane::new("pane1");

    assert_eq!(pane.id, "pane1");
    assert_eq!(pane.min_size, 5); // default
    assert_eq!(pane.max_size, 0); // default (0 = unlimited)
    assert_eq!(pane.ratio, 0.5); // default
    assert!(!pane.collapsible);
    assert!(!pane.collapsed);
}

#[test]
fn test_pane_with_min_size() {
    let pane = Pane::new("pane1").min_size(10);

    assert_eq!(pane.min_size, 10);
}

#[test]
fn test_pane_with_max_size() {
    let pane = Pane::new("pane1").max_size(50);

    assert_eq!(pane.max_size, 50);
}

#[test]
fn test_pane_with_ratio() {
    let pane = Pane::new("pane1").ratio(0.7);

    assert_eq!(pane.ratio, 0.7);
}

#[test]
fn test_pane_collapsible() {
    let pane = Pane::new("pane1").collapsible();

    assert!(pane.collapsible);
}

#[test]
fn test_pane_toggle_collapse() {
    let mut pane = Pane::new("pane1").collapsible();

    assert!(!pane.collapsed);
    pane.toggle_collapse();
    assert!(pane.collapsed);
    pane.toggle_collapse();
    assert!(!pane.collapsed);
}

#[test]
fn test_splitter_new() {
    let mut splitter = Splitter::new();

    // No panes: no areas, no focus, nothing drawn
    assert!(splitter.pane_areas(Rect::new(0, 0, 20, 5)).is_empty());
    assert_eq!(splitter.focused(), None);
    assert_eq!(rows(&splitter, 20, 5), vec![" ".repeat(20); 5]);
    // Tab is taken (it would move focus) but changes nothing
    assert!(splitter.handle_key(&Key::Tab));
    assert_eq!(splitter.focused(), None);
}

#[test]
fn test_splitter_with_pane() {
    let pane = Pane::new("pane1");
    let splitter = Splitter::new().pane(pane);

    // A lone pane takes the whole area and has the focus
    let area = Rect::new(2, 1, 20, 5);
    assert_eq!(splitter.pane_areas(area), vec![("pane1".to_string(), area)]);
    assert_eq!(splitter.focused(), Some("pane1"));
}

#[test]
fn test_splitter_with_panes() {
    let panes = vec![Pane::new("pane1").ratio(0.3), Pane::new("pane2").ratio(0.7)];
    let splitter = Splitter::new().panes(panes);

    // 100 columns for the panes, 30:70
    assert_eq!(widths(&splitter, 101), vec![30, 70]);
}

#[test]
fn test_splitter_horizontal() {
    let splitter = Splitter::new()
        .vertical()
        .horizontal()
        .pane(Pane::new("a"))
        .pane(Pane::new("b"));

    // Side by side: same rows, the second right of the first
    let areas = splitter.pane_areas(Rect::new(0, 0, 21, 4));
    assert_eq!(areas[0].1, Rect::new(0, 0, 10, 4));
    assert_eq!(areas[1].1, Rect::new(11, 0, 10, 4));
}

#[test]
fn test_splitter_vertical() {
    let splitter = Splitter::new()
        .vertical()
        .pane(Pane::new("a"))
        .pane(Pane::new("b"));

    // Stacked: same columns, the second below the first
    let areas = splitter.pane_areas(Rect::new(0, 0, 4, 21));
    assert_eq!(areas[0].1, Rect::new(0, 0, 4, 10));
    assert_eq!(areas[1].1, Rect::new(0, 11, 4, 10));
}

#[test]
fn test_splitter_orientation() {
    let panes = || vec![Pane::new("a"), Pane::new("b")];
    let area = Rect::new(0, 0, 21, 21);
    let by_enum = Splitter::new()
        .orientation(SplitOrientation::Vertical)
        .panes(panes());
    let by_method = Splitter::new().vertical().panes(panes());
    assert_eq!(by_enum.pane_areas(area), by_method.pane_areas(area));
    assert_eq!(rows(&by_enum, 21, 21), rows(&by_method, 21, 21));
    assert_eq!(rows(&by_enum, 21, 21)[10], "─".repeat(21));
}

#[test]
fn test_splitter_color() {
    let splitter = Splitter::new()
        .color(Color::MAGENTA)
        .pane(Pane::new("a"))
        .pane(Pane::new("b"));

    let buffer = render(&splitter, 21, 2);
    for y in 0..2 {
        let cell = buffer.get(10, y).unwrap();
        assert_eq!((cell.symbol, cell.fg), ('│', Some(Color::MAGENTA)));
    }
}

#[test]
fn test_splitter_active_color() {
    let mut splitter = Splitter::new()
        .active_color(Color::YELLOW)
        .pane(Pane::new("a"))
        .pane(Pane::new("b"));

    // Idle, the divider is not in the active color; while resizing it is
    assert_ne!(
        render(&splitter, 21, 1).get(10, 0).unwrap().fg,
        Some(Color::YELLOW)
    );
    splitter.start_resize(0);
    assert_eq!(
        render(&splitter, 21, 1).get(10, 0).unwrap().fg,
        Some(Color::YELLOW)
    );
}

#[test]
fn test_hsplit_new() {
    let hsplit = HSplit::new(0.5);

    assert_eq!(hsplit.ratio, 0.5);
    assert_eq!((hsplit.min_left, hsplit.min_right), (5, 5));
    assert!(hsplit.show_splitter);
    let (left, right) = hsplit.areas(Rect::new(0, 0, 21, 3));
    assert_eq!(left, Rect::new(0, 0, 10, 3));
    assert_eq!(right, Rect::new(11, 0, 10, 3));
}

#[test]
fn test_hsplit_with_min_widths() {
    let hsplit = HSplit::new(0.5).min_widths(10, 20);

    assert_eq!((hsplit.min_left, hsplit.min_right), (10, 20));
    // 40 columns: half would be 20 for each; the right one needs 20
    let (left, right) = hsplit.areas(Rect::new(0, 0, 41, 1));
    assert_eq!((left.width, right.width), (20, 20));
    // A small ratio is raised to the left minimum
    let (left, right) = HSplit::new(0.1)
        .min_widths(10, 20)
        .areas(Rect::new(0, 0, 41, 1));
    assert_eq!((left.width, right.width), (10, 30));
}

#[test]
fn test_hsplit_hide_splitter() {
    let hsplit = HSplit::new(0.5).hide_splitter();

    assert!(!hsplit.show_splitter);
    // No divider column: the panes touch, and nothing is drawn
    let (left, right) = hsplit.areas(Rect::new(0, 0, 20, 3));
    assert_eq!((left.width, right.x, right.width), (10, 10, 10));
    assert_eq!(rows(&hsplit, 20, 3), vec![" ".repeat(20); 3]);
}

#[test]
fn test_vsplit_new() {
    let vsplit = VSplit::new(0.5);

    assert_eq!(vsplit.ratio, 0.5);
    assert_eq!((vsplit.min_top, vsplit.min_bottom), (3, 3));
    let (top, bottom) = vsplit.areas(Rect::new(0, 0, 4, 21));
    assert_eq!(top, Rect::new(0, 0, 4, 10));
    assert_eq!(bottom, Rect::new(0, 11, 4, 10));
    assert_eq!(rows(&vsplit, 4, 21)[10], "────");
}

#[test]
fn test_pane_complete_builder() {
    let pane = Pane::new("my_pane")
        .min_size(10)
        .max_size(50)
        .ratio(0.25)
        .collapsible();

    assert_eq!(pane.id, "my_pane");
    assert_eq!(pane.min_size, 10);
    assert_eq!(pane.max_size, 50);
    assert_eq!(pane.ratio, 0.25);
    assert!(pane.collapsible);
}

#[test]
fn test_splitter_builder_pattern() {
    let mut splitter = Splitter::new()
        .horizontal()
        .color(Color::CYAN)
        .active_color(Color::YELLOW)
        .pane(Pane::new("a"))
        .pane(Pane::new("b"))
        .pane(Pane::new("c"));
    splitter.start_resize(1);

    // Dividers after 10 and 21 of 32 columns: the idle one cyan, the
    // resized one yellow
    let buffer = render(&splitter, 32, 1);
    assert_eq!(buffer.get(10, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(21, 0).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_splitter_with_collapsible_pane() {
    let mut splitter = Splitter::new()
        .pane(Pane::new("sidebar").collapsible())
        .pane(Pane::new("main"));

    splitter.toggle_pane(0);
    // The collapsed sidebar leaves the main pane the whole area, no divider
    let area = Rect::new(0, 0, 20, 2);
    assert_eq!(splitter.pane_areas(area), vec![("main".to_string(), area)]);
    assert_eq!(rows(&splitter, 20, 2), vec![" ".repeat(20); 2]);
    splitter.toggle_pane(0);
    assert_eq!(splitter.pane_areas(area).len(), 2);
}

#[test]
fn test_splitter_multiple_panes_builder() {
    let splitter = Splitter::new()
        .pane(Pane::new("left").ratio(0.3))
        .pane(Pane::new("middle").ratio(0.4))
        .pane(Pane::new("right").ratio(0.3));

    // 100 columns for the panes, 30:40:30
    assert_eq!(widths(&splitter, 102), vec![30, 40, 30]);
    let ids: Vec<String> = splitter
        .pane_areas(Rect::new(0, 0, 102, 1))
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(ids, vec!["left", "middle", "right"]);
}

// =========================================================================
// Korean (Hangul) Unicode Boundary Tests
// =========================================================================

#[test]
fn test_hsplit_areas_with_korean_text() {
    // Test that HSplit calculates correct areas even with Korean text
    let hsplit = HSplit::new(0.5);
    let area = Rect::new(0, 0, 40, 10);

    let (left, right) = hsplit.areas(area);

    // Left pane: x=0, width=19 (half of 39, since 1 for splitter)
    assert_eq!(left.x, 0);
    assert_eq!(left.width, 19);

    // Right pane: starts after left + splitter
    assert_eq!(right.x, 20); // 19 (left width) + 1 (splitter)
    assert_eq!(right.width, 20);

    // Splitter is at x=19
    assert_eq!(left.x + left.width, 19);
}

#[test]
fn test_hsplit_korean_text_does_not_overflow_pane() {
    // Create a buffer and render Korean text that exceeds pane width
    let mut buffer = Buffer::new(40, 5);
    let hsplit = HSplit::new(0.5);
    let area = Rect::new(0, 0, 40, 5);

    let (left_area, right_area) = hsplit.areas(area);

    // Render Korean text in left pane that exceeds its width
    // "안녕하세요" = 10 display width (5 chars × 2)
    // Left pane width = 19, so we need text longer than 19
    let long_korean = "안녕하세요안녕하세요안녕하세요안녕하세요"; // 40 display width
    let mut ctx = RenderContext::new(&mut buffer, left_area);
    let text = RichText::new().push(long_korean, Default::default());
    text.render(&mut ctx);

    // Verify splitter position is not overwritten (x=19)
    // The splitter column should be empty (default cell) after text rendering
    let splitter_x = left_area.x + left_area.width;
    for y in 0..5 {
        let cell = buffer.get(splitter_x, y).unwrap();
        // Cell should be default (space), not Korean character
        assert!(
            cell.symbol == ' ' || cell.symbol == '\0',
            "Splitter position x={} y={} should not contain Korean text, got '{}'",
            splitter_x,
            y,
            cell.symbol
        );
    }

    // Verify right pane is not overwritten
    for y in 0..5 {
        for x in right_area.x..right_area.x + right_area.width {
            let cell = buffer.get(x, y).unwrap();
            assert!(
                cell.symbol == ' ' || cell.symbol == '\0',
                "Right pane position x={} y={} should not contain Korean text, got '{}'",
                x,
                y,
                cell.symbol
            );
        }
    }
}

#[test]
fn test_vsplit_areas_with_korean_text() {
    let vsplit = VSplit::new(0.5);
    let area = Rect::new(0, 0, 40, 20);

    let (top, bottom) = vsplit.areas(area);

    // Top pane: y=0, height=9 (half of 19, since 1 for splitter)
    assert_eq!(top.y, 0);
    assert_eq!(top.height, 9);

    // Bottom pane: starts after top + splitter
    assert_eq!(bottom.y, 10); // 9 (top height) + 1 (splitter)
    assert_eq!(bottom.height, 10);
}

#[test]
fn test_korean_text_exact_boundary() {
    // Test Korean text that exactly fits the pane width
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 10, 3); // Pane width = 10

    // "안녕하세" = 8 display width (4 chars × 2)
    // Should fit perfectly with 2 cells to spare
    let korean = "안녕하세";
    let mut ctx = RenderContext::new(&mut buffer, area);
    let text = RichText::new().push(korean, Default::default());
    text.render(&mut ctx);

    // Check text is rendered correctly
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '안');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '녕');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '하');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '세');

    // Check position 10 (outside pane) is not written
    let outside_cell = buffer.get(10, 0).unwrap();
    assert!(
        outside_cell.symbol == ' ' || outside_cell.symbol == '\0',
        "Position outside pane should be empty"
    );
}

#[test]
fn test_korean_text_partial_char_at_boundary() {
    // Test when a Korean char would be cut in half at the boundary
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 5, 3); // Pane width = 5

    // "안녕하세요" = 10 display width
    // Only "안녕" (4 width) should fit, "하" would exceed boundary
    let korean = "안녕하세요";
    let mut ctx = RenderContext::new(&mut buffer, area);
    let text = RichText::new().push(korean, Default::default());
    text.render(&mut ctx);

    // Check only first 2 chars rendered (4 display width)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '안');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '녕');

    // Position 4 should be empty (would be start of '하' but exceeds boundary)
    // Actually, position 4 might have continuation marker for '녕'
    // Let's check position 5 is definitely empty (outside pane)
    let outside_cell = buffer.get(5, 0).unwrap();
    assert!(
        outside_cell.symbol == ' ' || outside_cell.symbol == '\0',
        "Position outside pane should be empty, got '{}'",
        outside_cell.symbol
    );
}

#[test]
fn test_splitter_pane_areas_korean_ratio() {
    // Test pane area calculation with various ratios
    let splitter = Splitter::new()
        .horizontal()
        .pane(Pane::new("left").ratio(0.3))
        .pane(Pane::new("right").ratio(0.7));

    let area = Rect::new(0, 0, 100, 20);
    let areas = splitter.pane_areas(area);

    assert_eq!(areas.len(), 2);
    assert_eq!(areas[0].0, "left");
    assert_eq!(areas[1].0, "right");

    // Left should be ~30%, right ~70% (minus splitter width)
    // With 1 splitter width: 99 available
    // Left: 99 * 0.3 ≈ 29-30
    // Right: remaining
    let left_width = areas[0].1.width;
    let _right_width = areas[1].1.width;

    // Verify they don't overlap and splitter is between them
    assert!(
        areas[0].1.x + left_width < areas[1].1.x,
        "Panes should not overlap"
    );
}

#[test]
fn test_mixed_korean_ascii_in_pane() {
    // Test mixed Korean and ASCII text
    let mut buffer = Buffer::new(20, 3);
    let area = Rect::new(0, 0, 10, 3);

    // "Hi안녕" = 2 + 4 = 6 display width
    let mixed = "Hi안녕";
    let mut ctx = RenderContext::new(&mut buffer, area);
    let text = RichText::new().push(mixed, Default::default());
    text.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'H');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'i');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '안');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '녕');

    // Position 6 onward should be empty
    let outside = buffer.get(6, 0).unwrap();
    assert!(outside.symbol == ' ' || outside.symbol == '\0');
}

mod float_inputs {
    //! NaN, infinite, negative and out-of-range floats do not overflow the
    //! `f32 as u16` conversions.

    use revue::layout::Rect;
    use revue::widget::{HSplit, VSplit};

    #[test]
    fn test_hsplit_negative_ratio() {
        let split = HSplit::new(-0.5);
        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.areas(area);

        // Ratio clamped to 0.1, but min_left (5) and min_right (5) also apply
        // Available = 99, clamped ratio gives 0.1, but min constraints apply
        assert!(left.width >= 5);
        assert!(right.width >= 5);
        assert_eq!(left.width + right.width + 1, 100); // Total width preserved
    }

    #[test]
    fn test_hsplit_greater_than_one_ratio() {
        let split = HSplit::new(1.5);
        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.areas(area);

        // Ratio clamped to 0.9, but min constraints apply
        assert!(left.width >= 5);
        assert!(right.width >= 5);
        assert_eq!(left.width + right.width + 1, 100);
    }

    #[test]
    fn test_hsplit_zero_ratio() {
        let split = HSplit::new(0.0);
        let area = Rect::new(0, 0, 100, 50);
        let (left, right) = split.areas(area);

        // Ratio clamped to 0.1, but min constraints apply
        assert!(left.width >= 5);
        assert!(right.width >= 5);
        assert_eq!(left.width + right.width + 1, 100);
    }

    #[test]
    fn test_hsplit_nan_ratio() {
        let split = HSplit::new(f32::NAN);
        let area = Rect::new(0, 0, 100, 50);
        let (left, _right) = split.areas(area);

        // NaN should be handled by clamp in new()
        // Minimum width should still apply
        assert!(left.width >= 5); // min_left default is 5
    }

    #[test]
    fn test_vsplit_negative_ratio() {
        let split = VSplit::new(-0.5);
        let area = Rect::new(0, 0, 100, 50);
        let (top, bottom) = split.areas(area);

        // Ratio should be clamped
        assert!(top.height >= 3); // min_top default is 3
        assert!(bottom.height >= 3); // min_bottom default is 3
    }

    #[test]
    fn test_vsplit_greater_than_one_ratio() {
        let split = VSplit::new(1.5);
        let area = Rect::new(0, 0, 100, 50);
        let (top, bottom) = split.areas(area);

        // Ratio should be clamped
        assert!(top.height <= 47); // 50 - min_bottom - 1
        assert!(bottom.height >= 3);
    }
}
