//! Proptests: keys, undo, the dialogs and both views against plain models

use proptest::prelude::*;
use revue::event::Key;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::{BlockType, EditorViewMode, RichTextEditor};

use super::{
    row, screen, BG, CODE_BG, CURSOR_BG, FG, HEADING_FG, PREVIEW_BG, QUOTE_FG, SELECTION_BG,
};

const LINK_FG: Color = Color::rgb(137, 180, 250);

// =========================================================================
// Keys
// =========================================================================

/// Typing keys: the shortcut characters (`#`, `-`, `*`, `>`, `1`, `.` and
/// space) come often enough to turn lines into headings, quotes and lists.
fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        4 => prop::sample::select(vec!['a', 'b', '한', ' ', '#', '-', '*', '>', '1', '.', ' ']).prop_map(Key::Char),
        1 => Just(Key::Left),
        1 => Just(Key::Right),
        1 => Just(Key::Up),
        1 => Just(Key::Down),
        1 => Just(Key::Home),
        1 => Just(Key::End),
        2 => Just(Key::Backspace),
        1 => Just(Key::Delete),
        2 => Just(Key::Enter),
        1 => Just(Key::Tab),
        1 => Just(Key::Escape),
        1 => Just(Key::PageDown),
    ]
}

/// The document as lines of chars with their block type, and the cursor
#[derive(Clone, Debug)]
struct Doc {
    blocks: Vec<(Vec<char>, BlockType)>,
    cursor: (usize, usize),
}

/// What a key did to the model
enum Step {
    Handled(bool),
    /// A case the editor gets wrong; the sequence stops there
    Known,
}

const SHORTCUTS: [(&str, BlockType); 7] = [
    ("# ", BlockType::Heading1),
    ("## ", BlockType::Heading2),
    ("### ", BlockType::Heading3),
    ("> ", BlockType::Quote),
    ("- ", BlockType::BulletList),
    ("* ", BlockType::BulletList),
    ("1. ", BlockType::NumberedList),
];

impl Doc {
    fn plain(text: &str) -> Self {
        let mut blocks: Vec<_> = text
            .lines()
            .map(|l| (l.chars().collect(), BlockType::Paragraph))
            .collect();
        if blocks.is_empty() {
            blocks.push((Vec::new(), BlockType::Paragraph));
        }
        Doc {
            blocks,
            cursor: (0, 0),
        }
    }

    fn len(&self, b: usize) -> usize {
        self.blocks[b].0.len()
    }

