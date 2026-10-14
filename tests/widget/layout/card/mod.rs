//! Card widget integration tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::LIGHT_GRAY;
use revue::widget::traits::RenderContext;
use revue::widget::Text;
use revue::widget::{card, BorderType, Card, CardVariant, StyledView, View};

mod view;

/// Render `c` into a `width` x `height` buffer.
fn render_buffer(c: &Card, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    c.render(&mut ctx);
    buffer
}

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
    let buffer = render_buffer(&c, 20, 10);
    assert_eq!(render_rows(&c, 20, 10)[1], "│ Test Title       │");
    // Bold and white by default
    let cell = buffer.get(2, 1).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_card_title_str() {
    let title = "제목입니다"; // Korean title test
    let c = Card::new().title(title);
    // Five wide characters take ten columns
    assert_eq!(render_rows(&c, 20, 10)[1], "│ 제목입니다       │");
    let buffer = render_buffer(&c, 20, 10);
    assert!(buffer.get(3, 1).unwrap().is_continuation());
}

#[test]
fn test_card_subtitle() {
    let c = Card::new().subtitle("Subtitle");
    // Without a title the subtitle takes the first row, gray and not bold
    assert_eq!(render_rows(&c, 20, 10)[1], "│ Subtitle         │");
    let cell = *render_buffer(&c, 20, 10).get(2, 1).unwrap();
    assert_eq!(cell.fg, Some(LIGHT_GRAY));
    assert!(!cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_card_subtitle_korean() {
    let subtitle = "부제목입니다"; // Korean subtitle test
    let c = Card::new().subtitle(subtitle);
    assert_eq!(render_rows(&c, 20, 10)[1], "│ 부제목입니다     │");
}

#[test]
fn test_card_title_and_subtitle() {
    let c = Card::new().title("Main Title").subtitle("Description");
    let rows = render_rows(&c, 20, 10);
    assert_eq!(rows[1], "│ Main Title       │");
    assert_eq!(rows[2], "│ Description      │");
    // No separator without a body
    assert_eq!(rows[3], "│                  │");
}

#[test]
fn test_card_body() {
    let body = Text::new("Body content");
    let c = Card::new().body(body);
    let rows = render_rows(&c, 20, 10);
    // Nothing above the body, so no separator
    assert_eq!(rows[1], "│ Body content     │");
    assert_eq!(rows[2], "│                  │");
}

#[test]
fn test_card_header() {
    let header = Text::new("Header content");
    let c = Card::new().header(header);
    assert_eq!(render_rows(&c, 20, 10)[1], "│ Header content   │");
}

#[test]
fn test_card_footer() {
    let footer = Text::new("Footer content");
    let c = Card::new().footer(footer);
    let rows = render_rows(&c, 20, 10);
    assert_eq!(rows[7], "├──────────────────┤");
    assert_eq!(rows[8], "│ Footer content   │");
    assert_eq!(rows[9], "└──────────────────┘");
}

#[test]
fn test_card_all_sections() {
    let c = Card::new()
        .title("Title")
        .subtitle("Subtitle")
        .header(Text::new("Header"))
        .body(Text::new("Body"))
        .footer(Text::new("Footer"));
    assert_eq!(
        render_rows(&c, 30, 10),
        [
            "┌────────────────────────────┐",
            "│ Title                      │",
            "│ Subtitle                   │",
            "│ Header                     │",
            "├────────────────────────────┤",
            "│ Body                       │",
            "│                            │",
            "├────────────────────────────┤",
            "│ Footer                     │",
            "└────────────────────────────┘",
        ]
    );
}

// =============================================================================
// Variant Tests
// 변형 테스트
// =============================================================================

#[test]
fn test_card_variant_outlined() {
    let c = Card::new().outlined();
    let buffer = render_buffer(&c, 20, 10);
    // A gray single border and no background
    for (x, y) in [(0, 0), (5, 0), (0, 5), (19, 9)] {
        assert_eq!(buffer.get(x, y).unwrap().fg, Some(Color::rgb(60, 60, 70)));
    }
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(5, 5).unwrap().bg, None);
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
    let c = Card::new().flat().title("Flat").body(Text::new("Body"));
    let rows = render_rows(&c, 12, 4);
    // No frame: the title starts after the padding on the first row, and the
    // separator is a plain line across the card
    assert_eq!(
        rows,
        [
            " Flat       ",
            "────────────",
            " Body       ",
            "            "
        ]
    );
    let buffer = render_buffer(&c, 12, 4);
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::rgb(50, 50, 50)));
    assert_eq!(buffer.get(5, 2).unwrap().bg, None);
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
        // Flat is documented as borderless, and variant(Flat) removes it (#693)
        (CardVariant::Flat, ' ', None),
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
    let rows = |c: &Card| render_rows(c, 10, 4);
    let blank = vec!["          "; 4];
    // A border chosen before `flat` is removed, by either builder
    assert_eq!(rows(&Card::new().rounded().flat()), blank);
    assert_eq!(
        rows(&Card::new().rounded().variant(CardVariant::Flat)),
        blank
    );
    // and measure no longer counts the frame's two rows
    let content = |c: Card| c.title("T").subtitle("S").body(Text::new("B"));
    assert_eq!(content(Card::new()).measure(10, 20), Some((10, 6)));
    assert_eq!(content(Card::new().flat()).measure(10, 20), Some((10, 4)));
    // A border chosen after `flat` is drawn
    assert_eq!(
        rows(&Card::new().flat().border_style(BorderType::Double))[0],
        "╔════════╗"
    );
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
    let buffer = render_buffer(&c, 20, 10);
    for x in 2..9 {
        assert_eq!(buffer.get(x, 1).unwrap().fg, Some(Color::GREEN), "{x}");
    }
    // The border keeps its own color
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(60, 60, 70)));
}

