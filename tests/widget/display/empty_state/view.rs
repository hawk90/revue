//! EmptyState placement, the stylesheet accent, and a model of each layout

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::{Color, Style as Css};
use revue::widget::traits::{RenderContext, View};
use revue::widget::{EmptyState, EmptyStateType, EmptyStateVariant, LIGHT_GRAY};

use crate::traits::placement::{assert_same_when_offset, assert_zero_areas_draw_nothing};

const VARIANTS: [EmptyStateVariant; 3] = [
    EmptyStateVariant::Full,
    EmptyStateVariant::Compact,
    EmptyStateVariant::Minimal,
];

fn draw(es: &EmptyState, width: u16, height: u16, css: Option<&Css>) -> Buffer {
    let mut buffer = Buffer::new(width, height);
    let area = Rect::new(0, 0, width, height);
    let mut ctx = match css {
        Some(css) => RenderContext::with_style(&mut buffer, area, css),
        None => RenderContext::new(&mut buffer, area),
    };
    es.render(&mut ctx);
    buffer
}

#[test]
fn test_empty_state_zero_areas_and_offset() {
    for variant in VARIANTS {
        let es = EmptyState::error("Nothing")
            .description("Try again")
            .action("Retry")
            .variant(variant);
        assert_zero_areas_draw_nothing(&es);
        assert_same_when_offset(es, 24, 7);
    }
}

#[test]
fn test_empty_state_stylesheet_color_is_the_accent() {
    let mut css = Css::default();
    css.visual.color = Color::MAGENTA;
    let es = EmptyState::error("T").description("d").action("a");
    let buffer = draw(&es, 20, 6, Some(&css));
    // Rows: icon 0, title 2, description 3, action 5
    assert_eq!(buffer.get(10, 0).unwrap().fg, Some(Color::MAGENTA));
    assert_eq!(buffer.get(7, 5).unwrap().symbol, '[');
    assert_eq!(buffer.get(7, 5).unwrap().fg, Some(Color::MAGENTA));
    // The title and description keep their own colors
    assert_eq!(buffer.get(9, 2).unwrap().fg, Some(Color::WHITE));
    assert_eq!(buffer.get(9, 3).unwrap().fg, Some(LIGHT_GRAY));
}

#[test]
fn test_empty_state_measure_is_its_height_across_the_offer() {
    let es = EmptyState::new("T").description("d").action("a");
    assert_eq!(es.measure(30, 100), Some((30, 6)));
    assert_eq!(es.measure(7, 4), Some((7, 4)));
    let compact = es.clone().variant(EmptyStateVariant::Compact);
    assert_eq!(compact.measure(30, 100), Some((30, 3)));
    let minimal = es.variant(EmptyStateVariant::Minimal);
    assert_eq!(minimal.measure(30, 100), Some((30, 1)));
}

/// A cell the model expects: symbol, fg and whether it is bold
type Want = (char, Option<Color>, bool);

struct Model {
    width: usize,
    height: usize,
    cells: Vec<Vec<Want>>,
}

impl Model {
    fn new(width: usize, height: usize) -> Self {
        Model {
            width,
            height,
            cells: vec![vec![(' ', None, false); width]; height],
        }
    }

    /// `text` from column `x` of row `y`, cut at the right edge
    fn put(&mut self, x: usize, y: usize, text: &str, fg: Color, bold: bool) {
        if y >= self.height {
            return;
        }
        for (i, ch) in text.chars().enumerate() {
            if x + i >= self.width {
                break;
            }
            self.cells[y][x + i] = (ch, Some(fg), bold);
        }
    }

    fn centered(&mut self, y: usize, text: &str, fg: Color, bold: bool) {
        let x = self.width.saturating_sub(text.len()) / 2;
        self.put(x, y, text, fg, bold);
    }
}

#[derive(Debug, Clone)]
struct Case {
    variant: EmptyStateVariant,
    state: EmptyStateType,
    title: String,
    description: Option<String>,
    action: Option<String>,
    icon: bool,
    width: u16,
    height: u16,
}

