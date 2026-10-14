//! Avatar drawing against a plain model of its cells, the name-derived
//! background against the HSL formula, placement, and the stylesheet color.

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Cell, Modifier};
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{avatar, Avatar, AvatarShape, AvatarSize};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const SIZES: [AvatarSize; 3] = [AvatarSize::Small, AvatarSize::Medium, AvatarSize::Large];
const SHAPES: [AvatarShape; 3] = [
    AvatarShape::Circle,
    AvatarShape::Square,
    AvatarShape::Rounded,
];

fn draw(a: &Avatar, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    a.render(&mut ctx);
    buffer
}

/// The HSL color (saturation 0.6, lightness 0.4) of `hue` degrees, by the
/// textbook formula in f64.
fn hsl(hue: u32) -> (f64, f64, f64) {
    let (s, l) = (0.6, 0.4);
    let c = (1.0 - (2.0 * l - 1.0f64).abs()) * s;
    let h = f64::from(hue) / 60.0;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let (r, g, b) = match hue / 60 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let m = l - c / 2.0;
    ((r + m) * 255.0, (g + m) * 255.0, (b + m) * 255.0)
}

/// Whether `got` is the HSL color of the hue the name's bytes sum to,
/// allowing a step per channel: the widget truncates f32 products.
fn is_name_color(name: &str, got: Color) -> bool {
    let hue = name.bytes().map(u32::from).sum::<u32>() % 360;
    let (r, g, b) = hsl(hue);
    let near = |want: f64, got: u8| (want - f64::from(got)).abs() < 1.5;
    near(r, got.r) && near(g, got.g) && near(b, got.b)
}

/// One of every size, shape and status setting.
fn every_avatar() -> Vec<Avatar> {
    let mut all = Vec::new();
    for size in SIZES {
        for shape in SHAPES {
            for status in [false, true] {
                let a = avatar("Jane Doe").size(size).shape(shape);
                all.push(if status { a.online() } else { a });
            }
        }
    }
    all
}

#[test]
fn zero_areas_draw_nothing() {
    for a in every_avatar() {
        assert_zero_areas_draw_nothing(&a);
    }
}

#[test]
fn same_when_offset() {
    for a in every_avatar() {
        assert_same_when_offset(a, 8, 3);
    }
}

#[test]
fn an_empty_name_shows_a_question_mark_only_at_small_size() {
    // Small falls back to '?'; medium and large keep their outline with
    // blank initials
    assert_eq!(
        draw(&avatar("").small(), 3, 1).get(0, 0).unwrap().symbol,
        '?'
    );
    let row = |b: &Buffer, y: u16| {
        (0..5)
            .map(|x| b.get(x, y).unwrap().symbol)
            .collect::<String>()
    };
    assert_eq!(row(&draw(&avatar("   "), 5, 1), 0), "◖  ◗ ");
    assert_eq!(row(&draw(&avatar("").large(), 5, 3), 1), "│   │");
}

#[test]
fn each_hue_sector_has_its_color() {
    // Both edges of every 60-degree sector
    for hue in [0, 30, 59, 60, 119, 120, 179, 180, 239, 240, 299, 300, 359] {
        let name = name_with_hue(hue);
        let bg = draw(&Avatar::new(&*name).small(), 1, 1)
            .get(0, 0)
            .unwrap()
            .bg
            .unwrap();
        assert!(is_name_color(&name, bg), "hue {hue}: {bg:?}");
    }
}

/// A name of printable ASCII whose bytes sum to `hue` mod 360.
fn name_with_hue(hue: u32) -> String {
    // Some '~' (126 each), then one byte in '!'..='~' for the rest.
    let mut sum = 0;
    let mut name = String::new();
    while (hue + 360 - sum % 360) % 360 > 126 || (hue + 360 - sum % 360) % 360 < 33 {
        name.push('~');
        sum += 126;
    }
    let last = (hue + 360 - sum % 360) % 360;
    name.push(char::from(last as u8));
    name
}

#[test]
fn the_stylesheet_color_is_the_initials_default_and_the_builder_wins() {
    let css = "#a { color: #00ff00; }";
    let mut h = PipelineHarness::with_css(css, 10, 1).dom_from_render(true);
    h.draw(&avatar("John Doe").element_id("a"));
    assert_eq!(
        h.buffer().get(1, 0).unwrap().fg,
        Some(Color::rgb(0, 255, 0))
    );

    let mut h = PipelineHarness::with_css(css, 10, 1).dom_from_render(true);
    h.draw(&avatar("John Doe").element_id("a").fg(Color::RED));
    assert_eq!(h.buffer().get(1, 0).unwrap().fg, Some(Color::RED));

    // Without a rule, white
    let mut h = PipelineHarness::with_css("#b { color: #00ff00; }", 10, 1).dom_from_render(true);
    h.draw(&avatar("John Doe").element_id("a"));
    assert_eq!(h.buffer().get(1, 0).unwrap().fg, Some(Color::WHITE));
}

