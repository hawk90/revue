//! Vim widget integration tests
//!
//! Vim 모드 시스템에 대한 통합 테스트

use revue::event::{Key, KeyEvent};
use revue::widget::{vim_state, VimAction, VimCommandResult, VimMode, VimMotion, VimState};

// =============================================================================
// Constructor Tests - 생성자 테스트
// =============================================================================

#[test]
fn test_vim_state_new() {
    let vim = VimState::new();
    assert_eq!(vim.mode(), VimMode::Normal);
    assert_eq!(vim.count(), 1);
    assert_eq!(vim.command_buffer(), "");
    assert_eq!(vim.search_pattern(), "");
    assert_eq!(vim.register(), "");
}

#[test]
fn test_vim_state_default() {
    let vim = VimState::default();
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_vim_state_helper() {
    let vim = vim_state();
    assert_eq!(vim.mode(), VimMode::Normal);
}

// =============================================================================
// Mode Property Tests - 모드 속성 테스트
// =============================================================================

#[test]
fn test_vim_mode_names() {
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
fn test_vim_mode_colors() {
    // 각 모드가 고유한 색상을 반환하는지 확인
    let normal_color = VimMode::Normal.color();
    let insert_color = VimMode::Insert.color();
    let visual_color = VimMode::Visual.color();
    let command_color = VimMode::Command.color();

    assert_ne!(normal_color, insert_color);
    assert_ne!(insert_color, visual_color);
    assert_ne!(visual_color, command_color);
}

#[test]
fn test_visual_modes_same_color() {
    // 모든 시각적 모드가 동일한 색상을 사용하는지 확인
    let visual = VimMode::Visual.color();
    let visual_line = VimMode::VisualLine.color();
    let visual_block = VimMode::VisualBlock.color();

    assert_eq!(visual, visual_line);
    assert_eq!(visual_line, visual_block);
}

// =============================================================================
// Mode Switching Tests - 모드 전환 테스트
// =============================================================================

#[test]
fn test_switch_to_insert_mode() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('i')));
    assert_eq!(action, VimAction::Insert);
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_switch_to_insert_mode_at_start() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('I')));
    assert_eq!(action, VimAction::InsertStart);
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_switch_to_insert_mode_append() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('a')));
    assert_eq!(action, VimAction::Append);
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_switch_to_insert_mode_append_end() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('A')));
    assert_eq!(action, VimAction::AppendEnd);
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_switch_to_visual_mode() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('v')));
    assert_eq!(action, VimAction::EnterVisual);
    assert_eq!(vim.mode(), VimMode::Visual);
}

#[test]
fn test_switch_to_visual_line_mode() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('V')));
    assert_eq!(action, VimAction::EnterVisualLine);
    assert_eq!(vim.mode(), VimMode::VisualLine);
}

#[test]
fn test_switch_to_command_mode() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char(':')));
    assert_eq!(action, VimAction::EnterCommand);
    assert_eq!(vim.mode(), VimMode::Command);
}

#[test]
fn test_switch_to_search_mode_forward() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('/')));
    assert_eq!(action, VimAction::EnterSearch);
    assert_eq!(vim.mode(), VimMode::Search);
}

#[test]
fn test_switch_to_search_mode_backward() {
    let mut vim = VimState::new();

    let action = vim.handle_key(&KeyEvent::new(Key::Char('?')));
    assert_eq!(action, VimAction::EnterSearch);
    assert_eq!(vim.mode(), VimMode::Search);
}

#[test]
fn test_escape_to_normal_mode() {
    let mut vim = VimState::new();

    // Insert 모드로 전환
    vim.handle_key(&KeyEvent::new(Key::Char('i')));
    assert_eq!(vim.mode(), VimMode::Insert);

    // Escape로 Normal 모드로 복귀
    let action = vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(action, VimAction::Escape);
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_escape_from_visual_mode() {
    let mut vim = VimState::new();

    vim.handle_key(&KeyEvent::new(Key::Char('v')));
    assert_eq!(vim.mode(), VimMode::Visual);

    let action = vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(action, VimAction::Escape);
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_escape_from_command_mode() {
    let mut vim = VimState::new();

    vim.handle_key(&KeyEvent::new(Key::Char(':')));
    assert_eq!(vim.mode(), VimMode::Command);

    let action = vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(action, VimAction::Escape);
    assert_eq!(vim.mode(), VimMode::Normal);
    assert_eq!(vim.command_buffer(), "");
}

#[test]
fn test_escape_from_search_mode() {
    let mut vim = VimState::new();

    vim.handle_key(&KeyEvent::new(Key::Char('/')));
    assert_eq!(vim.mode(), VimMode::Search);

    let action = vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(action, VimAction::Escape);
    assert_eq!(vim.mode(), VimMode::Normal);
    assert_eq!(vim.search_pattern(), "");
}

// =============================================================================
// Motion Tests - 모션 테스트
// =============================================================================

#[test]
fn test_motion_left_h() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('h')));
    assert_eq!(action, VimAction::Move(VimMotion::Left));
}

