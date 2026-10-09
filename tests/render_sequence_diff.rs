//! The diff compares what a cell's hyperlink and escape sequence say, not
//! their ids: each buffer numbers them in its own registry, and the app
//! alternates two buffers, so an equal id can mean different content (#864)

use revue::render::{diff, Buffer, Cell};

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
