//! CommandPalette implementation tests

use revue::event::Key;
use revue::style::Color;
use revue::widget::{Command, CommandPalette};

// =========================================================================
// CommandPalette::new tests
// =========================================================================

#[test]
fn test_command_palette_new() {
    let palette = CommandPalette::new();
    assert!(palette.commands.is_empty());
    assert!(palette.query.is_empty());
    assert!(!palette.is_visible());
    assert_eq!(palette.width, 60);
    assert_eq!(palette.max_visible, 10);
}

// =========================================================================
// command(s) builder tests
// =========================================================================

#[test]
fn test_command_palette_command() {
    let palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
    assert_eq!(palette.commands.len(), 1);
}

#[test]
fn test_command_palette_commands() {
    let cmds = vec![
        Command::new("cmd1", "Command 1"),
        Command::new("cmd2", "Command 2"),
    ];
    let palette = CommandPalette::new().commands(cmds);
    assert_eq!(palette.commands.len(), 2);
}

// =========================================================================
// Builder methods tests
// =========================================================================

#[test]
fn test_command_palette_width() {
    let palette = CommandPalette::new().width(40);
    assert_eq!(palette.width, 40);
}

#[test]
fn test_command_palette_width_minimum() {
    let palette = CommandPalette::new().width(10);
    assert_eq!(palette.width, 30); // minimum enforced
}

#[test]
fn test_command_palette_max_visible() {
    let palette = CommandPalette::new().max_visible(5);
    assert_eq!(palette.max_visible, 5);
}

#[test]
fn test_command_palette_max_visible_minimum() {
    let palette = CommandPalette::new().max_visible(1);
    assert_eq!(palette.max_visible, 3); // minimum enforced
}

#[test]
fn test_command_palette_placeholder() {
    let palette = CommandPalette::new().placeholder("Search...");
    assert_eq!(palette.placeholder, "Search...");
}

#[test]
fn test_command_palette_title() {
    let palette = CommandPalette::new().title("Commands");
    assert_eq!(palette.title, Some("Commands".to_string()));
}

#[test]
fn test_command_palette_show_descriptions() {
    let palette = CommandPalette::new().show_descriptions(false);
    assert!(!palette.show_descriptions);
}

#[test]
fn test_command_palette_show_shortcuts() {
    let palette = CommandPalette::new().show_shortcuts(false);
    assert!(!palette.show_shortcuts);
}

#[test]
fn test_command_palette_show_icons() {
    let palette = CommandPalette::new().show_icons(false);
    assert!(!palette.show_icons);
}

#[test]
fn test_command_palette_colors() {
    let palette = CommandPalette::new().colors(Color::BLACK, Color::WHITE, Color::RED);
    assert_eq!(palette.bg_color, Color::BLACK);
    assert_eq!(palette.border_color, Color::WHITE);
    assert_eq!(palette.selected_bg, Color::RED);
}

// =========================================================================
// Visibility tests
// =========================================================================

#[test]
fn test_command_palette_show() {
    let mut palette = CommandPalette::new();
    palette.show();
    assert!(palette.is_visible());
    assert!(palette.query.is_empty());
}

#[test]
fn test_command_palette_hide() {
    let mut palette = CommandPalette::new();
    palette.show();
    palette.hide();
    assert!(!palette.is_visible());
}

#[test]
fn test_command_palette_toggle_hidden_to_visible() {
    let mut palette = CommandPalette::new();
    palette.toggle();
    assert!(palette.is_visible());
}

#[test]
fn test_command_palette_toggle_visible_to_hidden() {
    let mut palette = CommandPalette::new();
    palette.show();
    palette.toggle();
    assert!(!palette.is_visible());
}

// =========================================================================
// Query tests
// =========================================================================