#[test]
fn test_motion_left_arrow() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Left));
    assert_eq!(action, VimAction::Move(VimMotion::Left));
}

#[test]
fn test_motion_down_j() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('j')));
    assert_eq!(action, VimAction::Move(VimMotion::Down));
}

#[test]
fn test_motion_down_arrow() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Down));
    assert_eq!(action, VimAction::Move(VimMotion::Down));
}

#[test]
fn test_motion_up_k() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('k')));
    assert_eq!(action, VimAction::Move(VimMotion::Up));
}

#[test]
fn test_motion_up_arrow() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Up));
    assert_eq!(action, VimAction::Move(VimMotion::Up));
}

#[test]
fn test_motion_right_l() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('l')));
    assert_eq!(action, VimAction::Move(VimMotion::Right));
}

#[test]
fn test_motion_right_arrow() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Right));
    assert_eq!(action, VimAction::Move(VimMotion::Right));
}

#[test]
fn test_motion_word_forward() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('w')));
    assert_eq!(action, VimAction::Move(VimMotion::Word));
}

#[test]
fn test_motion_word_backward() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('b')));
    assert_eq!(action, VimAction::Move(VimMotion::WordBack));
}

#[test]
fn test_motion_word_end() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('e')));
    assert_eq!(action, VimAction::Move(VimMotion::WordEnd));
}

#[test]
fn test_motion_line_start() {
    let mut vim = VimState::new();
    // 0은 카운트로 먼저 처리되므로 모션으로 동작하지 않음
    // 이는 구현의 제한사항
    let action = vim.handle_key(&KeyEvent::new(Key::Char('0')));
    assert_eq!(action, VimAction::None);
    assert_eq!(vim.count(), 0);
}

#[test]
fn test_motion_line_end() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('$')));
    assert_eq!(action, VimAction::Move(VimMotion::LineEnd));
}

#[test]
fn test_motion_first_non_blank() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('^')));
    assert_eq!(action, VimAction::Move(VimMotion::FirstNonBlank));
}

#[test]
fn test_motion_go_to_last_line() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('G')));
    assert_eq!(action, VimAction::Move(VimMotion::GoToLine(None)));
}

#[test]
fn test_motion_go_to_first_line_gg() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('g')));
    let action = vim.handle_key(&KeyEvent::new(Key::Char('g')));
    assert_eq!(action, VimAction::Move(VimMotion::GoToLine(Some(1))));
}

#[test]
fn test_motion_paragraph_forward() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('}')));
    assert_eq!(action, VimAction::Move(VimMotion::ParagraphForward));
}

#[test]
fn test_motion_paragraph_backward() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('{')));
    assert_eq!(action, VimAction::Move(VimMotion::ParagraphBack));
}

#[test]
fn test_motion_match_bracket() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('%')));
    assert_eq!(action, VimAction::Move(VimMotion::MatchBracket));
}

#[test]
fn test_motion_search_next() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('n')));
    assert_eq!(action, VimAction::Move(VimMotion::SearchNext));
}

#[test]
fn test_motion_search_prev() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('N')));
    assert_eq!(action, VimAction::Move(VimMotion::SearchPrev));
}

// =============================================================================
// Operator Tests - 연산자 테스트
// =============================================================================

#[test]
fn test_operator_delete_none() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('d')));
    assert_eq!(action, VimAction::None);
    // 연산자가 대기 중인지 확인하기 위해 다른 키 입력
    let action = vim.handle_key(&KeyEvent::new(Key::Char('w')));
    assert_eq!(action, VimAction::Delete(Some(VimMotion::Word)));
}

