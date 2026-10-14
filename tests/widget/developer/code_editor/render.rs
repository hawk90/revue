//! Rendering tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::CodeEditor;

/// The editor drawn in a `w` x `h` buffer
pub(super) fn screen(editor: &CodeEditor, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    editor.render(&mut ctx);
    buffer
}

/// Row `y`'s symbols, trailing blanks trimmed
pub(super) fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn bg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().bg
}

fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().fg
}

#[test]
fn test_render_empty() {
    let editor = CodeEditor::new();
    let buffer = screen(&editor, 40, 10);
    // Line number 1 in a 3-column gutter, the cursor in the first text column
    assert_eq!(row(&buffer, 0), "1");
    assert_eq!(fg(&buffer, 0, 0), Some(Color::WHITE));
    assert_eq!(bg(&buffer, 3, 0), Some(editor.cursor_bg));
    // The rest of the current line is banded, the rows below are plain
    assert_eq!(bg(&buffer, 4, 0), Some(editor.current_line_bg));
    assert_eq!(bg(&buffer, 39, 0), Some(editor.current_line_bg));
    for y in 1..10 {
        assert_eq!(row(&buffer, y), "", "row {y}");
        assert_eq!(bg(&buffer, 0, y), editor.bg, "row {y}");
        assert_eq!(bg(&buffer, 4, y), editor.bg, "row {y}");
    }
}

#[test]
fn test_render_with_content() {
    let editor = CodeEditor::new().content("hello\nworld");
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), "1  hello");
    assert_eq!(row(&buffer, 1), "2  world");
    assert_eq!(row(&buffer, 2), "");
    // The cursor on 'h': black on the cursor color
    assert_eq!(bg(&buffer, 3, 0), Some(editor.cursor_bg));
    assert_eq!(fg(&buffer, 3, 0), Some(Color::BLACK));
    // Other text in the text color: on the band on the cursor's line,
    // on the background below it
    assert_eq!(
        (fg(&buffer, 4, 0), bg(&buffer, 4, 0)),
        (editor.fg, Some(editor.current_line_bg))
    );
    assert_eq!(
        (fg(&buffer, 3, 1), bg(&buffer, 3, 1)),
        (editor.fg, editor.bg)
    );
}

#[test]
fn test_render_with_line_numbers() {
    let editor = CodeEditor::new().content("line1\nline2").line_numbers(true);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), "1  line1");
    assert_eq!(row(&buffer, 1), "2  line2");
    // The cursor line's number is white, the others in the number color
    assert_eq!(fg(&buffer, 0, 0), Some(Color::WHITE));
    assert_eq!(fg(&buffer, 0, 1), Some(editor.line_number_fg));
    assert_eq!(bg(&buffer, 0, 1), editor.bg);

    // Ten lines widen the gutter by a column
    let editor = CodeEditor::new().content("x\n".repeat(10));
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), " 1  x");
    assert_eq!(row(&buffer, 9), "10  x");

    // Without numbers the text starts at column 0
    let editor = CodeEditor::new()
        .content("line1\nline2")
        .line_numbers(false);
    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), "line1");
    assert_eq!(bg(&buffer, 0, 0), Some(editor.cursor_bg));
}

#[test]
fn test_render_with_minimap() {
    let editor = CodeEditor::new()
        .content("fn main() {\n    println!(\"Hello\");\n}")
        .minimap(true);
    let buffer = screen(&editor, 60, 10);
    // The minimap takes the right 10 columns: a mark per non-blank char
    let minimap =
        |y: u16| -> String { (50..60).map(|x| buffer.get(x, y).unwrap().symbol).collect() };
    assert_eq!(minimap(0), "▪▪ ▪▪▪▪▪▪ ");
    assert_eq!(minimap(1), "    ▪▪▪▪▪▪");
    assert_eq!(minimap(2), "▪         ");
    assert_eq!(minimap(3), "          ");
    // Every row shows lines in view: all on the visible-area color (checked
    // between the marks, which are drawn without it: #1082)
    for y in 0..10 {
        for x in 50..60 {
            if buffer.get(x, y).unwrap().symbol == ' ' {
                assert_eq!(
                    bg(&buffer, x, y),
                    Some(editor.minimap_visible_bg),
                    "{x},{y}"
                );
            }
        }
    }
    // The text stops before it: a long line is cut at column 50
    assert_eq!(
        row(&buffer, 0),
        format!("{:<50}{}", "1  fn main() {", "▪▪ ▪▪▪▪▪▪")
    );
    let mut editor = CodeEditor::new().content("y".repeat(80)).minimap(true);
    editor.set_cursor(0, 0);
    let buffer = screen(&editor, 60, 10);
    assert_eq!(
        row(&buffer, 0),
        format!("1  {}{}", "y".repeat(47), "▪".repeat(10))
    );

    // Thirty lines in ten rows: three lines a row, the first four rows
    // (lines 0, 3, 6, 9) in view
    let editor = CodeEditor::new().content("x\n".repeat(30)).minimap(true);
    let buffer = screen(&editor, 60, 10);
    for y in 0..10 {
        let expected = if y < 4 {
            editor.minimap_visible_bg
        } else {
            editor.minimap_bg
        };
        assert_eq!(bg(&buffer, 59, y), Some(expected), "row {y}");
        assert_eq!(buffer.get(50, y).unwrap().symbol, '▪', "row {y}");
    }
}

#[test]
fn test_render_with_selection() {
    let mut editor = CodeEditor::new().content("hello world");
    editor.start_selection();
    editor.set_cursor(0, 5);

    let buffer = screen(&editor, 40, 10);
    assert_eq!(row(&buffer, 0), "1  hello world");
    // "hello" selected, in the text color
    for x in 3..8 {
        assert_eq!(bg(&buffer, x, 0), Some(editor.selection_bg), "col {x}");
        assert_eq!(fg(&buffer, x, 0), editor.fg, "col {x}");
    }
    // The cursor on the space after it, then the plain band
    assert_eq!(bg(&buffer, 8, 0), Some(editor.cursor_bg));
    assert_eq!(bg(&buffer, 9, 0), Some(editor.current_line_bg));
}
