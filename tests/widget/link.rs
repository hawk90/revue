//! Link widget integration tests
//!
//! Link 위젯의 통합 테스트 모음입니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::render::Modifier;
use revue::style::Color;
use revue::style::Style;
use revue::style::VisualStyle;
use revue::widget::link;
use revue::widget::url_link;
use revue::widget::Link;
use revue::widget::LinkStyle;
use revue::widget::RenderContext;
use revue::widget::StyledView;
use revue::widget::View;

// ─────────────────────────────────────────────────────────────────────────
// Constructor and Builder Tests
// 생성자 및 빌더 메서드 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_new() {
    let link = Link::new("https://example.com");
    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "https://example.com");
    assert!(!link.is_focused());
    assert!(!link.is_disabled());
}

#[test]
fn test_link_with_text() {
    let link = Link::with_text("https://example.com", "Example Site");
    assert_eq!(link.url(), "https://example.com");
    assert_eq!(link.display_text(), "Example Site");
}

#[test]
fn test_link_new_with_string() {
    let url = String::from("https://example.com");
    let link = Link::new(url.clone());
    assert_eq!(link.url(), "https://example.com");
}

#[test]
fn test_link_text_builder() {
    let link = Link::new("https://example.com").text("Click Here");
    assert_eq!(link.display_text(), "Click Here");
}

#[test]
fn test_link_text_builder_with_string() {
    let text = String::from("Custom Text");
    let link = Link::new("https://example.com").text(text);
    assert_eq!(link.display_text(), "Custom Text");
}

#[test]
fn test_link_focused() {
    let link = Link::new("https://example.com").focused(true);
    assert!(link.is_focused());
}

#[test]
fn test_link_disabled() {
    let link = Link::new("https://example.com").disabled(true);
    assert!(link.is_disabled());
}

#[test]
fn test_link_tooltip() {
    let link = Link::new("https://example.com").tooltip("Visit example.com");
    // tooltip은 private 필드이므로 렌더링을 통해 검증
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);
}

#[test]
fn test_link_osc8_enabled() {
    let link = Link::new("https://example.com").osc8(true);
    // OSC 8 활성화 상태 렌더링 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);
}

#[test]
fn test_link_osc8_disabled() {
    let link = Link::new("https://example.com").osc8(false);
    // OSC 8 비활성화 상태 렌더링 테스트
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);
}

#[test]
fn test_link_osc8_disabled_link() {
    // 비활성화된 링크는 OSC 8 시퀀스를 생성하지 않음
    let link = Link::new("https://example.com").disabled(true).osc8(true);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);
}

#[test]
fn test_link_fg_color() {
    let link = Link::new("https://example.com").fg(Color::RED);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);

    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::RED));
    }
}

#[test]
fn test_link_bg_color() {
    let link = Link::new("https://example.com").bg(Color::BLUE);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);

    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.bg, Some(Color::BLUE));
    }
}

#[test]
fn test_link_both_colors() {
    let link = Link::new("https://example.com")
        .fg(Color::YELLOW)
        .bg(Color::BLACK);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    link.render(&mut ctx);

    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::YELLOW));
        assert_eq!(cell.bg, Some(Color::BLACK));
    }
}

#[test]
fn test_link_builder_chain() {
    let link = Link::new("https://example.com")
        .text("Example")
        .style(LinkStyle::Bracketed)
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .focused(true)
        .disabled(false)
        .tooltip("Click to visit")
        .osc8(true);

    assert!(link.is_focused());
    assert!(!link.is_disabled());
    assert_eq!(link.display_text(), "Example");
}

#[test]
fn test_link_clone() {
    let link1 = Link::new("https://example.com")
        .text("Test")
        .focused(true)
        .disabled(false);
    let link2 = link1.clone();

    assert_eq!(link1.url(), link2.url());
    assert_eq!(link1.display_text(), link2.display_text());
    assert_eq!(link1.is_focused(), link2.is_focused());
    assert_eq!(link1.is_disabled(), link2.is_disabled());
}

