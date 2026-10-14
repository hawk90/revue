//! CommandPalette proptests: keys and command edits against a plain model of
//! the query, the filtered list and the scrolled selection, and the rows on
//! the screen against the same model

use proptest::prelude::*;
use revue::event::Key;
use revue::style::Color;
use revue::widget::{Command, CommandPalette};

use super::view::{cells, render, row};

const LABELS: [&str; 12] = [
    "Save File",
    "Open File",
    "Close Tab",
    "Find",
    "Replace",
    "Format Code",
    "Go to Line",
    "Toggle Sidebar",
    "Split Editor",
    "Quit",
    "Reload Window",
    "Select All",
];

/// Commands `0..n` with every third pinned and every fourth recent, so the
/// sort by score has ties and reorders.
fn commands(n: usize) -> Vec<Command> {
    (0..n)
        .map(|i| {
            let mut c = Command::new(format!("c{i}"), LABELS[i % LABELS.len()]);
            if i % 3 == 2 {
                c = c.pinned();
            }
            if i % 4 == 3 {
                c = c.recent();
            }
            if i % 5 == 1 {
                c = c.description("from the disk");
            }
            c
        })
        .collect()
}

/// The palette's filter: the commands that match, best score first, ties in
/// command order.
fn filtered(commands: &[Command], query: &str) -> Vec<usize> {
    let mut v: Vec<usize> = (0..commands.len())
        .filter(|&i| commands[i].matches(query))
        .collect();
    v.sort_by_key(|&i| std::cmp::Reverse(commands[i].match_score(query)));
    v
}

#[derive(Debug, Clone)]
struct Model {
    commands: Vec<Command>,
    visible: bool,
    query: String,
    filtered: Vec<usize>,
    index: usize,
    offset: usize,
    max_visible: usize,
}

impl Model {
    fn new(commands: Vec<Command>, max_visible: usize) -> Self {
        let filtered = filtered(&commands, "");
        Self {
            commands,
            visible: false,
            query: String::new(),
            filtered,
            index: 0,
            offset: 0,
            max_visible,
        }
    }

    /// Any change to the query or the commands refilters and goes back to
    /// the first row.
    fn refilter(&mut self) {
        self.filtered = filtered(&self.commands, &self.query);
        self.index = 0;
        self.offset = 0;
    }

    /// Keep the selection inside the `max_visible` rows from `offset`.
    fn scroll(&mut self) {
        let len = self.filtered.len();
        if self.max_visible >= len {
            self.offset = 0;
            return;
        }
        if self.index >= self.offset + self.max_visible {
            self.offset = self.index + 1 - self.max_visible;
        }
        if self.index < self.offset {
            self.offset = self.index;
        }
    }

    fn next(&mut self) {
        let len = self.filtered.len();
        if len > 0 {
            self.index = (self.index + 1) % len;
            self.scroll();
        }
    }

    fn prev(&mut self) {
        let len = self.filtered.len();
        if len > 0 {
            self.index = if self.index == 0 {
                len - 1
            } else {
                self.index - 1
            };
            self.scroll();
        }
    }

    fn selected_id(&self) -> Option<String> {
        self.filtered
            .get(self.index)
            .map(|&i| self.commands[i].id.clone())
    }

    fn show(&mut self) {
        self.visible = true;
        self.query.clear();
        self.refilter();
    }

    fn key(&mut self, key: &Key) -> bool {
        if !self.visible {
            return false;
        }
        match key {
            Key::Escape => self.visible = false,
            Key::Enter => {
                if self.selected_id().is_some() {
                    self.visible = false;
                }
            }
            Key::Up => self.prev(),
            Key::Down => self.next(),
            Key::Backspace => {
                self.query.pop();
                self.refilter();
            }
            Key::Char(c) => {
                self.query.push(*c);
                self.refilter();
            }
            _ => return false,
        }
        true
    }
}

#[derive(Debug, Clone)]
enum Op {
    Key(Key),
    Show,
    Hide,
    Add(usize),
    Remove(usize),
    MarkRecent(usize),
    Execute,
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        4 => prop::sample::select(vec![Key::Up, Key::Down]),
        3 => prop::sample::select(vec!['e', 'f', 'i', 'l', 'o', 'a', 's', 'x', 'j', 'k', ' '])
            .prop_map(Key::Char),
        1 => Just(Key::Backspace),
        1 => prop::sample::select(vec![Key::Enter, Key::Escape, Key::Tab, Key::PageDown, Key::Home]),
    ]
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        12 => key().prop_map(Op::Key),
        2 => Just(Op::Show),
        1 => Just(Op::Hide),
        1 => (0usize..30).prop_map(Op::Add),
        1 => (0usize..30).prop_map(Op::Remove),
        1 => (0usize..30).prop_map(Op::MarkRecent),
        1 => Just(Op::Execute),
    ]
}

