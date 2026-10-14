//! Card placement, its layout against a plain model of the rows, the
//! stylesheet, and collapsing by keys

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::{BorderStyle, Color, Style, VisualStyle};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{vstack, BorderType, Card, CardVariant, Text};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

/// A card built on demand, so the placement checks (which need `Clone`)
/// can take a `Card`, which holds boxed children and is not `Clone`
#[derive(Clone)]
struct Built(fn() -> Card);

impl View for Built {
    fn render(&self, ctx: &mut RenderContext) {
        (self.0)().render(ctx)
    }
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        (self.0)().measure(max_width, max_height)
    }
    fn fills(&self) -> revue::widget::Fill {
        (self.0)().fills()
    }
}

#[test]
fn test_card_zero_areas_and_offset() {
    let cards: [fn() -> Card; 6] = [
        Card::new,
        || {
            Card::new()
                .title("제목 Title")
                .subtitle("Sub")
                .header(Text::new("Head"))
                .body(Text::new("Body"))
                .footer(Text::new("Foot"))
        },
        || Card::new().filled().title("T").body(Text::new("B")),
        || Card::new().elevated().title("T").footer(Text::new("F")),
        || Card::new().flat().title("T").body(Text::new("B")),
        || {
            Card::new()
                .collapsible(true)
                .expanded(false)
                .rounded()
                .background(Color::BLUE)
                .title("T")
                .body(Text::new("B"))
        },
    ];
    for make in cards {
        assert_zero_areas_draw_nothing(&make());
        assert_same_when_offset(Built(make), 16, 9);
    }
}

/// Render `c` into a `width` x `height` buffer, optionally under `style`.
fn render_buffer(c: &Card, width: u16, height: u16, style: Option<&Style>) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = match style {
        Some(s) => RenderContext::with_style(&mut buffer, area, s),
        None => RenderContext::new(&mut buffer, area),
    };
    c.render(&mut ctx);
    buffer
}

fn rows_of(buffer: &Buffer) -> Vec<String> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect()
        })
        .collect()
}

#[test]
fn test_card_empty_draws_only_its_frame() {
    let rows = rows_of(&render_buffer(&Card::new(), 6, 3, None));
    assert_eq!(rows, ["┌────┐", "│    │", "└────┘"]);
    // A borderless empty card draws nothing
    let rows = rows_of(&render_buffer(&Card::new().flat(), 6, 3, None));
    assert!(rows.iter().all(|r| r.trim().is_empty()), "{rows:?}");
    // and measures to the 3 rows it needs to draw at all
    assert_eq!(Card::new().measure(6, 10), Some((6, 3)));
}

#[test]
fn test_card_size_guards() {
    // 4x3 is the smallest card drawn; the shadow needs one more each way
    let c = Card::new().elevated();
    assert_eq!(
        rows_of(&render_buffer(&c, 4, 3, None)),
        ["┌──┐", "│  │", "└──┘"]
    );
    assert_eq!(
        rows_of(&render_buffer(&c, 5, 4, None)),
        ["┌──┐ ", "│  │▌", "└──┘▌", " ▀▀▀▀"]
    );
    for (w, h) in [(3, 3), (4, 2), (1, 1)] {
        let rows = rows_of(&render_buffer(&c, w, h, None));
        assert!(rows.iter().all(|r| r.trim().is_empty()), "{w}x{h}");
    }
}

#[test]
fn test_card_collapsed_hides_header_body_and_footer() {
    let c = Card::new()
        .collapsible(true)
        .expanded(false)
        .title("T")
        .header(Text::new("Head"))
        .body(Text::new("Body"))
        .footer(Text::new("Foot"));
    let rows = rows_of(&render_buffer(&c, 10, 6, None));
    assert_eq!(
        rows,
        [
            "┌────────┐",
            "│ ▶ T    │",
            "│        │",
            "│        │",
            "│        │",
            "└────────┘"
        ]
    );
    // and measures to the title row inside the frame
    assert_eq!(c.measure(10, 20), Some((10, 3)));
}

