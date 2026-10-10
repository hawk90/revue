//! Diff tests

use revue::layout::Rect;
use revue::render::{diff, Buffer, Cell};

// Helper to create a test rect
fn rect(x: u16, y: u16, width: u16, height: u16) -> Rect {
    Rect {
        x,
        y,
        width,
        height,
    }
}

#[test]
fn test_diff_empty_rects_fallbacks_to_full_diff() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X'));

    // No dirty rects should behave like a full diff for now
    let changes = diff(&buf1, &buf2, &[]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 5);
}

#[test]
fn test_diff_single_dirty_rect() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X')); // Change is inside the rect

    let changes = diff(&buf1, &buf2, &[rect(5, 5, 1, 1)]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 5);
    assert_eq!(changes[0].y, 5);
}

#[test]
fn test_diff_change_outside_dirty_rect() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X')); // Change is outside the rect

    let changes = diff(&buf1, &buf2, &[rect(0, 0, 1, 1)]);
    assert!(changes.is_empty());
}

#[test]
fn test_diff_multiple_dirty_rects() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(1, 1, Cell::new('A'));
    buf2.set(8, 8, Cell::new('B'));

    let dirty_rects = vec![rect(1, 1, 1, 1), rect(8, 8, 1, 1)];
    let changes = diff(&buf1, &buf2, &dirty_rects);
    assert_eq!(changes.len(), 2);
}

#[test]
fn test_diff_overlapping_dirty_rects() {
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(2, 2, Cell::new('C'));

    // Overlapping rects, both containing the change
    let dirty_rects = vec![rect(0, 0, 5, 5), rect(2, 2, 5, 5)];
    let changes = diff(&buf1, &buf2, &dirty_rects);

    // HashSet should ensure we only get one change
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 2);
    assert_eq!(changes[0].y, 2);
}

#[test]
fn test_original_diff_logic_with_full_rect() {
    let full_rect = rect(0, 0, 10, 10);

    // test_diff_identical_buffers
    let buf1 = Buffer::new(10, 10);
    let buf2 = Buffer::new(10, 10);
    let changes = diff(&buf1, &buf2, &[full_rect]);
    assert!(changes.is_empty());

    // test_diff_single_change
    let mut buf2_single = buf1.clone();
    buf2_single.set(5, 5, Cell::new('X'));
    let changes_single = diff(&buf1, &buf2_single, &[full_rect]);
    assert_eq!(changes_single.len(), 1);

    // test_diff_multiple_changes
    let mut buf2_multi = buf1.clone();
    buf2_multi.put_str(0, 0, "Hello");
    let changes_multi = diff(&buf1, &buf2_multi, &[full_rect]);
    assert_eq!(changes_multi.len(), 5);

    // test_diff_no_change_same_content
    let mut buf1_same = buf1.clone();
    let mut buf2_same = buf1.clone();
    buf1_same.set(5, 5, Cell::new('X'));
    buf2_same.set(5, 5, Cell::new('X'));
    let changes_same = diff(&buf1_same, &buf2_same, &[full_rect]);
    assert!(changes_same.is_empty());
}

#[test]
fn test_diff_no_overflow_near_u16_max() {
    // Test that diff doesn't panic with rects that would overflow u16::MAX
    // This is the fix for issue #145
    let buf1 = Buffer::new(100, 100);
    let buf2 = Buffer::new(100, 100);

    // Rect where x + width would overflow
    let overflow_x = rect(u16::MAX - 5, 0, 10, 1);
    let changes = diff(&buf1, &buf2, &[overflow_x]);
    assert!(changes.is_empty()); // No changes, but importantly no panic

    // Rect where y + height would overflow
    let overflow_y = rect(0, u16::MAX - 5, 1, 10);
    let changes = diff(&buf1, &buf2, &[overflow_y]);
    assert!(changes.is_empty());

    // Rect where both would overflow
    let overflow_both = rect(u16::MAX - 5, u16::MAX - 5, 10, 10);
    let changes = diff(&buf1, &buf2, &[overflow_both]);
    assert!(changes.is_empty());

    // Rect at exact u16::MAX
    let at_max = rect(u16::MAX, u16::MAX, 1, 1);
    let changes = diff(&buf1, &buf2, &[at_max]);
    assert!(changes.is_empty());
}

