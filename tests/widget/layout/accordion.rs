//! Accordion widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{accordion, section, Accordion, AccordionSection, View};

// ==================== Constructor Tests ====================

#[test]
fn test_accordion_new() {
    let acc = Accordion::new();
    assert!(acc.is_empty());
    assert_eq!(acc.selected(), 0);
}

#[test]
fn test_accordion_default() {
    let acc = Accordion::default();
    assert!(acc.is_empty());
    assert_eq!(acc.selected(), 0);
}

#[test]
fn test_accordion_helper() {
    let acc = accordion().section(section("Test"));
    assert_eq!(acc.len(), 1);
}

// ==================== Builder Tests ====================

#[test]
fn test_accordion_sections() {
    let acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert_eq!(acc.len(), 2);
}

#[test]
fn test_accordion_sections_batch() {
    let sections = vec![
        AccordionSection::new("A"),
        AccordionSection::new("B"),
        AccordionSection::new("C"),
    ];
    let acc = Accordion::new().sections(sections);
    assert_eq!(acc.len(), 3);
}

#[test]
fn test_accordion_multi_expand() {
    let _acc = Accordion::new().multi_expand(true);
    // Private field - just verify it compiles
}

#[test]
fn test_accordion_header_colors() {
    let _acc = Accordion::new().header_colors(Color::WHITE, Color::BLUE);
    // Private fields - just verify it compiles
}

#[test]
fn test_accordion_selected_bg() {
    let _acc = Accordion::new().selected_bg(Color::GREEN);
    // Private field - just verify it compiles
}

#[test]
fn test_accordion_content_colors() {
    let _acc = Accordion::new().content_colors(Color::YELLOW, Color::RED);
    // Private fields - just verify it compiles
}

#[test]
fn test_accordion_dividers() {
    let _acc = Accordion::new().dividers(false);
    // Private field - just verify it compiles
}

#[test]
fn test_accordion_border() {
    let _acc = Accordion::new().border(Color::CYAN);
    // Private field - just verify it compiles
}

#[test]
fn test_accordion_builder_chain() {
    let _acc = Accordion::new()
        .multi_expand(true)
        .header_colors(Color::WHITE, Color::BLUE)
        .selected_bg(Color::GREEN)
        .content_colors(Color::YELLOW, Color::RED)
        .dividers(false)
        .border(Color::CYAN);
    // Just verify it compiles
}

// ==================== Selection Tests ====================

#[test]
fn test_accordion_selection() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"))
        .section(AccordionSection::new("C"));

    assert_eq!(acc.selected(), 0);

    acc.select_next();
    assert_eq!(acc.selected(), 1);

    acc.select_next();
    assert_eq!(acc.selected(), 2);

    acc.select_next();
    assert_eq!(acc.selected(), 0); // Wrap

    acc.select_prev();
    assert_eq!(acc.selected(), 2); // Wrap back
}

#[test]
fn test_accordion_select_next_wrap() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.set_selected(1);
    acc.select_next();
    assert_eq!(acc.selected(), 0); // Wraps to first
}

#[test]
fn test_accordion_select_prev_wrap() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.select_prev();
    assert_eq!(acc.selected(), 1); // Wraps to last
}

#[test]
fn test_accordion_set_selected() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.set_selected(1);
    assert_eq!(acc.selected(), 1);
}

#[test]
fn test_accordion_selection_empty() {
    let mut acc = Accordion::new();
    acc.select_next(); // Should not panic
    acc.select_prev(); // Should not panic
    assert_eq!(acc.selected(), 0);
}

// ==================== Expansion/Collapse Tests ====================

#[test]
fn test_accordion_toggle_selected() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    acc.toggle_selected();
    // Section should now be expanded
    acc.toggle_selected();
    // Section should now be collapsed
}

#[test]
fn test_accordion_expand_selected() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    acc.expand_selected();
    // Should expand the selected section
}

#[test]
fn test_accordion_collapse_selected() {
    let mut acc =
        Accordion::new().section(AccordionSection::new("A").line("Content").expanded(true));

    acc.collapse_selected();
    // Should collapse the selected section
}

#[test]
fn test_accordion_expand_all() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_all();
    // All sections should be expanded
}

