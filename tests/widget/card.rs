//! Card widget integration tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::Text;
use revue::widget::{card, BorderType, Card, CardVariant, StyledView, View};

/// Render `c` into a `width` x `height` buffer and return each row as text
/// (wide characters appear once).
fn render_rows(c: &Card, width: u16, height: u16) -> Vec<String> {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .filter_map(|x| buffer.get(x, y))
                .filter(|cell| !cell.is_continuation())
                .map(|cell| cell.symbol)
                .collect()
        })
        .collect()
}

// =============================================================================
// Constructor and Builder Tests
// 생성자 및 빌더 테스트
// =============================================================================

#[test]
fn test_card_new() {
    let c = Card::new();
    assert!(!c.is_collapsible());
    assert!(c.is_expanded());
    assert!(!c.is_disabled());
}

#[test]
fn test_card_default() {
    let c = Card::default();
    assert!(!c.is_collapsible());
    assert!(c.is_expanded());
}

#[test]
fn test_card_helper() {
    let c = card();
    assert!(!c.is_collapsible());
    assert!(c.is_expanded());
}

#[test]
fn test_card_title() {
    let c = Card::new().title("Test Title");
    // Verify it renders correctly
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_title_str() {
    let title = "제목입니다"; // Korean title test
    let c = Card::new().title(title);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Check border corners exist
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_subtitle() {
    let c = Card::new().subtitle("Subtitle");
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_subtitle_korean() {
    let subtitle = "부제목입니다"; // Korean subtitle test
    let c = Card::new().subtitle(subtitle);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_title_and_subtitle() {
    let c = Card::new().title("Main Title").subtitle("Description");
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_body() {
    let body = Text::new("Body content");
    let c = Card::new().body(body);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_header() {
    let header = Text::new("Header content");
    let c = Card::new().header(header);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_footer() {
    let footer = Text::new("Footer content");
    let c = Card::new().footer(footer);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_all_sections() {
    let c = Card::new()
        .title("Title")
        .subtitle("Subtitle")
        .header(Text::new("Header"))
        .body(Text::new("Body"))
        .footer(Text::new("Footer"));
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

// =============================================================================
// Variant Tests
// 변형 테스트
// =============================================================================

#[test]
fn test_card_variant_outlined() {
    let c = Card::new().outlined();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_variant_filled() {
    let c = Card::new().filled();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Filled variant should have background color
    let cell = buffer.get(1, 1).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(30, 30, 35)));
}

#[test]
fn test_card_variant_flat() {
    let c = Card::new().flat();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Flat variant has no border
    assert_ne!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_variant_builder() {
    let c = Card::new().variant(CardVariant::Filled);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    let cell = buffer.get(1, 1).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(30, 30, 35)));
}

#[test]
fn test_card_all_variants() {
    let variants = [
        (CardVariant::Outlined, '┌', None),
        (CardVariant::Filled, '┌', Some(Color::rgb(30, 30, 35))),
        (CardVariant::Elevated, '┌', Some(Color::rgb(35, 35, 40))),
        // variant() only changes the look; the border stays
        (CardVariant::Flat, '┌', None),
    ];

    for (variant, corner, bg) in variants {
        let c = Card::new().variant(variant);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        c.render(&mut ctx);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, corner, "{variant:?}");
        assert_eq!(buffer.get(5, 5).unwrap().bg, bg, "{variant:?}");
    }
}

// =============================================================================
// Border Style Tests
// 테두리 스타일 테스트
// =============================================================================

#[test]
fn test_card_rounded() {
    let c = Card::new().rounded();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Check rounded corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 9).unwrap().symbol, '╰');
    assert_eq!(buffer.get(19, 9).unwrap().symbol, '╯');
}

#[test]
fn test_card_border_style() {
    let c = Card::new().border_style(BorderType::Double);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Double border corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
}

#[test]
fn test_card_flat_removes_border() {
    let c = Card::new().flat();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Flat variant removes border
    assert_ne!(buffer.get(0, 0).unwrap().symbol, '┌');
}

// =============================================================================
// Color Tests
// 색상 테스트
// =============================================================================

#[test]
fn test_card_background_color() {
    let c = Card::new().background(Color::RED);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    let cell = buffer.get(1, 1).unwrap();
    assert_eq!(cell.bg, Some(Color::RED));
}

#[test]
fn test_card_border_color() {
    let c = Card::new().border_color(Color::BLUE);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    let top_left = buffer.get(0, 0).unwrap();
    assert_eq!(top_left.fg, Some(Color::BLUE));
}

#[test]
fn test_card_title_color() {
    let c = Card::new().title("Colored").title_color(Color::GREEN);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Title should be rendered with green color
    // Find first non-border character (title start)
    for x in 1..19 {
        let cell = buffer.get(x, 1).unwrap();
        if cell.symbol == 'C' {
            assert_eq!(cell.fg, Some(Color::GREEN));
            break;
        }
    }
}

#[test]
fn test_card_all_colors() {
    let c = Card::new()
        .title("Title")
        .background(Color::rgb(30, 30, 35))
        .border_color(Color::rgb(60, 60, 70))
        .title_color(Color::WHITE);
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

// =============================================================================
// Collapsible Tests
// 접기/펼치기 테스트
// =============================================================================

#[test]
fn test_card_collapsible() {
    let c = Card::new().collapsible(true);
    assert!(c.is_collapsible());
}

#[test]
fn test_card_not_collapsible() {
    let c = Card::new();
    assert!(!c.is_collapsible());
}

#[test]
fn test_card_is_expanded_default() {
    let c = Card::new();
    assert!(c.is_expanded());
}

#[test]
fn test_card_expanded_method() {
    let c = Card::new().expanded(true);
    assert!(c.is_expanded());
}

#[test]
fn test_card_collapsed_method() {
    let c = Card::new().expanded(false);
    assert!(!c.is_expanded());
}

#[test]
fn test_card_toggle() {
    let mut c = Card::new().collapsible(true);
    assert!(c.is_expanded());

    c.toggle();
    assert!(!c.is_expanded());

    c.toggle();
    assert!(c.is_expanded());
}

#[test]
fn test_card_toggle_not_collapsible() {
    let mut c = Card::new().collapsible(false);
    assert!(c.is_expanded());

    c.toggle();
    // Should remain expanded when not collapsible
    assert!(c.is_expanded());
}

#[test]
fn test_card_expand() {
    let mut c = Card::new().expanded(false);
    assert!(!c.is_expanded());

    c.expand();
    assert!(c.is_expanded());
}

#[test]
fn test_card_collapse() {
    let mut c = Card::new().expanded(true);
    assert!(c.is_expanded());

    c.collapse();
    assert!(!c.is_expanded());
}

// =============================================================================
// Keyboard Handling Tests
// 키보드 처리 테스트
// =============================================================================

#[test]
fn test_card_handle_key_enter() {
    let mut c = Card::new().collapsible(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Enter);
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_card_handle_key_space() {
    let mut c = Card::new().collapsible(true);
    assert!(c.is_expanded());

    let handled = c.handle_key(&Key::Char(' '));
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_card_handle_key_left() {
    let mut c = Card::new().collapsible(true).expanded(true);
    let handled = c.handle_key(&Key::Left);
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_card_handle_key_right() {
    let mut c = Card::new().collapsible(true).expanded(false);
    let handled = c.handle_key(&Key::Right);
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_card_handle_key_h() {
    let mut c = Card::new().collapsible(true).expanded(true);
    let handled = c.handle_key(&Key::Char('h'));
    assert!(handled);
    assert!(!c.is_expanded());
}

#[test]
fn test_card_handle_key_l() {
    let mut c = Card::new().collapsible(true).expanded(false);
    let handled = c.handle_key(&Key::Char('l'));
    assert!(handled);
    assert!(c.is_expanded());
}

#[test]
fn test_card_handle_key_not_collapsible() {
    let mut c = Card::new();
    assert!(!c.handle_key(&Key::Enter));
    assert!(!c.handle_key(&Key::Char(' ')));
    assert!(!c.handle_key(&Key::Left));
    assert!(!c.handle_key(&Key::Right));
}

#[test]
fn test_card_handle_key_unknown() {
    let mut c = Card::new().collapsible(true);
    assert!(!c.handle_key(&Key::Char('x')));
    assert!(!c.handle_key(&Key::Up));
    assert!(!c.handle_key(&Key::Down));
}

// =============================================================================
// Padding Tests
// 패딩 테스트
// =============================================================================

#[test]
fn test_card_padding() {
    let c = Card::new().padding(2);
    // Padding affects rendering - verify it renders correctly
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_padding_zero() {
    let c = Card::new().padding(0);
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_padding_large() {
    let c = Card::new().padding(10);
    let mut buffer = Buffer::new(50, 20);
    let area = Rect::new(0, 0, 50, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

// =============================================================================
// Clickable Tests
// 클릭 가능 테스트
// =============================================================================

#[test]
fn test_card_clickable() {
    let c = Card::new().clickable(true);
    // Clickable is stored but affects focus behavior
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_not_clickable_default() {
    let c = Card::new();
    // Default is not clickable
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

// =============================================================================
// Rendering Tests
// 렌더링 테스트
// =============================================================================

#[test]
fn test_card_render_basic() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new();
    c.render(&mut ctx);

    // Check border corners for outlined variant (default)
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 9).unwrap().symbol, '└');
    assert_eq!(buffer.get(19, 9).unwrap().symbol, '┘');
}

#[test]
fn test_card_render_with_title() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().title("Test Card");
    c.render(&mut ctx);

    // Should still have border corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '┐');

    // Check title appears in first row
    let first_row: String = (0..20)
        .map(|x| buffer.get(x, 1).map(|c| c.symbol).unwrap_or(' '))
        .collect();
    assert!(first_row.contains("Test Card"));
}

#[test]
fn test_card_render_with_korean_title() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().title("테스트 카드"); // "Test Card" in Korean
    c.render(&mut ctx);

    // Check border corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');

    // Title should appear (even with Korean characters)
    let first_row: String = (0..20)
        .map(|x| buffer.get(x, 1).map(|c| c.symbol).unwrap_or(' '))
        .collect();
    assert!(first_row.contains('테') || first_row.contains('카'));
}

#[test]
fn test_card_render_with_title_and_subtitle() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().title("Title").subtitle("Subtitle");
    c.render(&mut ctx);

    // Check border
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_render_with_body() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().title("Title").body(Text::new("Body content"));
    c.render(&mut ctx);

    // Should have separator between title and body
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '├');
}

#[test]
fn test_card_render_with_footer() {
    let c = Card::new()
        .title("Title")
        .body(Text::new("Body"))
        .footer(Text::new("Footer"));
    let rows = render_rows(&c, 20, 10);

    assert_eq!(rows[1], "│ Title            │");
    assert_eq!(rows[2], "├──────────────────┤");
    assert_eq!(rows[3], "│ Body             │");
    // Footer separator, then the footer just above the bottom border
    assert_eq!(rows[7], "├──────────────────┤");
    assert_eq!(rows[8], "│ Footer           │");
    assert_eq!(rows[9], "└──────────────────┘");
}

#[test]
fn test_card_render_rounded_border() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().rounded().title("Rounded");
    c.render(&mut ctx);

    // Check rounded corners
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(19, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 9).unwrap().symbol, '╰');
    assert_eq!(buffer.get(19, 9).unwrap().symbol, '╯');
}

#[test]
fn test_card_render_flat_variant() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().flat().title("Flat");
    c.render(&mut ctx);

    // Flat variant should not have border corners
    assert_ne!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_render_filled_variant() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().filled().title("Filled");
    c.render(&mut ctx);

    // Should have background color
    let cell = buffer.get(1, 1).unwrap();
    assert_eq!(cell.bg, Some(Color::rgb(30, 30, 35)));
}

#[test]
#[ignore = "BUG: Card's elevated shadow is drawn in the last column/row and then overwritten by the border, so it never shows"]
fn test_card_render_elevated_variant() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().elevated().title("Elevated");
    c.render(&mut ctx);

    // Shadow on the right and bottom edges, inside the area (#495)
    assert_eq!(buffer.get(39, 5).unwrap().symbol, '▌');
    assert_eq!(buffer.get(5, 9).unwrap().symbol, '▀');
}

#[test]
fn test_card_render_elevated_without_border() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().elevated().border_style(BorderType::None);
    c.render(&mut ctx);

    // With no border on top, the shadow shows in the last column and row
    let right = buffer.get(19, 5).unwrap();
    assert_eq!(right.symbol, '▌');
    assert_eq!(right.fg, Some(Color::rgb(20, 20, 20)));
    assert_eq!(buffer.get(5, 9).unwrap().symbol, '▀');
    // Not beside the top row
    assert_ne!(buffer.get(19, 0).unwrap().symbol, '▌');
}

#[test]
fn test_card_render_collapsible_expanded() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new()
        .collapsible(true)
        .expanded(true)
        .title("Collapsible")
        .body(Text::new("Body content"));
    c.render(&mut ctx);

    // Should show collapse icon (▼) when expanded
    let first_row: String = (0..20)
        .map(|x| buffer.get(x, 1).map(|c| c.symbol).unwrap_or(' '))
        .collect();
    assert!(first_row.contains('▼'));
}

#[test]
fn test_card_render_collapsible_collapsed() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new()
        .collapsible(true)
        .expanded(false)
        .title("Collapsible")
        .body(Text::new("Body content"));
    c.render(&mut ctx);

    // Should show expand icon (▶) when collapsed
    let first_row: String = (0..20)
        .map(|x| buffer.get(x, 1).map(|c| c.symbol).unwrap_or(' '))
        .collect();
    assert!(first_row.contains('▶'));
}

#[test]
fn test_card_render_with_custom_colors() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new()
        .title("Colored")
        .border_color(Color::RED)
        .title_color(Color::GREEN);
    c.render(&mut ctx);

    // Check border color
    let top_left = buffer.get(0, 0).unwrap();
    assert_eq!(top_left.fg, Some(Color::RED));
}