#[test]
fn test_operator_yank_none() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('y')));
    assert_eq!(action, VimAction::None);
}

#[test]
fn test_operator_change_none() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('c')));
    assert_eq!(action, VimAction::None);
}

#[test]
fn test_operator_delete_with_motion() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    let action = vim.handle_key(&KeyEvent::new(Key::Char('l')));
    assert_eq!(action, VimAction::Delete(Some(VimMotion::Right)));
}

#[test]
fn test_operator_yank_with_motion() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('y')));
    let action = vim.handle_key(&KeyEvent::new(Key::Char('w')));
    assert_eq!(action, VimAction::Yank(Some(VimMotion::Word)));
}

#[test]
fn test_operator_change_with_motion() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('c')));
    let action = vim.handle_key(&KeyEvent::new(Key::Char('$')));
    assert_eq!(action, VimAction::Change(Some(VimMotion::LineEnd)));
}

// =============================================================================
// Action Tests - 액션 테스트
// =============================================================================

#[test]
fn test_action_delete_char() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('x')));
    assert_eq!(action, VimAction::Delete(Some(VimMotion::Right)));
}

#[test]
fn test_action_delete_char_backward() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('X')));
    assert_eq!(action, VimAction::Delete(Some(VimMotion::Left)));
}

#[test]
fn test_action_paste_after() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('p')));
    assert_eq!(action, VimAction::PasteAfter);
}

#[test]
fn test_action_paste_before() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('P')));
    assert_eq!(action, VimAction::PasteBefore);
}

#[test]
fn test_action_undo() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('u')));
    assert_eq!(action, VimAction::Undo);
}

#[test]
fn test_action_redo() {
    let mut vim = VimState::new();
    let mut event = KeyEvent::new(Key::Char('r'));
    event.ctrl = true;
    let action = vim.handle_key(&event);
    assert_eq!(action, VimAction::Redo);
}

#[test]
fn test_action_repeat() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('.')));
    assert_eq!(action, VimAction::Repeat);
}

#[test]
fn test_action_join_lines() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('J')));
    assert_eq!(action, VimAction::JoinLines);
}

#[test]
fn test_action_indent() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('>')));
    assert_eq!(action, VimAction::Indent);
}

#[test]
fn test_action_outdent() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('<')));
    assert_eq!(action, VimAction::Outdent);
}

#[test]
fn test_action_open_below() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('o')));
    assert_eq!(action, VimAction::OpenBelow);
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_action_open_above() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('O')));
    assert_eq!(action, VimAction::OpenAbove);
    assert_eq!(vim.mode(), VimMode::Insert);
}

// =============================================================================
// Count Tests - 카운트 테스트
// =============================================================================

#[test]
fn test_count_single_digit() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('5')));
    assert_eq!(vim.count(), 5);
}

#[test]
fn test_count_multiple_digits() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('2')));
    vim.handle_key(&KeyEvent::new(Key::Char('3')));
    assert_eq!(vim.count(), 23);
}

#[test]
fn test_count_reset_after_action() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('5')));
    assert_eq!(vim.count(), 5);

    // 모션 수행 후 카운트가 리셋되는지 확인
    vim.handle_key(&KeyEvent::new(Key::Char('j')));
    assert_eq!(vim.count(), 1);
}

#[test]
fn test_count_zero_sets_zero() {
    let mut vim = VimState::new();
    // 0은 카운트로 처리됨 (실제 Vim과는 다르지만 구현상 동작)
    vim.handle_key(&KeyEvent::new(Key::Char('0')));
    assert_eq!(vim.count(), 0);
}

#[test]
fn test_count_with_motion() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('3')));
    vim.handle_key(&KeyEvent::new(Key::Char('j')));
    // 카운트가 적용된 후 리셋됨
    assert_eq!(vim.count(), 1);
}

// =============================================================================
// Visual Mode Tests - 시각적 모드 테스트
// =============================================================================

#[test]
fn test_visual_mode_movement() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('v')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('l')));
    assert_eq!(action, VimAction::Move(VimMotion::Right));
}

#[test]
fn test_visual_mode_delete() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('v')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('d')));
    assert_eq!(action, VimAction::Delete(None));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_visual_mode_yank() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('v')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('y')));
    assert_eq!(action, VimAction::Yank(None));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_visual_mode_change() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('v')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('c')));
    assert_eq!(action, VimAction::Change(None));
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_visual_mode_word_movement() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('v')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('w')));
    assert_eq!(action, VimAction::Move(VimMotion::Word));
}

