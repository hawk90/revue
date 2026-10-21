//! Tabs placement, a model of the selection under keys, and the bar's
//! layout for any labels

use proptest::prelude::*;
use revue::event::Key;
use revue::style::Color;
use revue::utils::unicode::display_width;
use revue::widget::Tabs;

use super::{draw, row};
use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

#[test]
fn test_tabs_zero_areas_and_offset() {
    let t = Tabs::new()
        .tabs(vec!["One", "두 번째", "Three"])
        .selected(1)
        .fg(Color::RED);
    assert_zero_areas_draw_nothing(&t);
    assert_same_when_offset(t, 24, 1);
}

fn key() -> impl Strategy<Value = Key> {
    prop_oneof![
        Just(Key::Left),
        Just(Key::Right),
        Just(Key::Char('h')),
        Just(Key::Char('l')),
        Just(Key::Home),
        Just(Key::End),
        (0u32..10).prop_map(|d| Key::Char(char::from_digit(d, 10).unwrap())),
        Just(Key::Tab),
        Just(Key::Enter),
    ]
}

proptest! {
    /// Left/Right (and h/l) step and wrap around, Home and End go to the
    /// ends, 1-9 pick a tab that exists; a key is reported as used when the
    /// selection moved.
    #[test]
    fn prop_tabs_keys_follow_the_model(n in 0usize..12, keys in prop::collection::vec(key(), 0..40)) {
        let labels: Vec<String> = (0..n).map(|i| format!("t{i}")).collect();
        let mut t = Tabs::new().tabs(labels);
        let mut sel = 0usize;
        for k in &keys {
            let before = sel;
            match k {
                Key::Right | Key::Char('l') if n > 0 => sel = (sel + 1) % n,
                Key::Left | Key::Char('h') if n > 0 => sel = (sel + n - 1) % n,
                Key::Home => sel = 0,
                Key::End => sel = n.saturating_sub(1),
                Key::Char(c @ '1'..='9') => {
                    let i = (*c as usize) - ('1' as usize);
                    if i < n {
                        sel = i;
                    }
                }
                _ => {}
            }
            prop_assert_eq!(t.handle_key(k), sel != before, "{:?}", k);
            prop_assert_eq!(t.selected_index(), sel, "after {:?}", k);
        }
    }

    /// The bar is " label │ label │ ...", cut at the width, scrolled to show
    /// the active tab (#973): one draw scrolls from the left just far enough
    /// to show it and starts at a whole tab. The active label is on the
    /// highlight's background.
    #[test]
    fn prop_tabs_bar_layout(
        labels in prop::collection::vec("[a-z가]{0,6}", 1..6),
        sel in 0usize..6,
        width in 3u16..40,
    ) {
        let sel = sel % labels.len();
        // With an fg every tab is drawn with continuation cells; without
        // one, a wide glyph's second cell is left as a space (#974)
        let t = Tabs::new().tabs(labels.clone()).selected(sel).fg(Color::WHITE);
        let buffer = draw(&t, width, 1);
        // Each tab's columns: " label " and a divider after it
        let mut spans = Vec::new();
        let mut x = 0;
        for l in &labels {
            spans.push((x, x + display_width(l) + 2));
            x += display_width(l) + 3;
        }
        let (start, end) = spans[sel];
        let offset = if end - start > width as usize { start } else { end.saturating_sub(width as usize) };
        let first = spans.iter().position(|&(s, _)| s >= offset).unwrap();
        let full = format!(" {} ", labels[first..].join(" │ "));
        let mut expected = String::new();
        let mut cols = 0;
        for c in full.chars() {
            let w = display_width(&c.to_string());
            if cols + w > width as usize {
                break;
            }
            expected.push(c);
            cols += w;
        }
        prop_assert_eq!(row(&buffer, 0), expected.trim_end());
        // The active label's first cell, drawn whenever its tab fits
        let at = start - spans[first].0 + 1;
        if !labels[sel].is_empty() && at + display_width(&labels[sel]) <= width as usize {
            prop_assert_eq!(buffer.get(at as u16, 0).unwrap().bg, Some(Color::BLUE));
        }
        if end - spans[first].0 <= width as usize && !labels[sel].is_empty() {
            prop_assert!(row(&buffer, 0).contains(labels[sel].as_str()));
        }
    }
}

#[test]
fn test_tabs_inactive_tabs_take_the_stylesheet_colors() {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::{Style, VisualStyle};
    use revue::widget::traits::{RenderContext, View};
    let style = Style {
        visual: VisualStyle {
            color: Color::MAGENTA,
            background: Color::YELLOW,
            ..VisualStyle::default()
        },
        ..Default::default()
    };
    let t = Tabs::new().tabs(vec!["A", "B"]);
    let mut buffer = Buffer::new(10, 1);
    let mut ctx = RenderContext::with_style(&mut buffer, Rect::new(0, 0, 10, 1), &style);
    t.render(&mut ctx);
    let b = buffer.get(5, 0).unwrap();
    assert_eq!(
        (b.symbol, b.fg, b.bg),
        ('B', Some(Color::MAGENTA), Some(Color::YELLOW))
    );
    // The active tab keeps its highlight
    assert_eq!(buffer.get(1, 0).unwrap().bg, Some(Color::BLUE));
}

#[test]
fn test_tabs_divider_takes_the_fg() {
    let buffer = draw(&Tabs::new().tabs(vec!["A", "B"]).fg(Color::RED), 10, 1);
    let d = buffer.get(3, 0).unwrap();
    assert_eq!((d.symbol, d.fg), ('│', Some(Color::RED)));
}

#[test]
fn test_tabs_wide_label_that_does_not_fit_is_cut_before_it() {
    // " a │ 한": the wide glyph needs columns 5 and 6; in 6 columns it is left out
    let buffer = draw(&Tabs::new().tabs(vec!["a", "한"]), 6, 1);
    assert_eq!(row(&buffer, 0), " a │");
}

#[test]
fn test_tabs_min_width_above_max_width() {
    // The minimum wins, as it does for the other layout widgets
    let t = Tabs::new()
        .tabs(vec!["One", "Two"])
        .min_width(12)
        .max_width(5);
    assert_eq!(row(&draw(&t, 20, 1), 0), " One │ Two");
}

/// The bar used to draw from the first tab, so a selected tab past the
/// width was not on screen (#973).
#[test]
fn tabs_keep_the_selected_tab_in_view() {
    use revue::event::Key;
    let mut t = Tabs::new().tabs(vec!["Alpha", "Beta", "Gamma", "Delta", "Epsilon"]);
    t.handle_key(&Key::End);
    let shown = row(&draw(&t, 20, 1), 0);
    assert!(shown.contains("Epsilon"), "{shown:?}");
    // Moving left inside the shown tabs keeps the bar where it is
    t.handle_key(&Key::Left);
    assert_eq!(row(&draw(&t, 20, 1), 0), shown);
}
