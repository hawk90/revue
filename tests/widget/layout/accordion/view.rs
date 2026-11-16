//! Accordion placement, and a model of the selection and the expanded
//! sections under keys and section changes, checked on the screen

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Accordion, AccordionSection};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const SELECTED_BG: Color = Color::rgb(60, 90, 140);

fn draw(acc: &Accordion, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    acc.render(&mut ctx);
    buffer
}

/// Row `y` as text, trailing blanks trimmed.
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .filter(|&c| c != '\0') // a wide glyph's continuation cell
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn three() -> Accordion {
    Accordion::new()
        .section(
            AccordionSection::new("一 First")
                .line("a1 世界")
                .expanded(true),
        )
        .section(AccordionSection::new("Second").lines(&["b1", "b2"]))
        .section(AccordionSection::new("Third").line("c1").expanded(true))
}

#[test]
fn test_accordion_zero_areas_and_offset() {
    for acc in [
        three(),
        three().dividers(false),
        three().border(Color::GREEN),
        three()
            .header_colors(Color::YELLOW, Color::BLACK)
            .max_height(4),
        Accordion::new(),
    ] {
        assert_zero_areas_draw_nothing(&acc);
        assert_same_when_offset(acc.clone(), 20, 9);
    }
}

#[test]
fn test_accordion_border_insets_every_row() {
    let buffer = draw(&three().border(Color::GREEN), 14, 9);
    let rows: Vec<String> = (0..9).map(|y| row(&buffer, y)).collect();
    assert_eq!(
        rows,
        [
            "┌────────────┐",
            "│ ▼ 一 First │",
            "│   a1 世界  │",
            "│────────────│",
            "│ ▶ Second   │",
            "│────────────│",
            "│ ▼ Third    │",
            "│   c1       │",
            "└────────────┘",
        ]
    );
}

#[test]
fn test_accordion_default_colors() {
    let buffer = draw(&three(), 20, 9);
    // Selected header, its content row, a divider, an unselected header
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(SELECTED_BG));
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(0, 1).unwrap().bg, Some(Color::rgb(30, 30, 30)));
    assert_eq!(
        buffer.get(3, 1).unwrap().fg,
        Some(Color::rgb(200, 200, 200))
    );
    assert_eq!(buffer.get(0, 2).unwrap().fg, Some(Color::rgb(60, 60, 60)));
    assert_eq!(buffer.get(0, 3).unwrap().bg, Some(Color::rgb(50, 50, 50)));
}

#[test]
fn test_accordion_min_and_max_size() {
    // max_width: rows are cut to it
    let buffer = draw(&three().max_width(8), 20, 9);
    assert_eq!(row(&buffer, 0), " ▼ 一 F");
    assert_eq!(row(&buffer, 2), "────────");
    assert_eq!(buffer.get(8, 2).unwrap().symbol, ' ');
    // max_height: sections past it are not drawn
    let buffer = draw(&three().max_height(3), 20, 9);
    assert_eq!(row(&buffer, 2), "─".repeat(20));
    assert_eq!(row(&buffer, 3), "");
}

#[derive(Clone, Debug)]
enum Op {
    Key(Key),
    ExpandAll,
    CollapseAll,
    Select(usize),
    Add(bool),
    Remove(usize),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        prop_oneof![
            Just(Key::Up),
            Just(Key::Down),
            Just(Key::Char('j')),
            Just(Key::Char('k')),
            Just(Key::Enter),
            Just(Key::Char(' ')),
            Just(Key::Right),
            Just(Key::Left),
            Just(Key::Char('l')),
            Just(Key::Char('h')),
            Just(Key::Tab),
            Just(Key::Char('x')),
        ]
        .prop_map(Op::Key),
        Just(Op::ExpandAll),
        Just(Op::CollapseAll),
        (0usize..7).prop_map(Op::Select),
        any::<bool>().prop_map(Op::Add),
        (0usize..7).prop_map(Op::Remove),
    ]
}

