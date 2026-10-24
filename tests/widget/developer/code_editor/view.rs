//! CodeEditor on the screen: placement, whitespace markers, find and bracket
//! highlights, the find and go-to-line boxes, and a proptest that the cursor
//! stays in view after any keys at any size

use proptest::prelude::*;
use revue::event::Key;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::code_editor::EditorConfig;
use revue::widget::CodeEditor;

use super::render::{row, screen};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// An editor without the gutter, the band or bracket matching, so cells
/// show only what a test looks at
fn plain(content: &str) -> CodeEditor {
    CodeEditor::new()
        .content(content)
        .line_numbers(false)
        .highlight_current_line(false)
        .bracket_matching(false)
}

#[test]
fn test_code_editor_zero_areas_draw_nothing() {
    let mut editor = CodeEditor::new()
        .content("fn main() {\n    한글\n}")
        .minimap(true);
    editor.open_find();
    assert_zero_areas_draw_nothing(&editor);
}

#[test]
fn test_code_editor_same_when_offset() {
    let mut editor = CodeEditor::new()
        .content("fn main() {\n    let s = \"한글\";\n}\n")
        .minimap(true);
    editor.set_cursor(1, 9);
    editor.set_find_query("s");
    assert_same_when_offset(editor.clone(), 30, 4);
    editor.open_goto_line();
    assert_same_when_offset(editor, 30, 4);
}

#[test]
fn test_code_editor_whitespace_markers() {
    let config = EditorConfig {
        show_whitespace: true,
        highlight_current_line: false,
        bracket_matching: false,
        ..Default::default()
    };
    // Spaces only: a tab is drawn zero columns wide, so its `→` never shows
    // (#982)
    let editor = CodeEditor::new()
        .content("x a  b")
        .line_numbers(false)
        .focused(false)
        .config(config);
    let buffer = screen(&editor, 10, 1);
    assert_eq!(row(&buffer, 0), "x·a··b");
    // Markers in the line-number color, text in the text color
    assert_eq!(buffer.get(1, 0).unwrap().fg, Some(editor.line_number_fg));
    assert_eq!(buffer.get(4, 0).unwrap().fg, Some(editor.line_number_fg));
    assert_eq!(buffer.get(2, 0).unwrap().fg, editor.fg);
    // Off by default: blanks stay blank
    let buffer = screen(&plain("x a").focused(false), 10, 1);
    assert_eq!(row(&buffer, 0), "x a");
}

#[test]
fn test_code_editor_find_matches_highlighted_current_apart() {
    let mut editor = plain("ab ab").focused(false);
    editor.set_find_query("AB");
    let buffer = screen(&editor, 10, 1);
    let bgs: Vec<_> = (0..5).map(|x| buffer.get(x, 0).unwrap().bg).collect();
    let (cur, other) = (Some(editor.current_find_bg), Some(editor.find_match_bg));
    assert_eq!(bgs, vec![cur, cur, editor.bg, other, other]);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::BLACK));
    editor.find_next();
    let buffer = screen(&editor.clone().focused(false), 10, 1);
    let bgs: Vec<_> = (0..5).map(|x| buffer.get(x, 0).unwrap().bg).collect();
    assert_eq!(bgs, vec![other, other, editor.bg, cur, cur]);
}

#[test]
fn test_code_editor_matching_bracket_bold_on_its_color() {
    let editor = CodeEditor::new()
        .content("(a)")
        .line_numbers(false)
        .highlight_current_line(false);
    let buffer = screen(&editor, 10, 1);
    // The cursor on '(' and the ')' it matches
    assert_eq!(buffer.get(0, 0).unwrap().bg, Some(editor.cursor_bg));
    let close = buffer.get(2, 0).unwrap();
    assert_eq!(
        (close.symbol, close.bg, close.fg),
        (')', Some(editor.bracket_match_bg), Some(Color::BLACK))
    );
    assert!(close.modifier.contains(Modifier::BOLD));
    // Not with bracket matching off
    let buffer = screen(&editor.bracket_matching(false), 10, 1);
    assert_eq!(buffer.get(2, 0).unwrap().bg, buffer.get(1, 0).unwrap().bg);
    assert!(!buffer.get(2, 0).unwrap().modifier.contains(Modifier::BOLD));
}

#[test]
fn test_code_editor_unfocused_draws_no_cursor_or_band() {
    let editor = CodeEditor::new().content("ab\ncd").focused(false);
    let buffer = screen(&editor, 10, 3);
    for y in 0..3 {
        for x in 0..10 {
            let bg = buffer.get(x, y).unwrap().bg;
            assert_ne!(bg, Some(editor.cursor_bg), "{x},{y}");
            assert_ne!(bg, Some(editor.current_line_bg), "{x},{y}");
        }
    }
    // The cursor line's number is not white either
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(editor.line_number_fg));
}

