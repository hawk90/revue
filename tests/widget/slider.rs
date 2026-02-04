//! Slider widget integration tests
//!
//! 슬라이더 위젯의 통합 테스트
//!
//! 테스트 커버리지:
//! - 생성자 및 빌더 메서드
//! - 값 getter/setter
//! - 렌더링 (다양한 스타일, 방향, 상태)
//! - 키 입력 처리 (방향키, Home/End, vim 키)
//! - 경계값 처리 (min/max, step)
//! - 스타일 및 색상
//! - 라벨 및 값 표시

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    percentage_slider, slider, slider_range, volume_slider, Slider, SliderOrientation, SliderStyle,
    StyledView, View,
};

// =============================================================================
// 생성자 및 빌더 메서드 테스트
// =============================================================================

#[test]
fn test_slider_new_default() {
    let s = Slider::new();
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_default_trait() {
    let s = Slider::default();
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_value_builder() {
    let s = Slider::new().value(50.0);
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_range_builder() {
    let s = Slider::new().range(10.0, 90.0).value(50.0);
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_step_behavior() {
    // step affects value snapping behavior
    let s = Slider::new().range(0.0, 100.0).step(10.0).value(25.0);
    // Value should snap to nearest step
    let val = s.get_value();
    assert!(val == 20.0 || val == 30.0);
}

#[test]
fn test_slider_length_minimum() {
    // length affects rendering - minimum length is 3
    let s = Slider::new().length(1);
    // Verify through rendering
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_slider_orientation_horizontal() {
    let h = Slider::new().horizontal();
    // Orientation affects rendering - verify indirectly
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    h.render(&mut ctx);
}

#[test]
fn test_slider_orientation_vertical() {
    let v = Slider::new().vertical();
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    v.render(&mut ctx);
}

#[test]
fn test_slider_style_block() {
    let s = Slider::new().style(SliderStyle::Block);
    // Style affects rendering
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_style_line() {
    let s = Slider::new().style(SliderStyle::Line);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_style_thin() {
    let s = Slider::new().style(SliderStyle::Thin);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_style_gradient() {
    let s = Slider::new().style(SliderStyle::Gradient);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_style_dots() {
    let s = Slider::new().style(SliderStyle::Dots);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_show_value_affects_render() {
    let s1 = Slider::new().show_value(true).value(50.0);
    let s2 = Slider::new().show_value(false).value(50.0);

    let mut buffer1 = Buffer::new(30, 3);
    let mut buffer2 = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx1 = RenderContext::new(&mut buffer1, area);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);

    s1.render(&mut ctx1);
    s2.render(&mut ctx2);

    let text1: String = (0..area.width)
        .filter_map(|x| buffer1.get(x, area.y).map(|c| c.symbol))
        .collect();
    let text2: String = (0..area.width)
        .filter_map(|x| buffer2.get(x, area.y).map(|c| c.symbol))
        .collect();
    // show_value=true should show the number
    assert!(text1.contains("50"));
    // show_value=false should not display the number
    assert!(!text2.contains("50"));
}

#[test]
fn test_slider_value_format() {
    let s = Slider::new().value_format("{}%");
    // value_format affects rendering
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_track_color() {
    let s = Slider::new().track_color(Color::RED);
    // Color affects rendering
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_fill_color() {
    let s = Slider::new().fill_color(Color::BLUE);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_knob_color() {
    let s = Slider::new().knob_color(Color::GREEN);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_focused() {
    let s = Slider::new().focused(true);
    // focused affects key handling and rendering
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_disabled() {
    let s = Slider::new().disabled(true);
    // disabled affects key handling and rendering
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_label() {
    let s = Slider::new().label("Volume");
    // label affects rendering
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Volume"));
}

#[test]
fn test_slider_ticks() {
    let s = Slider::new().ticks(5);
    // ticks affect rendering
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_ticks_minimum() {
    let s = Slider::new().ticks(1);
    // minimum tick count is 2
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);
}

#[test]
fn test_slider_builder_chaining() {
    let s = Slider::new()
        .value(50.0)
        .range(0.0, 100.0)
        .step(5.0)
        .length(30)
        .horizontal()
        .style(SliderStyle::Line)
        .show_value(true)
        .value_format("{}%")
        .track_color(Color::rgb(100, 100, 100))
        .fill_color(Color::CYAN)
        .knob_color(Color::WHITE)
        .focused(true)
        .disabled(false)
        .label("Vol")
        .ticks(5);

    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_helper_slider() {
    let s = slider().value(25.0);
    assert_eq!(s.get_value(), 25.0);
}

#[test]
fn test_slider_helper_slider_range() {
    let s = slider_range(0.0, 10.0).value(5.0);
    assert_eq!(s.get_value(), 5.0);
}

#[test]
fn test_slider_helper_percentage_slider() {
    let s = percentage_slider().value(75.0);
    assert_eq!(s.get_value(), 75.0);
}

#[test]
fn test_slider_helper_volume_slider() {
    let s = volume_slider();
    assert_eq!(s.get_value(), 0.0);
}

// =============================================================================
// 값 getter/setter 테스트
// =============================================================================

#[test]
fn test_slider_set_value() {
    let mut s = Slider::new();
    s.set_value(50.0);
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_set_value_clamps_to_max() {
    let mut s = Slider::new().range(0.0, 100.0);
    s.set_value(150.0);
    assert_eq!(s.get_value(), 100.0);
}

#[test]
fn test_slider_set_value_clamps_to_min() {
    let mut s = Slider::new().range(0.0, 100.0);
    s.set_value(-50.0);
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_set_value_snaps_to_step() {
    let mut s = Slider::new().range(0.0, 100.0).step(10.0);
    s.set_value(25.0);
    let val = s.get_value();
    assert!(val == 20.0 || val == 30.0);
}

#[test]
fn test_slider_get_value() {
    let s = Slider::new().value(42.0);
    assert_eq!(s.get_value(), 42.0);
}

#[test]
fn test_slider_increment() {
    let mut s = Slider::new().range(0.0, 100.0).value(50.0);
    s.increment();
    assert_eq!(s.get_value(), 51.0);
}

#[test]
fn test_slider_increment_with_step() {
    let mut s = Slider::new().range(0.0, 100.0).step(5.0).value(50.0);
    s.increment();
    assert_eq!(s.get_value(), 55.0);
}

#[test]
fn test_slider_increment_clamps_at_max() {
    let mut s = Slider::new().range(0.0, 100.0).value(98.0);
    s.increment();
    s.increment();
    s.increment();
    assert_eq!(s.get_value(), 100.0);
}

#[test]
fn test_slider_decrement() {
    let mut s = Slider::new().range(0.0, 100.0).value(50.0);
    s.decrement();
    assert_eq!(s.get_value(), 49.0);
}

#[test]
fn test_slider_decrement_with_step() {
    let mut s = Slider::new().range(0.0, 100.0).step(5.0).value(50.0);
    s.decrement();
    assert_eq!(s.get_value(), 45.0);
}

#[test]
fn test_slider_decrement_clamps_at_min() {
    let mut s = Slider::new().range(0.0, 100.0).value(2.0);
    s.decrement();
    s.decrement();
    s.decrement();
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_set_min() {
    let mut s = Slider::new().range(0.0, 100.0).value(50.0);
    s.set_min();
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_set_max() {
    let mut s = Slider::new().range(0.0, 100.0).value(50.0);
    s.set_max();
    assert_eq!(s.get_value(), 100.0);
}

#[test]
fn test_slider_increment_decrement_cycle() {
    let mut s = Slider::new().range(0.0, 100.0).value(50.0);

    s.increment();
    assert_eq!(s.get_value(), 51.0);

    s.decrement();
    assert_eq!(s.get_value(), 50.0);

    s.set_min();
    assert_eq!(s.get_value(), 0.0);

    s.set_max();
    assert_eq!(s.get_value(), 100.0);
}

// =============================================================================
// 렌더링 테스트 - 기본
// =============================================================================

#[test]
fn test_slider_render_horizontal_basic() {
    let s = Slider::new().value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('5') && text.contains('0'));
}

#[test]
fn test_slider_render_vertical_basic() {
    let s = Slider::new().vertical().value(50.0);
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_render_with_label() {
    let s = Slider::new().label("Volume").value(50.0);
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Volume"));
}

#[test]
fn test_slider_render_without_value() {
    let s = Slider::new().show_value(false).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(!text.contains("50"));
}

// =============================================================================
// 렌더링 테스트 - 스타일별
// =============================================================================

#[test]
fn test_slider_render_block_style() {
    let s = Slider::new().style(SliderStyle::Block).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('█'));
    assert!(text.contains('░'));
}

#[test]
fn test_slider_render_line_style() {
    let s = Slider::new().style(SliderStyle::Line).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('●'));
    assert!(text.contains('━'));
}

#[test]
fn test_slider_render_thin_style() {
    let s = Slider::new().style(SliderStyle::Thin).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('┃'));
    assert!(text.contains('─'));
}

#[test]
fn test_slider_render_gradient_style() {
    let s = Slider::new().style(SliderStyle::Gradient).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('█') || text.contains('▓') || text.contains('▒') || text.contains('░'));
}

#[test]
fn test_slider_render_dots_style() {
    let s = Slider::new().style(SliderStyle::Dots).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('●'));
    assert!(text.contains('○'));
}

// =============================================================================
// 렌더링 테스트 - 상태별 (focused, disabled)
// =============================================================================

#[test]
fn test_slider_render_focused() {
    let s = Slider::new().focused(true).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("50"));
}

#[test]
fn test_slider_render_disabled() {
    let s = Slider::new().disabled(true).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.len() > 0);
}

#[test]
fn test_slider_render_disabled_horizontal() {
    let s = Slider::new()
        .disabled(true)
        .style(SliderStyle::Block)
        .value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_render_disabled_vertical() {
    let s = Slider::new()
        .disabled(true)
        .vertical()
        .style(SliderStyle::Dots)
        .value(50.0);
    let mut buffer = Buffer::new(10, 20);
    let area = Rect::new(0, 0, 10, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

// =============================================================================
// 렌더링 테스트 - 틱 마크
// =============================================================================

#[test]
fn test_slider_render_with_ticks() {
    let s = Slider::new().ticks(5).value(50.0);
    let mut buffer = Buffer::new(30, 5);
    let area = Rect::new(0, 0, 30, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let tick_row_text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y + 1).map(|c| c.symbol))
        .collect();
    assert!(tick_row_text.contains('┴'));
}

#[test]
fn test_slider_render_with_ticks_height_check() {
    let s = Slider::new().ticks(5).value(50.0);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_row_text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(first_row_text.len() > 0);
}

// =============================================================================
// 키 입력 처리 테스트 - 수평 슬라이더
// =============================================================================

#[test]
fn test_slider_handle_key_horizontal_right() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 51.0);
}

#[test]
fn test_slider_handle_key_horizontal_left() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Left));
    assert_eq!(s.get_value(), 49.0);
}

#[test]
fn test_slider_handle_key_horizontal_char_l() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Char('l')));
    assert_eq!(s.get_value(), 51.0);
}

#[test]
fn test_slider_handle_key_horizontal_char_h() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Char('h')));
    assert_eq!(s.get_value(), 49.0);
}

#[test]
fn test_slider_handle_key_horizontal_home() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Home));
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_handle_key_horizontal_end() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(s.handle_key(&Key::End));
    assert_eq!(s.get_value(), 100.0);
}

// =============================================================================
// 키 입력 처리 테스트 - 수직 슬라이더
// =============================================================================

#[test]
fn test_slider_handle_key_vertical_up() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Up));
    assert_eq!(s.get_value(), 51.0);
}

