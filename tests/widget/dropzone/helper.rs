//! Drop zone helper function tests

use revue::event::drag::DragData;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{Draggable, RenderContext, View};
use revue::widget::{drop_zone, DropZone, DropZoneStyle};

fn placeholder_row(zone: &impl View, width: u16) -> String {
    let mut buffer = Buffer::new(width, 3);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, 3));
    zone.render(&mut ctx);
    (0..width)
        .map(|x| buffer.get(x, 1).unwrap().symbol)
        .collect()
}

#[test]
fn test_drop_zone_function() {
    let zone = drop_zone("Drop files here");
    assert!(zone.id() > 0);
    assert!(placeholder_row(&zone, 30).contains("Drop files here"));
}

#[test]
fn test_drop_zone_function_matches_new() {
    let a = drop_zone("Upload");
    let b = DropZone::new("Upload");
    assert_eq!(placeholder_row(&a, 20), placeholder_row(&b, 20));
    assert_eq!(a.get_min_height(), b.get_min_height());
    assert_eq!(a.accepted_types(), b.accepted_types());
}

#[test]
fn test_drop_zone_function_with_string() {
    let zone = drop_zone("Upload files".to_string());
    assert!(placeholder_row(&zone, 30).contains("Upload files"));
}

#[test]
fn test_drop_zone_function_full_chain() {
    let zone = drop_zone("Upload")
        .accepts(&["file"])
        .style(DropZoneStyle::Solid)
        .min_height(10)
        .focused(true);

    assert!(zone.is_focused());
    assert_eq!(zone.get_min_height(), 10);
    assert_eq!(Draggable::accepted_types(&zone), &["file"]);
}

#[test]
fn test_drop_zone_function_empty_string() {
    let zone = drop_zone("");
    let row = placeholder_row(&zone, 10);
    // Only the side borders on the middle row
    assert_eq!(row, "│        │");
}

#[test]
fn test_drop_zone_function_unicode() {
    let zone = drop_zone("파일을 여기에 놓으세요");
    let row = placeholder_row(&zone, 40);
    assert!(row.contains('파'));
    assert!(row.contains('요'));
}

#[test]
fn test_drop_zone_function_long_text_is_clipped() {
    let long_text = "This is a very long placeholder text for the drop zone widget";
    let zone = drop_zone(long_text);
    let row = placeholder_row(&zone, 20);
    assert!(row.contains("This is a"));
    assert!(!row.contains("widget"));
    // The right border is not overwritten
    assert!(row.ends_with('│'));
}

#[test]
fn test_drop_zone_function_multiple_instances() {
    let zone1 = drop_zone("Zone 1");
    let zone2 = drop_zone("Zone 2");
    let zone3 = drop_zone("Zone 3");

    assert_ne!(zone1.id(), zone2.id());
    assert_ne!(zone2.id(), zone3.id());
    assert_ne!(zone1.id(), zone3.id());
}

#[test]
fn test_drop_zone_function_complete_with_handler() {
    let mut zone = drop_zone("Upload files")
        .accepts(&["file", "image"])
        .style(DropZoneStyle::Highlight)
        .on_drop(|data| data.type_id == "file");

    assert_eq!(Draggable::accepted_types(&zone), &["file", "image"]);
    assert!(Draggable::on_drop(&mut zone, DragData::file("/tmp/a.txt")));
    assert!(!Draggable::on_drop(&mut zone, DragData::text("hello")));
}