    fn content(&self) -> String {
        self.blocks
            .iter()
            .map(|(t, _)| t.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn insert(&mut self, ch: char) {
        let (b, c) = self.cursor;
        self.blocks[b].0.insert(c, ch);
        self.cursor.1 += 1;
    }

    fn join_next(&mut self, b: usize) {
        let (next, _) = self.blocks.remove(b + 1);
        self.blocks[b].0.extend(next);
    }

    /// The markdown shortcut after a typed space: a line starting with a
    /// marker takes the marker's type and loses the marker.
    fn shortcut(&mut self) -> Step {
        let b = self.cursor.0;
        let text: String = self.blocks[b].0.iter().collect();
        let trimmed = text.trim_start();
        if trimmed.len() != text.len() && SHORTCUTS.iter().any(|(p, _)| trimmed.starts_with(p)) {
            // A marker after leading blanks: #887
            return Step::Known;
        }
        if text.trim() == "---" || text.trim() == "***" {
            // The horizontal rule shortcut never fires: a bug reported
            // from the #930 audit
            return Step::Known;
        }
        if let Some((p, t)) = SHORTCUTS.iter().find(|(p, _)| text.starts_with(p)) {
            self.blocks[b].0.drain(..p.len());
            self.blocks[b].1 = *t;
            self.cursor.1 = self.cursor.1.saturating_sub(p.len());
        }
        Step::Handled(true)
    }

    fn key(&mut self, key: &Key) -> Step {
        let (b, c) = self.cursor;
        let n = self.blocks.len();
        match key {
            Key::Left => {
                if c > 0 {
                    self.cursor.1 -= 1;
                } else if b > 0 {
                    self.cursor = (b - 1, self.len(b - 1));
                }
            }
            Key::Right => {
                if c < self.len(b) {
                    self.cursor.1 += 1;
                } else if b + 1 < n {
                    self.cursor = (b + 1, 0);
                }
            }
            Key::Up => {
                if b > 0 {
                    self.cursor = (b - 1, c.min(self.len(b - 1)));
                }
            }
            Key::Down => {
                if b + 1 < n {
                    self.cursor = (b + 1, c.min(self.len(b + 1)));
                }
            }
            Key::Home => self.cursor.1 = 0,
            Key::End => self.cursor.1 = self.len(b),
            Key::Backspace => {
                if c > 0 {
                    self.blocks[b].0.remove(c - 1);
                    self.cursor.1 -= 1;
                } else if b > 0 {
                    let prev = self.len(b - 1);
                    self.join_next(b - 1);
                    self.cursor = (b - 1, prev);
                }
            }
            Key::Delete => {
                if c < self.len(b) {
                    self.blocks[b].0.remove(c);
                } else if b + 1 < n {
                    self.join_next(b);
                }
            }
            Key::Enter => {
                let rest = self.blocks[b].0.split_off(c);
                self.blocks.insert(b + 1, (rest, BlockType::Paragraph));
                self.cursor = (b + 1, 0);
            }
            Key::Tab => {
                for _ in 0..4 {
                    self.insert(' ');
                }
            }
            Key::Char(ch) => {
                self.insert(*ch);
                if *ch == ' ' {
                    return self.shortcut();
                }
            }
            _ => return Step::Handled(false),
        }
        Step::Handled(true)
    }
}

/// Starting documents: empty, two plain lines, and lines that already hold
/// shortcut markers (a space typed after them fires the shortcut)
fn start() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "",
        "ab\ncd",
        "# h\n- x\nlong line",
        "1.\n>\n##",
        "#",
        "-",
    ])
}

proptest! {
    /// Any keys leave the text, the cursor and the current block's type a
    /// plain model gives; keys the editor does not use return false.
    #[test]
    fn prop_rich_text_editor_keys_follow_the_model(
        text in start(),
        keys in prop::collection::vec(key(), 0..60),
    ) {
        let mut editor = RichTextEditor::new().content(text);
        let mut model = Doc::plain(text);
        for k in &keys {
            let handled = editor.handle_key(k);
            match model.key(k) {
                Step::Known => return Ok(()),
                Step::Handled(h) => prop_assert_eq!(handled, h, "{:?}", k),
            }
            prop_assert_eq!(editor.get_content(), model.content(), "after {:?}", k);
            prop_assert_eq!(editor.cursor_position(), model.cursor, "after {:?}", k);
            prop_assert_eq!(editor.block_count(), model.blocks.len());
            prop_assert_eq!(editor.current_block_type(), model.blocks[model.cursor.0].1);
        }
    }

    /// Typing, Backspace, Enter, Tab and the arrows are recorded: undoing
    /// everything gives the starting text back and redoing everything the
    /// final text. (Spaces are left out: the markdown shortcut they fire is
    /// not recorded, and Delete at a line's end merges without a record:
    /// #887.)
    #[test]
    fn prop_rich_text_editor_undo_all_then_redo_all(
        text in prop::sample::select(vec!["", "ab\ncd", "x\n\nyz"]),
        keys in prop::collection::vec(key(), 0..20),
    ) {
        let mut editor = RichTextEditor::new().content(text);
        let mut model = Doc::plain(text);
        for k in &keys {
            let (b, c) = model.cursor;
            if *k == Key::Char(' ') || (*k == Key::Delete && c == model.len(b)) {
                continue;
            }
            editor.handle_key(k);
            model.key(k);
        }
        let end = editor.get_content();
        prop_assert_eq!(&end, &model.content());
        for _ in 0..100 {
            editor.undo();
        }
        prop_assert_eq!(editor.get_content(), text);
        for _ in 0..100 {
            editor.redo();
        }
        prop_assert_eq!(editor.get_content(), end);
    }
}

