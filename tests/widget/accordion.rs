//! Accordion widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{Accordion, AccordionSection, View};

#[test]
fn test_accordion_new() {
    let acc = Accordion::new();
    assert!(acc.is_empty());
    assert_eq!(acc.selected(), 0);
}

#[test]
fn test_accordion_sections() {
    let acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert_eq!(acc.len(), 2);
}

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
fn test_accordion_set_selected() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.set_selected(1);
    assert_eq!(acc.selected(), 1);
}

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
fn test_accordion_default() {
    let acc = Accordion::default();
    assert!(acc.is_empty());
}

// =============================================================================
