//! What the picker draws: placement, each row's text and color, the path,
//! the scroll markers and each mode's footer

use super::{fixture, highlight};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::{DARK_GRAY, DISABLED_FG, LIGHT_GRAY, PLACEHOLDER_FG};
use revue::widget::traits::RenderContext;
use revue::widget::{dir_picker, file_picker, save_picker, FilePicker, View};
use std::fs;

fn draw(picker: &FilePicker, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    picker.render(&mut ctx);
    buffer
}

/// Row `y` as text, a wide glyph once (its continuation cell left out),
/// trailing blanks trimmed.
fn row(buffer: &Buffer, y: u16) -> String {
    let text: String = (0..buffer.width())
        .filter_map(|x| buffer.get(x, y))
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect();
    text.trim_end().to_string()
}

fn rows(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height()).map(|y| row(buffer, y)).collect()
}

fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().fg
}

fn bold(buffer: &Buffer, x: u16, y: u16) -> bool {
    buffer.get(x, y).unwrap().modifier.contains(Modifier::BOLD)
}

/// An entry row as the picker formats it: mark, icon, the name padded to 32
/// and the size right-aligned in 10 (trailing blanks trimmed).
fn entry(mark: &str, dir: bool, name: &str, size: &str) -> String {
    let icon = if dir { "📁" } else { "📄" };
    format!("{mark}{icon} {name:<32} {size:>10}")
        .trim_end()
        .to_string()
}

#[test]
fn test_filepicker_zero_areas_and_offset() {
    let dir = fixture();
    for picker in [
        file_picker(),
        save_picker(),
        dir_picker(),
        FilePicker::multi_select(),
    ] {
        let picker = picker.start_dir(dir.path()).title("T");
        assert_zero_areas_draw_nothing(&picker);
        assert_same_when_offset(picker, 60, 14);
    }
}

#[test]
fn test_filepicker_rows_text() {
    let dir = fixture();
    let picker = file_picker().start_dir(dir.path()).show_hidden(true);
    let buffer = draw(&picker, 70, 12);

    assert_eq!(
        rows(&buffer)[2..9],
        [
            "  📁 ..".to_string(),
            entry("  ", true, "zdir", ""),
            entry("  ", false, ".hidden", "0 B"),
            entry("  ", false, "A.rs", "12 B"),
            entry("  ", false, "b.txt", "5 B"),
            "─".repeat(60),
            "↑↓: Navigate | Enter: Select/Open | Backspace: Parent | h: Hidden | q: Cancel"
                .chars()
                .take(70)
                .collect::<String>(),
        ]
    );
}

#[test]
fn test_filepicker_row_colors() {
    let dir = fixture();
    let mut picker = file_picker().start_dir(dir.path()).show_hidden(true);
    let buffer = draw(&picker, 70, 12);

    assert_eq!(fg(&buffer, 1, 0), Some(Color::CYAN), "path");
    assert_eq!(fg(&buffer, 0, 1), Some(DARK_GRAY), "separator");
    assert_eq!(fg(&buffer, 2, 2), Some(LIGHT_GRAY), "parent row");
    // zdir is highlighted: cyan and bold, not the directory blue.
    assert_eq!(fg(&buffer, 5, 3), Some(Color::CYAN));
    assert!(bold(&buffer, 5, 3));
    assert_eq!(fg(&buffer, 5, 4), Some(PLACEHOLDER_FG), ".hidden");
    assert!(!bold(&buffer, 5, 4));
    assert_eq!(fg(&buffer, 5, 5), Some(Color::WHITE), "A.rs");
    assert!(!bold(&buffer, 5, 5));
    assert_eq!(fg(&buffer, 0, 7), Some(DARK_GRAY), "lower separator");
    assert_eq!(fg(&buffer, 0, 8), Some(DARK_GRAY), "help");

    picker.highlight_next();
    let buffer = draw(&picker, 70, 12);
    assert_eq!(fg(&buffer, 5, 3), Some(Color::BLUE), "zdir");
    assert!(!bold(&buffer, 5, 3));
    assert_eq!(fg(&buffer, 5, 4), Some(Color::CYAN), "highlighted .hidden");
    assert!(bold(&buffer, 5, 4));
}

#[test]
fn test_filepicker_separators_follow_width() {
    let dir = fixture();
    let picker = file_picker().start_dir(dir.path()).width(20);
    let buffer = draw(&picker, 70, 10);
    assert_eq!(row(&buffer, 1), "─".repeat(20));
    assert_eq!(row(&buffer, 6), "─".repeat(20));
}

