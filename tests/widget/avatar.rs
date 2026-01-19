//! Avatar widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{avatar, View};

#[test]
fn test_avatar_render_small() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe").small();
    a.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('J'));
}

#[test]
fn test_avatar_render_medium() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let a = avatar("John Doe");
    a.render(&mut ctx);

    // Should have initials in the middle
    let text: String = (0..10)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains('J') || text.contains('D'));
}

// =============================================================================
