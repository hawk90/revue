//! Border widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::layout::border::{border, Border, BorderType};
use revue::widget::traits::RenderContext;
use revue::widget::Text;

// =========================================================================
// BorderType enum tests
// =========================================================================

#[test]
fn test_border_type_default() {
    assert_eq!(BorderType::default(), BorderType::Single);
}

#[test]
fn test_border_type_clone() {
    let bt = BorderType::Double;
    assert_eq!(bt, bt.clone());
}

#[test]
fn test_border_type_copy() {
    let bt1 = BorderType::Rounded;
    let bt2 = bt1;
    assert_eq!(bt1, BorderType::Rounded);
    assert_eq!(bt2, BorderType::Rounded);
}

#[test]
fn test_border_type_partial_eq() {
    assert_eq!(BorderType::Single, BorderType::Single);
    assert_eq!(BorderType::Double, BorderType::Double);
    assert_eq!(BorderType::Rounded, BorderType::Rounded);
    assert_eq!(BorderType::Thick, BorderType::Thick);
    assert_eq!(BorderType::Ascii, BorderType::Ascii);
    assert_eq!(BorderType::None, BorderType::None);

    assert_ne!(BorderType::Single, BorderType::Double);
    assert_ne!(BorderType::Rounded, BorderType::None);
}

#[test]
fn test_border_type_all_variants_unique() {
    let variants = [
        BorderType::None,
        BorderType::Single,
        BorderType::Double,
        BorderType::Rounded,
        BorderType::Thick,
        BorderType::Ascii,
    ];

    // All variants should be different from None
    for variant in variants.iter().skip(1) {
        assert_ne!(*variant, BorderType::None);
    }

    // All variants should be different from Single
    for variant in variants.iter() {
        if *variant != BorderType::Single {
            assert_ne!(*variant, BorderType::Single);
        }
    }
}

#[test]
fn test_border_type_debug() {
    let debug_str = format!("{:?}", BorderType::Single);
    assert!(debug_str.contains("Single"));
}

// =========================================================================
// BorderType::chars tests
// =========================================================================

#[test]
fn test_border_type_chars_none() {
    let chars = BorderType::None.chars();
    assert_eq!(chars.top_left, ' ');
    assert_eq!(chars.top_right, ' ');
    assert_eq!(chars.bottom_left, ' ');
    assert_eq!(chars.bottom_right, ' ');
    assert_eq!(chars.horizontal, ' ');
    assert_eq!(chars.vertical, ' ');
}

#[test]
fn test_border_type_chars_single() {
    let chars = BorderType::Single.chars();
    assert_eq!(chars.top_left, '┌');
    assert_eq!(chars.top_right, '┐');
    assert_eq!(chars.bottom_left, '└');
    assert_eq!(chars.bottom_right, '┘');
    assert_eq!(chars.horizontal, '─');
    assert_eq!(chars.vertical, '│');
}

#[test]
fn test_border_type_chars_double() {
    let chars = BorderType::Double.chars();
    assert_eq!(chars.top_left, '╔');
    assert_eq!(chars.top_right, '╗');
    assert_eq!(chars.bottom_left, '╚');
    assert_eq!(chars.bottom_right, '╝');
    assert_eq!(chars.horizontal, '═');
    assert_eq!(chars.vertical, '║');
}

#[test]
fn test_border_type_chars_rounded() {
    let chars = BorderType::Rounded.chars();
    assert_eq!(chars.top_left, '╭');
    assert_eq!(chars.top_right, '╮');
    assert_eq!(chars.bottom_left, '╰');
    assert_eq!(chars.bottom_right, '╯');
    assert_eq!(chars.horizontal, '─');
    assert_eq!(chars.vertical, '│');
}

