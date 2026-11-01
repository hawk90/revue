//! MaskedInput on the screen: placement, the field, the cursor, the colors,
//! and proptests of the masked text and of the field against plain models

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::{DARK_GRAY, DISABLED_FG, PLACEHOLDER_FG};
use revue::widget::traits::RenderContext;
use revue::widget::{MaskStyle, MaskedInput, View};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

pub(super) fn screen(view: &impl View, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    view.render(&mut ctx);
    buffer
}

/// Row `y` without the continuation cells of wide glyphs and without
/// trailing blanks.
pub(super) fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap())
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

/// Columns of row `y` drawn as the cursor: black on white.
pub(super) fn cursor_cells(buffer: &Buffer, y: u16) -> Vec<u16> {
    (0..buffer.width())
        .filter(|&x| {
            let c = buffer.get(x, y).unwrap();
            c.bg == Some(Color::WHITE) && c.fg == Some(Color::BLACK)
        })
        .collect()
}

pub(super) fn fg(buffer: &Buffer, x: u16, y: u16) -> Option<Color> {
    buffer.get(x, y).unwrap().fg
}

// =========================================================================
// Placement
// =========================================================================

#[test]
fn test_masked_input_zero_areas_draw_nothing() {
    let mut input = MaskedInput::password()
        .label("Password")
        .value("Abc123!")
        .focused(true)
        .allow_reveal(true)
        .min_length(10);
    input.validate();
    assert_zero_areas_draw_nothing(&input);
}

#[test]
fn test_masked_input_same_when_offset() {
    let mut input = MaskedInput::password()
        .label("비밀번호")
        .value("Abc123!")
        .focused(true)
        .min_length(10)
        .width(12);
    input.validate();
    assert_same_when_offset(input, 24, 4);
    assert_same_when_offset(MaskedInput::new().placeholder("PIN").width(6), 10, 1);
}

// =========================================================================
// The field
// =========================================================================

#[test]
fn test_masked_input_field_is_width_columns_between_brackets() {
    for (width, value, expected) in [
        (None, "abc", "[●●●                 ]"),
        (Some(4), "abc", "[●●● ]"),
        (Some(4), "abcdef", "[●●●●]"),
        (Some(1), "", "[ ]"),
        (Some(0), "abc", "[]"),
    ] {
        let mut input = MaskedInput::new().value(value);
        if let Some(w) = width {
            input = input.width(w);
        }
        assert_eq!(
            row(&screen(&input, 30, 1), 0),
            expected.trim_end(),
            "{width:?} {value:?}"
        );
    }
}

#[test]
fn test_masked_input_wide_glyphs_fill_two_columns_of_the_field() {
    // Revealed Hangul: 4 glyphs take 8 of the 10 columns.
    let mut input = MaskedInput::new()
        .value("비밀번호")
        .allow_reveal(true)
        .width(10);
    input.toggle_reveal();
    let buffer = screen(&input, 30, 1);
    assert_eq!(row(&buffer, 0), "[비밀번호   👁]");
    // Columns 9 and 10 pad the field; 11 separates the indicator.
    for x in 9..12 {
        assert_eq!(buffer.get(x, 0).unwrap().symbol, ' ', "x={x}");
    }
    // A glyph cut at the edge is left out whole, and its column is a blank
    // so the field keeps its width
    let mut input = MaskedInput::new()
        .value("비밀번호")
        .allow_reveal(true)
        .width(5);
    input.toggle_reveal();
    let buffer = screen(&input, 30, 1);
    assert_eq!(row(&buffer, 0), "[비밀  👁]");
    assert_eq!(buffer.get(5, 0).unwrap().symbol, ' ');
}

/// A wide glyph cut by the field's right edge was dropped with the cursor
/// on it: it is drawn as a blank in its style
#[test]
fn test_masked_input_wide_glyph_cut_by_the_edge_keeps_its_cursor() {
    let mut input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(1))
        .width(1)
        .focused(true)
        .value("한");
    input.set_cursor(0);
    let buffer = screen(&input, 40, 1);
    assert_eq!(row(&buffer, 0).trim_end(), "[ ]");
    assert_eq!(cursor_cells(&buffer, 0), vec![1]);

    // Not focused: a plain blank
    let input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(1))
        .width(3)
        .value("a한한");
    let buffer = screen(&input, 40, 1);
    // "●●한": the last glyph does not fit
    assert_eq!(row(&buffer, 0).trim_end(), "[●● ]");
    assert!(cursor_cells(&buffer, 0).is_empty());
}

