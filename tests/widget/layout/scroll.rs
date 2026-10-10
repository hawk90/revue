//! ScrollView widget integration tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{scroll_view, ScrollView};

// =============================================================================
// 생성자 및 빌더 테스트 (Constructor and Builder Tests)
// =============================================================================

#[test]
fn test_scroll_view_new() {
    let sv = ScrollView::new();
    assert_eq!(sv.offset(), 0);
    // Note: can't directly test private fields, only through behavior
}

#[test]
fn test_scroll_view_default() {
    let sv = ScrollView::default();
    assert_eq!(sv.offset(), 0);
    // Note: show_scrollbar is private, tested through rendering behavior
}

#[test]
fn test_scroll_view_helper() {
    let sv = scroll_view();
    assert_eq!(sv.offset(), 0);
    // Note: show_scrollbar is private, tested through rendering behavior
}

#[test]
fn test_scroll_view_builder_content_height() {
    let sv = ScrollView::new().content_height(100);
    // content_height is private, test through is_scrollable behavior
    assert!(sv.is_scrollable(50));
}

#[test]
fn test_scroll_view_builder_scroll_offset() {
    let sv = ScrollView::new().scroll_offset(50);
    assert_eq!(sv.offset(), 50);
}

#[test]
fn test_scroll_view_builder_show_scrollbar() {
    let sv = ScrollView::new().content_height(100).show_scrollbar(false);
    // Test through rendering behavior - should not have scrollbar
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    sv.render(&mut ctx);
    // Should not have scrollbar at right edge
    let scrollbar_x = area.x + area.width - 1;
    assert_ne!(buffer.get(scrollbar_x, area.y).unwrap().symbol, '│');
}

#[test]
fn test_scroll_view_builder_scrollbar_style() {
    let sv = ScrollView::new()
        .content_height(100)
        .scrollbar_style(Color::RED, Color::BLUE);
    // Test through rendering behavior - scrollbar should have correct colors
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    sv.render(&mut ctx);
    // At top position, the thumb (RED) is rendered over the track (BLUE)
    // So we should see the thumb color
    let scrollbar_x = area.x + area.width - 1;
    let cell = buffer.get(scrollbar_x, area.y).unwrap();
    assert_eq!(cell.fg, Some(Color::RED)); // Thumb color (fg)
}

#[test]
fn test_scroll_view_builder_chain() {
    let sv = ScrollView::new()
        .content_height(200)
        .scroll_offset(10)
        .show_scrollbar(false)
        .scrollbar_style(Color::GREEN, Color::BLACK);

    assert_eq!(sv.offset(), 10);
    // Test private fields through behavior
    assert!(sv.is_scrollable(100));
}

// =============================================================================
// 스크롤 위치 테스트 (Scroll Position Tests)
// =============================================================================

#[test]
fn test_scroll_down() {
    let mut sv = ScrollView::new().content_height(100);

    sv.scroll_down(5, 20);
    assert_eq!(sv.offset(), 5);

    sv.scroll_down(10, 20);
    assert_eq!(sv.offset(), 15);
}

#[test]
fn test_scroll_up() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(20, 20);

    sv.scroll_up(5);
    assert_eq!(sv.offset(), 15);

    sv.scroll_up(10);
    assert_eq!(sv.offset(), 5);
}

#[test]
fn test_scroll_up_clamps_to_zero() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(10, 20);

    sv.scroll_up(10);
    assert_eq!(sv.offset(), 0);

    sv.scroll_up(10);
    assert_eq!(sv.offset(), 0); // Should not go below 0
}

#[test]
fn test_scroll_down_clamps_to_max() {
    let mut sv = ScrollView::new().content_height(50);

    // Viewport 20, content 50, max offset = 30
    sv.scroll_down(100, 20);
    assert_eq!(sv.offset(), 30);
}

#[test]
fn test_set_offset() {
    let mut sv = ScrollView::new().content_height(100);

    sv.set_offset(25, 20);
    assert_eq!(sv.offset(), 25);
}

