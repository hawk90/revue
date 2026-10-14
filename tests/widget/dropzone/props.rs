//! DropZone proptests: drags and drops against a plain model of the hover
//! state and the handler's calls, every cell of a render against a plain
//! model of the frame and the text, and the builders in any order

use std::cell::RefCell;
use std::rc::Rc;

use proptest::prelude::*;
use revue::event::drag::DragData;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::widget::theme::{DISABLED_FG, LIGHT_GRAY};
use revue::widget::traits::Draggable;
use revue::widget::{DropZone, DropZoneStyle};

use super::{render, row};

const TYPES: [&str; 3] = ["text", "file", "list_item"];
const HOVER: Color = Color::rgb(100, 200, 100);
const REJECT: Color = Color::rgb(200, 100, 100);

fn data(kind: usize) -> DragData {
    match kind {
        0 => DragData::text("t"),
        1 => DragData::file("/nowhere/f.txt"),
        _ => DragData::list_item(1, "item"),
    }
}

#[derive(Debug, Clone)]
enum Op {
    Enter(usize),
    Leave,
    Drop(usize),
    SetHovered(bool, bool),
}

fn op() -> impl Strategy<Value = Op> {
    prop_oneof![
        3 => (0..3usize).prop_map(Op::Enter),
        1 => Just(Op::Leave),
        2 => (0..3usize).prop_map(Op::Drop),
        1 => (any::<bool>(), any::<bool>()).prop_map(|(h, a)| Op::SetHovered(h, a)),
    ]
}

/// The zone's state as the drag ops should leave it.
#[derive(Debug, Default)]
struct Model {
    hovered: bool,
    accept: bool,
    calls: Vec<&'static str>,
}

proptest! {
    #[test]
    fn drags_and_drops_follow_the_model(
        accepts in proptest::sample::subsequence(TYPES.to_vec(), 0..=3),
        disabled in any::<bool>(),
        handler in proptest::option::of(any::<bool>()),
        ops in prop::collection::vec(op(), 0..24),
    ) {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let seen = calls.clone();
        let base = DropZone::new("Zone").accepts(&accepts).disabled(disabled);
        let accepted = |kind: usize| accepts.is_empty() || accepts.contains(&TYPES[kind]);
        let mut model = Model::default();

        // With a handler and without one, the zone keeps the same state.
        let mut with: Option<DropZone<_>> = handler.map(|says| {
            base.clone().on_drop(move |d: DragData| {
                seen.borrow_mut().push(d.type_id);
                says
            })
        });
        let mut without = base.clone();

        for op in &ops {
            match *op {
                Op::Enter(kind) => {
                    let d = data(kind);
                    if let Some(z) = with.as_mut() { z.on_drag_enter(&d); }
                    without.on_drag_enter(&d);
                    model.hovered = true;
                    model.accept = !disabled && accepted(kind);
                }
                Op::Leave => {
                    if let Some(z) = with.as_mut() { z.on_drag_leave(); }
                    without.on_drag_leave();
                    model.hovered = false;
                    model.accept = false;
                }
                Op::Drop(kind) => {
                    // A drop of a type the zone does not accept still runs the
                    // handler (#1108), so only accepted types are dropped.
                    if !accepted(kind) {
                        continue;
                    }
                    let expect = !disabled && handler == Some(true);
                    if let Some(z) = with.as_mut() {
                        prop_assert_eq!(Draggable::on_drop(z, data(kind)), expect);
                    }
                    prop_assert!(!Draggable::on_drop(&mut without, data(kind)));
                    if !disabled && handler.is_some() {
                        model.calls.push(TYPES[kind]);
                    }
                    model.hovered = false;
                    model.accept = false;
                }
                Op::SetHovered(h, a) => {
                    if let Some(z) = with.as_mut() { z.set_hovered(h, a); }
                    without.set_hovered(h, a);
                    model.hovered = h;
                    model.accept = a;
                }
            }
            prop_assert_eq!(without.is_hovered(), model.hovered);
            prop_assert_eq!(without.can_accept_current(), model.accept);
            prop_assert_eq!(without.can_drop(), !disabled);

            // The screen says the same thing.
            let buffer = render(&without, 24, 3);
            let (text, text_color) = match (model.hovered, model.accept) {
                (true, true) => ("Drop here!", HOVER),
                (true, false) => ("Cannot drop here", REJECT),
                (false, _) if disabled => ("Zone", DISABLED_FG),
                (false, _) => ("Zone", LIGHT_GRAY),
            };
            let border = match (model.hovered, model.accept) {
                (false, _) => DISABLED_FG,
                (true, true) => HOVER,
                (true, false) => REJECT,
            };
            prop_assert_eq!(row(&buffer, 1), format!("│ {text:<20} │"));
            prop_assert_eq!(buffer.get(2, 1).unwrap().fg, Some(text_color));
            prop_assert_eq!(buffer.get(0, 0).unwrap().fg, Some(border));
        }
        drop(with);
        prop_assert_eq!(&*calls.borrow(), &model.calls);
    }
}

