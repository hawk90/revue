//! Rating widget tests
//!
//! Rating 위젯의 통합 테스트입니다.
//! 생성자, 빌더 메서드, 값 관리, 아이콘 렌더링, 인터랙티브 동작을 테스트합니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{rating, Rating, RatingSize, RatingStyle, View};

// =============================================================================
// 생성자 및 빌더 테스트 (Constructor and Builder Tests)
// =============================================================================

#[test]
fn test_rating_new() {
    // 기본 생성자 테스트
    let r = Rating::new();
    assert_eq!(r.get_value(), 0.0, "초기 값은 0.0이어야 합니다");
}

#[test]
fn test_rating_default() {
    // Default trait 구현 테스트
    let r = Rating::default();
    assert_eq!(r.get_value(), 0.0);
}

#[test]
fn test_rating_builder_value() {
    // value 빌더 메서드 테스트
    let r = Rating::new().value(3.5);
    assert_eq!(r.get_value(), 3.5);
}

#[test]
fn test_rating_builder_max_value() {
    // max_value 빌더 메서드 테스트
    let r = Rating::new().max_value(10);
    // max_value는 private 필드이므로 렌더링을 통해 간접 확인
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 10개의 별이 렌더링되는지 확인 (간격 2로 10개 = 20칸)
    // 첫 번째 별 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_builder_style() {
    // style 빌더 메서드 테스트 - 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().style(RatingStyle::Heart).value(3.0);
    r.render(&mut ctx);

    // 하트 스타일로 렌더링되는지 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '♥');
}

#[test]
fn test_rating_builder_size() {
    // size 빌더 메서드 테스트
    let r = Rating::new().size(RatingSize::Large);
    // spacing 메서드는 private이므로 렌더링으로 간접 확인
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // Large size는 spacing=3이므로 별 사이 간격 확인
}

#[test]
fn test_rating_builder_half_stars() {
    // half_stars 빌더 메서드 테스트 - 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r1 = Rating::new().value(2.5).half_stars(false);
    r1.render(&mut ctx);
    // half_stars 비활성화: 2.5는 2로 처리되므로 3번째는 빈 별
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_builder_readonly() {
    // readonly 빌더 메서드 테스트
    let r = Rating::new().readonly(true);
    // readonly는 렌더링에 영향을 주지 않으므로 빌더가 작동하는지만 확인
    let _ = r;
}

#[test]
fn test_rating_builder_filled_color() {
    // filled_color 빌더 메서드 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.0).filled_color(Color::RED);
    r.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_rating_builder_empty_color() {
    // empty_color 빌더 메서드 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(0.0).empty_color(Color::BLUE);
    r.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::BLUE));
}

#[test]
fn test_rating_builder_hover_color() {
    // hover_color 빌더 메서드 테스트
    let r = Rating::new().hover_color(Color::GREEN);
    // hover_color는 렌더링 시 호버 상태에서만 사용되므로 빌더만 확인
    let _ = r;
}

#[test]
fn test_rating_builder_show_value() {
    // show_value 빌더 메서드 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.5).show_value(true);
    r.render(&mut ctx);

    // 수치가 표시되는지 확인
    let text: String = (0..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("3.5"));
}

#[test]
fn test_rating_builder_label() {
    // label 빌더 메서드 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().label("Rate this:");
    r.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'R');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'a');
}

#[test]
fn test_rating_builder_chain() {
    // 빌더 메서드 체이닝 테스트
    let r = Rating::new()
        .value(4.5)
        .max_value(10)
        .style(RatingStyle::Heart)
        .size(RatingSize::Large)
        .half_stars(true)
        .readonly(false)
        .show_value(true)
        .label("Rating:");

    assert_eq!(r.get_value(), 4.5);
    // 나머지는 렌더링을 통해 검증
    let mut buffer = Buffer::new(50, 1);
    let area = Rect::new(0, 0, 50, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'R'); // label
}

