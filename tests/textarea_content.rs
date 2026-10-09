//! `TextArea::set_content` keeps the text: `get_content` gives back what
//! was set, final line break included, so an editor that saves its text
//! does not drop a file's last newline.

use revue::widget::TextArea;

fn round_trip(text: &str) -> String {
    let mut ta = TextArea::new();
    ta.set_content(text);
    ta.get_content()
}

#[test]
fn a_final_newline_is_kept() {
    assert_eq!(round_trip("fn main() {}\n"), "fn main() {}\n");
    assert_eq!(round_trip("a\nb\n"), "a\nb\n");
}

#[test]
fn trailing_blank_lines_are_kept() {
    assert_eq!(round_trip("a\n\n"), "a\n\n");
    assert_eq!(round_trip("\n"), "\n");
}

#[test]
fn a_final_newline_leaves_an_empty_last_line_for_the_cursor() {
    let mut ta = TextArea::new();
    ta.set_content("a\n");
    assert_eq!(ta.line_count(), 2);
}

#[test]
fn text_without_a_final_newline_is_unchanged() {
    assert_eq!(round_trip(""), "");
    assert_eq!(round_trip("a"), "a");
    assert_eq!(round_trip("a\n\nb"), "a\n\nb");
}

#[test]
fn crlf_line_breaks_read_as_lines() {
    let mut ta = TextArea::new();
    ta.set_content("a\r\nb\r\n");
    assert_eq!(ta.line_count(), 3);
    assert_eq!(ta.get_content(), "a\nb\n");
}

#[test]
fn the_builder_keeps_it_too() {
    assert_eq!(TextArea::new().content("x\n").get_content(), "x\n");
}