#[test]
fn test_set_offset_clamps() {
    let mut sv = ScrollView::new().content_height(50);

    // Viewport 20, max offset = 30
    sv.set_offset(50, 20);
    assert_eq!(sv.offset(), 30); // Clamped to max
}

#[test]
fn test_scroll_to_top() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(50, 20);

    sv.scroll_to_top();
    assert_eq!(sv.offset(), 0);
}

#[test]
fn test_scroll_to_bottom() {
    let mut sv = ScrollView::new().content_height(100);

    sv.scroll_to_bottom(20);
    assert_eq!(sv.offset(), 80); // 100 - 20
}

#[test]
fn test_page_down() {
    let mut sv = ScrollView::new().content_height(100);

    sv.page_down(20);
    assert_eq!(sv.offset(), 19); // viewport - 1

    sv.page_down(20);
    assert_eq!(sv.offset(), 38);
}

#[test]
fn test_page_up() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(50, 20);

    sv.page_up(20);
    assert_eq!(sv.offset(), 31); // 50 - 19

    sv.page_up(20);
    assert_eq!(sv.offset(), 12);
}

#[test]
fn test_page_up_clamps_to_zero() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(10, 20);

    sv.page_up(20);
    assert_eq!(sv.offset(), 0);
}

#[test]
fn test_page_down_clamps_to_max() {
    let mut sv = ScrollView::new().content_height(50);
    sv.scroll_to_bottom(20);

    let before = sv.offset();
    sv.page_down(20);
    assert_eq!(sv.offset(), before);
}

// =============================================================================
// 키보드 입력 처리 테스트 (Keyboard Input Tests)
// =============================================================================

#[test]
fn test_handle_key_down() {
    let mut sv = ScrollView::new().content_height(100);

    let changed = sv.handle_key(&Key::Down, 20);
    assert!(changed);
    assert_eq!(sv.offset(), 1);
}

#[test]
fn test_handle_key_up() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(10, 20);

    let changed = sv.handle_key(&Key::Up, 20);
    assert!(changed);
    assert_eq!(sv.offset(), 9);
}

#[test]
fn test_handle_key_char_k() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(10, 20);

    let changed = sv.handle_key(&Key::Char('k'), 20);
    assert!(changed);
    assert_eq!(sv.offset(), 9);
}

#[test]
fn test_handle_key_char_j() {
    let mut sv = ScrollView::new().content_height(100);

    let changed = sv.handle_key(&Key::Char('j'), 20);
    assert!(changed);
    assert_eq!(sv.offset(), 1);
}

#[test]
fn test_handle_key_page_up() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(50, 20);

    let changed = sv.handle_key(&Key::PageUp, 20);
    assert!(changed);
    assert_eq!(sv.offset(), 31); // 50 - 19
}

#[test]
fn test_handle_key_page_down() {
    let mut sv = ScrollView::new().content_height(100);

    let changed = sv.handle_key(&Key::PageDown, 20);
    assert!(changed);
    assert_eq!(sv.offset(), 19);
}

#[test]
fn test_handle_key_home() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_down(50, 20);

    let changed = sv.handle_key(&Key::Home, 20);
    assert!(changed);
    assert_eq!(sv.offset(), 0);
}

#[test]
fn test_handle_key_end() {
    let mut sv = ScrollView::new().content_height(100);

    let changed = sv.handle_key(&Key::End, 20);
    assert!(changed);
    assert_eq!(sv.offset(), 80);
}

#[test]
fn test_handle_key_no_change_at_top() {
    let mut sv = ScrollView::new().content_height(100);

    let changed = sv.handle_key(&Key::Up, 20);
    assert!(!changed);
    assert_eq!(sv.offset(), 0);
}

#[test]
fn test_handle_key_no_change_at_bottom() {
    let mut sv = ScrollView::new().content_height(50);
    sv.scroll_to_bottom(20);

    let changed = sv.handle_key(&Key::Down, 20);
    assert!(!changed);
    assert_eq!(sv.offset(), 30);
}