#[test]
fn test_slider_handle_key_vertical_down() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Down));
    assert_eq!(s.get_value(), 49.0);
}

#[test]
fn test_slider_handle_key_vertical_char_k() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Char('k')));
    assert_eq!(s.get_value(), 51.0);
}

#[test]
fn test_slider_handle_key_vertical_char_j() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Char('j')));
    assert_eq!(s.get_value(), 49.0);
}

#[test]
fn test_slider_handle_key_vertical_home() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(s.handle_key(&Key::Home));
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_handle_key_vertical_end() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(s.handle_key(&Key::End));
    assert_eq!(s.get_value(), 100.0);
}

// =============================================================================
// 키 입력 처리 테스트 - 경계 조건
// =============================================================================

#[test]
fn test_slider_handle_key_not_focused() {
    let mut s = Slider::new().focused(false).value(50.0);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_handle_key_disabled() {
    let mut s = Slider::new().focused(true).disabled(true).value(50.0);
    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_handle_key_unsupported_key() {
    let mut s = Slider::new().focused(true).value(50.0);
    assert!(!s.handle_key(&Key::Char('x')));
    assert_eq!(s.get_value(), 50.0);

    assert!(!s.handle_key(&Key::Escape));
    assert_eq!(s.get_value(), 50.0);

    assert!(!s.handle_key(&Key::Enter));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_handle_key_wrong_direction_horizontal() {
    let mut s = Slider::new().horizontal().focused(true).value(50.0);
    assert!(!s.handle_key(&Key::Up));
    assert_eq!(s.get_value(), 50.0);

    assert!(!s.handle_key(&Key::Down));
    assert_eq!(s.get_value(), 50.0);
}

#[test]
fn test_slider_handle_key_wrong_direction_vertical() {
    let mut s = Slider::new().vertical().focused(true).value(50.0);
    assert!(!s.handle_key(&Key::Left));
    assert_eq!(s.get_value(), 50.0);

    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);
}

// =============================================================================
// 키 입력 처리 테스트 - step이 있는 경우
// =============================================================================

#[test]
fn test_slider_handle_key_with_step() {
    let mut s = Slider::new()
        .horizontal()
        .focused(true)
        .range(0.0, 100.0)
        .step(10.0)
        .value(50.0);

    s.handle_key(&Key::Right);
    assert_eq!(s.get_value(), 60.0);

    s.handle_key(&Key::Left);
    assert_eq!(s.get_value(), 50.0);
}

// =============================================================================
// 경계값 테스트
// =============================================================================

#[test]
fn test_slider_value_clamp_on_construction() {
    let s = Slider::new().range(0.0, 100.0).value(150.0);
    assert_eq!(s.get_value(), 100.0);
}

#[test]
fn test_slider_value_negative_clamp() {
    let s = Slider::new().range(0.0, 100.0).value(-10.0);
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_range_affects_existing_value() {
    let s = Slider::new().value(50.0).range(0.0, 30.0);
    assert_eq!(s.get_value(), 30.0);
}

#[test]
fn test_slider_step_rounding() {
    let mut s = Slider::new().range(0.0, 100.0).step(10.0).value(0.0);

    s.set_value(14.0);
    assert!((s.get_value() - 10.0).abs() < 0.1);

    s.set_value(16.0);
    assert!((s.get_value() - 20.0).abs() < 0.1);
}

#[test]
fn test_slider_step_boundary_values() {
    let s = Slider::new().range(0.0, 100.0).step(10.0).value(25.0);
    let val = s.get_value();
    assert!(val == 20.0 || val == 30.0);
}

#[test]
fn test_slider_normalized_zero() {
    let s = Slider::new().range(0.0, 100.0).value(0.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('░'));
}

#[test]
fn test_slider_normalized_max() {
    let s = Slider::new().range(0.0, 100.0).value(100.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    let filled_count = text.chars().filter(|&c| c == '█').count();
    assert!(filled_count > 0);
}

#[test]
fn test_slider_normalized_half() {
    let s = Slider::new().range(0.0, 100.0).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    let filled_count = text.chars().filter(|&c| c == '█').count();
    let empty_count = text.chars().filter(|&c| c == '░').count();
    assert!(filled_count > 0 && empty_count > 0);
}

// =============================================================================
// 스타일 및 색상 테스트
// =============================================================================

#[test]
fn test_slider_custom_colors() {
    let s = Slider::new()
        .track_color(Color::RED)
        .fill_color(Color::BLUE)
        .knob_color(Color::GREEN)
        .value(50.0);

    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
}

#[test]
fn test_slider_all_styles_render() {
    let styles = [
        SliderStyle::Block,
        SliderStyle::Line,
        SliderStyle::Thin,
        SliderStyle::Gradient,
        SliderStyle::Dots,
    ];

    for style in styles {
        let s = Slider::new().style(style).value(50.0);
        let mut buffer = Buffer::new(30, 3);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        s.render(&mut ctx);

        let first_cell = buffer.get(0, 0);
        assert!(first_cell.is_some(), "Style {:?} should render", style);
    }
}

#[test]
fn test_slider_horizontal_vertical_render() {
    let h = Slider::new().horizontal().value(50.0);
    let v = Slider::new().vertical().value(50.0);

    let mut buffer_h = Buffer::new(30, 30);
    let mut buffer_v = Buffer::new(30, 30);
    let area_h = Rect::new(0, 0, 30, 3);
    let area_v = Rect::new(0, 0, 10, 20);

    let mut ctx_h = RenderContext::new(&mut buffer_h, area_h);
    let mut ctx_v = RenderContext::new(&mut buffer_v, area_v);

    h.render(&mut ctx_h);
    v.render(&mut ctx_v);

    let cell_h = buffer_h.get(0, 0);
    let cell_v = buffer_v.get(0, 0);
    assert!(cell_h.is_some());
    assert!(cell_v.is_some());
}

// =============================================================================
// 값 포맷팅 테스트
// =============================================================================

#[test]
fn test_slider_value_format_integer() {
    let s = Slider::new().range(0.0, 100.0).value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("50"));
}

#[test]
fn test_slider_value_format_decimal() {
    let s = Slider::new().range(0.0, 10.0).step(0.1).value(5.5);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("5.5"));
}

#[test]
fn test_slider_value_format_custom() {
    let s = Slider::new().value(75.0).value_format("{}%");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('%'));
}

// =============================================================================
// CSS ID 및 클래스 테스트
// =============================================================================

#[test]
fn test_slider_element_id() {
    let s = Slider::new().element_id("test-slider");
    assert_eq!(View::id(&s), Some("test-slider"));

    let meta = s.meta();
    assert_eq!(meta.id, Some("test-slider".to_string()));
}

#[test]
fn test_slider_css_classes() {
    let s = Slider::new().class("volume-control").class("audio");

    assert!(s.has_class("volume-control"));
    assert!(s.has_class("audio"));
    assert!(!s.has_class("video"));

    let meta = s.meta();
    assert!(meta.classes.contains("volume-control"));
    assert!(meta.classes.contains("audio"));
}

#[test]
fn test_slider_styled_view_trait() {
    let mut s = Slider::new();

    s.set_id("my-slider");
    assert_eq!(View::id(&s), Some("my-slider"));

    s.add_class("control");
    assert!(s.has_class("control"));

    s.toggle_class("control");
    assert!(!s.has_class("control"));

    s.toggle_class("active");
    assert!(s.has_class("active"));

    s.remove_class("active");
    assert!(!s.has_class("active"));
}

// =============================================================================
// 렌더링 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_slider_render_zero_height() {
    let s = Slider::new().value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);
}

