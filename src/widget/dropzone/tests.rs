use super::*;

#[test]
fn test_dropzone_new() {
    let zone = DropZone::new("Drop here");
    assert_eq!(zone.placeholder, "Drop here");
    assert!(zone.accepts.is_empty());
}

#[test]
fn test_dropzone_accepts() {
    let zone = DropZone::new("Drop files").accepts(&["file", "text"]);

    assert_eq!(zone.accepts.len(), 2);
    assert!(zone.accepts.contains(&"file"));
    assert!(zone.accepts.contains(&"text"));
}

#[test]
fn test_dropzone_style() {
    let zone = DropZone::new("Test").style(DropZoneStyle::Dashed);

    assert_eq!(zone.style, DropZoneStyle::Dashed);
}

#[test]
fn test_dropzone_as_target() {
    let zone = DropZone::new("Test").accepts(&["text"]);

    let bounds = Rect::new(10, 5, 20, 10);
    let target = zone.as_target(bounds);

    assert_eq!(target.bounds, bounds);
    assert!(target.accepts.contains(&"text"));
}

#[test]
fn test_dropzone_hover_state() {
    let mut zone = DropZone::new("Test");

    assert!(!zone.hovered);
    zone.set_hovered(true, true);
    assert!(zone.hovered);
    assert!(zone.can_accept_current);

    zone.set_hovered(false, false);
    assert!(!zone.hovered);
}

#[test]
fn test_dropzone_draggable_trait() {
    let zone = DropZone::new("Test").accepts(&["file"]);

    assert!(zone.can_drop());
    assert_eq!(zone.accepted_types(), &["file"]);
}

#[test]
fn test_dropzone_accepts_all() {
    let zone = DropZone::new("Drop anything")
        .accepts(&["file"])
        .accepts_all();

    assert!(zone.accepts.is_empty());
}

#[test]
fn test_dropzone_colors() {
    let zone = DropZone::new("Zone")
        .border_color(Color::RED)
        .hover_color(Color::BLUE);

    assert_eq!(zone.border_color, Color::RED);
    assert_eq!(zone.hover_color, Color::BLUE);
}

#[test]
fn test_dropzone_min_height() {
    let zone = DropZone::new("Zone").min_height(5);
    assert_eq!(zone.min_height, 5);
}

#[test]
fn test_dropzone_on_drop_handler() {
    use std::cell::Cell;
    use std::rc::Rc;

    let called = Rc::new(Cell::new(false));
    let called_clone = called.clone();

    let mut zone = DropZone::new("Zone").on_drop(move |_data| {
        called_clone.set(true);
        true
    });

    let data = DragData::text("test");
    let result = Draggable::on_drop(&mut zone, data);

    assert!(result);
    assert!(called.get());
}

#[test]
fn test_dropzone_id() {
    let zone1 = DropZone::new("Zone 1");
    let zone2 = DropZone::new("Zone 2");

    // IDs should be unique
    assert_ne!(zone1.id(), zone2.id());
}

#[test]
fn test_dropzone_current_border_color() {
    let mut zone = DropZone::new("Zone");

    // Not hovered - use border_color
    assert_eq!(zone.current_border_color(), zone.border_color);

    // Hovered and can accept - use accept_color
    zone.set_hovered(true, true);
    assert_eq!(zone.current_border_color(), zone.accept_color);

    // Hovered but cannot accept - use reject_color
    zone.set_hovered(true, false);
    assert_eq!(zone.current_border_color(), zone.reject_color);
}

#[test]
fn test_dropzone_border_chars() {
    let solid = DropZone::new("Zone").style(DropZoneStyle::Solid);
    let chars = solid.border_chars();
    assert_eq!(chars.0, '┌');
    assert_eq!(chars.4, '─');

    let dashed = DropZone::new("Zone").style(DropZoneStyle::Dashed);
    let chars = dashed.border_chars();
    assert_eq!(chars.4, '╌');

    let highlight = DropZone::new("Zone").style(DropZoneStyle::Highlight);
    let chars = highlight.border_chars();
    assert_eq!(chars.0, ' ');
}

#[test]
fn test_dropzone_drag_enter_leave() {
    let mut zone = DropZone::new("Zone").accepts(&["file"]);

    let data = DragData::text("test");
    zone.on_drag_enter(&data);
    assert!(zone.hovered);

    zone.on_drag_leave();
    assert!(!zone.hovered);
    assert!(!zone.can_accept_current);
}

#[test]
fn test_dropzone_drop_bounds() {
    let zone = DropZone::new("Zone");
    let area = Rect::new(5, 5, 20, 10);
    let bounds = zone.drop_bounds(area);

    assert_eq!(bounds, area);
}

#[test]
fn test_dropzone_render() {
    use crate::render::Buffer;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let zone = DropZone::new("Drop files here");
    zone.render(&mut ctx);
}

#[test]
fn test_dropzone_render_hovered() {
    use crate::render::Buffer;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut zone = DropZone::new("Drop files here");
    zone.set_hovered(true, true);
    zone.render(&mut ctx);
}

