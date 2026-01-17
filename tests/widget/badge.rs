//! Badge widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{badge, dot_badge, BadgeVariant, View};

fn test_badge_render() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = badge("NEW").primary();
    b.render(&mut ctx);

    let text: String = (0..20)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("NEW"));
}

#[test]
fn test_badge_dot_render() {
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = dot_badge().success();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).map(|c| c.symbol), Some('●'));
}

#[test]
fn test_variant_colors() {
    let (bg, fg) = BadgeVariant::Success.colors();
    assert_eq!(fg, Color::WHITE);
    assert_ne!(bg, Color::WHITE);
}

// =============================================================================
