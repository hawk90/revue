//! CommandPalette on the screen: the frame, the search line, the rows and
//! their colors, the count, placement and the stylesheet

use revue::layout::Rect;
use revue::prelude::*;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::theme::{DARK_GRAY, DISABLED_FG, EDITOR_BG, SUBTLE_GRAY};
use revue::widget::{Command, CommandPalette};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// Render `palette` into a `w` x `h` area at the origin.
pub(super) fn render(palette: &CommandPalette, w: u16, h: u16) -> Buffer {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    palette.render(&mut ctx);
    buffer
}

/// Row `y` of `buffer` as text.
pub(super) fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .collect()
}

/// Every cell of `buffer`, row by row, to compare two renders.
pub(super) fn cells(buffer: &Buffer) -> Vec<revue::render::Cell> {
    buffer.iter_cells().map(|(_, _, c)| *c).collect()
}

fn cell(buffer: &Buffer, x: u16, y: u16) -> &revue::render::Cell {
    buffer.get(x, y).unwrap()
}

fn shown(commands: Vec<Command>) -> CommandPalette {
    let mut p = CommandPalette::new().commands(commands);
    p.show();
    p
}

fn sample() -> Vec<Command> {
    vec![
        Command::new("save", "Save File")
            .shortcut("Ctrl+S")
            .icon('S'),
        Command::new("open", "Open File").description("From disk"),
        Command::new("quit", "Quit"),
    ]
}

// In a 70-column area the default 60-column frame starts at column 5 and its
// right border is column 64; it starts two rows down. Without a title the
// search line is row 3, the separator row 4 and the first command row 5.
const X: u16 = 5;
const RIGHT: u16 = 64;

#[test]
fn zero_areas_draw_nothing() {
    assert_zero_areas_draw_nothing(&CommandPalette::new().commands(sample()));
    assert_zero_areas_draw_nothing(&shown(sample()));
    let mut titled = CommandPalette::new().title("Run").commands(sample());
    titled.show();
    assert_zero_areas_draw_nothing(&titled);
}

#[test]
fn draws_the_same_when_offset() {
    assert_same_when_offset(shown(sample()), 70, 14);
    let mut p = CommandPalette::new().title("Run").commands(sample());
    p.show();
    p.set_query("fi");
    p.select_next();
    assert_same_when_offset(p, 70, 14);
}

#[test]
fn the_frame_is_centered_two_rows_down_in_the_border_color() {
    let b = render(&shown(sample()), 70, 14);
    assert_eq!(row(&b, 0).trim(), "");
    assert_eq!(row(&b, 1).trim(), "");
    let top = row(&b, 2);
    assert_eq!(top.chars().nth(X as usize), Some('╭'), "{top}");
    assert_eq!(top.chars().nth(RIGHT as usize), Some('╮'), "{top}");
    assert!(top.chars().skip(X as usize + 1).take(58).all(|c| c == '─'));
    let sep = row(&b, 4);
    assert_eq!(sep.chars().nth(X as usize), Some('├'), "{sep}");
    assert_eq!(sep.chars().nth(RIGHT as usize), Some('┤'), "{sep}");
    for y in [3, 5] {
        assert_eq!(cell(&b, X, y).symbol, '│');
        assert_eq!(cell(&b, RIGHT, y).symbol, '│');
    }
    for (x, y) in [(X, 2), (RIGHT, 2), (X + 1, 2), (X, 3), (X, 4), (RIGHT, 5)] {
        assert_eq!(cell(&b, x, y).fg, Some(DARK_GRAY), "({x}, {y})");
    }
    // Nothing left or right of the frame.
    for y in 0..14 {
        assert_eq!(cell(&b, X - 1, y).symbol, ' ');
        assert_eq!(cell(&b, RIGHT + 1, y).symbol, ' ');
    }
}

