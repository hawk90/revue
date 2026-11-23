//! Menu proptests: keys against a plain model of the selection, read from
//! the screen and the callbacks, and where the boxes are drawn

use std::cell::RefCell;
use std::rc::Rc;

use proptest::prelude::*;
use revue::event::Key;
use revue::render::Buffer;
use revue::utils::display_width;
use revue::widget::{ContextMenu, Menu, MenuBar, MenuItem};

use super::render;

const SELECTED_BG: revue::style::Color = revue::style::Color::rgb(60, 100, 180);

#[derive(Clone, Copy, Debug, PartialEq)]
enum Kind {
    Plain,
    Disabled,
    Separator,
}

fn kind() -> impl Strategy<Value = Kind> {
    prop_oneof![
        3 => Just(Kind::Plain),
        1 => Just(Kind::Disabled),
        1 => Just(Kind::Separator),
    ]
}

/// Menus of 0..6 items of any kind, leading separators included (opening a
/// menu selects its first item that is not one, #1050)
fn menus() -> impl Strategy<Value = Vec<Vec<Kind>>> {
    prop::collection::vec(prop::collection::vec(kind(), 0..6), 0..4)
}

fn key() -> impl Strategy<Value = Key> {
    prop::sample::select(vec![
        Key::Left,
        Key::Right,
        Key::Up,
        Key::Down,
        Key::Char('h'),
        Key::Char('l'),
        Key::Char('j'),
        Key::Char('k'),
        Key::Enter,
        Key::Char(' '),
        Key::Escape,
        Key::Tab,
        Key::Char('x'),
    ])
}

type Log = Rc<RefCell<Vec<(usize, usize)>>>;

fn item(kind: Kind, m: usize, i: usize, log: &Log) -> MenuItem {
    let log = log.clone();
    match kind {
        Kind::Separator => MenuItem::separator(),
        Kind::Plain => {
            MenuItem::new(format!("i{i}")).on_select(move || log.borrow_mut().push((m, i)))
        }
        Kind::Disabled => MenuItem::new(format!("d{i}"))
            .disabled(true)
            .on_select(move || log.borrow_mut().push((m, i))),
    }
}

/// What the bar should hold after each key
#[derive(Debug, Default)]
struct BarModel {
    menus: Vec<Vec<Kind>>,
    menu: usize,
    item: Option<usize>,
    open: bool,
}

impl BarModel {
    fn open_menu(&mut self, i: usize) {
        if i < self.menus.len() {
            self.menu = i;
            self.open = true;
            self.item = self.menus[i].iter().position(|k| *k != Kind::Separator);
        }
    }

    fn close(&mut self) {
        self.open = false;
        self.item = None;
    }

    /// Step through the items by `step` (+1 or len-1), skipping separators
    fn step(&mut self, back: bool) {
        let Some(items) = self.menus.get(self.menu) else {
            return;
        };
        let n = items.len();
        // Nothing to select: only separators, or no items
        let Some(cur) = self.item else {
            return;
        };
        let mut next = cur;
        loop {
            next = if back {
                (next + n - 1) % n
            } else {
                (next + 1) % n
            };
            if items[next] != Kind::Separator || next == cur {
                break;
            }
        }
        self.item = Some(next);
    }

    fn move_menu(&mut self, back: bool) {
        let n = self.menus.len();
        if n > 0 {
            self.menu = if back {
                (self.menu + n - 1) % n
            } else {
                (self.menu + 1) % n
            };
            if self.open {
                self.open_menu(self.menu);
            }
        }
    }