#[test]
fn test_filepicker_path_is_cut_from_the_left() {
    let dir = tempfile::tempdir().unwrap();
    let leaf = dir.path().join("some_directory").join("leaf_directory");
    fs::create_dir_all(&leaf).unwrap();
    let full = leaf.display().to_string();

    // Room for the whole path: shown after one blank.
    let picker = file_picker().start_dir(&leaf).width(200);
    assert_eq!(row(&draw(&picker, 220, 6), 0), format!(" {full}"));

    // Width 24 leaves 20 columns: "..." and the last 17 characters.
    let picker = file_picker().start_dir(&leaf).width(24);
    let tail: String = full.chars().skip(full.chars().count() - 17).collect();
    assert_eq!(row(&draw(&picker, 40, 6), 0), format!(" ...{tail}"));
    assert!(tail.ends_with("leaf_directory"));
}

#[test]
fn test_filepicker_long_names_are_cut() {
    let dir = tempfile::tempdir().unwrap();
    let thirty = "abcdefghijklmnopqrstuvwxyz.txt";
    let forty = "a_name_that_is_forty_characters_long.txt";
    assert_eq!((thirty.len(), forty.len()), (30, 40));
    fs::write(dir.path().join(thirty), "").unwrap();
    fs::write(dir.path().join(forty), "").unwrap();
    let picker = file_picker().start_dir(dir.path());
    let buffer = draw(&picker, 70, 8);

    assert_eq!(
        row(&buffer, 3),
        entry("  ", false, "a_name_that_is_forty_charac...", "0 B")
    );
    assert_eq!(row(&buffer, 4), entry("  ", false, thirty, "0 B"));
}

#[test]
fn test_filepicker_scroll_markers() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..6 {
        fs::write(dir.path().join(format!("f{i}.txt")), "").unwrap();
    }
    let mut picker = file_picker().start_dir(dir.path()).max_visible(3);

    let buffer = draw(&picker, 60, 12);
    assert_eq!(row(&buffer, 3), entry("  ", false, "f0.txt", "0 B"));
    assert_eq!(row(&buffer, 5), entry("  ", false, "f2.txt", "0 B"));
    assert_eq!(row(&buffer, 6), "  ↓ more...");
    assert_eq!(fg(&buffer, 2, 6), Some(DISABLED_FG));
    assert_eq!(row(&buffer, 7), "─".repeat(60));

    highlight(&mut picker, "f3.txt");
    let buffer = draw(&picker, 60, 12);
    assert_eq!(row(&buffer, 3), "  ↑ more...");
    assert_eq!(fg(&buffer, 2, 3), Some(DISABLED_FG));
    assert_eq!(row(&buffer, 4), entry("  ", false, "f1.txt", "0 B"));
    assert_eq!(row(&buffer, 6), entry("  ", false, "f3.txt", "0 B"));
    assert_eq!(row(&buffer, 7), "  ↓ more...");

    highlight(&mut picker, "f5.txt");
    let buffer = draw(&picker, 60, 12);
    assert_eq!(row(&buffer, 3), "  ↑ more...");
    assert_eq!(row(&buffer, 6), entry("  ", false, "f5.txt", "0 B"));
    assert_eq!(row(&buffer, 7), "─".repeat(60));
}

#[test]
fn test_filepicker_max_visible_zero_shows_only_the_marker() {
    let dir = fixture();
    let picker = file_picker().start_dir(dir.path()).max_visible(0);
    let buffer = draw(&picker, 60, 8);
    assert_eq!(row(&buffer, 2), "  📁 ..");
    assert_eq!(row(&buffer, 3), "  ↓ more...");
    assert_eq!(row(&buffer, 4), "─".repeat(60));
}

#[test]
fn test_filepicker_empty_directory_view() {
    let dir = tempfile::tempdir().unwrap();
    let picker = file_picker().start_dir(dir.path());
    let buffer = draw(&picker, 90, 8);
    assert_eq!(row(&buffer, 2), "  📁 ..");
    assert_eq!(row(&buffer, 3), "─".repeat(60));
    assert!(row(&buffer, 4).starts_with("↑↓: Navigate"));
    assert_eq!(row(&buffer, 5), "");
}

