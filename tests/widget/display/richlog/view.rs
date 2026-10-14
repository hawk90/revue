//! RichLog placement, the prefix columns for any format, the entries kept
//! under logging and trimming, the selection and expansion under any moves,
//! and the scroll window for any scroll and key sequence

#![allow(deprecated)] // LogFormat::Custom draws like Standard

use proptest::prelude::*;
use revue::event::Key;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::{LogEntry, LogFormat, LogLevel, RichLog};

use super::{bare, draw, row, rows, text};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

#[test]
fn test_richlog_zero_areas_and_offset() {
    let mut log = RichLog::new().labels(true).bg(Color::BLUE);
    for i in 0..8 {
        log.log(
            LogEntry::new(format!("message {i}"))
                .level([LogLevel::Info, LogLevel::Error][i % 2])
                .timestamp("12:00:00")
                .source("app"),
        );
    }
    assert_zero_areas_draw_nothing(&log);
    assert_same_when_offset(log.clone(), 50, 5);
    let log = log.wrap(true).format(LogFormat::Simple);
    assert_zero_areas_draw_nothing(&log);
    assert_same_when_offset(log, 12, 4);
}

#[test]
fn test_richlog_empty_message_timestamp_and_source() {
    let mut log = RichLog::new().labels(true);
    log.log(LogEntry::new("").timestamp("").source(""));
    // Only the icon and the label are drawn
    assert_eq!(rows(&log, 40, 1), ["            ● INFO"]);
}

#[test]
fn test_richlog_one_column_and_one_row() {
    let mut log = bare();
    log.info("abc");
    assert_eq!(rows(&log, 1, 1), ["a"]);

    // One row of many entries: the entry at the scroll position, and the
    // marker on that row
    let mut log = bare().auto_scroll(false);
    for i in 0..5 {
        log.info(format!("m{i}"));
    }
    log.scroll_down(2);
    assert_eq!(rows(&log, 10, 1), ["m2       █"]);
}

#[test]
fn test_richlog_selection_stops_at_both_ends() {
    let mut log = bare();
    log.info("a");
    log.info("b");
    log.info("c");
    for _ in 0..5 {
        log.select_next();
    }
    let selected = |log: &RichLog| -> Vec<bool> {
        let buffer = draw(log, 5, 3);
        (0..3)
            .map(|y| buffer.get(0, y).unwrap().fg == Some(Color::WHITE))
            .collect()
    };
    assert_eq!(selected(&log), [false, false, true]);
    for _ in 0..5 {
        log.select_prev();
    }
    assert_eq!(selected(&log), [true, false, false]);
}

