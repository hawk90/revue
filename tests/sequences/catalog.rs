//! One entry per stateful widget: how to build it with ordinary content, how
//! a key (and, where it has one, a mouse event) reaches it, the data
//! mutations a sequence may interleave, and the cheap invariant checked after
//! every step.
//!
//! Keys reach a widget through its own public handler (`handle_key`,
//! `handle_key_event` or `Interactive::handle_key`). A few list-like widgets
//! have no key handler, only navigation methods; for those the entry maps
//! keys to the methods the way an app's handler would, and says so.
//!
//! Not here: `Link` (Enter and a click open the system browser), and widgets
//! without any input handling.

use revue::event::{Key, KeyEvent, MouseEvent, MouseEventKind};
use revue::layout::Rect;
use revue::widget::traits::Interactive;
use revue::widget::*;

/// What a sequence drives: a widget behind its handlers.
pub trait Subject {
    fn key(&mut self, ev: &KeyEvent, area: Rect);
    fn mouse(&mut self, ev: &MouseEvent, area: Rect);
    fn has_mouse(&self) -> bool;
    fn mutations(&self) -> Vec<&'static str>;
    fn mutate(&mut self, i: usize);
    /// The widget to render, if it is a view.
    fn view(&self) -> Option<&dyn View>;
    fn check(&self, area: Rect) -> Result<(), String>;
}

type KeyFn<T> = fn(&mut T, &KeyEvent, Rect);
type MouseFn<T> = fn(&mut T, &MouseEvent, Rect);
type CheckFn<T> = fn(&T, Rect) -> Result<(), String>;
type Mutation<T> = (&'static str, fn(&mut T));

/// A widget with its handlers, mutations and invariant.
pub struct W<T> {
    w: T,
    key: KeyFn<T>,
    mouse: Option<MouseFn<T>>,
    mutations: Vec<Mutation<T>>,
    check: CheckFn<T>,
    view: fn(&T) -> Option<&dyn View>,
}

fn as_view<T: View>(w: &T) -> Option<&dyn View> {
    Some(w)
}

impl<T: View + 'static> W<T> {
    fn new(w: T, key: KeyFn<T>) -> Self {
        Self {
            w,
            key,
            mouse: None,
            mutations: Vec::new(),
            check: |_, _| Ok(()),
            view: as_view::<T>,
        }
    }
}

impl<T: 'static> W<T> {
    /// A subject that is not a view (nothing to render).
    fn headless(w: T, key: KeyFn<T>) -> Self {
        Self {
            w,
            key,
            mouse: None,
            mutations: Vec::new(),
            check: |_, _| Ok(()),
            view: |_| None,
        }
    }

    fn mouse(mut self, f: MouseFn<T>) -> Self {
        self.mouse = Some(f);
        self
    }

    fn mutate(mut self, name: &'static str, f: fn(&mut T)) -> Self {
        self.mutations.push((name, f));
        self
    }

    fn check(mut self, f: CheckFn<T>) -> Self {
        self.check = f;
        self
    }

    fn done(self) -> Box<dyn Subject> {
        Box::new(self)
    }
}

impl<T> Subject for W<T> {
    fn key(&mut self, ev: &KeyEvent, area: Rect) {
        (self.key)(&mut self.w, ev, area);
    }
    fn mouse(&mut self, ev: &MouseEvent, area: Rect) {
        if let Some(f) = self.mouse {
            f(&mut self.w, ev, area);
        }
    }
    fn has_mouse(&self) -> bool {
        self.mouse.is_some()
    }
    fn mutations(&self) -> Vec<&'static str> {
        self.mutations.iter().map(|(n, _)| *n).collect()
    }
    fn mutate(&mut self, i: usize) {
        (self.mutations[i].1)(&mut self.w);
    }
    fn view(&self) -> Option<&dyn View> {
        (self.view)(&self.w)
    }
    fn check(&self, area: Rect) -> Result<(), String> {
        (self.check)(&self.w, area)
    }
}

/// A catalog entry.
pub struct Entry {
    pub name: &'static str,
    pub build: fn() -> Box<dyn Subject>,
}

fn e(name: &'static str, build: fn() -> Box<dyn Subject>) -> Entry {
    Entry { name, build }
}

// ─── Invariant helpers ──────────────────────────────────────────────────────

/// `index` points at an item, or is 0 when there are none.
fn index_in(what: &str, index: usize, len: usize) -> Result<(), String> {
    if index < len || (len == 0 && index == 0) {
        Ok(())
    } else {
        Err(format!("{what} {index} with {len} item(s)"))
    }
}

/// `index`, if any, points at an item.
fn opt_index_in(what: &str, index: Option<usize>, len: usize) -> Result<(), String> {
    match index {
        Some(i) if i >= len => Err(format!("{what} Some({i}) with {len} item(s)")),
        _ => Ok(()),
    }
}