#[test]
fn test_border_type_chars_thick() {
    let chars = BorderType::Thick.chars();
    assert_eq!(chars.top_left, '┏');
    assert_eq!(chars.top_right, '┓');
    assert_eq!(chars.bottom_left, '┗');
    assert_eq!(chars.bottom_right, '┛');
    assert_eq!(chars.horizontal, '━');
    assert_eq!(chars.vertical, '┃');
}

#[test]
fn test_border_type_chars_ascii() {
    let chars = BorderType::Ascii.chars();
    assert_eq!(chars.top_left, '+');
    assert_eq!(chars.top_right, '+');
    assert_eq!(chars.bottom_left, '+');
    assert_eq!(chars.bottom_right, '+');
    assert_eq!(chars.horizontal, '-');
    assert_eq!(chars.vertical, '|');
}

// =========================================================================
// Border::new and default tests
// =========================================================================

#[test]
fn test_border_new() {
    let b = Border::new();
    assert_eq!(b.get_border_type(), BorderType::Single);
    assert!(b.get_title().is_none());
    assert!(b.get_fg().is_none());
    assert!(b.get_bg().is_none());
}

#[test]
fn test_border_default() {
    let b = Border::default();
    assert_eq!(b.get_border_type(), BorderType::Single);
}

// =========================================================================
// Border builder tests
// =========================================================================

#[test]
fn test_border_child() {
    let b = Border::new().child(Text::new("Hi"));
    assert!(b.has_child());
}

#[test]
fn test_border_get_border_type() {
    let b = Border::new().border_type(BorderType::Double);
    assert_eq!(b.get_border_type(), BorderType::Double);
}

#[test]
fn test_border_title_str() {
    let b = Border::new().title("Test Title");
    assert_eq!(b.get_title(), Some("Test Title"));
}

#[test]
fn test_border_title_string() {
    let b = Border::new().title(String::from("Owned"));
    assert_eq!(b.get_title(), Some("Owned"));
}

#[test]
fn test_border_title_empty() {
    let b = Border::new().title("");
    assert_eq!(b.get_title(), Some(""));
}

#[test]
fn test_border_fg() {
    let b = Border::new().fg(Color::RED);
    assert_eq!(b.get_fg(), Some(Color::RED));
}

#[test]
fn test_border_bg() {
    let b = Border::new().bg(Color::BLUE);
    assert_eq!(b.get_bg(), Some(Color::BLUE));
}

#[test]
fn test_border_builder_chain() {
    let b = Border::new()
        .child(Text::new("Content"))
        .border_type(BorderType::Rounded)
        .title("Panel")
        .fg(Color::CYAN)
        .bg(Color::BLACK);

    assert!(b.has_child());
    assert_eq!(b.get_border_type(), BorderType::Rounded);
    assert_eq!(b.get_title(), Some("Panel"));
    assert_eq!(b.get_fg(), Some(Color::CYAN));
    assert_eq!(b.get_bg(), Some(Color::BLACK));
}

// =========================================================================
// Border preset tests
// =========================================================================

#[test]
fn test_border_types() {
    assert_eq!(Border::single().get_border_type(), BorderType::Single);
    assert_eq!(Border::double().get_border_type(), BorderType::Double);
    assert_eq!(Border::rounded().get_border_type(), BorderType::Rounded);
}

#[test]
fn test_border_thick() {
    let b = Border::thick();
    assert_eq!(b.get_border_type(), BorderType::Thick);
}

#[test]
fn test_border_ascii() {
    let b = Border::ascii();
    assert_eq!(b.get_border_type(), BorderType::Ascii);
}

#[test]
fn test_border_panel() {
    let b = Border::panel();
    assert_eq!(b.get_border_type(), BorderType::Double);
    assert_eq!(b.get_fg(), Some(Color::CYAN));
}

#[test]
fn test_border_card() {
    let b = Border::card();
    assert_eq!(b.get_border_type(), BorderType::Rounded);
    assert_eq!(b.get_fg(), Some(Color::WHITE));
}