/// One expected cell: symbol, fg, bg, bold.
type Cell = (char, Option<Color>, Option<Color>, bool);

#[derive(Debug, Clone)]
struct Setup {
    style: DropZoneStyle,
    width: u16,
    height: u16,
    min_height: Option<u16>,
    hovered: bool,
    accept: bool,
    focused: bool,
    disabled: bool,
    fg: Option<Color>,
    bg: Option<Color>,
    border: Option<Color>,
    hover: Option<Color>,
    text: &'static str,
}

const TEXTS: [&str; 4] = ["", "Drop", "Drop files here", "x"];

fn color() -> impl Strategy<Value = Option<Color>> {
    proptest::option::of(prop_oneof![
        Just(Color::YELLOW),
        Just(Color::CYAN),
        Just(Color::rgb(1, 2, 3)),
    ])
}

fn setup() -> impl Strategy<Value = Setup> {
    (
        (
            prop_oneof![
                Just(DropZoneStyle::Solid),
                Just(DropZoneStyle::Dashed),
                Just(DropZoneStyle::Highlight),
                Just(DropZoneStyle::Minimal),
            ],
            1..=26u16,
            // Three rows or more, and a minimum height that fits: a shorter
            // area or a larger minimum draws text over the frame or past the
            // area (#1107).
            3..=9u16,
            proptest::option::of(0..=9u16),
            any::<(bool, bool)>(),
            any::<(bool, bool)>(),
        ),
        (color(), color(), color(), color(), 0..TEXTS.len()),
    )
        .prop_map(
            |(
                (style, width, height, min_height, (hovered, accept), (focused, disabled)),
                (fg, bg, border, hover, t),
            )| {
                Setup {
                    style,
                    width,
                    height,
                    min_height: min_height.map(|m| m.min(height)),
                    hovered,
                    accept,
                    focused,
                    disabled,
                    fg,
                    bg,
                    border,
                    hover,
                    text: TEXTS[t],
                }
            },
        )
}

fn build(s: &Setup) -> DropZone {
    let mut z = DropZone::new(s.text)
        .style(s.style)
        .focused(s.focused)
        .disabled(s.disabled);
    if let Some(m) = s.min_height {
        z = z.min_height(m);
    }
    if let Some(c) = s.fg {
        z = z.fg(c);
    }
    if let Some(c) = s.bg {
        z = z.bg(c);
    }
    if let Some(c) = s.border {
        z = z.border_color(c);
    }
    if let Some(c) = s.hover {
        z = z.hover_color(c);
    }
    z.set_hovered(s.hovered, s.accept);
    z
}