proptest! {
    /// Keys, show/hide, execute and command edits against the model. Starts
    /// shown or hidden with up to 25 commands and 3..8 rows, so the selection
    /// scrolls and wraps.
    #[test]
    fn keys_and_edits_follow_the_model(
        n in 0usize..25,
        max_visible in 3u16..8,
        start_shown in any::<bool>(),
        ops in prop::collection::vec(op(), 0..60),
    ) {
        let mut p = CommandPalette::new().max_visible(max_visible).commands(commands(n));
        let mut m = Model::new(commands(n), max_visible as usize);
        if start_shown {
            p.show();
            m.show();
        }
        for op in &ops {
            match op {
                Op::Key(k) => {
                    // j and k move the selection instead of being typed
                    // while the query is empty (#889); not modeled.
                    if m.visible && m.query.is_empty() && matches!(k, Key::Char('j' | 'k')) {
                        continue;
                    }
                    let got = p.handle_key(k);
                    prop_assert_eq!(got, m.key(k), "{:?}", k);
                }
                Op::Show => { p.show(); m.show(); }
                Op::Hide => { p.hide(); m.visible = false; }
                Op::Add(i) => {
                    let c = Command::new(format!("a{i}"), LABELS[i % LABELS.len()]);
                    p.add_command(c.clone());
                    m.commands.push(c);
                    m.refilter();
                }
                Op::Remove(i) => {
                    let id = format!("c{i}");
                    p.remove_command(&id);
                    m.commands.retain(|c| c.id != id);
                    m.refilter();
                }
                Op::MarkRecent(i) => {
                    // Marking does not refilter: the order changes on the
                    // next query edit.
                    let id = format!("c{i}");
                    p.mark_recent(&id);
                    if let Some(c) = m.commands.iter_mut().find(|c| c.id == id) {
                        c.recent = true;
                    }
                }
                Op::Execute => {
                    let want = m.selected_id();
                    if want.is_some() {
                        m.visible = false;
                    }
                    prop_assert_eq!(p.execute(), want);
                }
            }
            prop_assert_eq!(p.is_visible(), m.visible);
            prop_assert_eq!(p.get_query(), m.query.as_str());
            prop_assert_eq!(&p.filtered, &m.filtered);
            prop_assert_eq!(p.selected_id().map(str::to_string), m.selected_id());
            prop_assert_eq!(p.selection.offset(), m.offset, "after {:?}", op);
        }
    }

    /// The rows on the screen are the filtered commands from the scroll
    /// offset, the selected one on the selection background, and the bottom
    /// border counts them.
    #[test]
    fn rows_show_the_filtered_commands_from_the_offset(
        n in 0usize..25,
        max_visible in 3u16..10,
        query in prop::sample::select(vec!["", "e", "fi", "o", "save", "zz", "Re"]),
        steps in 0usize..30,
        back in 0usize..10,
        titled in any::<bool>(),
        width in 30u16..72,
    ) {
        let mut p = CommandPalette::new().max_visible(max_visible).width(width).commands(commands(n));
        if titled {
            p = p.title("Commands");
        }
        p.show();
        p.set_query(query);
        for _ in 0..steps {
            p.select_next();
        }
        // Going back moves the selection off the last row, where the
        // offset rows above it can be checked.
        for _ in 0..back {
            p.select_prev();
        }
        let mut m = Model::new(commands(n), max_visible as usize);
        m.show();
        m.query = query.to_string();
        m.refilter();
        for _ in 0..steps {
            m.next();
        }
        for _ in 0..back {
            m.prev();
        }

        let b = render(&p, 80, 30);
        let x = (80 - width) / 2;
        let first_row = 2 + 3 + u16::from(titled);
        let shown = m.filtered.len().min(max_visible as usize);
        // The last row is left out: the bottom border is drawn over it
        // (draft-cmd-1).
        for r in 0..shown.saturating_sub(1) {
            let y = first_row + r as u16;
            let cmd = &m.commands[m.filtered[m.offset + r]];
            let label_cols = (width - 15).saturating_sub(4) as usize;
            let want: String = cmd.label.chars().take(label_cols).collect();
            let text = row(&b, y);
            let got: String = text.chars().skip(x as usize + 4).take(want.chars().count()).collect();
            prop_assert_eq!(&got, &want, "row {}: {}", r, text);
            let bg = b.get(x + 1, y).unwrap().bg;
            let want_bg = if m.offset + r == m.index {
                Color::rgb(50, 80, 120)
            } else {
                revue::widget::theme::EDITOR_BG
            };
            prop_assert_eq!(bg, Some(want_bg), "row {}", r);
        }
        let bottom = first_row + shown as u16 - 1;
        let bottom = if shown == 0 { first_row - 1 } else { bottom };
        let text = row(&b, bottom);
        prop_assert_eq!(text.chars().nth(x as usize), Some('╰'), "{}", text);
        let count = format!("{}/{}", m.filtered.len(), n);
        prop_assert!(text.contains(&count), "{} in {}", count, text);
    }

    /// The builders give the same palette in any order.
    #[test]
    fn builder_order_does_not_matter(
        n in 0usize..15,
        max_visible in 0u16..8,
        width in 0u16..70,
        order in Just(vec![0usize, 1, 2, 3]).prop_shuffle(),
    ) {
        let mut p = CommandPalette::new();
        for step in &order {
            p = match step {
                0 => p.commands(commands(n)),
                1 => p.max_visible(max_visible),
                2 => p.width(width),
                _ => p.title("T"),
            };
        }
        let mut q = CommandPalette::new()
            .commands(commands(n))
            .max_visible(max_visible)
            .width(width)
            .title("T");
        p.show();
        q.show();
        for _ in 0..n {
            p.select_next();
            q.select_next();
            prop_assert_eq!(p.selection.offset(), q.selection.offset());
        }
        prop_assert_eq!(cells(&render(&p, 80, 20)), cells(&render(&q, 80, 20)));
    }
}