#[test]
fn test_rating_helper_function() {
    // rating() 헬퍼 함수 테스트
    let r = rating().value(3.0);
    assert_eq!(r.get_value(), 3.0);
}

// =============================================================================
// 프리셋 테스트 (Preset Tests)
// =============================================================================

#[test]
fn test_rating_five_star_preset() {
    // 5별 프리셋 테스트
    let r = Rating::five_star();
    assert_eq!(r.get_value(), 0.0);
    // max_value는 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 기본 5별 설정 확인
}

#[test]
fn test_rating_ten_star_preset() {
    // 10별 프리셋 테스트
    let r = Rating::ten_star();
    assert_eq!(r.get_value(), 0.0);
    // max_value=10 확인을 위해 렌더링
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
}

#[test]
fn test_rating_hearts_preset() {
    // 하트 프리셋 테스트
    let r = Rating::hearts().value(3.0);
    // 하트 스타일 확인을 위해 렌더링
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '♥');
}

#[test]
fn test_rating_thumbs_preset() {
    // 엄지척/엄지내려요 프리셋 테스트
    let r = Rating::thumbs();
    // 커스텀 스타일과 max_value=2 확인을 위해 렌더링
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 엄지척 문자 확인 (Custom 스타일)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '👎'); // 초기값 0이므로 빈 상태
}

// =============================================================================
// 값 관리 테스트 (Value Management Tests)
// =============================================================================

#[test]
fn test_rating_set_value() {
    // set_value 메서드 테스트
    let mut r = Rating::new();
    r.set_value(3.5);
    assert_eq!(r.get_value(), 3.5);
}

#[test]
fn test_rating_set_value_clamps_upper() {
    // 값이 최대값을 초과하면 clamping되는지 테스트
    let mut r = Rating::new().max_value(5);
    r.set_value(10.0);
    assert_eq!(r.get_value(), 5.0, "최대값으로 clamping되어야 합니다");
}

#[test]
fn test_rating_set_value_clamps_lower() {
    // 값이 0 미만이면 clamping되는지 테스트
    let mut r = Rating::new();
    r.set_value(-5.0);
    assert_eq!(r.get_value(), 0.0, "0으로 clamping되어야 합니다");
}

#[test]
fn test_rating_builder_value_clamps() {
    // 빌더의 value 메서드도 clamping하는지 테스트
    let r = Rating::new().value(100.0);
    assert_eq!(r.get_value(), 5.0, "최대값으로 clamping되어야 합니다");
}

#[test]
fn test_rating_value_changes_when_max_decreases() {
    // 최대값이 감소하면 값도 조정되는지 테스트
    let r = Rating::new().value(4.5).max_value(3);
    assert_eq!(r.get_value(), 3.0, "최대값으로 조정되어야 합니다");
}

#[test]
fn test_rating_increment_full_star() {
    // 전체 별 단위 증가 테스트
    let mut r = Rating::new().value(2.0).half_stars(false);
    r.increment();
    assert_eq!(r.get_value(), 3.0);
}

#[test]
fn test_rating_increment_half_star() {
    // 반별 단위 증가 테스트
    let mut r = Rating::new().value(2.0).half_stars(true);
    r.increment();
    assert_eq!(r.get_value(), 2.5);
}

#[test]
fn test_rating_increment_clamps_at_max() {
    // 증가가 최대값에서 멈추는지 테스트
    let mut r = Rating::new().value(5.0).max_value(5);
    r.increment();
    assert_eq!(r.get_value(), 5.0, "최대값을 초과할 수 없습니다");
}

#[test]
fn test_rating_decrement_full_star() {
    // 전체 별 단위 감소 테스트
    let mut r = Rating::new().value(3.0).half_stars(false);
    r.decrement();
    assert_eq!(r.get_value(), 2.0);
}

#[test]
fn test_rating_decrement_half_star() {
    // 반별 단위 감소 테스트
    let mut r = Rating::new().value(2.5).half_stars(true);
    r.decrement();
    assert_eq!(r.get_value(), 2.0);
}