#[test]
fn test_border_error_box() {
    let b = Border::error_box();
    assert_eq!(b.get_border_type(), BorderType::Single);
    assert_eq!(b.get_fg(), Some(Color::RED));
}

#[test]
fn test_border_success_box() {
    let b = Border::success_box();
    assert_eq!(b.get_border_type(), BorderType::Single);
    assert_eq!(b.get_fg(), Some(Color::GREEN));
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_border_helper() {
    let b = border();
    assert_eq!(b.get_border_type(), BorderType::Single);
}

// =========================================================================
// Border rendering tests
// =========================================================================

#[test]
fn test_border_render_single() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '│');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '─');
}

#[test]
fn test_border_render_double() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::double();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╗');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '╚');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '╝');
}

#[test]
fn test_border_render_thick() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::thick();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┏');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '┓');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '┗');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '┛');
}

#[test]
fn test_border_render_ascii() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::ascii();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '+');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '+');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '-');
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '|');
}

#[test]
fn test_border_render_none() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::new().border_type(BorderType::None);
    b.render(&mut ctx);

    // No border should be drawn
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_border_render_rounded() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::rounded();
    b.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╭');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '╮');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '╰');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '╯');
}

#[test]
fn test_border_render_with_color() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().fg(Color::RED);
    b.render(&mut ctx);

    // Check color was applied
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_border_render_with_bg_color() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().bg(Color::BLUE);
    b.render(&mut ctx);

    // Check background was applied
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_border_render_small_width() {
    let mut buffer = Buffer::new(2, 5);
    let area = Rect::new(0, 0, 2, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single();
    b.render(&mut ctx);
    // Should not panic even with minimum width
}

#[test]
fn test_border_render_small_height() {
    let mut buffer = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single();
    b.render(&mut ctx);
    // Should not panic even with minimum height
}

#[test]
fn test_border_render_too_small() {
    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single();
    b.render(&mut ctx);
    // Should not render anything when too small
}

// =========================================================================
// Border title tests
// =========================================================================

#[test]
fn test_border_with_title() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().title("Test");
    b.render(&mut ctx);

    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'T');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 's');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 't');
}

#[test]
fn test_border_with_title_truncated() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // Title longer than available space (width 10 - 4 = 6 max chars)
    let b = Border::single().title("Very Long Title");
    b.render(&mut ctx);

    // Title should be truncated
    assert_eq!(buffer.get(2, 0).unwrap().symbol, 'V');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, 'e');
    assert_eq!(buffer.get(4, 0).unwrap().symbol, 'r');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 'y');
    assert_eq!(buffer.get(6, 0).unwrap().symbol, ' ');
    assert_eq!(buffer.get(7, 0).unwrap().symbol, 'L');
}

#[test]
fn test_border_with_title_color() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().title("Test").fg(Color::YELLOW);
    b.render(&mut ctx);

    // Title should have the border color
    assert_eq!(buffer.get(2, 0).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_border_with_title_unicode() {
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().title("🔧 Settings");
    b.render(&mut ctx);

    // Unicode title should render
    let char_at_2 = buffer.get(2, 0).unwrap().symbol;
    assert!(char_at_2 == '🔧' || char_at_2 == 'S'); // May vary by terminal
}

// =========================================================================
// Border child tests
// =========================================================================

#[test]
fn test_border_with_child() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().child(Text::new("Hi"));
    b.render(&mut ctx);

    // Child rendered at (1, 1) inside border
    assert_eq!(buffer.get(1, 1).unwrap().symbol, 'H');
    assert_eq!(buffer.get(2, 1).unwrap().symbol, 'i');
}

#[test]
fn test_border_with_child_no_child() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single();
    b.render(&mut ctx);

    // Inside area should be empty (no child)
    assert_eq!(buffer.get(1, 1).unwrap().symbol, ' ');
}

