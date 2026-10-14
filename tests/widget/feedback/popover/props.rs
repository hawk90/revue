//! Popover proptests against plain models: the wrapped lines and the box
//! size, the side and corner the box takes, clicks against the cells the box
//! covers, and the open state under keys and calls

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Popover, PopoverArrow, PopoverPosition, PopoverStyle, PopoverTrigger};

const SIDES: [PopoverPosition; 5] = [
    PopoverPosition::Top,
    PopoverPosition::Bottom,
    PopoverPosition::Left,
    PopoverPosition::Right,
    PopoverPosition::Auto,
];

const BORDERED: [PopoverStyle; 3] = [
    PopoverStyle::Default,
    PopoverStyle::Rounded,
    PopoverStyle::Elevated,
];

/// Words of many lengths, one longer than every max width below, so lines
/// fit, wrap, and overflow
const WORDS: [&str; 8] = [
    "a",
    "bb",
    "ccc",
    "word",
    "longer",
    "eleven-char",
    "supercalifragilistic",
    "",
];

/// Content: words joined by spaces or newlines ("" makes blank lines)
fn content() -> impl Strategy<Value = String> {
    prop::collection::vec((0..WORDS.len(), any::<bool>()), 0..12).prop_map(|parts| {
        let mut s = String::new();
        for (i, (w, newline)) in parts.iter().enumerate() {
            if i > 0 {
                s.push(if *newline { '\n' } else { ' ' });
            }
            s.push_str(WORDS[*w]);
        }
        s
    })
}

fn title() -> impl Strategy<Value = Option<String>> {
    prop_oneof![
        Just(None),
        Just(Some("T".to_string())),
        Just(Some("A rather long title".to_string())),
    ]
}

/// The lines the content should wrap to: a line that fits stays as it is,
/// a longer one is filled word by word
fn model_lines(content: &str, max_width: u16) -> Vec<String> {
    let max = if max_width == 0 {
        40
    } else {
        max_width as usize
    };
    let mut lines = Vec::new();
    for line in content.lines() {
        if line.len() <= max {
            lines.push(line.to_string());
            continue;
        }
        let mut current = String::new();
        for word in line.split_whitespace() {
            if current.is_empty() {
                current = word.to_string();
            } else if current.len() + 1 + word.len() <= max {
                current = format!("{current} {word}");
            } else {
                lines.push(std::mem::replace(&mut current, word.to_string()));
            }
        }
        if !current.is_empty() {
            lines.push(current);
        }
    }
    if lines.is_empty() {
        lines.push(String::new());
    }
    lines
}

/// The box size: the widest line or the title plus 2, plus border and
/// padding; one row per line, the border and the title row; at least 10 x 3
fn model_size(lines: &[String], title: Option<&str>, bordered: bool) -> (u16, u16) {
    let text = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let text = text.max(title.map_or(0, |t| t.len() + 2)) as u16;
    let w = text + if bordered { 4 } else { 2 };
    let h =
        lines.len() as u16 + if bordered { 2 } else { 0 } + u16::from(title.is_some() && bordered);
    (w.max(10), h.max(3))
}

/// The side the box takes: the requested one, or for Auto the first of
/// below, above, right, left with room, else below
fn model_side(
    side: PopoverPosition,
    (ax, ay): (u16, u16),
    (w, h): (u16, u16),
    arrow: u16,
    (aw, ah): (u16, u16),
) -> PopoverPosition {
    if side != PopoverPosition::Auto {
        return side;
    }
    if ah.saturating_sub(ay + 1) >= h + arrow {
        PopoverPosition::Bottom
    } else if ay >= h + arrow {
        PopoverPosition::Top
    } else if aw.saturating_sub(ax + 1) >= w + arrow {
        PopoverPosition::Right
    } else if ax >= w + arrow {
        PopoverPosition::Left
    } else {
        PopoverPosition::Bottom
    }
}

/// Whether `side` of the anchor has room for the box and the arrow
fn has_room(
    side: PopoverPosition,
    (ax, ay): (u16, u16),
    (w, h): (u16, u16),
    arrow: u16,
    (aw, ah): (u16, u16),
) -> bool {
    match side {
        PopoverPosition::Top => ay >= h + arrow,
        PopoverPosition::Bottom => ah.saturating_sub(ay + 1) >= h + arrow,
        PopoverPosition::Left => ax >= w + arrow,
        PopoverPosition::Right => aw.saturating_sub(ax + 1) >= w + arrow,
        PopoverPosition::Auto => unreachable!(),
    }
}