// =============================================================================
// CSS/Styling Tests
// CSS/스타일링 테스트
// =============================================================================

#[test]
fn test_card_css_id() {
    let c = Card::new().element_id("test-card");
    assert_eq!(View::id(&c), Some("test-card"));
}

#[test]
fn test_card_css_classes() {
    let c = Card::new().class("card-primary").class("shadow");

    assert!(c.has_class("card-primary"));
    assert!(c.has_class("shadow"));
    assert!(!c.has_class("card-secondary"));
}

#[test]
fn test_card_styled_view_methods() {
    let mut c = Card::new();

    c.set_id("my-card");
    assert_eq!(View::id(&c), Some("my-card"));

    c.add_class("active");
    assert!(c.has_class("active"));

    c.toggle_class("active");
    assert!(!c.has_class("active"));

    c.toggle_class("selected");
    assert!(c.has_class("selected"));

    c.remove_class("selected");
    assert!(!c.has_class("selected"));
}

// =============================================================================
// Edge Cases Tests
// 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_card_too_small_area() {
    // Width < 4 or height < 3 draws nothing
    let c = Card::new().title("Test");
    for row in render_rows(&c, 3, 5) {
        assert_eq!(row.trim(), "");
    }
    for row in render_rows(&c, 10, 2) {
        assert_eq!(row.trim(), "");
    }
    assert_eq!(render_rows(&c, 4, 3)[0], "┌──┐");
}