/// Where the shown text comes from.
#[derive(Clone, Debug)]
enum Source {
    Name(String),
    NameWithIcon(String, char),
    Initials(String),
    Icon(char),
}

/// What one model cell holds.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Want {
    /// An initial: the glyph in fg on bg, bold.
    Initial(char),
    /// An outline glyph in the background color.
    Edge(char),
    /// The status dot.
    Dot,
}

/// The cells an avatar draws, avatar-relative, before clipping to the area
/// (a large avatar in at least three rows).
fn model(
    text: &[char],
    size: AvatarSize,
    shape: AvatarShape,
    status: bool,
) -> Vec<(u16, u16, Want)> {
    let mut cells = Vec::new();
    let initial = |i: usize| text.get(i).copied().unwrap_or(' ');
    let (dot_x, dot_y) = match size {
        AvatarSize::Small => {
            cells.push((0, 0, Want::Initial(text.first().copied().unwrap_or('?'))));
            (1, 0)
        }
        AvatarSize::Medium => {
            let (l, r) = match shape {
                AvatarShape::Circle => ('◖', '◗'),
                AvatarShape::Square => ('[', ']'),
                AvatarShape::Rounded => ('(', ')'),
            };
            cells.push((0, 0, Want::Edge(l)));
            for (i, &ch) in text.iter().take(2).enumerate() {
                cells.push((1 + i as u16, 0, Want::Initial(ch)));
            }
            cells.push((3, 0, Want::Edge(r)));
            (4, 0)
        }
        AvatarSize::Large => {
            let [tl, tr, bl, br] = match shape {
                AvatarShape::Square => ['┌', '┐', '└', '┘'],
                AvatarShape::Circle | AvatarShape::Rounded => ['╭', '╮', '╰', '╯'],
            };
            for (y, l, r) in [(0, tl, tr), (2, bl, br)] {
                cells.push((0, y, Want::Edge(l)));
                for x in 1..4 {
                    cells.push((x, y, Want::Edge('─')));
                }
                cells.push((4, y, Want::Edge(r)));
            }
            cells.push((0, 1, Want::Edge('│')));
            cells.push((1, 1, Want::Initial(initial(0))));
            cells.push((2, 1, Want::Initial(initial(1))));
            cells.push((3, 1, Want::Initial(' ')));
            cells.push((4, 1, Want::Edge('│')));
            (5, 2)
        }
    };
    if status {
        cells.push((dot_x, dot_y, Want::Dot));
    }
    cells
}

/// The text the model shows for `source`: up to two upper-cased word
/// initials of a name (an icon replaces them), or the given initials as
/// they are.
fn shown(source: &Source) -> Vec<char> {
    match source {
        Source::Name(name) => name
            .split_whitespace()
            .filter_map(|w| w.chars().next())
            .take(2)
            .map(|c| c.to_ascii_uppercase())
            .collect(),
        Source::NameWithIcon(_, icon) | Source::Icon(icon) => vec![*icon],
        Source::Initials(s) => s.chars().collect(),
    }
}

fn build(source: &Source) -> Avatar {
    match source {
        Source::Name(name) => Avatar::new(name.clone()),
        Source::NameWithIcon(name, icon) => avatar(name.clone()).icon(*icon),
        Source::Initials(s) => Avatar::from_initials(s.clone()),
        Source::Icon(icon) => Avatar::from_icon(*icon),
    }
}

/// The name the background is derived from (empty for initials and icons).
fn color_name(source: &Source) -> &str {
    match source {
        Source::Name(name) | Source::NameWithIcon(name, _) => name,
        Source::Initials(_) | Source::Icon(_) => "",
    }
}

/// ASCII names: zero to four words of letters (either case), separated by
/// one or more spaces or tabs, sometimes with leading spaces. Wide glyphs are
/// left out (#974), and so are letters whose upper case is two letters
/// ("ß" -> "SS"), which push the second initial out (#1057).
fn name() -> impl Strategy<Value = String> {
    (
        prop::collection::vec(
            ("[a-zA-Z]{1,5}", prop::sample::select(vec![" ", "  ", "\t"])),
            0..5,
        ),
        any::<bool>(),
    )
        .prop_map(|(words, lead)| {
            let mut s = if lead { " ".to_string() } else { String::new() };
            for (w, sep) in words {
                s.push_str(&w);
                s.push_str(sep);
            }
            s
        })
}

