//! Skeleton widget integration tests
//!
//! 스켈레톤 위젯의 통합 테스트입니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView};
use revue::widget::{skeleton, skeleton_avatar, skeleton_paragraph, skeleton_text, Skeleton, SkeletonShape, View};

// =============================================================================
// Constructor and Builder Tests
// 생성자 및 빌더 테스트
// =============================================================================

#[test]
fn test_skeleton_new() {
    let s = Skeleton::new();
    // Verify through rendering behavior
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Default rectangle shape should render
    assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_default_trait() {
    let s = Skeleton::default();
    // Default should create valid skeleton
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_width() {
    let s = Skeleton::new().width(5);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Only first 5 cells should be filled
    assert_ne!(buffer.get(4, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_height() {
    let s = Skeleton::new().width(10).height(3);
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // First 3 rows should be filled
    assert_ne!(buffer.get(0, 2).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 3).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_width_height_chain() {
    let s = Skeleton::new().width(5).height(2);
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // 5x2 area should be filled
    assert_ne!(buffer.get(4, 1).unwrap().symbol, ' ');
    assert_eq!(buffer.get(5, 1).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_shape() {
    let s = Skeleton::new().shape(SkeletonShape::Circle);
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Circle should render box drawing chars
    assert!(matches!(
        buffer.get(0, 0).unwrap().symbol,
        '╭' | '●' | '╰' | '╮' | '╯'
    ));
}

#[test]
fn test_skeleton_builder_shape_rectangle() {
    let s = Skeleton::new().rectangle();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Rectangle renders skeleton chars
    assert!(matches!(buffer.get(0, 0).unwrap().symbol, '░' | '▒' | '▓'));
}

#[test]
fn test_skeleton_builder_shape_circle() {
    let s = Skeleton::new().circle().height(2);
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // 2x2 circle
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '╮');
}

#[test]
fn test_skeleton_builder_shape_paragraph() {
    let s = Skeleton::new().paragraph().lines(2);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should have 2 lines
    assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_ne!(buffer.get(0, 1).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_lines() {
    let s = Skeleton::new().paragraph().lines(4);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should have 4 lines
    assert_ne!(buffer.get(0, 3).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_no_animate() {
    let s = Skeleton::new().no_animate();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // No animate always uses '░'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
}

#[test]
fn test_skeleton_builder_color() {
    let s = Skeleton::new().width(5).color(Color::RED);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(5, 0).unwrap().fg, None);
}

#[test]
fn test_skeleton_builder_frame() {
    // Frame affects which animation char is used
    let s = Skeleton::new().width(5).frame(2);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Frame 2 should use '▓'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▓');
}

#[test]
fn test_skeleton_helper_function() {
    let s = skeleton();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should render with animation
    let ch = buffer.get(0, 0).unwrap().symbol;
    assert!(matches!(ch, '░' | '▒' | '▓'));
}

#[test]
fn test_skeleton_helper_text() {
    let s = skeleton_text();
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should fill entire width (height 1)
    for x in 0..20 {
        assert_ne!(buffer.get(x, 0).unwrap().symbol, ' ');
    }
}

#[test]
fn test_skeleton_helper_avatar() {
    let s = skeleton_avatar();
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Avatar is circle with height 3
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
}

#[test]
fn test_skeleton_helper_paragraph() {
    let s = skeleton_paragraph();
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Default 3 lines
    assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_ne!(buffer.get(0, 1).unwrap().symbol, ' ');
    assert_ne!(buffer.get(0, 2).unwrap().symbol, ' ');
    assert_eq!(buffer.get(0, 3).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_builder_element_id() {
    let s = Skeleton::new().element_id("my-skeleton");
    assert_eq!(View::id(&s), Some("my-skeleton"));
}

#[test]
fn test_skeleton_builder_class() {
    let s = Skeleton::new().class("loading").class("pulse");
    assert!(s.has_class("loading"));
    assert!(s.has_class("pulse"));
}

#[test]
fn test_skeleton_builder_classes() {
    let s = Skeleton::new().classes(vec!["loading", "skeleton", "fade"]);
    assert!(s.has_class("loading"));
    assert!(s.has_class("skeleton"));
    assert!(s.has_class("fade"));
}

#[test]
fn test_skeleton_builder_classes_no_duplicates() {
    let s = Skeleton::new().class("test").classes(vec!["test", "other"]);
    let classes = View::classes(&s);
    assert!(classes.contains(&"test".to_string()));
    assert!(classes.contains(&"other".to_string()));
}

// =============================================================================
// SkeletonShape Tests
// 스켈레톤 모양 테스트
// =============================================================================

#[test]
fn test_skeleton_shape_rectangle_default() {
    let s = Skeleton::new();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Default is rectangle
    assert!(matches!(buffer.get(0, 0).unwrap().symbol, '░' | '▒' | '▓'));
}

#[test]
fn test_skeleton_shape_circle() {
    let s = skeleton().circle().height(2);
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
}

#[test]
fn test_skeleton_shape_paragraph() {
    let s = skeleton().paragraph().lines(2);
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Paragraph renders multiple lines
    assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_ne!(buffer.get(0, 1).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_shape_partial_eq() {
    assert_eq!(SkeletonShape::Rectangle, SkeletonShape::Rectangle);
    assert_eq!(SkeletonShape::Circle, SkeletonShape::Circle);
    assert_eq!(SkeletonShape::Paragraph, SkeletonShape::Paragraph);
}

#[test]
fn test_skeleton_shape_not_equal() {
    assert_ne!(SkeletonShape::Rectangle, SkeletonShape::Circle);
    assert_ne!(SkeletonShape::Circle, SkeletonShape::Paragraph);
    assert_ne!(SkeletonShape::Paragraph, SkeletonShape::Rectangle);
}

// =============================================================================
// Animation Tests
// 애니메이션 테스트
// =============================================================================

#[test]
fn test_skeleton_animate_default_enabled() {
    // By default animation is enabled, so char can vary
    let s = Skeleton::new();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    let ch = buffer.get(0, 0).unwrap().symbol;
    // Frame 0 by default -> '░'
    assert_eq!(ch, '░');
}

#[test]
fn test_skeleton_animate_disabled() {
    let s = Skeleton::new().no_animate();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // No animate always uses '░'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
}

#[test]
fn test_skeleton_skeleton_char_frame_0() {
    let s = Skeleton::new().frame(0);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
}

#[test]
fn test_skeleton_skeleton_char_frame_1() {
    let s = Skeleton::new().frame(1);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▒');
}

#[test]
fn test_skeleton_skeleton_char_frame_2() {
    let s = Skeleton::new().frame(2);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▓');
}

#[test]
fn test_skeleton_skeleton_char_frame_3() {
    let s = Skeleton::new().frame(3);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Frame 3 also uses '▒' (animation cycle: ░ ▒ ▓ ▒ ░...)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▒');
}

#[test]
fn test_skeleton_skeleton_char_frame_4() {
    // Frame 4 should cycle back to '░' (4 % 4 = 0)
    let s = Skeleton::new().frame(4);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
}

#[test]
fn test_skeleton_skeleton_char_no_animate() {
    let s = Skeleton::new().no_animate();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
}

#[test]
fn test_skeleton_animation_cycle() {
    // Animation cycle: ░ -> ▒ -> ▓ -> ▒ -> ░...
    let frames = ['░', '▒', '▓', '▒', '░', '▒', '▓'];
    for (i, expected) in frames.iter().enumerate() {
        let s = Skeleton::new().frame(i as u8);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        s.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, *expected);
    }
}

// =============================================================================
// Render Tests - Rectangle Shape
// 렌더링 테스트 - 직사각형 모양
// =============================================================================

#[test]
fn test_skeleton_render_rectangle_basic() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).height(2).no_animate();
    s.render(&mut ctx);

    // First 5 cells of first 2 rows should be filled
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '░');
    assert_eq!(buffer.get(4, 1).unwrap().symbol, '░');

    // Cells beyond width should be empty
    assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_render_rectangle_full_width() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(0).height(2).no_animate(); // width 0 = fill
    s.render(&mut ctx);

    // All cells should be filled
    for x in 0..10 {
        for y in 0..2 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, '░');
        }
    }
}

#[test]
fn test_skeleton_render_rectangle_with_color() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).height(1).color(Color::RED).no_animate();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_skeleton_render_rectangle_animated() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).height(1).frame(2);
    s.render(&mut ctx);

    // Frame 2 should render '▓'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▓');
}

#[test]
fn test_skeleton_render_rectangle_clipped_to_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Width and height larger than area
    let s = skeleton().width(20).height(10).no_animate();
    s.render(&mut ctx);

    // Should be clipped to area size
    for x in 0..10 {
        for y in 0..5 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, '░');
        }
    }
}

#[test]
fn test_skeleton_render_rectangle_with_offset() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(5, 2, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).height(2).no_animate();
    s.render(&mut ctx);

    // Skeleton at x=5, y=2 with width=5, height=2
    assert_eq!(buffer.get(5, 2).unwrap().symbol, '░');
    assert_eq!(buffer.get(9, 2).unwrap().symbol, '░');
    assert_eq!(buffer.get(5, 3).unwrap().symbol, '░');
    assert_eq!(buffer.get(9, 3).unwrap().symbol, '░');

    // Area before skeleton should be empty
    assert_eq!(buffer.get(4, 2).unwrap().symbol, ' ');
    // Area after skeleton in row should be empty
    assert_eq!(buffer.get(10, 2).unwrap().symbol, ' ');
}

// =============================================================================
// Render Tests - Circle Shape
// 렌더링 테스트 - 원형 모양
// =============================================================================

#[test]
fn test_skeleton_render_circle_size_1() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().circle().height(1).no_animate();
    s.render(&mut ctx);

    // Size 1 circle uses '●'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '●');
}