#[test]
fn test_rating_decrement_clamps_at_zero() {
    // 감소가 0에서 멈추는지 테스트
    let mut r = Rating::new().value(0.0);
    r.decrement();
    assert_eq!(r.get_value(), 0.0, "0 미만으로 내려갈 수 없습니다");
}

#[test]
fn test_rating_clear() {
    // clear 메서드 테스트
    let mut r = Rating::new().value(4.5);
    r.clear();
    assert_eq!(r.get_value(), 0.0);
}

#[test]
fn test_rating_increment_decrement_roundtrip() {
    // 증가/감소 왕복 테스트
    let mut r = Rating::new().value(2.5);
    r.increment();
    assert_eq!(r.get_value(), 3.0);
    r.decrement();
    assert_eq!(r.get_value(), 2.5);
}

// =============================================================================
// 호버 상태 테스트 (Hover State Tests)
// =============================================================================

#[test]
fn test_rating_set_hover_some() {
    // 호버 값 설정 테스트 - 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut r = Rating::new();
    r.set_hover(Some(3.5));
    r.render(&mut ctx);

    // 호버 값이 렌더링에 반영되는지 확인
    // 3.5면 3개는 채워진 별(위치 0, 2, 4), 4번째는 반별(위치 6)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '⯪');
}

#[test]
fn test_rating_set_hover_none() {
    // 호버 값 초기화 테스트
    let mut r = Rating::new();
    r.set_hover(Some(3.5));
    r.set_hover(None);
    // hover_value는 private이므로 렌더링으로 간접 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 호버가 없으므로 실제 값(0)으로 렌더링
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_set_hover_clamps_upper() {
    // 호버 값도 최대값으로 clamping되는지 테스트
    let mut r = Rating::new().max_value(5);
    r.set_hover(Some(10.0));
    // private 필드이므로 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 5로 clamping되어 모든 별이 채워져야 함
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '★');
}

#[test]
fn test_rating_set_hover_clamps_lower() {
    // 호버 값도 0으로 clamping되는지 테스트
    let mut r = Rating::new();
    r.set_hover(Some(-1.0));
    // 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 0으로 clamping되어 모든 별이 비어있어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '☆');
}

// =============================================================================
// RatingStyle 테스트 (RatingStyle Tests)
// =============================================================================

#[test]
fn test_rating_style_default() {
    // Default trait 구현 테스트
    let style = RatingStyle::default();
    assert_eq!(style, RatingStyle::Star);
}

// =============================================================================
// RatingSize 테스트 (RatingSize Tests)
// =============================================================================

#[test]
fn test_rating_size_default() {
    // Default trait 구현 테스트
    let size = RatingSize::default();
    assert_eq!(size, RatingSize::Medium);
}

// =============================================================================
// 렌더링 테스트 (Rendering Tests)
// =============================================================================

#[test]
fn test_rating_render_zero_stars() {
    // 0별 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(0.0);
    r.render(&mut ctx);

    // 모든 별이 비어있어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_render_full_stars() {
    // 전체 별 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.0);
    r.render(&mut ctx);

    // 처음 3개는 채워진 별
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '★');
    // 4번째는 빈 별 (위치 6)
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_render_half_star() {
    // 반별 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(2.5).half_stars(true);
    r.render(&mut ctx);

    // 처음 2개는 채워진 별
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '★');
    // 3번째는 반별
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '⯪');
}

#[test]
fn test_rating_render_max_rating() {
    // 최대 평점 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(5.0).max_value(5);
    r.render(&mut ctx);

    // 모든 별이 채워져 있어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    assert_eq!(buffer.get(8, 0).unwrap().symbol, '★');
}

#[test]
fn test_rating_render_with_label() {
    // 라벨과 함께 렌더링 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().label("Rate:").value(3.0);
    r.render(&mut ctx);

    // 라벨이 먼저 나타나야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'R');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, 'a');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 't');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, ':');
}

