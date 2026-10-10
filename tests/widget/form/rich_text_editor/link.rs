//! Links and image references in the rich text editor

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::traits::View;
use revue::widget::EditorViewMode;
use revue::widget::ImageRef;
use revue::widget::MarkdownLink as Link;
use revue::widget::RichTextEditor;

#[test]
fn test_link_to_markdown_with_special_chars() {
    let link = Link::new("Click (here)", "https://example.com?param=value");
    assert_eq!(
        link.to_markdown(),
        "[Click (here)](https://example.com?param=value)"
    );
}

#[test]
fn test_link_to_markdown_empty_text() {
    let link = Link::new("", "https://example.com");
    assert_eq!(link.to_markdown(), "[](https://example.com)");
}

#[test]
fn test_link_to_markdown_empty_url() {
    let link = Link::new("Text", "");
    assert_eq!(link.to_markdown(), "[Text]()");
}

#[test]
fn test_link_accepts_owned_strings() {
    let link = Link::new(String::from("Text"), String::from("https://example.com"))
        .with_title(String::from("Title"));
    assert_eq!(link.to_markdown(), "[Text](https://example.com \"Title\")");
}

#[test]
fn test_image_ref_to_markdown_with_special_chars() {
    let img = ImageRef::new("A (great) photo", "path/to/image.png");
    assert_eq!(img.to_markdown(), "![A (great) photo](path/to/image.png)");
}

#[test]
fn test_image_ref_to_markdown_empty_alt() {
    let img = ImageRef::new("", "image.png");
    assert_eq!(img.to_markdown(), "![](image.png)");
}

#[test]
fn test_image_ref_to_markdown_empty_src() {
    let img = ImageRef::new("Alt", "");
    assert_eq!(img.to_markdown(), "![Alt]()");
}

#[test]
fn test_image_ref_accepts_owned_strings() {
    let img = ImageRef::new(String::from("Photo"), String::from("photo.jpg"))
        .with_title(String::from("A photo"));
    assert_eq!(img.to_markdown(), "![Photo](photo.jpg \"A photo\")");
}

#[test]
fn test_insert_link() {
    let mut editor = RichTextEditor::new();
    editor.insert_link("Google", "https://google.com");
    assert_eq!(editor.get_content(), "[Google](https://google.com)");
}

#[test]
fn test_insert_image() {
    let mut editor = RichTextEditor::new();
    editor.insert_image("Logo", "logo.png");
    assert_eq!(editor.get_content(), "![Logo](logo.png)");
}

#[test]
fn test_dialog_open_close() {
    let mut editor = RichTextEditor::new();
    assert!(!editor.is_dialog_open());

    editor.open_link_dialog();
    assert!(editor.is_dialog_open());

    editor.close_dialog();
    assert!(!editor.is_dialog_open());

    editor.open_image_dialog();
    assert!(editor.is_dialog_open());

    editor.close_dialog();
    assert!(!editor.is_dialog_open());
}

// Found by tests/event_sequences.rs: the shrunk sequence was
// `[2x1] <link dialog>` - an open dialog in an area too narrow for it.
#[test]
fn test_dialog_in_a_tiny_area() {
    for (w, h) in [(0, 0), (1, 1), (2, 1), (4, 3), (5, 2), (6, 8)] {
        let mut editor = RichTextEditor::new().content("text");
        editor.open_link_dialog();
        let mut buffer = Buffer::new(w.max(1), h.max(1));
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        editor.render(&mut ctx);
        editor.close_dialog();
        editor.open_image_dialog();
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
        editor.render(&mut ctx);
    }
}

/// Paint `editor` and return the fg of each cell of the row holding `needle`,
/// starting at its first character.
fn fgs_at(editor: &RichTextEditor, needle: &str) -> Vec<(char, Option<revue::style::Color>)> {
    let mut buffer = Buffer::new(60, 6);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 60, 6));
    editor.render(&mut ctx);
    for y in 0..6 {
        let row: String = (0..60).map(|x| buffer.get(x, y).unwrap().symbol).collect();
        if let Some(byte) = row.find(needle) {
            let start = row[..byte].chars().count() as u16;
            return (start..start + needle.chars().count() as u16)
                .map(|x| {
                    let c = buffer.get(x, y).unwrap();
                    (c.symbol, c.fg)
                })
                .collect();
        }
    }
    panic!("{needle:?} not painted");
}

/// #799: `link_fg` was stored and never read.
#[test]
fn test_link_fg_colors_links() {
    use revue::style::Color;
    const LINK: Color = Color {
        r: 250,
        g: 10,
        b: 10,
        a: 255,
    };
    for mode in [EditorViewMode::Editor, EditorViewMode::Preview] {
        let editor = RichTextEditor::new()
            .toolbar(false)
            .view_mode(mode)
            .content("see [docs](http://x) and ![img](a.png)")
            .link_fg(LINK);

        let cells = fgs_at(&editor, "see [docs](http://x) and ![img](a.png)");
        for (i, (ch, fg)) in cells.iter().enumerate() {
            let in_link = (4..20).contains(&i);
            assert_eq!(
                *fg == Some(LINK),
                in_link,
                "{mode:?}: {ch:?} at {i} in_link={in_link} fg={fg:?}"
            );
        }
    }
}
