//! Proptests against plain models: the screen any written bytes leave, the
//! command line after any keys, and the lines shown after any scrolling.
//!
//! Left out of the generators, each with its issue: wide characters
//! (#974), and output written while scrolled up (#973).

use proptest::prelude::*;
use revue::event::{Key, KeyEvent};
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::{Terminal, TerminalAction};

use super::{row, screen};

// ---------------------------------------------------------------------------
// Writing
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum Piece {
    Text(String),
    Newline,
    Return,
    Tab,
    /// A control character that takes no cell
    Control(char),
    /// `ESC[` params `m`
    Sgr(Vec<u16>),
    /// An SGR in its colon form, and the same codes with semicolons
    ColonSgr(&'static str, Vec<u16>),
    /// An escape sequence that leaves the screen and the colors alone
    Sequence(&'static str),
}

impl Piece {
    fn bytes(&self) -> String {
        match self {
            Piece::Text(s) => s.clone(),
            Piece::Newline => "\n".into(),
            Piece::Return => "\r".into(),
            Piece::Tab => "\t".into(),
            Piece::Control(c) => c.to_string(),
            Piece::Sgr(p) => format!(
                "\x1b[{}m",
                p.iter().map(u16::to_string).collect::<Vec<_>>().join(";")
            ),
            Piece::ColonSgr(raw, _) => raw.to_string(),
            Piece::Sequence(raw) => raw.to_string(),
        }
    }
}

/// Sequences a program writes that draw nothing here: private modes, a
/// cursor shape, cursor moves and keys, charset selection, saving the
/// cursor, titles and links (ended by BEL or by ST, with a backslash in the
/// payload, or empty), and device control strings
const SEQUENCES: [&str; 14] = [
    "\x1b[?25l",
    "\x1b[?2004h",
    "\x1b[2 q",
    "\x1b[3~",
    "\x1b[5A",
    "\x1b[>4;1m",
    "\x1b(B",
    "\x1b7",
    "\x1b]0;C:\\dir\x07",
    "\x1b]2;title\x1b\\",
    "\x1b]8;;http://x.example/a\\b\x1b\\",
    "\x1b]\x07",
    "\x1bP+q544e\x1b\\",
    "\x1b_payload\x1b\\",
];

/// Colon-form SGRs and the codes they mean
fn colon_sgr() -> impl Strategy<Value = Piece> {
    prop::sample::select(vec![
        ("\x1b[38:2:255:0:0m", vec![38u16, 2, 255, 0, 0]),
        ("\x1b[38:2::0:128:255m", vec![38, 2, 0, 128, 255]),
        ("\x1b[48:5:200m", vec![48, 5, 200]),
        ("\x1b[48:2::300:0:0m", vec![48, 2, 300, 0, 0]),
        ("\x1b[4:3m", vec![4]),
        ("\x1b[4:0m", vec![24]),
    ])
    .prop_map(|(raw, codes)| Piece::ColonSgr(raw, codes))
}

fn sgr() -> impl Strategy<Value = Vec<u16>> {
    prop_oneof![
        4 => prop::sample::select(vec![
            0u16, 1, 2, 3, 4, 5, 7, 8, 9, 21, 22, 23, 24, 25, 27, 29, 30, 31, 32, 33, 34, 35, 36,
            37, 39, 40, 41, 42, 43, 44, 45, 46, 47, 49, 90, 91, 92, 93, 94, 95, 96, 97, 100, 101,
            102, 103, 104, 105, 106, 107, 53, 60,
        ])
        .prop_map(|c| vec![c]),
        // The modifiers on and off again, often enough that an "off" code
        // meets the modifier it should clear
        3 => prop::sample::select(vec![1u16, 2, 3, 4, 7, 9, 22, 23, 24, 27, 29]).prop_map(|c| vec![c]),
        1 => Just(vec![]),
        // Indexes and components past 255 set nothing
        1 => (prop::sample::select(vec![38u16, 48]), 0u16..=300).prop_map(|(c, n)| vec![c, 5, n]),
        1 => (prop::sample::select(vec![38u16, 48]), 0u16..=300, 0u16..=255, 0u16..=300)
            .prop_map(|(c, r, g, b)| vec![c, 2, r, g, b]),
        1 => prop::collection::vec(prop::sample::select(vec![1u16, 3, 31, 44, 0, 22, 97, 37]), 2..4),
    ]
}

fn piece() -> impl Strategy<Value = Piece> {
    prop_oneof![
        6 => "[a-z0-9 .#]{1,9}".prop_map(Piece::Text),
        2 => Just(Piece::Newline),
        1 => Just(Piece::Return),
        2 => Just(Piece::Tab),
        1 => prop::sample::select(vec!['\x00', '\x07', '\x08', '\x7f']).prop_map(Piece::Control),
        3 => sgr().prop_map(Piece::Sgr),
        1 => colon_sgr(),
        2 => prop::sample::select(SEQUENCES.to_vec()).prop_map(Piece::Sequence),
    ]
}

/// The colors and modifiers codes set: a color of `None` is the
/// terminal's default one
#[derive(Clone, Copy, Debug, PartialEq)]
struct Pen {
    fg: Option<Color>,
    bg: Option<Color>,
    modifiers: Modifier,
}

/// The xterm palette as revue draws it: 16 named colors, a 6x6x6 cube, then
/// 24 grays; none past 255
fn palette(n: u16) -> Option<Color> {
    const SIXTEEN: [(u8, u8, u8); 16] = [
        (0, 0, 0),
        (255, 0, 0),
        (0, 255, 0),
        (255, 255, 0),
        (0, 0, 255),
        (255, 0, 255),
        (0, 255, 255),
        (255, 255, 255),
        (128, 128, 128),
        (255, 85, 85),
        (85, 255, 85),
        (255, 255, 85),
        (85, 85, 255),
        (255, 85, 255),
        (85, 255, 255),
        (255, 255, 255),
    ];
    const LEVELS: [u8; 6] = [0, 95, 135, 175, 215, 255];
    Some(match n {
        0..=15 => {
            let (r, g, b) = SIXTEEN[n as usize];
            Color::rgb(r, g, b)
        }
        16..=231 => {
            let i = (n - 16) as usize;
            Color::rgb(LEVELS[i / 36], LEVELS[i / 6 % 6], LEVELS[i % 6])
        }
        232..=255 => {
            let v = 8 + 10 * (n - 232) as u8;
            Color::rgb(v, v, v)
        }
        _ => return None,
    })
}

/// An RGB color; none with a component past 255
fn rgb(r: u16, g: u16, b: u16) -> Option<Color> {
    Some(Color::rgb(
        u8::try_from(r).ok()?,
        u8::try_from(g).ok()?,
        u8::try_from(b).ok()?,
    ))
}

impl Pen {
    fn reset() -> Self {
        Pen {
            fg: None,
            bg: None,
            modifiers: Modifier::empty(),
        }
    }

    fn apply(&mut self, params: &[u16]) {
        if params.is_empty() {
            *self = Pen::reset();
            return;
        }
        let mut i = 0;
        while i < params.len() {
            let code = params[i];
            match code {
                0 => *self = Pen::reset(),
                1 => self.modifiers |= Modifier::BOLD,
                2 => self.modifiers |= Modifier::DIM,
                3 => self.modifiers |= Modifier::ITALIC,
                4 | 21 => self.modifiers |= Modifier::UNDERLINE,
                7 => self.modifiers |= Modifier::REVERSE,
                9 => self.modifiers |= Modifier::CROSSED_OUT,
                22 => self.modifiers.remove(Modifier::BOLD | Modifier::DIM),
                23 => self.modifiers.remove(Modifier::ITALIC),
                24 => self.modifiers.remove(Modifier::UNDERLINE),
                27 => self.modifiers.remove(Modifier::REVERSE),
                29 => self.modifiers.remove(Modifier::CROSSED_OUT),
                30..=37 => self.fg = palette(code - 30),
                40..=47 => self.bg = palette(code - 40),
                90..=97 => self.fg = palette(code - 90 + 8),
                100..=107 => self.bg = palette(code - 100 + 8),
                39 => self.fg = None,
                49 => self.bg = None,
                38 | 48 => {
                    // The parameters are taken even when the color is out
                    // of range, which sets nothing
                    let color = match params.get(i + 1) {
                        Some(5) if i + 2 < params.len() => {
                            i += 2;
                            palette(params[i])
                        }
                        Some(2) if i + 4 < params.len() => {
                            i += 4;
                            rgb(params[i - 2], params[i - 1], params[i])
                        }
                        _ => None,
                    };
                    if let Some(c) = color {
                        if code == 38 {
                            self.fg = Some(c);
                        } else {
                            self.bg = Some(c);
                        }
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}

/// The text grid the bytes should leave: lines of cells, the cursor, and
/// the scrollback limit
struct Grid {
    lines: Vec<Vec<(char, Pen)>>,
    row: usize,
    col: usize,
    width: usize,
    keep: usize,
    pen: Pen,
}

impl Grid {
    fn new(width: usize, height: usize, scrollback: usize) -> Self {
        Grid {
            lines: vec![Vec::new(); height],
            row: 0,
            col: 0,
            width,
            keep: height + scrollback,
            pen: Pen::reset(),
        }
    }

    fn trim(&mut self) {
        while self.lines.len() > self.keep {
            self.lines.remove(0);
            self.row = self.row.saturating_sub(1);
        }
    }

    fn next_row(&mut self) {
        self.col = 0;
        self.row += 1;
        while self.row >= self.lines.len() {
            self.lines.push(Vec::new());
        }
        self.trim();
    }

    fn put(&mut self, ch: char) {
        while self.row >= self.lines.len() {
            self.lines.push(Vec::new());
        }
        let blank = (' ', Pen::reset());
        let line = &mut self.lines[self.row];
        if line.len() <= self.col {
            line.resize(self.col + 1, blank);
        }
        line[self.col] = (ch, self.pen);
        self.col += 1;
        if self.col >= self.width {
            self.next_row();
        }
    }

    fn write(&mut self, piece: &Piece) {
        match piece {
            Piece::Text(s) => s.chars().for_each(|c| self.put(c)),
            Piece::Newline => self.next_row(),
            Piece::Return => self.col = 0,
            Piece::Tab => {
                let stop = ((self.col / 8 + 1) * 8).min(self.width);
                for _ in self.col..stop.max(self.col) {
                    self.put(' ');
                }
            }
            Piece::Control(_) | Piece::Sequence(_) => {}
            Piece::Sgr(p) | Piece::ColonSgr(_, p) => self.pen.apply(p),
        }
    }
}

proptest! {
    /// Any text, controls, SGR codes and other escape sequences leave the
    /// cells, colors, modifiers and cursor of a plain grid model. A color no
    /// code set, or one reset by 0, 39 or 49, is drawn in the terminal's
    /// default (configured, or white on black); a color a code named, white
    /// and black included, is drawn as named.
    #[test]
    fn written_bytes_draw_the_model_grid(
        width in 1usize..12,
        height in 1usize..5,
        scrollback in prop::sample::select(vec![0usize, 1, 3, 10_000]),
        extra in (0u16..3, 0u16..2),
        focused in any::<bool>(),
        defaults in any::<bool>(),
        pieces in prop::collection::vec(piece(), 0..40),
    ) {
        let mut term = Terminal::new(width as u16, height as u16).max_scrollback(scrollback);
        let (dfg, dbg) = if defaults {
            (Color::YELLOW, Color::rgb(20, 20, 20))
        } else {
            (Color::WHITE, Color::BLACK)
        };
        if defaults {
            term = term.default_fg(dfg).default_bg(dbg);
        }
        let mut grid = Grid::new(width, height, scrollback);
        for p in &pieces {
            term.write(&p.bytes());
            grid.write(p);
        }
        if focused {
            term.focus();
        }

        // The area's rows show the newest lines (#973): an area taller than
        // the terminal shows scrollback above its screen
        let (aw, ah) = (width as u16 + extra.0, height as u16 + extra.1);
        let buffer = screen(&term, aw, ah);
        let first = grid.lines.len().saturating_sub(ah as usize);
        for y in 0..ah as usize {
            let line = grid.lines.get(first + y);
            for x in 0..aw as usize {
                let cell = buffer.get(x as u16, y as u16).unwrap();
                let cursor = focused && x == grid.col && y + first == grid.row;
                match line.and_then(|l| l.get(x)) {
                    _ if cursor => {
                        prop_assert_eq!(cell.symbol, '█');
                        prop_assert_eq!(cell.fg, Some(dfg));
                    }
                    Some(&(ch, pen)) => {
                        prop_assert_eq!(cell.symbol, ch, "({}, {})", x, y);
                        prop_assert_eq!(cell.fg, Some(pen.fg.unwrap_or(dfg)), "({}, {})", x, y);
                        prop_assert_eq!(cell.bg, Some(pen.bg.unwrap_or(dbg)), "({}, {})", x, y);
                        prop_assert_eq!(cell.modifier, pen.modifiers, "({}, {})", x, y);
                    }
                    None => {
                        prop_assert_eq!(cell.symbol, ' ', "({}, {})", x, y);
                        prop_assert_eq!(cell.bg, Some(dbg), "({}, {})", x, y);
                    }
                }
            }
        }
    }
}

proptest! {
    /// The same bytes written in two pieces, split anywhere (inside a
    /// sequence too), draw what they draw written at once
    #[test]
    fn a_write_split_anywhere_draws_the_same(
        pieces in prop::collection::vec(piece(), 0..20),
        at in any::<prop::sample::Index>(),
    ) {
        let bytes: String = pieces.iter().map(Piece::bytes).collect();
        let chars: Vec<char> = bytes.chars().collect();
        let at = at.index(chars.len() + 1);
        let head: String = chars[..at].iter().collect();
        let tail: String = chars[at..].iter().collect();
        let mut whole = Terminal::new(12, 4);
        whole.write(&bytes);
        let mut split = Terminal::new(12, 4);
        split.write(&head);
        split.write(&tail);
        let (a, b) = (screen(&whole, 12, 4), screen(&split, 12, 4));
        for y in 0..4 {
            for x in 0..12 {
                prop_assert_eq!(a.get(x, y), b.get(x, y), "({}, {})", x, y);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The command line
// ---------------------------------------------------------------------------

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        4 => prop::sample::select(vec!['a', 'b', ' ', '한']).prop_map(Key::Char),
        2 => Just(Key::Enter),
        1 => Just(Key::Backspace),
        2 => Just(Key::Up),
        2 => Just(Key::Down),
        1 => Just(Key::Home),
        1 => Just(Key::Escape),
        1 => Just(Key::Tab),
        1 => Just(Key::Left),
    ]
}

proptest! {
    /// Any keys leave the input and the action a plain model of a line
    /// editor with history gives: typing goes in at the cursor, Backspace
    /// takes the char before it, Left and Home move it, Enter submits and
    /// keeps a non-empty line, Up/Down walk the history (past the newest
    /// is an empty line), the cursor at the end.
    #[test]
    fn keys_follow_the_command_line_model(
        history in prop::collection::vec("[ab]{1,3}", 0..4),
        keys in prop::collection::vec(key(), 0..40),
    ) {
        let mut term = Terminal::new(20, 3);
        for h in &history {
            for c in h.chars() {
                term.handle_key(KeyEvent::new(Key::Char(c)));
            }
            term.handle_key(KeyEvent::new(Key::Enter));
        }
        let mut input: Vec<char> = Vec::new();
        let mut at = 0;
        let mut kept = history.clone();
        let mut pos = kept.len();
        for k in keys {
            let action = term.handle_key(KeyEvent::new(k));
            let want = match k {
                Key::Char(c) => {
                    input.insert(at, c);
                    at += 1;
                    None
                }
                Key::Backspace => {
                    if at > 0 {
                        at -= 1;
                        input.remove(at);
                    }
                    None
                }
                Key::Left => {
                    at = at.saturating_sub(1);
                    None
                }
                Key::Enter => {
                    let line: String = std::mem::take(&mut input).into_iter().collect();
                    at = 0;
                    if !line.is_empty() {
                        kept.push(line.clone());
                        pos = kept.len();
                    }
                    Some(TerminalAction::Submit(line))
                }
                Key::Up => {
                    if pos > 0 {
                        pos -= 1;
                        input = kept[pos].chars().collect();
                        at = input.len();
                    }
                    None
                }
                Key::Down => {
                    if pos < kept.len() {
                        pos += 1;
                        input = kept.get(pos).map_or(Vec::new(), |l| l.chars().collect());
                        at = input.len();
                    }
                    None
                }
                Key::Home => {
                    at = 0;
                    None
                }
                Key::Escape => Some(TerminalAction::Cancel),
                Key::Tab => Some(TerminalAction::TabComplete(input.iter().collect())),
                _ => None,
            };
            prop_assert_eq!(action, want, "{:?}", k);
            prop_assert_eq!(term.get_input(), input.iter().collect::<String>(), "{:?}", k);
        }
    }
}

// ---------------------------------------------------------------------------
// Scrolling
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum Scroll {
    Up(usize),
    Down(usize),
    Top,
    Bottom,
    PageUp,
    PageDown,
    End,
}

fn scroll() -> impl Strategy<Value = Scroll> {
    prop_oneof![
        (0usize..12).prop_map(Scroll::Up),
        (0usize..12).prop_map(Scroll::Down),
        Just(Scroll::Top),
        Just(Scroll::Bottom),
        Just(Scroll::PageUp),
        Just(Scroll::PageDown),
        Just(Scroll::End),
    ]
}

proptest! {
    /// Any scrolling shows `height` lines ending `offset` lines above the
    /// newest, with the offset kept between 0 and the oldest line, and the
    /// `↑offset` indicator exactly while scrolled.
    #[test]
    fn scrolling_shows_the_model_window(
        height in 1usize..7,
        written in 0usize..25,
        ops in prop::collection::vec(scroll(), 0..20),
    ) {
        let mut term = Terminal::new(20, height as u16);
        for i in 0..written {
            term.writeln(&format!("L{i}"));
        }
        // The written lines, the empty one after them, and the empty rows
        // below them while the output is shorter than the height
        let mut lines: Vec<String> = (0..written).map(|i| format!("L{i}")).collect();
        lines.push(String::new());
        while lines.len() < height {
            lines.push(String::new());
        }
        let max = lines.len() - height;
        let mut offset = 0usize;
        for op in ops {
            match op {
                Scroll::Up(n) => {
                    term.scroll_up(n);
                    offset = (offset + n).min(max);
                }
                Scroll::Down(n) => {
                    term.scroll_down(n);
                    offset = offset.saturating_sub(n);
                }
                Scroll::Top => {
                    term.scroll_to_top();
                    offset = max;
                }
                Scroll::Bottom => {
                    term.scroll_to_bottom();
                    offset = 0;
                }
                Scroll::PageUp => {
                    term.handle_key(KeyEvent::new(Key::PageUp));
                    offset = (offset + height / 2).min(max);
                }
                Scroll::PageDown => {
                    term.handle_key(KeyEvent::new(Key::PageDown));
                    offset = offset.saturating_sub(height / 2);
                }
                Scroll::End => {
                    term.handle_key(KeyEvent::ctrl(Key::End));
                    offset = 0;
                }
            }
            let buffer = screen(&term, 20, height as u16);
            let end = lines.len() - offset;
            for (y, want) in lines[end - height..end].iter().enumerate() {
                let got = row(&buffer, y as u16);
                let got = got.split('↑').next().unwrap().trim_end();
                prop_assert_eq!(got, want.as_str(), "row {} after {:?}", y, op);
            }
            let indicator = format!("↑{offset}");
            prop_assert_eq!(row(&buffer, 0).contains(&indicator), offset > 0, "{:?}", op);
        }
    }
}