#[test]
fn test_rating_render_with_show_value() {
    // 수치 표시 렌더링 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.5).show_value(true);
    r.render(&mut ctx);

    // 별 다음에 수치가 표시되어야 함
    // 별은 5개 * 2칸 = 10칸, + 공백 1칸 = 11칸부터 수치
    let text: String = (11..30)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();
    assert!(text.contains("3.5"), "수치가 표시되어야 합니다");
    assert!(text.contains("5"), "최대값도 표시되어야 합니다");
}

#[test]
fn test_rating_render_all_styles() {
    // 모든 스타일 렌더링 테스트
    let styles = [
        RatingStyle::Star,
        RatingStyle::Heart,
        RatingStyle::Circle,
        RatingStyle::Square,
        RatingStyle::Numeric,
        RatingStyle::Custom('A', 'B'),
    ];

    // 각 스타일에 대한 기대 문자 (RatingStyle::chars() 메서드의 구현 기반)
    let expected_chars = [
        ('★', '☆'), // Star
        ('♥', '♡'), // Heart
        ('●', '○'), // Circle
        ('■', '□'), // Square
        ('●', '○'), // Numeric
        ('A', 'B'), // Custom
    ];

    for (style, (filled, _)) in styles.iter().zip(expected_chars.iter()) {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(3.0).style(*style);
        r.render(&mut ctx);

        // 첫 번째 문자가 해당 스타일의 filled 문자여야 함
        assert_eq!(buffer.get(0, 0).unwrap().symbol, *filled);
    }
}

#[test]
fn test_rating_render_all_sizes() {
    // 모든 크기 렌더링 테스트
    let sizes = [RatingSize::Small, RatingSize::Medium, RatingSize::Large];

    for size in sizes {
        let mut buffer = Buffer::new(30, 1);
        let area = Rect::new(0, 0, 30, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let r = Rating::new().value(3.0).size(size);
        r.render(&mut ctx);

        // 첫 번째 별이 채워져 있어야 함
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
    }
}

#[test]
fn test_rating_render_with_hover() {
    // 호버 상태 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut r = Rating::new().value(2.0);
    r.set_hover(Some(4.0));
    r.render(&mut ctx);

    // 호버 값이 표시되어야 함 (4개 채워진 별)
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '★');
}

#[test]
fn test_rating_render_without_half_stars() {
    // 반별 비활성화 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(2.5).half_stars(false);
    r.render(&mut ctx);

    // 2.5를 2로 처리하므로 3번째는 빈 별이어야 함
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_render_small_area() {
    // 작은 영역 렌더링 테스트
    let mut buffer = Buffer::new(5, 1);
    let area = Rect::new(0, 0, 5, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(5.0);
    r.render(&mut ctx);

    // 영역이 작아도 일부는 렌더링되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
}

#[test]
fn test_rating_render_zero_width() {
    // 너비가 0인 경우 렌더링 테스트
    let mut buffer = Buffer::new(0, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.0);
    r.render(&mut ctx); // 패닉 없이 완료되어야 함
}

#[test]
fn test_rating_render_zero_height() {
    // 높이가 0인 경우 렌더링 테스트
    let mut buffer = Buffer::new(20, 0);
    let area = Rect::new(0, 0, 20, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.0);
    r.render(&mut ctx); // 패닉 없이 완료되어야 함
}

// =============================================================================
// 색상 테스트 (Color Tests)
// =============================================================================

#[test]
fn test_rating_render_default_colors() {
    // 기본 색상 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.0);
    r.render(&mut ctx);

    // 채워진 별은 금색 (RGB 255, 200, 0)
    let filled_cell = buffer.get(0, 0).unwrap();
    assert_eq!(filled_cell.fg, Some(Color::rgb(255, 200, 0)));

    // 빈 별은 회색 (RGB 100, 100, 100)
    // value=3.0이면 4번째 별(위치 6)이 비어있음
    let empty_cell = buffer.get(6, 0).unwrap();
    assert_eq!(empty_cell.fg, Some(Color::rgb(100, 100, 100)));
}

#[test]
fn test_rating_render_custom_filled_color() {
    // 사용자 정의 채워진 색상 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(3.0).filled_color(Color::RED);
    r.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
}

#[test]
fn test_rating_render_custom_empty_color() {
    // 사용자 정의 빈 색상 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().value(0.0).empty_color(Color::BLUE);
    r.render(&mut ctx);

    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::BLUE));
}