#[test]
fn test_accordion_collapse_all() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A").expanded(true))
        .section(AccordionSection::new("B").expanded(true));

    acc.collapse_all();
    // All sections should be collapsed
}

#[test]
fn test_accordion_single_expand_mode() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_selected();
    acc.set_selected(1);
    acc.expand_selected();
    // In single-expand mode, expanding one should collapse others
}

#[test]
fn test_accordion_multi_expand_mode() {
    let mut acc = Accordion::new()
        .multi_expand(true)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_selected();
    acc.set_selected(1);
    acc.expand_selected();
    // In multi-expand mode, both can be expanded
}

// ==================== Key Handling Tests ====================

#[test]
fn test_accordion_handle_key_up() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"))
        .section(AccordionSection::new("C"));

    acc.set_selected(2);
    assert!(acc.handle_key(&Key::Up));
    assert_eq!(acc.selected(), 1);
}

#[test]
fn test_accordion_handle_key_down() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert!(acc.handle_key(&Key::Down));
    assert_eq!(acc.selected(), 1);
}

#[test]
fn test_accordion_handle_key_j() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert!(acc.handle_key(&Key::Char('j')));
    assert_eq!(acc.selected(), 1);
}

#[test]
fn test_accordion_handle_key_k() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"))
        .section(AccordionSection::new("C"));

    acc.set_selected(2);
    assert!(acc.handle_key(&Key::Char('k')));
    assert_eq!(acc.selected(), 1);
}

#[test]
fn test_accordion_handle_key_enter() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Enter));
    // Should toggle the selected section
}

#[test]
fn test_accordion_handle_key_space() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Char(' ')));
    // Should toggle the selected section
}

#[test]
fn test_accordion_handle_key_right() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Right));
    // Should expand the selected section
}

#[test]
fn test_accordion_handle_key_left() {
    let mut acc =
        Accordion::new().section(AccordionSection::new("A").line("Content").expanded(true));

    assert!(acc.handle_key(&Key::Left));
    // Should collapse the selected section
}

#[test]
fn test_accordion_handle_key_l() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Char('l')));
    // Should expand the selected section
}

#[test]
fn test_accordion_handle_key_h() {
    let mut acc =
        Accordion::new().section(AccordionSection::new("A").line("Content").expanded(true));

    assert!(acc.handle_key(&Key::Char('h')));
    // Should collapse the selected section
}

#[test]
fn test_accordion_handle_key_invalid() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert!(!acc.handle_key(&Key::Tab));
    assert!(!acc.handle_key(&Key::Char('x')));
    assert!(!acc.handle_key(&Key::Char('z')));
    assert_eq!(acc.selected(), 0); // Should not change
}

// ==================== AccordionSection Tests ====================

#[test]
fn test_accordion_section_new() {
    let sec = AccordionSection::new("Title");
    assert_eq!(sec.title, "Title");
    assert!(sec.content.is_empty());
    assert!(!sec.expanded);
}

#[test]
fn test_accordion_section_line() {
    let sec = AccordionSection::new("Title").line("Line 1").line("Line 2");
    assert_eq!(sec.content.len(), 2);
}

#[test]
fn test_accordion_section_lines() {
    let sec = AccordionSection::new("Title").lines(&["A", "B", "C"]);
    assert_eq!(sec.content.len(), 3);
}

#[test]
fn test_accordion_section_content() {
    let sec = AccordionSection::new("Title").content("Line 1\nLine 2\nLine 3");
    assert_eq!(sec.content.len(), 3);
}

#[test]
fn test_accordion_section_expanded() {
    let sec = AccordionSection::new("Title").expanded(true);
    assert!(sec.expanded);
}

#[test]
fn test_accordion_section_icons() {
    let sec = AccordionSection::new("Title").icons('+', '-');
    assert_eq!(sec.collapsed_icon, '+');
    assert_eq!(sec.expanded_icon, '-');
}

#[test]
fn test_accordion_section_builder_chain() {
    let sec = AccordionSection::new("Title")
        .line("Line 1")
        .lines(&["A", "B"])
        .content("More\nContent")
        .expanded(true)
        .icons('+', '-');
    assert_eq!(sec.title, "Title");
    assert!(sec.expanded);
}

// ==================== Management Tests ====================