/// The box's corner on a side with room: centered on the anchor across the
/// side and pushed back into the area, one cell (two with an arrow) away
/// from the anchor along it
fn model_corner(
    side: PopoverPosition,
    (ax, ay): (u16, u16),
    (w, h): (u16, u16),
    arrow: u16,
    (aw, ah): (u16, u16),
) -> (u16, u16) {
    let (x, y) = match side {
        PopoverPosition::Top => (ax.saturating_sub(w / 2), ay - h - arrow),
        PopoverPosition::Bottom => (ax.saturating_sub(w / 2), ay + 1 + arrow),
        PopoverPosition::Left => (ax - w - arrow, ay.saturating_sub(h / 2)),
        PopoverPosition::Right => (ax + 1 + arrow, ay.saturating_sub(h / 2)),
        PopoverPosition::Auto => unreachable!(),
    };
    (x.min(aw - w), y.min(ah - h))
}

fn render_in(p: &Popover, aw: u16, ah: u16) -> Buffer {
    let mut buffer = Buffer::new(aw, ah);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, aw, ah));
    p.render(&mut ctx);
    buffer
}

fn symbol(b: &Buffer, x: u16, y: u16) -> char {
    b.get(x, y).map_or(' ', |c| c.symbol)
}

/// The box read off the screen: corner, width and height of its border
fn drawn_box(b: &Buffer) -> Option<(u16, u16, u16, u16)> {
    let (x, y) = (0..b.height()).find_map(|y| {
        (0..b.width())
            .find(|&x| matches!(symbol(b, x, y), '┌' | '╭'))
            .map(|x| (x, y))
    })?;
    let w = (x + 1..b.width()).find(|&cx| matches!(symbol(b, cx, y), '┐' | '╮'))? - x + 1;
    let h = (y + 1..b.height()).find(|&cy| matches!(symbol(b, x, cy), '└' | '╰'))? - y + 1;
    Some((x, y, w, h))
}

fn arrow_char(arrow: PopoverArrow, side: PopoverPosition) -> char {
    match (arrow, side) {
        (PopoverArrow::Simple, PopoverPosition::Top) => 'v',
        (PopoverArrow::Simple, PopoverPosition::Bottom) => '^',
        (PopoverArrow::Simple, PopoverPosition::Left) => '>',
        (PopoverArrow::Simple, PopoverPosition::Right) => '<',
        (PopoverArrow::Unicode, PopoverPosition::Top) => '▼',
        (PopoverArrow::Unicode, PopoverPosition::Bottom) => '▲',
        (PopoverArrow::Unicode, PopoverPosition::Left) => '▶',
        (PopoverArrow::Unicode, PopoverPosition::Right) => '◀',
        _ => unreachable!(),
    }
}

