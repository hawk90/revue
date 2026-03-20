//! Rendering overlap and boundary tests
//!
//! Verifies widgets render within their allocated areas and don't
//! overwrite adjacent widget content. Uses buffer inspection to
//! check exact cell contents at specific positions.

use revue::layout::Rect;
use revue::render::{Buffer, Cell};
use revue::style::Color;
use revue::widget::traits::{OverlayEntry, RenderContext, View};

/// Helper: dump a row of the buffer as a string (for debugging)
#[allow(dead_code)]
fn row_text(buffer: &Buffer, y: u16, x_start: u16, x_end: u16) -> String {
    (x_start..x_end)
        .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
        .collect()
}

/// Helper: check that a rectangular region contains ONLY the expected char
fn region_is(buffer: &Buffer, x: u16, y: u16, w: u16, h: u16, expected: char) -> bool {
    for dy in 0..h {
        for dx in 0..w {
            if let Some(cell) = buffer.get(x + dx, y + dy) {
                if cell.symbol != expected {
                    return false;
                }
            }
        }
    }
    true
}

// =============================================================================
// Widget Stays Within Its Area
// =============================================================================

#[test]
fn test_select_closed_renders_in_single_row() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = revue::widget::Select::new()
        .options(vec!["Apple", "Banana"])
        .focused(true);
    s.render(&mut ctx);

    // Row 0 should have content (arrow + text)
    assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');

    // Rows 1-9 should be empty (dropdown is closed)
    for y in 1..10 {
        assert!(
            region_is(&buffer, 0, y, 30, 1, ' '),
            "row {} should be empty when Select is closed",
            y
        );
    }
}

#[test]
fn test_widget_respects_sub_area_boundary() {
    // Render a Select at a specific sub-area and verify it doesn't
    // write outside that area
    let mut buffer = Buffer::new(40, 20);

    // Fill buffer with markers
    for y in 0..20 {
        for x in 0..40 {
            buffer.set(x, y, Cell::new('·'));
        }
    }

    // Render Select in sub-area (5, 3, 20, 1)
    let area = Rect::new(5, 3, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    let s = revue::widget::Select::new()
        .options(vec!["Hello"])
        .focused(true);
    s.render(&mut ctx);

    // Check that area OUTSIDE (5,3,20,1) still has markers
    // Row 2 (above) should be untouched
    assert!(
        region_is(&buffer, 0, 2, 40, 1, '·'),
        "row above Select area should be untouched"
    );
    // Row 4 (below) should be untouched
    assert!(
        region_is(&buffer, 0, 4, 40, 1, '·'),
        "row below Select area should be untouched"
    );
    // Left of area (x=0..4, y=3) should be untouched
    for x in 0..5 {
        assert_eq!(
            buffer.get(x, 3).unwrap().symbol,
            '·',
            "cell left of Select at x={} should be untouched",
            x
        );
    }
    // Right of area (x=25..39, y=3) should be untouched
    for x in 25..40 {
        assert_eq!(
            buffer.get(x, 3).unwrap().symbol,
            '·',
            "cell right of Select at x={} should be untouched",
            x
        );
    }
}

// =============================================================================
// Modal Content Stays Within Border
// =============================================================================

#[test]
fn test_modal_content_within_border() {
    use revue::widget::Modal;

    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut modal = Modal::new()
        .title("Test Modal")
        .content("Line 1\nLine 2")
        .width(30);
    modal.show();

    modal.render(&mut ctx);

    // Find modal border corners (they should be box-drawing chars)
    let mut found_border = false;
    for y in 0..15 {
        for x in 0..40 {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '┌' {
                    found_border = true;
                    // Check opposite corner exists
                    let right = buffer.get(x + 29, y);
                    assert!(
                        right.is_some() && right.unwrap().symbol == '┐',
                        "top-right corner should be at x+29"
                    );
                }
            }
        }
    }
    assert!(found_border, "modal border should be rendered");
}

// =============================================================================
// Adjacent Widgets Don't Overlap
// =============================================================================