/// A character-based cursor lies within the text (at most one past the end).
fn char_cursor(what: &str, cursor: usize, text: &str) -> Result<(), String> {
    let n = text.chars().count();
    if cursor <= n {
        Ok(())
    } else {
        Err(format!(
            "{what} {cursor} past the end of {text:?} ({n} chars)"
        ))
    }
}

/// A `(line, col)` cursor lies on a line, at most one past its last char.
fn line_cursor(what: &str, (line, col): (usize, usize), text: &str) -> Result<(), String> {
    let lines: Vec<&str> = text.split('\n').collect();
    match lines.get(line) {
        None => Err(format!("{what} line {line} with {} line(s)", lines.len())),
        Some(l) => char_cursor(&format!("{what} col (line {line})"), col, l),
    }
}

fn all(results: impl IntoIterator<Item = Result<(), String>>) -> Result<(), String> {
    results.into_iter().collect()
}

// ─── Key routing helpers ────────────────────────────────────────────────────

/// A key the way a list-like app handler maps it to navigation.
enum Nav {
    Next,
    Prev,
    First,
    Last,
    PageDown,
    PageUp,
    Activate,
    Other,
}

fn nav(ev: &KeyEvent) -> Nav {
    match ev.key {
        Key::Down | Key::Char('j') => Nav::Next,
        Key::Up | Key::Char('k') => Nav::Prev,
        Key::Home | Key::Char('g') => Nav::First,
        Key::End | Key::Char('G') => Nav::Last,
        Key::PageDown => Nav::PageDown,
        Key::PageUp => Nav::PageUp,
        Key::Enter | Key::Char(' ') => Nav::Activate,
        _ => Nav::Other,
    }
}

fn items(n: usize) -> Vec<String> {
    (0..n).map(|i| format!("item {i}")).collect()
}

const JSON: &str = r#"{"name":"revue","tags":["tui","rust"],"nested":{"a":1,"b":[true,null,{"c":"한글"}]},"empty":{}}"#;

const LOG: &str = "2026-10-07 12:00:00 INFO start\n2026-10-07 12:00:01 WARN slow 😀\n2026-10-07 12:00:02 ERROR failed\nplain line\n2026-10-07 12:00:03 DEBUG 한글 detail";

const CODE: &str = "fn main() {\n    let 한 = \"😀\";\n\tprintln!(\"{한}\");\n}\n";

fn tree_nodes() -> Vec<TreeNode> {
    vec![
        TreeNode::new("root")
            .expanded(true)
            .child(TreeNode::new("a").child(TreeNode::new("a1")))
            .child(TreeNode::new("b")),
        TreeNode::new("leaf 한글"),
    ]
}

fn file_entries() -> Vec<FileEntry> {
    vec![
        FileEntry::directory("src", "/p/src")
            .child(FileEntry::file("main.rs", "/p/src/main.rs"))
            .child(FileEntry::directory("empty", "/p/src/empty")),
        FileEntry::file("한글.txt", "/p/한글.txt"),
    ]
}

fn grid() -> DataGrid {
    DataGrid::new()
        .column(GridColumn::new("name", "Name").editable(true))
        .column(GridColumn::new("n", "N").sortable(true))
        .column(GridColumn::new("x", "X"))
        .data(
            (0..12)
                .map(|i| vec![format!("row {i}"), format!("{}", (i * 7) % 5), "x".into()])
                .collect(),
        )
}

fn commands() -> Vec<Command> {
    vec![
        Command::new("open", "Open file").shortcut("Ctrl+O"),
        Command::new("save", "Save").category("File"),
        Command::new("quit", "Quit 한글"),
    ]
}

fn menus() -> MenuBar {
    MenuBar::new()
        .menu(
            Menu::new("File")
                .item(MenuItem::new("Open"))
                .separator()
                .item(MenuItem::new("Recent").submenu(vec![MenuItem::new("a"), MenuItem::new("b")]))
                .item(MenuItem::new("Gone").disabled(true)),
        )
        .menu(Menu::new("Empty"))
        .menu(Menu::new("Edit").item(MenuItem::new("Undo")))
}

fn date() -> Date {
    Date::new(2026, 10, 7)
}

fn date_ok(what: &str, d: Date) -> Result<(), String> {
    if d.is_valid() {
        Ok(())
    } else {
        Err(format!("{what} {d:?} is not a valid date"))
    }
}

// ─── The catalog ────────────────────────────────────────────────────────────