    /// The key's return value and the item it chose, if any
    fn key(&mut self, key: &Key) -> (bool, Option<(usize, usize)>) {
        match key {
            Key::Left | Key::Char('h') => {
                self.move_menu(true);
                (true, None)
            }
            Key::Right | Key::Char('l') => {
                self.move_menu(false);
                (true, None)
            }
            Key::Up | Key::Char('k') if self.open => {
                self.step(true);
                (true, None)
            }
            Key::Down | Key::Char('j') if self.open => {
                self.step(false);
                (true, None)
            }
            Key::Enter | Key::Char(' ') if self.open => match self.item {
                Some(i) if self.menus[self.menu][i] == Kind::Plain => {
                    let chosen = (self.menu, i);
                    self.close();
                    (true, Some(chosen))
                }
                _ => (false, None),
            },
            Key::Enter | Key::Char(' ') => {
                self.open_menu(self.menu);
                (true, None)
            }
            Key::Escape if self.open => {
                self.close();
                (true, None)
            }
            _ => (false, None),
        }
    }
}

/// The x of each title in the bar
fn title_xs(titles: &[String]) -> Vec<u16> {
    let mut x = 0u16;
    titles
        .iter()
        .map(|t| {
            let at = x;
            x += display_width(t) as u16 + 2;
            at
        })
        .collect()
}

/// The highlighted title (by its first column) and dropdown row
fn highlighted(
    b: &Buffer,
    xs: &[u16],
    dropdown_x: u16,
    rows: usize,
) -> (Option<usize>, Option<usize>) {
    let title = xs
        .iter()
        .position(|&x| b.get(x, 0).unwrap().bg == Some(SELECTED_BG));
    let item =
        (0..rows).find(|&i| b.get(dropdown_x + 1, 2 + i as u16).unwrap().bg == Some(SELECTED_BG));
    (title, item)
}