#[test]
fn test_handle_key_ignored() {
    let mut sv = ScrollView::new().content_height(100);
    let before = sv.offset();

    let changed = sv.handle_key(&Key::Char('x'), 20);
    assert!(!changed);
    assert_eq!(sv.offset(), before);

    let changed = sv.handle_key(&Key::Enter, 20);
    assert!(!changed);
    assert_eq!(sv.offset(), before);
}

// =============================================================================
// 상태 쿼리 테스트 (State Query Tests)
// =============================================================================

#[test]
fn test_is_scrollable_true() {
    let sv = ScrollView::new().content_height(50);
    assert!(sv.is_scrollable(20)); // 50 > 20
}

#[test]
fn test_is_scrollable_false_equal() {
    let sv = ScrollView::new().content_height(50);
    assert!(!sv.is_scrollable(50)); // 50 == 50
}

#[test]
fn test_is_scrollable_false_less() {
    let sv = ScrollView::new().content_height(50);
    assert!(!sv.is_scrollable(100)); // 50 < 100
}

#[test]
fn test_scroll_percentage_at_top() {
    let sv = ScrollView::new().content_height(100);
    assert_eq!(sv.scroll_percentage(20), 0.0);
}

#[test]
fn test_scroll_percentage_at_bottom() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_to_bottom(20);
    assert_eq!(sv.scroll_percentage(20), 1.0);
}

#[test]
fn test_scroll_percentage_middle() {
    let mut sv = ScrollView::new().content_height(100);
    sv.set_offset(40, 20);
    assert!((sv.scroll_percentage(20) - 0.5).abs() < 0.01);
}

#[test]
fn test_scroll_percentage_zero_max_offset() {
    let sv = ScrollView::new().content_height(20);
    assert_eq!(sv.scroll_percentage(20), 0.0);
}

// =============================================================================
// 렌더링 테스트 (Rendering Tests)
// =============================================================================

#[test]
fn test_scroll_view_render_basic() {
    let sv = ScrollView::new().content_height(100);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // At top position, the scrollbar thumb is rendered
    let scrollbar_x = area.x + area.width - 1;
    assert_eq!(buffer.get(scrollbar_x, area.y).unwrap().symbol, '█');
}

#[test]
fn test_scroll_view_render_no_scrollbar_when_disabled() {
    let sv = ScrollView::new().content_height(100).show_scrollbar(false);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // Should not render scrollbar
    let scrollbar_x = area.x + area.width - 1;
    assert_ne!(buffer.get(scrollbar_x, area.y).unwrap().symbol, '│');
}

#[test]
fn test_scroll_view_render_no_scrollbar_when_not_scrollable() {
    let sv = ScrollView::new().content_height(10);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // Should not render scrollbar when content fits
    let scrollbar_x = area.x + area.width - 1;
    assert_ne!(buffer.get(scrollbar_x, area.y).unwrap().symbol, '│');
}

#[test]
fn test_scroll_view_render_scrollbar_colors() {
    let sv = ScrollView::new()
        .content_height(100)
        .scrollbar_style(Color::RED, Color::BLUE);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // Check that scrollbar thumb has correct color
    let scrollbar_x = area.x + area.width - 1;
    let cell = buffer.get(scrollbar_x, area.y).unwrap();
    assert_eq!(cell.fg, Some(Color::RED)); // Thumb color
}

#[test]
fn test_render_content() {
    let sv = ScrollView::new().content_height(50).scroll_offset(10);

    let mut content_buffer = Buffer::new(15, 50);
    // Fill content with some test data
    for y in 0..50 {
        for x in 0..15 {
            let ch = char::from_digit((y % 10) as u32, 10).unwrap();
            content_buffer.set(x, y, revue::render::Cell::new(ch));
        }
    }

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render_content(&mut ctx, &content_buffer);

    // Should render content starting from offset 10
    // Line 10 % 10 = 0
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '0'); // Content from line 10 (10 % 10 = 0)

    // Should have scrollbar (either track '│' or thumb '█')
    let scrollbar_x = area.x + area.width - 1;
    let symbol = buffer.get(scrollbar_x, 0).unwrap().symbol;
    assert!(symbol == '│' || symbol == '█');
}