#[test]
fn test_diff_rect_exceeds_buffer() {
    // Test that rects larger than the buffer are handled correctly
    let buf1 = Buffer::new(10, 10);
    let mut buf2 = Buffer::new(10, 10);
    buf2.set(5, 5, Cell::new('X'));

    // Rect larger than buffer
    let large_rect = rect(0, 0, 1000, 1000);
    let changes = diff(&buf1, &buf2, &[large_rect]);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].x, 5);
    assert_eq!(changes[0].y, 5);
}

/// A one-cell buffer whose cell carries `seq` as its escape sequence
fn with_sequence(seq: &str) -> Buffer {
    let mut buffer = Buffer::new(1, 1);
    let id = buffer.register_sequence(seq);
    buffer.set(0, 0, Cell::new('x').sequence(id));
    buffer
}

fn with_link(url: &str) -> Buffer {
    let mut buffer = Buffer::new(1, 1);
    let id = buffer.register_hyperlink(url);
    buffer.set(0, 0, Cell::new('x').hyperlink(id));
    buffer
}

#[test]
fn a_new_sequence_under_the_same_id_is_a_change() {
    // A BigText clock in OSC 66 mode: "10:00" then "10:01", both id 0
    let old = with_sequence("10:00");
    let new = with_sequence("10:01");
    assert_eq!(diff(&old, &new, &[]).len(), 1);
}

#[test]
fn a_new_url_under_the_same_id_is_a_change() {
    let old = with_link("https://a.example");
    let new = with_link("https://b.example");
    assert_eq!(diff(&old, &new, &[]).len(), 1);
}

#[test]
fn the_same_content_under_another_id_is_no_change() {
    let old = with_link("https://a.example");
    let mut new = Buffer::new(1, 1);
    new.register_hyperlink("https://other.example");
    let id = new.register_hyperlink("https://a.example");
    assert_eq!(id, 1);
    new.set(0, 0, Cell::new('x').hyperlink(id));
    assert!(diff(&old, &new, &[]).is_empty());
}

#[test]
fn clear_forgets_the_registries() {
    let mut buffer = with_sequence("seq");
    buffer.register_hyperlink("https://a.example");
    buffer.clear();
    assert!(buffer.sequences().is_empty());
    assert!(buffer.hyperlinks().is_empty());
    // A link registered again gets a fresh id that resolves
    let id = buffer.register_hyperlink("https://a.example");
    assert_eq!(buffer.get_hyperlink(id), Some("https://a.example"));
}

#[test]
fn clear_hyperlinks_forgets_the_cached_ids() {
    let mut buffer = Buffer::new(1, 1);
    buffer.register_hyperlink("https://a.example");
    buffer.register_hyperlink("https://b.example");
    buffer.clear_hyperlinks();
    let id = buffer.register_hyperlink("https://b.example");
    assert_eq!(buffer.get_hyperlink(id), Some("https://b.example"));
}

#[test]
fn a_sequence_registered_twice_keeps_one_id() {
    let mut buffer = Buffer::new(1, 1);
    let first = buffer.register_sequence("same");
    let second = buffer.register_sequence("same");
    assert_eq!(first, second);
    assert_eq!(buffer.sequences().len(), 1);
}

#[test]
fn copy_from_brings_the_links_its_cells_point_to() {
    let src = with_link("https://a.example");
    let mut dst = Buffer::new(1, 1);
    dst.register_hyperlink("https://stale.example");
    dst.copy_from(&src);
    let id = dst.get(0, 0).unwrap().hyperlink_id.unwrap();
    assert_eq!(dst.get_hyperlink(id), Some("https://a.example"));
}