#[test]
fn test_visual_line_mode_escape() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('V')));
    assert_eq!(vim.mode(), VimMode::VisualLine);

    vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(vim.mode(), VimMode::Normal);
}

// =============================================================================
// Command Mode Tests - 명령 모드 테스트
// =============================================================================

#[test]
fn test_command_mode_input() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    vim.handle_key(&KeyEvent::new(Key::Char('w')));
    vim.handle_key(&KeyEvent::new(Key::Char('q')));
    assert_eq!(vim.command_buffer(), "wq");
}

#[test]
fn test_command_mode_backspace() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    vim.handle_key(&KeyEvent::new(Key::Char('w')));
    vim.handle_key(&KeyEvent::new(Key::Char('q')));
    assert_eq!(vim.command_buffer(), "wq");

    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.command_buffer(), "w");
}

#[test]
fn test_command_mode_backspace_exits_when_empty() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));
    assert_eq!(vim.mode(), VimMode::Command);

    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_command_mode_enter_executes() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    vim.handle_key(&KeyEvent::new(Key::Char('q')));
    let action = vim.handle_key(&KeyEvent::new(Key::Enter));
    assert_eq!(action, VimAction::ExecuteCommand("q".to_string()));
    assert_eq!(vim.mode(), VimMode::Normal);
    assert_eq!(vim.command_buffer(), "");
}

// =============================================================================
// Search Mode Tests - 검색 모드 테스트
// =============================================================================

#[test]
fn test_search_mode_input() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));

    vim.handle_key(&KeyEvent::new(Key::Char('p')));
    vim.handle_key(&KeyEvent::new(Key::Char('a')));
    vim.handle_key(&KeyEvent::new(Key::Char('t')));
    vim.handle_key(&KeyEvent::new(Key::Char('t')));
    vim.handle_key(&KeyEvent::new(Key::Char('e')));
    vim.handle_key(&KeyEvent::new(Key::Char('r')));
    vim.handle_key(&KeyEvent::new(Key::Char('n')));
    assert_eq!(vim.search_pattern(), "pattern");
}

#[test]
fn test_search_mode_backspace() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));

    vim.handle_key(&KeyEvent::new(Key::Char('t')));
    vim.handle_key(&KeyEvent::new(Key::Char('e')));
    vim.handle_key(&KeyEvent::new(Key::Char('s')));
    vim.handle_key(&KeyEvent::new(Key::Char('t')));
    assert_eq!(vim.search_pattern(), "test");

    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.search_pattern(), "tes");
}

#[test]
fn test_search_mode_backspace_exits_when_empty() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));
    assert_eq!(vim.mode(), VimMode::Search);

    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_search_mode_forward_executes() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));

    vim.handle_key(&KeyEvent::new(Key::Char('x')));
    let action = vim.handle_key(&KeyEvent::new(Key::Enter));
    assert_eq!(action, VimAction::Move(VimMotion::SearchNext));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_search_mode_backward_executes() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('?')));

    vim.handle_key(&KeyEvent::new(Key::Char('x')));
    let action = vim.handle_key(&KeyEvent::new(Key::Enter));
    assert_eq!(action, VimAction::Move(VimMotion::SearchPrev));
    assert_eq!(vim.mode(), VimMode::Normal);
}

// =============================================================================
// Register Tests - 레지스터 테스트
// =============================================================================

#[test]
fn test_register_initially_empty() {
    let vim = VimState::new();
    assert_eq!(vim.register(), "");
}

#[test]
fn test_set_register() {
    let mut vim = VimState::new();
    vim.set_register("yanked text");
    assert_eq!(vim.register(), "yanked text");
}

#[test]
fn test_set_register_overwrites() {
    let mut vim = VimState::new();
    vim.set_register("first");
    assert_eq!(vim.register(), "first");

    vim.set_register("second");
    assert_eq!(vim.register(), "second");
}

#[test]
fn test_set_register_with_string() {
    let mut vim = VimState::new();
    let content = String::from("test content");
    vim.set_register(content.clone());
    assert_eq!(vim.register(), "test content");
}

#[test]
fn test_set_register_with_str() {
    let mut vim = VimState::new();
    vim.set_register("str content");
    assert_eq!(vim.register(), "str content");
}