#[test]
fn test_skeleton_render_circle_size_2() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().circle().height(2).no_animate();
    s.render(&mut ctx);

    // 2x2 circle uses box drawing characters
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '╰');
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '╯');
}

#[test]
fn test_skeleton_render_circle_size_3() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().circle().height(3).no_animate();
    s.render(&mut ctx);

    // Top row
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '─');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '╮');

    // Middle row
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '│');
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '░');
    assert_eq!(buffer.get(2, 1).unwrap().symbol, '│');

    // Bottom row
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '╰');
    assert_eq!(buffer.get(1, 2).unwrap().symbol, '─');
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '╯');
}

#[test]
fn test_skeleton_render_circle_with_color() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().circle().height(2).color(Color::BLUE).no_animate();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_skeleton_render_circle_animated() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().circle().height(3).frame(2);
    s.render(&mut ctx);

    // Frame 2 should render '▓' in the middle
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '▓');
}

#[test]
fn test_skeleton_render_circle_clipped() {
    let mut buffer = Buffer::new(2, 2);
    let area = Rect::new(0, 0, 2, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Height 3 but area is only 2
    let s = skeleton().circle().height(3).no_animate();
    s.render(&mut ctx);

    // Should render 2x2 circle
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '╰');
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '╯');
}

// =============================================================================
// Render Tests - Paragraph Shape
// 렌더링 테스트 - 문단 모양
// =============================================================================

