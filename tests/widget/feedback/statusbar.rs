//! StatusBar state read back through its getters
//!
//! tests/widget/statusbar.rs covers StatusSection and KeyHint (new, every
//! builder, width, clone, helpers) and renders each StatusBar builder;
//! these tests check the StatusBar's own state directly.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::{
    footer, header, statusbar, KeyHint, RenderContext, SectionAlign, StatusBar, StatusBarPosition,
    StatusSection, View,
};

fn row(bar: &StatusBar, y: u16) -> String {
    let mut buffer = Buffer::new(40, 5);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 40, 5));
    bar.render(&mut ctx);
    (0..40)
        .map(|x| buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '))
        .collect::<String>()
        .trim_end()
        .to_string()
}

fn contents(sections: &[StatusSection]) -> Vec<&str> {
    sections.iter().map(|s| s.content.as_str()).collect()
}

#[test]
fn test_section_align_default() {
    assert_eq!(SectionAlign::default(), SectionAlign::Left);
    assert_ne!(SectionAlign::Left, SectionAlign::Center);
    assert_ne!(SectionAlign::Center, SectionAlign::Right);
}

#[test]
fn test_status_bar_new() {
    let bar = StatusBar::new();
    assert!(bar.get_left().is_empty());
    assert!(bar.get_center().is_empty());
    assert!(bar.get_right().is_empty());
    assert!(bar.get_key_hints().is_empty());
    assert_eq!(bar.get_height(), 1);
    assert_eq!(bar.get_separator(), None);
    assert_eq!(bar.get_fg(), None);
    assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_default_and_helper_match_new() {
    for bar in [StatusBar::default(), statusbar()] {
        assert!(bar.get_left().is_empty());
        assert_eq!(bar.get_height(), 1);
        assert_eq!(bar.get_bg(), StatusBar::new().get_bg());
        assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
    }
}

#[test]
fn test_status_bar_position_default_and_variants() {
    assert_eq!(StatusBarPosition::default(), StatusBarPosition::Bottom);
    assert_ne!(StatusBarPosition::Top, StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_header_and_footer_positions() {
    assert_eq!(StatusBar::new().get_position(), StatusBarPosition::Bottom);
    assert_eq!(header().get_position(), StatusBarPosition::Top);
    assert_eq!(footer().get_position(), StatusBarPosition::Bottom);
    assert_eq!(
        StatusBar::new().header().get_position(),
        StatusBarPosition::Top
    );
    assert_eq!(header().footer().get_position(), StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_position_builder_moves_the_bar() {
    let top = StatusBar::new()
        .position(StatusBarPosition::Top)
        .left_text("Top");
    assert_eq!(top.get_position(), StatusBarPosition::Top);
    assert_eq!(row(&top, 0), "Top");
    assert_eq!(row(&top, 4), "");

    let bottom = top.position(StatusBarPosition::Bottom);
    assert_eq!(row(&bottom, 0), "");
    assert_eq!(row(&bottom, 4), "Top");
}

#[test]
fn test_status_bar_render_y() {
    assert_eq!(header().get_render_y(24), 0);
    assert_eq!(footer().get_render_y(24), 23);
    assert_eq!(footer().height(2).get_render_y(24), 22);
    assert_eq!(header().height(2).get_render_y(24), 0);
}

#[test]
fn test_status_bar_sections() {
    let bar = StatusBar::new()
        .left(StatusSection::new("Left").bold())
        .left_text("File.txt")
        .center_text("Line 1, Col 1")
        .center(StatusSection::new("Center"))
        .right_text("UTF-8")
        .right(StatusSection::new("Right").fg(Color::RED));

    assert_eq!(contents(bar.get_left()), ["Left", "File.txt"]);
    assert!(bar.get_left()[0].bold);
    assert_eq!(contents(bar.get_center()), ["Line 1, Col 1", "Center"]);
    assert_eq!(contents(bar.get_right()), ["UTF-8", "Right"]);
    assert_eq!(bar.get_right()[1].fg, Some(Color::RED));
}

#[test]
fn test_status_bar_colors() {
    let bar = StatusBar::new().bg(Color::BLUE).fg(Color::YELLOW);
    assert_eq!(bar.get_bg(), Color::BLUE);
    assert_eq!(bar.get_fg(), Some(Color::YELLOW));
}

#[test]
fn test_status_bar_keys() {
    let bar = StatusBar::new()
        .key("^X", "Exit")
        .keys(vec![KeyHint::new("^S", "Save"), KeyHint::new("^O", "Open")])
        .key("^Q", "Quit");
    let hints: Vec<(&str, &str)> = bar
        .get_key_hints()
        .iter()
        .map(|h| (h.key.as_str(), h.description.as_str()))
        .collect();
    assert_eq!(
        hints,
        [
            ("^X", "Exit"),
            ("^S", "Save"),
            ("^O", "Open"),
            ("^Q", "Quit")
        ]
    );
}

#[test]
fn test_status_bar_separator_and_height() {
    let bar = StatusBar::new().separator('|').height(2);
    assert_eq!(bar.get_separator(), Some('|'));
    assert_eq!(bar.get_height(), 2);
    assert_eq!(StatusBar::new().height(0).get_height(), 1);
}

#[test]
fn test_status_bar_update_sections() {
    let mut bar = StatusBar::new()
        .left_text("L0")
        .left_text("L1")
        .center_text("C0")
        .right_text("R0");
    bar.update_left(1, "L1 updated");
    bar.update_center(0, "C0 updated");
    bar.update_right(0, "R0 updated");
    assert_eq!(contents(bar.get_left()), ["L0", "L1 updated"]);
    assert_eq!(contents(bar.get_center()), ["C0 updated"]);
    assert_eq!(contents(bar.get_right()), ["R0 updated"]);
}

#[test]
fn test_status_bar_update_keeps_section_style() {
    let mut bar = StatusBar::new().left(StatusSection::new("Mode").bold().fg(Color::GREEN));
    bar.update_left(0, "INSERT");
    assert_eq!(bar.get_left()[0].content, "INSERT");
    assert!(bar.get_left()[0].bold);
    assert_eq!(bar.get_left()[0].fg, Some(Color::GREEN));
}

#[test]
fn test_status_bar_update_invalid_index() {
    let mut bar = StatusBar::new().left_text("Test");
    bar.update_left(5, "Won't update");
    bar.update_center(0, "No center");
    bar.update_right(3, "No right");
    assert_eq!(contents(bar.get_left()), ["Test"]);
    assert!(bar.get_center().is_empty());
    assert!(bar.get_right().is_empty());
}

#[test]
fn test_status_bar_clear() {
    let mut bar = StatusBar::new()
        .left_text("L")
        .center_text("C")
        .right_text("R")
        .key("Ctrl+S", "Save")
        .separator('|')
        .bg(Color::BLUE);
    bar.clear();
    assert!(bar.get_left().is_empty());
    assert!(bar.get_center().is_empty());
    assert!(bar.get_right().is_empty());
    assert!(bar.get_key_hints().is_empty());
    // Only content is cleared, not configuration.
    assert_eq!(bar.get_separator(), Some('|'));
    assert_eq!(bar.get_bg(), Color::BLUE);
}

#[test]
fn test_status_bar_render_left_and_right_text() {
    let bar = StatusBar::new().left_text("Left").right_text("Right");
    let bottom = row(&bar, 4);
    assert!(bottom.starts_with("Left"), "{bottom:?}");
    assert!(bottom.ends_with("Right"), "{bottom:?}");
    assert_eq!(row(&bar, 0), "");
}

/// #799: the separator only left a gap; its character was never drawn.
#[test]
fn test_status_bar_draws_the_separator_between_sections() {
    let bar = StatusBar::new()
        .height(1)
        .separator('|')
        .left_text("A")
        .left_text("B")
        .right_text("C")
        .right_text("D");
    let text = row(&bar, 4);
    assert!(text.starts_with("A|B"), "left sections: {text:?}");
    assert!(
        !text.starts_with("A|B|"),
        "separator after the last section: {text:?}"
    );
    assert!(text.ends_with("C|D"), "right sections: {text:?}");
}