/// The rows a tall area shows for `titles`/`lines`/`open` with dividers.
fn expected_rows(titles: &[String], lines: &[usize], open: &[bool]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, t) in titles.iter().enumerate() {
        out.push(format!(" {} {t}", if open[i] { '▼' } else { '▶' }));
        if open[i] {
            for j in 0..lines[i] {
                out.push(format!("   {t}.{j}"));
            }
        }
        if i + 1 < titles.len() {
            out.push("─".repeat(24));
        }
    }
    out
}

proptest! {
    /// After any keys, selections, expand/collapse-all and section changes,
    /// the selection and the open sections are what a plain model gives, and
    /// the screen shows exactly them: one header per section with its icon,
    /// the content of open ones, dividers between, and the selected header
    /// alone in the selected background.
    #[test]
    fn prop_accordion_follows_the_model(
        multi in any::<bool>(),
        start in prop::collection::vec((0usize..3, any::<bool>()), 0..5),
        ops in prop::collection::vec(op(), 0..30),
    ) {
        let mut titles: Vec<String> = Vec::new();
        let mut lines: Vec<usize> = Vec::new();
        let mut open: Vec<bool> = Vec::new();
        let mut next_id = 0usize;
        let mut section = |titles: &mut Vec<String>, n: usize, e: bool| {
            let t = format!("S{next_id}");
            next_id += 1;
            let body: Vec<String> = (0..n).map(|j| format!("{t}.{j}")).collect();
            let refs: Vec<&str> = body.iter().map(String::as_str).collect();
            titles.push(t.clone());
            AccordionSection::new(t).lines(&refs).expanded(e)
        };
        let mut acc = Accordion::new().multi_expand(multi);
        for &(n, e) in &start {
            acc = acc.section(section(&mut titles, n, e));
            lines.push(n);
            open.push(e);
        }
        let mut sel = 0usize;
        // An index set past the end, selected once a section is there
        // (#1011); a move forgets it
        let mut requested: Option<usize> = None;
        for o in &ops {
            let len = titles.len();
            match o {
                Op::Key(k) => {
                    let handled = acc.handle_key(k);
                    if matches!(k, Key::Up | Key::Char('k') | Key::Down | Key::Char('j')) {
                        requested = None;
                    }
                    let expect_handled = !matches!(k, Key::Tab | Key::Char('x'));
                    prop_assert_eq!(handled, expect_handled);
                    match k {
                        Key::Up | Key::Char('k') if len > 0 => sel = (sel + len - 1) % len,
                        Key::Down | Key::Char('j') if len > 0 => sel = (sel + 1) % len,
                        Key::Enter | Key::Char(' ') if sel < len => {
                            if multi { open[sel] = !open[sel] } else {
                                let was = open[sel];
                                open.iter_mut().for_each(|e| *e = false);
                                open[sel] = !was;
                            }
                        },
                        Key::Right | Key::Char('l') if sel < len => {
                            if !multi { open.iter_mut().for_each(|e| *e = false) }
                            open[sel] = true;
                        },
                        Key::Left | Key::Char('h') if sel < len => open[sel] = false,
                        _ => {}
                    }
                }
                Op::ExpandAll => { acc.expand_all(); open.iter_mut().for_each(|e| *e = true) }
                Op::CollapseAll => { acc.collapse_all(); open.iter_mut().for_each(|e| *e = false) }
                Op::Select(i) => {
                    acc.set_selected(*i);
                    sel = (*i).min(len.saturating_sub(1));
                    requested = (*i >= len).then_some(*i);
                }
                Op::Add(e) => {
                    acc.add_section(section(&mut titles, 1, *e));
                    lines.push(1);
                    open.push(*e);
                    if let Some(r) = requested.filter(|&r| r < titles.len()) {
                        sel = r;
                        requested = None;
                    }
                }
                Op::Remove(i) => {
                    // The selection stays on its section (#975): one above
                    // it moves it up by one
                    let removed = acc.remove_section(*i);
                    prop_assert_eq!(removed.is_some(), *i < len);
                    if *i < len {
                        titles.remove(*i);
                        lines.remove(*i);
                        open.remove(*i);
                        if *i < sel { sel -= 1 }
                        if let Some(r) = requested.filter(|&r| r < titles.len()) {
                            sel = r;
                            requested = None;
                        }
                        if sel >= titles.len() && !titles.is_empty() { sel = titles.len() - 1 }
                    }
                }
            }
            prop_assert_eq!(acc.selected(), sel);
            prop_assert_eq!(acc.len(), titles.len());
        }

        let expected = expected_rows(&titles, &lines, &open);
        let h = expected.len() as u16 + 2;
        let buffer = draw(&acc, 24, h);
        let drawn: Vec<String> = (0..h).map(|y| row(&buffer, y)).collect();
        prop_assert_eq!(&drawn[..expected.len()], &expected[..]);
        prop_assert!(drawn[expected.len()..].iter().all(|r| r.is_empty()));
        for (y, text) in expected.iter().enumerate() {
            let is_header = text.starts_with(" ▶") || text.starts_with(" ▼");
            let title: String = text.chars().skip(3).collect();
            let selected = is_header && titles.get(sel) == Some(&title);
            let bg = buffer.get(0, y as u16).unwrap().bg;
            prop_assert_eq!(bg == Some(SELECTED_BG), selected, "row {}", y);
        }
    }
}