/// Every cell of the zone, from the setup alone.
fn model(s: &Setup) -> Vec<Vec<Cell>> {
    let (w, h) = (s.width as usize, s.height as usize);
    let mut grid = vec![vec![(' ', None, s.bg, false); w]; h];
    let verdict = if s.accept {
        s.hover.unwrap_or(HOVER)
    } else {
        REJECT
    };
    let color = match (s.hovered, s.disabled) {
        (true, _) => verdict,
        (false, true) => DISABLED_FG,
        (false, false) => s.border.unwrap_or(DISABLED_FG),
    };
    let framed = matches!(s.style, DropZoneStyle::Solid | DropZoneStyle::Dashed);
    if framed {
        let (hz, vt) = if s.style == DropZoneStyle::Solid {
            ('─', '│')
        } else {
            ('╌', '╎')
        };
        for (y, line) in grid.iter_mut().enumerate() {
            for (x, cell) in line.iter_mut().enumerate() {
                let top = y == 0;
                let bottom = y == h - 1;
                let left = x == 0;
                let right = x == w - 1 && w > 1;
                let symbol = match (top, bottom, left, right) {
                    (true, _, true, _) => '┌',
                    (true, _, _, true) => '┐',
                    (_, true, true, _) => '└',
                    (_, true, _, true) => '┘',
                    (true, ..) | (_, true, ..) => hz,
                    (_, _, true, _) | (.., true) => vt,
                    _ => continue,
                };
                cell.0 = symbol;
                cell.1 = Some(color);
            }
        }
    }
    if s.style == DropZoneStyle::Highlight && s.hovered {
        let fill = if s.accept {
            Color::rgb(30, 60, 30)
        } else {
            Color::rgb(60, 30, 30)
        };
        for line in grid.iter_mut() {
            for cell in line.iter_mut() {
                cell.2 = Some(fill);
            }
        }
    }
    if s.style == DropZoneStyle::Minimal {
        let indicator = match (s.hovered, s.accept) {
            (false, _) => '│',
            (true, true) => '▶',
            (true, false) => '✗',
        };
        for line in grid.iter_mut() {
            line[0].0 = indicator;
            line[0].1 = Some(color);
        }
    }
    let text = match (s.hovered, s.accept) {
        (true, true) => "Drop here!",
        (true, false) => "Cannot drop here",
        (false, _) => s.text,
    };
    let text_color = if s.hovered {
        color
    } else if s.disabled {
        DISABLED_FG
    } else {
        s.fg.unwrap_or(LIGHT_GRAY)
    };
    let highlighted = s.hovered && s.style == DropZoneStyle::Highlight;
    for (i, ch) in text.chars().take(w.saturating_sub(4)).enumerate() {
        let cell = &mut grid[h / 2][2 + i];
        cell.0 = ch;
        cell.1 = Some(text_color);
        if !highlighted {
            cell.2 = s.bg;
        }
    }
    if s.focused && !s.disabled {
        for (y, line) in grid.iter_mut().enumerate() {
            for (x, cell) in line.iter_mut().enumerate() {
                cell.3 = if framed {
                    y == 0 || y == h - 1 || x == 0 || x == w - 1
                } else {
                    s.style == DropZoneStyle::Minimal && x == 0
                };
            }
        }
    }
    grid
}

fn cells(buffer: &Buffer) -> Vec<Vec<Cell>> {
    (0..buffer.height())
        .map(|y| {
            (0..buffer.width())
                .map(|x| {
                    let c = buffer.get(x, y).unwrap();
                    (c.symbol, c.fg, c.bg, c.modifier.contains(Modifier::BOLD))
                })
                .collect()
        })
        .collect()
}

proptest! {
    #[test]
    fn every_cell_follows_the_model(s in setup()) {
        let buffer = render(&build(&s), s.width, s.height);
        let (mut actual, mut expected) = (cells(&buffer), model(&s));
        if s.hovered && s.style == DropZoneStyle::Highlight {
            // The verdict's cells lose the highlight fill (#982), so
            // their background is not compared.
            let y = usize::from(s.height / 2);
            for x in 2..usize::from(s.width.saturating_sub(2)) {
                actual[y][x].2 = None;
                expected[y][x].2 = None;
            }
        }
        prop_assert_eq!(actual, expected, "{:?}", s);
    }

    #[test]
    fn the_builders_give_the_same_zone_in_any_order(
        s in setup(),
        order in Just((0..9usize).collect::<Vec<_>>()).prop_shuffle(),
    ) {
        let mut z = DropZone::new(s.text);
        for step in order {
            z = match step {
                0 => z.style(s.style),
                1 => z.focused(s.focused),
                2 => z.disabled(s.disabled),
                3 => match s.min_height { Some(m) => z.min_height(m), None => z },
                4 => match s.fg { Some(c) => z.fg(c), None => z },
                5 => match s.bg { Some(c) => z.bg(c), None => z },
                6 => match s.border { Some(c) => z.border_color(c), None => z },
                7 => match s.hover { Some(c) => z.hover_color(c), None => z },
                _ => z.accepts(&["file"]),
            };
        }
        z.set_hovered(s.hovered, s.accept);
        prop_assert_eq!(z.accepted_types(), &["file"]);
        let expected = render(&build(&s), s.width, s.height);
        prop_assert_eq!(cells(&render(&z, s.width, s.height)), cells(&expected));
    }
}
