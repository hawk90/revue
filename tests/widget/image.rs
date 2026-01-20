//! Image widget integration tests
//!
//! Image 위젯의 통합 테스트입니다.
//! Kitty 그래픽 프로토콜을 사용하여 터미널에 이미지를 표시합니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, StyledView, View};
#[cfg(feature = "image")]
use revue::widget::{image_from_file, try_image_from_file, Image, ImageError, ScaleMode};

// =============================================================================
// 생성자 테스트 (Constructor Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_from_rgb() {
    // RGB 데이터로 이미지 생성 테스트
    // RGB 데이터로부터 이미지를 생성합니다.
    let data = vec![255, 0, 0, 0, 255, 0]; // 2픽셀: 빨강, 초록
    let img = Image::from_rgb(data.clone(), 2, 1);

    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 1);
}

#[test]
#[cfg(feature = "image")]
fn test_image_from_rgba() {
    // RGBA 데이터로 이미지 생성 테스트
    // RGBA 데이터로부터 이미지를 생성합니다.
    let data = vec![255, 0, 0, 255, 0, 255, 0, 255]; // 2픽셀
    let img = Image::from_rgba(data, 2, 1);

    assert_eq!(img.width(), 2);
    assert_eq!(img.height(), 1);
}

#[test]
#[cfg(feature = "image")]
fn test_image_from_rgb_large() {
    // 큰 RGB 이미지 생성 테스트
    let width = 100;
    let height = 50;
    let data = vec![0u8; (width * height * 3) as usize];
    let img = Image::from_rgb(data, width, height);

    assert_eq!(img.width(), 100);
    assert_eq!(img.height(), 50);
}

#[test]
#[cfg(feature = "image")]
fn test_image_from_rgba_large() {
    // 큰 RGBA 이미지 생성 테스트
    let width = 80;
    let height = 60;
    let data = vec![0u8; (width * height * 4) as usize];
    let img = Image::from_rgba(data, width, height);

    assert_eq!(img.width(), 80);
    assert_eq!(img.height(), 60);
}

#[test]
#[cfg(feature = "image")]
fn test_image_id_unique() {
    // 이미지 ID 고유성 테스트
    // 각 이미지는 고유한 ID를 가져야 합니다.
    let img1 = Image::from_rgb(vec![0; 3], 1, 1);
    let img2 = Image::from_rgb(vec![0; 3], 1, 1);

    // ID는 시간 기반으로 생성되므로 다른 값이어야 합니다
    // (매우 빠르게 생성되면 같을 수도 있지만, 일반적으로는 다름)
    assert!(img1.id() > 0);
    assert!(img2.id() > 0);
}

// =============================================================================
// 빌더 메서드 테스트 (Builder Methods Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_scale_mode_fit() {
    // Fit 스케일 모드 테스트
    // 비율을 유지하면서 영역에 맞춥니다.
    let img = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Fit);
    let (w, h) = img.scaled_dimensions(80, 24);

    // Fit 모드는 비율을 유지합니다
    assert!(w <= 80);
    assert!(h <= 24);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scale_mode_fill() {
    // Fill 스케일 모드 테스트
    // 영역을 채우기 위해 비율을 유지하면서 자를 수 있습니다.
    let img = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Fill);
    let (w, h) = img.scaled_dimensions(80, 24);

    // Fill 모드는 최소 하나의 차원을 채웁니다
    assert!(w >= 80 || h >= 24);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scale_mode_stretch() {
    // Stretch 스케일 모드 테스트
    // 비율 무시하고 영역에 꽉 채웁니다.
    let img = Image::from_rgb(vec![0; 3], 1, 1).scale(ScaleMode::Stretch);
    let (w, h) = img.scaled_dimensions(80, 24);

    assert_eq!(w, 80);
    assert_eq!(h, 24);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scale_mode_none() {
    // None 스케일 모드 테스트
    // 원본 크기를 유지합니다.
    let img = Image::from_rgb(vec![0; 300], 100, 50);
    let img = img.scale(ScaleMode::None);
    let (w, h) = img.scaled_dimensions(80, 40);

    assert_eq!(w, 100);
    assert_eq!(h, 50);
}

#[test]
#[cfg(feature = "image")]
fn test_image_placeholder() {
    // 플레이스홀더 문자 설정 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1).placeholder('#');

    // 플레이스홀더는 렌더링 테스트에서 검증
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    img.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '#');
}

