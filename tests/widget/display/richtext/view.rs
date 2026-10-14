//! RichText placement, and models of the markup and of where characters land

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{RichText, Style};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

fn draw(rt: &RichText, width: u16, height: u16) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, width, height));
    rt.render(&mut ctx);
    buffer
}

#[test]
fn test_rich_text_zero_areas_and_offset() {
    let rt = RichText::markup("[bold red]ab[/] [on_blue]日本[/]\n[link=u]x[/]");
    assert_zero_areas_draw_nothing(&rt);
    assert_same_when_offset(rt, 12, 3);
}

#[test]
fn test_rich_text_wide_glyph_takes_its_span_background() {
    let rt = RichText::new()
        .push("日", Style::new().bg(Color::RED))
        .push_link("本", "https://x.example");
    let buffer = draw(&rt, 6, 1);
    let cont = buffer.get(1, 0).unwrap();
    assert!(cont.is_continuation());
    assert_eq!(cont.bg, Some(Color::RED));
    // The linked glyph's continuation is linked too
    let link = buffer.get(2, 0).unwrap().hyperlink_id;
    assert!(link.is_some());
    assert_eq!(buffer.get(3, 0).unwrap().hyperlink_id, link);
}

/// One piece of markup
#[derive(Clone, Debug)]
enum Token {
    /// Plain text, no brackets
    Text(String),
    /// `[a b c]`: attribute and color names, any case, or `link=URL`
    Tag(Vec<&'static str>),
    /// `[/]`
    Reset,
    /// `[[`, a literal `[`
    Bracket,
}

const PARTS: [&str; 18] = [
    "b",
    "bold",
    "I",
    "italic",
    "u",
    "underline",
    "dim",
    "s",
    "strike",
    "rev",
    "reverse",
    "RED",
    "green",
    "on_blue",
    "On_Red",
    "link=u1",
    "link=u2",
    "nonsense",
];

fn token() -> impl Strategy<Value = Token> {
    prop_oneof![
        3 => "[a-z ]{1,4}".prop_map(Token::Text),
        3 => prop::collection::vec(prop::sample::select(PARTS.to_vec()), 1..4).prop_map(Token::Tag),
        1 => Just(Token::Reset),
        1 => Just(Token::Bracket),
    ]
}

/// What a cell should hold: symbol, fg, bg, modifier and link URL
type Expected = (
    char,
    Option<Color>,
    Option<Color>,
    Modifier,
    Option<&'static str>,
);

/// The markup for `tokens`, and the cells a plain model says it draws
fn model(tokens: &[Token]) -> (String, Vec<Expected>) {
    let mut markup = String::new();
    let mut cells = Vec::new();
    let (mut fg, mut bg, mut m, mut link) = (None, None, Modifier::empty(), None);
    for t in tokens {
        match t {
            Token::Text(s) => {
                markup.push_str(s);
                cells.extend(s.chars().map(|c| (c, fg, bg, m, link)));
            }
            Token::Bracket => {
                markup.push_str("[[");
                cells.push(('[', fg, bg, m, link));
            }
            Token::Reset => {
                markup.push_str("[/]");
                (fg, bg, m, link) = (None, None, Modifier::empty(), None);
            }
            Token::Tag(parts) => {
                markup.push('[');
                markup.push_str(&parts.join(" "));
                markup.push(']');
                for p in parts {
                    if let Some(url) = p.strip_prefix("link=") {
                        link = Some(url);
                        m |= Modifier::UNDERLINE;
                        fg = fg.or(Some(Color::CYAN));
                        continue;
                    }
                    match p.to_lowercase().as_str() {
                        "b" | "bold" => m |= Modifier::BOLD,
                        "i" | "italic" => m |= Modifier::ITALIC,
                        "u" | "underline" => m |= Modifier::UNDERLINE,
                        "dim" => m |= Modifier::DIM,
                        "s" | "strike" => m |= Modifier::CROSSED_OUT,
                        "rev" | "reverse" => m |= Modifier::REVERSE,
                        "red" => fg = Some(Color::RED),
                        "green" => fg = Some(Color::GREEN),
                        "on_blue" => bg = Some(Color::BLUE),
                        "on_red" => bg = Some(Color::RED),
                        _ => {}
                    }
                }
            }
        }
    }
    (markup, cells)
}

proptest! {
    /// Markup draws what a plain model of the tags gives: each tag adds to
    /// the style in effect, `[/]` clears it and the link, a link is
    /// underlined and cyan unless a color was named, `[[` is a `[`, unknown
    /// names do nothing, and names match in any case.
    #[test]
    fn prop_markup_follows_the_model(tokens in prop::collection::vec(token(), 0..12)) {
        let (markup, cells) = model(&tokens);
        let rt = RichText::markup(&markup);
        let width = 60u16;
        prop_assert!(cells.len() < width as usize);
        let buffer = draw(&rt, width, 1);
        for x in 0..width {
            let got = buffer.get(x, 0).unwrap();
            let url = got.hyperlink_id.and_then(|id| buffer.get_hyperlink(id));
            let want = cells.get(x as usize).copied().unwrap_or((' ', None, None, Modifier::empty(), None));
            prop_assert_eq!(
                (got.symbol, got.fg, got.bg, got.modifier, url),
                want,
                "x={} markup={:?}", x, markup
            );
        }
        prop_assert_eq!(rt.width(), cells.len());
    }

    /// Escaped text shows as itself, unstyled, whatever brackets it holds.
    #[test]
    fn prop_escaped_text_renders_as_itself(
        chars in prop::collection::vec(prop::sample::select(vec!['[', ']', '/', 'b', 'x', ' ', '=']), 0..30),
    ) {
        let raw: String = chars.iter().collect();
        let rt = RichText::markup(&RichText::escape(&raw));
        let buffer = draw(&rt, 32, 1);
        let drawn: String = (0..raw.len() as u16).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
        prop_assert_eq!(&drawn, &raw);
        for x in 0..32 {
            let cell = buffer.get(x, 0).unwrap();
            prop_assert_eq!((cell.fg, cell.bg, cell.modifier), (None, None, Modifier::empty()));
        }
    }

    /// Characters land where a plain model puts them: a newline starts the
    /// next row at the left edge, a wide glyph takes two columns (the second
    /// a continuation), rows past the area are dropped, and a row wider than
    /// the area is cut. The measure is the widest line by the line count.
    #[test]
    fn prop_rich_text_places_characters_like_the_model(
        chars in prop::collection::vec(prop::sample::select(vec!['a', 'b', ' ', '日', '😀', '\n']), 0..40),
        split in 0usize..40,
        width in 1u16..12,
        height in 1u16..5,
    ) {
        let text: String = chars.iter().collect();
        // Two spans, so a newline can fall in either and the rows carry on
        let at = text.char_indices().nth(split).map_or(text.len(), |(i, _)| i);
        let rt = RichText::new().text(&text[..at]).push(&text[at..], Style::new().bold());

        let mut want = vec![vec![' '; width as usize]; height as usize];
        let mut cont = vec![vec![false; width as usize]; height as usize];
        let mut skip = vec![vec![false; width as usize]; height as usize];
        let (mut x, mut y) = (0usize, 0usize);
        let (mut widest, mut line) = (0usize, 0usize);
        for &c in &chars {
            if c == '\n' {
                (x, y) = (0, y + 1);
                widest = widest.max(line);
                line = 0;
                continue;
            }
            let w = if c.is_ascii() { 1 } else { 2 };
            line += w;
            if y < height as usize && x < width as usize {
                if w == 2 && x + 1 == width as usize {
                    // A wide glyph starting in the last column is drawn there
                    // with no room for its right half (#974);
                    // that cell is not checked.
                    skip[y][x] = true;
                } else {
                    want[y][x] = c;
                    if w == 2 {
                        cont[y][x + 1] = true;
                    }
                }
            }
            x += w;
        }
        widest = widest.max(line);
        let lines = 1 + chars.iter().filter(|&&c| c == '\n').count();

        let buffer = draw(&rt, width, height);
        for yy in 0..height as usize {
            for xx in 0..width as usize {
                let cell = buffer.get(xx as u16, yy as u16).unwrap();
                if skip[yy][xx] {
                    continue;
                }
                if cont[yy][xx] {
                    prop_assert!(cell.is_continuation(), "({}, {}) in {:?}", xx, yy, text);
                } else {
                    prop_assert_eq!(cell.symbol, want[yy][xx], "({}, {}) in {:?}", xx, yy, text);
                }
            }
        }
        prop_assert_eq!(rt.width(), widest);
        prop_assert_eq!(
            rt.measure(width, height),
            Some((widest.min(width as usize) as u16, lines.min(height as usize) as u16))
        );
    }
}
