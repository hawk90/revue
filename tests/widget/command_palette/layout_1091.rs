//! #1091: every row fits in the frame above its bottom border, the cursor
//! keeps the placeholder's first letter, and a shortcut stays clear of the
//! label and inside the frame. #1289: Ctrl+J and Ctrl+K move the selection.

use revue::event::{Key, KeyEvent};
use revue::style::Color;
use revue::widget::{Command, CommandPalette};

use super::view::{render, row};

// In a 70-column area the 60-column frame spans columns 5..=64; the text
// columns end before column 63, a label starts at column 9 (after the icon
// columns) and the first command row is row 5.
const X: usize = 5;
const RIGHT: usize = 64;
const TEXT_END: usize = 63;
const LABEL_X: usize = 9;

fn shown(commands: Vec<Command>) -> CommandPalette {
    let mut p = CommandPalette::new().commands(commands);
    p.show();
    p
}

fn chars(text: &str) -> Vec<char> {
    text.chars().collect()
}

#[test]
fn the_only_match_is_drawn() {
    let mut p = shown(vec![
        Command::new("save", "Save File"),
        Command::new("quit", "Quit"),
    ]);
    p.set_query("save");
    assert_eq!(p.selected_id(), Some("save"));
    let b = render(&p, 70, 12);
    assert!(row(&b, 5).contains("Save File"), "{}", row(&b, 5));
    assert_eq!(chars(&row(&b, 6))[X], '╰');
    assert!(row(&b, 6).contains("1/2"), "{}", row(&b, 6));
}

#[test]
fn every_row_up_to_max_visible_is_drawn() {
    let commands = (0..5)
        .map(|i| Command::new(format!("c{i}"), format!("Command {i}")))
        .collect();
    let mut p = shown(commands);
    for _ in 0..4 {
        p.select_next();
    }
    assert_eq!(p.selected_id(), Some("c4"));
    let b = render(&p, 70, 14);
    for i in 0..5u16 {
        assert!(
            row(&b, 5 + i).contains(&format!("Command {i} ")),
            "{}",
            row(&b, 5 + i)
        );
    }
    // The selected last row is on the selection background
    assert_eq!(
        b.get(X as u16 + 1, 9).unwrap().bg,
        Some(Color::rgb(50, 80, 120))
    );
    assert_eq!(chars(&row(&b, 10))[X], '╰');
}

#[test]
fn with_a_title_the_last_row_is_drawn() {
    let mut p = CommandPalette::new()
        .title("Run")
        .commands(vec![Command::new("a", "Alpha"), Command::new("b", "Beta")]);
    p.show();
    let b = render(&p, 70, 12);
    assert!(row(&b, 7).contains("Beta"), "{}", row(&b, 7));
    assert_eq!(chars(&row(&b, 8))[X], '╰');
}

#[test]
fn an_empty_palette_keeps_its_separator() {
    let b = render(&shown(vec![]), 70, 12);
    assert_eq!(chars(&row(&b, 4))[X], '├');
    assert_eq!(chars(&row(&b, 5))[X], '╰');
}

#[test]
fn the_placeholder_keeps_its_first_letter() {
    let b = render(&shown(vec![]), 70, 10);
    assert!(row(&b, 3).contains("Type to search..."), "{}", row(&b, 3));
}

#[test]
fn a_long_query_keeps_the_cursor_inside_the_frame() {
    let mut p = shown(vec![]);
    p.set_query("q".repeat(80));
    let b = render(&p, 70, 12);
    let line = chars(&row(&b, 3));
    // The end of the query and the cursor after it, in the last text column
    assert!(
        line[LABEL_X..TEXT_END - 1].iter().all(|&c| c == 'q'),
        "{}",
        row(&b, 3)
    );
    assert_eq!(
        b.get(TEXT_END as u16 - 1, 3).unwrap().bg,
        Some(Color::WHITE)
    );
    assert_eq!(line[TEXT_END], ' ');
    assert_eq!(line[RIGHT], '│');
}

#[test]
fn a_long_label_stops_a_column_before_its_shortcut() {
    let label = "Rebuild every project in the workspace";
    let p = shown(vec![
        Command::new("l", label).shortcut("Ctrl+Alt+Shift+B"),
        Command::new("quit", "Quit"),
    ]);
    let b = render(&p, 70, 12);
    let line = chars(&row(&b, 5));
    // The shortcut ends at the text columns' end: it starts at 63 - 16
    let sx = TEXT_END - 16;
    let got: String = line[LABEL_X..sx - 1].iter().collect();
    assert_eq!(got, label[..sx - 1 - LABEL_X]);
    assert_eq!(line[sx - 1], ' ');
    let got: String = line[sx..TEXT_END].iter().collect();
    assert_eq!(got, "Ctrl+Alt+Shift+B");
    assert_eq!(line[TEXT_END], ' ');
    assert_eq!(line[RIGHT], '│');
}

