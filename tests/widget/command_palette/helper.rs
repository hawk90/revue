//! CommandPalette helper function tests

use revue::style::Color;
use revue::widget::theme::{DARK_GRAY, EDITOR_BG};
use revue::widget::{command_palette, Command, CommandPalette};

use super::view::{cells, render, row};

// =========================================================================
// command_palette() helper function tests
// =========================================================================

#[test]
fn test_command_palette_helper() {
    let mut cp = command_palette().command(Command::new("a", "Alpha"));
    let mut new = CommandPalette::new().command(Command::new("a", "Alpha"));
    cp.show();
    new.show();
    assert_eq!(cells(&render(&cp, 70, 12)), cells(&render(&new, 70, 12)));
}

#[test]
fn test_command_palette_initial_state() {
    let cp = command_palette();
    // Query should be empty initially
    assert!(cp.query.is_empty());
}

#[test]
fn test_command_palette_multiple_instances() {
    let cp1 = command_palette();
    let cp2 = command_palette();
    // Each should be independent
    assert!(cp1.query.is_empty());
    assert!(cp2.query.is_empty());
}

#[test]
fn test_command_palette_is_chainable() {
    let mut cp = command_palette()
        .title("Run")
        .width(40)
        .command(Command::new("a", "Alpha"))
        .command(Command::new("b", "Beta"));
    cp.show();
    let b = render(&cp, 70, 12);
    // A 40-column frame in 70 columns starts at column 15.
    assert_eq!(b.get(15, 2).unwrap().symbol, '╭');
    assert_eq!(b.get(54, 2).unwrap().symbol, '╮');
    assert!(row(&b, 3).contains("Run"), "{}", row(&b, 3));
    assert!(row(&b, 6).contains("Alpha"), "{}", row(&b, 6));
}

#[test]
fn test_command_palette_can_be_created_repeatedly() {
    // Create multiple palettes in sequence
    for _ in 0..10 {
        let cp = command_palette();
        assert!(cp.query.is_empty());
    }
}

#[test]
fn test_command_palette_initial_filter_state() {
    let cp = command_palette();
    // Filtered indices should be empty initially
    assert!(cp.filtered.is_empty());
}

#[test]
fn test_command_palette_initial_visible_state() {
    let cp = command_palette().command(Command::new("a", "Alpha"));
    assert!(!cp.visible);
    let b = render(&cp, 70, 12);
    assert!((0..12).all(|y| row(&b, y).trim().is_empty()));
}

#[test]
fn test_command_palette_initial_width() {
    let cp = command_palette();
    // Should have a default width
    assert!(cp.width > 0);
}

#[test]
fn test_command_palette_initial_max_visible() {
    let cp = command_palette();
    // Should have a default max visible items setting
    assert!(cp.max_visible > 0);
}

#[test]
fn test_command_palette_initial_placeholder() {
    let mut cp = command_palette();
    assert_eq!(cp.placeholder, "Type to search...");
    cp.show();
    // The first letter is under the cursor (#1091).
    assert!(row(&render(&cp, 70, 12), 3).contains("ype to search..."));
}

#[test]
fn test_command_palette_initial_title() {
    let cp = command_palette();
    // Title should be None initially
    assert!(cp.title.is_none());
}

#[test]
fn test_command_palette_initial_show_descriptions() {
    let mut cp = command_palette()
        .command(Command::new("a", "Alpha").description("first"))
        .command(Command::new("z", "Zed"));
    // Two commands: the last row is under the bottom border (#1091).
    assert!(cp.show_descriptions);
    cp.show();
    assert!(row(&render(&cp, 70, 12), 5).contains("Alpha  first"));
}

#[test]
fn test_command_palette_initial_show_shortcuts() {
    let mut cp = command_palette()
        .command(Command::new("a", "Alpha").shortcut("F1"))
        .command(Command::new("z", "Zed"));
    assert!(cp.show_shortcuts);
    cp.show();
    assert!(row(&render(&cp, 70, 12), 5).contains("F1 │"));
}

#[test]
fn test_command_palette_initial_show_icons() {
    let mut cp = command_palette()
        .command(Command::new("a", "Alpha").icon('*'))
        .command(Command::new("z", "Zed"));
    assert!(cp.show_icons);
    cp.show();
    assert!(row(&render(&cp, 70, 12), 5).contains("│ * Alpha"));
}

#[test]
fn test_command_palette_empty_initially() {
    let cp = command_palette();
    // Commands list should be empty initially
    assert!(cp.commands.is_empty());
}

#[test]
fn test_command_palette_initial_selection_state() {
    let cp = command_palette();
    assert_eq!(cp.selection.index, 0);
    assert_eq!(cp.selection.len, 0);
    assert_eq!(cp.selection.offset(), 0);
}

#[test]
fn test_command_palette_initial_colors() {
    let mut cp =
        command_palette().commands(vec![Command::new("a", "Alpha"), Command::new("b", "Beta")]);
    assert_eq!(cp.match_color, Color::YELLOW);
    cp.show();
    cp.set_query("a");
    let b = render(&cp, 70, 12);
    // Frame at column 5; the selected first row is row 5.
    assert_eq!(b.get(5, 2).unwrap().fg, Some(DARK_GRAY));
    assert_eq!(b.get(6, 3).unwrap().bg, Some(EDITOR_BG));
    assert_eq!(b.get(6, 5).unwrap().bg, Some(Color::rgb(50, 80, 120)));
    assert_eq!(b.get(9, 5).unwrap().symbol, 'A');
    assert_eq!(b.get(9, 5).unwrap().fg, Some(Color::YELLOW));
}

#[test]
fn test_command_palette_initial_props() {
    let cp = command_palette();
    assert!(cp.props.id.is_none());
    assert!(cp.props.classes.is_empty());
    assert!(cp.props.inline_style.is_none());
}

#[test]
fn test_command_palette_clone_behavior() {
    // Test that multiple palettes are independent
    let cp1 = command_palette();
    let cp2 = command_palette();

    // Both should have independent state
    assert_eq!(cp1.query, cp2.query);
    assert!(cp1.commands.is_empty());
    assert!(cp2.commands.is_empty());
}

#[test]
fn test_command_palette_initial_width_is_reasonable() {
    let cp = command_palette();
    // Width should be a reasonable value
    assert!(cp.width >= 20);
    assert!(cp.width <= 200);
}

#[test]
fn test_command_palette_initial_max_visible_is_reasonable() {
    let cp = command_palette();
    // Max visible should be a reasonable value
    assert!(cp.max_visible >= 5);
    assert!(cp.max_visible <= 50);
}