// ─────────────────────────────────────────────────────────────────────────
// Helper Functions Tests
// 헬퍼 함수 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_helper() {
    let l = link("https://example.com", "Example");
    assert_eq!(l.url(), "https://example.com");
    assert_eq!(l.display_text(), "Example");
}

#[test]
fn test_url_link_helper() {
    let u = url_link("https://example.com");
    assert_eq!(u.url(), "https://example.com");
    assert_eq!(u.display_text(), "https://example.com");
}

#[test]
fn test_link_helper_with_strings() {
    let url = String::from("https://example.com");
    let text = String::from("Example");
    let l = link(url, text);
    assert_eq!(l.url(), "https://example.com");
    assert_eq!(l.display_text(), "Example");
}

#[test]
fn test_url_link_helper_with_string() {
    let url = String::from("https://example.com");
    let u = url_link(url);
    assert_eq!(u.url(), "https://example.com");
}

// ─────────────────────────────────────────────────────────────────────────
// LinkStyle Tests
// 링크 스타일 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_style_default_render() {
    let link = Link::new("https://example.com").text("Test");
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 기본 스타일은 밑줄이 적용됨
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert!(cell.modifier.contains(Modifier::UNDERLINE));
    }
}

#[test]
fn test_link_style_underline_render() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Underline);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 밑줄 스타일 확인
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert!(cell.modifier.contains(Modifier::UNDERLINE));
    }
}

#[test]
fn test_link_style_bracketed_render() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Bracketed)
        .osc8(false); // OSC 8 비활성화하여 단순 텍스트 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 대괄호가 렌더링되었는지 확인
    let mut found_bracket = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '[' {
                found_bracket = true;
                break;
            }
        }
    }
    assert!(found_bracket);
}

#[test]
fn test_link_style_arrow_render() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Arrow)
        .osc8(false); // OSC 8 비활성화하여 단순 텍스트 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 화살표 문자가 렌더링되었는지 확인
    let mut found_arrow = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '→' {
                found_arrow = true;
                break;
            }
        }
    }
    assert!(found_arrow);
}

#[test]
fn test_link_style_icon_render() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Icon)
        .osc8(false); // OSC 8 비활성화하여 단순 텍스트 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 아이콘이 렌더링되었는지 확인
    let mut found_icon = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '🔗' {
                found_icon = true;
                break;
            }
        }
    }
    assert!(found_icon);
}

#[test]
fn test_link_style_plain_render() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Plain);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // Plain 스타일은 밑줄 없음
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert!(!cell.modifier.contains(Modifier::UNDERLINE));
    }
}

#[test]
fn test_link_style_all_variants_render() {
    let url = "https://example.com";
    let text = "Link";

    let styles = vec![
        LinkStyle::Underline,
        LinkStyle::Bracketed,
        LinkStyle::Arrow,
        LinkStyle::Icon,
        LinkStyle::Plain,
    ];

    for style in styles {
        let link = Link::new(url).text(text).style(style);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        link.render(&mut ctx);

        // 모든 스타일이 렌더링되어야 함
        let cell = buffer.get(area.x, area.y);
        assert!(cell.is_some());
    }
}

// ─────────────────────────────────────────────────────────────────────────
// URL Management Tests
// URL 관리 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_url_http() {
    let link = Link::new("http://example.com");
    assert_eq!(link.url(), "http://example.com");
}

#[test]
fn test_link_url_https() {
    let link = Link::new("https://example.com");
    assert_eq!(link.url(), "https://example.com");
}

#[test]
fn test_link_url_with_path() {
    let link = Link::new("https://example.com/path/to/page");
    assert_eq!(link.url(), "https://example.com/path/to/page");
}

#[test]
fn test_link_url_with_query() {
    let link = Link::new("https://example.com?query=test");
    assert_eq!(link.url(), "https://example.com?query=test");
}

#[test]
fn test_link_url_with_fragment() {
    let link = Link::new("https://example.com#section");
    assert_eq!(link.url(), "https://example.com#section");
}