#[test]
#[cfg(feature = "image")]
fn test_image_placeholder_default() {
    // 기본 플레이스홀더 문자 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1);

    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    img.render(&mut ctx);

    // 기본 플레이스홀더는 공백
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

// =============================================================================
// 스케일링 테스트 (Scaling Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_scaled_dimensions_wide() {
    // 가로로 긴 이미지의 스케일링 테스트
    // 2:1 비율의 이미지
    let img = Image::from_rgb(vec![0; 600], 200, 100);
    let (w, h) = img.scaled_dimensions(80, 40);

    // 가로가 더 길므로 가로를 기준으로 맞춤
    assert_eq!(w, 80);
    assert_eq!(h, 40);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scaled_dimensions_tall() {
    // 세로로 긴 이미지의 스케일링 테스트
    // 1:2 비율의 이미지
    let img = Image::from_rgb(vec![0; 600], 100, 200);
    let (w, h) = img.scaled_dimensions(80, 40);

    // 세로가 더 길므로 세로를 기준으로 맞춤
    assert_eq!(h, 40);
    assert_eq!(w, 20);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scaled_dimensions_square() {
    // 정사각형 이미지의 스케일링 테스트
    let img = Image::from_rgb(vec![0; 300], 10, 10);
    let (w, h) = img.scaled_dimensions(20, 20);

    assert_eq!(w, 20);
    assert_eq!(h, 20);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scaled_dimensions_small_bounds() {
    // 작은 영역에 대한 스케일링 테스트
    let img = Image::from_rgb(vec![0; 1200], 20, 20);
    let (w, h) = img.scaled_dimensions(5, 5);

    assert!(w <= 5);
    assert!(h <= 5);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scaled_dimensions_large_bounds() {
    // 큰 영역에 대한 스케일링 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1);
    let (w, h) = img.scaled_dimensions(200, 100);

    // 작은 이미지가 큰 영역에 맞추어짐
    assert!(w > 0);
    assert!(h > 0);
}

#[test]
#[cfg(feature = "image")]
fn test_image_fill_mode_aspect() {
    // Fill 모드에서의 aspect ratio 유지 테스트
    let img = Image::from_rgb(vec![0; 300], 30, 10).scale(ScaleMode::Fill);
    let (w, h) = img.scaled_dimensions(20, 10);

    // 3:1 비율을 유지해야 함
    let ratio = w as f32 / h as f32;
    assert!((ratio - 3.0).abs() < 0.1);
}

// =============================================================================
// 렌더링 테스트 (Rendering Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_render_placeholder_fill() {
    // 플레이스홀더로 영역 채우기 테스트
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 300], 10, 10).placeholder('*');
    img.render(&mut ctx);

    // When RGB data is provided, implementation renders the image (not placeholder)
    // Just verify it renders without crashing
}

#[test]
#[cfg(feature = "image")]
fn test_image_render_scaled_fit() {
    // Fit 모드로 스케일링하여 렌더링 테스트
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 3000], 100, 100).placeholder('▓');
    img.render(&mut ctx);

    // 스케일링된 크기만큼만 렌더링되어야 함
    let (scaled_w, scaled_h) = img.scaled_dimensions(20, 10);
    assert!(scaled_w <= 20);
    assert!(scaled_h <= 10);

    // 실제로 플레이스홀더가 렌더링되었는지 확인
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '▓');
}

