//! Modal placement, the box a plain model of the layout draws, a model of
//! button selection under keys, and the focus trap

use proptest::prelude::*;
use revue::event::{FocusManager, Key};
use revue::layout::Rect;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::{Modal, ModalButton, ModalButtonStyle, Text};

use super::{draw, draw_at, row};
use crate::traits::placement::assert_zero_areas_draw_nothing;

fn sample() -> Modal {
    let mut m = Modal::new()
        .title("Delete 파일")
        .content("This cannot be undone.\nReally?")
        .buttons(vec![
            ModalButton::danger("Delete"),
            ModalButton::new("Keep"),
        ]);
    m.show();
    m
}

#[test]
fn test_modal_zero_areas_and_offset() {
    assert_zero_areas_draw_nothing(&sample());
    // Drawn into an area at (7, 4) of a larger buffer, it is the same box
    // moved by that offset, and nothing outside the area changes
    let alone = draw(&sample(), 40, 12);
    let placed = draw_at(&sample(), 60, 20, Rect::new(7, 4, 40, 12));
    for y in 0..20 {
        for x in 0..60 {
            let inside = (7..47).contains(&x) && (4..16).contains(&y);
            let cell = placed.get(x, y).unwrap();
            if inside {
                assert_eq!(cell, alone.get(x - 7, y - 4).unwrap(), "({x}, {y})");
            } else {
                assert_eq!(cell.symbol, ' ', "({x}, {y})");
            }
        }
    }
}

#[test]
fn test_modal_hidden_draws_nothing() {
    let mut m = sample();
    m.hide();
    let b = draw(&m, 40, 12);
    assert!((0..12).all(|y| row(&b, y).trim().is_empty()));
    m.toggle();
    assert!(row(&draw(&m, 40, 12), 2).contains('┌'));
}

#[test]
fn test_modal_colors() {
    // Border and title colors from the builders; content in light grey
    let mut m = Modal::new()
        .title("T")
        .content("body")
        .title_fg(Color::CYAN)
        .border_fg(Color::GREEN);
    m.show();
    // 40 x 6 box centered in 44 x 9: rows 1 to 6 from x = 2. The separator
    // under the title (row 3) joins the border, so it is in the border color
    // too (#1060).
    let b = draw(&m, 44, 9);
    assert_eq!(b.get(2, 1).unwrap().symbol, '┌');
    assert_eq!(row(&b, 3).trim(), format!("├{}┤", "─".repeat(38)));
    for (x, y) in [
        (2, 1),
        (20, 1),
        (41, 1),
        (2, 3),
        (20, 3),
        (41, 3),
        (2, 5),
        (41, 5),
        (2, 6),
        (41, 6),
    ] {
        assert_eq!(b.get(x, y).unwrap().fg, Some(Color::GREEN), "({x}, {y})");
    }
    let title = b.get(4, 2).unwrap();
    assert_eq!((title.symbol, title.fg), ('T', Some(Color::CYAN)));
    assert!(title.modifier.contains(Modifier::BOLD));
    let body = b.get(4, 4).unwrap();
    assert_eq!(
        (body.symbol, body.fg),
        ('b', Some(Color::rgb(220, 220, 220)))
    );
}

#[test]
fn test_modal_selected_button_styles() {
    // Only the selected button is highlighted, in its style's colors
    let cases = [
        (ModalButtonStyle::Primary, Color::WHITE, Color::BLUE),
        (ModalButtonStyle::Danger, Color::WHITE, Color::RED),
        (ModalButtonStyle::Default, Color::BLACK, Color::WHITE),
    ];
    for (style, fg, bg) in cases {
        let mut m = Modal::new().buttons(vec![
            ModalButton::new("A").style(style),
            ModalButton::new("B").style(style),
        ]);
        m.show();
        m.next_button();
        // "[ A ]  [ B ]" is 12 wide, centered in the 40-wide box at x = 2
        let b = draw(&m, 44, 8);
        let y = (0..8).find(|&y| row(&b, y).contains("[ A ]")).unwrap();
        assert_eq!(row(&b, y), format!("  │{:^38}│  ", "[ A ]  [ B ]"));
        let a = b.get(16, y).unwrap();
        assert_eq!((a.symbol, a.fg, a.bg), ('[', None, None), "{style:?}");
        let sel = b.get(23, y).unwrap();
        assert_eq!(
            (sel.symbol, sel.fg, sel.bg),
            ('[', Some(fg), Some(bg)),
            "{style:?}"
        );
        assert!(sel.modifier.contains(Modifier::BOLD));
    }
}