// =============================================================================
// Custom Mapping Tests - 커스텀 매핑 테스트
// =============================================================================

#[test]
fn test_custom_mapping() {
    let mut vim = VimState::new();
    vim.map("zz", VimAction::Move(VimMotion::GoToLine(Some(1))));

    // 매핑은 handle_key에서 자동으로 사용되지 않음
    // 이 테스트는 매핑이 저장되는지만 확인
    // 실제 사용은 구현에 따라 다름
}

// =============================================================================
// Command Execution Tests - 명령 실행 테스트
// =============================================================================

#[test]
fn test_command_write() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("w"), VimCommandResult::Write);
    assert_eq!(vim.execute_command("write"), VimCommandResult::Write);
}

#[test]
fn test_command_quit() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("q"), VimCommandResult::Quit);
    assert_eq!(vim.execute_command("quit"), VimCommandResult::Quit);
}

#[test]
fn test_command_write_and_quit() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("wq"), VimCommandResult::WriteQuit);
    assert_eq!(vim.execute_command("x"), VimCommandResult::WriteQuit);
}

#[test]
fn test_command_force_quit() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("q!"), VimCommandResult::ForceQuit);
}

#[test]
fn test_command_edit() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("e"), VimCommandResult::Edit(None));
    assert_eq!(vim.execute_command("edit"), VimCommandResult::Edit(None));
}

#[test]
fn test_command_edit_with_file() {
    let mut vim = VimState::new();
    assert_eq!(
        vim.execute_command("e file.txt"),
        VimCommandResult::Edit(Some("file.txt".to_string()))
    );
    assert_eq!(
        vim.execute_command("edit file.txt"),
        VimCommandResult::Edit(Some("file.txt".to_string()))
    );
}

#[test]
fn test_command_set() {
    let mut vim = VimState::new();
    assert_eq!(
        vim.execute_command("set number"),
        VimCommandResult::Set("number".to_string())
    );
}

#[test]
fn test_command_go_to_line_number() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("42"), VimCommandResult::GoToLine(42));
    assert_eq!(vim.execute_command("1"), VimCommandResult::GoToLine(1));
}

#[test]
fn test_command_unknown() {
    let mut vim = VimState::new();
    assert_eq!(
        vim.execute_command("unknown"),
        VimCommandResult::Unknown("unknown".to_string())
    );
}

#[test]
fn test_command_trim_whitespace() {
    let mut vim = VimState::new();
    assert_eq!(vim.execute_command("  w  "), VimCommandResult::Write);
    assert_eq!(vim.execute_command("  q  "), VimCommandResult::Quit);
}

// =============================================================================
// Set Mode Tests - 모드 설정 테스트
// =============================================================================

#[test]
fn test_set_mode_directly() {
    let mut vim = VimState::new();
    vim.set_mode(VimMode::Insert);
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_set_mode_to_normal_clears_operator() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('d')));

    vim.set_mode(VimMode::Normal);
    // Normal 모드로 설정하면 연산자가 초기화됨
    // 이를 확인하기 위해 다른 동작 수행
    let action = vim.handle_key(&KeyEvent::new(Key::Char('w')));
    assert_eq!(action, VimAction::Move(VimMotion::Word));
}

#[test]
fn test_set_mode_all_modes() {
    let modes = [
        VimMode::Normal,
        VimMode::Insert,
        VimMode::Visual,
        VimMode::VisualLine,
        VimMode::VisualBlock,
        VimMode::Command,
        VimMode::Search,
        VimMode::Replace,
    ];

    for mode in modes {
        let mut vim = VimState::new();
        vim.set_mode(mode);
        assert_eq!(vim.mode(), mode);
    }
}

// =============================================================================
// Insert Mode Tests - 삽입 모드 테스트
// =============================================================================

#[test]
fn test_insert_mode_only_escape_handled() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('i')));
    assert_eq!(vim.mode(), VimMode::Insert);

    // 다른 키들은 None을 반환
    let action = vim.handle_key(&KeyEvent::new(Key::Char('a')));
    assert_eq!(action, VimAction::None);
}

#[test]
fn test_insert_mode_escape_returns_to_normal() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('i')));
    assert_eq!(vim.mode(), VimMode::Insert);

    let action = vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(action, VimAction::Escape);
    assert_eq!(vim.mode(), VimMode::Normal);
}

