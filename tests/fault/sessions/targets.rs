//! Editing sessions read from bytes: each byte picks a step (a key with
//! modifiers, a paste, new text, a resize, a draw) and the bytes after it
//! its arguments, and every editing widget plays the same steps. Shared
//! with the `fuzz` crate (`fuzz/src/lib.rs` includes this file), so it
//! uses only `revue` and `std`, and it reads the bytes itself rather than
//! through `arbitrary`, so the fault layer can drive it with random bytes
//! too.
//!
//! Checked after every step, besides "no panic": TextArea's and
//! CodeEditor's cursors are inside their lines, and Input's is inside its
//! text.

use revue::event::{Key, KeyEvent};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{
    CodeEditor, Combobox, Input, MaskedInput, NumberInput, RenderContext, SearchBar, TextArea, View,
};

/// What the steps type: ASCII, a wide glyph, a letter with a combining
/// accent, an emoji ZWJ sequence, zero-width and control characters, and
/// the brackets and quotes the code editor closes
const PIECES: &[&str] = &[
    "a",
    " ",
    "\n",
    "\t",
    "한",
    "e\u{301}",
    "\u{301}",
    "👩\u{200D}💻",
    "\u{200B}",
    "(",
    "{",
    ":",
    "\"",
    "Z",
    "\r",
    "1",
    ".",
    "-",
];

const KEYS: &[Key] = &[
    Key::Enter,
    Key::Tab,
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
    Key::Escape,
    Key::F(3),
];

/// The bytes, read one at a time; zeros once they run out
struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn byte(&mut self) -> u8 {
        let b = self.data.get(self.at).copied().unwrap_or(0);
        self.at += 1;
        b
    }

    fn done(&self) -> bool {
        self.at >= self.data.len()
    }

    /// Up to 15 pieces
    fn text(&mut self) -> String {
        let n = self.byte() % 16;
        (0..n)
            .map(|_| PIECES[self.byte() as usize % PIECES.len()])
            .collect()
    }

    fn key(&mut self) -> KeyEvent {
        let b = self.byte() as usize;
        let key = if b < KEYS.len() * 4 {
            KEYS[b % KEYS.len()]
        } else {
            // A typed char: the first char of a piece
            let piece = PIECES[b % PIECES.len()];
            Key::Char(piece.chars().next().unwrap_or('a'))
        };
        let mods = self.byte();
        let mut event = KeyEvent::new(key);
        event.ctrl = mods & 1 != 0;
        event.alt = mods & 2 != 0;
        event.shift = mods & 4 != 0;
        event
    }
}

struct Fields {
    input: Input,
    textarea: TextArea,
    wrapped: TextArea,
    editor: CodeEditor,
    combobox: Combobox,
    masked: MaskedInput,
    number: NumberInput,
    search: SearchBar,
}

impl Fields {
    fn new() -> Self {
        Self {
            input: Input::new(),
            textarea: TextArea::new().focused(true).wrap(false).max_lines(6),
            wrapped: TextArea::new().focused(true).line_numbers(true),
            editor: CodeEditor::new(),
            combobox: Combobox::new().options(vec!["apple", "한글", "e\u{301}"]),
            masked: MaskedInput::new().focused(true).max_length(12),
            number: NumberInput::new(),
            search: SearchBar::new(),
        }
    }

    fn key(&mut self, event: &KeyEvent) {
        self.input.handle_key_event(event);
        self.textarea.handle_key_event(event);
        self.wrapped.handle_key_event(event);
        self.editor.handle_key_event(event);
        self.combobox.handle_key_event(event);
        self.masked.handle_key_event(event);
        self.number.handle_key_event(event);
        self.search.handle_key_event(event);
    }

    fn paste(&mut self, text: &str) {
        self.textarea.insert_str(text);
        self.wrapped.insert_str(text);
        self.editor.insert_str(text);
    }

    fn set_text(&mut self, text: &str) {
        self.input.set_value(text);
        self.textarea.set_content(text);
        self.wrapped.set_content(text);
        self.editor.set_content(text);
        self.masked.set_value(text);
    }

    fn draw(&self, width: u16, height: u16) {
        let views: [&dyn View; 8] = [
            &self.input,
            &self.textarea,
            &self.wrapped,
            &self.editor,
            &self.combobox,
            &self.masked,
            &self.number,
            &self.search,
        ];
        for view in views {
            let mut buffer = Buffer::new(width, height);
            let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
            view.render(&mut ctx);
        }
    }

    /// The cursors are inside the text
    fn check(&self) {
        for (name, text, (line, col)) in [
            (
                "TextArea",
                self.textarea.get_content(),
                self.textarea.cursor_position(),
            ),
            (
                "TextArea (wrapped)",
                self.wrapped.get_content(),
                self.wrapped.cursor_position(),
            ),
            (
                "CodeEditor",
                self.editor.get_content(),
                self.editor.cursor_position(),
            ),
        ] {
            let lines: Vec<&str> = text.split('\n').collect();
            assert!(
                line < lines.len() && col <= lines[line].chars().count(),
                "{name} cursor {:?} outside {text:?}",
                (line, col)
            );
        }
        assert!(
            self.input.cursor() <= self.input.text().chars().count(),
            "Input cursor {} outside {:?}",
            self.input.cursor(),
            self.input.text()
        );
    }
}

/// Play the session `data` on every editing widget
pub fn editing(data: &[u8]) {
    let mut r = Reader { data, at: 0 };
    let mut fields = Fields::new();
    let (mut width, mut height) = (20, 4);
    // A session long enough to matter, short enough for a fuzz run
    let mut steps = 0;
    while !r.done() && steps < 512 {
        steps += 1;
        match r.byte() % 12 {
            0..=6 => {
                let event = r.key();
                fields.key(&event);
            }
            7 => {
                let text = r.text();
                fields.paste(&text);
            }
            8 => {
                let text = r.text();
                fields.set_text(&text);
            }
            9 => {
                width = u16::from(r.byte() % 40);
                height = u16::from(r.byte() % 8);
            }
            _ => fields.draw(width, height),
        }
        fields.check();
    }
    fields.draw(width, height);
}