#[test]
fn test_card_empty_title() {
    let c = Card::new().title("");
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_very_long_title() {
    let long_title = "This is a very long title that exceeds the card width";
    let c = Card::new().title(long_title);
    // Clipped to the content width, inside the border and padding
    assert_eq!(render_rows(&c, 20, 10)[1], "│ This is a very l │");
}

#[test]
fn test_card_unicode_in_title() {
    let unicode_title = "🎉 Celebration Card 🎊";
    let c = Card::new().title(unicode_title);
    // Each emoji takes two columns
    assert_eq!(render_rows(&c, 30, 10)[1], "│ 🎉 Celebration Card 🎊     │");
}

#[test]
fn test_card_wide_title_clipped_before_half_a_char() {
    // A wide character that would straddle the edge is left out
    let c = Card::new().title("ab🎉");
    assert_eq!(render_rows(&c, 7, 3)[1], "│ ab  │");
}

#[test]
fn test_card_multiple_chained_builders() {
    let c = Card::new()
        .title("Complex Card")
        .subtitle("With many options")
        .collapsible(true)
        .clickable(true)
        .padding(2)
        .elevated()
        .rounded()
        .background(Color::rgb(40, 40, 45))
        .border_color(Color::CYAN)
        .title_color(Color::YELLOW)
        .body(Text::new("Body"))
        .footer(Text::new("Footer"));

    assert!(c.is_collapsible());

    let mut buffer = Buffer::new(30, 12);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    // Should render with rounded corners and shadow
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
}

#[test]
fn test_card_disabled_keyboard() {
    let mut c = Card::new().collapsible(true).disabled(true);
    assert!(c.is_disabled());

    // Should not handle keys when disabled
    assert!(!c.handle_key(&Key::Enter));
    assert!(!c.handle_key(&Key::Char(' ')));
}

#[test]
fn test_card_focused_state() {
    let c = Card::new().focused(true);
    // Focus state is set via builder
    // The widget can be focused in the context of the application
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    c.render(&mut ctx);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_card_disabled_state() {
    let c = Card::new().disabled(true);
    assert!(c.is_disabled());
}

#[test]
fn test_card_with_only_header() {
    let c = Card::new().header(Text::new("Only header"));
    let rows = render_rows(&c, 20, 10);
    assert_eq!(rows[1], "│ Only header      │");
    // No separator without a body
    assert_eq!(rows[2], "│                  │");
}

#[test]
fn test_card_with_only_footer() {
    let c = Card::new().footer(Text::new("Only footer"));
    let rows = render_rows(&c, 20, 10);
    assert_eq!(rows[7], "├──────────────────┤");
    assert_eq!(rows[8], "│ Only footer      │");
}

#[test]
fn test_card_without_border_padding() {
    let c = Card::new().flat().padding(0).title("No padding");
    assert_eq!(render_rows(&c, 20, 10)[0], "No padding          ");
}

#[test]
fn test_card_large_padding() {
    let c = Card::new()
        .padding(5)
        .title("Large padding")
        .body(Text::new("Content"));
    let rows = render_rows(&c, 40, 20);
    assert!(rows[1].starts_with("│     Large padding "), "{:?}", rows[1]);
    assert!(rows[2].starts_with('├'));
    assert!(rows[3].starts_with("│     Content "), "{:?}", rows[3]);
}