/// A cursor on a wide glyph the right edge cuts scrolls the field one
/// column, so the whole glyph shows
#[test]
fn test_masked_input_cursor_on_a_cut_wide_glyph_scrolls_it_into_view() {
    let mut input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(1))
        .width(8)
        .focused(true)
        .value("aaaaaaa한");
    input.set_cursor(7);
    let buffer = screen(&input, 40, 1);
    assert_eq!(row(&buffer, 0).trim_end(), "[●●●●●●한]");
    assert_eq!(cursor_cells(&buffer, 0).first(), Some(&7));
    assert_eq!(buffer.get(8, 0).unwrap().bg, Some(Color::WHITE));
}

#[test]
fn test_masked_input_cursor_drawn_on_its_char_only_when_focused() {
    let mut input = MaskedInput::new().value("abcd").width(8);
    assert!(cursor_cells(&screen(&input, 20, 1), 0).is_empty());

    input = input.focused(true);
    // At the end: the blank after the text
    assert_eq!(cursor_cells(&screen(&input, 20, 1), 0), vec![5]);
    input.set_cursor(1);
    let buffer = screen(&input, 20, 1);
    assert_eq!(cursor_cells(&buffer, 0), vec![2]);
    assert_eq!(buffer.get(2, 0).unwrap().symbol, '●');
    // The field keeps its width.
    assert_eq!(buffer.get(9, 0).unwrap().symbol, ']');

    // Disabled: no cursor
    let input = input.disabled(true);
    assert!(cursor_cells(&screen(&input, 20, 1), 0).is_empty());
}

#[test]
fn test_masked_input_bracket_colors_by_state() {
    let bracket = |input: &MaskedInput| fg(&screen(input, 30, 3), 0, 0);
    assert_eq!(bracket(&MaskedInput::new()), Some(PLACEHOLDER_FG));
    assert_eq!(
        bracket(&MaskedInput::new().focused(true)),
        Some(Color::CYAN)
    );
    assert_eq!(
        bracket(&MaskedInput::new().focused(true).disabled(true)),
        Some(DARK_GRAY)
    );

    let mut valid = MaskedInput::new().value("abc").focused(true);
    valid.validate();
    assert_eq!(bracket(&valid), Some(Color::GREEN));

    let mut invalid = MaskedInput::new().value("abc").min_length(5).focused(true);
    invalid.validate();
    assert_eq!(bracket(&invalid), Some(Color::RED));
    // Disabled wins over the validation state.
    let invalid = invalid.disabled(true);
    assert_eq!(bracket(&invalid), Some(DARK_GRAY));
}

#[test]
fn test_masked_input_text_colors_by_state() {
    // Placeholder gray, focused or not
    for focused in [false, true] {
        let buffer = screen(
            &MaskedInput::new().placeholder("PIN").focused(focused),
            30,
            1,
        );
        assert_eq!(fg(&buffer, 2, 0), Some(PLACEHOLDER_FG), "focused={focused}");
    }
    // Disabled gray over `fg`
    let buffer = screen(
        &MaskedInput::new()
            .value("ab")
            .fg(Color::CYAN)
            .disabled(true),
        30,
        1,
    );
    assert_eq!(fg(&buffer, 1, 0), Some(DISABLED_FG));
    // `fg` on the value
    let buffer = screen(&MaskedInput::new().value("ab").fg(Color::CYAN), 30, 1);
    assert_eq!(fg(&buffer, 1, 0), Some(Color::CYAN));
    assert_eq!(fg(&buffer, 3, 0), Some(Color::CYAN));
}