#[test]
fn a_narrow_area_narrows_the_frame() {
    let b = render(&shown(sample()), 40, 12);
    assert_eq!(cell(&b, 0, 2).symbol, '╭');
    assert_eq!(cell(&b, 39, 2).symbol, '╮');
}

#[test]
fn the_search_line_shows_the_icon_the_query_and_a_cursor_after_it() {
    let mut p = shown(sample());
    p.set_query("fi");
    let b = render(&p, 70, 14);
    assert_eq!(cell(&b, X + 2, 3).symbol, '🔍');
    assert_eq!(cell(&b, X + 4, 3).symbol, 'f');
    assert_eq!(cell(&b, X + 5, 3).symbol, 'i');
    for x in [X + 4, X + 5] {
        assert_eq!(cell(&b, x, 3).fg, Some(Color::WHITE));
        assert_eq!(cell(&b, x, 3).bg, Some(EDITOR_BG));
    }
    assert_eq!(cell(&b, X + 6, 3).symbol, '▏');
    assert_eq!(cell(&b, X + 6, 3).fg, Some(Color::WHITE));
}

/// The placeholder is drawn in the disabled grey. Its first column is left
/// out: the cursor is drawn over the placeholder's first letter
/// (draft-cmd-1).
#[test]
fn the_placeholder_is_grey() {
    let mut p = CommandPalette::new().placeholder("Find a command");
    p.show();
    let b = render(&p, 70, 12);
    assert!(row(&b, 3).contains("ind a command"), "{}", row(&b, 3));
    for x in X + 5..X + 18 {
        assert_eq!(cell(&b, x, 3).fg, Some(DISABLED_FG), "column {x}");
    }
}

#[test]
fn the_title_is_bold_cyan_above_the_search_line() {
    let mut p = CommandPalette::new().title("Run").commands(sample());
    p.show();
    let b = render(&p, 70, 14);
    let title: String = (X + 2..X + 5).map(|x| cell(&b, x, 3).symbol).collect();
    assert_eq!(title, "Run");
    for x in X + 2..X + 5 {
        assert_eq!(cell(&b, x, 3).fg, Some(Color::CYAN));
        assert!(cell(&b, x, 3).modifier.contains(Modifier::BOLD));
    }
    assert_eq!(cell(&b, X, 3).symbol, '│');
    assert_eq!(cell(&b, X + 2, 4).symbol, '🔍');
    assert_eq!(cell(&b, X, 5).symbol, '├');
    assert!(row(&b, 6).contains("Save File"), "{}", row(&b, 6));
}

#[test]
fn a_long_title_stops_before_the_right_border() {
    let mut p = CommandPalette::new().title("T".repeat(80));
    p.show();
    let b = render(&p, 70, 12);
    // The text ends two columns before the frame's end: one blank, then the border.
    assert_eq!(cell(&b, RIGHT - 2, 3).symbol, 'T');
    assert_eq!(cell(&b, RIGHT - 1, 3).symbol, ' ');
    assert_eq!(cell(&b, RIGHT, 3).symbol, '│');
}

#[test]
fn the_selected_row_takes_the_selection_background() {
    let mut p = shown(sample());
    p.select_next();
    let b = render(&p, 70, 14);
    let selected = Color::rgb(50, 80, 120);
    for x in X + 1..RIGHT {
        assert_eq!(
            cell(&b, x, 6).bg,
            Some(selected),
            "selected row, column {x}"
        );
        assert_eq!(cell(&b, x, 5).bg, Some(EDITOR_BG), "first row, column {x}");
    }
    assert!(row(&b, 6).contains("Open File"));
}