#[test]
fn test_card_size_constraints() {
    let c = || Card::new().title("T");
    // max_width and max_height shrink the card from the top left
    let rows = rows_of(&render_buffer(&c().max_size(6, 4), 10, 6, None));
    assert_eq!(rows[0], "┌────┐    ");
    assert_eq!(rows[3], "└────┘    ");
    assert_eq!(rows[4].trim(), "");
    // min_width and min_height grow it past its area, which clips the far
    // edges (the shared `constraints::constrain` rule)
    let rows = rows_of(&render_buffer(&c().min_size(20, 10), 8, 4, None));
    assert_eq!(rows[0], "┌───────");
    assert_eq!(rows[3], "│       ");
    // measure takes them too
    assert_eq!(c().max_size(6, 4).measure(10, 6), Some((6, 3)));
    assert_eq!(c().min_size(9, 5).measure(20, 20), Some((20, 5)));
    assert_eq!(c().constrain(0, 7, 12, 0).measure(20, 20), Some((12, 7)));
}

fn with_visual(visual: VisualStyle) -> Style {
    Style {
        visual,
        ..Default::default()
    }
}

#[test]
fn test_card_stylesheet_border_style() {
    let c = Card::new().title("T");
    for (style, corner) in [
        (BorderStyle::Solid, '┌'),
        (BorderStyle::Dashed, '┌'),
        (BorderStyle::Double, '╔'),
        (BorderStyle::Rounded, '╭'),
    ] {
        let s = with_visual(VisualStyle {
            border_style: Some(style),
            ..VisualStyle::default()
        });
        // It replaces the builder's border, even a flat card's
        for card in [&c, &Card::new().flat().title("T")] {
            let rows = rows_of(&render_buffer(card, 8, 3, Some(&s)));
            assert_eq!(rows[0].chars().next(), Some(corner), "{style:?}");
        }
    }
    let none = with_visual(VisualStyle {
        border_style: Some(BorderStyle::None),
        ..VisualStyle::default()
    });
    let rows = rows_of(&render_buffer(&c, 8, 3, Some(&none)));
    assert_eq!(rows, [" T      ", "        ", "        "]);
}

#[test]
fn test_card_stylesheet_colors_under_the_builders() {
    let s = with_visual(VisualStyle {
        background: Color::rgb(1, 2, 3),
        border_color: Color::rgb(4, 5, 6),
        color: Color::rgb(7, 8, 9),
        ..VisualStyle::default()
    });
    let b = render_buffer(&Card::new().title("T"), 8, 3, Some(&s));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::rgb(4, 5, 6)));
    assert_eq!(b.get(4, 1).unwrap().bg, Some(Color::rgb(1, 2, 3)));
    // The text color is the border's fallback; the title keeps its own
    let text_only = with_visual(VisualStyle {
        color: Color::rgb(7, 8, 9),
        ..VisualStyle::default()
    });
    let b = render_buffer(&Card::new().title("T"), 8, 3, Some(&text_only));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::rgb(7, 8, 9)));
    assert_eq!(b.get(2, 1).unwrap().fg, Some(Color::WHITE));
    // The builder's colors win over the stylesheet's
    let c = Card::new()
        .title("T")
        .background(Color::RED)
        .border_color(Color::BLUE);
    let b = render_buffer(&c, 8, 3, Some(&s));
    assert_eq!(b.get(0, 0).unwrap().fg, Some(Color::BLUE));
    assert_eq!(b.get(4, 1).unwrap().bg, Some(Color::RED));
}

#[test]
fn test_card_variant_colors() {
    let at = |c: Card, x, y| *render_buffer(&c.title("T"), 10, 5, None).get(x, y).unwrap();
    assert_eq!(
        at(Card::new().filled(), 0, 0).fg,
        Some(Color::rgb(50, 50, 60))
    );
    assert_eq!(
        at(Card::new().filled(), 4, 2).bg,
        Some(Color::rgb(30, 30, 35))
    );
    assert_eq!(
        at(Card::new().elevated(), 0, 0).fg,
        Some(Color::rgb(70, 70, 80))
    );
    assert_eq!(
        at(Card::new().elevated(), 4, 2).bg,
        Some(Color::rgb(35, 35, 40))
    );
    // The shadow is dark gray
    assert_eq!(
        at(Card::new().elevated(), 9, 2).fg,
        Some(Color::rgb(20, 20, 20))
    );
    // Outlined and flat cards have no background
    assert_eq!(at(Card::new(), 4, 2).bg, None);
    assert_eq!(at(Card::new().flat(), 4, 2).bg, None);
}

/// What a card is made of, for the layout model
#[derive(Debug, Clone)]
struct Spec {
    title: Option<String>,
    subtitle: Option<String>,
    header: Option<String>,
    body: Option<Vec<String>>,
    footer: Option<String>,
    variant: usize,
    border: Option<usize>,
    padding: u16,
    collapsible: bool,
    expanded: bool,
}

