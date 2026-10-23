//! EditState: grapheme positions, words, the goal column, cursors, undo
//! grouping, single-line and read-only rules, and the standard keys

use std::cell::Cell;
use std::rc::Rc;
use std::time::{Duration, Instant};

use revue::event::{Key, KeyEvent};
use revue::utils::{EditCursor, EditMotion, EditOutcome, EditPos, EditState};

mod model;

fn p(line: usize, col: usize) -> EditPos {
    EditPos::new(line, col)
}

/// A clock the test moves by hand
fn clock() -> (Rc<Cell<Instant>>, impl Fn() -> Instant + 'static) {
    let now = Rc::new(Cell::new(Instant::now()));
    let shared = now.clone();
    (now, move || shared.get())
}

fn wait(now: &Cell<Instant>, ms: u64) {
    now.set(now.get() + Duration::from_millis(ms));
}

fn at(text: &str, pos: EditPos) -> EditState {
    let mut s = EditState::multi_line().with_text(text);
    s.set_cursor(pos);
    s
}

fn key(k: Key) -> KeyEvent {
    KeyEvent::new(k)
}

// ---------------------------------------------------------------------------
// Grapheme clusters
// ---------------------------------------------------------------------------

#[test]
fn positions_are_grapheme_clusters() {
    // "e" + combining acute is one cluster; so is a family emoji
    let mut s = at("e\u{301}👨\u{200D}👩\u{200D}👧x", p(0, 0));
    assert_eq!(s.line_len(0), 3);
    s.move_cursors(EditMotion::Right, false);
    assert_eq!(s.cursor(), p(0, 1));
    s.move_cursors(EditMotion::Right, false);
    assert_eq!(s.cursor(), p(0, 2));
    // Delete takes the whole cluster after the cursor
    s.set_cursor(p(0, 0));
    assert!(s.delete());
    assert_eq!(s.text(), "👨\u{200D}👩\u{200D}👧x");
}

#[test]
fn backspace_takes_the_last_code_point_but_an_emoji_sequence_whole() {
    for (text, left) in [
        // Devanagari ka + vowel sign i: the vowel sign goes
        ("\u{915}\u{93F}", "\u{915}"),
        // Hangul jamo L + V, one cluster: the vowel goes
        ("\u{1100}\u{1161}", "\u{1100}"),
        // e + combining acute: the accent goes
        ("e\u{301}", "e"),
        // Precomposed syllable and plain letters: one code point, all of it
        ("한", ""),
        ("a", ""),
        // Emoji sequences go whole
        ("👨\u{200D}👩\u{200D}👧", ""),
        ("🇰🇷", ""),
        ("❤\u{FE0F}", ""),
        ("👍🏽", ""),
        ("1\u{FE0F}\u{20E3}", ""),
    ] {
        let mut s = at(text, p(0, 1));
        assert!(s.backspace(), "{text:?}");
        assert_eq!(s.text(), left, "{text:?}");
        // The cursor stays on a cluster boundary: after what is left
        assert_eq!(s.cursor(), p(0, s.line_len(0)), "{text:?}");
    }
}

#[test]
fn backspace_and_delete_cross_line_breaks() {
    let mut s = at("ab\ncd", p(1, 0));
    assert!(s.backspace());
    assert_eq!((s.text(), s.cursor()), ("abcd".to_string(), p(0, 2)));
    assert!(s.delete());
    assert_eq!(s.text(), "abd");
    let mut s = at("ab\ncd", p(0, 2));
    assert!(s.delete());
    assert_eq!(s.text(), "abcd");
    // Nothing before the start or after the end
    assert!(!at("ab", p(0, 0)).backspace());
    assert!(!at("ab", p(0, 2)).delete());
}

// ---------------------------------------------------------------------------
// Words
// ---------------------------------------------------------------------------

fn word_stops(text: &str, forward: bool) -> Vec<usize> {
    let mut s = at(
        text,
        if forward {
            p(0, 0)
        } else {
            p(0, text.chars().count())
        },
    );
    let mut stops = Vec::new();
    loop {
        let motion = if forward {
            EditMotion::WordRight
        } else {
            EditMotion::WordLeft
        };
        if !s.move_cursors(motion, false) {
            return stops;
        }
        stops.push(s.cursor().col);
    }
}