#[test]
fn test_richlog_select_prev_without_a_selection_selects_nothing() {
    let mut log = bare();
    log.info("a");
    log.select_prev();
    assert_eq!(draw(&log, 5, 1).get(0, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_richlog_clear_drops_the_selection() {
    let mut log = bare();
    log.info("a");
    log.info("b");
    log.select_next();
    log.select_next();
    assert!(log.handle_key(&Key::Char('c')));
    log.info("x");
    log.info("y");
    let buffer = draw(&log, 5, 2);
    assert_eq!(buffer.get(0, 1).unwrap().fg, Some(Color::CYAN));
    log.select_next();
    let buffer = draw(&log, 5, 2);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_richlog_level_styles() {
    let mut log = RichLog::new().timestamps(false).sources(false).labels(true);
    for level in [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warning,
        LogLevel::Error,
        LogLevel::Fatal,
    ] {
        log.log(LogEntry::new("m").level(level));
    }
    log.scroll_to_top();
    let buffer = draw(&log, 20, 6);
    let bold = [false, false, false, false, true, true];
    for (y, level) in [
        LogLevel::Trace,
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warning,
        LogLevel::Error,
        LogLevel::Fatal,
    ]
    .iter()
    .enumerate()
    {
        let y = y as u16;
        assert_eq!(
            row(&buffer, y),
            format!("{} {:<7}m", level.icon(), level.label())
        );
        // icon, label (always bold) and message in the level's color
        for x in [0, 2, 9] {
            assert_eq!(
                buffer.get(x, y).unwrap().fg,
                Some(level.color()),
                "{level:?} x={x}"
            );
        }
        assert!(buffer.get(2, y).unwrap().modifier.contains(Modifier::BOLD));
        assert!(!buffer.get(0, y).unwrap().modifier.contains(Modifier::BOLD));
        // the message is bold from Error up
        assert_eq!(
            buffer.get(9, y).unwrap().modifier.contains(Modifier::BOLD),
            bold[y as usize],
            "{level:?}"
        );
    }
    assert_eq!(LogLevel::Fatal.color(), Color::rgb(255, 50, 50));
}

#[test]
fn test_richlog_keys_move_the_view() {
    let mut log = bare().auto_scroll(false);
    for i in 0..30 {
        log.info(format!("m{i}"));
    }
    let first = |log: &RichLog| text(log, 10, 3)[0].clone();
    let steps = [
        (Key::PageDown, "m10"),
        (Key::Char('j'), "m11"),
        (Key::Down, "m12"),
        (Key::Char('k'), "m11"),
        (Key::Up, "m10"),
        (Key::PageUp, "m0"),
        (Key::End, "m27"),
        (Key::Home, "m0"),
        (Key::Char('G'), "m27"),
        (Key::Char('g'), "m0"),
    ];
    for (key, want) in steps {
        assert!(log.handle_key(&key));
        assert_eq!(first(&log), want, "after {key:?}");
    }
    // End turns following on again: a new entry is shown on the bottom row
    log.handle_key(&Key::End);
    log.info("new");
    assert_eq!(text(&log, 10, 3), ["m28", "m29", "new"]);
}

// ── The prefix columns for any format and builder set ─────────────────────

fn format() -> impl Strategy<Value = LogFormat> {
    prop_oneof![
        Just(LogFormat::Simple),
        Just(LogFormat::Standard),
        Just(LogFormat::Detailed),
        Just(LogFormat::Custom),
    ]
}

fn level() -> impl Strategy<Value = LogLevel> {
    prop_oneof![
        Just(LogLevel::Trace),
        Just(LogLevel::Debug),
        Just(LogLevel::Info),
        Just(LogLevel::Warning),
        Just(LogLevel::Error),
        Just(LogLevel::Fatal),
    ]
}

/// Put `s` at column `x` of `row`, one column per (ASCII) character
fn put(row: &mut [char], x: usize, s: &str) {
    for (i, ch) in s.chars().enumerate() {
        if x + i < row.len() {
            row[x + i] = ch;
        }
    }
}

proptest! {
    /// Each shown column sits at the sum of the widths before it (timestamp
    /// 12, icon 2, label 7, source 15), the timestamp and source are cut one
    /// short of their field, and the message starts after them, cut at the
    /// area's edge. `Simple` shows no columns, `Detailed` always shows the
    /// timestamp and source.
    #[test]
    fn prop_richlog_columns_line_up(
        format in format(),
        [ts_on, src_on, icon_on, label_on] in any::<[bool; 4]>(),
        level in level(),
        message in "[a-z ]{0,30}",
        timestamp in proptest::option::of("[0-9:]{0,16}"),
        source in proptest::option::of("[a-z:]{0,20}"),
        width in 1u16..70,
    ) {
        let log_with = |mut log: RichLog| {
            let mut entry = LogEntry::new(message.clone()).level(level);
            if let Some(ts) = &timestamp {
                entry = entry.timestamp(ts.clone());
            }
            if let Some(src) = &source {
                entry = entry.source(src.clone());
            }
            log.log(entry);
            log
        };
        let log = log_with(
            RichLog::new()
                .format(format)
                .timestamps(ts_on)
                .sources(src_on)
                .icons(icon_on)
                .labels(label_on),
        );

        let (ts, src, icon, label) = match format {
            LogFormat::Simple => (false, false, false, false),
            LogFormat::Detailed => (true, true, icon_on, label_on),
            _ => (ts_on, src_on, icon_on, label_on),
        };
        let mut want = vec![' '; width as usize];
        let mut x = 0;
        if ts {
            if let Some(t) = &timestamp {
                put(&mut want, 0, &t[..t.len().min(11)]);
            }
            x += 12;
        }
        if icon {
            put(&mut want, x, &level.icon().to_string());
            x += 2;
        }
        if label {
            put(&mut want, x, level.label());
            x += 7;
        }
        if src {
            if let Some(s) = &source {
                put(&mut want, x, &s[..s.len().min(14)]);
            }
            x += 15;
        }
        put(&mut want, x, &message);
        let want: String = want.into_iter().collect();
        prop_assert_eq!(&rows(&log, width, 1)[0], want.trim_end());
    }

    /// The log keeps the entries at or above `min_level`, in order, at most
    /// `max_entries` of them (the newest), and none after `clear`; scrolled
    /// to the top in a tall enough area, it draws exactly those.
    #[test]
    fn prop_richlog_keeps_the_newest_entries(
        min in level(),
        max in 0usize..6,
        ops in proptest::collection::vec(prop_oneof![
            8 => level().prop_map(Some),
            1 => Just(None), // clear
        ], 0..30),
    ) {
        let mut log = bare().min_level(min).max_entries(max);
        let mut model: Vec<String> = Vec::new();
        for (i, op) in ops.iter().enumerate() {
            match op {
                Some(level) => {
                    log.write(*level, format!("m{i}"));
                    if *level >= min {
                        model.push(format!("m{i}"));
                        if max > 0 && model.len() > max {
                            model.remove(0);
                        }
                    }
                }
                None => {
                    log.clear();
                    model.clear();
                }
            }
        }
        prop_assert_eq!(log.len(), model.len());
        prop_assert_eq!(log.is_empty(), model.is_empty());
        log.scroll_to_top();
        let h = model.len().max(1) as u16;
        let mut want = model.clone();
        want.resize(h as usize, String::new());
        prop_assert_eq!(rows(&log, 10, h), want);
    }
}

// ── Selection and expansion under any moves ────────────────────────────────

#[derive(Clone, Copy, Debug)]
enum Op {
    Next,
    Prev,
    Toggle,
    Log(bool), // with a detail line
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        3 => Just(Op::Next),
        2 => Just(Op::Prev),
        2 => Just(Op::Toggle),
        2 => any::<bool>().prop_map(Op::Log),
    ]
}

proptest! {
    /// A plain model: the selection starts on nothing, Next selects the first
    /// entry and then moves down until the last, Prev moves up until the
    /// first, Toggle opens or closes the selected entry's details. The
    /// screen shows each entry, its details when open, and the selected
    /// entry's message in white bold.
    #[test]
    fn prop_richlog_selection_and_details_follow_a_model(
        start in 0usize..5,
        ops in proptest::collection::vec(op(), 0..40),
    ) {
        // Entries are never trimmed or filtered here: a selection kept as a
        // position drifts when they are (#884, #975).
        let mut log = bare().auto_scroll(false).max_entries(0);
        // (has a detail, expanded)
        let mut model: Vec<(bool, bool)> = Vec::new();
        let mut selected: Option<usize> = None;
        let add = |log: &mut RichLog, model: &mut Vec<(bool, bool)>, detail: bool| {
            let i = model.len();
            let mut entry = LogEntry::new(format!("e{i}"));
            if detail {
                entry = entry.detail(format!("d{i}"));
            }
            log.log(entry);
            model.push((detail, false));
        };
        for i in 0..start {
            add(&mut log, &mut model, i % 2 == 0);
        }
        for op in ops {
            match op {
                Op::Next => {
                    log.select_next();
                    selected = match selected {
                        Some(i) if i + 1 < model.len() => Some(i + 1),
                        None if !model.is_empty() => Some(0),
                        s => s,
                    };
                }
                Op::Prev => {
                    log.select_prev();
                    selected = selected.map(|i| i.saturating_sub(1));
                }
                Op::Toggle => {
                    log.toggle_selected();
                    if let Some(i) = selected {
                        model[i].1 = !model[i].1;
                    }
                }
                Op::Log(detail) => add(&mut log, &mut model, detail),
            }
        }

        let mut want = Vec::new();
        let mut selected_row = None;
        for (i, (detail, open)) in model.iter().enumerate() {
            if selected == Some(i) {
                selected_row = Some(want.len());
            }
            want.push(format!("e{i}"));
            if *detail && *open {
                want.push(format!("  d{i}"));
            }
        }
        let h = want.len().max(1) as u16;
        let buffer = draw(&log, 10, h);
        let got: Vec<String> = (0..h).map(|y| row(&buffer, y)).collect();
        want.resize(h as usize, String::new());
        prop_assert_eq!(&got, &want);
        for (y, line) in want.iter().enumerate() {
            if line.starts_with('e') {
                let cell = buffer.get(0, y as u16).unwrap();
                let is_selected = selected_row == Some(y);
                prop_assert_eq!(cell.fg == Some(Color::WHITE), is_selected, "row {}", y);
                prop_assert_eq!(cell.modifier.contains(Modifier::BOLD), is_selected, "row {}", y);
            }
        }
    }
}

// ── The scroll window ──────────────────────────────────────────────────────

#[derive(Clone, Debug)]
enum ScrollOp {
    Up(usize),
    Down(usize),
    Top,
    Bottom,
    Key(Key),
    Log,
}

fn scroll_op() -> impl Strategy<Value = ScrollOp> {
    prop_oneof![
        1 => (1usize..15).prop_map(ScrollOp::Up),
        1 => (1usize..15).prop_map(ScrollOp::Down),
        1 => Just(ScrollOp::Top),
        1 => Just(ScrollOp::Bottom),
        2 => prop_oneof![
            Just(Key::Up),
            Just(Key::Down),
            Just(Key::PageUp),
            Just(Key::PageDown),
            Just(Key::Home),
            Just(Key::End),
            Just(Key::Char('j')),
            Just(Key::Char('k')),
            Just(Key::Char('g')),
            Just(Key::Char('G')),
        ]
        .prop_map(ScrollOp::Key),
        3 => Just(ScrollOp::Log),
    ]
}

proptest! {
    /// Whatever the scroll calls and keys, the area shows a run of
    /// consecutive entries with no blank row while entries remain below,
    /// and the marker is drawn exactly when the entries do not fit: on the
    /// top row at the first entry, on the bottom row at the last page. Top
    /// shows the first entry first; Bottom, and logging while following,
    /// show the last entry on the bottom row.
    #[test]
    fn prop_richlog_scroll_window(
        start in 0usize..25,
        auto in any::<bool>(),
        h in 1u16..8,
        ops in proptest::collection::vec(scroll_op(), 0..25),
    ) {
        let mut log = bare().auto_scroll(auto);
        let mut n = 0usize;
        for _ in 0..start {
            log.info(format!("m{n}"));
            n += 1;
        }
        // Whether the view is known to sit at the top or the bottom
        let mut at_top = !auto || n == 0;
        let mut following = auto;
        for op in ops {
            match op {
                ScrollOp::Up(k) => {
                    log.scroll_up(k);
                    following = false;
                }
                ScrollOp::Down(k) => log.scroll_down(k),
                ScrollOp::Top | ScrollOp::Key(Key::Home | Key::Char('g')) => {
                    if let ScrollOp::Key(k) = &op {
                        log.handle_key(k);
                    } else {
                        log.scroll_to_top();
                    }
                    at_top = true;
                    following = false;
                    continue;
                }
                ScrollOp::Bottom | ScrollOp::Key(Key::End | Key::Char('G')) => {
                    if let ScrollOp::Key(k) = &op {
                        log.handle_key(k);
                    } else {
                        log.scroll_to_bottom();
                    }
                    following = true;
                }
                ScrollOp::Key(k) => {
                    log.handle_key(&k);
                    if matches!(k, Key::Up | Key::PageUp | Key::Char('k')) {
                        following = false;
                    }
                }
                ScrollOp::Log => {
                    log.info(format!("m{n}"));
                    n += 1;
                }
            }
            at_top = false;
        }

        let buffer = draw(&log, 8, h);
        let shown: Vec<String> = (0..h)
            .map(|y| {
                (0..7)
                    .map(|x| buffer.get(x, y).unwrap().symbol)
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .collect();
        let marker: Vec<u16> = (0..h)
            .filter(|&y| buffer.get(7, y).unwrap().symbol == '█')
            .collect();
        let h = h as usize;
        if n == 0 {
            prop_assert!(shown.iter().all(|r| r.is_empty()));
            prop_assert!(marker.is_empty());
            return Ok(());
        }
        let first: usize = shown[0][1..].parse().unwrap();
        let count = (n - first).min(h);
        for (y, line) in shown.iter().enumerate() {
            if y < count {
                prop_assert_eq!(line, &format!("m{}", first + y));
            } else {
                prop_assert!(line.is_empty());
            }
        }
        // No blank row while entries remain: the last page is full
        prop_assert!(count == h || first == 0);
        let fits = n <= h;
        prop_assert_eq!(marker.len(), usize::from(!fits));
        if !fits {
            if first == 0 {
                prop_assert_eq!(marker[0], 0);
            }
            if first == n - h {
                prop_assert_eq!(marker[0] as usize, h - 1);
            }
        }
        if at_top {
            prop_assert_eq!(first, 0);
        }
        if following {
            prop_assert_eq!(first + count, n, "the last entry is not shown");
        }
    }

    /// With wrapping and open details an entry takes several rows; scrolled
    /// to the bottom, the last entry's last row is on the bottom row, and a
    /// log shorter than the area starts on the top row.
    #[test]
    fn prop_richlog_wrapped_bottom_is_full(
        messages in proptest::collection::vec(("[a-z]{1,6}( [a-z]{1,6}){0,4}", 0usize..3), 1..8),
        w in 4u16..20,
        k in 1usize..10,
        extra in 1usize..4,
    ) {
        let mut log = bare().wrap(true).auto_scroll(false);
        // The rows each entry takes, drawn alone
        let mut heights = Vec::new();
        for (message, details) in &messages {
            let mut entry = LogEntry::new(message.clone());
            for d in 0..*details {
                entry = entry.detail(format!("d{d}"));
            }
            entry.toggle();
            let mut alone = bare().wrap(true);
            alone.log(entry.clone());
            heights.push(rows(&alone, w, 50).iter().filter(|r| !r.is_empty()).count());
            log.log(entry);
        }
        log.log(LogEntry::new("END"));
        heights.push(1);
        // The area ends a page on an entry's first row: the last k entries
        // fill it exactly, or (k past the first entry) everything fits with
        // rows to spare. The bottom page starts at an entry's first row, so
        // when the rows from that entry to the end are more than the area,
        // the newest entries are cut off below it (#973: the scroll counts
        // entries, not rows); those heights are not generated.
        let total: usize = heights.iter().sum();
        let tail = if k <= heights.len() {
            heights[heights.len() - k..].iter().sum()
        } else {
            total + extra
        };
        let h = tail as u16;

        log.scroll_to_bottom();
        let shown = text(&log, w, h);
        let last = shown.iter().rposition(|r| !r.is_empty()).unwrap();
        prop_assert_eq!(&shown[last], "END");
        // On the bottom row, or right after the last entry when all fit
        prop_assert_eq!(last + 1, tail.min(total));
        if last + 1 < h as usize {
            // Everything fits: the log starts on the top row
            log.scroll_to_top();
            prop_assert_eq!(text(&log, w, h), shown);
        }
    }
}
