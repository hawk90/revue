//! Accordion widget tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{accordion, section, Accordion, AccordionSection, View};

/// Render `acc` into a fresh `w` x `h` buffer.
fn draw(acc: &Accordion, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    acc.render(&mut ctx);
    buffer
}

/// Row `y` of `buffer` as text, trailing blanks trimmed.
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Every row of `buffer`, trailing blank rows dropped.
fn rows(buffer: &Buffer) -> Vec<String> {
    let mut out: Vec<String> = (0..buffer.height()).map(|y| row(buffer, y)).collect();
    while out.last().is_some_and(|r| r.is_empty()) {
        out.pop();
    }
    out
}

/// The headers' icons, top to bottom: one per section, `▼` while expanded.
fn icons(acc: &Accordion) -> String {
    let buffer = draw(acc, 30, 60);
    (0..60)
        .map(|y| buffer.get(1, y).unwrap().symbol)
        .filter(|c| *c == '▶' || *c == '▼')
        .collect()
}

/// Whether no cell of `buffer` was drawn.
fn blank(buffer: &Buffer) -> bool {
    (0..buffer.height()).all(|y| {
        (0..buffer.width()).all(|x| {
            let c = buffer.get(x, y).unwrap();
            c.symbol == ' ' && c.fg.is_none() && c.bg.is_none()
        })
    })
}

fn two() -> Accordion {
    Accordion::new()
        .section(AccordionSection::new("A").line("a1"))
        .section(AccordionSection::new("B").line("b1"))
}

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
    let open_both = |acc: Accordion| {
        let mut acc = acc;
        acc.expand_selected();
        acc.select_next();
        acc.expand_selected();
        icons(&acc)
    };
    assert_eq!(open_both(two().multi_expand(true)), "▼▼");
    assert_eq!(open_both(two().multi_expand(false)), "▶▼");
}

#[test]
fn test_accordion_header_colors() {
    let acc = two().header_colors(Color::YELLOW, Color::BLUE);
    let buffer = draw(&acc, 20, 6);
    // Row 2 is the second header (row 1 is the divider); it is not selected
    let icon = buffer.get(1, 2).unwrap();
    assert_eq!(icon.symbol, '▶');
    assert_eq!(icon.fg, Some(Color::YELLOW));
    assert_eq!(icon.bg, Some(Color::BLUE));
    assert_eq!(buffer.get(19, 2).unwrap().bg, Some(Color::BLUE));
    // The selected header keeps the fg but takes the selected background
    assert_eq!(buffer.get(3, 0).unwrap().fg, Some(Color::YELLOW));
    assert_ne!(buffer.get(3, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_accordion_selected_bg() {
    let mut acc = two().selected_bg(Color::GREEN);
    let buffer = draw(&acc, 20, 6);
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(Color::GREEN));
    assert_eq!(buffer.get(19, 0).unwrap().bg, Some(Color::GREEN));
    assert_ne!(buffer.get(0, 2).unwrap().bg, Some(Color::GREEN));

    // It follows the selection
    acc.select_next();
    let buffer = draw(&acc, 20, 6);
    assert_ne!(buffer.get(0, 0).unwrap().bg, Some(Color::GREEN));
    assert_eq!(buffer.get(0, 2).unwrap().bg, Some(Color::GREEN));
}

#[test]
fn test_accordion_content_colors() {
    let acc = Accordion::new()
        .content_colors(Color::YELLOW, Color::RED)
        .section(AccordionSection::new("A").line("text").expanded(true));
    let buffer = draw(&acc, 20, 4);
    assert_eq!(row(&buffer, 1), "   text");
    let text = buffer.get(3, 1).unwrap();
    assert_eq!((text.fg, text.bg), (Some(Color::YELLOW), Some(Color::RED)));
    // The background spans the row
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::RED));
    assert_eq!(buffer.get(19, 1).unwrap().bg, Some(Color::RED));
}

#[test]
fn test_accordion_dividers() {
    // Off: the second header follows the first
    assert_eq!(rows(&draw(&two().dividers(false), 10, 5)), [" ▶ A", " ▶ B"]);
    // On (the default): a line between them
    assert_eq!(rows(&draw(&two(), 10, 5)), [" ▶ A", "──────────", " ▶ B"]);
}