#[test]
fn test_two_widgets_in_adjacent_areas_dont_overlap() {
    use revue::widget::Input;

    let mut buffer = Buffer::new(40, 2);

    // Fill with markers
    for y in 0..2 {
        for x in 0..40 {
            buffer.set(x, y, Cell::new('·'));
        }
    }

    // Render Input 1 in row 0
    {
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let input1 = Input::new().value("AAAA").focused(true);
        input1.render(&mut ctx);
    }

    // Render Input 2 in row 1
    {
        let area = Rect::new(0, 1, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let input2 = Input::new().value("BBBB").focused(false);
        input2.render(&mut ctx);
    }

    // Row 0 should contain 'A's, not 'B's
    let row0 = row_text(&buffer, 0, 0, 20);
    assert!(row0.contains('A'), "row 0 should have input1 content");
    assert!(!row0.contains('B'), "row 0 should NOT have input2 content");

    // Row 1 should contain 'B's, not 'A's
    let row1 = row_text(&buffer, 1, 0, 20);
    assert!(row1.contains('B'), "row 1 should have input2 content");
    assert!(!row1.contains('A'), "row 1 should NOT have input1 content");
}

// =============================================================================
// Overlay Renders At Correct Position
// =============================================================================

#[test]
fn test_overlay_renders_at_absolute_position() {
    use revue::widget::traits::OverlayEntry;

    let mut buffer = Buffer::new(30, 10);

    // Fill with markers
    for y in 0..10 {
        for x in 0..30 {
            buffer.set(x, y, Cell::new('·'));
        }
    }

    // Create overlay entry at absolute position (5, 3)
    let mut entry = OverlayEntry::new(100, Rect::new(5, 3, 10, 2));
    for x in 0..10 {
        entry.push(x, 0, Cell::new('X'));
        entry.push(x, 1, Cell::new('X'));
    }

    // Render overlay directly
    let mut queue = revue::widget::OverlayQueue::new();
    queue.push(entry);
    queue.render_to(&mut buffer);

    // Verify overlay cells are at correct absolute positions
    assert_eq!(buffer.get(5, 3).unwrap().symbol, 'X');
    assert_eq!(buffer.get(14, 3).unwrap().symbol, 'X');
    assert_eq!(buffer.get(5, 4).unwrap().symbol, 'X');

    // Verify cells outside overlay are untouched
    assert_eq!(buffer.get(4, 3).unwrap().symbol, '·');
    assert_eq!(buffer.get(15, 3).unwrap().symbol, '·');
    assert_eq!(buffer.get(5, 2).unwrap().symbol, '·');
    assert_eq!(buffer.get(5, 5).unwrap().symbol, '·');
}

#[test]
fn test_overlay_z_order_higher_on_top() {
    let mut buffer = Buffer::new(10, 1);

    // Two overlays at same position, different z-index
    let mut low = OverlayEntry::new(10, Rect::new(0, 0, 5, 1));
    for x in 0..5 {
        low.push(x, 0, Cell::new('L'));
    }

    let mut high = OverlayEntry::new(20, Rect::new(0, 0, 5, 1));
    for x in 0..5 {
        high.push(x, 0, Cell::new('H'));
    }

    let mut queue = revue::widget::OverlayQueue::new();
    queue.push(low);
    queue.push(high);
    queue.render_to(&mut buffer);

    // Higher z-index should be on top (rendered last, overwrites)
    for x in 0..5 {
        assert_eq!(
            buffer.get(x, 0).unwrap().symbol,
            'H',
            "higher z-index should overwrite at x={}",
            x
        );
    }
}

#[test]
fn test_overlay_clipped_at_buffer_edge() {
    let mut buffer = Buffer::new(10, 5);

    // Overlay extends past buffer right edge
    let mut entry = OverlayEntry::new(100, Rect::new(8, 0, 5, 1));
    for x in 0..5 {
        entry.push(x, 0, Cell::new('X'));
    }

    let mut queue = revue::widget::OverlayQueue::new();
    queue.push(entry);
    queue.render_to(&mut buffer);

    // Cells within buffer should be written
    assert_eq!(buffer.get(8, 0).unwrap().symbol, 'X');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, 'X');
    // Cells outside buffer (x=10,11,12) are silently skipped — no panic
}

// =============================================================================
// CJK Text Rendering Width Correctness
// =============================================================================

#[test]
fn test_cjk_text_occupies_correct_width() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // "한" is 2 display columns wide
    ctx.draw_text(0, 0, "한A", Color::WHITE);

    // '한' at x=0 (2 wide), 'A' at x=2
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '한');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'A');
}

#[test]
fn test_cjk_text_clipped_at_boundary() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // "ABCDE한" — '한' needs 2 cols but only 0 left, should be skipped
    ctx.draw_text(0, 0, "ABCD한", Color::WHITE);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'A');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'D');
    // x=4 should be space (한 doesn't fit in 1 remaining column)
    assert_eq!(buffer.get(4, 0).unwrap().symbol, ' ');
}
