//! Normal mode: counts, pending operators, mode keys, motions and commands

use super::{VimAction, VimMode, VimMotion, VimState};
use crate::event::{Key, KeyEvent};

/// The motions normal and visual mode share: h j k l (and the arrows), w, b
pub(super) fn basic_motion(key: &Key) -> Option<VimMotion> {
    match key {
        Key::Char('h') | Key::Left => Some(VimMotion::Left),
        Key::Char('j') | Key::Down => Some(VimMotion::Down),
        Key::Char('k') | Key::Up => Some(VimMotion::Up),
        Key::Char('l') | Key::Right => Some(VimMotion::Right),
        Key::Char('w') => Some(VimMotion::Word),
        Key::Char('b') => Some(VimMotion::WordBack),
        _ => None,
    }
}

impl VimState {
    /// Handle key event in normal mode
    pub(super) fn handle_normal(&mut self, key: &KeyEvent) -> VimAction {
        if self.take_count_digit(key) {
            return VimAction::None;
        }
        if let Some(action) = self.complete_operator(key) {
            return action;
        }
        if let Some(action) = self.normal_mode_change(key) {
            return action;
        }
        if let Some(action) = self.normal_motion(key) {
            return action;
        }
        self.normal_command(key)
    }

    /// Add a digit to the count; true when `key` was one. A leading `0` is
    /// not a count digit: as in vim, it moves to the line start unless a
    /// count is already being typed.
    fn take_count_digit(&mut self, key: &KeyEvent) -> bool {
        if let Key::Char(ch) = key.key {
            if let Some(digit) = ch.to_digit(10).filter(|&d| d != 0 || self.count.is_some()) {
                let digit = digit as usize;
                self.count = Some(self.count.unwrap_or(0) * 10 + digit);
                return true;
            }
        }
        false
    }

    /// With an operator pending, a motion key completes it
    fn complete_operator(&mut self, key: &KeyEvent) -> Option<VimAction> {
        let op = self.operator?;
        if let Key::Char(ch) = key.key {
            let motion = self.char_to_motion(ch);
            if motion.is_some() {
                self.operator = None;
                return Some(match op {
                    'd' => VimAction::Delete(motion),
                    'y' => VimAction::Yank(motion),
                    'c' => VimAction::Change(motion),
                    _ => VimAction::None,
                });
            }
        }
        None
    }

    /// Keys that switch to insert, visual, command or search mode
    fn normal_mode_change(&mut self, key: &KeyEvent) -> Option<VimAction> {
        let (mode, action) = match key.key {
            Key::Char('i') => (VimMode::Insert, VimAction::Insert),
            Key::Char('I') => (VimMode::Insert, VimAction::InsertStart),
            Key::Char('a') => (VimMode::Insert, VimAction::Append),
            Key::Char('A') => (VimMode::Insert, VimAction::AppendEnd),
            Key::Char('o') => (VimMode::Insert, VimAction::OpenBelow),
            Key::Char('O') => (VimMode::Insert, VimAction::OpenAbove),
            Key::Char('v') => (VimMode::Visual, VimAction::EnterVisual),
            Key::Char('V') => (VimMode::VisualLine, VimAction::EnterVisualLine),
            Key::Char(':') => {
                self.set_mode(VimMode::Command);
                self.command_buffer.clear();
                return Some(VimAction::EnterCommand);
            }
            Key::Char('/') => {
                self.set_mode(VimMode::Search);
                self.search_pattern.clear();
                self.search_forward = true;
                return Some(VimAction::EnterSearch);
            }
            Key::Char('?') => {
                self.set_mode(VimMode::Search);
                self.search_pattern.clear();
                self.search_forward = false;
                return Some(VimAction::EnterSearch);
            }
            _ => return None,
        };
        self.set_mode(mode);
        Some(action)
    }

    /// Motion keys; `g` waits for a second `g`
    fn normal_motion(&mut self, key: &KeyEvent) -> Option<VimAction> {
        if let Some(motion) = basic_motion(&key.key) {
            return Some(VimAction::Move(motion));
        }
        let motion = match key.key {
            Key::Char('e') => VimMotion::WordEnd,
            Key::Char('0') => VimMotion::LineStart,
            Key::Char('$') => VimMotion::LineEnd,
            Key::Char('^') => VimMotion::FirstNonBlank,
            Key::Char('G') => VimMotion::GoToLine(self.count),
            Key::Char('g') => {
                self.key_buffer.push('g');
                return Some(VimAction::None);
            }
            Key::Char('{') => VimMotion::ParagraphBack,
            Key::Char('}') => VimMotion::ParagraphForward,
            Key::Char('%') => VimMotion::MatchBracket,
            Key::Char('n') => VimMotion::SearchNext,
            Key::Char('N') => VimMotion::SearchPrev,
            _ => return None,
        };
        Some(VimAction::Move(motion))
    }

    /// Operators (which wait for a motion), edits and Escape
    fn normal_command(&mut self, key: &KeyEvent) -> VimAction {
        match key.key {
            // Operators
            Key::Char(op @ ('d' | 'y' | 'c')) => {
                self.operator = Some(op);
                VimAction::None
            }

            // Actions
            Key::Char('x') => VimAction::Delete(Some(VimMotion::Right)),
            Key::Char('X') => VimAction::Delete(Some(VimMotion::Left)),
            Key::Char('p') => VimAction::PasteAfter,
            Key::Char('P') => VimAction::PasteBefore,
            Key::Char('u') => VimAction::Undo,
            Key::Char('r') if key.ctrl => VimAction::Redo,
            Key::Char('.') => VimAction::Repeat,
            Key::Char('J') => VimAction::JoinLines,
            Key::Char('>') => VimAction::Indent,
            Key::Char('<') => VimAction::Outdent,

            Key::Escape => {
                self.count = None;
                self.operator = None;
                VimAction::Escape
            }

            _ => VimAction::None,
        }
    }

    /// Convert character to motion
    fn char_to_motion(&self, ch: char) -> Option<VimMotion> {
        match ch {
            'h' => Some(VimMotion::Left),
            'j' => Some(VimMotion::Down),
            'k' => Some(VimMotion::Up),
            'l' => Some(VimMotion::Right),
            'w' => Some(VimMotion::Word),
            'b' => Some(VimMotion::WordBack),
            'e' => Some(VimMotion::WordEnd),
            '0' => Some(VimMotion::LineStart),
            '$' => Some(VimMotion::LineEnd),
            '^' => Some(VimMotion::FirstNonBlank),
            'G' => Some(VimMotion::GoToLine(None)),
            '{' => Some(VimMotion::ParagraphBack),
            '}' => Some(VimMotion::ParagraphForward),
            '%' => Some(VimMotion::MatchBracket),
            // Same key repeats = line
            'd' | 'y' | 'c' => Some(VimMotion::Down),
            _ => None,
        }
    }
}
