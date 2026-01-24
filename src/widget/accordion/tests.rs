use super::*;
use crate::render::Buffer;

#[test]
fn test_section_new() {
    let s = AccordionSection::new("Title");
    assert_eq!(s.title, "Title");
    assert!(!s.expanded);
}

#[test]
fn test_section_builder() {
    let s = AccordionSection::new("FAQ")
        .line("Question 1")
        .line("Answer 1")
        .expanded(true);

    assert_eq!(s.content.len(), 2);
    assert!(s.expanded);
}

#[test]
fn test_section_content() {
    let s = AccordionSection::new("Multi").content("Line 1\nLine 2\nLine 3");

    assert_eq!(s.content.len(), 3);
}

#[test]
fn test_section_height() {
    let collapsed = AccordionSection::new("A");
    assert_eq!(collapsed.height(), 1);

    let expanded = AccordionSection::new("B")
        .line("1")
        .line("2")
        .expanded(true);
    assert_eq!(expanded.height(), 3);
}

#[test]
fn test_accordion_toggle() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(!acc.sections[0].expanded);

    acc.toggle_selected();
    assert!(acc.sections[0].expanded);

    acc.toggle_selected();
    assert!(!acc.sections[0].expanded);
}

#[test]
fn test_accordion_single_expand() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_selected();
    assert!(acc.sections[0].expanded);
    assert!(!acc.sections[1].expanded);

    acc.select_next();
    acc.expand_selected();
    assert!(!acc.sections[0].expanded);
    assert!(acc.sections[1].expanded);
}

#[test]
fn test_accordion_multi_expand() {
    let mut acc = Accordion::new()
        .multi_expand(true)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_selected();
    acc.select_next();
    acc.expand_selected();

    assert!(acc.sections[0].expanded);
    assert!(acc.sections[1].expanded);
}

#[test]
fn test_accordion_expand_collapse_all() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_all();
    assert!(acc.sections.iter().all(|s| s.expanded));

    acc.collapse_all();
    assert!(acc.sections.iter().all(|s| !s.expanded));
}

#[test]
fn test_accordion_handle_key() {
    use crate::event::Key;

    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert!(acc.handle_key(&Key::Down));
    assert_eq!(acc.selected(), 1);

    assert!(acc.handle_key(&Key::Up));
    assert_eq!(acc.selected(), 0);

    assert!(acc.handle_key(&Key::Enter));
    assert!(acc.sections[0].expanded);
}

#[test]
fn test_accordion_add_remove_title() {
    let mut acc = Accordion::new();

    acc.add_section(AccordionSection::new("A"));
    acc.add_section(AccordionSection::new("B"));
    assert_eq!(acc.len(), 2);

    acc.remove_section(0);
    assert_eq!(acc.len(), 1);
    assert_eq!(acc.sections[0].title, "B");
}

#[test]
fn test_helpers() {
    let acc = accordion().section(section("Test").line("Content"));

    assert_eq!(acc.len(), 1);
}

#[test]
fn test_section_icons() {
    let s = AccordionSection::new("Test").icons('+', '-');

    assert_eq!(s.collapsed_icon, '+');
    assert_eq!(s.expanded_icon, '-');
    assert_eq!(s.icon(), '+');
}

#[test]
fn test_accordion_collapse_selected() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").expanded(true));

    assert!(acc.sections[0].expanded);
    acc.collapse_selected();
    assert!(!acc.sections[0].expanded);
}

#[test]
fn test_accordion_collapse_selected_empty() {
    let mut acc = Accordion::new();
    // Should not panic on empty
    acc.collapse_selected();
}

#[test]
fn test_accordion_expand_selected_empty() {
    let mut acc = Accordion::new();
    // Should not panic on empty
    acc.expand_selected();
}

#[test]
fn test_accordion_toggle_multi_expand() {
    let mut acc = Accordion::new()
        .multi_expand(true)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.toggle_selected();
    assert!(acc.sections[0].expanded);

    acc.toggle_selected();
    assert!(!acc.sections[0].expanded);
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
fn test_accordion_handle_key_j_k() {
    use crate::event::Key;

    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    // j for down
    assert!(acc.handle_key(&Key::Char('j')));
    assert_eq!(acc.selected(), 1);

    // k for up
    assert!(acc.handle_key(&Key::Char('k')));
    assert_eq!(acc.selected(), 0);
}

#[test]
fn test_accordion_handle_key_space() {
    use crate::event::Key;

    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Char(' ')));
    assert!(acc.sections[0].expanded);
}

#[test]
fn test_accordion_handle_key_l_h() {
    use crate::event::Key;

    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    // l for expand
    assert!(acc.handle_key(&Key::Char('l')));
    assert!(acc.sections[0].expanded);

    // h for collapse
    assert!(acc.handle_key(&Key::Char('h')));
    assert!(!acc.sections[0].expanded);
}

#[test]
fn test_accordion_handle_key_unhandled() {
    use crate::event::Key;

    let mut acc = Accordion::new().section(AccordionSection::new("A"));

    let changed = acc.handle_key(&Key::Tab);
    assert!(!changed);
}

#[test]
fn test_accordion_colors() {
    let acc = Accordion::new()
        .header_colors(Color::WHITE, Color::RED)
        .content_colors(Color::BLACK, Color::GREEN)
        .selected_bg(Color::BLUE);

    assert_eq!(acc.header_fg, Color::WHITE);
    assert_eq!(acc.header_bg, Color::RED);
    assert_eq!(acc.content_fg, Color::BLACK);
    assert_eq!(acc.content_bg, Color::GREEN);
    assert_eq!(acc.selected_bg, Color::BLUE);
}

#[test]
fn test_accordion_dividers() {
    let acc = Accordion::new().dividers(false);
    assert!(!acc.show_dividers);
}

#[test]
fn test_accordion_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(AccordionSection::new("Test"));

    acc.render(&mut ctx);
    // Small area should not panic
}

#[test]
fn test_accordion_render_content_overflow() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(
        AccordionSection::new("Section 1")
            .line("Line 1")
            .line("Line 2")
            .line("Line 3")
            .line("Line 4")
            .line("Line 5")
            .expanded(true),
    );

    acc.render(&mut ctx);
    // Should not panic when content exceeds area
}

#[test]
fn test_section_lines() {
    let s = AccordionSection::new("Test").lines(&["Line 1", "Line 2", "Line 3"]);
    assert_eq!(s.content.len(), 3);
}

#[test]
fn test_section_icon_expanded() {
    let s = AccordionSection::new("Test").expanded(true);
    assert_eq!(s.icon(), '▼');
}

#[test]
fn test_section_clone() {
    let s = AccordionSection::new("Test").line("Content");
    let cloned = s.clone();
    assert_eq!(cloned.title, "Test");
    assert_eq!(cloned.content.len(), 1);
}