#[test]
fn test_border_child_area() {
    let mut buffer = Buffer::new(12, 8);
    let area = Rect::new(0, 0, 12, 8);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single().child(Text::new("X"));
    b.render(&mut ctx);

    // Child should be rendered with 1 cell padding on all sides
    assert_eq!(buffer.get(1, 1).unwrap().symbol, 'X');
    // Border corners should still be correct
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(11, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 7).unwrap().symbol, '└');
    assert_eq!(buffer.get(11, 7).unwrap().symbol, '┘');
}

// =========================================================================
// draw_border utility tests
// =========================================================================

#[test]
fn test_draw_border_utility() {
    use revue::widget::layout::border::draw_border;

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);

    draw_border(&mut buffer, area, BorderType::Single, None, None);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(9, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 4).unwrap().symbol, '└');
    assert_eq!(buffer.get(9, 4).unwrap().symbol, '┘');
}

#[test]
fn test_draw_border_none_type() {
    use revue::widget::layout::border::draw_border;

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);

    draw_border(&mut buffer, area, BorderType::None, None, None);

    // Border should not be drawn
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_border_with_color() {
    use revue::widget::layout::border::draw_border;

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);

    draw_border(
        &mut buffer,
        area,
        BorderType::Single,
        Some(Color::RED),
        None,
    );

    // Check color was applied
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(5, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_draw_border_with_bg_color() {
    use revue::widget::layout::border::draw_border;

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);

    draw_border(
        &mut buffer,
        area,
        BorderType::Single,
        None,
        Some(Color::BLUE),
    );

    // Check background was applied
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_draw_border_small_area() {
    use revue::widget::layout::border::draw_border;

    let mut buffer = Buffer::new(2, 2);
    let area = Rect::new(0, 0, 2, 2);

    draw_border(&mut buffer, area, BorderType::Single, None, None);

    // Should draw corners only
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
    assert_eq!(buffer.get(1, 0).unwrap().symbol, '┐');
    assert_eq!(buffer.get(0, 1).unwrap().symbol, '└');
    assert_eq!(buffer.get(1, 1).unwrap().symbol, '┘');
}

#[test]
fn test_draw_border_too_small() {
    use revue::widget::layout::border::draw_border;

    let mut buffer = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 1, 1);

    draw_border(&mut buffer, area, BorderType::Single, None, None);

    // Should not draw anything
    assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
}

#[test]
fn test_draw_border_all_types() {
    use revue::widget::layout::border::draw_border;

    let types = [
        BorderType::Single,
        BorderType::Double,
        BorderType::Rounded,
        BorderType::Thick,
        BorderType::Ascii,
    ];

    for bt in types {
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);

        draw_border(&mut buffer, area, bt, None, None);

        // All types except None should draw borders
        assert_ne!(buffer.get(0, 0).unwrap().symbol, ' ');
        assert_ne!(buffer.get(9, 0).unwrap().symbol, ' ');
    }
}

// =========================================================================
// Border edge case tests
// =========================================================================

#[test]
fn test_border_offset_area() {
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(5, 3, 10, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::single();
    b.render(&mut ctx);

    // Border should be at offset position
    assert_eq!(buffer.get(5, 3).unwrap().symbol, '┌');
    assert_eq!(buffer.get(14, 3).unwrap().symbol, '┐');
    assert_eq!(buffer.get(5, 7).unwrap().symbol, '└');
    assert_eq!(buffer.get(14, 7).unwrap().symbol, '┘');
}

#[test]
fn test_border_large_area() {
    let mut buffer = Buffer::new(100, 50);
    let area = Rect::new(0, 0, 100, 50);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let b = Border::double();
    b.render(&mut ctx);

    // Check corners on large area
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '╔');
    assert_eq!(buffer.get(99, 0).unwrap().symbol, '╗');
    assert_eq!(buffer.get(0, 49).unwrap().symbol, '╚');
    assert_eq!(buffer.get(99, 49).unwrap().symbol, '╝');
}