pub fn catalog() -> Vec<Entry> {
    vec![
        // ── Lists and tables ────────────────────────────────────────────────
        e("List", || {
            W::new(List::new(items(5)), |w, k, _| {
                w.handle_key(&k.key);
            })
            .check(|w, _| index_in("selected", w.selected_index(), w.len()))
            .done()
        }),
        e("List(empty)", || {
            W::new(List::<String>::new(vec![]), |w, k, _| {
                w.handle_key(&k.key);
            })
            .check(|w, _| index_in("selected", w.selected_index(), w.len()))
            .done()
        }),
        // Table has no key handler: keys map to its navigation methods.
        e("Table", || {
            W::new(
                Table::new(vec![Column::new("A"), Column::new("B")]).rows(
                    (0..30)
                        .map(|i| vec![format!("r{i}"), "한글".into()])
                        .collect(),
                ),
                |w, k, area| match nav(k) {
                    Nav::Next => w.select_next(),
                    Nav::Prev => w.select_prev(),
                    Nav::First => w.select_first(),
                    Nav::Last => w.select_last(),
                    Nav::PageDown => w.page_down(area.height as usize),
                    Nav::PageUp => w.page_up(area.height as usize),
                    Nav::Activate => w.jump_to(17),
                    Nav::Other => {}
                },
            )
            .check(|w, _| index_in("selected", w.selected_index(), w.row_count()))
            .done()
        }),
        e("DataGrid", || {
            W::new(grid(), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mouse(|w, m, area| {
                w.handle_mouse(m.kind, m.x, m.y, area);
            })
            .mutate("filter 'row 1'", |w| w.set_filter("row 1"))
            .mutate("filter none-match", |w| w.set_filter("zzz"))
            .mutate("clear filter", |w| w.set_filter(""))
            .mutate("sort col 1", |w| w.sort(1))
            .mutate("drop rows", |w| {
                w.rows.truncate(2);
                w.recompute_cache();
            })
            .check(|w, _| {
                let n = w.row_count();
                all([
                    index_in("selected_row", w.selected_row, n),
                    index_in("selected_col", w.selected_col, w.columns.len()),
                    if w.scroll_row <= w.selected_row.max(n.saturating_sub(1)) {
                        Ok(())
                    } else {
                        Err(format!(
                            "scroll_row {} past selection {} ({n} rows)",
                            w.scroll_row, w.selected_row
                        ))
                    },
                    if w.filtered_indices().iter().all(|&i| i < w.rows.len()) {
                        Ok(())
                    } else {
                        Err("filtered index past the rows".into())
                    },
                ])
            })
            .done()
        }),
        e("Tree", || {
            W::new(
                Tree::new().nodes(tree_nodes()).searchable(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("query 'a'", |w| w.set_query("a"))
            .mutate("query no match", |w| w.set_query("zzz"))
            .mutate("clear query", |w| w.clear_query())
            .check(|w, _| {
                all([
                    index_in("selected", w.selected_index(), w.visible_count()),
                    // `current_match_index` is 1-based (0 when there are none).
                    if (w.match_count() == 0 && w.current_match_index() == 0)
                        || (1..=w.match_count()).contains(&w.current_match_index())
                    {
                        Ok(())
                    } else {
                        Err(format!(
                            "match {} of {}",
                            w.current_match_index(),
                            w.match_count()
                        ))
                    },
                ])
            })
            .done()
        }),
        e("FileTree", || {
            W::new(FileTree::new().root(file_entries()), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("collapse all", |w| w.collapse_all())
            .mutate("expand all", |w| w.expand_all())
            .done()
        }),
        // VirtualList has no key handler: keys map to its navigation methods.
        e("VirtualList", || {
            W::new(VirtualList::new(items(40)), |w, k, area| match nav(k) {
                Nav::Next => w.select_next(),
                Nav::Prev => w.select_prev(),
                Nav::First => w.select_first(),
                Nav::Last => w.select_last(),
                Nav::PageDown => w.page_down(area.height),
                Nav::PageUp => w.page_up(area.height),
                Nav::Activate => w.jump_to(25),
                Nav::Other => w.scroll_by(-3),
            })
            .mutate("clear", |w| w.clear())
            .mutate("3 items", |w| w.set_items(items(3)))
            .mutate("remove 0", |w| {
                w.remove(0);
            })
            .mutate("push", |w| w.push("new".into()))
            .check(|w, _| opt_index_in("selected", w.selected_index(), w.len()))
            .done()
        }),
        // OptionList has no key handler: keys map to its navigation methods.
        e("OptionList", || {
            W::new(
                OptionList::new()
                    .group("G")
                    .option("one", "1")
                    .separator()
                    .add_option(OptionItem::new("off").disabled(true))
                    .option("two 한글", "2"),
                |w, k, _| match nav(k) {
                    Nav::Next => w.highlight_next(),
                    Nav::Prev => w.highlight_previous(),
                    Nav::First => w.highlight_first(),
                    Nav::Last => w.highlight_last(),
                    Nav::Activate => {
                        w.select_highlighted();
                    }
                    Nav::PageDown => w.select(9),
                    Nav::PageUp => w.clear_selection(),
                    Nav::Other => {}
                },
            )
            .check(|w, _| {
                all([
                    index_in("highlighted", w.__test_highlighted(), w.option_count()),
                    opt_index_in("selected", w.__test_selected(), w.option_count()),
                ])
            })
            .done()
        }),
        e("SelectionList", || {
            W::new(
                SelectionList::new(vec!["a", "b 한글", "c"]).focused(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .check(|w, _| {
                if w.get_selected().iter().all(|&i| i < 3) {
                    Ok(())
                } else {
                    Err(format!("selected {:?} of 3", w.get_selected()))
                }
            })
            .done()
        }),
        e("SortableList", || {
            W::new(SortableList::new(["a", "b", "c", "d"]), |w, k, _| {
                Interactive::handle_key(w, k);
            })
            .mouse(|w, m, area| {
                Interactive::handle_mouse(w, m, area);
            })
            .mutate("remove 0", |w| {
                w.remove(0);
            })
            .mutate("remove last", |w| {
                let n = w.items().len();
                if n > 0 {
                    w.remove(n - 1);
                }
            })
            .mutate("push", |w| w.push("new"))
            .check(|w, _| opt_index_in("selected", w.selected(), w.items().len()))
            .done()
        }),
        e("RichLog", || {
            let mut log = RichLog::new();
            for i in 0..20 {
                log.info(format!("line {i} 한글"));
            }
            W::new(log, |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("clear", |w| w.clear())
            .mutate("append", |w| w.error("boom"))
            .done()
        }),
        e("LogViewer", || {
            let mut v = LogViewer::new();
            v.load(LOG);
            W::new(v, |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("clear", |w| w.clear())
            .mutate("push", |w| w.push("2026-10-07 12:00:09 INFO pushed"))
            .mutate("search 'e'", |w| w.search("e"))
            .mutate("filter ERROR", |w| {
                w.set_filter(LogFilter::new().contains("ERROR"))
            })
            .mutate("clear filter", |w| w.clear_filter())
            .check(|w, _| {
                if w.search_match_count() == 0 || w.current_search_index() < w.search_match_count()
                {
                    Ok(())
                } else {
                    Err(format!(
                        "search match {} of {}",
                        w.current_search_index(),
                        w.search_match_count()
                    ))
                }
            })
            .done()
        }),
        // JsonViewer has no key handler: keys map to its navigation methods.
        e("JsonViewer", || {
            W::new(JsonViewer::from_content(JSON), |w, k, area| match k.key {
                Key::Left | Key::Char('h') => w.collapse(),
                Key::Right | Key::Char('l') => w.expand(),
                Key::Char('a') => w.expand_all(),
                Key::Char('x') => w.collapse_all(),
                _ => match nav(k) {
                    Nav::Next => w.select_down(),
                    Nav::Prev => w.select_up(),
                    Nav::First => w.select_first(),
                    Nav::Last => w.select_last(),
                    Nav::PageDown => w.page_down(area.height as usize),
                    Nav::PageUp => w.page_up(area.height as usize),
                    Nav::Activate => w.toggle(),
                    Nav::Other => {}
                },
            })
            .mutate("parse []", |w| w.parse("[]"))
            .mutate("parse scalar", |w| w.parse("1"))
            .mutate("parse invalid", |w| w.parse("{"))
            .mutate("parse sample", |w| w.parse(JSON))
            .check(|w, _| index_in("selected", w.selected_index(), w.visible_count()))
            .done()
        }),
        // CsvViewer has no key handler: keys map to its navigation methods.
        e("CsvViewer", || {
            W::new(
                CsvViewer::from_content("a,b,c\n1,한글,3\n4,5\n7,8,9,10\n"),
                |w, k, area| match k.key {
                    Key::Left => w.select_left(),
                    Key::Right => w.select_right(),
                    Key::Char('n') => w.next_match(),
                    Key::Char('p') => w.prev_match(),
                    Key::Char('1') => w.sort_by(1),
                    Key::Char('0') => w.reset_sort(),
                    _ => match nav(k) {
                        Nav::Next => w.select_down(),
                        Nav::Prev => w.select_up(),
                        Nav::First => w.select_first_row(),
                        Nav::Last => w.select_last_row(),
                        Nav::PageDown => w.page_down(area.height as usize),
                        Nav::PageUp => w.page_up(area.height as usize),
                        _ => {}
                    },
                },
            )
            .mutate("parse empty", |w| w.parse(""))
            .mutate("parse 1 row", |w| w.parse("x"))
            .mutate("search '1'", |w| w.search("1"))
            .mutate("clear search", |w| w.clear_search())
            .check(|w, _| {
                all([
                    index_in("selected_row", w.selected_row(), w.row_count()),
                    index_in("selected_col", w.selected_col(), w.column_count()),
                ])
            })
            .done()
        }),
        // ── Choice widgets ─────────────────────────────────────────────────
        e("Tabs", || {
            W::new(
                Tabs::new().tabs(vec!["One", "Two 한글", "Three"]),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .check(|w, _| index_in("selected", w.selected_index(), w.len()))
            .done()
        }),
        e("Select", || {
            W::new(
                Select::new()
                    .options(vec!["apple", "banana", "한글", "cherry"])
                    .searchable(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mouse(|w, m, area| {
                Interactive::handle_mouse(w, m, area);
            })
            .mutate("query 'an'", |w| w.set_query("an"))
            .mutate("query no match", |w| w.set_query("zzz"))
            .mutate("clear query", |w| w.clear_query())
            .check(|w, _| {
                all([
                    index_in("selected", w.selected_index(), w.len()),
                    if w.filtered_options().iter().all(|&i| i < w.len()) {
                        Ok(())
                    } else {
                        Err("filtered option past the options".into())
                    },
                ])
            })
            .done()
        }),
        e("Combobox", || {
            W::new(
                Combobox::new()
                    .options(["apple", "banana", "한글", "cherry"])
                    .allow_custom(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mouse(|w, m, area| {
                Interactive::handle_mouse(w, m, area);
            })
            .mutate("set input 'b'", |w| w.set_input("b"))
            .mutate("set input no match", |w| w.set_input("zzz"))
            .mutate("clear input", |w| w.clear_input())
            .check(|w, _| {
                if w.filtered_count() <= w.option_count() {
                    Ok(())
                } else {
                    Err(format!(
                        "{} filtered of {} options",
                        w.filtered_count(),
                        w.option_count()
                    ))
                }
            })
            .done()
        }),
        e("Combobox(multi)", || {
            W::new(
                Combobox::new()
                    .options(["apple", "banana", "한글"])
                    .multi_select(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("set input 'a'", |w| w.set_input("a"))
            .mutate("clear input", |w| w.clear_input())
            .done()
        }),
        e("Autocomplete", || {
            W::new(
                Autocomplete::new().suggestions(["apple", "apricot", "banana", "한글"]),
                |w, k, _| {
                    w.handle_key(k.clone());
                },
            )
            .mutate("set value 'ap'", |w| w.set_value("ap"))
            .mutate("no suggestions", |w| w.set_suggestions(vec![]))
            .mutate("one suggestion", |w| {
                w.set_suggestions(vec![Suggestion::new("apple")])
            })
            .mutate("focus", |w| w.focus())
            .mutate("blur", |w| w.blur())
            .done()
        }),
        e("MultiSelect", || {
            W::new(
                MultiSelect::new()
                    .options(vec!["a", "b", "한글", "d"])
                    .searchable(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("query 'a'", |w| w.set_query("a"))
            .mutate("query no match", |w| w.set_query("zzz"))
            .mutate("clear query", |w| w.clear_query())
            .mutate("select all", |w| w.select_all())
            .mutate("clear selection", |w| w.clear_selection())
            .check(|w, _| {
                all([
                    if w.get_selected_indices().iter().all(|&i| i < w.len()) {
                        Ok(())
                    } else {
                        Err("selected index past the options".into())
                    },
                    opt_index_in("tag cursor", w.get_tag_cursor(), w.selection_count()),
                    index_in(
                        "dropdown cursor",
                        w.get_dropdown_cursor(),
                        w.get_filtered().len(),
                    ),
                ])
            })
            .done()
        }),
        e("RadioGroup", || {
            W::new(RadioGroup::new(["a", "b", "c"]).focused(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .check(|w, _| index_in("selected", w.selected_index(), 3))
            .done()
        }),
        e("Checkbox", || {
            W::new(Checkbox::new("check 한글"), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("set checked", |w| w.set_checked(true))
            .done()
        }),
        e("Switch", || {
            W::new(Switch::new(), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mouse(|w, m, area| {
                Interactive::handle_mouse(w, m, area);
            })
            .done()
        }),
        e("Button", || {
            W::new(Button::new("Press 한글"), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mouse(|w, m, area| {
                w.handle_mouse(m, area);
            })
            .done()
        }),
        e("ThemePicker", || {
            W::new(ThemePicker::new(), |w, k, _| {
                Interactive::handle_key(w, k);
            })
            .done()
        }),
        e("Breadcrumb", || {
            W::new(
                Breadcrumb::new().path("home/src/한글/file.rs"),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("pop", |w| {
                w.pop();
            })
            .check(|w, _| index_in("selected", w.selected(), w.len()))
            .done()
        }),
        // Pagination has no key handler: keys map to its methods.
        e("Pagination", || {
            W::new(Pagination::new(7), |w, k, _| match nav(k) {
                Nav::Next => {
                    w.next_page();
                }
                Nav::Prev => {
                    w.prev_page();
                }
                Nav::First => w.first(),
                Nav::Last => w.last(),
                Nav::PageDown => w.goto(99),
                Nav::PageUp => w.goto(0),
                _ => {}
            })
            .mutate("total 0", |w| w.set_total(0))
            .mutate("total 1", |w| w.set_total(1))
            .mutate("total 3", |w| w.set_total(3))
            .check(|w, _| {
                let (cur, total) = (w.get_current(), w.get_total());
                if cur >= 1 && cur <= total.max(1) {
                    Ok(())
                } else {
                    Err(format!("page {cur} of {total}"))
                }
            })
            .done()
        }),
        // Stepper has no key handler: keys map to its methods.
        e("Stepper", || {
            W::new(
                Stepper::new()
                    .add_step("One")
                    .add_step("Two 한글")
                    .add_step("Three"),
                |w, k, _| match nav(k) {
                    Nav::Next => {
                        w.next_step();
                    }
                    Nav::Prev => {
                        w.prev();
                    }
                    Nav::First => w.go_to(0),
                    Nav::Last => w.go_to(9),
                    Nav::Activate => w.complete_current(),
                    Nav::PageDown => w.skip(1),
                    Nav::PageUp => w.mark_error(2),
                    Nav::Other => {}
                },
            )
            .check(|w, _| {
                let p = w.progress();
                if (0.0..=1.0).contains(&p) {
                    Ok(())
                } else {
                    Err(format!("progress {p}"))
                }
            })
            .done()
        }),
        // ── Text widgets ────────────────────────────────────────────────────
        e("Input", || {
            W::new(
                Input::new().value("héllo 한글").focused(true),
                |w, k, _| {
                    w.handle_key_event(k);
                },
            )
            .mutate("set value", |w| w.set_value("ab"))
            .mutate("clear", |w| w.clear())
            .mutate("select all", |w| w.select_all())
            .mutate("undo", |w| {
                w.undo();
            })
            .check(|w, _| {
                all([
                    char_cursor("cursor", w.cursor(), w.text()),
                    match w.selection() {
                        Some((a, b)) if a > b || b > w.text().chars().count() => {
                            Err(format!("selection {a}..{b} in {:?}", w.text()))
                        }
                        _ => Ok(()),
                    },
                ])
            })
            .done()
        }),
        e("TextArea", || {
            W::new(
                TextArea::new()
                    .content("one\n한글 two\n\nfour 😀")
                    .focused(true),
                |w, k, _| {
                    w.handle_key_event(k);
                },
            )
            .mutate("set content", |w| w.set_content("x"))
            .mutate("empty content", |w| w.set_content(""))
            .mutate("find 'o'", |w| {
                w.open_find();
                w.set_find_query("o");
            })
            .mutate("add cursor below", |w| w.add_cursor_below())
            .check(|w, _| {
                let text = w.get_content();
                w.cursor_positions()
                    .into_iter()
                    .try_for_each(|p| line_cursor("cursor", p, &text))
            })
            .done()
        }),
        e("CodeEditor", || {
            W::new(CodeEditor::new().content(CODE).focused(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("set content", |w| w.set_content("x"))
            .mutate("empty content", |w| w.set_content(""))
            .mutate("find 'n'", |w| {
                w.open_find();
                w.set_find_query("n");
            })
            .mutate("goto line", |w| w.open_goto_line())
            .check(|w, _| {
                let (line, col) = w.cursor_position();
                match w.get_line(line) {
                    None => Err(format!("cursor line {line} of {}", w.line_count())),
                    Some(l) => char_cursor(&format!("cursor col (line {line})"), col, &l),
                }
            })
            .done()
        }),
        e("RichTextEditor", || {
            W::new(
                RichTextEditor::new()
                    .content("# Title\n\n한글 body *em*\n- item")
                    .focused(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("set content", |w| w.set_content("x"))
            .mutate("empty content", |w| w.set_content(""))
            .mutate("link dialog", |w| w.open_link_dialog())
            .check(|w, _| {
                let (block, _) = w.cursor_position();
                index_in("cursor block", block, w.block_count())
            })
            .done()
        }),
        e("SearchBar", || {
            W::new(SearchBar::new(), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("set query", |w| w.set_query("a:b"))
            .mutate("clear", |w| w.clear())
            .mutate("focus", |w| w.focus())
            .done()
        }),
        e("MaskedInput", || {
            W::new(MaskedInput::password().focused(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("set value", |w| w.set_value("ab한"))
            .mutate("clear", |w| w.clear())
            .mutate("reveal", |w| w.toggle_reveal())
            .check(|w, _| char_cursor("cursor", w.get_cursor(), w.get_value()))
            .done()
        }),
        e("MaskedInput(pin)", || {
            W::new(MaskedInput::pin(4).focused(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .check(|w, _| {
                all([
                    char_cursor("cursor", w.get_cursor(), w.get_value()),
                    if w.get_value().chars().count() <= 4 {
                        Ok(())
                    } else {
                        Err(format!("{:?} longer than the pin", w.get_value()))
                    },
                ])
            })
            .done()
        }),
        e("NumberInput", || {
            W::new(
                NumberInput::new().min(-5.0).max(10.0).step(2.5).value(1.0),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("set 99", |w| w.set_value(99.0))
            .mutate("set -99", |w| w.set_value(-99.0))
            .check(|w, _| {
                let v = w.get_value();
                if (-5.0..=10.0).contains(&v) {
                    Ok(())
                } else {
                    Err(format!("value {v} outside -5..=10"))
                }
            })
            .done()
        }),
        e("Slider", || {
            W::new(
                Slider::new()
                    .range(-10.0, 10.0)
                    .step(3.0)
                    .value(0.0)
                    .focused(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("set 50", |w| w.set_value(50.0))
            .mutate("set NaN", |w| w.set_value(f64::NAN))
            .mutate("set -50", |w| w.set_value(-50.0))
            .check(|w, _| {
                let v = w.get_value();
                if (-10.0..=10.0).contains(&v) {
                    Ok(())
                } else {
                    Err(format!("value {v} outside -10..=10"))
                }
            })
            .done()
        }),
        e("Terminal", || {
            let mut t = Terminal::new(40, 10);
            t.writeln("$ echo 한글");
            t.focus();
            W::new(t, |w, k, _| {
                w.handle_key(k.clone());
            })
            .mutate("write lines", |w| {
                for i in 0..30 {
                    w.writeln(&format!("line {i}"));
                }
            })
            .mutate("clear", |w| w.clear())
            .mutate("resize 3x2", |w| w.resize(3, 2))
            .mutate("resize 0x0", |w| w.resize(0, 0))
            .done()
        }),
        e("Vim", || {
            W::headless(VimState::new(), |w, k, _| {
                w.handle_key(k);
            })
            .mutate("run :s", |w| {
                w.execute_command("s");
            })
            .done()
        }),
        // ── Pickers ─────────────────────────────────────────────────────────
        e("Calendar", || {
            W::new(
                Calendar::new(2026, 10).selected(date()).focused(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("select Feb 29", |w| w.select(Date::new(2024, 2, 29)))
            .mutate("clear selection", |w| w.clear_selection())
            .check(|w, _| match w.get_selected() {
                Some(d) => date_ok("selected", d),
                None => Ok(()),
            })
            .done()
        }),
        e("DateTimePicker", || {
            W::new(
                DateTimePicker::new().selected_date(Date::new(2024, 1, 31)),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .check(|w, _| {
                all([
                    date_ok("date", w.get_date()),
                    if w.get_time().is_valid() {
                        Ok(())
                    } else {
                        Err(format!("time {:?} is not valid", w.get_time()))
                    },
                ])
            })
            .done()
        }),
        e("DateTimePicker(range)", || {
            W::new(
                DateTimePicker::new()
                    .selected_date(date())
                    .date_range(Date::new(2026, 10, 1), Date::new(2026, 10, 20)),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .check(|w, _| {
                let d = w.get_date();
                all([
                    date_ok("date", d),
                    if w.is_date_valid(&d) {
                        Ok(())
                    } else {
                        Err(format!("date {d:?} outside the allowed range"))
                    },
                ])
            })
            .done()
        }),
        // Month/week navigation moves the stored dates along with the cursor
        // (by design, see `keep_cursor_in_limits`): the plain entry checks
        // the dates stay real dates, `(order)` that start stays before end.
        e("RangePicker", || {
            W::new(
                RangePicker::new().range(Date::new(2026, 1, 31), Date::new(2026, 3, 31)),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("start after end", |w| w.set_start(Date::new(2027, 1, 1)))
            .mutate("end Feb 29", |w| w.set_end(Date::new(2028, 2, 29)))
            .check(|w, _| {
                let (s, e) = w.get_range();
                all([date_ok("start", s), date_ok("end", e)])
            })
            .done()
        }),
        e("RangePicker(order)", || {
            W::new(
                RangePicker::new().range(date(), Date::new(2026, 10, 20)),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("start after end", |w| w.set_start(Date::new(2027, 1, 1)))
            .check(|w, _| {
                let (s, e) = w.get_range();
                if s <= e {
                    Ok(())
                } else {
                    Err(format!("start {s:?} after end {e:?}"))
                }
            })
            .done()
        }),
        e("ColorPicker", || {
            W::new(ColorPicker::new(), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("next mode", |w| w.next_mode())
            .mutate("set hex", |w| {
                w.set_hex("#ff8000");
            })
            .done()
        }),
        // ── Layout and disclosure ───────────────────────────────────────────
        e("Splitter", || {
            W::new(
                Splitter::new()
                    .pane(Pane::new("a").ratio(0.3).collapsible())
                    .pane(Pane::new("b").collapsible())
                    .pane(Pane::new("c")),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("collapse 0", |w| w.toggle_pane(0))
            .mutate("collapse 1", |w| w.toggle_pane(1))
            .mutate("resize divider 1", |w| w.start_resize(1))
            .check(|w, area| {
                let areas = w.pane_areas(area);
                for (id, r) in &areas {
                    let inside = r.width == 0
                        || r.height == 0
                        || (r.x >= area.x
                            && r.y >= area.y
                            && r.x + r.width <= area.x + area.width
                            && r.y + r.height <= area.y + area.height);
                    if !inside {
                        return Err(format!("pane {id} at {r:?} outside {area:?}"));
                    }
                }
                Ok(())
            })
            .done()
        }),
        e("Accordion", || {
            W::new(
                Accordion::new()
                    .section(AccordionSection::new("One").content("body 1"))
                    .section(AccordionSection::new("Two 한글").content("body\n2"))
                    .section(AccordionSection::new("Three")),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mutate("remove last", |w| {
                let n = w.len();
                if n > 0 {
                    w.remove_section(n - 1);
                }
            })
            .mutate("remove first", |w| {
                w.remove_section(0);
            })
            .mutate("add", |w| w.add_section(AccordionSection::new("New")))
            .check(|w, _| index_in("selected", w.selected(), w.len()))
            .done()
        }),
        e("Collapsible", || {
            W::new(
                Collapsible::new("Title").content("a\nb\n한글"),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .done()
        }),
        e("Card", || {
            W::new(Card::new().title("Card").collapsible(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .done()
        }),
        e("Callout", || {
            W::new(Callout::note("body 한글").collapsible(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .done()
        }),
        e("Alert", || {
            W::new(Alert::info("message").dismissible(true), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("reset", |w| w.reset())
            .done()
        }),
        e("ScrollView", || {
            W::new(ScrollView::new().content_height(50), |w, k, area| {
                w.handle_key(&k.key, area.height);
            })
            .mouse(|w, m, area| {
                w.handle_mouse(m, area.height);
            })
            .check(|w, area| {
                let max = 50u16.saturating_sub(area.height);
                if w.offset() <= max {
                    Ok(())
                } else {
                    Err(format!("offset {} past max {max}", w.offset()))
                }
            })
            .done()
        }),
        e("Resizable", || {
            W::new(Resizable::new(20, 6), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("set size 0x0", |w| w.set_size(0, 0))
            .done()
        }),
        // ── Overlays and menus ──────────────────────────────────────────────
        e("CommandPalette", || {
            let mut p = CommandPalette::new().commands(commands());
            p.show();
            W::new(p, |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("query 'sa'", |w| w.set_query("sa"))
            .mutate("query no match", |w| w.set_query("zzz"))
            .mutate("clear commands", |w| w.clear_commands())
            .mutate("remove save", |w| w.remove_command("save"))
            .mutate("add command", |w| w.add_command(Command::new("new", "New")))
            .mutate("show", |w| w.show())
            .done()
        }),
        e("MenuBar", || {
            W::new(menus(), |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("open menu 1", |w| w.open_menu(1))
            .mutate("close", |w| w.close())
            .done()
        }),
        e("ContextMenu", || {
            let mut m = ContextMenu::new().items(vec![
                MenuItem::new("Cut"),
                MenuItem::separator(),
                MenuItem::new("Off").disabled(true),
                MenuItem::new("More").submenu(vec![MenuItem::new("x")]),
            ]);
            m.show(3, 2);
            W::new(m, |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("show far", |w| w.show(u16::MAX, u16::MAX))
            .mutate("show", |w| w.show(1, 1))
            .done()
        }),
        e("Modal", || {
            let mut m = Modal::confirm("Title", "Body 한글");
            m.show();
            W::new(m, |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("show", |w| w.show())
            .check(|w, _| index_in("button", w.selected_button(), w.get_buttons().len()))
            .done()
        }),
        e("NotificationCenter", || {
            let mut n = NotificationCenter::new().focused(true);
            n.info("one");
            n.warning("two 한글");
            n.error("three");
            W::new(n, |w, k, _| {
                w.handle_key(&k.key);
            })
            .mutate("clear", |w| w.clear())
            .mutate("push", |w| w.info("new"))
            .mutate("tick", |w| w.tick())
            .done()
        }),
        e("Popover", || {
            W::new(
                Popover::new("content 한글").anchor(10, 3).open(true),
                |w, k, _| {
                    w.handle_key(&k.key);
                },
            )
            .mouse(|w, m, area| {
                if let MouseEventKind::Down(_) = m.kind {
                    w.handle_click(m.x, m.y, area.width, area.height);
                }
            })
            .mutate("show", |w| w.show())
            .mutate("anchor far", |w| w.set_anchor(u16::MAX, u16::MAX))
            .done()
        }),
        e("ToastQueue", || {
            let mut q = ToastQueue::new().pause_on_hover(true);
            q.info("one");
            q.error("two");
            W::new(q, |w, k, _| {
                if k.key == Key::Escape {
                    w.dismiss_first();
                }
            })
            .mouse(|w, m, area| {
                w.handle_mouse(m, area);
            })
            .mutate("push", |w| w.warning("more"))
            .mutate("tick", |w| w.tick())
            .mutate("dismiss all", |w| w.dismiss_all())
            .check(|w, _| {
                if w.visible_count() <= w.get_max_visible() {
                    Ok(())
                } else {
                    Err(format!(
                        "{} visible, max {}",
                        w.visible_count(),
                        w.get_max_visible()
                    ))
                }
            })
            .done()
        }),
    ]
}