#[test]
fn test_accordion_add_remove() {
    let mut acc = Accordion::new();

    acc.add_section(AccordionSection::new("A"));
    acc.add_section(AccordionSection::new("B"));
    assert_eq!(acc.len(), 2);

    let removed = acc.remove_section(0);
    assert!(removed.is_some());
    assert_eq!(acc.len(), 1);
}

#[test]
fn test_accordion_remove_section_out_of_range() {
    let mut acc = Accordion::new().section(AccordionSection::new("A"));

    let removed = acc.remove_section(10);
    assert!(removed.is_none());
    assert_eq!(acc.len(), 1);
}

#[test]
fn test_accordion_len() {
    let acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"))
        .section(AccordionSection::new("C"));
    assert_eq!(acc.len(), 3);
}

#[test]
fn test_accordion_is_empty() {
    let acc = Accordion::new();
    assert!(acc.is_empty());

    let acc = Accordion::new().section(AccordionSection::new("A"));
    assert!(!acc.is_empty());
}

// ==================== Rendering Tests ====================

#[test]
fn test_accordion_render() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new()
        .section(
            AccordionSection::new("Section 1")
                .line("Content 1")
                .expanded(true),
        )
        .section(AccordionSection::new("Section 2").line("Content 2"));

    acc.render(&mut ctx);
    // Smoke test - should not panic
}

#[test]
fn test_accordion_render_empty() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new();
    acc.render(&mut ctx);
    // Should render without panic
}

#[test]
fn test_accordion_with_border() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new()
        .border(Color::WHITE)
        .section(AccordionSection::new("Test"));

    acc.render(&mut ctx);

    assert_eq!(buffer.get(0, 0).unwrap().symbol, '┌');
}

#[test]
fn test_accordion_render_expanded_section() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(
        AccordionSection::new("Section 1")
            .line("Content 1")
            .line("Content 2")
            .expanded(true),
    );

    acc.render(&mut ctx);
    // Should render expanded content
}

#[test]
fn test_accordion_render_collapsed_section() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(
        AccordionSection::new("Section 1")
            .line("Content 1")
            .expanded(false),
    );

    acc.render(&mut ctx);
    // Should not render content
}

#[test]
fn test_accordion_render_zero_area() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(AccordionSection::new("Test"));
    acc.render(&mut ctx);
    // Should handle zero area gracefully
}

#[test]
fn test_accordion_render_small_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 2, 1); // Too small for border
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(AccordionSection::new("Test"));
    acc.render(&mut ctx);
    // Should handle small area gracefully
}

#[test]
fn test_accordion_render_with_dividers() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new()
        .dividers(true)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.render(&mut ctx);
    // Should render dividers
}

#[test]
fn test_accordion_render_without_dividers() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new()
        .dividers(false)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.render(&mut ctx);
    // Should not render dividers
}

// ==================== Edge Cases ====================

#[test]
fn test_accordion_section_with_unicode_title() {
    let acc = Accordion::new().section(AccordionSection::new("标题"));
    assert_eq!(acc.len(), 1);
}

#[test]
fn test_accordion_section_with_unicode_content() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(
        AccordionSection::new("Title")
            .content("Hello 世界\nこんにちは")
            .expanded(true),
    );

    acc.render(&mut ctx);
    // Should handle unicode content
}

#[test]
fn test_accordion_section_empty_title() {
    let sec = AccordionSection::new("");
    assert_eq!(sec.title, "");
}

#[test]
fn test_accordion_section_empty_content() {
    let sec = AccordionSection::new("Title").content("");
    assert!(sec.content.is_empty());
}

#[test]
fn test_accordion_long_title() {
    let long_title = "This is a very long title that exceeds normal width";
    let acc = Accordion::new().section(AccordionSection::new(long_title));
    assert_eq!(acc.len(), 1);
}

#[test]
fn test_accordion_long_content() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(
        AccordionSection::new("Title")
            .lines(&["Line 1", "Line 2", "Line 3", "Line 4", "Line 5"])
            .expanded(true),
    );

    acc.render(&mut ctx);
    // Should handle long content that exceeds area
}

#[test]
fn test_accordion_many_sections() {
    let sections: Vec<_> = (0..20)
        .map(|i| AccordionSection::new(format!("Section {}", i)))
        .collect();
    let acc = Accordion::new().sections(sections);
    assert_eq!(acc.len(), 20);
}