#[test]
fn test_card_all_colors() {
    let c = Card::new()
        .title("Title")
        .background(Color::rgb(30, 30, 35))
        .border_color(Color::rgb(60, 60, 70))
        .title_color(Color::WHITE);
    // Through a stack, as under an app's root: the background claimed by the
    // card also goes under the cells drawn on it
    let mut buffer = Buffer::new(20, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 20, 10));
    revue::widget::vstack().child_sized(c, 10).render(&mut ctx);
    let corner = buffer.get(0, 0).unwrap();
    assert_eq!(corner.symbol, '┌');
    assert_eq!(corner.fg, Some(Color::rgb(60, 60, 70)));
    assert_eq!(buffer.get(2, 1).unwrap().fg, Some(Color::WHITE));
    // The background fills the whole card, border and title included
    for (x, y) in [(0, 0), (2, 1), (10, 5), (19, 9)] {
        assert_eq!(buffer.get(x, y).unwrap().bg, Some(Color::rgb(30, 30, 35)));
    }
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
    let c = Card::new()
        .padding(2)
        .title("Title")
        .body(Text::new("Clipped at ten"));
    let rows = render_rows(&c, 30, 10);
    // Two columns each side inside the border
    assert_eq!(rows[1], "│  Title                     │");
    assert_eq!(rows[3], "│  Clipped at ten            │");
    let rows = render_rows(&c, 16, 10);
    assert_eq!(rows[3], "│  Clipped at  │");
}

#[test]
fn test_card_padding_zero() {
    let c = Card::new()
        .padding(0)
        .title("Title")
        .body(Text::new("Body"));
    let rows = render_rows(&c, 30, 10);
    assert_eq!(rows[1], "│Title                       │");
    assert_eq!(rows[3], "│Body                        │");
}

#[test]
fn test_card_padding_large() {
    let c = Card::new()
        .padding(10)
        .title("Title")
        .body(Text::new("Content that runs past the padding"));
    let rows = render_rows(&c, 50, 20);
    assert_eq!(
        rows[1],
        format!("│{}Title{}│", " ".repeat(10), " ".repeat(33))
    );
    // 28 columns between the paddings
    assert_eq!(
        rows[3],
        format!(
            "│{}Content that runs past the p{}│",
            " ".repeat(10),
            " ".repeat(10)
        )
    );
}

// =============================================================================
// Clickable Tests
// 클릭 가능 테스트
// =============================================================================

#[test]
fn test_card_clickable() {
    // A focused clickable card shows its focus with a cyan frame and
    // separator
    let c = Card::new()
        .clickable(true)
        .focused(true)
        .title("T")
        .body(Text::new("B"));
    let buffer = render_buffer(&c, 20, 10);
    for (x, y) in [(0, 0), (19, 0), (0, 5), (0, 2), (5, 2), (19, 9)] {
        assert_eq!(
            buffer.get(x, y).unwrap().fg,
            Some(Color::CYAN),
            "({x}, {y})"
        );
    }
    // Unfocused, the frame is the default gray
    let c = Card::new().clickable(true).title("T");
    assert_eq!(
        render_buffer(&c, 20, 10).get(0, 0).unwrap().fg,
        Some(Color::rgb(60, 60, 70))
    );
}

