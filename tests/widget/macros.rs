//! Tests for the declarative UI macros
//!
//! Each macro is shorthand for a builder chain: check that it builds the
//! same widget by rendering both and comparing every cell.

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::{hstack, vstack, Border, RenderContext, Text, View};
use revue::{bordered, hstack, text, ui, vstack};

/// Render `view` into a `width` x `height` buffer.
fn render(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

/// Every cell of `buffer` as (symbol, fg, bg, modifier).
fn cells(buffer: &Buffer) -> Vec<(char, Option<Color>, Option<Color>, Modifier)> {
    (0..buffer.height())
        .flat_map(|y| (0..buffer.width()).map(move |x| (x, y)))
        .map(|(x, y)| {
            let c = buffer.get(x, y).unwrap();
            (c.symbol, c.fg, c.bg, c.modifier)
        })
        .collect()
}

fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

fn assert_same(a: &impl View, b: &impl View, width: u16, height: u16) {
    assert_eq!(
        cells(&render(a, width, height)),
        cells(&render(b, width, height))
    );
}

// =========================================================================
// vstack! / hstack!
// =========================================================================

#[test]
fn test_vstack_macro() {
    let stack = vstack![Text::new("Line 1"), Text::new("Line 2"),];
    assert_eq!(stack.len(), 2);
    assert_same(
        &stack,
        &vstack()
            .child(Text::new("Line 1"))
            .child(Text::new("Line 2")),
        10,
        4,
    );
    let rows = rows(&render(&stack, 10, 4));
    assert!(rows.iter().any(|r| r == "Line 1"));
    assert!(rows.iter().any(|r| r == "Line 2"));
}

#[test]
fn test_vstack_macro_with_gap() {
    let stack = vstack![gap: 2;
        Text::new("A"),
        Text::new("B"),
    ];
    assert_eq!(stack.len(), 2);
    assert_same(
        &stack,
        &vstack().gap(2).child(Text::new("A")).child(Text::new("B")),
        5,
        6,
    );
}

#[test]
fn test_hstack_macro() {
    let stack = hstack![Text::new("Left"), Text::new("Right"),];
    assert_eq!(stack.len(), 2);
    assert_same(
        &stack,
        &hstack().child(Text::new("Left")).child(Text::new("Right")),
        20,
        1,
    );
    let row = &rows(&render(&stack, 20, 1))[0];
    assert!(row.contains("Left") && row.contains("Right"), "{row:?}");
}

#[test]
fn test_hstack_macro_with_gap() {
    let stack = hstack![gap: 2;
        Text::new("A"),
        Text::new("B"),
    ];
    assert_eq!(stack.len(), 2);
    assert_same(
        &stack,
        &hstack().gap(2).child(Text::new("A")).child(Text::new("B")),
        10,
        1,
    );
}

#[test]
fn test_stack_macro_trailing_comma_and_single_child() {
    assert_eq!(vstack![Text::new("A"), Text::new("B"),].len(), 2);
    assert_eq!(hstack![Text::new("A"), Text::new("B"),].len(), 2);
    assert_eq!(vstack![Text::new("Only")].len(), 1);
    assert_eq!(hstack![Text::new("Only")].len(), 1);
}

#[test]
fn test_stack_macro_empty() {
    let v = vstack![];
    let h = hstack![];
    assert_eq!(v.len(), 0);
    assert_eq!(h.len(), 0);
    assert!(rows(&render(&v, 5, 2)).iter().all(|r| r.is_empty()));
}

#[test]
fn test_nested_layout() {
    let layout = vstack![
        Text::heading("Title"),
        hstack![Text::new("Left"), Text::new("Right"),],
        Text::muted("Footer"),
    ];
    assert_eq!(layout.len(), 3);
    assert_same(
        &layout,
        &vstack()
            .child(Text::heading("Title"))
            .child(hstack().child(Text::new("Left")).child(Text::new("Right")))
            .child(Text::muted("Footer")),
        20,
        6,
    );
}

// =========================================================================
// text!
// =========================================================================

#[test]
fn test_text_macro() {
    let t = text!("Hello");
    assert_eq!(t.content(), "Hello");
    assert_same(&t, &Text::new("Hello"), 8, 1);
}

#[test]
fn test_text_macro_named_colors() {
    // The four semantic colors map to the matching Text constructors
    assert_same(&text!("Error", red), &Text::error("Error"), 8, 1);
    assert_same(&text!("Success", green), &Text::success("Success"), 8, 1);
    assert_same(&text!("Warning", yellow), &Text::warning("Warning"), 8, 1);
    assert_same(&text!("Info", cyan), &Text::info("Info"), 8, 1);
    assert_eq!(text!("Error", red).content(), "Error");
}

#[test]
fn test_text_macro_color_constant() {
    let t = text!("Plain", MAGENTA);
    assert_eq!(render(&t, 8, 1).get(0, 0).unwrap().fg, Some(Color::MAGENTA));
}

#[test]
fn test_text_macro_bold_modifier() {
    let t = text!("Bold", WHITE, bold);
    assert_eq!(t.content(), "Bold");
    let buffer = render(&t, 8, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::WHITE));
    assert!(cell.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_text_macro_italic_modifier() {
    let t = text!("Italic", CYAN, italic);
    let buffer = render(&t, 8, 1);
    let cell = buffer.get(0, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::CYAN));
    assert!(cell.modifier.contains(Modifier::ITALIC));
    assert!(!cell.modifier.contains(Modifier::BOLD));
}

// =========================================================================
// bordered!
// =========================================================================

#[test]
fn test_bordered_macro() {
    let b = bordered![Text::new("Content")];
    assert_same(&b, &Border::single().child(Text::new("Content")), 12, 3);
    let rows = rows(&render(&b, 12, 3));
    assert!(rows[0].starts_with('┌'), "{rows:?}");
    assert!(rows[1].contains("Content"), "{rows:?}");
}

#[test]
fn test_bordered_macro_with_title() {
    let b = bordered!["Title"; Text::new("Content")];
    assert_same(
        &b,
        &Border::single().title("Title").child(Text::new("Content")),
        20,
        5,
    );
    assert!(rows(&render(&b, 20, 5))[0].contains("Title"));
}

#[test]
fn test_bordered_macro_border_type_and_title() {
    let b = bordered![double, "Card"; Text::new("Content")];
    assert_same(
        &b,
        &Border::double().title("Card").child(Text::new("Content")),
        20,
        5,
    );
    let top = &rows(&render(&b, 20, 5))[0];
    assert!(top.starts_with('╔') && top.contains("Card"), "{top:?}");
}

#[test]
fn test_bordered_macro_rounded() {
    let b = bordered![rounded, "Card"; Text::new("Content")];
    assert_same(
        &b,
        &Border::rounded().title("Card").child(Text::new("Content")),
        20,
        5,
    );
    assert!(rows(&render(&b, 20, 5))[0].starts_with('╭'));
}

// =========================================================================
// ui!
// =========================================================================

#[test]
#[allow(unused_parens)]
fn test_ui_macro() {
    // Each child is a single token tree, so expressions are parenthesized
    // (which ui! then reports as unnecessary parentheses).
    // (The nested `hstack { .. }` form shown in the ui! docs does not
    // compile: the children are matched one token tree at a time.)
    let layout = ui! {
        vstack(gap: 1) {
            (Text::heading("Title"))
            (Text::new("Body"))
        }
    };
    assert_eq!(layout.len(), 2);
    assert_same(
        &layout,
        &vstack()
            .gap(1)
            .child(Text::heading("Title"))
            .child(Text::new("Body")),
        20,
        6,
    );

    let row = ui! {
        hstack {
            (Text::new("L"))
            (Text::new("R"))
        }
    };
    assert_eq!(row.len(), 2);
    assert_same(
        &row,
        &hstack().child(Text::new("L")).child(Text::new("R")),
        10,
        1,
    );
}