proptest! {
    /// The box on the screen has the model's size and lines, lies inside the
    /// area, and, where its side has room, sits at the model's corner with
    /// the arrow next to the anchor and the anchor uncovered
    #[test]
    fn prop_box_matches_the_model(
        text in content(),
        title in title(),
        max_width in prop::sample::select(vec![0u16, 1, 5, 8, 12, 40]),
        style in prop::sample::select(BORDERED.to_vec()),
        arrow in prop::sample::select(vec![PopoverArrow::None, PopoverArrow::Simple, PopoverArrow::Unicode]),
        side in prop::sample::select(SIDES.to_vec()),
        (extra_w, extra_h) in (0u16..40, 0u16..20),
        (fx, fy) in (0.0f64..1.0, 0.0f64..1.0),
    ) {
        let lines = model_lines(&text, max_width);
        let size = model_size(&lines, title.as_deref(), true);
        let area = (size.0 + extra_w, size.1 + extra_h);
        let anchor = ((fx * area.0 as f64) as u16, (fy * area.1 as f64) as u16);
        let mut p = Popover::new(text.clone())
            .max_width(max_width)
            .popover_style(style)
            .arrow(arrow)
            .position(side)
            .anchor(anchor.0, anchor.1)
            .open(true);
        if let Some(t) = &title {
            p = p.title(t.clone());
        }
        let b = render_in(&p, area.0, area.1);
        let arrow_len = u16::from(arrow != PopoverArrow::None);
        let taken = model_side(side, anchor, size, arrow_len, area);
        let room = has_room(taken, anchor, size, arrow_len, area);
        // Without room on its side the box is pushed over the anchor and the
        // arrow can land on the box's border (draft-po-1); with an arrow only
        // the sides with room are checked
        if !room && arrow != PopoverArrow::None {
            return Ok(());
        }

        let (x, y, w, h) = drawn_box(&b).expect("a whole box");
        prop_assert_eq!((w, h), size);
        prop_assert!(x + w <= area.0 && y + h <= area.1);
        let first = y + 1 + u16::from(title.is_some());
        if let Some(t) = &title {
            let shown: String = (x + 2..x + 2 + t.len() as u16).map(|cx| symbol(&b, cx, y + 1)).collect();
            prop_assert_eq!(&shown, t);
        }
        for (i, line) in lines.iter().enumerate() {
            let shown: String = (x + 2..x + w - 2).map(|cx| symbol(&b, cx, first + i as u16)).collect();
            prop_assert_eq!(shown.trim_end(), line.trim_end(), "line {}", i);
        }

        // and where the box goes without room is draft-po-1
        if !room {
            return Ok(());
        }
        prop_assert_eq!((x, y), model_corner(taken, anchor, size, arrow_len, area));
        let covers_anchor = (x..x + w).contains(&anchor.0) && (y..y + h).contains(&anchor.1);
        prop_assert!(!covers_anchor);
        if arrow != PopoverArrow::None {
            let (ax, ay) = anchor;
            let at = match taken {
                PopoverPosition::Top => (ax, ay - 1),
                PopoverPosition::Bottom => (ax, ay + 1),
                PopoverPosition::Left => (ax - 1, ay),
                PopoverPosition::Right => (ax + 1, ay),
                PopoverPosition::Auto => unreachable!(),
            };
            prop_assert_eq!(symbol(&b, at.0, at.1), arrow_char(arrow, taken));
        }
    }

    /// A click inside the cells the box covers keeps it open; one outside
    /// closes it exactly when close_on_click_outside is on. A closed popover
    /// opens only on a click on the anchor with the Click trigger.
    #[test]
    fn prop_clicks_follow_the_drawn_box(
        text in content(),
        style in prop::sample::select(vec![
            PopoverStyle::Default,
            PopoverStyle::Rounded,
            PopoverStyle::Minimal,
            PopoverStyle::Elevated,
        ]),
        arrow in prop::sample::select(vec![PopoverArrow::None, PopoverArrow::Unicode]),
        side in prop::sample::select(SIDES.to_vec()),
        trigger in prop::sample::select(vec![
            PopoverTrigger::Click,
            PopoverTrigger::Hover,
            PopoverTrigger::Focus,
            PopoverTrigger::Manual,
        ]),
        open in any::<bool>(),
        close_outside in any::<bool>(),
        (aw, ah) in (1u16..60, 1u16..30),
        anchor in (0u16..60, 0u16..30),
        click in (0.0f64..1.0, 0.0f64..1.0),
        on_anchor in any::<bool>(),
    ) {
        let p = Popover::new(text)
            .popover_style(style)
            .arrow(arrow)
            .position(side)
            .trigger(trigger)
            .close_on_click_outside(close_outside)
            .bg(Color::rgb(1, 2, 3))
            .anchor(anchor.0, anchor.1)
            .open(open);
        // Clicks land inside the area, where the screen shows what is there
        let click = ((click.0 * aw as f64) as u16, (click.1 * ah as f64) as u16);
        let (cx, cy) = if on_anchor { anchor } else { click };
        let mut q = p.clone();
        let handled = q.handle_click(cx, cy, aw, ah);
        if !open {
            let opens = trigger == PopoverTrigger::Click && (cx, cy) == anchor;
            prop_assert_eq!(handled, opens);
            prop_assert_eq!(q.is_open(), opens);
            return Ok(());
        }
        // Off the area nothing is drawn to compare with
        if cx >= aw || cy >= ah {
            return Ok(());
        }
        // The box's cells are the ones painted with its background
        let b = render_in(&p, aw, ah);
        // Without room the arrow can be drawn over the box (draft-po-1)
        if matches!(symbol(&b, cx, cy), '▲' | '▼' | '◀' | '▶') {
            return Ok(());
        }
        let inside = b.get(cx, cy).is_some_and(|c| c.bg == Some(Color::rgb(1, 2, 3)));
        if inside {
            prop_assert!(handled);
            prop_assert!(q.is_open());
        } else {
            prop_assert_eq!(handled, close_outside);
            prop_assert_eq!(q.is_open(), !close_outside);
        }
    }

    /// show, hide, toggle and keys against a plain open flag; the popover
    /// draws something exactly when it is open
    #[test]
    fn prop_open_state_matches_the_model(
        start in any::<bool>(),
        close_on_escape in any::<bool>(),
        ops in prop::collection::vec(0u8..7, 0..24),
    ) {
        let mut p = Popover::new("Test")
            .anchor(10, 4)
            .close_on_escape(close_on_escape)
            .open(start);
        let mut open = start;
        for op in ops {
            match op {
                0 => { p.show(); open = true; }
                1 => { p.hide(); open = false; }
                2 => { p.toggle(); open = !open; }
                3 => {
                    let closes = open && close_on_escape;
                    prop_assert_eq!(p.handle_key(&Key::Escape), closes);
                    if closes { open = false; }
                }
                4 => prop_assert!(!p.handle_key(&Key::Enter)),
                5 => prop_assert!(!p.handle_key(&Key::Char('q'))),
                _ => p.set_anchor(12, 6),
            }
            prop_assert_eq!(p.is_open(), open);
        }
        let b = render_in(&p, 30, 12);
        let drawn = (0..12).any(|y| (0..30).any(|x| symbol(&b, x, y) != ' '));
        prop_assert_eq!(drawn, open);
    }
}
