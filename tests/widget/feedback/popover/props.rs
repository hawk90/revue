//! Popover proptests against plain models: the wrapped lines and the box
//! size, the side and corner the box takes, clicks against the cells the box
//! covers, and the open state under keys and calls

use proptest::prelude::*;
use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::text::wrap_words;
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

/// The columns a line gets: the max width (0 means 40), but no more than
/// the area leaves inside the border and padding, and at least 1
fn model_wrap_width(max_width: u16, area_width: u16) -> usize {
    let max = if max_width == 0 {
        40
    } else {
        max_width as usize
    };
    max.min((area_width as usize).saturating_sub(4)).max(1)
}

/// The lines the content wraps to: the shared word wrap (checked in
/// `tests/text`) at the model's width
fn model_lines(content: &str, wrap_width: usize) -> Vec<String> {
    wrap_words(content, wrap_width)
}

/// The box size: the widest line or the title plus 2, plus border and
/// padding, at least 10 but no wider than the area (unless the widest
/// line, at least one column, does not fit); one row per line, the border and the title row,
/// at least 3
fn model_size(lines: &[String], title: Option<&str>, area_width: u16) -> (u16, u16) {
    let widest = lines.iter().map(|l| l.len()).max().unwrap_or(0) as u16;
    let text = widest.max(title.map_or(0, |t| t.len() as u16 + 2));
    let w = (text + 4).max(10).min(area_width.max(widest.max(1) + 4));
    let h = lines.len() as u16 + 2 + u16::from(title.is_some());
    (w, h.max(3))
}

/// The cells free on `side` of the anchor past the arrow, in the area
fn room(side: PopoverPosition, (ax, ay): (u16, u16), arrow: u16, (aw, ah): (u16, u16)) -> u16 {
    match side {
        PopoverPosition::Top => ay.saturating_sub(arrow).min(ah),
        PopoverPosition::Bottom => ah.saturating_sub(ay.saturating_add(1 + arrow)),
        PopoverPosition::Left => ax.saturating_sub(arrow).min(aw),
        PopoverPosition::Right => aw.saturating_sub(ax.saturating_add(1 + arrow)),
        PopoverPosition::Auto => unreachable!(),
    }
}

fn opposite(side: PopoverPosition) -> PopoverPosition {
    match side {
        PopoverPosition::Top => PopoverPosition::Bottom,
        PopoverPosition::Bottom => PopoverPosition::Top,
        PopoverPosition::Left => PopoverPosition::Right,
        PopoverPosition::Right => PopoverPosition::Left,
        PopoverPosition::Auto => unreachable!(),
    }
}

/// The rows (above or below) or columns (left or right) a `(w, h)` block
/// takes
fn depth(side: PopoverPosition, (w, h): (u16, u16)) -> u16 {
    if matches!(side, PopoverPosition::Left | PopoverPosition::Right) {
        w
    } else {
        h
    }
}

/// Whether `side` has room for the whole `(w, h)` block past the arrow
fn has_room(
    side: PopoverPosition,
    anchor: (u16, u16),
    block: (u16, u16),
    arrow: u16,
    area: (u16, u16),
) -> bool {
    room(side, anchor, arrow, area) >= depth(side, block)
}

/// The side the block (the box, with an elevated box's shadow) takes: the
/// first of the requested side and its opposite (for Auto: below, above,
/// right, left) with room for it, else the roomier of the first two (the
/// first on a tie)
fn model_side(
    side: PopoverPosition,
    anchor: (u16, u16),
    block: (u16, u16),
    arrow: u16,
    area: (u16, u16),
) -> PopoverPosition {
    let order = if side == PopoverPosition::Auto {
        vec![
            PopoverPosition::Bottom,
            PopoverPosition::Top,
            PopoverPosition::Right,
            PopoverPosition::Left,
        ]
    } else {
        vec![side, opposite(side)]
    };
    order
        .iter()
        .copied()
        .find(|&s| has_room(s, anchor, block, arrow, area))
        .unwrap_or_else(|| {
            let (first, other) = (order[0], opposite(order[0]));
            if room(other, anchor, arrow, area) > room(first, anchor, arrow, area) {
                other
            } else {
                first
            }
        })
}