#[test]
fn test_card_not_clickable_default() {
    // Focus shows only on a clickable card, and a card is not clickable by
    // default
    let c = Card::new().focused(true);
    let buffer = render_buffer(&c, 20, 10);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::rgb(60, 60, 70)));
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
    // The title is bold and white, the subtitle below it gray and plain
    let title = buffer.get(2, 1).unwrap();
    assert_eq!((title.symbol, title.fg), ('T', Some(Color::WHITE)));
    assert!(title.modifier.contains(Modifier::BOLD));
    let subtitle = buffer.get(2, 2).unwrap();
    assert_eq!((subtitle.symbol, subtitle.fg), ('S', Some(LIGHT_GRAY)));
    assert!(!subtitle.modifier.contains(Modifier::BOLD));
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
fn test_card_render_elevated_variant() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new().elevated().title("Elevated");
    c.render(&mut ctx);

    // Shadow on the right and bottom edges, inside the area (#495)
    assert_eq!(buffer.get(39, 5).unwrap().symbol, '▌');
    assert_eq!(buffer.get(39, 5).unwrap().fg, Some(Color::rgb(20, 20, 20)));
    assert_eq!(buffer.get(5, 9).unwrap().symbol, '▀');
    // The card's border sits just inside the shadow
    assert_eq!(buffer.get(38, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(38, 5).unwrap().symbol, '│');
    assert_eq!(buffer.get(0, 8).unwrap().symbol, '└');
    assert_eq!(buffer.get(38, 8).unwrap().symbol, '┘');
    // Nothing beside the top row or under the left edge
    assert_ne!(buffer.get(39, 0).unwrap().symbol, '▌');
    assert_ne!(buffer.get(0, 9).unwrap().symbol, '▀');
}

#[test]
fn test_card_render_elevated_footer_stays_inside_shadow() {
    let mut buffer = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let c = Card::new()
        .elevated()
        .title("T")
        .body(Text::new("Body"))
        .footer(Text::new("Foot"));
    c.render(&mut ctx);

    // Footer on the row above the card's bottom border, which is above the shadow
    let footer_row: String = (0..30).map(|x| buffer.get(x, 7).unwrap().symbol).collect();
    assert!(footer_row.contains("Foot"), "{footer_row:?}");
    assert_eq!(buffer.get(0, 8).unwrap().symbol, '└');
    assert_eq!(buffer.get(5, 9).unwrap().symbol, '▀');
    assert_eq!(buffer.get(29, 7).unwrap().symbol, '▌');
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
    assert_eq!(buffer.get(19, 9).unwrap().fg, Some(Color::RED));
    // and the title's
    assert_eq!(buffer.get(2, 1).unwrap().fg, Some(Color::GREEN));
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
    // An empty title still takes its row: the separator comes below it
    let c = Card::new().title("").body(Text::new("Body"));
    let rows = render_rows(&c, 20, 10);
    assert_eq!(rows[1], "│                  │");
    assert_eq!(rows[2], "├──────────────────┤");
    assert_eq!(rows[3], "│ Body             │");
    assert_eq!(c.measure(20, 10), Some((20, 5)));
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
    let row = |y: u16| -> String { (0..30).map(|x| buffer.get(x, y).unwrap().symbol).collect() };
    assert_eq!(row(0), "╭───────────────────────────╮ ");
    assert_eq!(row(1), "│  ▼ Complex Card           │▌");
    assert_eq!(row(2), "│  With many options        │▌");
    assert_eq!(row(3), "├───────────────────────────┤▌");
    assert_eq!(row(4), "│  Body                     │▌");
    assert_eq!(row(6), "├───────────────────────────┤▌");
    assert_eq!(row(7), "│  Footer                   │▌");
    assert_eq!(row(8), "╰───────────────────────────╯▌");
    assert_eq!(row(9), " ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀");
    // Nothing below the area
    assert_eq!(row(10).trim(), "");
    let at = |x, y| *buffer.get(x, y).unwrap();
    assert_eq!(at(0, 0).fg, Some(Color::CYAN));
    assert_eq!(at(5, 1).fg, Some(Color::YELLOW));
    assert_eq!(at(10, 5).bg, Some(Color::rgb(40, 40, 45)));
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
    assert!(c.is_focused());
    assert!(!Card::new().is_focused());
    // Focus changes only the frame: a clickable card's title keeps its color
    let c = Card::new().clickable(true).focused(true).title("T");
    let buffer = render_buffer(&c, 20, 10);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(2, 1).unwrap().fg, Some(Color::WHITE));
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