#[test]
fn test_code_editor_find_box_shows_query_and_count() {
    let mut editor = CodeEditor::new().content("abc\nxabx");
    editor.open_find();
    for ch in "ab".chars() {
        editor.handle_key(&Key::Char(ch));
    }
    let buffer = screen(&editor, 40, 4);
    // Centered on row 0: 30 columns from x = 5, the count at its right end
    assert_eq!(row(&buffer, 0), format!("{:<32}1/2", "     Find: ab"));
    let label = buffer.get(5, 0).unwrap();
    assert_eq!(
        (label.fg, label.bg),
        (Some(Color::WHITE), Some(Color::rgb(49, 50, 68)))
    );
    assert_eq!(
        buffer.get(11, 0).unwrap().fg,
        Some(Color::rgb(166, 227, 161))
    );
    assert_eq!(
        buffer.get(33, 0).unwrap().fg,
        Some(Color::rgb(180, 190, 254))
    );
    assert_eq!(buffer.get(34, 0).unwrap().bg, Some(Color::rgb(49, 50, 68)));
    assert_eq!(buffer.get(35, 0).unwrap().bg, editor.bg);
    // The lines start below it
    assert_eq!(row(&buffer, 1), "1  abc");
    // A long query scrolls to its end, before the count, with a cell for
    // the cursor after it
    for ch in "cdefghijklmnopqrstuvwxyz".chars() {
        editor.handle_key(&Key::Char(ch));
    }
    let buffer = screen(&editor, 40, 4);
    assert_eq!(
        row(&buffer, 0),
        format!("{:<32}0/0", "     Find: lmnopqrstuvwxyz")
    );
    assert_eq!(
        buffer.get(26, 0).unwrap().bg,
        Some(Color::rgb(166, 227, 161))
    );
}

/// The find and go-to-line boxes only grew at the end and shrank from it:
/// they are edited as every single-line field is (#985)
#[test]
fn test_code_editor_dialog_boxes_are_edit_fields() {
    use revue::event::KeyEvent;
    let mut editor = CodeEditor::new().content("one two\nthree").focused(true);
    editor.open_find();
    for k in [Key::Char('t'), Key::Char('o'), Key::Left, Key::Char('w')] {
        editor.handle_key(&k);
    }
    assert_eq!(editor.find_match_count(), 1, "the query is \"two\"");
    // Ctrl+Backspace deletes "tw" before the cursor: the query is "o"
    editor.handle_key_event(&KeyEvent::ctrl(Key::Backspace));
    assert_eq!(editor.find_match_count(), 2);
    // Typed at the cursor, the start: "ho"
    editor.handle_key(&Key::Char('h'));
    assert_eq!(editor.find_match_count(), 0);
    editor.handle_key(&Key::Escape);

    // The line number: digits only, edited in place
    let text: Vec<String> = (1..=20).map(|i| format!("line {i}")).collect();
    let mut editor = CodeEditor::new().content(text.join("\n")).focused(true);
    editor.open_goto_line();
    for k in [Key::Char('1'), Key::Char('x'), Key::Char('2')] {
        editor.handle_key(&k);
    }
    editor.handle_key(&Key::Enter);
    assert_eq!(
        editor.cursor_position().0,
        11,
        "went to line 12, the x not typed"
    );
    editor.open_goto_line();
    for k in [Key::Char('1'), Key::Char('5'), Key::Home, Key::Delete] {
        editor.handle_key(&k);
    }
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.cursor_position().0, 4, "went to line 5");
}

#[test]
fn test_code_editor_goto_box_shows_typed_number() {
    let mut editor = CodeEditor::new().content("a\nb");
    editor.open_goto_line();
    for ch in "12".chars() {
        editor.handle_key(&Key::Char(ch));
    }
    let buffer = screen(&editor, 30, 3);
    assert_eq!(row(&buffer, 0), "     Go to line: 12");
    assert_eq!(buffer.get(5, 0).unwrap().bg, Some(Color::rgb(49, 50, 68)));
    assert_eq!(buffer.get(24, 0).unwrap().bg, Some(Color::rgb(49, 50, 68)));
    assert_eq!(buffer.get(25, 0).unwrap().bg, editor.bg);
    assert_eq!(row(&buffer, 1), "1  a");
    // In a single row the box is drawn over the line's row
    let buffer = screen(&editor, 30, 1);
    assert_eq!(row(&buffer, 0), "1  a Go to line: 12");
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char('한')),
        Just(Key::Char(' ')),
        Just(Key::Enter),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Home),
        Just(Key::End),
        Just(Key::PageUp),
        Just(Key::PageDown),
    ]
}

/// Start texts: empty, wide glyphs, a line longer than any view, many lines
fn start() -> impl Strategy<Value = String> {
    prop_oneof![
        Just(String::new()),
        Just("ab\n한글한글\n\n  x".to_string()),
        Just(format!("{}한\nshort", "w".repeat(70))),
        Just((0..40).map(|i| format!("line {i}\n")).collect::<String>()),
    ]
}

proptest! {
    /// After any keys, at any size with room for text, the cursor is drawn,
    /// on the row whose number is the cursor's line.
    #[test]
    fn prop_code_editor_cursor_stays_in_view(
        text in start(),
        keys in prop::collection::vec(key(), 0..40),
        w in 5u16..30,
        h in 1u16..8,
        numbers in any::<bool>(),
    ) {
        let mut editor = CodeEditor::new().content(text.as_str()).line_numbers(numbers);
        for (i, k) in keys.iter().enumerate() {
            editor.handle_key(k);
            // Render now and then, as an app does, so the scroll is kept
            if i % 3 == 0 {
                screen(&editor, w, h);
            }
        }
        let gutter = editor.line_number_width();
        prop_assume!(w > gutter + 1);
        let buffer = screen(&editor, w, h);
        let cursor_rows: Vec<u16> = (0..h)
            .filter(|&y| (gutter..w).any(|x| buffer.get(x, y).unwrap().bg == Some(editor.cursor_bg)))
            .collect();
        prop_assert_eq!(cursor_rows.len(), 1, "{:?}", (0..h).map(|y| row(&buffer, y)).collect::<Vec<_>>());
        if numbers {
            let number = row(&buffer, cursor_rows[0]);
            let number = number.split_whitespace().next().unwrap_or("");
            prop_assert_eq!(number, (editor.cursor_position().0 + 1).to_string());
        }
    }
}
