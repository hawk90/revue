//! ThemePicker on the screen: placement, empty lists, edge sizes, and a
//! model of the open dropdown

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};
use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{get_theme, register_theme, set_theme_by_id, use_theme, Theme};
use revue::widget::traits::RenderContext;
use revue::widget::{ThemePicker, View};
use serial_test::serial;

const KNOWN: [&str; 8] = [
    "dark",
    "light",
    "high_contrast",
    "dracula",
    "nord",
    "monokai",
    "solarized_dark",
    "solarized_light",
];

fn render(picker: &ThemePicker, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    picker.render(&mut ctx);
    buffer
}

fn rows(buffer: &Buffer, width: u16, height: u16) -> Vec<String> {
    (0..height)
        .map(|y| {
            (0..width)
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

fn opened(picker: ThemePicker) -> ThemePicker {
    let mut picker = picker;
    picker.open();
    picker
}

fn pickers() -> Vec<ThemePicker> {
    vec![
        ThemePicker::new().themes(["dark", "nord"]),
        opened(ThemePicker::new().themes(["dark", "nord", "dracula"])),
        opened(
            ThemePicker::new()
                .themes(["dark", "nord"])
                .show_preview(false),
        ),
        ThemePicker::new().compact(true),
        opened(ThemePicker::new().themes(["dark", "nord"]).compact(true)),
    ]
}

#[test]
fn zero_areas_draw_nothing() {
    for picker in pickers() {
        assert_zero_areas_draw_nothing(&picker);
    }
}

#[test]
fn same_when_offset() {
    for picker in pickers() {
        assert_same_when_offset(picker, 30, 8);
    }
}

#[test]
fn open_with_no_themes_draws_the_header_only() {
    let picker = opened(ThemePicker::new().themes::<Vec<&str>, _>(vec![]));
    let buffer = render(&picker, 30, 6);
    let rows = rows(&buffer, 30, 6);
    assert!(
        rows[0].starts_with("Theme: ") && rows[0].contains('▲'),
        "{rows:?}"
    );
    for row in &rows[1..] {
        assert_eq!(row.trim(), "", "{rows:?}");
    }

    // Compact draws the current swatch and no list
    let compact = opened(
        ThemePicker::new()
            .themes::<Vec<&str>, _>(vec![])
            .compact(true),
    );
    let buffer = render(&compact, 30, 6);
    assert!(buffer.get(0, 0).unwrap().bg.is_some());
    assert!(rows_after_first_blank(&buffer, 30, 6));
}

fn rows_after_first_blank(buffer: &Buffer, width: u16, height: u16) -> bool {
    (1..height).all(|y| (0..width).all(|x| buffer.get(x, y).unwrap().symbol == ' '))
}

#[test]
fn open_in_two_rows_draws_the_top_border_only() {
    let picker = opened(ThemePicker::new().themes(["dark", "nord"]));
    let rows = rows(&render(&picker, 20, 2), 20, 2);
    assert_eq!(rows[1], format!("┌{}┐", "─".repeat(18)));
}

#[test]
fn selected_row_uses_the_selection_colors() {
    let current = use_theme().get();
    let mut picker = opened(ThemePicker::new().themes(["dark", "nord"]));
    picker.select_next();
    let buffer = render(&picker, 30, 6);

    // Row 2 is "dark", row 3 the selected "nord"; column 7 is the space
    // after the swatch, column 8 the name's first letter
    let plain = buffer.get(8, 2).unwrap();
    assert_eq!(
        (plain.fg, plain.bg),
        (Some(current.colors.text), Some(current.colors.surface))
    );
    let selected = buffer.get(8, 3).unwrap();
    assert_eq!(
        (selected.fg, selected.bg),
        (
            Some(current.colors.selection_text),
            Some(current.colors.selection)
        )
    );
    // The marker is drawn in the listed theme's primary color
    let nord = get_theme("nord").unwrap();
    assert_eq!(buffer.get(1, 3).unwrap().symbol, '▶');
    assert_eq!(buffer.get(1, 3).unwrap().fg, Some(nord.palette.primary));
    // The borders are in the current theme's border color
    assert_eq!(buffer.get(0, 3).unwrap().fg, Some(current.colors.border));
    assert_eq!(buffer.get(29, 3).unwrap().fg, Some(current.colors.border));
}

#[test]
fn compact_open_lists_one_swatch_per_theme() {
    let mut picker = opened(
        ThemePicker::new()
            .themes(["dark", "nord", "dracula"])
            .compact(true),
    );
    picker.select_next();
    let buffer = render(&picker, 20, 6);
    for (y, id) in [(1, "dark"), (2, "nord"), (3, "dracula")] {
        let theme = get_theme(id).unwrap();
        let marker = if id == "nord" { '>' } else { ' ' };
        assert_eq!(buffer.get(0, y).unwrap().symbol, marker, "row {y}");
        assert_eq!(buffer.get(0, y).unwrap().fg, Some(theme.palette.primary));
        assert_eq!(buffer.get(1, y).unwrap().bg, Some(theme.colors.background));
        assert_eq!(buffer.get(4, y).unwrap().bg, Some(theme.palette.error));
        assert_eq!(buffer.get(5, y).unwrap().bg, None);
    }
    // The list stops at the area's last row
    let buffer = render(&picker, 20, 3);
    assert_eq!(buffer.get(0, 2).unwrap().symbol, '>');
}

#[test]
fn the_arrow_follows_the_open_state() {
    let mut picker = ThemePicker::new().themes(["dark"]);
    // "Theme: Dark", a space, the 4-cell swatch, then " ▼"
    let arrow = 7 + use_theme().get().name.chars().count() as u16 + 5 + 1;
    assert_eq!(render(&picker, 35, 1).get(arrow, 0).unwrap().symbol, '▼');
    picker.open();
    assert_eq!(render(&picker, 35, 3).get(arrow, 0).unwrap().symbol, '▲');
}

#[test]
fn the_swatch_shows_four_theme_colors_in_order() {
    let current = use_theme().get();
    let buffer = render(&ThemePicker::new().compact(true), 20, 1);
    let colors: Vec<_> = (0..4).map(|x| buffer.get(x, 0).unwrap().bg).collect();
    assert_eq!(
        colors,
        [
            Some(current.colors.background),
            Some(current.palette.primary),
            Some(current.palette.success),
            Some(current.palette.error),
        ]
    );
}

#[test]
#[serial]
fn a_long_name_is_cut_before_the_swatch() {
    register_theme(
        "audit-long-name",
        Theme::custom("Abcdefghijklmnopqrstuvwxyz Abcdefghij").build(),
    );
    set_theme_by_id("audit-long-name");
    let buffer = render(&ThemePicker::new().themes(["audit-long-name"]), 40, 1);
    let row = rows(&buffer, 40, 1).remove(0);
    set_theme_by_id("dark");

    // The name is cut, and no letter reaches the six columns before the
    // 35-column width (the swatch and arrow need more: #1097)
    assert!(row.starts_with("Theme: Abcdefghijklmnop"), "{row:?}");
    for (x, ch) in row.chars().enumerate().skip(29) {
        assert!(
            !ch.is_alphabetic(),
            "name letter {ch:?} at column {x}: {row:?}"
        );
    }
}

proptest! {
    /// The open dropdown, against a plain model: a top border, one row per
    /// theme while a row is left for the bottom border, the marker on the
    /// selected row, each name cut at the right border. The marker is checked
    /// only when the selected row fits: the list does not scroll (#973).
    #[test]
    fn open_dropdown_matches_the_model(
        picks in proptest::collection::vec(0..KNOWN.len(), 0..8),
        moves in 0usize..10,
        width in 10u16..50,
        height in 1u16..14,
        preview in any::<bool>(),
    ) {
        // Unknown ids are left out: an unknown selected id shows no marker (#1097)
        let ids: Vec<&str> = picks.iter().map(|&i| KNOWN[i]).collect();
        let mut picker = ThemePicker::new().themes(ids.clone()).show_preview(preview);
        for _ in 0..moves {
            picker.select_next();
        }
        picker.open();
        let buffer = render(&picker, width, height);
        let rows = rows(&buffer, width, height);
        let box_width = width.min(35) as usize;

        prop_assert!(rows[0].starts_with(&"Theme: "[..7.min(width as usize)]));
        if ids.is_empty() || height < 2 {
            for row in &rows[1..] {
                prop_assert_eq!(row.trim(), "");
            }
            return Ok(());
        }
        let top = format!("┌{}┐", "─".repeat(box_width - 2));
        prop_assert!(rows[1].starts_with(&top), "{:?}", rows);
        if height == 2 {
            return Ok(());
        }

        let shown = ids.len().min(height as usize - 3);
        let selected = moves.min(ids.len() - 1);
        for (i, id) in ids.iter().take(shown).enumerate() {
            let row: Vec<char> = rows[2 + i].chars().collect();
            prop_assert_eq!(row[0], '│');
            prop_assert_eq!(row[box_width - 1], '│');
            prop_assert_eq!(row[1], if i == selected { '▶' } else { ' ' });
            let name_x = if preview { 7 } else { 3 };
            let name: String = get_theme(id).unwrap().name.chars().take(box_width - 1 - name_x).collect();
            let drawn: String = row[name_x..name_x + name.chars().count()].iter().collect();
            prop_assert_eq!(drawn, name);
        }
        let bottom = format!("└{}┘", "─".repeat(box_width - 2));
        prop_assert!(rows[2 + shown].starts_with(&bottom), "{:?}", rows);
        for row in &rows[3 + shown..] {
            prop_assert_eq!(row.trim(), "");
        }
    }
}