// =========================================================================
// Dialogs
// =========================================================================

fn dialog_key() -> impl Strategy<Value = Key> {
    prop_oneof![
        4 => prop::sample::select(vec!['a', 'z', '/', '한', ' ']).prop_map(Key::Char),
        2 => Just(Key::Tab),
        2 => Just(Key::Backspace),
        1 => Just(Key::Left),
        1 => Just(Key::Delete),
    ]
}

proptest! {
    /// In either dialog, typing goes to the active field, Tab switches it
    /// and Backspace pops; Enter inserts the markdown at the cursor and
    /// Escape inserts nothing. The open dialog shows both fields.
    #[test]
    fn prop_rich_text_editor_dialog_follows_the_model(
        image in any::<bool>(),
        at in 0usize..6,
        keys in prop::collection::vec(dialog_key(), 0..25),
        enter in any::<bool>(),
    ) {
        let mut editor = RichTextEditor::new().content("hello\nworld");
        editor.set_cursor(0, at);
        if image {
            editor.open_image_dialog();
        } else {
            editor.open_link_dialog();
        }
        let mut fields = [String::new(), String::new()];
        let mut field = 0;
        for k in &keys {
            let handled = editor.handle_key(k);
            match k {
                Key::Char(ch) => fields[field].push(*ch),
                Key::Tab => field = 1 - field,
                Key::Backspace => {
                    fields[field].pop();
                }
                _ => {}
            }
            prop_assert_eq!(handled, matches!(k, Key::Char(_) | Key::Tab | Key::Backspace));
            prop_assert!(editor.is_dialog_open());
            prop_assert_eq!(editor.get_content(), "hello\nworld");
        }
        let buffer = screen(&editor, 60, 20);
        let labels = if image { ["Alt:  ", "Src:  "] } else { ["Text: ", "URL:  "] };
        for (i, label) in labels.iter().enumerate() {
            let line = row(&buffer, 9 + i as u16);
            // ASCII values short enough for the 30-column field
            if fields[i].is_ascii() && fields[i].len() <= 30 {
                let shown = format!("│ {label}{}", fields[i]);
                prop_assert!(line.contains(shown.trim_end()), "{:?} {:?}", line, fields);
            }
            let active = buffer.get(18, 9 + i as u16).unwrap().bg;
            if !fields[i].is_empty() && fields[i].is_ascii() {
                let want = if i == field { SELECTION_BG } else { super::DIALOG_BG };
                prop_assert_eq!(active, Some(want));
            }
        }
        let insert = if image {
            format!("![{}]({})", fields[0], fields[1])
        } else {
            format!("[{}]({})", fields[0], fields[1])
        };
        editor.handle_key(if enter { &Key::Enter } else { &Key::Escape });
        prop_assert!(!editor.is_dialog_open());
        let col = at.min(5);
        let want = if enter {
            format!("{}{}{}\nworld", &"hello"[..col], insert, &"hello"[col..])
        } else {
            "hello\nworld".to_string()
        };
        prop_assert_eq!(editor.get_content(), want);
    }
}

// =========================================================================
// The editor and the preview on the screen
// =========================================================================

/// A block: its type and its text. The text has no markdown markers of
/// its own, so it parses back to the same block; `[`, `]`, `(`, `)` and `!`
/// make links and images.
fn block() -> impl Strategy<Value = (BlockType, String)> {
    let text = prop::collection::vec(
        prop::sample::select(vec!['a', 'b', ' ', '[', ']', '(', ')', '!']),
        0..14,
    )
    .prop_map(|v| v.into_iter().collect::<String>());
    let ty = prop::sample::select(vec![
        BlockType::Paragraph,
        BlockType::Heading1,
        BlockType::Heading2,
        BlockType::Heading3,
        BlockType::Heading4,
        BlockType::Heading5,
        BlockType::Heading6,
        BlockType::Quote,
        BlockType::BulletList,
        BlockType::NumberedList,
        BlockType::CodeBlock,
        BlockType::HorizontalRule,
    ]);
    (ty, text).prop_map(|(t, s)| match t {
        BlockType::HorizontalRule => (t, String::new()),
        // A trimmed text, so the line is not read as a blank paragraph
        BlockType::Paragraph if s.trim().is_empty() => (t, "p".to_string()),
        _ => (t, s),
    })
}