#[test]
fn test_filepicker_help_line_per_mode() {
    let dir = tempfile::tempdir().unwrap();
    let cases = [
        (
            file_picker(),
            "↑↓: Navigate | Enter: Select/Open | Backspace: Parent | h: Hidden | q: Cancel",
        ),
        (
            save_picker(),
            "↑↓: Navigate | Enter: Save | Type: Filename | Backspace: Delete | q: Cancel",
        ),
        (
            dir_picker(),
            "↑↓: Navigate | Enter: Open | Space: Select | Backspace: Parent | q: Cancel",
        ),
        (
            FilePicker::multi_select(),
            "↑↓: Navigate | Space: Toggle | Enter: Confirm | a: All | n: None | q: Cancel",
        ),
    ];
    for (picker, help) in cases {
        let buffer = draw(&picker.start_dir(dir.path()), 90, 10);
        let rows = rows(&buffer);
        let last = rows.iter().rposition(|r| !r.is_empty()).unwrap();
        assert_eq!(rows[last], help);
        assert_eq!(fg(&buffer, 0, last as u16), Some(DARK_GRAY));
    }
}

#[test]
fn test_filepicker_title_is_bold_and_first() {
    let dir = tempfile::tempdir().unwrap();
    let picker = file_picker().start_dir(dir.path()).title("Open File");
    let buffer = draw(&picker, 90, 8);
    assert_eq!(row(&buffer, 0), "Open File");
    assert!(bold(&buffer, 0, 0));
    assert_eq!(
        fg(&buffer, 1, 1),
        Some(Color::CYAN),
        "the path follows the title"
    );
    assert_eq!(row(&buffer, 3), "  📁 ..");

    // The constructors' titles.
    for (picker, title) in [
        (save_picker(), "Save File"),
        (dir_picker(), "Select Directory"),
        (FilePicker::multi_select(), "Select Files"),
    ] {
        let buffer = draw(&picker.start_dir(dir.path()), 90, 8);
        assert_eq!(row(&buffer, 0), title);
    }
}

#[test]
fn test_filepicker_save_footer_shows_the_typed_name() {
    let dir = tempfile::tempdir().unwrap();
    let mut picker = save_picker().start_dir(dir.path());
    let buffer = draw(&picker, 90, 10);
    // title, path, separator, "..", separator, filename, help
    assert_eq!(row(&buffer, 5), "Filename: _");
    assert_eq!(fg(&buffer, 0, 5), Some(Color::YELLOW));

    picker.input_char('한');
    picker.input_char('b');
    let buffer = draw(&picker, 90, 10);
    assert_eq!(row(&buffer, 5), "Filename: 한b_");

    let picker = save_picker()
        .start_dir(dir.path())
        .default_name("untitled.rs");
    assert_eq!(row(&draw(&picker, 90, 10), 5), "Filename: untitled.rs_");
}

#[test]
fn test_filepicker_multi_select_footer_and_marks() {
    let dir = fixture();
    let mut picker = FilePicker::multi_select().start_dir(dir.path());
    let buffer = draw(&picker, 90, 12);
    // No count line while nothing is selected: the help follows the separator.
    assert_eq!(row(&buffer, 7), "─".repeat(60));
    assert!(row(&buffer, 8).starts_with("↑↓: Navigate | Space: Toggle"));

    highlight(&mut picker, "b.txt");
    picker.toggle_selection();
    highlight(&mut picker, "A.rs");
    let buffer = draw(&picker, 90, 12);
    assert_eq!(row(&buffer, 5), entry("  ", false, "A.rs", "12 B"));
    assert_eq!(row(&buffer, 6), entry("✓ ", false, "b.txt", "5 B"));
    assert!(row(&buffer, 8).starts_with("Selected: 1 "));
    assert_eq!(fg(&buffer, 0, 8), Some(Color::GREEN));
    assert!(row(&buffer, 9).starts_with("↑↓: Navigate | Space: Toggle"));

    picker.toggle_selection();
    let buffer = draw(&picker, 90, 12);
    assert_eq!(row(&buffer, 5), entry("✓ ", false, "A.rs", "12 B"));
    assert_eq!(row(&buffer, 8), "Selected: 2 files");
}

#[test]
fn test_filepicker_shows_fifteen_entries_by_default() {
    let dir = tempfile::tempdir().unwrap();
    for i in 0..20 {
        fs::write(dir.path().join(format!("f{i:02}.txt")), "").unwrap();
    }
    let picker = file_picker().start_dir(dir.path());
    let buffer = draw(&picker, 60, 24);
    assert_eq!(row(&buffer, 3), entry("  ", false, "f00.txt", "0 B"));
    assert_eq!(row(&buffer, 17), entry("  ", false, "f14.txt", "0 B"));
    assert_eq!(row(&buffer, 18), "  ↓ more...");
}
