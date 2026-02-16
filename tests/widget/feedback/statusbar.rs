//! Tests for StatusBar widget
//!
//! Extracted from src/widget/feedback/statusbar.rs

use revue::prelude::*;

// =========================================================================
// StatusBarPosition enum tests
// =========================================================================

#[test]
fn test_status_bar_position_default() {
    let pos = StatusBarPosition::default();
    assert_eq!(pos, StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_position_clone() {
    let pos = StatusBarPosition::Top;
    let cloned = pos.clone();
    assert_eq!(pos, cloned);
}

#[test]
fn test_status_bar_position_copy() {
    let pos1 = StatusBarPosition::Bottom;
    let pos2 = pos1;
    assert_eq!(pos1, StatusBarPosition::Bottom);
    assert_eq!(pos2, StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_position_partial_eq() {
    assert_eq!(StatusBarPosition::Top, StatusBarPosition::Top);
    assert_ne!(StatusBarPosition::Top, StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_position_debug() {
    let pos = StatusBarPosition::Top;
    assert!(format!("{:?}", pos).contains("Top"));
}

// =========================================================================
// SectionAlign enum tests
// =========================================================================

#[test]
fn test_section_align_default() {
    let align = SectionAlign::default();
    assert_eq!(align, SectionAlign::Left);
}

#[test]
fn test_section_align_clone() {
    let align = SectionAlign::Center;
    let cloned = align.clone();
    assert_eq!(align, cloned);
}

#[test]
fn test_section_align_copy() {
    let align1 = SectionAlign::Right;
    let align2 = align1;
    assert_eq!(align1, SectionAlign::Right);
    assert_eq!(align2, SectionAlign::Right);
}

#[test]
fn test_section_align_partial_eq() {
    assert_eq!(SectionAlign::Left, SectionAlign::Left);
    assert_ne!(SectionAlign::Left, SectionAlign::Center);
}

#[test]
fn test_section_align_debug() {
    let align = SectionAlign::Center;
    assert!(format!("{:?}", align).contains("Center"));
}

// =========================================================================
// StatusSection tests
// =========================================================================

#[test]
fn test_status_section_new() {
    let section = StatusSection::new("Test");
    assert_eq!(section.content, "Test");
    assert!(section.fg.is_none());
    assert!(section.bg.is_none());
    assert!(!section.bold);
    assert_eq!(section.min_width, 0);
    assert_eq!(section.priority, 0);
}

#[test]
fn test_status_section() {
    let section = StatusSection::new("Hello")
        .fg(Color::WHITE)
        .bold()
        .min_width(10);

    assert_eq!(section.content, "Hello");
    assert_eq!(section.width(), 10);
    assert!(section.bold);
}

#[test]
fn test_status_section_fg() {
    let section = StatusSection::new("Test").fg(Color::RED);
    assert_eq!(section.fg, Some(Color::RED));
}

#[test]
fn test_status_section_bg() {
    let section = StatusSection::new("Test").bg(Color::BLUE);
    assert_eq!(section.bg, Some(Color::BLUE));
}

#[test]
fn test_status_section_bold() {
    let section = StatusSection::new("Test").bold();
    assert!(section.bold);
}

#[test]
fn test_status_section_min_width() {
    let section = StatusSection::new("Test").min_width(20);
    assert_eq!(section.min_width, 20);
}

#[test]
fn test_status_section_priority() {
    let section = StatusSection::new("Test").priority(5);
    assert_eq!(section.priority, 5);
}

#[test]
fn test_status_section_width_content() {
    let section = StatusSection::new("Hello World");
    assert_eq!(section.width(), 11);
}

#[test]
fn test_status_section_width_min_width() {
    let section = StatusSection::new("Hi").min_width(10);
    assert_eq!(section.width(), 10);
}

#[test]
fn test_status_section_clone() {
    let section1 = StatusSection::new("Test").fg(Color::RED).bold().priority(3);
    let section2 = section1.clone();
    assert_eq!(section1.content, section2.content);
    assert_eq!(section1.fg, section2.fg);
    assert_eq!(section1.bold, section2.bold);
    assert_eq!(section1.priority, section2.priority);
}

#[test]
fn test_status_section_builder_chain() {
    let section = StatusSection::new("Chained")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .bold()
        .min_width(15)
        .priority(10);
    assert_eq!(section.content, "Chained");
    assert_eq!(section.fg, Some(Color::CYAN));
    assert_eq!(section.bg, Some(Color::BLACK));
    assert!(section.bold);
    assert_eq!(section.min_width, 15);
    assert_eq!(section.priority, 10);
}

// =========================================================================
// KeyHint tests
// =========================================================================

#[test]
fn test_key_hint_new() {
    let hint = KeyHint::new("Ctrl+S", "Save");
    assert_eq!(hint.key, "Ctrl+S");
    assert_eq!(hint.description, "Save");
}

#[test]
fn test_key_hint_clone() {
    let hint1 = KeyHint::new("Ctrl+Q", "Quit");
    let hint2 = hint1.clone();
    assert_eq!(hint1.key, hint2.key);
    assert_eq!(hint1.description, hint2.description);
}

// =========================================================================
// StatusBar builder tests
// =========================================================================

#[test]
fn test_status_bar_new() {
    let bar = StatusBar::new();
    assert!(bar.get_left().is_empty());
    assert!(bar.get_center().is_empty());
    assert!(bar.get_right().is_empty());
    assert!(bar.get_key_hints().is_empty());
    assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
    assert_eq!(bar.get_height(), 1);
    assert!(bar.get_separator().is_none());
}

#[test]
fn test_status_bar() {
    let bar = StatusBar::new()
        .left_text("File.txt")
        .center_text("Line 1, Col 1")
        .right_text("UTF-8");

    assert_eq!(bar.get_left().len(), 1);
    assert_eq!(bar.get_center().len(), 1);
    assert_eq!(bar.get_right().len(), 1);
}

#[test]
fn test_status_bar_position_builder() {
    let bar = StatusBar::new().position(StatusBarPosition::Top);
    assert_eq!(bar.get_position(), StatusBarPosition::Top);
}

#[test]
fn test_status_bar_header() {
    let bar = StatusBar::new().header();
    assert_eq!(bar.get_position(), StatusBarPosition::Top);
}

#[test]
fn test_status_bar_footer() {
    let bar = StatusBar::new().footer();
    assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
}

#[test]
fn test_status_bar_left() {
    let section = StatusSection::new("Left Text");
    let bar = StatusBar::new().left(section.clone());
    assert_eq!(bar.get_left().len(), 1);
    assert_eq!(bar.get_left()[0].content, "Left Text");
}

#[test]
fn test_status_bar_center() {
    let section = StatusSection::new("Center");
    let bar = StatusBar::new().center(section);
    assert_eq!(bar.get_center().len(), 1);
    assert_eq!(bar.get_center()[0].content, "Center");
}

#[test]
fn test_status_bar_right() {
    let section = StatusSection::new("Right");
    let bar = StatusBar::new().right(section);
    assert_eq!(bar.get_right().len(), 1);
    assert_eq!(bar.get_right()[0].content, "Right");
}

#[test]
fn test_status_bar_left_text() {
    let bar = StatusBar::new().left_text("File");
    assert_eq!(bar.get_left().len(), 1);
    assert_eq!(bar.get_left()[0].content, "File");
}

#[test]
fn test_status_bar_center_text() {
    let bar = StatusBar::new().center_text("Line 1");
    assert_eq!(bar.get_center().len(), 1);
    assert_eq!(bar.get_center()[0].content, "Line 1");
}

#[test]
fn test_status_bar_right_text() {
    let bar = StatusBar::new().right_text("UTF-8");
    assert_eq!(bar.get_right().len(), 1);
    assert_eq!(bar.get_right()[0].content, "UTF-8");
}

#[test]
fn test_status_bar_bg() {
    let bar = StatusBar::new().bg(Color::BLUE);
    assert_eq!(bar.get_bg(), Color::BLUE);
}

#[test]
fn test_status_bar_fg() {
    let bar = StatusBar::new().fg(Color::YELLOW);
    assert_eq!(bar.get_fg(), Color::YELLOW);
}

#[test]
fn test_status_bar_with_keys() {
    let bar = StatusBar::new()
        .key("^X", "Exit")
        .key("^S", "Save")
        .key("^O", "Open");

    assert_eq!(bar.get_key_hints().len(), 3);
}

#[test]
fn test_status_bar_key() {
    let bar = StatusBar::new().key("Ctrl+S", "Save");
    assert_eq!(bar.get_key_hints().len(), 1);
    assert_eq!(bar.get_key_hints()[0].key, "Ctrl+S");
    assert_eq!(bar.get_key_hints()[0].description, "Save");
}

#[test]
fn test_status_bar_keys() {
    let hints = vec![
        KeyHint::new("Ctrl+S", "Save"),
        KeyHint::new("Ctrl+Q", "Quit"),
    ];
    let bar = StatusBar::new().keys(hints);
    assert_eq!(bar.get_key_hints().len(), 2);
}

#[test]
fn test_status_bar_separator() {
    let bar = StatusBar::new().separator('|');
    assert_eq!(bar.get_separator(), Some('|'));
}

#[test]
fn test_status_bar_height() {
    let bar = StatusBar::new().height(2);
    assert_eq!(bar.get_height(), 2);
}

#[test]
fn test_status_bar_height_minimum() {
    let bar = StatusBar::new().height(0);
    assert_eq!(bar.get_height(), 1); // Minimum is 1
}

#[test]
fn test_status_bar_position() {
    let top = StatusBar::new().header();
    assert_eq!(top.get_position(), StatusBarPosition::Top);
    assert_eq!(top.get_render_y(24), 0);

    let bottom = StatusBar::new().footer();
    assert_eq!(bottom.get_position(), StatusBarPosition::Bottom);
    assert_eq!(bottom.get_render_y(24), 23);
}

#[test]
fn test_status_bar_update() {
    let mut bar = StatusBar::new().left_text("Original");

    bar.update_left(0, "Updated");
    assert_eq!(bar.get_left()[0].content, "Updated");
}

#[test]
fn test_status_bar_update_center() {
    let mut bar = StatusBar::new().center_text("Original");
    bar.update_center(0, "Updated");
    assert_eq!(bar.get_center()[0].content, "Updated");
}

#[test]
fn test_status_bar_update_right() {
    let mut bar = StatusBar::new().right_text("Original");
    bar.update_right(0, "Updated");
    assert_eq!(bar.get_right()[0].content, "Updated");
}

#[test]
fn test_status_bar_update_invalid_index() {
    let mut bar = StatusBar::new().left_text("Test");
    bar.update_left(5, "Won't update");
    assert_eq!(bar.get_left()[0].content, "Test"); // Unchanged
}

#[test]
fn test_status_bar_clear() {
    let mut bar = StatusBar::new()
        .left_text("L")
        .center_text("C")
        .right_text("R")
        .key("Ctrl+S", "Save");

    bar.clear();
    assert!(bar.get_left().is_empty());
    assert!(bar.get_center().is_empty());
    assert!(bar.get_right().is_empty());
    assert!(bar.get_key_hints().is_empty());
}

// =========================================================================
// StatusBar Default trait tests
// =========================================================================

#[test]
fn test_status_bar_default() {
    let bar = StatusBar::default();
    assert!(bar.get_left().is_empty());
    assert!(bar.get_center().is_empty());
    assert!(bar.get_right().is_empty());
    assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
}

// =========================================================================
// StatusBar builder chain tests
// =========================================================================

#[test]
fn test_status_bar_builder_chain() {
    let bar = StatusBar::new()
        .header()
        .bg(Color::BLUE)
        .fg(Color::WHITE)
        .separator('|')
        .height(2)
        .left_text("File.txt")
        .center_text("Line 1")
        .right_text("UTF-8")
        .key("Ctrl+S", "Save")
        .key("Ctrl+Q", "Quit");

    assert_eq!(bar.get_position(), StatusBarPosition::Top);
    assert_eq!(bar.get_bg(), Color::BLUE);
    assert_eq!(bar.get_fg(), Color::WHITE);
    assert_eq!(bar.get_separator(), Some('|'));
    assert_eq!(bar.get_height(), 2);
    assert_eq!(bar.get_left().len(), 1);
    assert_eq!(bar.get_center().len(), 1);
    assert_eq!(bar.get_right().len(), 1);
    assert_eq!(bar.get_key_hints().len(), 2);
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_statusbar_helper() {
    let bar = statusbar();
    assert!(bar.get_left().is_empty());
    assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
}

#[test]
fn test_header_helper() {
    let bar = header();
    assert_eq!(bar.get_position(), StatusBarPosition::Top);
}

#[test]
fn test_footer_helper() {
    let bar = footer();
    assert_eq!(bar.get_position(), StatusBarPosition::Bottom);
}

#[test]
fn test_section_helper() {
    let s = section("Content");
    assert_eq!(s.content, "Content");
}

#[test]
fn test_key_hint_helper() {
    let hint = key_hint("Ctrl+C", "Copy");
    assert_eq!(hint.key, "Ctrl+C");
    assert_eq!(hint.description, "Copy");
}

// =========================================================================
// StatusBar render_y tests
// =========================================================================

#[test]
fn test_render_y_top() {
    let bar = StatusBar::new().position(StatusBarPosition::Top);
    assert_eq!(bar.get_render_y(24), 0);
}

#[test]
fn test_render_y_bottom() {
    let bar = StatusBar::new().position(StatusBarPosition::Bottom);
    assert_eq!(bar.get_render_y(24), 23);
}

#[test]
fn test_render_y_bottom_with_height() {
    let bar = StatusBar::new()
        .position(StatusBarPosition::Bottom)
        .height(2);
    assert_eq!(bar.get_render_y(24), 22);
}

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_status_bar_render() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bar = StatusBar::new().left_text("Left").right_text("Right");

    bar.render(&mut ctx);
    // Smoke test
}