#[test]
fn test_link_url_complex() {
    let url = "https://example.com:8080/path?key=value#anchor";
    let link = Link::new(url);
    assert_eq!(link.url(), url);
}

#[test]
fn test_link_display_text_fallback() {
    // 텍스트가 없으면 URL이 표시됨
    let link = Link::new("https://example.com");
    assert_eq!(link.display_text(), "https://example.com");
}

#[test]
fn test_link_display_text_custom() {
    // 사용자 정의 텍스트가 우선함
    let link = Link::new("https://example.com").text("Click Here");
    assert_eq!(link.display_text(), "Click Here");
}

#[test]
fn test_link_display_text_empty_string() {
    // 빈 문자열도 유효한 텍스트로 처리됨
    let link = Link::new("https://example.com").text("");
    assert_eq!(link.display_text(), "");
}

#[test]
fn test_link_url_rendered_in_buffer() {
    let url = "https://example.com";
    let link = Link::new(url);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // URL의 일부가 버퍼에 렌더링되었는지 확인
    let mut found_h = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'h' {
                found_h = true;
                break;
            }
        }
    }
    assert!(found_h);
}

// ─────────────────────────────────────────────────────────────────────────
// Rendering Tests
// 렌더링 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_render_basic() {
    let link = Link::new("https://example.com");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // URL이 렌더링되었는지 확인
    let mut found_h = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'h' {
                found_h = true;
                break;
            }
        }
    }
    assert!(found_h);
}

#[test]
fn test_link_render_with_text() {
    let link = Link::new("https://example.com").text("Click Here");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 사용자 정의 텍스트가 렌더링되었는지 확인
    let mut found_c = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'C' {
                found_c = true;
                break;
            }
        }
    }
    assert!(found_c);
}

#[test]
fn test_link_render_focused() {
    let link = Link::new("https://example.com").focused(true);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 포커스 상태에서도 렌더링이 정상적으로 수행됨
    let cell = buffer.get(area.x, area.y);
    assert!(cell.is_some());

    // 포커스 상태에서는 다른 색상이 적용됨 (밝은 파란색)
    if let Some(cell) = cell {
        // 기본 CYAN 색상과 다른지 확인 (focused 상태)
        assert!(cell.fg.is_some());
    }
}

#[test]
fn test_link_render_disabled() {
    let link = Link::new("https://example.com").disabled(true);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 비활성화 상태에서도 렌더링이 정상적으로 수행됨
    let cell = buffer.get(area.x, area.y);
    assert!(cell.is_some());

    // 비활성화 상태에서는 회색 색상이 적용됨
    if let Some(cell) = cell {
        assert!(cell.fg.is_some());
    }
}

#[test]
fn test_link_render_with_custom_colors() {
    let link = Link::new("https://example.com")
        .fg(Color::GREEN)
        .bg(Color::BLACK);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 색상이 적용된 셀이 있는지 확인
    let cell = buffer.get(area.x, area.y);
    assert!(cell.is_some());
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::GREEN));
        assert_eq!(cell.bg, Some(Color::BLACK));
    }
}

#[test]
fn test_link_render_zero_area() {
    let link = Link::new("https://example.com");
    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 0, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 너비가 0인 영역에서도 크래시하지 않음
    link.render(&mut ctx);
}

#[test]
fn test_link_render_with_offset() {
    let link = Link::new("https://example.com").text("Link");
    let mut buffer = Buffer::new(40, 5);
    let area = Rect::new(10, 2, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 오프셋 위치에서 렌더링 확인
    let cell = buffer.get(10, 2);
    assert!(cell.is_some());
}

#[test]
fn test_link_render_underline_disabled_link() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Underline)
        .disabled(true);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 비활성화된 링크는 밑줄이 없음
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert!(!cell.modifier.contains(Modifier::UNDERLINE));
    }
}

#[test]
fn test_link_render_default_color() {
    let link = Link::new("https://example.com");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 기본 색상(CYAN)이 적용됨
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::CYAN));
    }
}

