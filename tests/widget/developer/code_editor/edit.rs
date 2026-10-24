//! CodeEditor editing against plain models: keys on an empty editor, a
//! proptest of the text and cursor after any keys, undo and redo of any
//! edits, bracket matching both ways, and find matches

use proptest::prelude::*;
use revue::event::Key;
use revue::widget::CodeEditor;

/// Columns a char takes on screen: the Hangul used here takes two
fn width(ch: char) -> usize {
    if ('가'..='힣').contains(&ch) {
        2
    } else {
        1
    }
}

/// The text as lines of chars and the cursor as (line, char index): what
/// the editor's keys should do, written plainly. No auto-indent and no
/// auto-closed brackets. Up and Down aim for the screen column they
/// started from (`goal`), kept across short lines.
#[derive(Debug, Clone)]
struct Model {
    lines: Vec<Vec<char>>,
    cursor: (usize, usize),
    goal: Option<usize>,
}

impl Model {
    fn new(text: &str) -> Self {
        let mut lines: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
        if lines.is_empty() {
            lines.push(Vec::new());
        }
        Self {
            lines,
            cursor: (0, 0),
            goal: None,
        }
    }

    fn text(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Up and Down keep the screen column: the char covering the goal
    /// column on the other line, or the end
    fn move_to_line(&mut self, line: usize, col: usize) {
        let mut start = 0;
        let mut idx = self.lines[line].len();
        for (i, &ch) in self.lines[line].iter().enumerate() {
            if col < start + width(ch) {
                idx = i;
                break;
            }
            start += width(ch);
        }
        self.cursor = (line, idx);
    }

    fn press(&mut self, key: &Key) {
        let (l, c) = self.cursor;
        let len = self.lines[l].len();
        if matches!(key, Key::Up | Key::Down) {
            let col = self.lines[l][..c].iter().map(|&ch| width(ch)).sum();
            let goal = *self.goal.get_or_insert(col);
            match key {
                // On the first line Up goes to the start, on the last line
                // Down to the end
                Key::Up if l == 0 => self.cursor.1 = 0,
                Key::Up => self.move_to_line(l - 1, goal),
                _ if l + 1 == self.lines.len() => self.cursor.1 = len,
                _ => self.move_to_line(l + 1, goal),
            }
            return;
        }
        // A move drops the goal column; so does an edit, unless it had
        // nothing to do (Backspace at the start), which leaves the cursor
        // as it was
        let before = self.lines.clone();
        let edit = matches!(
            key,
            Key::Char(_) | Key::Enter | Key::Backspace | Key::Delete
        );
        self.edit_or_move(key, (l, c), len);
        if !edit || self.lines != before {
            self.goal = None;
        }
    }

    fn edit_or_move(&mut self, key: &Key, (l, c): (usize, usize), len: usize) {
        match key {
            Key::Char(ch) => {
                self.lines[l].insert(c, *ch);
                self.cursor.1 += 1;
            }
            Key::Enter => {
                let rest = self.lines[l].split_off(c);
                self.lines.insert(l + 1, rest);
                self.cursor = (l + 1, 0);
            }
            Key::Backspace if c > 0 => {
                self.lines[l].remove(c - 1);
                self.cursor.1 -= 1;
            }
            Key::Backspace if l > 0 => {
                let line = self.lines.remove(l);
                let prev = self.lines[l - 1].len();
                self.lines[l - 1].extend(line);
                self.cursor = (l - 1, prev);
            }
            Key::Delete if c < len => {
                self.lines[l].remove(c);
            }
            Key::Delete if l + 1 < self.lines.len() => {
                let next = self.lines.remove(l + 1);
                self.lines[l].extend(next);
            }
            Key::Left if c > 0 => self.cursor.1 -= 1,
            Key::Left if l > 0 => self.cursor = (l - 1, self.lines[l - 1].len()),
            Key::Right if c < len => self.cursor.1 += 1,
            Key::Right if l + 1 < self.lines.len() => self.cursor = (l + 1, 0),
            Key::Home => {
                // First non-blank, then column 0, then back
                let first = self.lines[l]
                    .iter()
                    .position(|ch| !ch.is_whitespace())
                    .unwrap_or(0);
                self.cursor.1 = if c == 0 {
                    first
                } else if c == first {
                    0
                } else {
                    first
                };
            }
            Key::End => self.cursor.1 = len,
            _ => {}
        }
    }
}

fn plain(text: &str) -> CodeEditor {
    CodeEditor::new()
        .content(text)
        .auto_indent(false)
        .bracket_matching(false)
}

#[test]
fn test_code_editor_keys_on_an_empty_editor() {
    for key in [
        Key::Backspace,
        Key::Delete,
        Key::Left,
        Key::Right,
        Key::Up,
        Key::Down,
        Key::Home,
        Key::End,
        Key::PageUp,
        Key::PageDown,
    ] {
        let mut editor = CodeEditor::new();
        assert!(editor.handle_key(&key), "{key:?}");
        assert_eq!(
            (editor.get_content(), editor.cursor_position()),
            (String::new(), (0, 0)),
            "{key:?}"
        );
        assert_eq!(editor.line_count(), 1, "{key:?}");
    }
    let mut editor = CodeEditor::new();
    editor.undo();
    editor.redo();
    editor.delete_line();
    editor.move_word_left();
    editor.move_word_right();
    editor.find_next();
    editor.find_previous();
    editor.goto_line(5);
    editor.set_find_query("x");
    assert_eq!(
        (editor.get_content(), editor.cursor_position()),
        (String::new(), (0, 0))
    );
    assert_eq!(
        (editor.find_match_count(), editor.current_find_index()),
        (0, 0)
    );
    assert!(editor.find_matching_bracket().is_none());
    assert_eq!(editor.get_selection(), None);
    editor.select_all();
    assert_eq!(editor.get_selection(), Some(String::new()));
    editor.duplicate_line();
    assert_eq!(
        (editor.get_content(), editor.line_count()),
        ("\n".to_string(), 2)
    );
}

#[test]
fn test_code_editor_enter_indents_after_an_opener() {
    // Each of `{ [ ( :` at the end of a line adds a level; anything else
    // keeps the line's own indent
    for (line, indent) in [
        ("  if x {", "      "),
        ("  v = [", "      "),
        ("  f(", "      "),
        ("  def f():", "      "),
        ("  x = 1", "  "),
    ] {
        let mut editor = CodeEditor::new().content(line).bracket_matching(false);
        editor.move_end();
        editor.handle_key(&Key::Enter);
        assert_eq!(editor.get_line(1).as_deref(), Some(indent), "{line:?}");
        assert_eq!(editor.cursor_position(), (1, indent.len()), "{line:?}");
    }
}

#[test]
fn test_code_editor_indent_size_is_at_least_one() {
    let mut editor = CodeEditor::new().indent_size(0);
    editor.handle_key(&Key::Tab);
    assert_eq!(editor.get_content(), " ");
    let mut editor = CodeEditor::new().indent_size(2);
    editor.handle_key(&Key::Tab);
    assert_eq!(editor.get_content(), "  ");
}

#[test]
fn test_code_editor_undo_keeps_the_last_100_edits() {
    let mut editor = CodeEditor::new()
        .bracket_matching(false)
        .clock(super::stepping_clock());
    for _ in 0..105 {
        editor.insert_char('a');
    }
    for _ in 0..200 {
        editor.undo();
    }
    // The first five edits fell off the history
    assert_eq!(editor.get_content(), "aaaaa");
    for _ in 0..200 {
        editor.redo();
    }
    assert_eq!(editor.get_content(), "a".repeat(105));
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Char('a')),
        Just(Key::Char(' ')),
        Just(Key::Char('한')),
        Just(Key::Enter),
        Just(Key::Backspace),
        Just(Key::Delete),
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Up),
        Just(Key::Down),
        Just(Key::Home),
        Just(Key::End),
    ]
}