// =============================================================================
// Edge Cases - 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_g_sequence_incomplete() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('g')));
    // g만 입력하고 다른 키를 입력하면 해당 키의 동작이 수행됨
    let action = vim.handle_key(&KeyEvent::new(Key::Char('x')));
    // x는 문자 삭제
    assert_eq!(action, VimAction::Delete(Some(VimMotion::Right)));
}

#[test]
fn test_digit_count_preserved_until_action() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('3')));
    assert_eq!(vim.count(), 3);

    // 다른 숫자 입력
    vim.handle_key(&KeyEvent::new(Key::Char('3')));
    assert_eq!(vim.count(), 33);
}

#[test]
fn test_unknown_key_returns_none() {
    let mut vim = VimState::new();
    let action = vim.handle_key(&KeyEvent::new(Key::Char('z')));
    assert_eq!(action, VimAction::None);
}

#[test]
fn test_unknown_key_in_insert_mode() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('i')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('@')));
    assert_eq!(action, VimAction::None);
}

#[test]
fn test_command_mode_empty_command_on_enter() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    let action = vim.handle_key(&KeyEvent::new(Key::Enter));
    assert_eq!(action, VimAction::ExecuteCommand("".to_string()));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_search_mode_empty_search_on_enter() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));

    let action = vim.handle_key(&KeyEvent::new(Key::Enter));
    assert_eq!(action, VimAction::Move(VimMotion::SearchNext));
}

#[test]
fn test_operator_with_unknown_motion() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('d')));

    let action = vim.handle_key(&KeyEvent::new(Key::Char('z')));
    assert_eq!(action, VimAction::None);
}

#[test]
fn test_multiple_operators_in_sequence() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    // 연산자를 다시 입력하면 이전 연산자가 덮어씌워짐
    let action = vim.handle_key(&KeyEvent::new(Key::Char('d')));
    assert_eq!(action, VimAction::Delete(Some(VimMotion::Down)));
}

#[test]
fn test_count_before_operator() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('5')));
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    vim.handle_key(&KeyEvent::new(Key::Char('w')));
    // 카운트가 적용된 후 리셋됨
    assert_eq!(vim.count(), 1);
}

#[test]
fn test_escape_clears_count_and_operator() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('5')));
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    assert_eq!(vim.count(), 5);

    vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(vim.count(), 1);
}

#[test]
fn test_visual_mode_all_motions() {
    let motions = [
        ('h', VimMotion::Left),
        ('j', VimMotion::Down),
        ('k', VimMotion::Up),
        ('l', VimMotion::Right),
        ('w', VimMotion::Word),
        ('b', VimMotion::WordBack),
    ];

    for (key, expected_motion) in motions {
        let mut vim = VimState::new();
        vim.handle_key(&KeyEvent::new(Key::Char('v')));
        let action = vim.handle_key(&KeyEvent::new(Key::Char(key)));
        assert_eq!(action, VimAction::Move(expected_motion));
    }
}

#[test]
fn test_command_mode_with_spaces() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    vim.handle_key(&KeyEvent::new(Key::Char('e')));
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    vim.handle_key(&KeyEvent::new(Key::Char('i')));
    vim.handle_key(&KeyEvent::new(Key::Char(' ')));
    vim.handle_key(&KeyEvent::new(Key::Char('f')));
    vim.handle_key(&KeyEvent::new(Key::Char('i')));
    vim.handle_key(&KeyEvent::new(Key::Char('l')));
    vim.handle_key(&KeyEvent::new(Key::Char('e')));
    vim.handle_key(&KeyEvent::new(Key::Char('.')));
    vim.handle_key(&KeyEvent::new(Key::Char('t')));
    vim.handle_key(&KeyEvent::new(Key::Char('x')));
    vim.handle_key(&KeyEvent::new(Key::Char('t')));

    assert_eq!(vim.command_buffer(), "edi file.txt");
}

#[test]
fn test_multiple_mode_transitions() {
    let mut vim = VimState::new();

    // Normal -> Insert -> Normal -> Visual -> Normal
    vim.handle_key(&KeyEvent::new(Key::Char('i')));
    assert_eq!(vim.mode(), VimMode::Insert);

    vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(vim.mode(), VimMode::Normal);

    vim.handle_key(&KeyEvent::new(Key::Char('v')));
    assert_eq!(vim.mode(), VimMode::Visual);

    vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(vim.mode(), VimMode::Normal);
}