#[test]
fn a_wide_glyph_does_not_cross_into_the_gap_before_the_shortcut() {
    let p = shown(vec![
        Command::new("w", "가".repeat(30)).shortcut("Ctrl+Alt+Shift+B"),
        Command::new("quit", "Quit"),
    ]);
    let b = render(&p, 70, 12);
    // The label has columns 9..46: 18 glyphs fill 9..45; a 19th would take
    // 45 and 46, the gap
    let sx = TEXT_END - 16;
    assert_eq!(b.get(43, 5).unwrap().symbol, '가');
    assert_eq!(b.get(45, 5).unwrap().symbol, ' ');
    assert_eq!(b.get(sx as u16 - 1, 5).unwrap().symbol, ' ');
    assert!(!b.get(sx as u16 - 1, 5).unwrap().is_continuation());
    assert_eq!(b.get(sx as u16, 5).unwrap().symbol, 'C');
}

#[test]
fn a_shortcut_that_leaves_one_label_column_is_drawn() {
    // 63 - 52 = 11: the label keeps column 9, column 10 is the gap
    let shortcut = "s".repeat(52);
    let p = shown(vec![
        Command::new("a", "Alpha").shortcut(shortcut.clone()),
        Command::new("quit", "Quit"),
    ]);
    let b = render(&p, 70, 12);
    let line = chars(&row(&b, 5));
    assert_eq!(line[LABEL_X], 'A');
    assert_eq!(line[LABEL_X + 1], ' ');
    let got: String = line[LABEL_X + 2..TEXT_END].iter().collect();
    assert_eq!(got, shortcut);
}

#[test]
fn a_shortcut_that_leaves_no_label_column_is_left_out() {
    let p = shown(vec![
        Command::new("a", "Alpha").shortcut("s".repeat(53)),
        Command::new("quit", "Quit"),
    ]);
    let b = render(&p, 70, 12);
    let line = row(&b, 5);
    assert!(line.contains("Alpha"), "{line}");
    assert!(!line.contains('s'), "{line}");
}

#[test]
fn a_shortcut_wider_than_the_frame_stays_out_of_its_borders() {
    let p = shown(vec![
        Command::new("pin", "Pin").shortcut("Ctrl+Shift+P"),
        Command::new("quit", "Quit"),
    ]);
    // A 12-column area: the frame spans columns 0..=11 and the shortcut
    // does not fit beside the label
    let b = render(&p, 12, 10);
    assert_eq!(row(&b, 5), "│   Pin    │");
}

#[test]
fn ctrl_j_and_ctrl_k_move_the_selection() {
    let mut p = shown(vec![
        Command::new("a", "Alpha"),
        Command::new("b", "Beta"),
        Command::new("c", "Gamma"),
    ]);
    assert!(p.handle_key_event(&KeyEvent::ctrl(Key::Char('j'))));
    assert_eq!(p.selected_id(), Some("b"));
    assert!(p.handle_key_event(&KeyEvent::ctrl(Key::Char('J'))));
    assert_eq!(p.selected_id(), Some("c"));
    assert!(p.handle_key_event(&KeyEvent::ctrl(Key::Char('k'))));
    assert_eq!(p.selected_id(), Some("b"));
    assert_eq!(p.get_query(), "");
}

#[test]
fn a_hidden_palette_leaves_ctrl_j() {
    let mut p = CommandPalette::new().commands(vec![Command::new("a", "Alpha")]);
    assert!(!p.handle_key_event(&KeyEvent::ctrl(Key::Char('j'))));
}

#[test]
fn alt_j_is_left_and_altgr_j_is_typed() {
    let mut p = shown(vec![Command::new("a", "Alpha"), Command::new("b", "Beta")]);
    assert!(!p.handle_key_event(&KeyEvent::alt(Key::Char('j'))));
    assert_eq!(p.selected_id(), Some("a"));
    let mut altgr = KeyEvent::ctrl(Key::Char('j'));
    altgr.alt = true;
    assert!(p.handle_key_event(&altgr));
    assert_eq!(p.get_query(), "j");
}

#[test]
fn a_bare_j_is_still_typed() {
    let mut p = shown(vec![Command::new("a", "Alpha"), Command::new("b", "Beta")]);
    assert!(p.handle_key(&Key::Char('j')));
    assert_eq!(p.get_query(), "j");
}