/// A selected section below the area used never to be drawn (#973).
#[test]
fn accordion_selected_section_scrolls_into_view() {
    let mut acc = Accordion::new().dividers(false);
    for i in 0..8 {
        acc = acc.section(AccordionSection::new(format!("S{i}")));
    }
    acc.set_selected(7);
    let buffer = draw(&acc, 12, 3);
    let shown: Vec<String> = (0..3).map(|y| row(&buffer, y)).collect();
    assert!(shown.iter().any(|r| r.contains("S7")), "{shown:?}");
}

/// An expanded section shows its content under the header when it fits.
#[test]
fn accordion_an_expanded_section_is_scrolled_into_view_whole() {
    let mut acc = Accordion::new().dividers(false);
    for i in 0..6 {
        acc = acc.section(AccordionSection::new(format!("S{i}")).line("a").line("b"));
    }
    acc.set_selected(5);
    acc.expand_selected();
    let buffer = draw(&acc, 12, 4);
    let shown: Vec<String> = (0..4).map(|y| row(&buffer, y).trim().to_string()).collect();
    assert_eq!(&shown[1..], ["▼ S5", "a", "b"], "{shown:?}");
}

/// Removing a section above the selected one moved the selection to the
/// next section (#975).
#[test]
fn accordion_selection_stays_on_its_section_when_one_above_is_removed() {
    let mut acc = Accordion::new()
        .section(AccordionSection::new("A"))
        .section(AccordionSection::new("B"))
        .section(AccordionSection::new("C"));
    acc.set_selected(1); // B
    acc.remove_section(0);
    assert_eq!(acc.selected(), 0, "B is now at 0");
}

// ---------------------------------------------------------------------------
// A border from the stylesheet (#883)
// ---------------------------------------------------------------------------

/// Draw `acc` with `style` as its computed style.
fn draw_styled(acc: &Accordion, style: &revue::style::Style, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, w, h), style);
    acc.render(&mut ctx);
    buffer
}

fn css_style(border_color: Color, color: Color) -> revue::style::Style {
    let mut style = revue::style::Style::default();
    style.visual.border_color = border_color;
    style.visual.color = color;
    style
}

fn one() -> Accordion {
    Accordion::new().section(AccordionSection::new("A").line("a1").expanded(true))
}

const FRAMED_ONE: [&str; 5] = [
    "┌──────────┐",
    "│ ▼ A      │",
    "│   a1     │",
    "│          │",
    "└──────────┘",
];