#[test]
fn test_masked_input_label_row_is_bold() {
    let buffer = screen(&MaskedInput::new().label("PIN").value("12"), 30, 3);
    assert_eq!(row(&buffer, 0), "PIN");
    assert!(buffer.get(0, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert!(row(&buffer, 1).starts_with("[●●"));
}

#[test]
fn test_masked_input_strength_bar_per_strength() {
    for (value, bar, label, color) in [
        ("abc", "█░░░░", "Very Weak", Color::RED),
        ("abcdefgh", "██░░░", "Weak", Color::rgb(255, 128, 0)),
        ("abcdefghijkl", "███░░", "Fair", Color::YELLOW),
        ("abcdefghijk1", "████░", "Strong", Color::rgb(128, 255, 0)),
        ("Abcdefghijk1!", "█████", "Very Strong", Color::GREEN),
    ] {
        let input = MaskedInput::new().show_strength(true).value(value);
        let buffer = screen(&input, 30, 3);
        assert_eq!(row(&buffer, 1), format!("{bar} {label}"), "{value}");
        let filled = bar.chars().filter(|&c| c == '█').count() as u16;
        assert_eq!(fg(&buffer, 0, 1), Some(color), "{value}");
        if filled < 5 {
            assert_eq!(fg(&buffer, filled, 1), Some(DARK_GRAY), "{value}");
        }
        assert_eq!(fg(&buffer, 6, 1), Some(color), "{value}");
    }
    // No bar while empty
    let buffer = screen(&MaskedInput::new().show_strength(true), 30, 3);
    assert_eq!(row(&buffer, 1), "");
}

#[test]
fn test_masked_input_invalid_message_below_the_field() {
    let mut input = MaskedInput::new()
        .label("PIN")
        .min_length(4)
        .value("12")
        .show_strength(true);
    input.validate();
    let buffer = screen(&input, 40, 4);
    assert_eq!(row(&buffer, 3), "Minimum 4 characters required");
    assert_eq!(fg(&buffer, 0, 3), Some(Color::RED));
    // Fixed: the message goes away
    input.set_value("1234");
    input.validate();
    assert_eq!(row(&screen(&input, 40, 4), 3), "");
}

// =========================================================================
// Proptest: masked_display against a plain model
// =========================================================================

fn mask_style() -> impl Strategy<Value = MaskStyle> {
    prop_oneof![
        Just(MaskStyle::Full),
        (0usize..6).prop_map(MaskStyle::ShowLast),
        (0usize..6).prop_map(MaskStyle::ShowFirst),
        Just(MaskStyle::Peek),
        Just(MaskStyle::Hidden),
    ]
}

fn value() -> impl Strategy<Value = String> {
    prop::collection::vec(
        prop::sample::select(vec!['a', 'Z', '1', '!', ' ', '한']),
        0..12,
    )
    .prop_map(|v| v.into_iter().collect())
}

fn model_display(
    value: &[char],
    style: MaskStyle,
    mask: char,
    cursor: usize,
    peek: usize,
    revealing: bool,
) -> String {
    let len = value.len();
    let masked = |_| mask;
    if revealing {
        return value.iter().collect();
    }
    match style {
        MaskStyle::Full => (0..len).map(masked).collect(),
        MaskStyle::ShowLast(n) => (0..len)
            .map(|i| if i + n >= len { value[i] } else { mask })
            .collect(),
        MaskStyle::ShowFirst(n) => (0..len)
            .map(|i| if i < n { value[i] } else { mask })
            .collect(),
        MaskStyle::Peek => (0..len)
            .map(|i| {
                if peek > 0 && cursor > 0 && i == cursor - 1 {
                    value[i]
                } else {
                    mask
                }
            })
            .collect(),
        MaskStyle::Hidden => String::new(),
    }
}

proptest! {
    #[test]
    fn prop_masked_display_matches_model(
        value in value(),
        style in mask_style(),
        mask in prop::sample::select(vec!['●', '*', '비']),
        cursor in 0usize..14,
        peek in 0usize..3,
        revealing in any::<bool>(),
    ) {
        let mut input = MaskedInput::new()
            .mask_char(mask)
            .mask_style(style)
            .allow_reveal(true)
            .value(value.clone());
        input.set_cursor(cursor);
        input.set_peek_countdown(peek);
        if revealing {
            input.toggle_reveal();
        }
        let chars: Vec<char> = value.chars().collect();
        let expected = model_display(
            &chars,
            style,
            mask,
            input.get_cursor(),
            peek,
            revealing,
        );
        prop_assert_eq!(input.get_cursor(), cursor.min(chars.len()));
        prop_assert_eq!(input.masked_display(), expected);
    }
}

// =========================================================================
// Proptest: the field on the screen against the masked text
// =========================================================================

/// `text` padded with blanks or cut to `cols` terminal columns, one entry per
/// char, never splitting a wide glyph.
fn fit(text: &str, cols: usize) -> Vec<char> {
    let mut out = Vec::new();
    let mut used = 0;
    for ch in text.chars() {
        let w = revue::utils::char_width(ch);
        if used + w > cols {
            break;
        }
        used += w;
        out.push(ch);
    }
    out.extend(std::iter::repeat_n(' ', cols.saturating_sub(used)));
    out
}

proptest! {
    // About one case in seven is skipped by the assumptions below.
    #![proptest_config(ProptestConfig {
        max_global_rejects: 100_000,
        ..ProptestConfig::default()
    })]

    #[test]
    fn prop_field_shows_masked_text_and_cursor(
        value in value(),
        style in mask_style(),
        width in 1u16..16,
        cursor in 0usize..14,
        focused in any::<bool>(),
    ) {
        let mut input = MaskedInput::new()
            .mask_style(style)
            .width(width)
            .focused(focused)
            .value(value);
        input.set_cursor(cursor);
        let shown = fit(&input.masked_display(), width as usize);
        let text_cols: usize = shown.iter().map(|&c| revue::utils::char_width(c)).sum();
        // Hidden shows nothing, and its cursor stays at the start so it
        // does not give the length away
        let hidden = input.masked_display().is_empty();
        let cursor_at = if hidden { 0 } else { input.get_cursor() };
        // A cursor past the shown text, or on a wide glyph the right edge
        // cuts, scrolls the field so the whole glyph shows (tested on its
        // own); keep it inside, where the field shows from the start.
        let masked: Vec<char> = input.masked_display().chars().collect();
        let cursor_end: usize = masked[..cursor_at.min(masked.len())]
            .iter()
            .map(|&c| revue::utils::char_width(c))
            .sum::<usize>()
            + masked.get(cursor_at).map_or(1, |&c| revue::utils::char_width(c).max(1));
        prop_assume!(!focused || (cursor_at < shown.len() && cursor_end <= width as usize));

        let buffer = screen(&input, 40, 1);
        prop_assert_eq!(buffer.get(0, 0).unwrap().symbol, '[');
        let mut x = 1u16;
        for &ch in &shown {
            prop_assert_eq!(buffer.get(x, 0).unwrap().symbol, ch, "x={}", x);
            x += revue::utils::char_width(ch) as u16;
        }
        prop_assert_eq!(x as usize, 1 + text_cols);
        prop_assert_eq!(buffer.get(x, 0).unwrap().symbol, ']');

        let cursors = cursor_cells(&buffer, 0);
        if focused {
            let col = 1 + shown[..cursor_at]
                .iter()
                .map(|&c| revue::utils::char_width(c) as u16)
                .sum::<u16>();
            prop_assert_eq!(cursors.first().copied(), Some(col));
        } else {
            prop_assert!(cursors.is_empty());
        }
    }
}

