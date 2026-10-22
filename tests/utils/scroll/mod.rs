//! `ScrollState`: the scroll model shared by views (#973)

mod model;
mod properties;

use revue::utils::ScrollState;

fn drawn(content: usize, viewport: usize) -> ScrollState {
    let s = ScrollState::new();
    s.set_content(content);
    s.set_viewport(viewport);
    s
}

/// Tail mode used to scroll to `len - 1` and show one line (#858).
#[test]
fn following_the_end_shows_a_full_last_page() {
    let s = ScrollState::new().follow_end(true);
    s.set_content(10);
    s.set_viewport(4);
    assert_eq!(s.visible(), 6..10);
    s.set_content(12);
    assert_eq!(s.visible(), 8..12);
}

#[test]
fn scrolling_up_stops_following_and_the_end_starts_it_again() {
    let mut s = drawn(10, 4).follow_end(true);
    s.scroll_by(-1);
    assert!(!s.is_following());
    s.set_content(20);
    assert_eq!(s.visible(), 5..9, "the view moved with new content below");
    s.scroll_by(100);
    assert!(s.is_following());
    s.set_content(21);
    assert_eq!(s.visible(), 17..21);
}

/// The view used to jump when the selection moved inside it (#967, #969).
#[test]
fn revealing_rows_already_on_screen_does_not_scroll() {
    let mut s = drawn(20, 5);
    s.reveal(12..13);
    assert_eq!(s.visible(), 8..13);
    s.reveal(9..10);
    assert_eq!(s.visible(), 8..13);
}

#[test]
fn reveal_scrolls_just_enough_in_either_direction() {
    let mut s = drawn(20, 5);
    s.reveal(12..13);
    assert_eq!(s.offset(), 8);
    s.reveal(3..4);
    assert_eq!(s.offset(), 3);
}

/// A tall item (a wrapped log entry, an expanded section) shows its top.
#[test]
fn a_range_taller_than_the_view_shows_its_first_rows() {
    let mut s = drawn(20, 3);
    s.reveal(10..15);
    assert_eq!(s.visible(), 10..13);
}

/// A selection set before the first draw is shown by the first draw (#1011).
#[test]
fn a_reveal_before_the_first_draw_waits_for_the_viewport() {
    let mut s = ScrollState::new();
    s.set_content(50);
    s.reveal(30..31);
    s.set_viewport(10);
    assert_eq!(s.visible(), 21..31);
}

/// Trimming old log entries above the view keeps the rows on screen.
#[test]
fn rows_removed_above_keep_the_view_on_its_rows() {
    let mut s = drawn(20, 5);
    s.scroll_to(10);
    s.rows_removed(0, 3);
    assert_eq!(s.offset(), 7);
    assert_eq!(s.content(), 17);
}

#[test]
fn rows_inserted_above_keep_the_view_on_its_rows() {
    let mut s = drawn(20, 5);
    s.scroll_to(10);
    s.rows_inserted(2, 4);
    assert_eq!(s.offset(), 14);
    s.rows_inserted(14, 1); // at the first visible row: below the anchor
    assert_eq!(s.offset(), 14);
}

/// A removal that takes the first visible rows leaves the view where they were.
#[test]
fn rows_removed_across_the_top_of_the_view() {
    let mut s = drawn(20, 5);
    s.scroll_to(10);
    s.rows_removed(8, 4);
    assert_eq!(s.offset(), 8);
}

/// httpclient and aistream kept any offset (#973 comments).
#[test]
fn the_offset_stops_at_the_last_page() {
    let mut s = drawn(6, 4);
    s.scroll_by(100);
    assert_eq!(s.offset(), 2);
    s.set_content(3);
    assert_eq!(s.offset(), 0);
}

/// terminal: a shorter area shows the newest rows while following.
#[test]
fn a_smaller_viewport_keeps_the_end_while_following() {
    let s = drawn(5, 5).follow_end(true);
    s.set_viewport(3);
    assert_eq!(s.visible(), 2..5);
}

