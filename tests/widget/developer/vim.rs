//! Public API tests for Vim Mode system

use revue::widget::developer::vim::{VimAction, VimCommandResult, VimMode, VimMotion, VimState};

#[test]
fn test_vim_mode_name() {
    // Test all modes have proper names
    assert_eq!(VimMode::Normal.name(), "NORMAL");
    assert_eq!(VimMode::Insert.name(), "INSERT");
    assert_eq!(VimMode::Visual.name(), "VISUAL");
    assert_eq!(VimMode::VisualLine.name(), "V-LINE");
    assert_eq!(VimMode::VisualBlock.name(), "V-BLOCK");
    assert_eq!(VimMode::Command.name(), "COMMAND");
    assert_eq!(VimMode::Search.name(), "SEARCH");
    assert_eq!(VimMode::Replace.name(), "REPLACE");
}

#[test]
fn test_vim_mode_color() {
    // Test all modes have colors
    assert_eq!(VimMode::Normal.color().to_string(), "rgb(100, 150, 255)");
    assert_eq!(VimMode::Insert.color().to_string(), "rgb(100, 255, 100)");
    assert_eq!(VimMode::Visual.color().to_string(), "rgb(255, 150, 100)");
    assert_eq!(VimMode::VisualLine.color().to_string(), "rgb(255, 150, 100)");
    assert_eq!(VimMode::VisualBlock.color().to_string(), "rgb(255, 150, 100)");
    assert_eq!(VimMode::Command.color().to_string(), "rgb(255, 255, 100)");
    assert_eq!(VimMode::Search.color().to_string(), "rgb(255, 100, 255)");
    assert_eq!(VimMode::Replace.color().to_string(), "rgb(255, 100, 100)");
}

#[test]
fn test_vim_state_new() {
    // Test new vim state creation
    let vim = VimState::new();
    assert_eq!(vim.mode(), VimMode::Normal);
    assert_eq!(vim.count(), 1); // Default count is 1
    assert_eq!(vim.command_buffer(), "");
    assert_eq!(vim.search_pattern(), "");
    assert_eq!(vim.register(), "");
}

#[test]
fn test_vim_state_mode() {
    // Test mode management
    let mut vim = VimState::new();

    // Test initial mode
    assert_eq!(vim.mode(), VimMode::Normal);

    // Test changing mode
    vim.set_mode(VimMode::Insert);
    assert_eq!(vim.mode(), VimMode::Insert);

    // Test setting back to normal clears operator and count
    vim.set_mode(VimMode::Normal);
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_vim_state_count() {
    // Test count functionality
    let mut vim = VimState::new();

    // Test default count
    assert_eq!(vim.count(), 1);

    // Test handling keys with digits
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('2')));
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('5')));

    assert_eq!(vim.count(), 25);
}

#[test]
fn test_vim_state_command_buffer() {
    // Test command buffer functionality
    let mut vim = VimState::new();

    // Test empty buffer initially
    assert_eq!(vim.command_buffer(), "");

    // Test setting command buffer through command mode
    vim.set_mode(VimMode::Command);
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char(':')));
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('w')));
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('q')));

    assert_eq!(vim.command_buffer(), ":wq");
}

#[test]
fn test_vim_state_register() {
    // Test register functionality
    let mut vim = VimState::new();

    // Test empty register initially
    assert_eq!(vim.register(), "");

    // Test setting register content
    vim.set_register("test content");
    assert_eq!(vim.register(), "test content");
}

#[test]
fn test_vim_state_map() {
    // Test custom key mapping
    let mut vim = VimState::new();

    // Test adding a mapping
    vim.map("dd", VimAction::Delete(Some(VimMotion::LineStart)));

    // Note: Actual mapping functionality would be tested if the mapping was public
    // This test just verifies the public API exists
}

#[test]
fn test_vim_state_handle_key_normal_mode() {
    // Test key handling in normal mode
    let mut vim = VimState::new();

    // Test navigation keys
    let action = vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('h')));
    assert_eq!(action, VimAction::Move(VimMotion::Left));

    let action = vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('j')));
    assert_eq!(action, VimAction::Move(VimMotion::Down));

    let action = vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('k')));
    assert_eq!(action, VimAction::Move(VimMotion::Up));

    let action = vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('l')));
    assert_eq!(action, VimAction::Move(VimMotion::Right));
}

#[test]
fn test_vim_state_handle_mode_transitions() {
    // Test mode transitions
    let mut vim = VimState::new();

    // Test transition to insert mode
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('i')));
    assert_eq!(vim.mode(), VimMode::Insert);

    // Test escape back to normal mode
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Escape));
    assert_eq!(vim.mode(), VimMode::Normal);

    // Test transition to visual mode
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Char('v')));
    assert_eq!(vim.mode(), VimMode::Visual);

    // Test escape back to normal mode
    vim.handle_key(&revue::event::KeyEvent::new(revue::event::Key::Escape));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_vim_command_result() {
    // Test command result enum
    let write_result = VimCommandResult::Write;
    let quit_result = VimCommandResult::Quit;
    let write_quit_result = VimCommandResult::WriteQuit;
    let force_quit_result = VimCommandResult::ForceQuit;
    let edit_result = VimCommandResult::Edit(Some("file.txt".to_string()));
    let set_result = VimCommandResult::Set("number".to_string());
    let goto_line_result = VimCommandResult::GoToLine(42);
    let unknown_result = VimCommandResult::Unknown("unknown".to_string());

    // Verify the results exist and can be matched
    match write_result {
        VimCommandResult::Write => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_vim_state_execute_command() {
    // Test command execution
    let mut vim = VimState::new();

    // Test basic commands
    assert_eq!(vim.execute_command("w"), VimCommandResult::Write);
    assert_eq!(vim.execute_command("q"), VimCommandResult::Quit);
    assert_eq!(vim.execute_command("wq"), VimCommandResult::WriteQuit);
    assert_eq!(vim.execute_command("q!"), VimCommandResult::ForceQuit);
    assert_eq!(vim.execute_command("e"), VimCommandResult::Edit(None));
    assert_eq!(vim.execute_command("e file.txt"), VimCommandResult::Edit(Some("file.txt".to_string())));
    assert_eq!(vim.execute_command("set number"), VimCommandResult::Set("number".to_string()));

    // Test line number command
    assert_eq!(vim.execute_command("42"), VimCommandResult::GoToLine(42));

    // Test unknown command
    assert_eq!(vim.execute_command("unknown"), VimCommandResult::Unknown("unknown".to_string()));
}

#[test]
fn test_vim_motion_equality() {
    // Test motion equality
    let left1 = VimMotion::Left;
    let left2 = VimMotion::Left;
    let right = VimMotion::Right;

    assert_eq!(left1, left2);
    assert_ne!(left1, right);
}

#[test]
fn test_vim_action_equality() {
    // Test action equality
    let move_left1 = VimAction::Move(VimMotion::Left);
    let move_left2 = VimAction::Move(VimMotion::Left);
    let delete = VimAction::Delete(Some(VimMotion::Right));

    assert_eq!(move_left1, move_left2);
    assert_ne!(move_left1, delete);
}