#[test]
fn test_skeleton_render_paragraph_basic() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton_paragraph().lines(3);
    s.render(&mut ctx);

    // Should have 3 lines
    assert!(buffer.get(0, 0).unwrap().symbol != ' ');
    assert!(buffer.get(0, 1).unwrap().symbol != ' ');
    assert!(buffer.get(0, 2).unwrap().symbol != ' ');

    // 4th line should be empty
    assert_eq!(buffer.get(0, 3).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_render_paragraph_full_width() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().width(0).lines(3).no_animate();
    s.render(&mut ctx);

    // First line should be full width
    for x in 0..20 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_skeleton_render_paragraph_varying_line_lengths() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().width(20).lines(3).no_animate();
    s.render(&mut ctx);

    // Line 0 (index 0, even): full width
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');

    // Line 1 (index 1, odd): width - 4 = 16
    assert_eq!(buffer.get(15, 1).unwrap().symbol, '░');
    assert_eq!(buffer.get(16, 1).unwrap().symbol, ' ');

    // Line 2 (index 2, last): width * 2/3 ≈ 13
    assert_eq!(buffer.get(12, 2).unwrap().symbol, '░');
    assert_eq!(buffer.get(13, 2).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_render_paragraph_with_custom_lines() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().lines(5).no_animate();
    s.render(&mut ctx);

    // Should have 5 lines
    for y in 0..5 {
        assert!(buffer.get(0, y).unwrap().symbol != ' ');
    }

    // 6th line should be empty
    assert_eq!(buffer.get(0, 5).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_render_paragraph_with_color() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().lines(2).color(Color::GREEN).no_animate();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_skeleton_render_paragraph_animated() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().lines(2).frame(1);
    s.render(&mut ctx);

    // Frame 1 should render '▒'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▒');
}

#[test]
fn test_skeleton_render_paragraph_clipped_to_area() {
    let mut buffer = Buffer::new(20, 2);
    let area = Rect::new(0, 0, 20, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 5 lines but area height is only 2
    let s = skeleton().paragraph().lines(5).no_animate();
    s.render(&mut ctx);

    // Should only render 2 lines
    assert!(buffer.get(0, 0).unwrap().symbol != ' ');
    assert!(buffer.get(0, 1).unwrap().symbol != ' ');
}

// =============================================================================
// Edge Cases Tests
// 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_skeleton_render_zero_width() {
    let s = skeleton().width(0).height(1).no_animate();
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Width 0 means "fill available width"
    for x in 0..10 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
    }
}

#[test]
fn test_skeleton_render_empty_buffer() {
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().no_animate();
    s.render(&mut ctx);

    // Should not panic
}

#[test]
fn test_skeleton_render_small_area() {
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(1).height(1).no_animate();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '░');
}