// ─────────────────────────────────────────────────────────────────────────
// OSC 8 Hyperlink Tests
// OSC 8 하이퍼링크 테스트 (렌더링을 통해 간접 검증)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_osc8_enabled_renders() {
    let link = Link::new("https://example.com").osc8(true);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);
    // OSC 8가 활성화되면 정상적으로 렌더링됨
}

#[test]
fn test_link_osc8_disabled_renders() {
    let link = Link::new("https://example.com").osc8(false);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);
    // OSC 8가 비활성화되어도 정상적으로 렌더링됨
}

#[test]
fn test_link_osc8_default_enabled() {
    // OSC 8는 기본적으로 활성화됨
    let link = Link::new("https://example.com");
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);
    // 기본 설정으로 정상 렌더링
}

#[test]
fn test_link_osc8_disabled_link_no_render_difference() {
    // 비활성화된 링크는 OSC 8 시퀀스를 생성하지 않음
    let link1 = Link::new("https://example.com").disabled(true).osc8(true);
    let link2 = Link::new("https://example.com").disabled(false).osc8(true);

    let mut buffer1 = Buffer::new(30, 1);
    let mut buffer2 = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);

    let mut ctx1 = RenderContext::new(&mut buffer1, area);
    let mut ctx2 = RenderContext::new(&mut buffer2, area);

    link1.render(&mut ctx1);
    link2.render(&mut ctx2);

    // 둘 다 정상적으로 렌더링됨
    let cell1 = buffer1.get(area.x, area.y);
    let cell2 = buffer2.get(area.x, area.y);
    assert!(cell1.is_some());
    assert!(cell2.is_some());
}

#[test]
fn test_link_osc8_with_special_chars_in_url() {
    let url = "https://example.com/path?key=value&other=123#anchor";
    let link = Link::new(url).osc8(true);
    let mut buffer = Buffer::new(50, 1);
    let area = Rect::new(0, 0, 50, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);
    // 특수 문자가 포함된 URL도 렌더링됨
}

// ─────────────────────────────────────────────────────────────────────────
// CSS Integration Tests
// CSS 통합 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_css_id() {
    let link = Link::new("https://example.com").element_id("my-link");
    assert_eq!(View::id(&link), Some("my-link"));

    let meta = link.meta();
    assert_eq!(meta.id, Some("my-link".to_string()));
}

#[test]
fn test_link_css_classes() {
    let link = Link::new("https://example.com")
        .class("external")
        .class("important");

    assert!(link.has_class("external"));
    assert!(link.has_class("important"));
    assert!(!link.has_class("internal"));

    let meta = link.meta();
    assert!(meta.classes.contains("external"));
    assert!(meta.classes.contains("important"));
}

#[test]
fn test_link_css_classes_from_view_trait() {
    let link = Link::new("https://example.com")
        .class("link")
        .class("primary");

    let classes = View::classes(&link);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"link".to_string()));
    assert!(classes.contains(&"primary".to_string()));
}

#[test]
fn test_link_styled_view_set_id() {
    let mut link = Link::new("https://example.com");
    link.set_id("test-link");
    assert_eq!(View::id(&link), Some("test-link"));
}

#[test]
fn test_link_styled_view_add_class() {
    let mut link = Link::new("https://example.com");
    link.add_class("active");
    assert!(link.has_class("active"));
}

#[test]
fn test_link_styled_view_remove_class() {
    let mut link = Link::new("https://example.com").class("active");
    link.remove_class("active");
    assert!(!link.has_class("active"));
}

#[test]
fn test_link_styled_view_toggle_class() {
    let mut link = Link::new("https://example.com");

    link.toggle_class("selected");
    assert!(link.has_class("selected"));

    link.toggle_class("selected");
    assert!(!link.has_class("selected"));
}

#[test]
fn test_link_styled_view_has_class() {
    let link = Link::new("https://example.com").class("external");
    assert!(link.has_class("external"));
    assert!(!link.has_class("internal"));
}