#[test]
fn word_moves_go_by_category_runs() {
    // foo . bar ( x )
    assert_eq!(word_stops("foo.bar(x)", true), [3, 4, 7, 8, 9, 10]);
    assert_eq!(word_stops("foo.bar(x)", false), [9, 8, 7, 4, 3, 0]);
    // Whitespace is skipped before the run
    assert_eq!(word_stops("  foo  bar", true), [5, 10]);
    assert_eq!(word_stops("  foo  bar", false), [7, 2, 0]);
    // Letters of any script and numbers are word characters
    assert_eq!(word_stops("한글 단어_2 ->", true), [2, 7, 10]);
}

#[test]
fn word_moves_cross_line_breaks() {
    let mut s = at("ab\ncd", p(0, 2));
    s.move_cursors(EditMotion::WordRight, false);
    assert_eq!(s.cursor(), p(1, 0));
    s.move_cursors(EditMotion::WordLeft, false);
    assert_eq!(s.cursor(), p(0, 2));
}

#[test]
fn word_deletes_skip_one_space_only() {
    // One space next to the cursor goes with the word
    let mut s = at("foo bar ", p(0, 8));
    assert!(s.delete_word_backward());
    assert_eq!(s.text(), "foo ");
    // Two or more spaces are a word of their own
    let mut s = at("foo bar  ", p(0, 9));
    assert!(s.delete_word_backward());
    assert_eq!(s.text(), "foo bar");
    // Forward the same
    let mut s = at(" foo.bar", p(0, 0));
    assert!(s.delete_word_forward());
    assert_eq!(s.text(), ".bar");
    // At a line edge, the line break
    let mut s = at("ab\ncd", p(1, 0));
    assert!(s.delete_word_backward());
    assert_eq!(s.text(), "abcd");
}

// ---------------------------------------------------------------------------
// Up and Down
// ---------------------------------------------------------------------------

#[test]
fn up_and_down_keep_the_goal_column() {
    let mut s = at("abcdef\nab\nabcdef", p(0, 5));
    s.move_cursors(EditMotion::Down, false);
    assert_eq!(s.cursor(), p(1, 2));
    s.move_cursors(EditMotion::Down, false);
    assert_eq!(s.cursor(), p(2, 5));
    // A sideways move sets a new goal
    s.move_cursors(EditMotion::Left, false);
    s.move_cursors(EditMotion::Up, false);
    s.move_cursors(EditMotion::Up, false);
    assert_eq!(s.cursor(), p(0, 4));
}

#[test]
fn the_goal_column_is_a_screen_column() {
    // Column 2 of "한글한글" is screen column 4
    let mut s = at("한글한글\nabcdef\n한글한글", p(0, 2));
    s.move_cursors(EditMotion::Down, false);
    assert_eq!(s.cursor(), p(1, 4));
    // From screen column 5 the wide glyph that covers it is chosen
    s.set_cursor(p(1, 5));
    s.move_cursors(EditMotion::Down, false);
    assert_eq!(s.cursor(), p(2, 2));
}

#[test]
fn up_from_the_first_line_and_down_from_the_last_go_to_the_ends() {
    let mut s = at("abc\ndef", p(0, 2));
    s.move_cursors(EditMotion::Up, false);
    assert_eq!(s.cursor(), p(0, 0));
    s.set_cursor(p(1, 1));
    s.move_cursors(EditMotion::Down, false);
    assert_eq!(s.cursor(), p(1, 3));
    // The goal column (1) stays through the move to the line's end
    s.move_cursors(EditMotion::PageUp(10), false);
    assert_eq!(s.cursor(), p(0, 1));
}

// ---------------------------------------------------------------------------
// Selections
// ---------------------------------------------------------------------------