#[test]
fn page_moves_and_markers() {
    let mut s = drawn(10, 4);
    assert!(!s.has_above() && s.has_below());
    s.page_down();
    assert_eq!(s.offset(), 4);
    s.page_down();
    assert_eq!(s.offset(), 6);
    assert!(s.has_above() && !s.has_below());
    s.page_up();
    assert_eq!(s.offset(), 2);
    s.scroll_to_top();
    assert_eq!(s.offset(), 0);
}

#[test]
fn an_empty_view() {
    let mut s = drawn(0, 5);
    s.scroll_by(3);
    s.reveal(0..1);
    assert_eq!(s.visible(), 0..0);
    assert!(!s.is_following(), "no tail mode: it never follows");
}

/// A list without a tail mode that reaches its end does not start following:
/// a longer list later keeps the view where it was.
#[test]
fn a_view_without_a_tail_mode_never_follows() {
    let mut s = drawn(3, 5);
    s.scroll_to_top();
    s.reveal(2..3);
    s.scroll_to_end();
    assert!(!s.is_following());
    s.set_content(20);
    assert_eq!(s.offset(), 0);
}

/// A paused log stays paused while the selection moves on screen.
#[test]
fn a_reveal_on_screen_leaves_following_as_it_is() {
    let mut s = drawn(3, 5).follow_end(true);
    s.set_follow_end(false);
    s.reveal(2..3);
    assert!(!s.is_following());
    s.set_content(10);
    assert_eq!(
        s.visible(),
        0..5,
        "paused: new rows below do not move the view"
    );
    s.set_follow_end(true);
    assert_eq!(s.visible(), 5..10);
}

/// `track` reveals when the rows change and leaves a scrolled view alone
/// while they do not.
#[test]
fn track_follows_changes_only() {
    let mut s = drawn(20, 5);
    s.track(12..13);
    assert_eq!(s.offset(), 8);
    s.scroll_to(0);
    s.track(12..13);
    assert_eq!(
        s.offset(),
        0,
        "an unchanged selection does not pull the view back"
    );
    s.track(13..14);
    assert_eq!(s.offset(), 9);
}

#[test]
fn track_before_the_first_draw_waits_for_the_viewport() {
    let s = ScrollState::new();
    s.set_content(30);
    s.track(20..21);
    s.set_viewport(5);
    assert_eq!(s.visible(), 16..21);
}

#[test]
fn aligned_reveals() {
    use revue::utils::ScrollAlign;
    let mut s = drawn(100, 10);
    s.reveal_aligned(50..51, ScrollAlign::Start);
    assert_eq!(s.offset(), 50);
    s.reveal_aligned(50..51, ScrollAlign::End);
    assert_eq!(s.offset(), 41);
    s.reveal_aligned(50..51, ScrollAlign::Center);
    assert_eq!(s.visible(), 45..55, "row 50 is the middle row");
    s.reveal_aligned(98..99, ScrollAlign::Center);
    assert_eq!(s.offset(), 90, "near the end it goes as far as it can");
    s.reveal_aligned(50..51, ScrollAlign::Nearest);
    assert_eq!(s.offset(), 50);
}

/// Start needs no viewport; the others wait for the first draw.
#[test]
fn aligned_reveals_before_the_first_draw() {
    use revue::utils::ScrollAlign;
    let mut s = ScrollState::new();
    s.set_content(100);
    s.reveal_aligned(40..41, ScrollAlign::Start);
    assert_eq!(s.offset(), 40);
    s.reveal_aligned(60..61, ScrollAlign::End);
    assert_eq!(s.offset(), 40, "waits for the viewport");
    s.set_viewport(10);
    assert_eq!(s.offset(), 51);
}

#[test]
fn set_offset_places_the_view_in_range() {
    let s = drawn(20, 5);
    s.set_offset(7);
    assert_eq!(s.offset(), 7);
    s.set_offset(100);
    assert_eq!(s.offset(), 15);
}