#[test]
fn test_modal_buttons_that_do_not_fit_are_not_drawn() {
    // "[ Cancel ]" is 10 wide: an 11-wide box shows it, a 9-wide one does not
    for (width, shown) in [(11, true), (10, true), (9, false)] {
        let mut m = Modal::new().width(width).cancel();
        m.show();
        let b = draw(&m, 20, 8);
        let any = (0..8).any(|y| row(&b, y).contains("[ Cancel ]"));
        assert_eq!(any, shown, "{width}");
    }
}

#[test]
fn test_modal_body_widget_is_drawn_inside() {
    let mut m = Modal::new()
        .title("Form")
        .body(Text::new("Widget body"))
        .ok();
    m.show();
    // Body modals reserve five content rows: 3 + 5 + 1 + 1 + 1
    assert_eq!(m.required_height(), 11);
    let b = draw(&m, 44, 13);
    assert_eq!(row(&b, 4).trim_end(), format!("{:<41}│", "  │ Widget body"));
    // Text content is ignored once a body is set
    let mut m = Modal::new().content("ignored").body(Text::new("shown"));
    m.show();
    let b = draw(&m, 44, 13);
    assert!((0..13).all(|y| !row(&b, y).contains("ignored")));
    assert!((0..13).any(|y| row(&b, y).contains("shown")));
}

#[test]
fn test_modal_space_confirms_and_releases_the_trap() {
    let mut fm = FocusManager::new();
    for id in [1, 10, 11] {
        fm.register(id);
    }
    fm.focus(1);
    let mut m = Modal::new().ok_cancel();
    m.show_with_focus_trap(&mut fm, 100, &[10, 11]);
    m.handle_key_with_focus(&Key::Right, &mut fm);
    assert!(fm.is_trapped());
    assert_eq!(m.handle_key_with_focus(&Key::Char(' '), &mut fm), Some(1));
    assert!(!fm.is_trapped());
    assert!(!m.has_focus_trap());
}

#[test]
fn test_modal_enter_without_buttons_keeps_the_trap() {
    let mut fm = FocusManager::new();
    for id in [1, 10] {
        fm.register(id);
    }
    fm.focus(1);
    let mut m = Modal::new();
    m.show_with_focus_trap(&mut fm, 100, &[10]);
    assert_eq!(m.handle_key_with_focus(&Key::Enter, &mut fm), None);
    assert!(fm.is_trapped());
    assert!(m.has_focus_trap());
    // A second hide restores nothing more
    m.hide_with_focus_restore(&mut fm);
    m.hide_with_focus_restore(&mut fm);
    assert_eq!(fm.current(), Some(1));
}

/// The box a plain model of the layout draws for ASCII text: centered,
/// `width` wide (at most the area minus four columns) and cut to the area's
/// height, the title row and separator when the box has rows for them inside
/// its border, the content lines cut at the padding, and the buttons
/// centered on the row above the bottom border when they fit.
fn model(
    title: &str,
    lines: &[String],
    labels: &[String],
    width: u16,
    aw: u16,
    ah: u16,
    req: u16,
) -> Vec<String> {
    let (aw, ah) = (aw as usize, ah as usize);
    let mut g = vec![vec![' '; aw]; ah];
    let mw = (width as usize).min(aw.saturating_sub(4));
    let mh = (req as usize).min(ah.saturating_sub(2));
    let x = (aw - mw) / 2;
    let y = (ah - mh) / 2;
    let put = |g: &mut Vec<Vec<char>>, x0: usize, yy: usize, s: &str, max: usize| {
        for (i, c) in s.chars().take(max).enumerate() {
            if x0 + i < aw && yy < ah {
                g[yy][x0 + i] = c;
            }
        }
    };
    if mw >= 2 && mh >= 2 {
        for (yy, r) in g.iter_mut().enumerate().skip(y).take(mh) {
            let (l, m, rt) = if yy == y {
                ('┌', '─', '┐')
            } else if yy == y + mh - 1 {
                ('└', '─', '┘')
            } else {
                ('│', ' ', '│')
            };
            r[x] = l;
            for c in r.iter_mut().skip(x + 1).take(mw - 2) {
                *c = m;
            }
            r[x + mw - 1] = rt;
        }
    }
    let text = labels
        .iter()
        .map(|l| format!("[ {l} ]"))
        .collect::<Vec<_>>()
        .join("  ");
    let total = text.chars().count();
    let buttons = !labels.is_empty() && mh > 2 && total <= mw;
    // The title and separator take rows `1..end` of the box, above the
    // buttons' row and the bottom border
    let end = mh.saturating_sub(1 + usize::from(buttons));
    let has_title = !title.is_empty() && mw > 4;
    if has_title && end > 1 {
        put(&mut g, x + 2, y + 1, title, mw - 4);
    }
    if has_title && end > 2 {
        let sep: String = std::iter::once('├')
            .chain(std::iter::repeat_n('─', mw - 2))
            .chain(std::iter::once('┤'))
            .collect();
        put(&mut g, x, y + 2, &sep, mw);
    }
    let cy0 = if has_title { y + 3 } else { y + 1 };
    for (i, line) in lines.iter().enumerate() {
        if cy0 + i >= y + mh.saturating_sub(2) {
            break;
        }
        put(&mut g, x + 2, cy0 + i, line, mw.saturating_sub(4));
    }
    if buttons {
        put(&mut g, x + (mw - total) / 2, y + mh - 2, &text, total);
    }
    g.into_iter().map(|r| r.into_iter().collect()).collect()
}