#[test]
fn shift_moves_select_and_plain_moves_collapse_to_the_side() {
    let mut s = at("hello world", p(0, 2));
    s.move_cursors(EditMotion::WordRight, true);
    assert_eq!(s.selected_text(), "llo");
    // Left from a selection goes to its start, Right to its end
    s.move_cursors(EditMotion::Left, false);
    assert_eq!((s.cursor(), s.has_selection()), (p(0, 2), false));
    s.select(p(0, 2), p(0, 5));
    s.move_cursors(EditMotion::Right, false);
    assert_eq!(s.cursor(), p(0, 5));
    // Typing replaces the selection
    s.select(p(0, 0), p(0, 5));
    assert!(s.insert("bye"));
    assert_eq!(s.text(), "bye world");
}

#[test]
fn select_all_and_selected_text_across_lines() {
    let mut s = at("ab\ncd", p(0, 0));
    s.select_all();
    assert_eq!(s.selected_text(), "ab\ncd");
    assert!(s.backspace());
    assert!(s.is_empty());
}

// ---------------------------------------------------------------------------
// Several cursors
// ---------------------------------------------------------------------------

#[test]
fn an_edit_applies_at_every_cursor() {
    let mut s = at("ab\ncd\nef", p(0, 1));
    s.add_cursor(EditCursor::at(p(1, 1)));
    s.add_cursor(EditCursor::at(p(2, 1)));
    assert!(s.insert("X"));
    assert_eq!(s.text(), "aXb\ncXd\neXf");
    let heads: Vec<EditPos> = s.cursors().iter().map(EditCursor::head).collect();
    assert_eq!(heads, [p(0, 2), p(1, 2), p(2, 2)]);
    // A new line at every cursor moves the later ones down
    assert!(s.insert("\n"));
    assert_eq!(s.text(), "aX\nb\ncX\nd\neX\nf");
    let heads: Vec<EditPos> = s.cursors().iter().map(EditCursor::head).collect();
    assert_eq!(heads, [p(1, 0), p(3, 0), p(5, 0)]);
    assert!(s.backspace());
    assert_eq!(s.text(), "aXb\ncXd\neXf");
}

#[test]
fn cursors_that_meet_merge_and_the_primary_is_kept() {
    let mut s = at("abcdef", p(0, 2));
    s.add_cursor(EditCursor::at(p(0, 4)));
    assert_eq!(s.cursor(), p(0, 4));
    assert_eq!(s.cursors().len(), 2);
    // Two word deletes reaching the same word are one
    assert!(s.delete_word_backward());
    assert_eq!(s.text(), "ef");
    assert_eq!(s.cursors().len(), 1);
    // A cursor at the end of a selection merges; selections that only
    // touch do not
    let mut s = at("abcdef", p(0, 0));
    s.select(p(0, 0), p(0, 3));
    s.add_cursor(EditCursor::at(p(0, 3)));
    assert_eq!(s.cursors().len(), 1);
    s.add_cursor(EditCursor::selecting(p(0, 3), p(0, 5)));
    assert_eq!(s.cursors().len(), 2);
    assert_eq!(s.cursor(), p(0, 5));
    s.clear_secondary();
    assert_eq!(s.cursors(), [EditCursor::selecting(p(0, 3), p(0, 5))]);
}

#[test]
fn line_commands_take_every_line_a_cursor_touches() {
    let mut s = at("1\n2\n3\n4", p(0, 0));
    s.add_cursor(EditCursor::at(p(2, 0)));
    assert!(s.duplicate_lines());
    assert_eq!(s.text(), "1\n1\n2\n3\n3\n4");
    let heads: Vec<EditPos> = s.cursors().iter().map(EditCursor::head).collect();
    assert_eq!(heads, [p(1, 0), p(4, 0)]);
    assert!(s.delete_lines());
    assert_eq!(s.text(), "1\n2\n3\n4");
    // Two cursors on one line copy it once
    let mut s = at("ab\ncd", p(0, 0));
    s.add_cursor(EditCursor::at(p(0, 2)));
    assert!(s.duplicate_lines());
    assert_eq!(s.text(), "ab\nab\ncd");
    // The last line goes with the break before it
    let mut s = at("ab\ncd", p(1, 1));
    assert!(s.delete_lines());
    assert_eq!(s.text(), "ab");
}