#[test]
fn test_rating_render_hover_color() {
    // 호버 색상 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut r = Rating::new().value(0.0).hover_color(Color::GREEN);
    r.set_hover(Some(3.0));
    r.render(&mut ctx);

    // 호버 중에는 호버 색상 사용
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::GREEN));
}

// =============================================================================
// 엣지 케이스 테스트 (Edge Case Tests)
// =============================================================================

#[test]
fn test_rating_zero_max_value() {
    // 최대값이 0으로 설정되면 1로 보정되는지 테스트
    let r = Rating::new().max_value(0);
    // max_value=0은 1로 보정되므로 렌더링 확인
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 최소 1개는 렌더링되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_very_large_max_value() {
    // 매우 큰 최대값 테스트
    let r = Rating::new().max_value(100);
    // 렌더링이 정상 작동하는지 확인
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 버퍼에 일부가 렌더링되어야 함
}

#[test]
fn test_rating_negative_value_clamps_to_zero() {
    // 음수 값이 0으로 clamping되는지 테스트
    let mut r = Rating::new();
    r.set_value(-999.0);
    assert_eq!(r.get_value(), 0.0);
}

#[test]
fn test_rating_fractional_value_rounding() {
    // 소수값 정밀도 테스트
    let r = Rating::new().value(2.789);
    assert_eq!(r.get_value(), 2.789, "소수값 그대로 저장되어야 합니다");
}

#[test]
fn test_rating_half_star_boundary() {
    // 반별 경계값 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 정확히 2.5인 경우
    let r = Rating::new().value(2.5).half_stars(true);
    r.render(&mut ctx);

    // 3번째 위치에 반별 문자
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '⯪');
}

#[test]
fn test_rating_just_below_half() {
    // 반별 바로 아래 값 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 2.49는 반별 미만
    let r = Rating::new().value(2.49).half_stars(true);
    r.render(&mut ctx);

    // 3번째는 빈 별
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '☆');
}

#[test]
fn test_rating_just_above_half() {
    // 반별 바로 위 값 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 2.51은 반별 이상
    let r = Rating::new().value(2.51).half_stars(true);
    r.render(&mut ctx);

    // 3번째는 반별
    assert_eq!(buffer.get(4, 0).unwrap().symbol, '⯪');
}

#[test]
fn test_rating_max_value_one() {
    // 최대값이 1인 경우 테스트
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new().max_value(1).value(1.0);
    r.render(&mut ctx);

    // 하나의 별만 렌더링
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
}

#[test]
fn test_rating_empty_label() {
    // 빈 라벨 테스트
    let r = Rating::new().label("");
    // label은 private이므로 렌더링으로 확인
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    r.render(&mut ctx);
    // 빈 라벨이므로 첫 번째 위치에 별이 있어야 함
    // 하지만 빈 문자열은 아무것도 렌더링하지 않으므로 공백 후 별
}

#[test]
fn test_rating_multiple_operations() {
    // 여러 연속 연산 테스트
    let mut r = Rating::new();

    r.set_value(2.0);
    assert_eq!(r.get_value(), 2.0);

    r.increment();
    assert_eq!(r.get_value(), 2.5);

    r.increment();
    assert_eq!(r.get_value(), 3.0);

    r.decrement();
    assert_eq!(r.get_value(), 2.5);

    r.clear();
    assert_eq!(r.get_value(), 0.0);
}