fn source() -> impl Strategy<Value = Source> {
    let icon = prop::sample::select(vec!['X', '@', '*', '7', 'q']);
    prop_oneof![
        3 => name().prop_map(Source::Name),
        1 => (name(), icon.clone()).prop_map(|(n, i)| Source::NameWithIcon(n, i)),
        1 => "[a-z0-9]{0,4}".prop_map(Source::Initials),
        1 => icon.prop_map(Source::Icon),
    ]
}

proptest! {
    /// Every cell of a drawn avatar is the model's: initials, edges and the
    /// status dot, with the builder's colors or the defaults (white initials
    /// on the name's color), clipped to the area, and nothing else.
    #[test]
    fn cells_match_the_model(
        source in source(),
        size in prop::sample::select(SIZES.to_vec()),
        shape in prop::sample::select(SHAPES.to_vec()),
        status in prop::option::of(prop::sample::select(vec![Color::RED, Color::rgb(1, 2, 3)])),
        bg in prop::option::of(prop::sample::select(vec![Color::BLUE, Color::rgb(9, 8, 7)])),
        fg in prop::option::of(prop::sample::select(vec![Color::YELLOW, Color::BLACK])),
        width in 0u16..8,
        height in 0u16..5,
    ) {
        // Not modeled: in fewer than three rows a large avatar draws one plain
        // initial rather than the medium form (#1057),
        // so a large one gets three or four rows.
        let height = if size == AvatarSize::Large { 3 + height % 2 } else { height };
        let mut a = build(&source).size(size).shape(shape);
        if let Some(c) = status { a = a.status(c); }
        if let Some(c) = bg { a = a.bg(c); }
        if let Some(c) = fg { a = a.fg(c); }
        let buf = draw(&a, width, height);

        let want = model(&shown(&source), size, shape, status.is_some());
        let fg_want = fg.unwrap_or(Color::WHITE);
        let blank = Cell::default();
        for y in 0..height {
            for x in 0..width {
                let cell = *buf.get(x, y).unwrap();
                let Some(&(_, _, w)) = want.iter().rev().find(|c| c.0 == x && c.1 == y) else {
                    prop_assert_eq!(cell, blank, "({}, {})", x, y);
                    continue;
                };
                let check_bg = |c: Option<Color>| match bg {
                    Some(b) => c == Some(b),
                    None => c.is_some_and(|c| is_name_color(color_name(&source), c)),
                };
                match w {
                    Want::Initial(ch) => {
                        prop_assert_eq!(cell.symbol, ch, "({}, {})", x, y);
                        prop_assert_eq!(cell.fg, Some(fg_want));
                        prop_assert!(check_bg(cell.bg), "bg {:?} at ({}, {})", cell.bg, x, y);
                        prop_assert_eq!(cell.modifier, Modifier::BOLD);
                    }
                    Want::Edge(ch) => {
                        prop_assert_eq!(cell.symbol, ch, "({}, {})", x, y);
                        prop_assert!(check_bg(cell.fg), "edge fg {:?} at ({}, {})", cell.fg, x, y);
                        prop_assert_eq!(cell.bg, None);
                        prop_assert_eq!(cell.modifier, Modifier::empty());
                    }
                    Want::Dot => {
                        prop_assert_eq!(cell.symbol, '●');
                        prop_assert_eq!(cell.fg, status);
                        prop_assert_eq!(cell.bg, None);
                        prop_assert_eq!(cell.modifier, Modifier::empty());
                    }
                }
            }
        }
    }

    /// The derived background is the HSL color of the name's byte sum, for
    /// any name, and so does not depend on the order of the bytes.
    #[test]
    fn the_background_is_the_hue_of_the_name(name in "[ -~]{0,12}") {
        let bg = draw(&Avatar::new(&*name).small(), 1, 1).get(0, 0).unwrap().bg.unwrap();
        prop_assert!(is_name_color(&name, bg), "{:?}: {:?}", name, bg);
        let reversed: String = name.chars().rev().collect();
        let bg2 = draw(&Avatar::new(reversed).small(), 1, 1).get(0, 0).unwrap().bg.unwrap();
        prop_assert_eq!(bg, bg2);
    }
}