const VARIANTS: [CardVariant; 4] = [
    CardVariant::Outlined,
    CardVariant::Filled,
    CardVariant::Elevated,
    CardVariant::Flat,
];
const BORDERS: [(BorderType, [char; 6]); 3] = [
    (BorderType::None, [' '; 6]),
    (BorderType::Single, ['┌', '┐', '└', '┘', '─', '│']),
    (BorderType::Rounded, ['╭', '╮', '╰', '╯', '─', '│']),
];

impl Spec {
    fn card(&self) -> Card {
        let mut c = Card::new()
            .variant(VARIANTS[self.variant])
            .padding(self.padding)
            .collapsible(self.collapsible)
            .expanded(self.expanded);
        if let Some(b) = self.border {
            c = c.border_style(BORDERS[b].0);
        }
        if let Some(t) = &self.title {
            c = c.title(t.clone());
        }
        if let Some(t) = &self.subtitle {
            c = c.subtitle(t.clone());
        }
        if let Some(t) = &self.header {
            c = c.header(Text::new(t.clone()));
        }
        if let Some(lines) = &self.body {
            let mut v = vstack();
            for l in lines {
                v = v.child(Text::new(l.clone()));
            }
            c = c.body(v);
        }
        if let Some(t) = &self.footer {
            c = c.footer(Text::new(t.clone()));
        }
        c
    }

    /// The border drawn: the one given last, `variant(Flat)` clearing it
    fn border(&self) -> usize {
        match self.border {
            Some(b) => b,
            None if VARIANTS[self.variant] == CardVariant::Flat => 0,
            None => 1,
        }
    }

    /// The rows a card of `w` x `h` should show (at least its measured
    /// height)
    #[allow(clippy::needless_range_loop)] // grid cells by column
    fn model(&self, w: u16, h: u16) -> Vec<String> {
        let (w, h) = (w as usize, h as usize);
        let mut g = vec![vec![' '; w]; h];
        let elevated = VARIANTS[self.variant] == CardVariant::Elevated && w > 4 && h > 3;
        let (bw, bh) = if elevated { (w - 1, h - 1) } else { (w, h) };
        if elevated {
            for row in g.iter_mut().skip(1) {
                row[w - 1] = '▌';
            }
            for x in 1..w {
                g[h - 1][x] = '▀';
            }
        }
        let border = self.border();
        let framed = border != 0;
        let b = usize::from(framed);
        let [tl, tr, bl, br, hz, vt] = BORDERS[border].1;
        if framed {
            for x in 1..bw - 1 {
                g[0][x] = hz;
                g[bh - 1][x] = hz;
            }
            for row in g.iter_mut().take(bh - 1).skip(1) {
                row[0] = vt;
                row[bw - 1] = vt;
            }
            (g[0][0], g[0][bw - 1], g[bh - 1][0], g[bh - 1][bw - 1]) = (tl, tr, bl, br);
        }
        let p = self.padding as usize;
        let cx = b + p;
        let cw = bw.saturating_sub(2 * b + 2 * p);
        let put = |g: &mut Vec<Vec<char>>, x: usize, y: usize, text: &str, max: usize| {
            for (i, ch) in text.chars().take(max).enumerate() {
                g[y][x + i] = ch;
            }
        };
        let separator = |g: &mut Vec<Vec<char>>, y: usize| {
            if framed {
                g[y][0] = '├';
                g[y][bw - 1] = '┤';
                for x in 1..bw - 1 {
                    g[y][x] = '─';
                }
            }
        };
        let shown = self.expanded || !self.collapsible;
        let mut y = b;
        if let Some(t) = &self.title {
            if self.collapsible {
                g[y][cx] = if self.expanded { '▼' } else { '▶' };
                put(&mut g, cx + 2, y, t, cw.saturating_sub(2));
            } else {
                put(&mut g, cx, y, t, cw);
            }
            y += 1;
        }
        if let Some(t) = &self.subtitle {
            put(&mut g, cx, y, t, cw);
            y += 1;
        }
        if shown {
            if let Some(t) = &self.header {
                put(&mut g, cx, y, t, cw);
                y += 1;
            }
            let has_header =
                self.title.is_some() || self.subtitle.is_some() || self.header.is_some();
            if has_header && self.body.is_some() {
                if framed {
                    separator(&mut g, y);
                } else {
                    for x in 0..bw {
                        g[y][x] = '─';
                    }
                }
                y += 1;
            }
            let footer_rows = if self.footer.is_some() { 2 } else { 0 };
            if let Some(lines) = &self.body {
                let end = bh - b - footer_rows;
                for (i, l) in lines.iter().enumerate().take(end - y) {
                    put(&mut g, cx, y + i, l, cw);
                }
                y = end.max(y);
            }
            if let Some(t) = &self.footer {
                let fy = bh - 1 - b;
                if fy > y {
                    separator(&mut g, fy - 1);
                    put(&mut g, cx, fy, t, cw);
                }
            }
        }
        g.into_iter().map(|r| r.into_iter().collect()).collect()
    }
}