/// The issue's case: `border-color` alone draws the border, and the header and
/// content rows sit inside it instead of over its sides and top.
#[test]
fn a_border_color_from_the_stylesheet_insets_the_rows() {
    let style = css_style(Color::MAGENTA, Color::default());
    let buffer = draw_styled(&one(), &style, 12, 5);
    let rows: Vec<String> = (0..5).map(|y| row(&buffer, y)).collect();
    assert_eq!(rows, FRAMED_ONE);
    for (x, y) in [(0, 1), (11, 1), (0, 2), (11, 2), (5, 0)] {
        assert_eq!(
            buffer.get(x, y).unwrap().fg,
            Some(Color::MAGENTA),
            "({x},{y})"
        );
    }
}

/// `color` alone stands in for `border-color` (CSS's `currentColor`), and
/// insets the rows the same way.
#[test]
fn a_color_from_the_stylesheet_insets_the_rows() {
    let style = css_style(Color::default(), Color::RED);
    let buffer = draw_styled(&one(), &style, 12, 5);
    let rows: Vec<String> = (0..5).map(|y| row(&buffer, y)).collect();
    assert_eq!(rows, FRAMED_ONE);
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::RED));
}

/// The builder's color outranks the stylesheet's; the frame is the same.
#[test]
fn the_builder_border_outranks_the_stylesheet() {
    let style = css_style(Color::MAGENTA, Color::default());
    let buffer = draw_styled(&one().border(Color::GREEN), &style, 12, 5);
    let rows: Vec<String> = (0..5).map(|y| row(&buffer, y)).collect();
    assert_eq!(rows, FRAMED_ONE);
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::GREEN));
}

/// At the narrowest width that draws, the border - the stylesheet's or the
/// builder's - keeps its sides: the icon, a column past the one inside, is
/// clipped instead of drawn over the right side.
#[test]
fn a_border_keeps_its_sides_at_three_columns() {
    let style = css_style(Color::MAGENTA, Color::default());
    for buffer in [
        draw_styled(&one(), &style, 3, 4),
        draw(&one().border(Color::GREEN), 3, 4),
    ] {
        let rows: Vec<String> = (0..4).map(|y| row(&buffer, y)).collect();
        assert_eq!(rows, ["┌─┐", "│ │", "│ │", "└─┘"]);
    }
}

/// A style that names neither color draws no border, and the rows start at
/// the edge as without a style.
#[test]
fn a_style_without_border_colors_draws_no_border() {
    let style = css_style(Color::default(), Color::default());
    let styled = draw_styled(&one(), &style, 12, 3);
    let plain = draw(&one(), 12, 3);
    let rows: Vec<String> = (0..3).map(|y| row(&styled, y)).collect();
    assert_eq!(rows, [" ▼ A", "   a1", ""]);
    for y in 0..3 {
        for x in 0..12 {
            let (a, b) = (styled.get(x, y).unwrap(), plain.get(x, y).unwrap());
            assert_eq!((a.symbol, a.fg, a.bg), (b.symbol, b.fg, b.bg), "({x},{y})");
        }
    }
}

/// In a content-sized stack the accordion's measured height includes the
/// stylesheet's border, so the next sibling starts below its bottom row
/// rather than over it.
#[test]
fn a_content_sized_stack_leaves_room_for_a_stylesheet_border() {
    use revue::prelude::{vstack, Text};
    use revue::testing::PipelineHarness;

    struct Page;
    impl View for Page {
        fn render(&self, ctx: &mut RenderContext) {
            vstack()
                .content_sized(true)
                .child(one())
                .child(Text::new("after"))
                .render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Page"
        }
    }

    for css in [
        "Accordion { border-color: magenta; }",
        "Accordion { color: red; }",
    ] {
        let mut h = PipelineHarness::with_css(css, 12, 8)
            .dom_from_render(true)
            .css_layout(true);
        h.draw(&Page);
        assert_eq!(
            h.screen_text(),
            "┌──────────┐\n│ ▼ A      │\n│   a1     │\n└──────────┘\nafter",
            "{css}"
        );
    }
}
