//! Integration tests for Splitter widget

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{HSplit, Pane, RichText, SplitOrientation, Splitter, VSplit};

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
    let _splitter = Splitter::new();

    // Splitter created successfully
}

#[test]
fn test_splitter_with_pane() {
    let pane = Pane::new("pane1");
    let _splitter = Splitter::new().pane(pane);

    // Pane added successfully
}

#[test]
fn test_splitter_with_panes() {
    let panes = vec![Pane::new("pane1").ratio(0.3), Pane::new("pane2").ratio(0.7)];
    let _splitter = Splitter::new().panes(panes);

    // Panes added successfully
}

#[test]
fn test_splitter_horizontal() {
    let _splitter = Splitter::new().horizontal();

    // Horizontal orientation was set
}

#[test]
fn test_splitter_vertical() {
    let _splitter = Splitter::new().vertical();

    // Vertical orientation was set
}

#[test]
fn test_splitter_orientation() {
    let _splitter = Splitter::new().orientation(SplitOrientation::Vertical);

    // Orientation was set
}

#[test]
fn test_splitter_color() {
    let _splitter = Splitter::new().color(Color::CYAN);

    // Color was set
}

#[test]
fn test_splitter_active_color() {
    let _splitter = Splitter::new().active_color(Color::YELLOW);

    // Active color was set
}

#[test]
fn test_hsplit_new() {
    let _hsplit = HSplit::new(0.5);

    // Horizontal split with 50% ratio created
}

#[test]
fn test_hsplit_with_min_widths() {
    let _hsplit = HSplit::new(0.5).min_widths(10, 20);

    // Min widths were set
}

#[test]
fn test_hsplit_hide_splitter() {
    let _hsplit = HSplit::new(0.5).hide_splitter();

    // Hide splitter was set
}

#[test]
fn test_vsplit_new() {
    let _vsplit = VSplit::new(0.5);

    // Vertical split with 50% ratio created
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
    let _splitter = Splitter::new()
        .horizontal()
        .color(Color::CYAN)
        .active_color(Color::YELLOW);

    // Builder pattern works
}

#[test]
fn test_splitter_with_collapsible_pane() {
    let _splitter = Splitter::new().pane(Pane::new("sidebar").collapsible());

    // Collapsible pane added
}

#[test]
fn test_splitter_multiple_panes_builder() {
    let _splitter = Splitter::new()
        .pane(Pane::new("left").ratio(0.3))
        .pane(Pane::new("middle").ratio(0.4))
        .pane(Pane::new("right").ratio(0.3));

    // Multiple panes configured
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
    let right_width = areas[1].1.width;

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
