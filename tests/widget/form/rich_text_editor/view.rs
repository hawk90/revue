//! RichTextEditor on the screen: placement, the smallest areas, stylesheet
//! colors, the toolbar's active formats, selections and wide text

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{Color, Style, VisualStyle};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{EditorViewMode, RichTextEditor, ToolbarAction};

use super::{
    cells_with_bg, row, screen, BG, CURSOR_BG, DIALOG_BG, FG, HEADING_FG, PREVIEW_BG, SELECTION_BG,
    TOOLBAR, TOOLBAR_BG,
};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const DOC: &str = "# Title\n> quote [l](u)\n- item\n```\ncode\n```\n---\n한글 text";

fn editors() -> Vec<RichTextEditor> {
    let doc = || RichTextEditor::new().from_markdown(DOC);
    let mut selected = doc();
    selected.set_cursor(1, 2);
    selected.start_selection();
    selected.set_cursor(2, 3);
    let mut dialog = doc().view_mode(EditorViewMode::Split);
    dialog.open_link_dialog();
    vec![
        RichTextEditor::new(),
        doc(),
        doc().view_mode(EditorViewMode::Preview),
        doc().view_mode(EditorViewMode::Split).toolbar(false),
        doc().focused(false),
        selected,
        dialog,
    ]
}

#[test]
fn test_rich_text_editor_zero_areas_draw_nothing() {
    for editor in editors() {
        assert_zero_areas_draw_nothing(&editor);
    }
}

#[test]
fn test_rich_text_editor_same_when_offset() {
    for editor in editors() {
        assert_same_when_offset(editor.clone(), 50, 12);
        assert_same_when_offset(editor, 8, 4);
    }
}

#[test]
fn test_rich_text_editor_two_columns_are_the_least_it_draws() {
    let editor = RichTextEditor::new().content("abc");
    assert!((0..1).all(|y| row(&screen(&editor, 1, 3), y).is_empty()));
    assert_eq!(cells_with_bg(&screen(&editor, 1, 3), BG), vec![]);
    let buffer = screen(&editor, 2, 3);
    assert_eq!(row(&buffer, 0), " B");
    assert_eq!(row(&buffer, 1), "ab");
    assert_eq!(row(&buffer, 2), "");
}

#[test]
fn test_rich_text_editor_empty_markdown_is_one_empty_line() {
    let editor = RichTextEditor::new().from_markdown("");
    assert_eq!(
        (editor.block_count(), editor.get_content()),
        (1, String::new())
    );
    let buffer = screen(&editor, 20, 4);
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(0, 1)]);
    let editor = RichTextEditor::new().content("");
    assert_eq!(editor.block_count(), 1);
}

#[test]
fn test_rich_text_editor_stylesheet_colors_reach_the_editor_and_preview() {
    let css = Style {
        visual: VisualStyle {
            color: Color::rgb(7, 8, 9),
            background: Color::rgb(1, 2, 3),
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let draw = |editor: &RichTextEditor| {
        let mut buffer = Buffer::new(40, 4);
        let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, 40, 4), &css);
        editor.render(&mut ctx);
        buffer
    };
    let editor = RichTextEditor::new()
        .content("ab")
        .view_mode(EditorViewMode::Split);
    let buffer = draw(&editor);
    // The editor takes both; the preview keeps its own background
    let b = buffer.get(1, 1).unwrap();
    assert_eq!(
        (b.fg, b.bg),
        (Some(Color::rgb(7, 8, 9)), Some(Color::rgb(1, 2, 3)))
    );
    let p = buffer.get(21, 1).unwrap();
    assert_eq!((p.fg, p.bg), (Some(Color::rgb(7, 8, 9)), Some(PREVIEW_BG)));
    // The builder wins over the stylesheet
    let built = draw(&editor.clone().bg(Color::RED).fg(Color::GREEN));
    let b = built.get(1, 1).unwrap();
    assert_eq!((b.fg, b.bg), (Some(Color::GREEN), Some(Color::RED)));
    assert_eq!(built.get(21, 1).unwrap().fg, Some(Color::GREEN));
}

#[test]
fn test_rich_text_editor_default_colors() {
    let editor = RichTextEditor::new().from_markdown("# Hi\nab");
    let buffer = screen(&editor, 20, 4);
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(HEADING_FG));
    let b = buffer.get(1, 2).unwrap();
    assert_eq!((b.symbol, b.fg, b.bg), ('b', Some(FG), Some(BG)));
}

#[test]
fn test_rich_text_editor_toolbar_shows_the_active_formats() {
    let mut editor = RichTextEditor::new();
    editor.toolbar_action(ToolbarAction::Bold);
    editor.toolbar_action(ToolbarAction::Code);
    let buffer = screen(&editor, 40, 2);
    assert_eq!(row(&buffer, 0), TOOLBAR);
    let active = Color::rgb(137, 180, 250);
    // B (column 1) and ` (column 9) are lit; I, U and S are not
    for (x, on) in [(1, true), (3, false), (5, false), (7, false), (9, true)] {
        let cell = buffer.get(x, 0).unwrap();
        let want = if on {
            (Some(Color::rgb(30, 30, 46)), Some(active))
        } else {
            (Some(Color::rgb(166, 173, 200)), Some(TOOLBAR_BG))
        };
        assert_eq!((cell.fg, cell.bg), want, "column {x}");
    }
}

#[test]
fn test_rich_text_editor_selection_across_lines() {
    let mut editor = RichTextEditor::new().content("abcd\nefgh\nijkl");
    editor.set_cursor(2, 1);
    editor.start_selection();
    editor.set_cursor(0, 2);
    let buffer = screen(&editor, 10, 5);
    // From the cursor at (0, 2) up to the anchor at (2, 1), the anchor's
    // character left out; the cursor cell keeps the cursor color
    let selected: Vec<_> = cells_with_bg(&buffer, SELECTION_BG);
    let mut want = vec![(3, 1)];
    want.extend((0..4).map(|x| (x, 2)));
    want.push((0, 3));
    assert_eq!(selected, want);
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(2, 1)]);
    assert_eq!(editor.get_selection().as_deref(), Some("cd\nefgh\ni"));
}

#[test]
fn test_rich_text_editor_cursor_after_wide_text() {
    let mut editor = RichTextEditor::new().content("한글");
    editor.set_cursor(0, 2);
    let buffer = screen(&editor, 10, 3);
    // Two wide glyphs take four columns; the cursor is the fifth
    assert_eq!(cells_with_bg(&buffer, CURSOR_BG), vec![(4, 1)]);
}

#[test]
fn test_rich_text_editor_dialog_wide_field_scrolls_to_its_end() {
    let mut editor = RichTextEditor::new();
    editor.open_link_dialog();
    for ch in "가나다라마바사아자차카타파하가나".chars() {
        editor.handle_key(&revue::event::Key::Char(ch));
    }
    let buffer = screen(&editor, 60, 20);
    // 16 wide glyphs in a 30-column field: the last 15 are shown, each on
    // two cells, and nothing spills past the field
    let field: String = (18..48)
        .map(|x| buffer.get(x, 9).unwrap())
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect();
    assert_eq!(field, "나다라마바사아자차카타파하가나");
    assert_eq!(buffer.get(48, 9).unwrap().bg, Some(DIALOG_BG));
    assert_eq!(buffer.get(49, 9).unwrap().symbol, '│');
}