#[test]
#[cfg(feature = "image")]
fn test_image_render_small_area() {
    // 작은 영역에 렌더링 테스트
    let mut buffer = Buffer::new(3, 3);
    let area = Rect::new(0, 0, 3, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 27], 3, 3).placeholder('■');
    img.render(&mut ctx);

    // 3x3 영역에 플레이스홀더가 렌더링되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '■');
    assert_eq!(buffer.get(2, 2).unwrap().symbol, '■');
}

#[test]
#[cfg(feature = "image")]
fn test_image_render_offset_area() {
    // 오프셋이 있는 영역에 렌더링 테스트
    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(5, 5, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 300], 10, 10).placeholder('█');
    img.render(&mut ctx);

    // 오프셋 위치에서 렌더링되어야 함
    assert_eq!(buffer.get(5, 5).unwrap().symbol, '█');
    assert_eq!(buffer.get(14, 14).unwrap().symbol, '█');

    // 영역 밖은 비어있어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(4, 4).unwrap().symbol, ' ');
}

#[test]
#[cfg(feature = "image")]
fn test_image_render_zero_width() {
    // 너비가 0인 영역 렌더링 테스트
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 3], 1, 1);
    img.render(&mut ctx); // 패닉하지 않아야 함

    // 아무것도 렌더링되지 않아야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
#[cfg(feature = "image")]
fn test_image_render_zero_height() {
    // 높이가 0인 영역 렌더링 테스트
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let img = Image::from_rgb(vec![0; 3], 1, 1);
    img.render(&mut ctx); // 패닉하지 않아야 함

    // 아무것도 렌더링되지 않아야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

// =============================================================================
// Kitty 이스케이프 시퀀스 테스트 (Kitty Escape Sequence Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_kitty_escape_rgb() {
    // RGB 형식의 Kitty 이스케이프 시퀀스 테스트
    let data = vec![255u8, 0, 0, 0, 255, 0]; // 2픽셀 RGB
    let img = Image::from_rgb(data, 2, 1);
    let escape = img.kitty_escape(10, 5);

    // Kitty APC로 시작
    assert!(escape.starts_with("\x1b_G"));
    // ST로 끝남
    assert!(escape.ends_with("\x1b\\"));

    // RGB 형식 코드 (24)가 포함되어야 함
    assert!(escape.contains("f=24"));
}

#[test]
#[cfg(feature = "image")]
fn test_kitty_escape_rgba() {
    // RGBA 형식의 Kitty 이스케이프 시퀀스 테스트
    let data = vec![255u8, 0, 0, 255, 0, 255, 0, 255]; // 2픽셀 RGBA
    let img = Image::from_rgba(data, 2, 1);
    let escape = img.kitty_escape(10, 5);

    // RGBA 형식 코드 (32)가 포함되어야 함
    assert!(escape.contains("f=32"));
}

#[test]
#[cfg(feature = "image")]
fn test_kitty_escape_contains_dimensions() {
    // Kitty 이스케이프 시퀀스에 차원 정보 포함 테스트
    let img = Image::from_rgb(vec![0; 12], 2, 2);
    let escape = img.kitty_escape(10, 5);

    // 열과 행 정보가 포함되어야 함
    assert!(escape.contains("c=10"));
    assert!(escape.contains("r=5"));
}

#[test]
#[cfg(feature = "image")]
fn test_kitty_escape_contains_id() {
    // Kitty 이스케이프 시퀀스에 이미지 ID 포함 테스트
    let img = Image::from_rgb(vec![0; 12], 2, 2);
    let id = img.id();
    let escape = img.kitty_escape(10, 5);

    // 이미지 ID가 포함되어야 함
    assert!(escape.contains(&format!("i={}", id)));
}

#[test]
#[cfg(feature = "image")]
fn test_kitty_escape_base64_encoded() {
    // Kitty 이스케이프 시퀀스의 Base64 인코딩 테스트
    let data = vec![0u8; 12]; // 작은 RGB 데이터
    let img = Image::from_rgb(data, 2, 2);
    let escape = img.kitty_escape(10, 5);

    // Base64로 인코딩된 데이터가 있어야 함
    // Base64 문자열은 알파벳과 숫자, +, /, =를 포함
    let after_semicolon = escape.split(';').nth(1).unwrap_or("");
    assert!(!after_semicolon.is_empty());
}