// ---------------------------------------------------------------------------
// Undo
// ---------------------------------------------------------------------------

#[test]
fn typing_close_together_is_one_undo_step() {
    let (now, clock) = clock();
    let mut s = EditState::multi_line().clock(clock);
    for c in ["a", "b", "c"] {
        wait(&now, 100);
        s.insert(c);
    }
    // A pause longer than the delay starts a new step
    wait(&now, 600);
    s.insert("d");
    wait(&now, 100);
    // Backspace joins typing
    s.backspace();
    s.insert("e");
    assert_eq!(s.text(), "abce");
    assert!(s.undo());
    assert_eq!(s.text(), "abc");
    assert!(s.undo());
    assert_eq!(s.text(), "");
    assert!(!s.undo());
    assert!(s.redo() && s.redo());
    assert_eq!(s.text(), "abce");
}

#[test]
fn a_move_or_another_kind_of_edit_starts_a_new_step() {
    let (now, clock) = clock();
    let mut s = EditState::multi_line().clock(clock);
    s.insert("ab");
    wait(&now, 10);
    s.move_cursors(EditMotion::Left, false);
    s.move_cursors(EditMotion::Right, false);
    s.insert("c");
    wait(&now, 10);
    s.paste("PP");
    wait(&now, 10);
    s.insert("d");
    wait(&now, 10);
    s.insert("\n");
    let mut texts = vec![s.text()];
    while s.undo() {
        texts.push(s.text());
    }
    assert_eq!(texts, ["abcPPd\n", "abcPPd", "abcPP", "abc", "ab", ""]);
}

#[test]
fn undo_puts_back_the_cursors_and_selections() {
    let mut s = at("hello world", p(0, 0));
    s.select(p(0, 6), p(0, 11));
    s.add_cursor(EditCursor::selecting(p(0, 0), p(0, 5)));
    let before = s.cursors().to_vec();
    let primary = s.cursor();
    assert!(s.insert("X"));
    assert_eq!(s.text(), "X X");
    assert!(s.undo());
    assert_eq!(s.text(), "hello world");
    assert_eq!(s.cursors(), before);
    assert_eq!(s.cursor(), primary);
    assert!(s.redo());
    assert_eq!(s.text(), "X X");
    assert_eq!(
        s.cursors(),
        [EditCursor::at(p(0, 1)), EditCursor::at(p(0, 3))]
    );
}

#[test]
fn the_history_keeps_its_limit_and_a_new_edit_drops_redo() {
    let mut s = EditState::multi_line().history_limit(2);
    for c in ["a\n", "b\n", "c\n"] {
        s.paste(c);
    }
    assert!(s.undo() && s.undo());
    assert!(!s.undo());
    assert_eq!(s.text(), "a\n");
    s.paste("z");
    assert!(!s.can_redo());
}

// ---------------------------------------------------------------------------
// Rules for the text
// ---------------------------------------------------------------------------

#[test]
fn a_single_line_field_keeps_one_line() {
    let mut s = EditState::single_line().with_text("a\nb\r\nc");
    assert_eq!(s.text(), "a b c");
    s.set_cursor(s.end());
    assert!(s.paste("\nd\n"));
    assert_eq!(s.text(), "a b c d ");
    assert_eq!(s.line_count(), 1);
    // Enter, Up and Down are left to the widget
    for k in [Key::Enter, Key::Up, Key::Down, Key::PageUp] {
        assert_eq!(s.edit_key(&key(k), 5), EditOutcome::Unhandled, "{k:?}");
    }
    assert!(!s.move_cursors(EditMotion::Up, false));
}

#[test]
fn max_lines_drops_the_newlines_past_it() {
    let mut s = EditState::multi_line().max_lines(3).with_text("a\nb\nc\nd");
    assert_eq!(s.text(), "a\nb\nc");
    s.set_cursor(p(0, 1));
    assert!(s.paste("1\n2\n3"));
    assert_eq!(s.line_count(), 3);
    assert_eq!(s.text(), "a123\nb\nc");
    assert!(!s.insert("\n"));
}