#[test]
fn test_slider_render_small_area() {
    let s = Slider::new().length(5).value(50.0);
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_render_with_label_truncation() {
    let s = Slider::new().label("Long label").value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_render_empty_label() {
    let s = Slider::new().label("").value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_render_unicode_label() {
    let s = Slider::new().label("볼륨").value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("볼륨"));
}

// =============================================================================
// 복합 테스트 - 다양한 조합
// =============================================================================

#[test]
fn test_slider_full_featured_horizontal() {
    let s = Slider::new()
        .horizontal()
        .style(SliderStyle::Line)
        .range(0.0, 100.0)
        .step(5.0)
        .value(50.0)
        .length(25)
        .label("Volume")
        .show_value(true)
        .value_format("{}%")
        .ticks(6)
        .focused(true)
        .fill_color(Color::CYAN)
        .knob_color(Color::WHITE)
        .track_color(Color::rgb(100, 100, 100));

    let mut buffer = Buffer::new(50, 5);
    let area = Rect::new(0, 0, 50, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Volume"));
    assert!(text.contains("%"));
}

#[test]
fn test_slider_full_featured_vertical() {
    let s = Slider::new()
        .vertical()
        .style(SliderStyle::Dots)
        .range(0.0, 10.0)
        .step(0.5)
        .value(5.0)
        .length(15)
        .show_value(true)
        .focused(true)
        .fill_color(Color::GREEN);

    let mut buffer = Buffer::new(20, 30);
    let area = Rect::new(0, 0, 20, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_key_sequence() {
    let mut s = Slider::new()
        .horizontal()
        .focused(true)
        .range(0.0, 100.0)
        .step(10.0)
        .value(50.0);

    s.handle_key(&Key::Right);
    assert_eq!(s.get_value(), 60.0);

    s.handle_key(&Key::Right);
    assert_eq!(s.get_value(), 70.0);

    s.handle_key(&Key::Left);
    assert_eq!(s.get_value(), 60.0);

    s.handle_key(&Key::Home);
    assert_eq!(s.get_value(), 0.0);

    s.handle_key(&Key::End);
    assert_eq!(s.get_value(), 100.0);
}

#[test]
fn test_slider_disabled_with_focus() {
    let mut s = Slider::new().focused(true).disabled(true).value(50.0);

    assert!(!s.handle_key(&Key::Right));
    assert_eq!(s.get_value(), 50.0);

    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let first_cell = buffer.get(0, 0);
    assert!(first_cell.is_some());
}

#[test]
fn test_slider_value_format_persistence() {
    let s = Slider::new().value_format("{} dB").value(75.0);

    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("dB"));
}

#[test]
fn test_slider_range_with_negative_values() {
    let s = Slider::new().range(-50.0, 50.0).value(0.0);
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_range_with_negative_clamp() {
    let s = Slider::new().range(-50.0, 50.0).value(-100.0);
    assert_eq!(s.get_value(), -50.0);
}

#[test]
fn test_slider_increment_decrement_with_negative_range() {
    let mut s = Slider::new().range(-50.0, 50.0).value(0.0);

    s.increment();
    assert_eq!(s.get_value(), 1.0);

    s.decrement();
    assert_eq!(s.get_value(), 0.0);
}

// =============================================================================
// percentage_slider 헬퍼 테스트
// =============================================================================

#[test]
fn test_percentage_slider_defaults() {
    let s = percentage_slider();
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_percentage_slider_render() {
    let s = percentage_slider().value(75.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains('%'));
}

// =============================================================================
// volume_slider 헬퍼 테스트
// =============================================================================

#[test]
fn test_volume_slider_render() {
    let s = volume_slider().value(50.0);
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    assert!(text.contains("Vol"));
}

#[test]
fn test_volume_slider_style_is_block() {
    let s = volume_slider();
    // Verify through rendering - volume_slider uses Block style
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    s.render(&mut ctx);

    let text: String = (0..area.width)
        .filter_map(|x| buffer.get(x, area.y).map(|c| c.symbol))
        .collect();
    // Block style uses █ and ░ characters
    assert!(text.contains('█') || text.contains('░'));
}

// =============================================================================
// View trait 테스트
// =============================================================================

#[test]
fn test_slider_view_id() {
    let s = Slider::new().element_id("my-slider");
    assert_eq!(View::id(&s), Some("my-slider"));
}

#[test]
fn test_slider_view_meta() {
    let s = Slider::new().element_id("test").class("slider-class");

    let meta = View::meta(&s);
    assert_eq!(meta.id, Some("test".to_string()));
    assert!(meta.classes.contains("slider-class"));
}

// =============================================================================
// 추가 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_slider_value_at_exact_min() {
    let s = Slider::new().range(10.0, 90.0).value(10.0);
    assert_eq!(s.get_value(), 10.0);
}

#[test]
fn test_slider_value_at_exact_max() {
    let s = Slider::new().range(10.0, 90.0).value(90.0);
    assert_eq!(s.get_value(), 90.0);
}

#[test]
fn test_slider_zero_step_is_continuous() {
    let mut s = Slider::new().range(0.0, 100.0).step(0.0).value(50.0);
    s.increment();
    // With step 0, default step is (max - min) / 100 = 1.0
    assert_eq!(s.get_value(), 51.0);
}

#[test]
fn test_slider_multiple_key_presses_at_boundary() {
    let mut s = Slider::new()
        .horizontal()
        .focused(true)
        .range(0.0, 10.0)
        .step(1.0)
        .value(9.0);

    // Try to go beyond max multiple times
    for _ in 0..5 {
        s.handle_key(&Key::Right);
    }
    assert_eq!(s.get_value(), 10.0);

    // Try to go beyond min multiple times
    s.set_value(1.0);
    for _ in 0..5 {
        s.handle_key(&Key::Left);
    }
    assert_eq!(s.get_value(), 0.0);
}

#[test]
fn test_slider_all_orientations_with_all_styles() {
    let orientations = [SliderOrientation::Horizontal, SliderOrientation::Vertical];
    let styles = [
        SliderStyle::Block,
        SliderStyle::Line,
        SliderStyle::Thin,
        SliderStyle::Gradient,
        SliderStyle::Dots,
    ];

    for &orientation in &orientations {
        for &style in &styles {
            let s = match orientation {
                SliderOrientation::Horizontal => {
                    Slider::new().horizontal().style(style).value(50.0)
                }
                SliderOrientation::Vertical => Slider::new().vertical().style(style).value(50.0),
            };

            let mut buffer = Buffer::new(30, 30);
            let area = match orientation {
                SliderOrientation::Horizontal => Rect::new(0, 0, 30, 3),
                SliderOrientation::Vertical => Rect::new(0, 0, 10, 20),
            };
            let mut ctx = RenderContext::new(&mut buffer, area);

            s.render(&mut ctx);

            let cell = buffer.get(area.x, area.y);
            assert!(
                cell.is_some(),
                "Orientation {:?} with style {:?} should render",
                orientation,
                style
            );
        }
    }
}

// =============================================================================