proptest! {
    /// Any title, lines, buttons and width draw the box the model gives, in
    /// any area: in one too short for the whole modal the title and the
    /// separator keep to the rows above the buttons or are left out (#1060).
    #[test]
    fn prop_modal_box_follows_the_model(
        title in "[A-Za-z ]{0,30}",
        lines in prop::collection::vec("[a-z ]{0,40}", 0..5),
        labels in prop::collection::vec("[A-Za-z]{1,8}", 0..4),
        sel in 0usize..4,
        width in 0u16..60,
        aw in 0u16..60,
        extra in 0u16..4,
        cut in 0u16..12,
    ) {
        let mut m = Modal::new()
            .title(title.clone())
            .buttons(labels.iter().map(ModalButton::new).collect())
            .width(width);
        for l in &lines {
            m = m.line(l.clone());
        }
        for _ in 0..sel {
            m.next_button();
        }
        m.show();
        let req = m.required_height();
        let ah = (req + 2 + extra).saturating_sub(cut);
        let b = draw(&m, aw, ah);
        let want = model(&title, &lines, &labels, width, aw, ah, req);
        for (y, line) in want.iter().enumerate() {
            prop_assert_eq!(&row(&b, y as u16), line, "row {}", y);
        }
        // The selected button is the highlighted one
        if !labels.is_empty() {
            let s = sel % labels.len();
            let by = (0..ah).find(|&y| row(&b, y).contains("[ "));
            if let Some(by) = by {
                let r = row(&b, by);
                let starts: Vec<usize> = r.match_indices("[ ").map(|(i, _)| r[..i].chars().count()).collect();
                prop_assert_eq!(starts.len(), labels.len());
                for (i, x) in starts.iter().enumerate() {
                    let cell = b.get(*x as u16, by).unwrap();
                    prop_assert_eq!(cell.bg.is_some(), i == s, "button {}", i);
                }
            }
        }
    }

    /// Left/h and Right/l/Tab move the selection and wrap; Enter and Space
    /// confirm it when there are buttons; Escape hides the modal, and a
    /// hidden modal ignores every key.
    #[test]
    fn prop_modal_keys_follow_the_model(
        n in 0usize..5,
        keys in prop::collection::vec(
            prop_oneof![
                Just(Key::Left),
                Just(Key::Right),
                Just(Key::Char('h')),
                Just(Key::Char('l')),
                Just(Key::Tab),
                Just(Key::Enter),
                Just(Key::Char(' ')),
                Just(Key::Escape),
                Just(Key::Up),
                Just(Key::Char('x')),
            ],
            0..30,
        ),
    ) {
        let mut m = Modal::new().buttons((0..n).map(|i| ModalButton::new(i.to_string())).collect());
        m.show();
        let mut sel = 0usize;
        let mut visible = true;
        for k in &keys {
            // Once Escape hides it, the modal handles no keys (#1060)
            let want = match k {
                _ if !visible => None,
                Key::Enter | Key::Char(' ') if n > 0 => Some(sel),
                Key::Left | Key::Char('h') if n > 0 => {
                    sel = (sel + n - 1) % n;
                    None
                }
                Key::Right | Key::Char('l') | Key::Tab if n > 0 => {
                    sel = (sel + 1) % n;
                    None
                }
                Key::Escape => {
                    visible = false;
                    None
                }
                _ => None,
            };
            prop_assert_eq!(m.handle_key(k), want, "{:?}", k);
            prop_assert_eq!(m.selected_button(), sel);
            prop_assert_eq!(m.is_visible(), visible);
        }
    }
}

#[test]
fn test_modal_required_height() {
    // Borders, the title and its separator, the lines, a row of padding and
    // the button row when there are buttons; a set height wins
    let m = Modal::new().title("T").content("a\nb");
    assert_eq!(m.required_height(), 7);
    assert_eq!(m.ok().required_height(), 8);
    assert_eq!(Modal::new().title("T").height(3).ok().required_height(), 3);
    // Without a title there are no rows for it (#1060): border, two lines,
    // padding, buttons, border
    let m = Modal::new().content("a\nb");
    assert_eq!(m.required_height(), 5);
    assert_eq!(m.ok().required_height(), 6);
}