/// The box drawn, `(x, y, w, h)`: the block on `side`, one arrow cell from
/// the anchor, shortened to the room there, centered across the anchor and
/// shifted inside the area along the side, less the shadow; `None` when no
/// box is left
fn model_box(
    side: PopoverPosition,
    (ax, ay): (u16, u16),
    (w, h): (u16, u16),
    shadow: u16,
    arrow: u16,
    (aw, ah): (u16, u16),
) -> Option<(u16, u16, u16, u16)> {
    let (bw, bh) = (w + shadow, h + shadow);
    let d = depth(side, (bw, bh)).min(room(side, (ax, ay), arrow, (aw, ah)));
    if d <= shadow {
        return None;
    }
    let shift = |at: u16, size: u16, edge: u16| at.min(edge.saturating_sub(size));
    let (x, y, bw, bh) = match side {
        PopoverPosition::Bottom => (
            shift(ax.saturating_sub(bw / 2), bw, aw),
            ay + 1 + arrow,
            bw,
            d,
        ),
        PopoverPosition::Right => (
            ax + 1 + arrow,
            shift(ay.saturating_sub(bh / 2), bh, ah),
            d,
            bh,
        ),
        PopoverPosition::Left => (
            ax.saturating_sub(arrow).min(aw) - d,
            shift(ay.saturating_sub(bh / 2), bh, ah),
            d,
            bh,
        ),
        _ => (
            shift(ax.saturating_sub(bw / 2), bw, aw),
            ay.saturating_sub(arrow).min(ah) - d,
            bw,
            d,
        ),
    };
    Some((x, y, bw - shadow, bh - shadow))
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
    /// The box on the screen is the model's: on the side it takes, shortened
    /// to the room there, with its lines; the anchor is uncovered and the
    /// arrow next to it
    #[test]
    fn prop_box_matches_the_model(
        text in content(),
        title in title(),
        max_width in prop::sample::select(vec![0u16, 1, 5, 8, 12, 40]),
        style in prop::sample::select(BORDERED.to_vec()),
        arrow in prop::sample::select(vec![PopoverArrow::None, PopoverArrow::Simple, PopoverArrow::Unicode]),
        side in prop::sample::select(SIDES.to_vec()),
        (area_w, extra_h) in (5u16..60, 0u16..20),
        (fx, fy) in (0.0f64..1.0, 0.0f64..1.0),
    ) {
        let wrap_width = model_wrap_width(max_width, area_w);
        let lines = model_lines(&text, wrap_width);
        for line in &lines {
            // A word wider than the line is broken by columns
            prop_assert!(line.len() <= wrap_width, "{:?} wider than {}", line, wrap_width);
        }
        let kept: String = lines.concat().split_whitespace().collect();
        prop_assert_eq!(kept, text.split_whitespace().collect::<String>());
        let size = model_size(&lines, title.as_deref(), area_w);
        let area = (area_w, size.1 + extra_h);
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
        let shadow = u16::from(style == PopoverStyle::Elevated);
        let block = (size.0 + shadow, size.1 + shadow);
        let taken = model_side(side, anchor, block, arrow_len, area);
        let Some((x, y, w, h)) = model_box(taken, anchor, size, shadow, arrow_len, area) else {
            prop_assert!((0..area.1).all(|cy| (0..area.0).all(|cx| symbol(&b, cx, cy) == ' ')));
            return Ok(());
        };

        // The anchor is never covered, by the box or its shadow (#1045), and
        // the arrow is next to it
        let (ax, ay) = anchor;
        prop_assert_eq!(b.get(ax, ay).unwrap(), &revue::render::Cell::default());
        if arrow != PopoverArrow::None {
            let at = match taken {
                PopoverPosition::Top => (ax, ay - 1),
                PopoverPosition::Bottom => (ax, ay + 1),
                PopoverPosition::Left => (ax - 1, ay),
                PopoverPosition::Right => (ax + 1, ay),
                PopoverPosition::Auto => unreachable!(),
            };
            prop_assert_eq!(symbol(&b, at.0, at.1), arrow_char(arrow, taken));
        }

        // A box shortened to fewer rows (or columns) than its border, title
        // and lines need is checked for its corner only
        let drawn = drawn_box(&b);
        if (w, h) != size || h < 2 {
            // (a box one cell wide or high has no corners to find)
            if w >= 2 && h >= 2 {
                prop_assert_eq!(drawn.map(|d| (d.0, d.1, d.3)), Some((x, y, h)));
            }
            return Ok(());
        }
        let (dx, dy, dw, dh) = drawn.expect("a whole box");
        prop_assert_eq!((dx, dy, dh), (x, y, h));
        prop_assert_eq!(dw, w);
        prop_assert!(x + w <= area.0 && y + h <= area.1);
        let first = y + 1 + u16::from(title.is_some());
        if let Some(t) = &title {
            // A title wider than the box is cut at its padding
            let room = (w - 4) as usize;
            let shown: String = (x + 2..x + w - 2).map(|cx| symbol(&b, cx, y + 1)).collect();
            prop_assert_eq!(shown.trim_end(), t[..t.len().min(room)].trim_end());
        }
        for (i, line) in lines.iter().enumerate() {
            let shown: String = (x + 2..x + w - 2).map(|cx| symbol(&b, cx, first + i as u16)).collect();
            prop_assert_eq!(shown.trim_end(), line.trim_end(), "line {}", i);
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