#[test]
fn test_render_content_with_offset() {
    let sv = ScrollView::new().content_height(100).scroll_offset(50);

    let mut content_buffer = Buffer::new(15, 100);
    for y in 0..100 {
        let ch = char::from_digit((y % 10) as u32, 10).unwrap();
        content_buffer.set(0, y, revue::render::Cell::new(ch));
    }

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render_content(&mut ctx, &content_buffer);

    // Should show content starting from offset 50
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.symbol, '0'); // Line 50 % 10 = 0
}

// =============================================================================
// 콘텐츠 영역 테스트 (Content Area Tests)
// =============================================================================

#[test]
fn test_content_area_with_scrollbar() {
    let sv = ScrollView::new().content_height(100);
    let area = Rect::new(0, 0, 80, 24);

    let content_area = sv.content_area(area);
    assert_eq!(content_area.x, 0);
    assert_eq!(content_area.y, 0);
    assert_eq!(content_area.width, 79); // -1 for scrollbar
    assert_eq!(content_area.height, 24);
}

#[test]
fn test_content_area_without_scrollbar() {
    let sv = ScrollView::new().content_height(10);
    let area = Rect::new(0, 0, 80, 24);

    let content_area = sv.content_area(area);
    assert_eq!(content_area.width, 80); // Full width, no scrollbar
}

#[test]
fn test_content_area_scrollbar_disabled() {
    let sv = ScrollView::new().content_height(100).show_scrollbar(false);
    let area = Rect::new(0, 0, 80, 24);

    let content_area = sv.content_area(area);
    assert_eq!(content_area.width, 80); // Full width when disabled
}

// =============================================================================
// 버퍼 생성 테스트 (Buffer Creation Tests)
// =============================================================================

#[test]
fn test_create_content_buffer() {
    let sv = ScrollView::new().content_height(50);
    let buffer = sv.create_content_buffer(20);

    assert_eq!(buffer.width(), 20);
    assert_eq!(buffer.height(), 50);
}

// =============================================================================
// 엣지 케이스 테스트 (Edge Cases)
// =============================================================================

#[test]
fn test_scroll_with_zero_content_height() {
    let mut sv = ScrollView::new().content_height(0);

    sv.scroll_down(10, 20);
    assert_eq!(sv.offset(), 0);

    sv.scroll_up(10);
    assert_eq!(sv.offset(), 0);
}

#[test]
fn test_scroll_with_zero_viewport() {
    let mut sv = ScrollView::new().content_height(100);

    // With zero viewport, max_offset = content_height - 0 = 100
    sv.scroll_down(10, 0);
    // scroll_down with viewport 0: max_offset = 100 - 0 = 100, so can scroll to 10
    assert_eq!(sv.offset(), 10);

    sv.scroll_to_bottom(0);
    // max_offset = 100 - 0 = 100
    assert_eq!(sv.offset(), 100);
}

#[test]
fn test_scroll_with_large_offset() {
    let mut sv = ScrollView::new().content_height(100);
    sv.set_offset(u16::MAX, 20);

    let max_offset = 100u16.saturating_sub(20);
    assert_eq!(sv.offset(), max_offset);
}

#[test]
fn test_scroll_percentage_with_zero_viewport() {
    let sv = ScrollView::new().content_height(100);
    let percentage = sv.scroll_percentage(0);
    assert_eq!(percentage, 0.0);
}