fn case() -> impl Strategy<Value = Case> {
    let state = prop::sample::select(vec![
        EmptyStateType::Empty,
        EmptyStateType::NoResults,
        EmptyStateType::Error,
        EmptyStateType::NoPermission,
        EmptyStateType::Offline,
        EmptyStateType::FirstUse,
    ]);
    (
        prop::sample::select(VARIANTS.to_vec()),
        state,
        "[a-z ]{0,14}",
        prop::option::of("[a-z ]{0,14}"),
        prop::option::of("[a-z]{0,10}"),
        any::<bool>(),
        0u16..24,
        0u16..10,
    )
        .prop_map(
            |(variant, state, title, description, action, icon, width, height)| Case {
                variant,
                state,
                title,
                description,
                action,
                icon,
                width,
                height,
            },
        )
}

/// The rows a plain model says the full layout takes
fn full_rows(c: &Case) -> u16 {
    let mut rows = 1;
    if c.icon {
        rows += 2;
    }
    if c.description.is_some() {
        rows += 1;
    }
    if c.action.is_some() {
        rows += if c.description.is_some() { 2 } else { 1 };
    }
    rows
}

/// What a plain model of the three layouts draws
fn model(c: &Case) -> (u16, Model) {
    let (w, h) = (c.width as usize, c.height as usize);
    let mut m = Model::new(w, h);
    let accent = c.state.color();
    let rows = match c.variant {
        EmptyStateVariant::Full => full_rows(c),
        EmptyStateVariant::Compact => {
            1 + u16::from(c.description.is_some()) + u16::from(c.action.is_some())
        }
        EmptyStateVariant::Minimal => 1,
    };
    if w < 5 || h < 1 {
        return (rows, m);
    }
    match c.variant {
        EmptyStateVariant::Full => {
            // Centered as a block; parts that fall past the bottom are dropped
            let mut y = h.saturating_sub(rows as usize) / 2;
            if c.icon {
                m.put(w / 2, y, "*", accent, false);
                y += 2;
            }
            if y < h {
                m.centered(y, &c.title, Color::WHITE, true);
                y += 1;
            }
            if let Some(d) = &c.description {
                if y < h {
                    m.centered(y, d, LIGHT_GRAY, false);
                    y += 2;
                }
            }
            if let Some(a) = &c.action {
                m.centered(y, &format!("[ {a} ]"), accent, false);
            }
        }
        EmptyStateVariant::Compact => {
            // Icon and title on the first row; the rest below, past the icon
            let indent = if c.icon { 2 } else { 0 };
            if c.icon {
                m.put(0, 0, "*", accent, false);
            }
            m.put(indent, 0, &c.title, Color::WHITE, true);
            let mut y = 1;
            if let Some(d) = &c.description {
                if y < h {
                    m.put(indent, y, d, LIGHT_GRAY, false);
                    y += 1;
                }
            }
            if let Some(a) = &c.action {
                m.put(indent, y, &format!("[{a}]"), accent, false);
            }
        }
        EmptyStateVariant::Minimal => {
            let indent = if c.icon { 2 } else { 0 };
            if c.icon {
                m.put(0, 0, "*", accent, false);
            }
            m.put(indent, 0, &c.title, LIGHT_GRAY, false);
        }
    }
    (rows, m)
}

proptest! {
    /// Every variant draws what a plain model of its layout gives, at any
    /// size: the full layout centered as a block with the parts that do not
    /// fit dropped from the bottom, the compact one from the top-left, the
    /// minimal one on one row; text cut at the right edge; nothing below 5
    /// columns. `height()` is the model's row count.
    #[test]
    fn prop_empty_state_layouts_follow_the_model(c in case()) {
        let mut es = EmptyState::new(c.title.clone()).variant(c.variant).state_type(c.state);
        if let Some(d) = &c.description {
            es = es.description(d.clone());
        }
        if let Some(a) = &c.action {
            es = es.action(a.clone());
        }
        es = if c.icon { es.custom_icon('*') } else { es.icon(false) };

        let (rows, m) = model(&c);
        prop_assert_eq!(es.height(), rows);
        let buffer = draw(&es, c.width, c.height, None);
        for y in 0..c.height {
            for x in 0..c.width {
                let cell = buffer.get(x, y).unwrap();
                let got = (cell.symbol, cell.fg, cell.modifier.contains(Modifier::BOLD));
                prop_assert_eq!(got, m.cells[y as usize][x as usize], "({}, {}) {:?}", x, y, c);
            }
        }
    }
}