#[test]
fn test_skeleton_render_large_dimension_small_area() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Dimensions much larger than area
    let s = skeleton().width(100).height(100).no_animate();
    s.render(&mut ctx);

    // Should be clipped to area size
    for x in 0..5 {
        for y in 0..5 {
            assert_eq!(buffer.get(x, y).unwrap().symbol, '░');
        }
    }
}

#[test]
fn test_skeleton_frame_wrap_around() {
    // Test that animation frames wrap correctly
    for frame in 0..10 {
        let s = Skeleton::new().frame(frame);
        let mut buffer = Buffer::new(10, 1);
        let area = Rect::new(0, 0, 10, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        s.render(&mut ctx);
        // Should not panic and should produce valid chars
        let ch = buffer.get(0, 0).unwrap().symbol;
        assert!(matches!(ch, '░' | '▒' | '▓'));
    }
}

#[test]
fn test_skeleton_large_frame_value() {
    let s = Skeleton::new().frame(255);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // 255 % 4 = 3, so should be '▒'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▒');
}

// =============================================================================
// StyledView Trait Tests
// StyledView 트레이트 테스트
// =============================================================================

#[test]
fn test_skeleton_styled_view_set_id() {
    let mut s = Skeleton::new();
    StyledView::set_id(&mut s, "test-id");
    assert_eq!(View::id(&s), Some("test-id"));
}

#[test]
fn test_skeleton_styled_view_add_class() {
    let mut s = Skeleton::new();
    StyledView::add_class(&mut s, "first");
    StyledView::add_class(&mut s, "second");
    assert!(StyledView::has_class(&s, "first"));
    assert!(StyledView::has_class(&s, "second"));
    assert_eq!(View::classes(&s).len(), 2);
}

#[test]
fn test_skeleton_styled_view_add_class_no_duplicates() {
    let mut s = Skeleton::new();
    StyledView::add_class(&mut s, "test");
    StyledView::add_class(&mut s, "test");
    let classes = View::classes(&s);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_skeleton_styled_view_remove_class() {
    let mut s = Skeleton::new().class("a").class("b").class("c");
    StyledView::remove_class(&mut s, "b");
    assert!(StyledView::has_class(&s, "a"));
    assert!(!StyledView::has_class(&s, "b"));
    assert!(StyledView::has_class(&s, "c"));
}

#[test]
fn test_skeleton_styled_view_remove_nonexistent_class() {
    let mut s = Skeleton::new().class("test");
    StyledView::remove_class(&mut s, "nonexistent");
    assert!(StyledView::has_class(&s, "test"));
}

#[test]
fn test_skeleton_styled_view_toggle_class_add() {
    let mut s = Skeleton::new();
    StyledView::toggle_class(&mut s, "test");
    assert!(StyledView::has_class(&s, "test"));
}

#[test]
fn test_skeleton_styled_view_toggle_class_remove() {
    let mut s = Skeleton::new().class("test");
    StyledView::toggle_class(&mut s, "test");
    assert!(!StyledView::has_class(&s, "test"));
}

#[test]
fn test_skeleton_styled_view_has_class() {
    let s = Skeleton::new().class("present");
    assert!(StyledView::has_class(&s, "present"));
    assert!(!StyledView::has_class(&s, "absent"));
}

// =============================================================================
// View Trait Tests
// View 트레이트 테스트
// =============================================================================

#[test]
fn test_skeleton_view_widget_type() {
    let s = Skeleton::new();
    assert_eq!(s.widget_type(), "Skeleton");
}

#[test]
fn test_skeleton_view_id_none() {
    let s = Skeleton::new();
    assert!(View::id(&s).is_none());
}

#[test]
fn test_skeleton_view_id_some() {
    let s = Skeleton::new().element_id("my-id");
    assert_eq!(View::id(&s), Some("my-id"));
}

#[test]
fn test_skeleton_view_classes_empty() {
    let s = Skeleton::new();
    assert!(View::classes(&s).is_empty());
}

#[test]
fn test_skeleton_view_classes_with_values() {
    let s = Skeleton::new().class("first").class("second");
    let classes = View::classes(&s);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"first".to_string()));
    assert!(classes.contains(&"second".to_string()));
}

