//! Rendering

use super::{fixture, highlight};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::RenderContext;
use revue::widget::{file_picker, save_picker, FilePicker, View};

fn render_text(picker: &FilePicker, width: u16, height: u16) -> String {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    picker.render(&mut ctx);
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn test_filepicker_render_title_and_entries() {
    let dir = fixture();
    let picker = file_picker().start_dir(dir.path()).title("Open File");
    let text = render_text(&picker, 80, 20);

    assert!(text.contains("Open File"), "{text}");
    assert!(text.contains("zdir"), "{text}");
    assert!(text.contains("A.rs"), "{text}");
    assert!(text.contains("b.txt"), "{text}");
    assert!(!text.contains(".hidden"), "{text}");
    assert!(text.contains("Enter: Select/Open"), "{text}");
}

#[test]
fn test_filepicker_render_save_mode_shows_filename() {
    let dir = fixture();
    let mut picker = save_picker().start_dir(dir.path());
    picker.input_char('a');
    let text = render_text(&picker, 80, 20);

    assert!(text.contains("Save File"), "{text}");
    assert!(text.contains("Filename: a_"), "{text}");
}

#[test]
fn test_filepicker_render_multi_select_count() {
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());

    let text = render_text(&picker, 80, 20);
    assert!(!text.contains("Selected:"), "{text}");

    highlight(&mut picker, "A.rs");
    picker.toggle_selection();
    highlight(&mut picker, "b.txt");
    picker.toggle_selection();
    let text = render_text(&picker, 80, 20);
    assert!(text.contains("Selected: 2 files"), "{text}");
    assert!(text.contains('✓'), "{text}");
}

#[test]
fn test_filepicker_render_scrolls_to_highlight() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..10 {
        std::fs::write(dir.path().join(format!("f{i}.txt")), "").unwrap();
    }
    let mut picker = file_picker().start_dir(dir.path()).max_visible(3);

    let text = render_text(&picker, 80, 20);
    assert!(text.contains("f0.txt") && text.contains("f2.txt"), "{text}");
    assert!(!text.contains("f3.txt"), "{text}");
    assert!(text.contains("more..."), "{text}");

    highlight(&mut picker, "f9.txt");
    let text = render_text(&picker, 80, 20);
    assert!(text.contains("f9.txt"), "{text}");
    assert!(!text.contains("f0.txt"), "{text}");
}