#[test]
fn test_accordion_border() {
    let acc = Accordion::new()
        .border(Color::CYAN)
        .section(AccordionSection::new("T"));
    let buffer = draw(&acc, 8, 4);
    assert_eq!(
        rows(&buffer),
        ["┌──────┐", "│ ▶ T  │", "│      │", "└──────┘"]
    );
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(7, 3).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_accordion_builder_chain() {
    let mut acc = Accordion::new()
        .multi_expand(true)
        .header_colors(Color::WHITE, Color::BLUE)
        .selected_bg(Color::GREEN)
        .content_colors(Color::YELLOW, Color::RED)
        .dividers(false)
        .border(Color::CYAN)
        .section(AccordionSection::new("A").line("a1").expanded(true))
        .section(AccordionSection::new("B").line("b1"));
    acc.select_next();
    acc.expand_selected();
    let buffer = draw(&acc, 10, 6);
    assert_eq!(
        rows(&buffer),
        [
            "┌────────┐",
            "│ ▼ A    │",
            "│   a1   │",
            "│ ▼ B    │",
            "│   b1   │",
            "└────────┘"
        ]
    );
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_eq!(buffer.get(1, 1).unwrap().bg, Some(Color::BLUE));
    assert_eq!(buffer.get(1, 3).unwrap().bg, Some(Color::GREEN));
    assert_eq!(buffer.get(4, 2).unwrap().fg, Some(Color::YELLOW));
    assert_eq!(buffer.get(1, 2).unwrap().bg, Some(Color::RED));
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
    assert_eq!(icons(&acc), "▼");
    acc.toggle_selected();
    assert_eq!(icons(&acc), "▶");
}

#[test]
fn test_accordion_expand_selected() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    acc.expand_selected();
    assert_eq!(rows(&draw(&acc, 20, 4)), [" ▼ A", "   Content"]);
    // Expanding again keeps it open
    acc.expand_selected();
    assert_eq!(icons(&acc), "▼");
}

#[test]
fn test_accordion_collapse_selected() {
    let mut acc =
        Accordion::new().section(AccordionSection::new("A").line("Content").expanded(true));

    acc.collapse_selected();
    assert_eq!(rows(&draw(&acc, 20, 4)), [" ▶ A"]);
}

#[test]
fn test_accordion_expand_all() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_all();
    // Even in single-expand mode
    assert_eq!(icons(&acc), "▼▼");
}

#[test]
fn test_accordion_collapse_all() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A").expanded(true))
        .section(AccordionSection::new("B").expanded(true));

    acc.collapse_all();
    assert_eq!(icons(&acc), "▶▶");
}

#[test]
fn test_accordion_single_expand_mode() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    acc.expand_selected();
    assert_eq!(icons(&acc), "▼▶");
    acc.set_selected(1);
    acc.expand_selected();
    assert_eq!(icons(&acc), "▶▼");
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
    assert_eq!(icons(&acc), "▼▼");
    // Toggling one leaves the other open
    acc.toggle_selected();
    assert_eq!(icons(&acc), "▼▶");
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
    assert_eq!(icons(&acc), "▼");
    assert!(acc.handle_key(&Key::Enter));
    assert_eq!(icons(&acc), "▶");
}

#[test]
fn test_accordion_handle_key_space() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Char(' ')));
    assert_eq!(icons(&acc), "▼");
    assert!(acc.handle_key(&Key::Char(' ')));
    assert_eq!(icons(&acc), "▶");
}

#[test]
fn test_accordion_handle_key_right() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Right));
    assert_eq!(icons(&acc), "▼");
    // Right only expands
    assert!(acc.handle_key(&Key::Right));
    assert_eq!(icons(&acc), "▼");
}

#[test]
fn test_accordion_handle_key_left() {
    let mut acc =
        Accordion::new().section(AccordionSection::new("A").line("Content").expanded(true));

    assert!(acc.handle_key(&Key::Left));
    assert_eq!(icons(&acc), "▶");
    // Left only collapses
    assert!(acc.handle_key(&Key::Left));
    assert_eq!(icons(&acc), "▶");
}

#[test]
fn test_accordion_handle_key_l() {
    let mut acc = Accordion::new().section(AccordionSection::new("A").line("Content"));

    assert!(acc.handle_key(&Key::Char('l')));
    assert_eq!(icons(&acc), "▼");
}

#[test]
fn test_accordion_handle_key_h() {
    let mut acc =
        Accordion::new().section(AccordionSection::new("A").line("Content").expanded(true));

    assert!(acc.handle_key(&Key::Char('h')));
    assert_eq!(icons(&acc), "▶");
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
    let acc = Accordion::new()
        .section(
            AccordionSection::new("Section 1")
                .line("Content 1")
                .expanded(true),
        )
        .section(AccordionSection::new("Section 2").line("Content 2"));

    let buffer = draw(&acc, 40, 10);
    assert_eq!(
        rows(&buffer),
        [
            " ▼ Section 1",
            "   Content 1",
            "─".repeat(40).as_str(),
            " ▶ Section 2"
        ]
    );
    // The first section is selected, so its title is bold
    assert!(buffer
        .get(3, 0)
        .unwrap()
        .modifier
        .contains(revue::render::Modifier::BOLD));
    assert!(!buffer
        .get(3, 3)
        .unwrap()
        .modifier
        .contains(revue::render::Modifier::BOLD));
}

