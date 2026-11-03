//! The keys VimAction, VimMode and VimMotion promise (#1081), and counts
//! around an operator (#1363)

use revue::event::{Key, KeyEvent};
use revue::widget::{VimAction, VimCommandResult, VimMode, VimMotion, VimState};

// =========================================================================
// #1081 and #1363: the keys the public types promise
// =========================================================================

#[test]
fn test_vim_r_waits_for_the_replacement_char() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('r')));
    assert_eq!(vim.mode(), VimMode::Replace);
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('x'))),
        VimAction::ReplaceChar('x')
    );
}

#[test]
fn test_vim_ctrl_v_enters_visual_block() {
    let mut vim = VimState::new();
    assert_eq!(
        vim.handle_key(&KeyEvent::ctrl(Key::Char('v'))),
        VimAction::EnterVisualBlock
    );
}

#[test]
fn test_vim_f_waits_for_the_char_to_find() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('f')));
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('x'))),
        VimAction::Move(VimMotion::FindChar('x'))
    );
}

#[test]
fn test_vim_ctrl_u_is_not_undo() {
    let mut vim = VimState::new();
    assert_ne!(
        vim.handle_key(&KeyEvent::ctrl(Key::Char('u'))),
        VimAction::Undo
    );
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::ctrl(Key::Char('d')));
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('w'))),
        VimAction::Move(VimMotion::Word)
    );
}

#[test]
fn test_vim_long_count_does_not_panic() {
    let mut vim = VimState::new();
    for _ in 0..25 {
        vim.handle_key(&KeyEvent::new(Key::Char('9')));
    }
}

#[test]
fn test_vim_change_motion_enters_insert() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('c')));
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('w'))),
        VimAction::Change(Some(VimMotion::Word))
    );
    assert_eq!(vim.mode(), VimMode::Insert);
}

#[test]
fn test_vim_backspace_keeps_the_empty_command_line() {
    let mut vim = VimState::new();
    for ch in ":w".chars() {
        vim.handle_key(&KeyEvent::new(Key::Char(ch)));
    }
    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.mode(), VimMode::Command);
}

#[test]
fn test_vim_empty_command_is_not_go_to_line_1() {
    let mut vim = VimState::new();
    assert_ne!(vim.execute_command(""), VimCommandResult::GoToLine(1));
    assert_ne!(
        vim.execute_command("99999999999999999999999"),
        VimCommandResult::GoToLine(1)
    );
}

#[test]
fn test_vim_count_before_and_after_an_operator_multiplies() {
    let mut vim = VimState::new();
    for ch in "2d3w".chars() {
        vim.handle_key(&KeyEvent::new(Key::Char(ch)));
    }
    assert_eq!(vim.count(), 6);
}

/// `R` replaces char after char until Escape; `r` replaces one (#1081)
#[test]
fn test_vim_big_r_replaces_until_escape() {
    let mut vim = VimState::new();
    vim.handle_key(&KeyEvent::new(Key::Char('R')));
    assert_eq!(vim.mode(), VimMode::Replace);
    for ch in "ab".chars() {
        assert_eq!(
            vim.handle_key(&KeyEvent::new(Key::Char(ch))),
            VimAction::ReplaceChar(ch)
        );
        assert_eq!(vim.mode(), VimMode::Replace);
    }
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Escape)),
        VimAction::Escape
    );
    assert_eq!(vim.mode(), VimMode::Normal);
    // `r` after `R`: one char again
    vim.handle_key(&KeyEvent::new(Key::Char('r')));
    vim.handle_key(&KeyEvent::new(Key::Char('z')));
    assert_eq!(vim.mode(), VimMode::Normal);
    // `3rx`: the count goes with the replace
    vim.handle_key(&KeyEvent::new(Key::Char('3')));
    vim.handle_key(&KeyEvent::new(Key::Char('r')));
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('x'))),
        VimAction::ReplaceChar('x')
    );
    assert_eq!(vim.count(), 3);
}

/// `F`, `t` and `T` wait for their char too, and so does each after an
/// operator: `dtx` deletes till `x` (#1081)
#[test]
fn test_vim_find_and_till_motions_with_and_without_an_operator() {
    let press = |vim: &mut VimState, keys: &str| {
        keys.chars()
            .map(|ch| vim.handle_key(&KeyEvent::new(Key::Char(ch))))
            .last()
            .unwrap()
    };
    let mut vim = VimState::new();
    assert_eq!(
        press(&mut vim, "Fq"),
        VimAction::Move(VimMotion::FindCharBack('q'))
    );
    assert_eq!(
        press(&mut vim, "t;"),
        VimAction::Move(VimMotion::TillChar(';'))
    );
    assert_eq!(
        press(&mut vim, "T "),
        VimAction::Move(VimMotion::TillCharBack(' '))
    );
    // A digit or a motion key is the char to find, not a count or motion
    assert_eq!(
        press(&mut vim, "f3"),
        VimAction::Move(VimMotion::FindChar('3'))
    );
    assert_eq!(
        press(&mut vim, "dtx"),
        VimAction::Delete(Some(VimMotion::TillChar('x')))
    );
    assert_eq!(
        press(&mut vim, "cfx"),
        VimAction::Change(Some(VimMotion::FindChar('x')))
    );
    assert_eq!(vim.mode(), VimMode::Insert);
    // Escape instead of the char cancels the find and the operator
    let mut vim = VimState::new();
    press(&mut vim, "df");
    vim.handle_key(&KeyEvent::new(Key::Escape));
    assert_eq!(press(&mut vim, "w"), VimAction::Move(VimMotion::Word));
}

/// The counts before and after an operator multiply, for the action and for
/// `count` while it waits (#1363)
#[test]
fn test_vim_operator_counts_multiply() {
    let mut vim = VimState::new();
    for ch in "2d".chars() {
        vim.handle_key(&KeyEvent::new(Key::Char(ch)));
    }
    assert_eq!(vim.count(), 2);
    vim.handle_key(&KeyEvent::new(Key::Char('3')));
    assert_eq!(vim.count(), 6);
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('w'))),
        VimAction::Delete(Some(VimMotion::Word))
    );
    assert_eq!(vim.count(), 6);
    // One side only: that count
    let mut vim = VimState::new();
    for ch in "d4w".chars() {
        vim.handle_key(&KeyEvent::new(Key::Char(ch)));
    }
    assert_eq!(vim.count(), 4);
    // `2d0`: the 0 after the operator is the motion, not a digit
    let mut vim = VimState::new();
    for ch in "2d".chars() {
        vim.handle_key(&KeyEvent::new(Key::Char(ch)));
    }
    assert_eq!(
        vim.handle_key(&KeyEvent::new(Key::Char('0'))),
        VimAction::Delete(Some(VimMotion::LineStart))
    );
}

/// The search line stays open on Backspace until it is empty, as the
/// command line does; a line number past usize goes to the last line
#[test]
fn test_vim_search_backspace_and_huge_line_numbers() {
    let mut vim = VimState::new();
    for ch in "/a".chars() {
        vim.handle_key(&KeyEvent::new(Key::Char(ch)));
    }
    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.mode(), VimMode::Search);
    vim.handle_key(&KeyEvent::new(Key::Backspace));
    assert_eq!(vim.mode(), VimMode::Normal);
    assert_eq!(
        vim.execute_command("99999999999999999999999"),
        VimCommandResult::GoToLine(usize::MAX)
    );
    assert_eq!(
        vim.execute_command(""),
        VimCommandResult::Unknown(String::new())
    );
    assert_eq!(vim.execute_command("12"), VimCommandResult::GoToLine(12));
}
