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
/// auto-closed brackets.
#[derive(Debug, Clone)]
struct Model {
    lines: Vec<Vec<char>>,
    cursor: (usize, usize),
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
        }
    }

    fn text(&self) -> String {
        self.lines
            .iter()
            .map(|l| l.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Up and Down keep the screen column: the char covering it on the
    /// other line, or the end
    fn move_to_line(&mut self, line: usize) {
        let (l, c) = self.cursor;
        let col: usize = self.lines[l][..c].iter().map(|&ch| width(ch)).sum();
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
            Key::Up if l > 0 => self.move_to_line(l - 1),
            Key::Down if l + 1 < self.lines.len() => self.move_to_line(l + 1),
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
    /// gives the end text back. Without auto-indent and auto-closed brackets,
    /// which record too little to undo (#892).
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