#[test]
fn test_accordion_render_empty() {
    let acc = Accordion::new();
    assert!(blank(&draw(&acc, 40, 10)));
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
    let acc = Accordion::new().section(
        AccordionSection::new("Section 1")
            .line("Content 1")
            .line("Content 2")
            .expanded(true),
    );

    assert_eq!(
        rows(&draw(&acc, 40, 10)),
        [" ▼ Section 1", "   Content 1", "   Content 2"]
    );
}

#[test]
fn test_accordion_render_collapsed_section() {
    let acc = Accordion::new().section(
        AccordionSection::new("Section 1")
            .line("Content 1")
            .expanded(false),
    );

    assert_eq!(rows(&draw(&acc, 40, 10)), [" ▶ Section 1"]);
}

#[test]
fn test_accordion_render_zero_area() {
    let mut buffer = Buffer::new(10, 10);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(AccordionSection::new("Test"));
    acc.render(&mut ctx);
    assert!(blank(&buffer));
}

#[test]
fn test_accordion_render_small_area() {
    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 2, 1); // Too small for border
    let mut ctx = RenderContext::new(&mut buffer, area);

    let acc = Accordion::new().section(AccordionSection::new("Test"));
    acc.render(&mut ctx);
    // Below 3 columns nothing is drawn; at 3 the header is
    assert!(blank(&buffer));
    assert_eq!(row(&draw(&acc, 3, 1), 0), " ▶");
}

#[test]
fn test_accordion_render_with_dividers() {
    let acc = Accordion::new()
        .dividers(true)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    let buffer = draw(&acc, 40, 10);
    // One divider between the two sections, none after the last
    assert_eq!(rows(&buffer), [" ▶ A", "─".repeat(40).as_str(), " ▶ B"]);
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::rgb(60, 60, 60)));
}

#[test]
fn test_accordion_render_without_dividers() {
    let acc = Accordion::new()
        .dividers(false)
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"));

    assert_eq!(rows(&draw(&acc, 40, 10)), [" ▶ A", " ▶ B"]);
}

// ==================== Edge Cases ====================

#[test]
fn test_accordion_section_with_unicode_title() {
    let acc = Accordion::new().section(AccordionSection::new("标题"));
    assert_eq!(acc.len(), 1);
    let buffer = draw(&acc, 20, 2);
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '标');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, '题');
}

#[test]
fn test_accordion_section_with_unicode_content() {
    let acc = Accordion::new().section(
        AccordionSection::new("Title")
            .content("Hello 世界\nこんにちは")
            .expanded(true),
    );

    let buffer = draw(&acc, 40, 10);
    assert_eq!(buffer.get(9, 1).unwrap().symbol, '世');
    assert_eq!(buffer.get(11, 1).unwrap().symbol, '界');
    assert_eq!(buffer.get(3, 2).unwrap().symbol, 'こ');
    assert_eq!(buffer.get(5, 2).unwrap().symbol, 'ん');
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
    // Cut to the width less the icon's three columns and one more
    assert_eq!(row(&draw(&acc, 20, 2), 0), " ▶ This is a very l");
}

#[test]
fn test_accordion_long_content() {
    let acc = Accordion::new().section(
        AccordionSection::new("Title")
            .lines(&["Line 1", "Line 2", "Line 3", "Line 4", "Line 5"])
            .expanded(true),
    );

    // Lines past the area are cut off
    assert_eq!(
        rows(&draw(&acc, 40, 4)),
        [" ▼ Title", "   Line 1", "   Line 2", "   Line 3"]
    );
}

#[test]
fn test_accordion_many_sections() {
    let sections: Vec<_> = (0..20)
        .map(|i| AccordionSection::new(format!("Section {}", i)))
        .collect();
    let acc = Accordion::new().sections(sections);
    assert_eq!(acc.len(), 20);
}

mod snapshots {

    use revue::testing::{Pilot, TestApp};
    use revue::widget::Accordion;

    #[test]
    fn test_accordion_basic() {
        use revue::widget::AccordionSection;

        let view = Accordion::new()
            .section(AccordionSection::new("Section 1").content("Content for section 1"))
            .section(AccordionSection::new("Section 2").content("Content for section 2"))
            .section(AccordionSection::new("Section 3").content("Content for section 3"));

        let mut app = TestApp::new(view);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("accordion_basic");
    }
}

mod view;