// Long values scroll to the cursor (#985)

#[test]
fn test_masked_input_long_value_scrolls_to_the_cursor() {
    // The failing case from #985: the shown end of ShowLast is in view
    let mut input = MaskedInput::new()
        .mask_style(MaskStyle::ShowLast(2))
        .value("abcdefghij")
        .width(6)
        .focused(true);
    let buffer = screen(&input, 20, 1);
    assert_eq!(row(&buffer, 0), "[●●●ij ]");
    assert_eq!(cursor_cells(&buffer, 0), vec![6]);
    assert_eq!(buffer.get(7, 0).unwrap().symbol, ']');
    // Two steps left stay inside the view: it does not move
    input.move_left();
    input.move_left();
    let buffer = screen(&input, 20, 1);
    assert_eq!(row(&buffer, 0), "[●●●ij ]");
    assert_eq!(cursor_cells(&buffer, 0), vec![4]);
    // Home: from the start; a step right inside the view keeps it there
    input.move_start();
    assert_eq!(row(&screen(&input, 20, 1), 0), "[●●●●●●]");
    input.move_right();
    let buffer = screen(&input, 20, 1);
    assert_eq!(row(&buffer, 0), "[●●●●●●]");
    assert_eq!(cursor_cells(&buffer, 0), vec![2]);
}

#[test]
fn test_masked_input_hidden_cursor_does_not_show_the_length() {
    // The same screen whatever the length, the cursor at the start
    for value in ["", "s", "secret", "a much longer secret than the field"] {
        let input = MaskedInput::new()
            .mask_style(MaskStyle::Hidden)
            .value(value)
            .width(10)
            .focused(true);
        let buffer = screen(&input, 20, 1);
        assert_eq!(row(&buffer, 0), "[          ]", "{value:?}");
        assert_eq!(cursor_cells(&buffer, 0), vec![1], "{value:?}");
    }
}

#[test]
fn test_masked_input_selection_is_drawn() {
    let mut input = MaskedInput::new().value("abcd").width(6).focused(true);
    input.handle_key_event(&revue::event::KeyEvent {
        shift: true,
        ..revue::event::KeyEvent::new(revue::event::Key::Home)
    });
    let buffer = screen(&input, 20, 1);
    // The cursor on the first char, the other three selected
    assert_eq!(cursor_cells(&buffer, 0), vec![1]);
    for x in 2..5 {
        assert_eq!(
            buffer.get(x, 0).unwrap().bg,
            Some(revue::style::use_theme().get().colors.selection),
            "{x}"
        );
    }
    assert_ne!(
        buffer.get(5, 0).unwrap().bg,
        Some(revue::style::use_theme().get().colors.selection)
    );
}

/// The cursor is drawn on the mask char of its character: after a letter
/// with an accent it is on the second bullet, not the third
#[test]
fn test_masked_input_cursor_on_the_mask_of_its_character() {
    let mut input = MaskedInput::new()
        .mask_style(MaskStyle::Full)
        .focused(true)
        .value("e\u{301}x");
    input.handle_key(&revue::event::Key::Left);
    let buffer = screen(&input, 40, 1);
    assert!(row(&buffer, 0).starts_with("[●● "));
    assert_eq!(cursor_cells(&buffer, 0), vec![2]);
}