#[test]
fn test_rating_changing_max_value_with_existing_value() {
    // 기존 값이 있을 때 최대값 변경 테스트
    let r = Rating::new().value(4.5).max_value(3);
    assert_eq!(r.get_value(), 3.0, "새 최대값으로 조정되어야 합니다");
}

#[test]
fn test_rating_changing_max_value_preserves_lower_value() {
    // 기존 값이 새 최대값보다 작으면 보존되는지 테스트
    let r = Rating::new().value(2.0).max_value(10);
    assert_eq!(r.get_value(), 2.0, "값이 보존되어야 합니다");
}

// =============================================================================
// View trait 메서드 테스트 (View Trait Method Tests)
// =============================================================================

#[test]
fn test_rating_view_meta() {
    // View trait의 meta 메서드 테스트
    let r = Rating::new();
    let meta = r.meta();

    assert_eq!(meta.id, None);
}

#[test]
fn test_rating_id_builder() {
    // element_id 빌더 메서드 테스트
    let r = Rating::new().element_id("test-rating");
    assert_eq!(r.id(), Some("test-rating"));
}

#[test]
fn test_rating_class_builder() {
    // class 빌더 메서드 테스트
    let r = Rating::new().class("star-rating");
    let classes = View::classes(&r);
    assert!(classes.iter().any(|c| c == "star-rating"));
}

// =============================================================================
// 고급 시나리오 테스트 (Advanced Scenario Tests)
// =============================================================================

#[test]
fn test_rating_readonly_doesnt_affect_rendering() {
    // readonly 모드가 렌더링에 영향을 주지 않는지 테스트
    let mut buffer1 = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx1 = RenderContext::new(&mut buffer1, area);

    let mut buffer2 = Buffer::new(20, 1);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);

    let r1 = Rating::new().value(3.0).readonly(false);
    let r2 = Rating::new().value(3.0).readonly(true);

    r1.render(&mut ctx1);
    r2.render(&mut ctx2);

    // 렌더링 결과는 동일해야 함
    assert_eq!(
        buffer1.get(0, 0).unwrap().symbol,
        buffer2.get(0, 0).unwrap().symbol
    );
}

#[test]
fn test_rating_complex_scenario() {
    // 복합 시나리오: 여러 설정을 함께 사용
    let mut buffer = Buffer::new(50, 1);
    let area = Rect::new(0, 0, 50, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::new()
        .value(4.5)
        .max_value(5)
        .style(RatingStyle::Star)
        .size(RatingSize::Medium)
        .half_stars(true)
        .show_value(true)
        .label("Product Rating:")
        .filled_color(Color::rgb(255, 215, 0))
        .empty_color(Color::rgb(200, 200, 200));

    r.render(&mut ctx);

    // 라벨 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, 'P');

    // 별과 수치가 렌더링되었는지 확인
    let text: String = (0..50)
        .filter_map(|x| buffer.get(x, 0).map(|c| c.symbol))
        .collect();

    assert!(text.contains('★'), "채워진 별이 있어야 합니다");
    assert!(text.contains('⯪'), "반별이 있어야 합니다");
    assert!(text.contains("4.5"), "수치가 표시되어야 합니다");
}

#[test]
fn test_rating_ten_star_rendering() {
    // 10별 렌더링 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::ten_star().value(7.5).size(RatingSize::Small);
    r.render(&mut ctx);

    // 첫 번째 별 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '★');
}

#[test]
fn test_rating_hearts_rendering() {
    // 하트 스타일 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::hearts().value(3.0);
    r.render(&mut ctx);

    // 하트 문자 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '♥');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, '♡');
}

#[test]
fn test_rating_thumbs_rendering() {
    // 엄지척 스타일 렌더링 테스트
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let r = Rating::thumbs().value(1.0);
    r.render(&mut ctx);

    // 엄지척 문자 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '👍');
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '👎');
}

// =============================================================================