fn word() -> impl Strategy<Value = String> {
    "[a-z]{1,8}"
}

fn spec() -> impl Strategy<Value = Spec> {
    (
        (
            prop::option::of("[a-z]{0,8}"),
            prop::option::of(word()),
            prop::option::of(word()),
            prop::option::of(prop::collection::vec(word(), 1..4)),
            prop::option::of(word()),
        ),
        (
            0usize..4,
            prop::option::of(0usize..3),
            0u16..4,
            any::<bool>(),
            any::<bool>(),
        ),
    )
        .prop_map(
            |(
                (title, subtitle, header, body, footer),
                (variant, border, padding, collapsible, expanded),
            )| Spec {
                title,
                subtitle,
                header,
                body,
                footer,
                variant,
                border,
                padding,
                collapsible,
                expanded,
            },
        )
}

proptest! {
    /// At its measured height, or taller, a card shows the rows a plain
    /// model gives: frame, title (with the collapse icon), subtitle,
    /// header, separator, body lines, footer and shadow, each clipped to
    /// the content width.
    #[test]
    fn prop_card_layout_matches_the_model(spec in spec(), w in 12u16..30, extra in 0u16..4) {
        let card = spec.card();
        let (mw, mh) = card.measure(w, 100).unwrap();
        prop_assert_eq!(mw, w);
        let h = mh + extra;
        let rows = rows_of(&render_buffer(&card, w, h, None));
        prop_assert_eq!(rows, spec.model(w, h), "{:?} at {}x{}", spec, w, h);
    }

    /// Keys collapse and expand only an enabled collapsible card; `toggle`
    /// needs a collapsible card, `expand` and `collapse` do not.
    #[test]
    fn prop_card_keys_and_calls(
        collapsible in any::<bool>(),
        disabled in any::<bool>(),
        expanded in any::<bool>(),
        ops in prop::collection::vec(0usize..12, 0..24),
    ) {
        const KEYS: [Key; 9] = [
            Key::Enter, Key::Char(' '), Key::Left, Key::Right, Key::Char('h'),
            Key::Char('l'), Key::Char('x'), Key::Up, Key::Tab,
        ];
        let mut c = Card::new().collapsible(collapsible).disabled(disabled).expanded(expanded);
        let mut open = expanded;
        for op in ops {
            match op {
                9 => { c.toggle(); if collapsible { open = !open; } }
                10 => { c.expand(); open = true; }
                11 => { c.collapse(); open = false; }
                k => {
                    let key = KEYS[k];
                    let live = collapsible && !disabled;
                    let used = live && k < 6;
                    prop_assert_eq!(c.handle_key(&key), used, "{:?}", key);
                    if used {
                        open = match key {
                            Key::Enter | Key::Char(' ') => !open,
                            Key::Right | Key::Char('l') => true,
                            _ => false,
                        };
                    }
                }
            }
            prop_assert_eq!(c.is_expanded(), open);
        }
        prop_assert_eq!(c.is_collapsible(), collapsible);
    }
}

#[test]
fn test_card_collapsible_title_is_bold_after_its_icon() {
    let c = Card::new()
        .collapsible(true)
        .title("Title")
        .title_color(Color::GREEN);
    let b = render_buffer(&c, 12, 3, None);
    // The icon takes the title color; the title starts two columns later,
    // bold like any title
    let icon = b.get(2, 1).unwrap();
    assert_eq!((icon.symbol, icon.fg), ('▼', Some(Color::GREEN)));
    let t = b.get(4, 1).unwrap();
    assert_eq!((t.symbol, t.fg), ('T', Some(Color::GREEN)));
    assert!(t.modifier.contains(revue::render::Modifier::BOLD));
}