/// Start texts with leading blanks (smart Home), wide glyphs (Up and Down
/// keep the column), empty lines and an empty text
fn start() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just(""),
        Just("  ab\n한글x\n\n a 한"),
        Just("x\n  \nab한cd"),
    ]
}

proptest! {
    #[test]
    fn prop_code_editor_keys_follow_the_model(text in start(), keys in prop::collection::vec(key(), 0..60)) {
        let mut editor = plain(text);
        let mut model = Model::new(text);
        for k in &keys {
            prop_assert!(editor.handle_key(k));
            model.press(k);
            prop_assert_eq!(editor.get_content(), model.text(), "after {:?}", k);
            prop_assert_eq!(editor.cursor_position(), model.cursor, "after {:?}", k);
            prop_assert_eq!(editor.line_count(), model.lines.len());
        }
    }

    /// Undoing every edit restores the start text, and redoing them all
    /// gives the end text back.
    #[test]
    fn prop_code_editor_undo_all_then_redo_all(text in start(), keys in prop::collection::vec(key(), 0..60)) {
        let mut editor = plain(text);
        for k in &keys {
            editor.handle_key(k);
        }
        let end = editor.get_content();
        for _ in 0..keys.len() {
            editor.undo();
        }
        prop_assert_eq!(editor.get_content(), text);
        for _ in 0..keys.len() {
            editor.redo();
        }
        prop_assert_eq!(editor.get_content(), end);
    }

    /// The same with auto-indent and auto-closed brackets on, and keys that
    /// use them: what each adds undoes with the edit (#892)
    #[test]
    fn prop_code_editor_undo_all_then_redo_all_with_auto_edits(
        text in start(),
        keys in prop::collection::vec(prop_oneof![
            key(),
            prop::sample::select(vec![Key::Char('('), Key::Char('{'), Key::Char(':'), Key::Char('"'), Key::Tab]),
        ], 0..60),
    ) {
        let mut editor = CodeEditor::new().content(text);
        for k in &keys {
            editor.handle_key(k);
        }
        let end = editor.get_content();
        for _ in 0..keys.len() {
            editor.undo();
        }
        prop_assert_eq!(editor.get_content(), text);
        for _ in 0..keys.len() {
            editor.redo();
        }
        prop_assert_eq!(editor.get_content(), end);
    }

    /// A bracket's match matches it back
    #[test]
    fn prop_code_editor_bracket_match_is_mutual(text in "[()\\[\\]{}a\n]{0,24}") {
        let editor = CodeEditor::new().content(text.as_str());
        let mut probe = editor.clone();
        for (l, line) in text.lines().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                probe.set_cursor(l, c);
                let Some(m) = probe.find_matching_bracket() else { continue };
                let pair = [('(', ')'), ('[', ']'), ('{', '}')];
                prop_assert!(pair.iter().any(|&(o, cl)| (ch, m.char) == (o, cl) || (ch, m.char) == (cl, o)), "{} -> {}", ch, m.char);
                let mut back = editor.clone();
                back.set_cursor(m.position.0, m.position.1);
                prop_assert_eq!(back.find_matching_bracket().map(|b| b.position), Some((l, c)));
            }
        }
    }

    /// Find reports every place the query starts, overlapping and ignoring
    /// case, in reading order
    #[test]
    fn prop_code_editor_find_matches_the_model(text in "[aAb한\n]{0,30}", query in prop_oneof![Just("a"), Just("A"), Just("aa"), Just("ab"), Just("한a")]) {
        let mut editor = CodeEditor::new().content(text.as_str());
        editor.set_find_query(query);
        let q: Vec<char> = query.to_lowercase().chars().collect();
        let mut expected = Vec::new();
        for (l, line) in text.lines().enumerate() {
            let chars: Vec<char> = line.to_lowercase().chars().collect();
            for s in 0..chars.len() {
                if chars[s..].starts_with(&q) {
                    expected.push((l, s));
                }
            }
        }
        prop_assert_eq!(editor.find_match_count(), expected.len());
        // Each char reports the first match covering it
        for (l, line) in text.lines().enumerate() {
            for c in 0..line.chars().count() {
                let covering = expected.iter().position(|&(ml, ms)| ml == l && ms <= c && c < ms + q.len());
                prop_assert_eq!(editor.get_find_match_at(l, c), covering.map(|i| (i == 0, i)));
            }
        }
        // find_next walks them in order and wraps
        for &(l, s) in expected.iter().skip(1).chain(expected.first()) {
            editor.find_next();
            prop_assert_eq!(editor.cursor_position(), (l, s));
        }
    }
}