#[test]
fn query_matches_are_bold_in_the_match_color() {
    // Two matches with the same letters: the last row is under the bottom
    // border (draft-cmd-1).
    let mut p = shown(vec![
        Command::new("a", "Save File").icon('S'),
        Command::new("b", "Save Files").icon('S'),
        Command::new("c", "Quit"),
    ]);
    p.set_query("sf");
    let b = render(&p, 70, 14);
    // "Save File" after the two icon columns; S and F match.
    let label: String = (X + 4..X + 13).map(|x| cell(&b, x, 5).symbol).collect();
    assert_eq!(label, "Save File");
    for (i, x) in (X + 4..X + 13).enumerate() {
        let c = cell(&b, x, 5);
        let matched = i == 0 || i == 5;
        assert_eq!(
            c.fg,
            Some(if matched { Color::YELLOW } else { Color::WHITE }),
            "{i}"
        );
        assert_eq!(c.modifier.contains(Modifier::BOLD), matched, "{i}");
    }
}

#[test]
fn icons_are_cyan_and_the_label_moves_left_without_them() {
    let b = render(&shown(sample()), 70, 14);
    assert_eq!(cell(&b, X + 2, 5).symbol, 'S');
    assert_eq!(cell(&b, X + 2, 5).fg, Some(Color::CYAN));
    assert_eq!(cell(&b, X + 4, 5).symbol, 'S');
    // A command without an icon keeps the icon's columns.
    assert_eq!(cell(&b, X + 2, 6).symbol, ' ');
    assert_eq!(cell(&b, X + 4, 6).symbol, 'O');

    let mut p = CommandPalette::new().show_icons(false).commands(sample());
    p.show();
    let b = render(&p, 70, 14);
    assert_eq!(cell(&b, X + 2, 5).symbol, 'S');
    assert_eq!(cell(&b, X + 2, 5).fg, Some(Color::WHITE));
    assert_eq!(cell(&b, X + 2, 6).symbol, 'O');
}

#[test]
fn shortcuts_end_before_the_border_and_descriptions_follow_the_label() {
    let b = render(&shown(sample()), 70, 14);
    let shortcut: String = (RIGHT - 7..RIGHT - 1)
        .map(|x| cell(&b, x, 5).symbol)
        .collect();
    assert_eq!(shortcut, "Ctrl+S");
    for x in RIGHT - 7..RIGHT - 1 {
        assert_eq!(cell(&b, x, 5).fg, Some(SUBTLE_GRAY));
    }
    // "Open File", two blanks, then the description.
    let desc: String = (X + 15..X + 24).map(|x| cell(&b, x, 6).symbol).collect();
    assert_eq!(desc, "From disk");
    assert_eq!(cell(&b, X + 13, 6).symbol, ' ');
    assert_eq!(cell(&b, X + 15, 6).fg, Some(SUBTLE_GRAY));
}

#[test]
fn a_description_stops_before_the_shortcut() {
    // A second command: the last row is under the bottom border (draft-cmd-1).
    let p = shown(vec![
        Command::new("a", "A")
            .description("d".repeat(80))
            .shortcut("Ctrl+S"),
        Command::new("b", "B"),
    ]);
    let b = render(&p, 70, 12);
    // The shortcut starts at RIGHT - 7; one blank column separates them.
    assert_eq!(cell(&b, RIGHT - 9, 5).symbol, 'd');
    assert_eq!(cell(&b, RIGHT - 8, 5).symbol, ' ');
    assert_eq!(cell(&b, RIGHT - 7, 5).symbol, 'C');
}

#[test]
fn a_long_label_stops_at_the_label_column() {
    let label = "L".repeat(80);
    let b = render(
        &shown(vec![Command::new("a", &label), Command::new("b", "B")]),
        70,
        12,
    );
    // The label column ends 15 columns before the frame does.
    assert_eq!(cell(&b, RIGHT - 15, 5).symbol, 'L');
    assert_eq!(cell(&b, RIGHT - 14, 5).symbol, ' ');
}