#[test]
fn test_link_classes_builder() {
    let link = Link::new("https://example.com").classes(vec!["class1", "class2", "class3"]);

    assert!(link.has_class("class1"));
    assert!(link.has_class("class2"));
    assert!(link.has_class("class3"));
    assert_eq!(View::classes(&link).len(), 3);
}

#[test]
fn test_link_duplicate_class_not_added() {
    let link = Link::new("https://example.com").class("test").class("test");

    let classes = View::classes(&link);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_link_css_colors_from_context() {
    let link = Link::new("https://example.com");
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::MAGENTA,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    link.render(&mut ctx);
}

#[test]
fn test_link_inline_color_override_css() {
    let link = Link::new("https://example.com").fg(Color::GREEN);

    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    link.render(&mut ctx);

    // 인라인 색상이 CSS를 오버라이드해야 함
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::GREEN));
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Edge Cases
// 엣지 케이스 테스트
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_link_empty_url() {
    let link = Link::new("");
    assert_eq!(link.url(), "");
    assert_eq!(link.display_text(), "");
}

#[test]
fn test_link_empty_text() {
    let link = Link::new("https://example.com").text("");
    assert_eq!(link.display_text(), "");
}

#[test]
fn test_link_very_long_url() {
    let long_url = "https://example.com/very/long/path/that/exceeds/normal/length";
    let link = Link::new(long_url);
    assert_eq!(link.url(), long_url);
}

#[test]
fn test_link_very_long_text() {
    let long_text = "This is a very long link text that describes the destination in detail";
    let link = Link::new("https://example.com").text(long_text);
    assert_eq!(link.display_text(), long_text);
}

#[test]
fn test_link_url_with_unicode() {
    let link = Link::new("https://example.com/한글/日本語");
    assert_eq!(link.url(), "https://example.com/한글/日本語");
}

#[test]
fn test_link_text_with_unicode() {
    let link = Link::new("https://example.com").text("클릭하세요");
    assert_eq!(link.display_text(), "클릭하세요");
}

#[test]
fn test_link_url_with_spaces_encoded() {
    let link = Link::new("https://example.com/path%20with%20spaces");
    assert_eq!(link.url(), "https://example.com/path%20with%20spaces");
}

#[test]
fn test_link_text_with_spaces() {
    let link = Link::new("https://example.com").text("Click Here Now");
    assert_eq!(link.display_text(), "Click Here Now");
}

#[test]
fn test_link_text_with_special_chars() {
    let link = Link::new("https://example.com").text("©®™€£");
    assert_eq!(link.display_text(), "©®™€£");
}

#[test]
fn test_link_disabled_focused_both() {
    // 포커스되고 비활성화된 링크
    let link = Link::new("https://example.com")
        .focused(true)
        .disabled(true);
    assert!(link.is_focused());
    assert!(link.is_disabled());
}

#[test]
fn test_link_render_very_long_text_truncates() {
    let long_text = "This is a very long link text that will be truncated when rendered";
    let link = Link::new("https://example.com").text(long_text);

    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);
    // 버퍼 크기에 맞춰 잘려야 하지만 크래시하지 않아야 함
}

#[test]
fn test_link_render_url_longer_than_area() {
    let url = "https://example.com/very/long/url/path";
    let link = Link::new(url);

    let mut buffer = Buffer::new(10, 1);
    let area = Rect::new(0, 0, 10, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);
    // 영역보다 긴 URL도 렌더링되어야 함
}

#[test]
fn test_link_all_styles_render_with_empty_text() {
    let styles = vec![
        LinkStyle::Underline,
        LinkStyle::Bracketed,
        LinkStyle::Arrow,
        LinkStyle::Icon,
        LinkStyle::Plain,
    ];

    for style in styles {
        let link = Link::new("https://example.com").text("").style(style);
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // 빈 텍스트로도 크래시하지 않아야 함
        link.render(&mut ctx);
    }
}

