//! A `CharWidthTable` answers with the widths it was configured with.

use revue::text::CharWidthTable;

#[test]
fn defaults_follow_unicode() {
    let table = CharWidthTable::new();
    assert_eq!(table.width('a'), 1);
    assert_eq!(table.width('한'), 2);
    assert_eq!(table.width('日'), 2);
    assert_eq!(table.width('😀'), 2);
    assert_eq!(table.width('\u{e0b0}'), 1, "powerline arrow (Nerd Font)");
}

#[test]
fn text_presentation_characters_are_not_emoji_width() {
    // These carry the Unicode Emoji property but render as text by default
    let table = CharWidthTable::new().with_emoji(2);
    for ch in ['1', '#', '*', '©', '®'] {
        assert_eq!(table.width(ch), 1, "{ch:?} is one column");
    }
}

#[test]
fn with_cjk_sets_the_width_of_wide_characters() {
    let table = CharWidthTable::new().with_cjk(1);
    assert_eq!(table.width('한'), 1);
    assert_eq!(table.width('日'), 1);
    assert_eq!(table.width('Ａ'), 1, "fullwidth Latin");
    assert_eq!(table.width('a'), 1);
    assert_eq!(table.width('😀'), 2, "emoji keep the emoji width");
}

#[test]
fn with_nerd_font_sets_the_width_of_private_use_icons() {
    let table = CharWidthTable::new().with_nerd_font(2);
    assert_eq!(table.width('\u{e0b0}'), 2, "powerline arrow");
    assert_eq!(table.width('\u{f115}'), 2, "Font Awesome folder");
    assert_eq!(table.width('\u{f0001}'), 2, "Material Design (plane 15)");
    assert_eq!(table.width('a'), 1);
}

#[test]
fn an_override_beats_every_class() {
    let mut table = CharWidthTable::new().with_cjk(1).with_nerd_font(2);
    table.set_override('한', 3);
    table.set_override('\u{e0b0}', 1);
    assert_eq!(table.width('한'), 3);
    assert_eq!(table.width('\u{e0b0}'), 1);
}
