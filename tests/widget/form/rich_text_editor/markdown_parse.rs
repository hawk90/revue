//! Markdown Parsing tests

use revue::widget::{BlockType, RichTextEditor};

#[test]
fn test_from_markdown_heading() {
    let editor = RichTextEditor::new().from_markdown("# Hello");
    assert_eq!(editor.current_block_type(), BlockType::Heading1);
    assert_eq!(editor.get_content(), "Hello");
}

#[test]
fn test_from_markdown_heading2() {
    let editor = RichTextEditor::new().from_markdown("## Section");
    assert_eq!(editor.current_block_type(), BlockType::Heading2);
    assert_eq!(editor.get_content(), "Section");
}

#[test]
fn test_from_markdown_quote() {
    let editor = RichTextEditor::new().from_markdown("> Quote");
    assert_eq!(editor.current_block_type(), BlockType::Quote);
    assert_eq!(editor.get_content(), "Quote");
}

#[test]
fn test_from_markdown_bullet_list() {
    let editor = RichTextEditor::new().from_markdown("- Item");
    assert_eq!(editor.current_block_type(), BlockType::BulletList);
    assert_eq!(editor.get_content(), "Item");
}

#[test]
fn test_from_markdown_horizontal_rule() {
    let editor = RichTextEditor::new().from_markdown("---");
    assert_eq!(editor.current_block_type(), BlockType::HorizontalRule);
}

// =========================================================================
// #1076: fences, numbered lines, code rows, preview numbering and headings
// =========================================================================

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::widget::traits::{RenderContext, View};
use revue::widget::EditorViewMode;

fn screen_1076(editor: &RichTextEditor, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    editor.render(&mut ctx);
    buffer
}

fn row_1076(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[test]
fn unclosed_fence_keeps_its_lines() {
    let editor = RichTextEditor::new().from_markdown("intro\n```\nlet x = 1;");
    assert!(
        editor.get_content().contains("let x = 1;"),
        "{:?}",
        editor.get_content()
    );
}

#[test]
fn digit_line_is_not_a_list() {
    let editor = RichTextEditor::new().from_markdown("2024 was good. Really");
    assert_eq!(editor.current_block_type(), BlockType::Paragraph);
    assert_eq!(editor.get_content(), "2024 was good. Really");
}

#[test]
fn multiline_code_block_rows() {
    let editor = RichTextEditor::new().from_markdown("```\na\nb\n```");
    assert_eq!(row_1076(&screen_1076(&editor, 20, 4), 1), "` a");
}

#[test]
fn preview_numbers_items() {
    let editor = RichTextEditor::new()
        .from_markdown("Steps:\n1. one\n2. two")
        .view_mode(EditorViewMode::Preview);
    assert_eq!(row_1076(&screen_1076(&editor, 20, 4), 2), "1. one");
}

#[test]
fn preview_heading4() {
    let editor = RichTextEditor::new()
        .from_markdown("#### Four")
        .view_mode(EditorViewMode::Preview);
    let buffer = screen_1076(&editor, 20, 3);
    assert!(buffer.get(0, 1).unwrap().modifier.contains(Modifier::BOLD));
}

/// Export makes one fence of the lines read from one, an unclosed one
/// closed; fences of different languages stay apart (#1076)
#[test]
fn code_fences_round_trip() {
    let editor = RichTextEditor::new().from_markdown("intro\n```\nlet x = 1;");
    assert_eq!(editor.to_markdown(), "intro\n```\nlet x = 1;\n```");
    let md = "```rust\na\n```\n```py\nb\n```";
    assert_eq!(RichTextEditor::new().from_markdown(md).to_markdown(), md);
    let empty = RichTextEditor::new().from_markdown("```\n```");
    assert_eq!(empty.block_count(), 1);
    assert_eq!(empty.to_markdown(), "```\n\n```");
}

/// A digit line is a list item only when it is digits then ". "; numbering
/// starts again in each list
#[test]
fn numbered_items_by_their_marker_and_list() {
    for (md, ty) in [
        ("12. twelve", BlockType::NumberedList),
        ("1.5 is not", BlockType::Paragraph),
        ("3 . no", BlockType::Paragraph),
        ("2024 was good. Really", BlockType::Paragraph),
    ] {
        assert_eq!(
            RichTextEditor::new().from_markdown(md).current_block_type(),
            ty,
            "{md}"
        );
    }
    let editor = RichTextEditor::new()
        .from_markdown("1. a\n2. b\ntext\n1. c")
        .view_mode(EditorViewMode::Preview);
    let buffer = screen_1076(&editor, 20, 6);
    let rows: Vec<String> = (1..5).map(|y| row_1076(&buffer, y)).collect();
    assert_eq!(rows, ["1. a", "2. b", "text", "1. c"]);
}
