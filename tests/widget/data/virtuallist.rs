//! VirtualList rendering and helper tests
//!
//! The rest of VirtualList's API is covered by virtuallist_core.rs.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::data::virtuallist::{virtual_list, VirtualList};
use revue::widget::traits::{RenderContext, View};

fn row(buffer: &Buffer, y: u16, width: u16) -> String {
    (0..width)
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[test]
fn test_virtual_list_render() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let list = VirtualList::new(items).selected_style(Color::BLACK, Color::CYAN);
    list.render(&mut ctx);

    // Only the first five items fit; the last column is the scrollbar
    for y in 0..5 {
        assert_eq!(row(&buffer, y, 19), format!("Item {}", y));
    }
    // The first item is selected by default
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::CYAN));
    assert_eq!(buffer.get(0, 1).unwrap().bg, None);
}

#[test]
fn test_virtual_list_render_follows_selection() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let list = VirtualList::new(items).selected(50).show_scrollbar(false);
    list.render(&mut ctx);

    let rows: Vec<String> = (0..5).map(|y| row(&buffer, y, 20)).collect();
    assert!(
        rows.contains(&"Item 50".to_string()),
        "selected item not drawn: {rows:?}"
    );
}

#[test]
fn test_virtual_list_helper() {
    let list = virtual_list(vec!["a", "b", "c"]);
    assert_eq!(list.len(), 3);
    assert_eq!(list.selected_item(), Some(&"a"));
}