#[test]
fn test_command_palette_get_query() {
    let palette = CommandPalette::new();
    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_command_palette_set_query() {
    let mut palette = CommandPalette::new();
    palette.set_query("test");
    assert_eq!(palette.get_query(), "test");
}

#[test]
fn test_command_palette_clear_query() {
    let mut palette = CommandPalette::new();
    palette.set_query("test");
    palette.clear_query();
    assert_eq!(palette.get_query(), "");
}

// =========================================================================
// Selection tests
// =========================================================================

#[test]
fn test_command_palette_select_next() {
    let mut palette = CommandPalette::new()
        .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
    palette.select_next();
    // Just verify it doesn't panic
}

#[test]
fn test_command_palette_select_prev() {
    let mut palette = CommandPalette::new()
        .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
    palette.select_prev();
    // Just verify it doesn't panic
}

#[test]
fn test_command_palette_selected_command_empty() {
    let palette = CommandPalette::new();
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_command_palette_selected_command() {
    let palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
    let cmd = palette.selected_command();
    assert!(cmd.is_some());
}

#[test]
fn test_command_palette_selected_id_empty() {
    let palette = CommandPalette::new();
    assert!(palette.selected_id().is_none());
}

#[test]
fn test_command_palette_selected_id() {
    let palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
    let id = palette.selected_id();
    assert_eq!(id, Some("cmd1"));
}

// =========================================================================
// Execute tests
// =========================================================================

#[test]
fn test_command_palette_execute_empty() {
    let mut palette = CommandPalette::new();
    let result = palette.execute();
    assert!(result.is_none());
}

#[test]
fn test_command_palette_execute_with_selection() {
    let mut palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
    let result = palette.execute();
    assert_eq!(result, Some("cmd1".to_string()));
    assert!(!palette.is_visible());
}

// =========================================================================
// Input tests
// =========================================================================

#[test]
fn test_command_palette_input() {
    let mut palette = CommandPalette::new();
    palette.input('a');
    assert_eq!(palette.get_query(), "a");
}

#[test]
fn test_command_palette_input_multiple() {
    let mut palette = CommandPalette::new();
    palette.input('a');
    palette.input('b');
    palette.input('c');
    assert_eq!(palette.get_query(), "abc");
}

#[test]
fn test_command_palette_backspace() {
    let mut palette = CommandPalette::new();
    palette.input('a');
    palette.backspace();
    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_command_palette_backspace_empty() {
    let mut palette = CommandPalette::new();
    palette.backspace();
    assert_eq!(palette.get_query(), "");
}

// =========================================================================
// Key handling tests
// =========================================================================

#[test]
fn test_handle_key_escape_hides() {
    let mut palette = CommandPalette::new();
    palette.show();
    palette.handle_key(&Key::Escape);
    assert!(!palette.is_visible());
}

#[test]
fn test_handle_key_enter_executes() {
    let mut palette = CommandPalette::new().command(Command::new("cmd1", "Command 1"));
    palette.show();
    let result = palette.handle_key(&Key::Enter);
    assert!(result);
    assert!(!palette.is_visible());
}

#[test]
fn test_handle_key_up() {
    let mut palette = CommandPalette::new()
        .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
    palette.show();
    let handled = palette.handle_key(&Key::Up);
    assert!(handled);
}

#[test]
fn test_handle_key_down() {
    let mut palette = CommandPalette::new()
        .commands(vec![Command::new("cmd1", "A"), Command::new("cmd2", "B")]);
    palette.show();
    let handled = palette.handle_key(&Key::Down);
    assert!(handled);
}

#[test]
fn test_handle_key_char() {
    let mut palette = CommandPalette::new();
    palette.show();
    let handled = palette.handle_key(&Key::Char('a'));
    assert!(handled);
    assert_eq!(palette.get_query(), "a");
}

#[test]
fn test_handle_key_backspace() {
    let mut palette = CommandPalette::new();
    palette.show();
    palette.input('a');
    let handled = palette.handle_key(&Key::Backspace);
    assert!(handled);
    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_handle_key_when_hidden() {
    let mut palette = CommandPalette::new();
    let handled = palette.handle_key(&Key::Char('a'));
    assert!(!handled);
}

#[test]
fn test_handle_key_unknown() {
    let mut palette = CommandPalette::new();
    palette.show();
    let handled = palette.handle_key(&Key::PageUp);
    assert!(!handled);
}

// =========================================================================
// Command management tests
// =========================================================================

#[test]
fn test_add_command() {
    let mut palette = CommandPalette::new();
    palette.add_command(Command::new("cmd1", "Command 1"));
    assert_eq!(palette.commands.len(), 1);
}

#[test]
fn test_remove_command() {
    let mut palette = CommandPalette::new();
    palette.add_command(Command::new("cmd1", "Command 1"));
    palette.remove_command("cmd1");
    assert_eq!(palette.commands.len(), 0);
}

#[test]
fn test_remove_command_nonexistent() {
    let mut palette = CommandPalette::new();
    palette.remove_command("nonexistent");
    assert_eq!(palette.commands.len(), 0);
}

#[test]
fn test_clear_commands() {
    let mut palette = CommandPalette::new();
    palette.add_command(Command::new("cmd1", "Command 1"));
    palette.add_command(Command::new("cmd2", "Command 2"));
    palette.clear_commands();
    assert_eq!(palette.commands.len(), 0);
    assert!(palette.filtered.is_empty());
}

#[test]
fn test_mark_recent() {
    let mut palette = CommandPalette::new();
    palette.add_command(Command::new("cmd1", "Command 1"));
    palette.mark_recent("cmd1");
    assert!(palette.commands[0].recent);
}

#[test]
fn test_mark_recent_nonexistent() {
    let mut palette = CommandPalette::new();
    palette.mark_recent("nonexistent");
    // Should not panic
}

// =========================================================================
// highlight_match tests
// =========================================================================

#[test]
fn test_highlight_match_empty_query() {
    let palette = CommandPalette::new();
    let result = palette.highlight_match("Save");
    assert_eq!(result.len(), 4);
    // All chars should not be highlighted
    assert!(!result[0].1);
    assert!(!result[1].1);
    assert!(!result[2].1);
    assert!(!result[3].1);
}

#[test]
fn test_highlight_match_with_query() {
    let mut palette = CommandPalette::new();
    palette.set_query("sv");
    let result = palette.highlight_match("Save");
    assert_eq!(result.len(), 4);
}

#[test]
fn test_highlight_match_empty_label() {
    let palette = CommandPalette::new();
    let result = palette.highlight_match("");
    assert!(result.is_empty());
}

// =========================================================================
// Filter tests
// =========================================================================

#[test]
fn test_filter_with_query() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("cmd1", "Save File"),
        Command::new("cmd2", "Open File"),
        Command::new("cmd3", "Exit"),
    ]);
    palette.set_query("save");
    assert!(palette.filtered.len() > 0);
}

#[test]
fn test_filter_empty_query() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("cmd1", "Save"),
        Command::new("cmd2", "Open"),
    ]);
    palette.set_query("");
    assert_eq!(palette.filtered.len(), 2);
}

#[test]
fn test_filter_no_match() {
    let mut palette = CommandPalette::new().commands(vec![Command::new("cmd1", "Save")]);
    palette.set_query("xyz");
    assert_eq!(palette.filtered.len(), 0);
}