#[test]
fn test_render_with_zero_area() {
    let sv = ScrollView::new().content_height(100);
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_render_with_zero_width() {
    let sv = ScrollView::new().content_height(100);
    let mut buffer = Buffer::new(0, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_render_with_zero_height() {
    let sv = ScrollView::new().content_height(100);
    let mut buffer = Buffer::new(20, 0);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);
    // Should not panic
}

#[test]
fn test_content_width_greater_than_viewport() {
    let sv = ScrollView::new().content_height(100);
    let area = Rect::new(0, 0, 5, 10);

    let content_area = sv.content_area(area);
    assert_eq!(content_area.width, 4); // -1 for scrollbar
}

#[test]
fn test_multiple_scroll_operations() {
    let mut sv = ScrollView::new().content_height(100);

    // Simulate user scrolling down multiple times
    for _ in 0..10 {
        sv.handle_key(&Key::Down, 20);
    }
    assert_eq!(sv.offset(), 10);

    // Scroll up
    for _ in 0..5 {
        sv.handle_key(&Key::Up, 20);
    }
    assert_eq!(sv.offset(), 5);
}

#[test]
fn test_page_navigation_multiple_times() {
    let mut sv = ScrollView::new().content_height(100);

    // Page down multiple times
    sv.page_down(20);
    assert_eq!(sv.offset(), 19);

    sv.page_down(20);
    assert_eq!(sv.offset(), 38);

    sv.page_down(20);
    assert_eq!(sv.offset(), 57);

    // Page up
    sv.page_up(20);
    assert_eq!(sv.offset(), 38);
}

#[test]
fn test_scrollbar_position_at_top() {
    let sv = ScrollView::new().content_height(100);
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // Scrollbar thumb should be at top
    let scrollbar_x = area.x + area.width - 1;
    assert_eq!(buffer.get(scrollbar_x, 0).unwrap().symbol, '█');
}

#[test]
fn test_scrollbar_position_at_bottom() {
    let mut sv = ScrollView::new().content_height(100);
    sv.scroll_to_bottom(20);

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // Scrollbar thumb should be at bottom
    let scrollbar_x = area.x + area.width - 1;
    assert_eq!(buffer.get(scrollbar_x, 19).unwrap().symbol, '█');
}

#[test]
fn test_scrollbar_position_middle() {
    let mut sv = ScrollView::new().content_height(100);
    sv.set_offset(40, 20);

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    sv.render(&mut ctx);

    // Scrollbar thumb should be somewhere in the middle
    let scrollbar_x = area.x + area.width - 1;
    let thumb_found = (0..20).any(|y| buffer.get(scrollbar_x, y).unwrap().symbol == '█');
    assert!(thumb_found);
}

// =============================================================================
// View Trait 테스트 (View Trait Tests)
// =============================================================================

#[test]
fn test_scroll_view_widget_type() {
    let sv = ScrollView::new();
    assert_eq!(sv.widget_type(), "ScrollView");
}

#[test]
fn test_scroll_view_id_none() {
    let sv = ScrollView::new();
    assert!(View::id(&sv).is_none());
}

#[test]
fn test_scroll_view_id_some() {
    let sv = ScrollView::new().element_id("my-scroll");
    assert_eq!(View::id(&sv), Some("my-scroll"));
}

#[test]
fn test_scroll_view_classes_empty() {
    let sv = ScrollView::new();
    assert!(View::classes(&sv).is_empty());
}

#[test]
fn test_scroll_view_classes_with_values() {
    let sv = ScrollView::new().class("scrollable").class("vertical");
    let classes = View::classes(&sv);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"scrollable".to_string()));
    assert!(classes.contains(&"vertical".to_string()));
}

#[test]
fn test_scroll_view_meta() {
    let sv = ScrollView::new().element_id("test-scroll").class("scroll");
    let meta = sv.meta();
    assert_eq!(meta.widget_type, "ScrollView");
    assert_eq!(meta.id, Some("test-scroll".to_string()));
    assert!(meta.classes.contains("scroll"));
}

// =============================================================================
// StyledView Trait 테스트 (StyledView Trait Tests)
// =============================================================================

#[test]
fn test_scroll_view_styled_set_id() {
    let mut sv = ScrollView::new();
    StyledView::set_id(&mut sv, "test-id");
    assert_eq!(View::id(&sv), Some("test-id"));
}