fn markdown(blocks: &[(BlockType, String)]) -> String {
    blocks
        .iter()
        .map(|(t, s)| match t {
            BlockType::CodeBlock => format!("```\n{s}\n```"),
            BlockType::HorizontalRule => "---".to_string(),
            BlockType::Paragraph => s.clone(),
            _ => format!("{}{s}", t.markdown_prefix()),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Char ranges of `[text](url)` not preceded by `!`
fn links(text: &[char]) -> Vec<std::ops::Range<usize>> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < text.len() {
        if text[i] == '[' && (i == 0 || text[i - 1] != '!') {
            if let Some(close) = (i + 1..text.len()).find(|&j| text[j] == ']') {
                if text.get(close + 1) == Some(&'(') {
                    if let Some(end) = (close + 2..text.len()).find(|&j| text[j] == ')') {
                        out.push(i..end + 1);
                        i = end + 1;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    out
}

fn editor_prefix(t: BlockType) -> &'static str {
    match t {
        BlockType::Paragraph => "",
        BlockType::BulletList => "• ",
        BlockType::CodeBlock => "` ",
        BlockType::HorizontalRule => "──",
        _ => t.markdown_prefix(),
    }
}

proptest! {
    /// Each visible block is drawn on its row as its type's marker and its
    /// text, cut at the width: the marker in the heading, quote or code
    /// color, link text in the link color, the cursor and the selection as
    /// backgrounds. (The cursor is kept on a visible row: the view does not
    /// scroll down to it, #973.)
    #[test]
    fn prop_rich_text_editor_editor_view_follows_the_model(
        blocks in prop::collection::vec(block(), 1..8),
        width in 4u16..36,
        height in 2u16..10,
        cursor in (0usize..8, 0usize..16),
        anchor in prop::option::of((0usize..8, 0usize..16)),
        focused in any::<bool>(),
    ) {
        let mut editor = RichTextEditor::new().from_markdown(markdown(&blocks)).focused(focused);
        prop_assert_eq!(editor.block_count(), blocks.len());
        if let Some((b, c)) = anchor {
            editor.set_cursor(b, c);
            editor.start_selection();
        }
        // The cursor row on screen
        editor.set_cursor(cursor.0 % (height - 1) as usize, cursor.1);
        let cur = editor.cursor_position();
        let sel = anchor.map(|(b, c)| {
            let b = b.min(blocks.len() - 1);
            let a = (b, c.min(blocks[b].1.chars().count()));
            if a < cur { (a, cur) } else { (cur, a) }
        });
        let buffer = screen(&editor, width, height);
        for (i, (t, text)) in blocks.iter().enumerate().take((height - 1) as usize) {
            let y = 1 + i as u16;
            let prefix = editor_prefix(*t);
            let chars: Vec<char> = text.chars().collect();
            let line: String = prefix.chars().chain(chars.iter().copied()).take(width as usize).collect();
            prop_assert_eq!(row(&buffer, y), line.trim_end(), "row {}", y);
            let p = prefix.chars().count();
            let prefix_fg = match t {
                BlockType::Heading1 | BlockType::Heading2 | BlockType::Heading3
                | BlockType::Heading4 | BlockType::Heading5 | BlockType::Heading6 => HEADING_FG,
                BlockType::Quote => QUOTE_FG,
                BlockType::CodeBlock => CODE_BG,
                _ => FG,
            };
            for x in 0..p.min(width as usize) {
                prop_assert_eq!(buffer.get(x as u16, y).unwrap().fg, Some(prefix_fg));
            }
            let link = if *t == BlockType::CodeBlock { Vec::new() } else { links(&chars) };
            for (ci, _) in chars.iter().enumerate() {
                let x = p + ci;
                if x >= width as usize {
                    break;
                }
                let cell = buffer.get(x as u16, y).unwrap();
                let at = (i, ci);
                let bg = if focused && at == cur {
                    CURSOR_BG
                } else if sel.is_some_and(|(s, e)| s <= at && at < e) {
                    SELECTION_BG
                } else {
                    BG
                };
                let fg = if link.iter().any(|r| r.contains(&ci)) { LINK_FG } else { FG };
                prop_assert_eq!((cell.fg, cell.bg), (Some(fg), Some(bg)), "({}, {})", x, y);
                prop_assert_eq!(cell.modifier, Modifier::empty());
            }
            let end = p + chars.len();
            if focused && cur.0 == i && cur.1 >= chars.len() && end < width as usize {
                prop_assert_eq!(buffer.get(end as u16, y).unwrap().bg, Some(CURSOR_BG));
            }
        }
    }

    /// The preview draws each visible block by its type: headings 1-3 in
    /// the heading color, bold (1 also underlined); quotes after a bar,
    /// bullets after a dot, code inset on the code background, a rule
    /// across the row, other text plain with links colored.
    #[test]
    fn prop_rich_text_editor_preview_follows_the_model(
        blocks in prop::collection::vec(block(), 1..8),
        width in 4u16..36,
        height in 2u16..10,
    ) {
        let editor = RichTextEditor::new()
            .from_markdown(markdown(&blocks))
            .view_mode(EditorViewMode::Preview);
        let buffer = screen(&editor, width, height);
        let w = width as usize;
        let cut = |s: String| s.chars().take(w).collect::<String>().trim_end().to_string();
        for (i, (t, text)) in blocks.iter().enumerate().take((height - 1) as usize) {
            let y = 1 + i as u16;
            let chars: Vec<char> = text.chars().collect();
            let first = buffer.get(0, y).unwrap();
            match t {
                BlockType::Heading1 | BlockType::Heading2 | BlockType::Heading3 => {
                    prop_assert_eq!(row(&buffer, y), cut(text.clone()));
                    if !chars.is_empty() {
                        let m = if *t == BlockType::Heading1 { Modifier::BOLD | Modifier::UNDERLINE } else { Modifier::BOLD };
                        prop_assert_eq!((first.fg, first.modifier), (Some(HEADING_FG), m));
                    }
                }
                BlockType::Quote => {
                    prop_assert_eq!(row(&buffer, y), cut(format!("│ {text}")));
                    prop_assert_eq!(first.fg, Some(QUOTE_FG));
                }
                BlockType::BulletList => {
                    prop_assert_eq!(row(&buffer, y), cut(format!("• {text}")));
                }
                BlockType::CodeBlock => {
                    let inner: String = chars.iter().take(w.saturating_sub(2)).collect();
                    let shown = format!(" {inner}");
                    prop_assert_eq!(row(&buffer, y), shown.trim_end());
                    for x in 0..width {
                        prop_assert_eq!(buffer.get(x, y).unwrap().bg, Some(CODE_BG));
                    }
                }
                BlockType::HorizontalRule => {
                    prop_assert_eq!(row(&buffer, y), "─".repeat(w));
                }
                BlockType::Paragraph => {
                    prop_assert_eq!(row(&buffer, y), cut(text.clone()));
                    let link = links(&chars);
                    for (ci, _) in chars.iter().enumerate().take(w) {
                        let cell = buffer.get(ci as u16, y).unwrap();
                        let fg = if link.iter().any(|r| r.contains(&ci)) { LINK_FG } else { FG };
                        prop_assert_eq!((cell.fg, cell.bg), (Some(fg), Some(PREVIEW_BG)));
                    }
                }
                // Numbered items are numbered by block position and
                // headings 4-6 are drawn as plain text: bugs reported from
                // the #930 audit
                _ => {}
            }
        }
    }
}