#[test]
fn test_untitled_modal_keeps_one_row_of_padding() {
    // The content starts under the top border and one empty row separates
    // it from the buttons, as with a title (#1060)
    let mut m = Modal::new().content("a\nb").ok().width(20);
    m.show();
    let b = draw(&m, 24, 8);
    let rows: Vec<String> = (0..8).map(|y| row(&b, y)).collect();
    assert_eq!(
        rows,
        [
            " ".repeat(24),
            "  ┌──────────────────┐  ".to_string(),
            "  │ a                │  ".to_string(),
            "  │ b                │  ".to_string(),
            "  │                  │  ".to_string(),
            "  │      [ OK ]      │  ".to_string(),
            "  └──────────────────┘  ".to_string(),
            " ".repeat(24),
        ]
    );
}

#[test]
fn test_hidden_modal_does_not_confirm() {
    // Keys sent to a modal before it is shown, or after Escape hid it, do not
    // confirm or move the selection the user cannot see (#1060)
    let mut m = Modal::confirm("Delete", "Sure?");
    assert!(!m.is_visible());
    for k in [
        Key::Enter,
        Key::Char(' '),
        Key::Right,
        Key::Tab,
        Key::Char('l'),
    ] {
        assert_eq!(m.handle_key(&k), None, "{k:?}");
    }
    assert_eq!(m.selected_button(), 0);
    m.show();
    m.handle_key(&Key::Right);
    m.handle_key(&Key::Escape);
    assert_eq!(m.handle_key(&Key::Enter), None);
    assert_eq!(m.selected_button(), 1);
}

#[test]
fn test_modal_content_stops_above_the_button_row() {
    // A 5-row box has room for two lines: the rows above the padding row
    let mut m = Modal::new().content("1\n2\n3\n4\n5").width(10).height(5);
    m.show();
    // Centered in 14 x 7: rows 1 to 5 from x = 2
    let b = draw(&m, 14, 7);
    let rows: Vec<String> = (1..6).map(|y| row(&b, y)).collect();
    assert_eq!(
        rows,
        [
            "  ┌────────┐  ",
            "  │ 1      │  ",
            "  │ 2      │  ",
            "  │        │  ",
            "  └────────┘  ",
        ]
    );
}

#[test]
fn test_short_modal_keeps_its_title_inside_the_box() {
    // In a 4-row area the box is 2 rows (rows 1 and 2), with no row inside
    // the border for the title
    let mut m = Modal::new().title("Test").content("Body").ok();
    m.show();
    let b = draw(&m, 80, 4);
    assert_eq!(row(&b, 2).trim(), format!("└{}┘", "─".repeat(38))); // was "└─Test───…┘"
    assert_eq!(row(&b, 3).trim(), ""); // was "├───…┤"
}

#[test]
fn test_modal_three_rows_tall_has_its_title_but_no_separator() {
    // A 5-row area gives a 3-row box: the title's row is the only one
    // inside the border, and the row under it is the bottom border
    let mut m = Modal::new().title("Test").content("Body");
    m.show();
    let b = draw(&m, 80, 5);
    assert_eq!(row(&b, 2).trim(), format!("│ Test{}│", " ".repeat(33)));
    assert_eq!(row(&b, 3).trim(), format!("└{}┘", "─".repeat(38))); // was "├───…┤"
}

#[test]
fn test_modal_with_no_box_draws_no_title() {
    // A 3-row area leaves one row for the box: no box, so no title
    let mut m = Modal::new().title("Test").content("Body");
    m.show();
    let b = draw(&m, 80, 3);
    for y in 0..3 {
        assert_eq!(row(&b, y).trim(), "", "row {y}"); // was "Test" on row 1
    }
}

#[test]
fn test_short_modal_keeps_the_buttons_row_for_the_buttons() {
    // A 6-row area gives a 4-row box: the title's row, then the buttons'
    // row, where the separator used to run between the buttons
    let mut m = Modal::new().title("Test").content("Body").ok();
    m.show();
    let b = draw(&m, 80, 6);
    assert_eq!(row(&b, 2).trim(), format!("│ Test{}│", " ".repeat(33)));
    assert_eq!(
        row(&b, 3).trim(),
        format!("│{}[ OK ]{}│", " ".repeat(16), " ".repeat(16))
    ); // was "├───…[ OK ]…┤"

    // A 5-row area gives a 3-row box: its one row inside is the buttons'
    let b = draw(&m, 80, 5);
    assert_eq!(
        row(&b, 2).trim(),
        format!("│{}[ OK ]{}│", " ".repeat(16), " ".repeat(16))
    ); // was "│ Test   [ OK ]…│"
}