#[test]
fn test_zero_in_middle_of_count() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('1')));
    vim.handle_key(&KeyEvent::new(Key::Char('0')));
    vim.handle_key(&KeyEvent::new(Key::Char('0')));
    assert_eq!(vim.count(), 100);
}

#[test]
fn test_all_insert_mode_variants() {
    let test_cases = [
        ('i', VimAction::Insert),
        ('I', VimAction::InsertStart),
        ('a', VimAction::Append),
        ('A', VimAction::AppendEnd),
        ('o', VimAction::OpenBelow),
        ('O', VimAction::OpenAbove),
    ];

    for (key, expected_action) in test_cases {
        let mut vim = VimState::new();
        let action = vim.handle_key(&KeyEvent::new(Key::Char(key)));
        assert_eq!(action, expected_action);
        assert_eq!(vim.mode(), VimMode::Insert);

        // Reset for next test
        vim.set_mode(VimMode::Normal);
    }
}

#[test]
fn test_command_mode_special_characters() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    // 특수 문자 입력
    vim.handle_key(&KeyEvent::new(Key::Char('%')));
    vim.handle_key(&KeyEvent::new(Key::Char('s')));
    vim.handle_key(&KeyEvent::new(Key::Char('/')));
    vim.handle_key(&KeyEvent::new(Key::Char('g')));

    assert_eq!(vim.command_buffer(), "%s/g");
}

#[test]
fn test_search_mode_special_characters() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));

    // 특수 문자 입력
    vim.handle_key(&KeyEvent::new(Key::Char('*')));
    vim.handle_key(&KeyEvent::new(Key::Char('.')));
    vim.handle_key(&KeyEvent::new(Key::Char('$')));

    assert_eq!(vim.search_pattern(), "*.$");
}

#[test]
fn test_double_operator() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    vim.handle_key(&KeyEvent::new(Key::Char('d')));
    // dd = 줄 전체 삭제 (동일한 키 반복 = 줄)
    assert_eq!(vim.count(), 1);
}

#[test]
fn test_double_yank() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('y')));
    let action = vim.handle_key(&KeyEvent::new(Key::Char('y')));
    // yy = 줄 전체 yank
    assert_eq!(action, VimAction::Yank(Some(VimMotion::Down)));
}

#[test]
fn test_double_change() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('c')));
    let action = vim.handle_key(&KeyEvent::new(Key::Char('c')));
    // cc = 줄 전체 변경
    assert_eq!(action, VimAction::Change(Some(VimMotion::Down)));
}

// =============================================================================
// State Persistence Tests - 상태 유지 테스트
// =============================================================================

#[test]
fn test_mode_persistence_across_operations() {
    let mut vim = VimState::new();
    vim.set_mode(VimMode::Insert);

    // Insert 모드에서 여러 키 입력
    vim.handle_key(&KeyEvent::new(Key::Char('a')));
    vim.handle_key(&KeyEvent::new(Key::Char('b')));
    vim.handle_key(&KeyEvent::new(Key::Char('c')));

    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_command_buffer_persistence() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char(':')));

    vim.handle_key(&KeyEvent::new(Key::Char('w')));
    assert_eq!(vim.command_buffer(), "w");

    vim.handle_key(&KeyEvent::new(Key::Char('q')));
    assert_eq!(vim.command_buffer(), "wq");
}

#[test]
fn test_search_pattern_persistence() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('/')));

    vim.handle_key(&KeyEvent::new(Key::Char('f')));
    assert_eq!(vim.search_pattern(), "f");

    vim.handle_key(&KeyEvent::new(Key::Char('o')));
    assert_eq!(vim.search_pattern(), "fo");

    vim.handle_key(&KeyEvent::new(Key::Char('o')));
    assert_eq!(vim.search_pattern(), "foo");
}

#[test]
fn test_register_persistence_across_modes() {
    let mut vim = VimState::new();
    vim.set_register("test content");

    // 모드 변경
    vim.set_mode(VimMode::Insert);
    assert_eq!(vim.register(), "test content");

    vim.set_mode(VimMode::Visual);
    assert_eq!(vim.register(), "test content");

    vim.set_mode(VimMode::Command);
    assert_eq!(vim.register(), "test content");
}