proptest! {
    /// Every key does what the model does: the return value, open or
    /// closed, the callback that runs, and the highlighted title and row.
    #[test]
    fn prop_menu_bar_keys_match_model(
        menus in menus(),
        keys in prop::collection::vec(key(), 0..40),
    ) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let titles: Vec<String> = (0..menus.len()).map(|m| format!("M{m}")).collect();
        let mut bar = MenuBar::new();
        for (m, kinds) in menus.iter().enumerate() {
            let items = kinds.iter().enumerate().map(|(i, &k)| item(k, m, i, &log)).collect();
            bar = bar.menu(Menu::new(titles[m].clone()).items(items));
        }
        let xs = title_xs(&titles);
        let mut model = BarModel { menus: menus.clone(), ..BarModel::default() };
        for key in &keys {
            log.borrow_mut().clear();
            let (handled, chosen) = model.key(key);
            prop_assert_eq!(bar.handle_key(key), handled, "{:?} {:?}", key, model);
            prop_assert_eq!(bar.is_open(), model.open);
            prop_assert_eq!(log.borrow().first().copied(), chosen);
            let b = render(&bar, 40, 9);
            let dx = xs.get(model.menu).copied().unwrap_or(0);
            let (title, row) = highlighted(&b, &xs, dx, 6);
            prop_assert_eq!(title, (!menus.is_empty()).then_some(model.menu));
            // A separator row is never highlighted: the model never selects one
            let expect = if model.open { model.item } else { None };
            prop_assert_eq!(row, expect, "{:?}", model);
        }
    }

    /// The context menu: Up and Down skip separators and stop at the ends,
    /// Enter runs a plain item and closes, and on a disabled item or a
    /// separator returns false and stays open, as MenuBar's does (#1051);
    /// Escape closes, and a hidden menu takes no keys.
    #[test]
    fn prop_context_menu_keys_match_model(
        kinds in prop::collection::vec(kind(), 0..6),
        keys in prop::collection::vec(key(), 0..40),
        at in (0u16..30, 0u16..12),
    ) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let items = kinds.iter().enumerate().map(|(i, &k)| item(k, 0, i, &log));
        let mut m = ContextMenu::new().items(items.collect());
        m.show(at.0, at.1);
        let first = kinds.iter().position(|k| *k != Kind::Separator).unwrap_or(0);
        let (mut selected, mut visible) = (first, true);
        let n = kinds.len();
        // The next item that is not a separator from `from` in `dir`, or
        // `from` when there is none that way
        let step = |from: usize, down: bool| -> usize {
            let mut i = from;
            loop {
                i = match (down, i) {
                    (true, i) if i + 1 < n => i + 1,
                    (false, i) if i > 0 => i - 1,
                    _ => return from,
                };
                if kinds[i] != Kind::Separator {
                    return i;
                }
            }
        };
        for key in &keys {
            log.borrow_mut().clear();
            let (handled, chosen) = if !visible {
                (false, None)
            } else {
                match key {
                    Key::Up | Key::Char('k') => {
                        selected = step(selected, false);
                        (true, None)
                    }
                    Key::Down | Key::Char('j') => {
                        selected = step(selected, true);
                        (true, None)
                    }
                    Key::Enter | Key::Char(' ') => match kinds.get(selected) {
                        Some(Kind::Plain) => {
                            visible = false;
                            (true, Some((0, selected)))
                        }
                        _ => (false, None),
                    },
                    Key::Escape => {
                        visible = false;
                        (true, None)
                    }
                    _ => (false, None),
                }
            };
            prop_assert_eq!(m.handle_key(key), handled, "{:?}", key);
            prop_assert_eq!(m.is_visible(), visible);
            prop_assert_eq!(log.borrow().first().copied(), chosen);
            // The highlighted row, read from the column right of the frame
            let b = render(&m, 30, 12);
            let rows: Vec<u16> = (0..12)
                .filter(|&y| (0..30).any(|x| b.get(x, y).unwrap().bg == Some(SELECTED_BG)))
                .collect();
            // A separator row is never highlighted: only separators leaves
            // nothing highlighted
            if visible && kinds.get(selected).is_some_and(|k| *k != Kind::Separator) {
                let top = at.1.min(12 - (n as u16 + 2));
                prop_assert_eq!(rows, vec![top + 1 + selected as u16]);
            } else {
                prop_assert!(rows.is_empty());
            }
            if !visible {
                m.show(at.0, at.1);
                selected = first;
                visible = true;
            }
        }
    }

    /// The open menu's dropdown: under its title, moved left to fit the area
    /// (#1050), as wide as its widest item (or the area) and as tall as its
    /// items (cut at the bottom), with a full frame and nothing drawn outside
    /// it below the bar, shortcuts included.
    #[test]
    fn prop_menu_bar_dropdown_box(
        titles in prop::collection::vec(prop::sample::select(vec!["F", "Edit", "파일", "", "📁 x"]), 1..4),
        labels in prop::collection::vec(
            (prop::sample::select(vec!["a", "Open", "설정", "A longer label", ""]),
             prop::option::of(prop::sample::select(vec!["F1", "Ctrl+Z", "단축"]))),
            1..8,
        ),
        pick in any::<prop::sample::Index>(),
        w in 0u16..40,
        h in 0u16..12,
    ) {
        let open = pick.index(titles.len());
        let title_x = titles[..open].iter().map(|t| display_width(t) as u16 + 2).sum::<u16>();
        let width_of = |labels: &[(&str, Option<&str>)]| {
            labels
                .iter()
                .map(|(l, s)| display_width(l) + s.map_or(0, |s| display_width(s) + 2) + 4)
                .max()
                .unwrap() as u16
        };
        let need = width_of(&labels);
        let mut bar = MenuBar::new();
        for (m, t) in titles.iter().enumerate() {
            let mut menu = Menu::new(*t);
            if m == open {
                for (label, shortcut) in &labels {
                    let mut it = MenuItem::new(*label);
                    if let Some(s) = shortcut {
                        it = it.shortcut(*s);
                    }
                    menu = menu.item(it);
                }
            }
            bar = bar.menu(menu);
        }
        bar.open_menu(open);
        let b = render(&bar, w, h);
        let sym = |x: u16, y: u16| b.get(x, y).unwrap().symbol;
        if h < 2 || title_x >= w {
            // No room: nothing below the bar
            for y in 1..h {
                for x in 0..w {
                    prop_assert_eq!(sym(x, y), ' ');
                }
            }
            return Ok(());
        }
        let bw = need.min(w);
        let x0 = title_x.min(w - bw);
        let bh = (labels.len() as u16 + 2).min(h - 1);
        for y in 1..h {
            for x in 0..w {
                let inside = x >= x0 && x < x0 + bw && y < 1 + bh;
                if !inside {
                    prop_assert_eq!(sym(x, y), ' ', "outside at ({}, {})", x, y);
                }
            }
        }
        prop_assert_eq!(sym(x0, 1), '┌');
        if bw > 1 {
            prop_assert_eq!(sym(x0 + bw - 1, 1), '┐');
        }
        if bh > 1 {
            prop_assert_eq!(sym(x0, bh), '└');
            for y in 2..bh {
                prop_assert_eq!(sym(x0, y), '│');
                if bw > 1 {
                    prop_assert_eq!(sym(x0 + bw - 1, y), '│');
                }
            }
        }
        // When the box is not cut, each label and shortcut is drawn whole
        if bw == need && bh == labels.len() as u16 + 2 {
            for (i, (label, shortcut)) in labels.iter().enumerate() {
                let y = 2 + i as u16;
                let mut text = String::new();
                for x in x0 + 2..x0 + 2 + display_width(label) as u16 {
                    let c = b.get(x, y).unwrap();
                    if !c.is_continuation() {
                        text.push(c.symbol);
                    }
                }
                prop_assert_eq!(&text, label);
                if let Some(s) = shortcut {
                    let end = x0 + bw - 2;
                    let mut text = String::new();
                    for x in end - display_width(s) as u16..end {
                        let c = b.get(x, y).unwrap();
                        if !c.is_continuation() {
                            text.push(c.symbol);
                        }
                    }
                    prop_assert_eq!(&text, s);
                }
            }
        }
    }

    /// The context menu's box: moved left and up to fit, no wider than the
    /// area and no taller (the items scroll, #973), with a full frame and
    /// nothing outside it; nothing in an area with no row inside a frame.
    #[test]
    fn prop_context_menu_box(
        labels in prop::collection::vec(prop::sample::select(vec!["a", "Open", "설정", "A longer label", ""]), 1..8),
        at in (0u16..60, 0u16..20),
        w in 0u16..30,
        h in 0u16..12,
    ) {
        let mut m = ContextMenu::new().items(labels.iter().map(|l| MenuItem::new(*l)).collect());
        m.show(at.0, at.1);
        let b = render(&m, w, h);
        let bw = (labels.iter().map(|l| display_width(l)).max().unwrap() as u16 + 4).min(w);
        let bh = (labels.len() as u16 + 2).min(h);
        if bh < 3 {
            prop_assert!(b.cells().iter().all(|c| c.symbol == ' '));
            return Ok(());
        }
        let x0 = at.0.min(w.saturating_sub(bw));
        let y0 = at.1.min(h.saturating_sub(bh));
        let sym = |x: u16, y: u16| b.get(x, y).unwrap().symbol;
        for y in 0..h {
            for x in 0..w {
                let inside = x >= x0 && x < x0 + bw && y >= y0 && y < y0 + bh;
                if !inside {
                    prop_assert_eq!(sym(x, y), ' ', "outside at ({}, {})", x, y);
                } else if (x, y) == (x0, y0) {
                    prop_assert_eq!(sym(x, y), '┌');
                } else if bw > 1 && (x, y) == (x0 + bw - 1, y0 + bh - 1) {
                    prop_assert_eq!(sym(x, y), '┘');
                } else if x == x0 && y > y0 && y < y0 + bh - 1 {
                    prop_assert_eq!(sym(x, y), '│');
                } else if y == y0 && x > x0 && x < x0 + bw - 1 {
                    prop_assert_eq!(sym(x, y), '─');
                }
            }
        }
    }
}
