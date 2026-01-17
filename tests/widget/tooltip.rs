//! Tooltip widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{tooltip, Tooltip, TooltipPosition, TooltipStyle, View};

#[test]
fn test_tooltip_visibility() {
    let mut t = Tooltip::new("Test");

    t.hide();
    assert!(!t.is_visible());

    t.show();
    // Note: is_visible checks delay too, but delay defaults to 0
    assert!(t.is_visible());

    t.toggle();
    assert!(!t.is_visible());
}

#[test]
fn test_tooltip_delay() {
    let mut t = Tooltip::new("Test").delay(5);
    assert!(!t.is_visible());

    for _ in 0..4 {
        t.tick();
    }
    assert!(!t.is_visible());

    t.tick();
    assert!(t.is_visible());
}

#[test]
fn test_tooltip_render() {
    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(0, 0, 40, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let t = Tooltip::new("Hello World")
        .anchor(20, 10)
        .position(TooltipPosition::Top)
        .style(TooltipStyle::Bordered);

    t.render(&mut ctx);
    // Smoke test - renders without panic
}

#[test]
fn test_tooltip_helper() {
    let t = tooltip("Quick tooltip");
    assert!(t.is_visible());
}

// =============================================================================