#[test]
fn test_scroll_view_styled_add_class() {
    let mut sv = ScrollView::new();
    StyledView::add_class(&mut sv, "first");
    StyledView::add_class(&mut sv, "second");
    assert!(StyledView::has_class(&sv, "first"));
    assert!(StyledView::has_class(&sv, "second"));
    assert_eq!(View::classes(&sv).len(), 2);
}

#[test]
fn test_scroll_view_styled_remove_class() {
    let mut sv = ScrollView::new().class("a").class("b").class("c");
    StyledView::remove_class(&mut sv, "b");
    assert!(StyledView::has_class(&sv, "a"));
    assert!(!StyledView::has_class(&sv, "b"));
    assert!(StyledView::has_class(&sv, "c"));
}

#[test]
fn test_scroll_view_styled_toggle_class() {
    let mut sv = ScrollView::new();
    StyledView::toggle_class(&mut sv, "test");
    assert!(StyledView::has_class(&sv, "test"));
    StyledView::toggle_class(&mut sv, "test");
    assert!(!StyledView::has_class(&sv, "test"));
}

#[test]
fn test_scroll_view_builder_element_id() {
    let sv = ScrollView::new().element_id("my-scroll-view");
    assert_eq!(View::id(&sv), Some("my-scroll-view"));
}

#[test]
fn test_scroll_view_builder_class() {
    let sv = ScrollView::new().class("scrollable").class("content");
    assert!(sv.has_class("scrollable"));
    assert!(sv.has_class("content"));
}

#[test]
fn test_scroll_view_builder_classes() {
    let sv = ScrollView::new().classes(vec!["first", "second", "third"]);
    assert!(sv.has_class("first"));
    assert!(sv.has_class("second"));
    assert!(sv.has_class("third"));
}

// =============================================================================
// 복잡한 시나리오 테스트 (Complex Scenario Tests)
// =============================================================================

#[test]
fn test_full_scroll_cycle() {
    let mut sv = ScrollView::new().content_height(100);

    // Start at top
    assert_eq!(sv.offset(), 0);

    // Scroll to bottom
    sv.scroll_to_bottom(20);
    assert_eq!(sv.offset(), 80);

    // Scroll back to top
    sv.scroll_to_top();
    assert_eq!(sv.offset(), 0);

    // Scroll by pages
    sv.page_down(20);
    assert_eq!(sv.offset(), 19);

    sv.page_down(20);
    assert_eq!(sv.offset(), 38);

    sv.page_up(20);
    assert_eq!(sv.offset(), 19);
}

#[test]
fn test_keyboard_navigation_full_range() {
    let mut sv = ScrollView::new().content_height(100);

    // Down to bottom
    for _ in 0..80 {
        sv.handle_key(&Key::Down, 20);
    }
    assert_eq!(sv.offset(), 80);

    // Up back to top
    for _ in 0..80 {
        sv.handle_key(&Key::Up, 20);
    }
    assert_eq!(sv.offset(), 0);

    // Home/End
    sv.handle_key(&Key::End, 20);
    assert_eq!(sv.offset(), 80);

    sv.handle_key(&Key::Home, 20);
    assert_eq!(sv.offset(), 0);
}

#[test]
fn test_vim_keys_navigation() {
    let mut sv = ScrollView::new().content_height(100);

    // Use j/k for scrolling
    for _ in 0..10 {
        sv.handle_key(&Key::Char('j'), 20);
    }
    assert_eq!(sv.offset(), 10);

    for _ in 0..5 {
        sv.handle_key(&Key::Char('k'), 20);
    }
    assert_eq!(sv.offset(), 5);
}

#[test]
fn test_scroll_percentage_tracking() {
    let mut sv = ScrollView::new().content_height(100);

    assert_eq!(sv.scroll_percentage(20), 0.0);

    sv.scroll_down(25, 20);
    assert!((sv.scroll_percentage(20) - 0.3125).abs() < 0.01);

    sv.scroll_down(25, 20);
    assert!((sv.scroll_percentage(20) - 0.625).abs() < 0.01);

    sv.scroll_to_bottom(20);
    assert_eq!(sv.scroll_percentage(20), 1.0);
}