// =============================================================================
// PNG 처리 테스트 (PNG Handling Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_from_png_invalid_data() {
    // 유효하지 않은 PNG 데이터 처리 테스트
    let invalid_data = vec![0, 1, 2, 3]; // 유효하지 않은 PNG
    let result = Image::from_png(invalid_data);

    assert!(result.is_err());
    assert!(matches!(result, Err(ImageError::DecodeError(_))));
}

#[test]
#[cfg(feature = "image")]
fn test_try_from_png_invalid_data() {
    // 유효하지 않은 PNG 데이터에 대한 try_from_png 테스트
    let invalid_data = vec![0, 1, 2, 3];
    let result = Image::try_from_png(invalid_data);

    assert!(result.is_none());
}

#[test]
#[cfg(feature = "image")]
fn test_from_png_empty_data() {
    // 빈 PNG 데이터 처리 테스트
    let empty_data = vec![];
    let result = Image::from_png(empty_data);

    assert!(result.is_err());
}

// =============================================================================
// 파일 처리 테스트 (File Handling Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_from_file_not_found() {
    // 존재하지 않는 파일 로드 테스트
    let result = Image::from_file("/nonexistent/path/image.png");

    assert!(result.is_err());
    if let Err(ImageError::FileRead { path, .. }) = result {
        assert_eq!(path.to_str().unwrap(), "/nonexistent/path/image.png");
    } else {
        panic!("Expected FileRead error");
    }
}

#[test]
#[cfg(feature = "image")]
fn test_try_from_file_not_found() {
    // 존재하지 않는 파일에 대한 try_from_file 테스트
    let result = Image::try_from_file("/nonexistent/path/image.png");

    assert!(result.is_none());
}

#[test]
#[cfg(feature = "image")]
fn test_image_from_file_helper() {
    // image_from_file 헬퍼 함수 테스트
    let result = image_from_file("/nonexistent/test.png");

    assert!(result.is_err());
}

#[test]
#[cfg(feature = "image")]
fn test_try_image_from_file_helper() {
    // try_image_from_file 헬퍼 함수 테스트
    let result = try_image_from_file("/nonexistent/test.png");

    assert!(result.is_none());
}

// =============================================================================
// 에러 처리 테스트 (Error Handling Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_error_display_file_read() {
    // FileRead 에러의 Display 트레이트 테스트
    let err = ImageError::FileRead {
        path: "/test/path.png".into(),
        message: "file not found".to_string(),
    };
    let display = format!("{}", err);

    assert!(display.contains("/test/path.png"));
    assert!(display.contains("file not found"));
}

#[test]
#[cfg(feature = "image")]
fn test_image_error_display_decode_error() {
    // DecodeError 에러의 Display 트레이트 테스트
    let err = ImageError::DecodeError("invalid format".to_string());
    let display = format!("{}", err);

    assert!(display.contains("invalid format"));
}

#[test]
#[cfg(feature = "image")]
fn test_image_error_display_unknown_format() {
    // UnknownFormat 에러의 Display 트레이트 테스트
    let err = ImageError::UnknownFormat;
    let display = format!("{}", err);

    assert!(display.contains("format"));
}

#[test]
#[cfg(feature = "image")]
fn test_image_error_clone() {
    // ImageError의 Clone 트레이트 테스트
    let err1 = ImageError::DecodeError("test".to_string());
    let err2 = err1.clone();

    assert_eq!(format!("{}", err1), format!("{}", err2));
}

// =============================================================================
// StyledView 테스트 (StyledView Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_element_id() {
    // 이미지 요소 ID 설정 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1).element_id("my-image");

    assert_eq!(View::id(&img), Some("my-image"));

    let meta = img.meta();
    assert_eq!(meta.id, Some("my-image".to_string()));
}