#[test]
fn the_count_sits_on_the_bottom_border() {
    let mut p = shown(sample());
    p.set_query("file");
    let b = render(&p, 70, 14);
    let bottom = (0..14)
        .find(|&y| cell(&b, X, y).symbol == '╰')
        .expect("a bottom border");
    assert_eq!(cell(&b, RIGHT, bottom).symbol, '╯');
    let count: String = (RIGHT - 4..RIGHT - 1)
        .map(|x| cell(&b, x, bottom).symbol)
        .collect();
    assert_eq!(count, "2/3");
    assert_eq!(cell(&b, RIGHT - 4, bottom).fg, Some(DARK_GRAY));
    assert_eq!(cell(&b, RIGHT - 1, bottom).symbol, '─');
}

#[test]
fn an_empty_palette_draws_its_frame_and_a_zero_count() {
    let b = render(&shown(vec![]), 70, 12);
    assert_eq!(cell(&b, X, 2).symbol, '╭');
    assert_eq!(cell(&b, X + 2, 3).symbol, '🔍');
    let bottom = (0..12).find(|&y| cell(&b, X, y).symbol == '╰').unwrap();
    assert!(row(&b, bottom).contains("0/0"), "{}", row(&b, bottom));
}

/// The frame needs two rows above it, the search line, the separator, the
/// rows and the borders; in a shorter area it draws nothing rather than a
/// cut frame.
#[test]
fn a_palette_taller_than_the_area_draws_nothing() {
    let p = shown(sample());
    // Three commands: the frame's height is computed as 3 + 3, and two rows
    // above it make 8 (draft-cmd-1 counts one row short); 7 do not fit.
    let b = render(&p, 70, 8);
    assert_eq!(cell(&b, X, 2).symbol, '╭');
    let b = render(&p, 70, 7);
    assert!((0..7).all(|y| row(&b, y).trim().is_empty()));
}

#[test]
fn rows_scroll_with_the_selection() {
    let commands = (0..20)
        .map(|i| Command::new(format!("c{i}"), format!("Command {i}")))
        .collect();
    let mut p = CommandPalette::new().max_visible(5).commands(commands);
    p.show();
    for _ in 0..7 {
        p.select_next();
    }
    assert_eq!(p.selection.offset(), 3);
    let b = render(&p, 70, 14);
    // Rows 5.. show commands from the offset on. The fifth row is left out:
    // the bottom border covers it (draft-cmd-1).
    for i in 0..4 {
        let text = row(&b, 5 + i);
        assert!(text.contains(&format!("Command {} ", 3 + i)), "{text}");
    }
}

#[test]
fn the_query_takes_the_stylesheet_color_and_the_placeholder_keeps_its_grey() {
    struct Root(CommandPalette);
    impl View for Root {
        fn render(&self, ctx: &mut RenderContext) {
            vstack().child(self.0.clone()).render(ctx);
        }
        fn widget_type(&self) -> &'static str {
            "Root"
        }
        fn id(&self) -> Option<&str> {
            Some("root")
        }
    }
    let red = Color::rgb(255, 0, 0);
    let draw = |query: &str| {
        let mut p = CommandPalette::new().element_id("p").commands(sample());
        p.show();
        p.set_query(query);
        let mut h =
            PipelineHarness::with_css("#p { color: #ff0000; }", 70, 14).dom_from_render(true);
        h.draw(&Root(p));
        h
    };
    let h = draw("fi");
    let b = h.buffer();
    let search = (0..14)
        .find(|&y| (0..70).any(|x| b.get(x, y).unwrap().symbol == '🔍'))
        .unwrap();
    let f = (0..70)
        .find(|&x| b.get(x, search).unwrap().symbol == 'f')
        .unwrap();
    assert_eq!(b.get(f, search).unwrap().fg, Some(red));
    assert_eq!(b.get(f + 1, search).unwrap().fg, Some(red));

    let h = draw("");
    let b = h.buffer();
    let y = (0..14)
        .find(|&y| (0..70).any(|x| b.get(x, y).unwrap().symbol == '🔍'))
        .unwrap();
    let s = (0..70)
        .find(|&x| b.get(x, y).unwrap().symbol == 's')
        .unwrap();
    assert_eq!(b.get(s, y).unwrap().fg, Some(DISABLED_FG));
}