/// Enter with auto-indent recorded only the split, so undo joined the
/// indented line back with its indent (#892)
#[test]
fn test_code_editor_undo_takes_the_auto_indent_with_the_new_line() {
    let mut editor = CodeEditor::new().content("  ab");
    editor.set_cursor(0, 3);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.get_content(), "  a\n  b");
    editor.undo();
    assert_eq!(editor.get_content(), "  ab");
    assert_eq!(editor.cursor_position(), (0, 3));
}

/// The auto-closed bracket was not recorded, so undo left it (#892)
#[test]
fn test_code_editor_undo_takes_the_closed_bracket() {
    let mut editor = CodeEditor::new();
    editor.handle_key(&Key::Char('('));
    assert_eq!(editor.get_content(), "()");
    assert_eq!(editor.cursor_position(), (0, 1));
    editor.undo();
    assert_eq!(editor.get_content(), "");
    editor.redo();
    assert_eq!(editor.get_content(), "()");
    assert_eq!(editor.cursor_position(), (0, 1));
}

/// insert_str, the line commands and a multi-line delete_selection pushed
/// no undo step (#892)
#[test]
fn test_code_editor_undoes_pastes_line_commands_and_deleted_selections() {
    let mut editor = CodeEditor::new()
        .clock(super::stepping_clock())
        .content("one\ntwo\nthree");
    editor.insert_str("x\ny");
    editor.duplicate_line();
    editor.delete_line();
    editor.set_cursor(0, 1);
    editor.start_selection();
    editor.set_cursor(2, 2);
    editor.delete_selection();
    for _ in 0..4 {
        editor.undo();
    }
    assert_eq!(editor.get_content(), "one\ntwo\nthree");
}

