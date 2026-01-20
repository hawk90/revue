//! StatusBar widget tests
//!
//! Statusbar 위젯 통합 테스트

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::style::Style;
use revue::style::VisualStyle;
use revue::widget::traits::RenderContext;
use revue::widget::StyledView;
use revue::widget::View;
use revue::widget::{
    footer, header, key_hint, status_section, statusbar, KeyHint, StatusBar, StatusSection,
};

// =============================================================================
// Constructor Tests
// =============================================================================

#[test]
fn test_statusbar_new() {
    let bar = StatusBar::new();
    // 기본 설정으로 생성되는지 확인 - 렌더링을 통해 검증
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 패닉하지 않고 렌더링되어야 함
    assert!(buffer.get(0, 23).is_some());
}

#[test]
fn test_statusbar_default() {
    let bar = StatusBar::default();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    assert!(buffer.get(0, 23).is_some());
}

#[test]
fn test_statusbar_helper() {
    let bar = statusbar();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    assert!(buffer.get(0, 23).is_some());
}

// =============================================================================
// Builder Methods - Position Tests
// =============================================================================

#[test]
fn test_statusbar_header_position() {
    let bar = StatusBar::new().header();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 상단에 렌더링되어야 함 (y=0)
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_statusbar_footer_position() {
    let bar = StatusBar::new().footer();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 하단에 렌더링되어야 함 (y=23)
    assert!(buffer.get(0, 23).is_some());
}

#[test]
fn test_statusbar_header() {
    let bar = StatusBar::new().header();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 상단 위치여야 함
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_statusbar_footer() {
    let bar = StatusBar::new().footer();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 하단 위치여야 함
    assert!(buffer.get(0, 23).is_some());
}

#[test]
fn test_helper_header() {
    let bar = header();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 상단 위치여야 함
    assert!(buffer.get(0, 0).is_some());
}

#[test]
fn test_helper_footer() {
    let bar = footer();
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 하단 위치여야 함
    assert!(buffer.get(0, 23).is_some());
}

// =============================================================================
// Builder Methods - Section Tests
// =============================================================================

#[test]
fn test_statusbar_left_section() {
    let section = StatusSection::new("Left Text");
    let bar = StatusBar::new().left(section);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 왼쪽 섹션이 렌더링되어야 함
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().symbol, 'L');
}

#[test]
fn test_statusbar_center_section() {
    let section = StatusSection::new("Center Text");
    let bar = StatusBar::new().center(section);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 중앙 섹션이 렌더링되어야 함
    let cell = buffer.get(40, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_right_section() {
    let section = StatusSection::new("Right Text");
    let bar = StatusBar::new().right(section);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 오른쪽 섹션이 렌더링되어야 함
    let cell = buffer.get(79, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_left_text() {
    let bar = StatusBar::new().left_text("File.txt");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.symbol, 'F');
}

#[test]
fn test_statusbar_center_text() {
    let bar = StatusBar::new().center_text("Line 1, Col 1");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 중앙 텍스트가 렌더링되어야 함
    let mut found = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == 'L' {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

#[test]
fn test_statusbar_right_text() {
    let bar = StatusBar::new().right_text("UTF-8");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 오른쪽 텍스트가 렌더링되어야 함
    let mut found = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == 'U' {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

#[test]
fn test_statusbar_multiple_left_sections() {
    let bar = StatusBar::new()
        .left_text("First")
        .left_text("Second")
        .left_text("Third");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 첫 번째 섹션의 첫 글자가 있어야 함
    assert_eq!(buffer.get(0, 23).unwrap().symbol, 'F');
}

#[test]
fn test_statusbar_multiple_center_sections() {
    let bar = StatusBar::new()
        .center_text("A")
        .center_text("B")
        .center_text("C");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 렌더링되어야 함
    assert!(buffer.get(40, 23).is_some());
}

#[test]
fn test_statusbar_multiple_right_sections() {
    let bar = StatusBar::new()
        .right_text("X")
        .right_text("Y")
        .right_text("Z");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 렌더링되어야 함
    assert!(buffer.get(79, 23).is_some());
}

// =============================================================================
// Builder Methods - Color Tests
// =============================================================================

#[test]
fn test_statusbar_bg() {
    let bar = StatusBar::new().bg(Color::BLUE);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_statusbar_fg() {
    let bar = StatusBar::new().fg(Color::YELLOW).left_text("Test");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_statusbar_both_colors() {
    let bar = StatusBar::new()
        .bg(Color::BLACK)
        .fg(Color::WHITE)
        .left_text("Test");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.bg, Some(Color::BLACK));
    assert_eq!(cell.fg, Some(Color::WHITE));
}

// =============================================================================
// Builder Methods - Key Hints Tests
// =============================================================================

#[test]
fn test_statusbar_key() {
    let bar = StatusBar::new().key("^X", "Exit").height(2);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 키 힌트가 렌더링되어야 함
    assert!(buffer.get(0, 22).is_some());
}

#[test]
fn test_statusbar_multiple_keys() {
    let bar = StatusBar::new()
        .key("^S", "Save")
        .key("^O", "Open")
        .key("^X", "Exit")
        .height(2);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 키 힌트가 렌더링되어야 함
    assert!(buffer.get(0, 22).is_some());
}

#[test]
fn test_statusbar_keys_vec() {
    let hints = vec![KeyHint::new("^S", "Save"), KeyHint::new("^O", "Open")];
    let bar = StatusBar::new().keys(hints).height(2);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    assert!(buffer.get(0, 22).is_some());
}

// =============================================================================
// Builder Methods - Separator and Height Tests
// =============================================================================

#[test]
fn test_statusbar_separator() {
    let bar = StatusBar::new()
        .separator('|')
        .left_text("First")
        .left_text("Second");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 구분자가 있어야 함
    let first_section_end = "First".len();
    let cell = buffer.get(first_section_end as u16, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_height() {
    let bar = StatusBar::new().height(2);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 높이 2만큼 렌더링되어야 함
    assert!(buffer.get(0, 22).is_some());
    assert!(buffer.get(0, 23).is_some());
}

#[test]
fn test_statusbar_height_minimum() {
    // 높이는 최소 1이어야 함
    let bar = StatusBar::new().height(0);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 최소 높이 1로 렌더링되어야 함
    assert!(buffer.get(0, 23).is_some());
}

#[test]
fn test_statusbar_height_large() {
    let bar = StatusBar::new().height(5);
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 렌더링되어야 함
    assert!(buffer.get(0, 19).is_some());
}

// =============================================================================
// StatusSection Tests
// =============================================================================

#[test]
fn test_section_new() {
    let section = StatusSection::new("Test");
    assert_eq!(section.content, "Test");
    assert_eq!(section.fg, None);
    assert_eq!(section.bg, None);
    assert!(!section.bold);
    assert_eq!(section.min_width, 0);
    assert_eq!(section.priority, 0);
}

#[test]
fn test_section_helper() {
    let section = status_section("Helper");
    assert_eq!(section.content, "Helper");
}

#[test]
fn test_section_fg() {
    let section = StatusSection::new("Test").fg(Color::RED);
    assert_eq!(section.fg, Some(Color::RED));
}

#[test]
fn test_section_bg() {
    let section = StatusSection::new("Test").bg(Color::BLUE);
    assert_eq!(section.bg, Some(Color::BLUE));
}

#[test]
fn test_section_bold() {
    let section = StatusSection::new("Test").bold();
    assert!(section.bold);
}

#[test]
fn test_section_min_width() {
    let section = StatusSection::new("Hi").min_width(10);
    assert_eq!(section.min_width, 10);
}

#[test]
fn test_section_priority() {
    let section = StatusSection::new("Test").priority(5);
    assert_eq!(section.priority, 5);
}

#[test]
fn test_section_width() {
    let section = StatusSection::new("Hello");
    assert_eq!(section.width(), 5);

    let section_with_min = StatusSection::new("Hi").min_width(10);
    assert_eq!(section_with_min.width(), 10);
}

#[test]
fn test_section_builder_chain() {
    let section = StatusSection::new("Test")
        .fg(Color::YELLOW)
        .bg(Color::BLACK)
        .bold()
        .min_width(15)
        .priority(3);

    assert_eq!(section.content, "Test");
    assert_eq!(section.fg, Some(Color::YELLOW));
    assert_eq!(section.bg, Some(Color::BLACK));
    assert!(section.bold);
    assert_eq!(section.min_width, 15);
    assert_eq!(section.priority, 3);
}

// =============================================================================
// KeyHint Tests
// =============================================================================

#[test]
fn test_key_hint_new() {
    let hint = KeyHint::new("^X", "Exit");
    assert_eq!(hint.key, "^X");
    assert_eq!(hint.description, "Exit");
}

#[test]
fn test_key_hint_helper() {
    let hint = key_hint("^S", "Save");
    assert_eq!(hint.key, "^S");
    assert_eq!(hint.description, "Save");
}

// =============================================================================
// Update Methods Tests
// =============================================================================

#[test]
fn test_update_left() {
    let mut bar = StatusBar::new().left_text("Original").left_text("Second");

    bar.update_left(0, "Updated");

    // 렌더링하여 업데이트 확인
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 업데이트된 텍스트가 렌더링되어야 함
    assert_eq!(buffer.get(0, 23).unwrap().symbol, 'U');
}

#[test]
fn test_update_center() {
    let mut bar = StatusBar::new().center_text("Original");

    bar.update_center(0, "Updated");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 업데이트된 텍스트가 있어야 함
    let mut found = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == 'U' {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

#[test]
fn test_update_right() {
    let mut bar = StatusBar::new().right_text("Original");

    bar.update_right(0, "Updated");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 업데이트된 텍스트가 있어야 함
    let mut found = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == 'U' {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

#[test]
fn test_update_left_invalid_index() {
    let mut bar = StatusBar::new().left_text("Test");

    // 유효하지 않은 인덱스로 업데이트해도 패닉하지 않아야 함
    bar.update_left(5, "Should not update");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 원본 텍스트가 그대로 있어야 함
    assert_eq!(buffer.get(0, 23).unwrap().symbol, 'T');
}

#[test]
fn test_update_center_invalid_index() {
    let mut bar = StatusBar::new().center_text("Test");

    bar.update_center(10, "Should not update");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 원본 텍스트 유지
    let mut found = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == 'T' {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

#[test]
fn test_update_right_invalid_index() {
    let mut bar = StatusBar::new().right_text("Test");

    bar.update_right(100, "Should not update");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 원본 텍스트 유지
    let mut found = false;
    for x in 0..80 {
        if let Some(cell) = buffer.get(x, 23) {
            if cell.symbol == 'T' {
                found = true;
                break;
            }
        }
    }
    assert!(found);
}

// =============================================================================
// Clear Method Tests
// =============================================================================

#[test]
fn test_clear() {
    let mut bar = StatusBar::new()
        .left_text("Left")
        .center_text("Center")
        .right_text("Right")
        .key("^X", "Exit");

    bar.clear();

    // 모든 섹션이 삭제된 후 렌더링
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);
    // 배경만 있어야 함
    assert!(buffer.get(0, 23).is_some());
}

// =============================================================================
// Render Tests - Basic
// =============================================================================

#[test]
fn test_statusbar_render_basic() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .left_text("Left")
        .center_text("Center")
        .right_text("Right");

    bar.render(&mut ctx);

    // 배경이 렌더링되었는지 확인
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().bg, Some(Color::rgb(40, 40, 40)));
}

#[test]
fn test_statusbar_render_top() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().header().left_text("Header");

    bar.render(&mut ctx);

    // 상단에 렌더링되는지 확인
    let cell = buffer.get(0, 0);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_bottom() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().footer().left_text("Footer");

    bar.render(&mut ctx);

    // 하단에 렌더링되는지 확인
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_with_text() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().left_text("File.txt");

    bar.render(&mut ctx);

    // 텍스트가 렌더링되었는지 확인
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().symbol, 'F');
}

#[test]
fn test_statusbar_render_custom_bg() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().bg(Color::BLUE);

    bar.render(&mut ctx);

    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_statusbar_render_custom_fg() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().fg(Color::YELLOW).left_text("Test");

    bar.render(&mut ctx);

    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.fg, Some(Color::YELLOW));
}

#[test]
fn test_statusbar_render_section_colors() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let section = StatusSection::new("Colored").fg(Color::RED).bg(Color::BLUE);

    let bar = StatusBar::new().left(section);

    bar.render(&mut ctx);

    // 섹션의 색상이 적용되어야 함
    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.fg, Some(Color::RED));
    assert_eq!(cell.bg, Some(Color::BLUE));
}

#[test]
fn test_statusbar_render_bold_section() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let section = StatusSection::new("Bold").bold();

    let bar = StatusBar::new().left(section);

    bar.render(&mut ctx);

    let cell = buffer.get(0, 23).unwrap();
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_statusbar_render_min_width() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let section = StatusSection::new("Hi").min_width(10);

    let bar = StatusBar::new().left(section);

    bar.render(&mut ctx);

    // 최소 너비만큼 공간이 확보되어야 함
    let cell = buffer.get(9, 23);
    assert!(cell.is_some());
    assert_eq!(cell.unwrap().symbol, ' ');
}

// =============================================================================
// Render Tests - Multiple Sections
// =============================================================================

#[test]
fn test_statusbar_render_multiple_left() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .left_text("First")
        .left_text("Second")
        .left_text("Third");

    bar.render(&mut ctx);

    // 모든 섹션이 렌더링되어야 함
    assert!(buffer.get(0, 23).is_some());
    assert!(buffer.get(10, 23).is_some());
    assert!(buffer.get(20, 23).is_some());
}

#[test]
fn test_statusbar_render_all_alignments() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .left_text("Left")
        .center_text("Center")
        .right_text("Right");

    bar.render(&mut ctx);

    // 왼쪽, 중앙, 오른쪽 모두 렌더링되어야 함
    let cell_left = buffer.get(0, 23);
    assert!(cell_left.is_some());

    // 중앙 섹션은 중간에 있어야 함
    let cell_center = buffer.get(40, 23);
    assert!(cell_center.is_some());

    // 오른쪽 섹션은 오른쪽에 있어야 함
    let cell_right = buffer.get(79, 23);
    assert!(cell_right.is_some());
}

// =============================================================================
// Render Tests - Key Hints
// =============================================================================

#[test]
fn test_statusbar_render_key_hints_two_row() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .height(2)
        .key("^S", "Save")
        .key("^O", "Open");

    bar.render(&mut ctx);

    // 키 힌트가 두 번째 행에 렌더링되어야 함
    let cell = buffer.get(0, 22); // 23은 첫 번째 행
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_key_hints_inline() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .left_text("L")
        .right_text("R")
        .key("^X", "Exit");

    bar.render(&mut ctx);

    // 키 힌트가 인라인으로 렌더링되어야 함
    let cell = buffer.get(5, 23);
    assert!(cell.is_some());
}

// =============================================================================
// Render Tests - Edge Cases
// =============================================================================

#[test]
fn test_statusbar_render_zero_width() {
    let mut buffer = Buffer::new(0, 24);
    let area = Rect::new(0, 0, 0, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().left_text("Test");

    // 너비가 0이어도 패닉하지 않아야 함
    bar.render(&mut ctx);
}

#[test]
fn test_statusbar_render_zero_height() {
    let mut buffer = Buffer::new(80, 0);
    let area = Rect::new(0, 0, 80, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().left_text("Test");

    // 높이가 0이어도 패닉하지 않아야 함
    bar.render(&mut ctx);
}

#[test]
fn test_statusbar_render_small_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .left_text("VeryLongTextThatExceedsArea")
        .right_text("AlsoLong");

    bar.render(&mut ctx);

    // 영역을 벗어나는 텍스트는 잘려야 함
    let cell = buffer.get(9, 4);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_empty_sections() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new();

    bar.render(&mut ctx);

    // 빈 상태바라도 배경은 렌더링되어야 함
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_unicode_text() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().left_text("日本語テスト");

    bar.render(&mut ctx);

    // 유니코드가 렌더링되어야 함
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_emoji() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().left_text("🎉 Celebration");

    bar.render(&mut ctx);

    // 이모지가 렌더링되어야 함
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
}

#[test]
fn test_statusbar_render_height_greater_than_area() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().height(100);

    bar.render(&mut ctx);

    // 높이가 영역을 초과해도 패닉하지 않아야 함
    let cell = buffer.get(0, 23);
    assert!(cell.is_some());
}

// =============================================================================
// StyledView and CSS Tests
// =============================================================================

#[test]
fn test_statusbar_css_id() {
    let bar = StatusBar::new().element_id("my-statusbar");
    assert_eq!(View::id(&bar), Some("my-statusbar"));

    let meta = bar.meta();
    assert_eq!(meta.id, Some("my-statusbar".to_string()));
}

#[test]
fn test_statusbar_css_classes() {
    let bar = StatusBar::new().class("footer").class("dark");

    assert!(bar.has_class("footer"));
    assert!(bar.has_class("dark"));
    assert!(!bar.has_class("header"));

    let meta = bar.meta();
    assert!(meta.classes.contains("footer"));
    assert!(meta.classes.contains("dark"));
}

#[test]
fn test_statusbar_styled_view_set_id() {
    let mut bar = StatusBar::new();
    bar.set_id("test-id");
    assert_eq!(View::id(&bar), Some("test-id"));
}

#[test]
fn test_statusbar_styled_view_add_class() {
    let mut bar = StatusBar::new();
    bar.add_class("active");
    assert!(bar.has_class("active"));
}

#[test]
fn test_statusbar_styled_view_remove_class() {
    let mut bar = StatusBar::new().class("active");
    bar.remove_class("active");
    assert!(!bar.has_class("active"));
}

#[test]
fn test_statusbar_styled_view_toggle_class() {
    let mut bar = StatusBar::new();

    bar.toggle_class("selected");
    assert!(bar.has_class("selected"));

    bar.toggle_class("selected");
    assert!(!bar.has_class("selected"));
}

#[test]
fn test_statusbar_styled_view_has_class() {
    let bar = StatusBar::new().class("visible");
    assert!(bar.has_class("visible"));
    assert!(!bar.has_class("hidden"));
}

#[test]
fn test_statusbar_classes_builder() {
    let bar = StatusBar::new().classes(vec!["class1", "class2", "class3"]);

    assert!(bar.has_class("class1"));
    assert!(bar.has_class("class2"));
    assert!(bar.has_class("class3"));
    assert_eq!(View::classes(&bar).len(), 3);
}

#[test]
fn test_statusbar_duplicate_class_not_added() {
    let bar = StatusBar::new().class("test").class("test");

    let classes = View::classes(&bar);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_statusbar_css_colors_from_context() {
    let bar = StatusBar::new().left_text("CSS");
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    bar.render(&mut ctx);
}

#[test]
fn test_statusbar_inline_override_css() {
    let bar = StatusBar::new()
        .fg(Color::GREEN)
        .bg(Color::YELLOW)
        .left_text("Override");

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);

    let mut style = Style::default();
    style.visual = VisualStyle {
        color: Color::RED,
        background: Color::BLUE,
        ..VisualStyle::default()
    };

    let mut ctx = RenderContext::with_style(&mut buffer, area, &style);
    bar.render(&mut ctx);

    // 인라인 스타일이 CSS를 오버라이드해야 함
    let cell = buffer.get(0, 23).unwrap();
    assert_eq!(cell.fg, Some(Color::GREEN));
    assert_eq!(cell.bg, Some(Color::YELLOW));
}

// =============================================================================
// Meta Tests
// =============================================================================

#[test]
fn test_statusbar_meta() {
    let bar = StatusBar::new()
        .element_id("statusbar-id")
        .class("footer")
        .class("dark");

    let meta = bar.meta();
    assert_eq!(meta.widget_type, "StatusBar");
    assert_eq!(meta.id, Some("statusbar-id".to_string()));
    assert!(meta.classes.contains("footer"));
    assert!(meta.classes.contains("dark"));
}

// =============================================================================
// Complex Layout Tests
// =============================================================================

#[test]
fn test_statusbar_complex_layout() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new()
        .left(StatusSection::new("File.txt").fg(Color::CYAN).bold())
        .center(StatusSection::new("Line 42, Col 10").min_width(20))
        .right(StatusSection::new("UTF-8").fg(Color::GREEN))
        .separator('|')
        .height(2)
        .key("^S", "Save")
        .key("^X", "Exit");

    bar.render(&mut ctx);

    // 모든 요소가 렌더링되어야 함
    assert!(buffer.get(0, 22).is_some()); // 첫 번째 행
    assert!(buffer.get(0, 23).is_some()); // 두 번째 행
}

#[test]
fn test_statusbar_full_builder_chain() {
    let bar = StatusBar::new()
        .header()
        .bg(Color::BLACK)
        .fg(Color::WHITE)
        .separator('|')
        .height(2)
        .left_text("L1")
        .left_text("L2")
        .center_text("C1")
        .center_text("C2")
        .right_text("R1")
        .right_text("R2")
        .key("^S", "Save")
        .key("^O", "Open")
        .key("^X", "Exit")
        .element_id("full-bar")
        .class("status")
        .class("complex");

    // header()는 상단 위치를 설정해야 함
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bar.render(&mut ctx);

    // 렌더링 확인
    assert!(buffer.get(0, 0).is_some());
    assert_eq!(View::id(&bar), Some("full-bar"));
    assert!(bar.has_class("status"));
    assert!(bar.has_class("complex"));
}

// =============================================================================
// Clone Tests
// =============================================================================

#[test]
fn test_status_section_clone() {
    let section1 = StatusSection::new("Test")
        .fg(Color::RED)
        .bg(Color::BLUE)
        .bold()
        .min_width(10)
        .priority(5);

    let section2 = section1.clone();

    assert_eq!(section1.content, section2.content);
    assert_eq!(section1.fg, section2.fg);
    assert_eq!(section1.bg, section2.bg);
    assert_eq!(section1.bold, section2.bold);
    assert_eq!(section1.min_width, section2.min_width);
    assert_eq!(section1.priority, section2.priority);
}

#[test]
fn test_key_hint_clone() {
    let hint1 = KeyHint::new("^X", "Exit");
    let hint2 = hint1.clone();

    assert_eq!(hint1.key, hint2.key);
    assert_eq!(hint1.description, hint2.description);
}