#[test]
fn test_dropzone_render_rejected() {
    use crate::render::Buffer;

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut zone = DropZone::new("Drop files here");
    zone.set_hovered(true, false);
    zone.render(&mut ctx);
}

#[test]
fn test_dropzone_render_styles() {
    use crate::render::Buffer;

    for style in [
        DropZoneStyle::Solid,
        DropZoneStyle::Dashed,
        DropZoneStyle::Highlight,
        DropZoneStyle::Minimal,
    ] {
        let mut buffer = Buffer::new(40, 10);
        let area = Rect::new(0, 0, 40, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let zone = DropZone::new("Zone").style(style);
        zone.render(&mut ctx);
    }
}

#[test]
fn test_dropzone_render_small_area() {
    use crate::render::Buffer;

    let mut buffer = Buffer::new(5, 2);
    let area = Rect::new(0, 0, 5, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let zone = DropZone::new("This is a long placeholder text");
    zone.render(&mut ctx);
}

#[test]
fn test_dropzone_helper() {
    let zone = drop_zone("Drop here");
    assert_eq!(zone.placeholder, "Drop here");
}

#[test]
fn test_dropzone_style_default() {
    assert_eq!(DropZoneStyle::default(), DropZoneStyle::Solid);
}

#[test]
fn test_dropzone_on_drop_no_handler() {
    let mut zone = DropZone::new("Zone");
    let data = DragData::text("test");

    let result = Draggable::on_drop(&mut zone, data);
    assert!(!result); // No handler, returns false
}

#[test]
fn test_dropzone_all_styles() {
    let styles = [
        DropZoneStyle::Solid,
        DropZoneStyle::Dashed,
        DropZoneStyle::Highlight,
        DropZoneStyle::Minimal,
    ];

    for style in styles {
        let zone = DropZone::new("Zone").style(style);
        assert_eq!(zone.style, style);
    }
}

#[test]
fn test_dropzone_styled_view_id() {
    let mut zone = DropZone::new("Zone");
    zone.set_id("test-zone");
    assert_eq!(zone.props.id.as_deref(), Some("test-zone"));
}

#[test]
fn test_dropzone_styled_view_classes() {
    let mut zone = DropZone::new("Zone");

    zone.add_class("active");
    assert!(zone.has_class("active"));
    assert_eq!(zone.props.classes.len(), 1);

    zone.add_class("highlighted");
    assert!(zone.has_class("highlighted"));
    assert_eq!(zone.props.classes.len(), 2);
}

#[test]
fn test_dropzone_styled_view_remove_class() {
    let mut zone = DropZone::new("Zone");
    zone.add_class("active");
    zone.add_class("highlighted");

    zone.remove_class("active");
    assert!(!zone.has_class("active"));
    assert!(zone.has_class("highlighted"));
    assert_eq!(zone.props.classes.len(), 1);
}

#[test]
fn test_dropzone_styled_view_toggle_class() {
    let mut zone = DropZone::new("Zone");

    zone.toggle_class("active");
    assert!(zone.has_class("active"));

    zone.toggle_class("active");
    assert!(!zone.has_class("active"));
}

#[test]
fn test_dropzone_builder_focused() {
    let zone = DropZone::new("Zone").focused(true);
    assert!(zone.is_focused());

    let zone = DropZone::new("Zone").focused(false);
    assert!(!zone.is_focused());
}

#[test]
fn test_dropzone_builder_disabled() {
    let zone = DropZone::new("Zone").disabled(true);
    assert!(zone.is_disabled());

    let zone = DropZone::new("Zone").disabled(false);
    assert!(!zone.is_disabled());
}

#[test]
fn test_dropzone_builder_colors() {
    let zone = DropZone::new("Zone").fg(Color::CYAN).bg(Color::BLUE);

    assert_eq!(zone.state.fg, Some(Color::CYAN));
    assert_eq!(zone.state.bg, Some(Color::BLUE));
}

#[test]
fn test_dropzone_builder_set_focused() {
    let mut zone = DropZone::new("Zone");
    assert!(!zone.is_focused());

    zone.set_focused(true);
    assert!(zone.is_focused());

    zone.set_focused(false);
    assert!(!zone.is_focused());
}

#[test]
fn test_dropzone_drag_enter_with_accepted_type() {
    let mut zone = DropZone::new("Zone").accepts(&["text"]);

    let data = DragData::text("test content");
    zone.on_drag_enter(&data);

    assert!(zone.hovered);
    assert!(zone.can_accept_current);
}

#[test]
fn test_dropzone_drag_enter_with_rejected_type() {
    let mut zone = DropZone::new("Zone").accepts(&["file"]);

    let data = DragData::text("test content"); // text is not in accepts
    zone.on_drag_enter(&data);

    assert!(zone.hovered);
    assert!(!zone.can_accept_current);
}

#[test]
fn test_dropzone_drag_enter_accepts_all() {
    let mut zone = DropZone::new("Zone").accepts_all();

    let data = DragData::text("test content");
    zone.on_drag_enter(&data);

    assert!(zone.hovered);
    assert!(zone.can_accept_current);
}