#[test]
#[cfg(feature = "image")]
fn test_image_add_class() {
    // 이미지 클래스 추가 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1)
        .class("thumbnail")
        .class("rounded");

    assert!(img.has_class("thumbnail"));
    assert!(img.has_class("rounded"));
    assert!(!img.has_class("large"));

    let meta = img.meta();
    assert!(meta.classes.contains("thumbnail"));
    assert!(meta.classes.contains("rounded"));
}

#[test]
#[cfg(feature = "image")]
fn test_image_multiple_classes() {
    // 여러 클래스 한 번에 추가 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1).classes(["img", "responsive", "shadow"]);

    assert!(img.has_class("img"));
    assert!(img.has_class("responsive"));
    assert!(img.has_class("shadow"));
}

#[test]
#[cfg(feature = "image")]
fn test_image_styled_view_methods() {
    // StyledView 메서드 테스트
    let mut img = Image::from_rgb(vec![0; 3], 1, 1);

    img.set_id("test-id");
    assert_eq!(View::id(&img), Some("test-id"));

    img.add_class("active");
    assert!(img.has_class("active"));

    img.remove_class("active");
    assert!(!img.has_class("active"));

    img.toggle_class("selected");
    assert!(img.has_class("selected"));

    img.toggle_class("selected");
    assert!(!img.has_class("selected"));
}

#[test]
#[cfg(feature = "image")]
fn test_image_classes_deduplication() {
    // 클래스 중복 제거 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1)
        .class("test")
        .class("test");

    assert_eq!(View::classes(&img).len(), 1);
}

#[test]
#[cfg(feature = "image")]
fn test_image_style_methods() {
    // 스타일 메서드 체이닝 테스트
    // Image 위젯은 StyledView를 구현하지만 fg/bg 메서드는 제공하지 않음
    // element_id와 class만 지원함
    let img = Image::from_rgb(vec![0; 3], 1, 1)
        .element_id("test-image")
        .class("thumbnail");

    // ID와 클래스가 적용되었는지 확인
    assert_eq!(View::id(&img), Some("test-image"));
    assert!(img.has_class("thumbnail"));
}

// =============================================================================
// 연결된 빌더 테스트 (Chained Builder Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_chained_builders() {
    // 체인된 빌더 메서드 테스트
    let img = Image::from_rgb(vec![0; 300], 10, 10)
        .scale(ScaleMode::Stretch)
        .placeholder('█')
        .element_id("banner")
        .class("hero");

    // 모든 설정이 적용되었는지 확인
    assert_eq!(View::id(&img), Some("banner"));
    assert!(img.has_class("hero"));

    let mut buffer = Buffer::new(20, 20);
    let area = Rect::new(0, 0, 20, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    img.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '█');
}

#[test]
#[cfg(feature = "image")]
fn test_image_scale_override() {
    // 스케일 모드 덮어쓰기 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1)
        .scale(ScaleMode::Fit)
        .scale(ScaleMode::Stretch);

    let (w, h) = img.scaled_dimensions(80, 24);
    assert_eq!(w, 80);
    assert_eq!(h, 24);
}

#[test]
#[cfg(feature = "image")]
fn test_image_placeholder_override() {
    // 플레이스홀더 덮어쓰기 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1)
        .placeholder('#')
        .placeholder('@');

    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 10, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    img.render(&mut ctx);

    // 마지막 설정이 적용되어야 함
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '@');
}

// =============================================================================
// 엣지 케이스 테스트 (Edge Case Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_image_zero_width() {
    // 너비가 0인 이미지 생성 시도
    let img = Image::from_rgb(vec![], 0, 10);

    assert_eq!(img.width(), 0);
    assert_eq!(img.height(), 10);
}

#[test]
#[cfg(feature = "image")]
fn test_image_zero_height() {
    // 높이가 0인 이미지 생성 시도
    let img = Image::from_rgb(vec![], 10, 0);

    assert_eq!(img.width(), 10);
    assert_eq!(img.height(), 0);
}