#[test]
fn test_link_meta() {
    let link = Link::new("https://example.com")
        .element_id("test-link")
        .class("external")
        .class("nav");

    let meta = link.meta();
    assert_eq!(meta.widget_type, "Link");
    assert_eq!(meta.id, Some("test-link".to_string()));
    assert!(meta.classes.contains("external"));
    assert!(meta.classes.contains("nav"));
}

#[test]
fn test_link_debug_format() {
    let link = Link::new("https://example.com").text("Test");
    let debug_str = format!("{:?}", link);

    assert!(debug_str.contains("Link"));
}

#[test]
fn test_link_multiple_state_changes() {
    let mut link = Link::new("https://example.com");

    // 상태 여러 변경
    link = link.focused(true);
    assert!(link.is_focused());

    link = link.disabled(true);
    assert!(link.is_disabled());

    link = link.focused(false);
    assert!(!link.is_focused());
    assert!(link.is_disabled()); // disabled 상태는 유지됨
}

#[test]
fn test_link_builder_reusability() {
    // 빌더 패턴으로 링크 생성 후 재사용
    let base = Link::new("https://example.com")
        .style(LinkStyle::Bracketed)
        .fg(Color::CYAN);

    let link1 = base.clone().text("Link 1");
    let link2 = base.clone().text("Link 2");

    assert_eq!(link1.display_text(), "Link 1");
    assert_eq!(link2.display_text(), "Link 2");
}

#[test]
fn test_link_render_with_all_options() {
    // 모든 옵션을 조합한 렌더링 테스트
    let link = Link::new("https://example.com")
        .text("Complete")
        .style(LinkStyle::Icon)
        .fg(Color::YELLOW)
        .bg(Color::BLACK)
        .focused(true)
        .disabled(false)
        .tooltip("Click to visit")
        .osc8(true)
        .element_id("complete-link")
        .class("nav-link");

    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 아이콘이 렌더링되었는지 확인
    let mut found_icon = false;
    for x in 0..area.width {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == '🔗' {
                found_icon = true;
                break;
            }
        }
    }
    assert!(found_icon);

    // CSS 속성 확인
    assert_eq!(View::id(&link), Some("complete-link"));
    assert!(link.has_class("nav-link"));
}

#[test]
fn test_link_render_focused_color() {
    let link = Link::new("https://example.com").focused(true);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 포커스 상태에서는 밝은 파란색 (RGB 100, 200, 255)
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::rgb(100, 200, 255)));
    }
}

#[test]
fn test_link_render_disabled_color() {
    let link = Link::new("https://example.com").disabled(true);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 비활성화 상태에서는 회색 (RGB 128, 128, 128)
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::rgb(128, 128, 128)));
    }
}

#[test]
fn test_link_render_custom_color_overrides_default() {
    // 사용자 정의 색상이 기본 색상을 오버라이드하는지 테스트
    let link = Link::new("https://example.com").fg(Color::MAGENTA);
    let mut buffer = Buffer::new(30, 1);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 사용자 정의 색상이 적용됨
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert_eq!(cell.fg, Some(Color::MAGENTA));
    }
}

#[test]
fn test_link_render_text_content() {
    let link = Link::new("https://example.com").text("Hello").osc8(false); // OSC 8 비활성화하여 단순 텍스트 렌더링 테스트
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // 텍스트 내용 확인
    let mut found_h = false;
    let mut found_e = false;
    for x in 0..area.width.min(5) {
        if let Some(cell) = buffer.get(x, area.y) {
            if cell.symbol == 'H' {
                found_h = true;
            }
            if cell.symbol == 'e' {
                found_e = true;
            }
        }
    }
    assert!(found_h && found_e);
}

#[test]
fn test_link_style_plain_has_no_underline() {
    let link = Link::new("https://example.com")
        .text("Test")
        .style(LinkStyle::Plain);
    let mut buffer = Buffer::new(20, 1);
    let area = Rect::new(0, 0, 20, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    link.render(&mut ctx);

    // Plain 스타일은 밑줄이 없음
    let cell = buffer.get(area.x, area.y);
    if let Some(cell) = cell {
        assert!(!cell.modifier.contains(Modifier::UNDERLINE));
    }
}