#[test]
fn test_skeleton_view_meta() {
    let s = Skeleton::new().element_id("test-id").class("test-class");
    let meta = s.meta();
    assert_eq!(meta.widget_type, "Skeleton");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

#[test]
fn test_skeleton_view_children_default() {
    let s = Skeleton::new();
    assert!(View::children(&s).is_empty());
}

// =============================================================================
// Color Tests
// 색상 테스트
// =============================================================================

#[test]
fn test_skeleton_custom_color() {
    let s = Skeleton::new().color(Color::CYAN);
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_skeleton_render_with_custom_color() {
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().width(5).color(Color::MAGENTA).no_animate();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::MAGENTA));
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(Color::MAGENTA));
    // Cells beyond width should not have color set
    assert_eq!(buffer.get(5, 0).unwrap().fg, None);
}

#[test]
fn test_skeleton_render_circle_with_custom_color() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().circle().height(3).color(Color::YELLOW).no_animate();
    s.render(&mut ctx);

    // All circle elements should have the custom color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::YELLOW));
    assert_eq!(buffer.get(1, 1).unwrap().fg, Some(Color::YELLOW));
    assert_eq!(buffer.get(2, 2).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_skeleton_render_paragraph_with_custom_color() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().lines(3).color(Color::WHITE).no_animate();
    s.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::WHITE));
}

// =============================================================================
// Complex Builder Chain Tests
// 복합 빌더 체인 테스트
// =============================================================================

#[test]
fn test_skeleton_complex_builder_chain() {
    let s = skeleton()
        .width(15)
        .height(3)
        .color(Color::BLUE)
        .element_id("complex-skeleton")
        .class("loading")
        .class("animated")
        .frame(2);

    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    assert_eq!(View::id(&s), Some("complex-skeleton"));
    assert!(s.has_class("loading"));
    assert!(s.has_class("animated"));
    // Frame 2 should render '▓'
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▓');
    // Check color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::BLUE));
}

#[test]
fn test_skeleton_avatar_helper_render() {
    let mut buffer = Buffer::new(5, 5);
    let area = Rect::new(0, 0, 5, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton_avatar();
    s.render(&mut ctx);

    // Avatar should be circle with height 3
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
}

#[test]
fn test_skeleton_text_helper_render() {
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton_text();
    s.render(&mut ctx);

    // Text skeleton should fill width
    for x in 0..20 {
        assert!(buffer.get(x, 0).unwrap().symbol != ' ');
    }
}

// =============================================================================
// Paragraph Edge Cases
// 문단 엣지 케이스
// =============================================================================

#[test]
fn test_skeleton_paragraph_last_line_shorter() {
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Paragraph with 3 lines, last should be 2/3 width
    let s = skeleton().paragraph().width(30).lines(3).no_animate();
    s.render(&mut ctx);

    // Last line should be shorter (30 * 2 / 3 = 20)
    assert_eq!(buffer.get(19, 2).unwrap().symbol, '░');
    assert_eq!(buffer.get(20, 2).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_paragraph_alternating_lines() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().width(20).lines(4).no_animate();
    s.render(&mut ctx);

    // Line 0 (even): full width (20)
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '░');
    // Line 1 (odd): width - 4 (16)
    assert_eq!(buffer.get(15, 1).unwrap().symbol, '░');
    assert_eq!(buffer.get(16, 1).unwrap().symbol, ' ');
    // Line 2 (even, but last-1): full width (20)
    assert_eq!(buffer.get(19, 2).unwrap().symbol, '░');
    // Line 3 (last): 2/3 width (13)
    assert_eq!(buffer.get(12, 3).unwrap().symbol, '░');
    assert_eq!(buffer.get(13, 3).unwrap().symbol, ' ');
}

#[test]
fn test_skeleton_paragraph_zero_width_fills() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = skeleton().paragraph().width(0).lines(2).no_animate();
    s.render(&mut ctx);

    // Should fill buffer width
    for x in 0..20 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, '░');
    }
}
