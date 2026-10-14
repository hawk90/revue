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

    /// The bar is " label │ label │ ...", cut at the width; the active tab's
    /// label is on the highlight's background wherever it is drawn.
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
        let full = format!(" {} ", labels.join(" │ "));
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
        // The active label's first cell, when it is on screen
        let start: usize = labels[..sel].iter().map(|l| display_width(l) + 3).sum::<usize>() + 1;
        if !labels[sel].is_empty() && start + display_width(&labels[sel]) <= width as usize {
            prop_assert_eq!(buffer.get(start as u16, 0).unwrap().bg, Some(Color::BLUE));
        }
    }
}