/// delete_selection edited a read-only editor (#1080)
#[test]
fn test_code_editor_read_only_delete_selection_keeps_text() {
    let mut editor = CodeEditor::new().content("keep me").read_only(true);
    editor.select_all();
    editor.delete_selection();
    assert_eq!(editor.get_content(), "keep me");
    assert!(!editor.can_undo());
}

/// `config` let indent_size be 0, and Tab then typed nothing (#1080)
#[test]
fn test_code_editor_config_indent_size_zero_still_indents() {
    let config = revue::widget::code_editor::EditorConfig {
        indent_size: 0,
        ..Default::default()
    };
    let mut editor = CodeEditor::new().config(config);
    editor.handle_key(&Key::Tab);
    assert_eq!(editor.get_content(), " ");
}

/// Keys with modifiers: Shift selects, Ctrl+Z undoes, Ctrl and an unbound
/// letter is left to the app
#[test]
fn test_code_editor_takes_the_modified_editing_keys() {
    use revue::event::KeyEvent;
    let shift = |key| {
        let mut e = KeyEvent::new(key);
        e.shift = true;
        e
    };
    let mut editor = CodeEditor::new().content("foo.bar baz");
    assert!(editor.handle_key_event(&shift(Key::End)));
    assert_eq!(editor.get_selection().as_deref(), Some("foo.bar baz"));
    assert!(editor.handle_key_event(&KeyEvent::new(Key::Left)));
    assert_eq!(editor.cursor_position(), (0, 0));
    assert!(editor.handle_key_event(&KeyEvent::ctrl(Key::Right)));
    assert_eq!(editor.cursor_position(), (0, 3));
    assert!(editor.handle_key_event(&KeyEvent::ctrl(Key::Backspace)));
    assert_eq!(editor.get_content(), ".bar baz");
    assert!(editor.handle_key_event(&KeyEvent::ctrl(Key::Char('z'))));
    assert_eq!(editor.get_content(), "foo.bar baz");
    assert!(!editor.handle_key_event(&KeyEvent::ctrl(Key::Char('q'))));
    assert_eq!(editor.get_content(), "foo.bar baz");
}

/// Typing a bracket over the same closed pair left the cursor after it
#[test]
fn test_code_editor_bracket_over_the_same_pair_lands_between() {
    let mut editor = CodeEditor::new().content("()");
    editor.select_all();
    editor.handle_key(&Key::Char('('));
    assert_eq!(editor.get_content(), "()");
    assert_eq!(editor.cursor_position(), (0, 1));
}

/// Home compared the cursor with the first non-blank char's index, which
/// for a combining mark on a blank is inside a character, so it never went
/// to the line's start
#[test]
fn test_code_editor_home_reaches_the_start_past_a_combining_mark() {
    let mut editor = CodeEditor::new().content(" \u{301}x");
    editor.set_cursor(0, 3);
    editor.handle_key(&Key::Home);
    assert_eq!(editor.cursor_position(), (0, 2));
    editor.handle_key(&Key::Home);
    assert_eq!(editor.cursor_position(), (0, 0));
}

/// Enter's indent came from the selection's moving end, not from where the
/// new line goes in, and Enter inside the indent doubled it
#[test]
fn test_code_editor_enter_indents_from_where_the_line_breaks() {
    use revue::event::KeyEvent;
    let mut shift_end = KeyEvent::new(Key::End);
    shift_end.shift = true;

    let mut editor = CodeEditor::new().content("if x {abc");
    editor.set_cursor(0, 6);
    editor.handle_key_event(&shift_end);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.get_content(), "if x {\n    ");

    let mut editor = CodeEditor::new().content("    a\nb");
    editor.set_cursor(1, 1);
    editor.start_selection();
    editor.set_cursor(0, 5);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.get_content(), "    a\n    ");

    let mut editor = CodeEditor::new().content("    foo");
    editor.set_cursor(0, 2);
    editor.handle_key(&Key::Enter);
    assert_eq!(editor.get_content(), "  \n    foo");
}

/// An edit that changed nothing (a read-only editor) ended selection mode
#[test]
fn test_code_editor_selection_mode_survives_an_edit_that_changes_nothing() {
    let mut editor = CodeEditor::new().content("ab").read_only(true);
    editor.start_selection();
    editor.insert_char('x');
    assert!(editor.has_selection());
    assert_eq!(editor.get_content(), "ab");
}