#[test]
fn control_characters_are_dropped_but_tab_kept() {
    let mut s = EditState::multi_line();
    assert!(s.paste("a\u{7}b\tc\r\nd"));
    assert_eq!(s.text(), "ab\tc\nd");
    assert!(!s.insert("\u{1b}"));
}

#[test]
fn read_only_text_moves_and_selects_but_does_not_change() {
    let mut s = EditState::multi_line().with_text("abc").read_only(true);
    assert!(!s.insert("x") && !s.paste("x") && !s.backspace() && !s.delete());
    assert!(!s.delete_lines() && !s.duplicate_lines() && !s.delete_selection());
    assert!(s.move_cursors(EditMotion::End, true));
    assert_eq!(s.selected_text(), "abc");
    assert_eq!(s.text(), "abc");
    s.set_read_only(false);
    assert!(s.backspace());
    s.set_read_only(true);
    assert!(!s.undo());
}

// ---------------------------------------------------------------------------
// Keys
// ---------------------------------------------------------------------------

#[test]
fn the_standard_keys() {
    let mut s = EditState::multi_line().with_text("foo bar");
    let ctrl = |k| KeyEvent {
        ctrl: true,
        ..key(k)
    };
    let shift = |k| KeyEvent {
        shift: true,
        ..key(k)
    };
    assert_eq!(s.edit_key(&ctrl(Key::End), 5), EditOutcome::Moved);
    assert_eq!(s.cursor(), p(0, 7));
    assert_eq!(s.edit_key(&ctrl(Key::Backspace), 5), EditOutcome::Edited);
    assert_eq!(s.text(), "foo ");
    assert_eq!(s.edit_key(&shift(Key::Home), 5), EditOutcome::Moved);
    assert_eq!(s.selected_text(), "foo ");
    assert_eq!(s.edit_key(&key(Key::Char('x')), 5), EditOutcome::Edited);
    assert_eq!(s.text(), "x");
    // AltGr (Ctrl+Alt) types; Ctrl or Alt alone with a letter is not ours
    let altgr = KeyEvent {
        ctrl: true,
        alt: true,
        ..key(Key::Char('@'))
    };
    assert_eq!(s.edit_key(&altgr, 5), EditOutcome::Edited);
    assert_eq!(s.edit_key(&ctrl(Key::Char('c')), 5), EditOutcome::Unhandled);
    let alt = KeyEvent {
        alt: true,
        ..key(Key::Char('f'))
    };
    assert_eq!(s.edit_key(&alt, 5), EditOutcome::Unhandled);
    for k in [Key::Tab, Key::Escape, Key::F(1)] {
        assert_eq!(s.edit_key(&key(k), 5), EditOutcome::Unhandled, "{k:?}");
    }
    // Undo, and redo by Ctrl+Y and Ctrl+Shift+Z
    assert_eq!(s.edit_key(&ctrl(Key::Char('z')), 5), EditOutcome::Edited);
    let undone = s.text();
    let redo_shift = KeyEvent {
        shift: true,
        ..ctrl(Key::Char('z'))
    };
    assert_eq!(s.edit_key(&redo_shift, 5), EditOutcome::Edited);
    assert_ne!(s.text(), undone);
    assert_eq!(s.edit_key(&ctrl(Key::Char('z')), 5), EditOutcome::Edited);
    assert_eq!(s.edit_key(&ctrl(Key::Char('y')), 5), EditOutcome::Edited);
    // An editing key with nothing to do is still the field's
    s.set_cursor(p(0, 0));
    assert_eq!(s.edit_key(&key(Key::Backspace), 5), EditOutcome::Moved);
    assert_eq!(s.edit_key(&ctrl(Key::Char('a')), 5), EditOutcome::Moved);
    // PageDown moves the rows given
    let mut s = at("0\n1\n2\n3\n4\n5", p(0, 0));
    s.edit_key(&key(Key::PageDown), 3);
    assert_eq!(s.cursor(), p(3, 0));
    s.edit_key(&key(Key::Enter), 3);
    assert_eq!(s.line_count(), 7);
}