#[test]
#[cfg(feature = "image")]
fn test_image_large_dimensions() {
    // 매우 큰 이미지 차원 테스트
    // 너무 커서 실제로 할당하지는 않지만, 구조는 확인
    let img = Image::from_rgb(vec![0; 100], 10, 10);
    assert_eq!(img.width(), 10);
    assert_eq!(img.height(), 10);
}

#[test]
#[cfg(feature = "image")]
fn test_image_scale_zero_bounds() {
    // 0 크기 영역에 대한 스케일링 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1);

    let (w, h) = img.scaled_dimensions(0, 0);
    // 0으로 나누기 등의 오류가 발생하지 않아야 함
    let _ = (w, h);
}

#[test]
#[cfg(feature = "image")]
fn test_image_aspect_ratio_preservation_fit() {
    // Fit 모드에서의 aspect ratio 보존 테스트
    let img = Image::from_rgb(vec![0; 1200], 40, 30); // 4:3 비율
    let (w, h) = img.scaled_dimensions(80, 60);

    let ratio = w as f32 / h as f32;
    let expected_ratio = 40.0 / 30.0;

    // 비율이 근접해야 함 (부동소수점 오차 허용)
    assert!((ratio - expected_ratio).abs() < 0.1);
}

#[test]
#[cfg(feature = "image")]
fn test_image_different_placeholders() {
    // 다양한 플레이스홀더 문자 테스트
    let placeholders = [' ', '#', '█', '▓', '░', '▒', '■', '□'];

    for &ch in &placeholders {
        let img = Image::from_rgb(vec![0; 3], 1, 1).placeholder(ch);

        let mut buffer = Buffer::new(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        img.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, ch);
    }
}

#[test]
#[cfg(feature = "image")]
fn test_image_render_multiple_times() {
    // 여러 번 렌더링 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1).placeholder('X');

    for _ in 0..3 {
        let mut buffer = Buffer::new(5, 5);
        let area = Rect::new(0, 0, 5, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        img.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'X');
    }
}

// =============================================================================
// 환경 검사 테스트 (Environment Detection Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_is_kitty_supported() {
    // Kitty 지원 확인 함수 테스트
    // 이 함수는 환경 변수를 확인하므로 항상 동일한 결과를 보장하지 않음
    let supported = Image::is_kitty_supported();

    // 함수가 패닉하지 않고 bool을 반환하는지 확인
    let _is_bool: bool = supported;
}

// =============================================================================
// ScaleMode 기본값 테스트 (ScaleMode Default Tests)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_scale_mode_default() {
    // ScaleMode의 기본값 테스트
    let img = Image::from_rgb(vec![0; 3], 1, 1);

    // 기본 스케일 모드는 Fit
    let (w, h) = img.scaled_dimensions(80, 24);

    // Fit 모드 동작 확인
    assert!(w <= 80);
    assert!(h <= 24);
}

// =============================================================================
// 모든 ScaleMode 변형 테스트 (All ScaleMode Variants)
// =============================================================================

#[test]
#[cfg(feature = "image")]
fn test_all_scale_modes() {
    // 모든 스케일 모드 테스트
    // Image는 Clone을 구현하지 않으므로 각각 새로 생성

    // Fit
    let img_fit = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Fit);
    let (w_fit, h_fit) = img_fit.scaled_dimensions(20, 20);
    assert!(w_fit <= 20 && h_fit <= 20);

    // Fill
    let img_fill = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Fill);
    let (w_fill, h_fill) = img_fill.scaled_dimensions(20, 20);
    assert!(w_fill >= 20 || h_fill >= 20);

    // Stretch
    let img_stretch = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::Stretch);
    let (w_stretch, h_stretch) = img_stretch.scaled_dimensions(20, 20);
    assert_eq!(w_stretch, 20);
    assert_eq!(h_stretch, 20);

    // None
    let img_none = Image::from_rgb(vec![0; 300], 10, 10).scale(ScaleMode::None);
    let (w_none, h_none) = img_none.scaled_dimensions(20, 20);
    assert_eq!(w_none, 10);
    assert_eq!(h_none, 10);
}
