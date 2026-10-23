//! Tooltip proptests against plain models: the wrapped lines, the box
//! size, the side and corner it takes; the anchor left free and the arrow
//! next to it; the overlay layer drawing what the inline fallback draws

use proptest::prelude::*;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::text::wrap_words;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{OverlayQueue, Tooltip, TooltipArrow, TooltipPosition, TooltipStyle};

const SIDES: [TooltipPosition; 5] = [
    TooltipPosition::Top,
    TooltipPosition::Bottom,
    TooltipPosition::Left,
    TooltipPosition::Right,
    TooltipPosition::Auto,
];

const STYLES: [TooltipStyle; 7] = [
    TooltipStyle::Plain,
    TooltipStyle::Bordered,
    TooltipStyle::Rounded,
    TooltipStyle::Info,
    TooltipStyle::Warning,
    TooltipStyle::Error,
    TooltipStyle::Success,
];

const ARROWS: [TooltipArrow; 3] = [
    TooltipArrow::None,
    TooltipArrow::Simple,
    TooltipArrow::Unicode,
];

/// Words of many lengths (one longer than most max widths below), wide
/// ones, and "" for blank lines and doubled spaces
const WORDS: [&str; 9] = [
    "a",
    "bb",
    "word",
    "longer",
    "eleven-char",
    "supercalifragilistic",
    "한글",
    "🔄x",
    "",
];

/// Terminal columns of the chars used here
fn width(s: &str) -> usize {
    s.chars()
        .map(|c| {
            if matches!(c, '한' | '글' | '🔄' | '제' | '목') {
                2
            } else {
                1
            }
        })
        .sum()
}

/// Text: words joined by spaces or newlines
fn text() -> impl Strategy<Value = String> {
    prop::collection::vec((0..WORDS.len(), any::<bool>()), 0..10).prop_map(|parts| {
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
        Just(Some("제목".to_string())),
    ]
}

/// The columns a line gets: the max width (0 means 40), but no more than
/// the area leaves inside the padding and border, and at least 1
fn model_wrap_width(max_width: u16, area_width: u16, bordered: bool) -> usize {
    let max = if max_width == 0 {
        40
    } else {
        max_width as usize
    };
    let chrome = if bordered { 4 } else { 2 };
    max.min((area_width as usize).saturating_sub(chrome)).max(1)
}

/// The lines the text wraps to: the shared word wrap (checked in
/// `tests/text`) at the model's width
fn model_lines(text: &str, wrap_width: usize) -> Vec<String> {
    wrap_words(text, wrap_width)
}

/// The box size: the widest line, or the title plus 2, plus padding (and
/// border), no wider than the area unless the widest line (at least one
/// column) does not fit; one row per line, plus the border and the title row
fn model_size(
    lines: &[String],
    title: Option<&str>,
    bordered: bool,
    area_width: u16,
) -> (u16, u16) {
    let widest = lines.iter().map(|l| width(l)).max().unwrap_or(0) as u16;
    let text = widest.max(title.map_or(0, |t| width(t) as u16 + 2));
    let chrome = if bordered { 4 } else { 2 };
    let w = (text + chrome).min(area_width.max(widest.max(1) + chrome));
    let h =
        lines.len() as u16 + if bordered { 2 } else { 0 } + u16::from(title.is_some() && bordered);
    (w, h)
}

/// Whether `side` of the anchor has room for the box and the arrow
fn has_room(
    side: TooltipPosition,
    (ax, ay): (u16, u16),
    (w, h): (u16, u16),
    arrow: u16,
    (aw, ah): (u16, u16),
) -> bool {
    match side {
        TooltipPosition::Top => ay >= h + arrow,
        TooltipPosition::Bottom => ah.saturating_sub(ay.saturating_add(1)) >= h + arrow,
        TooltipPosition::Left => ax >= w + arrow,
        TooltipPosition::Right => aw.saturating_sub(ax.saturating_add(1)) >= w + arrow,
        TooltipPosition::Auto => false,
    }
}

/// The side the box takes: the requested one, or for Auto the first of
/// above, below, right, left with room, else above
fn model_side(
    side: TooltipPosition,
    anchor: (u16, u16),
    size: (u16, u16),
    arrow: u16,
    area: (u16, u16),
) -> TooltipPosition {
    if side != TooltipPosition::Auto {
        return side;
    }
    [
        TooltipPosition::Top,
        TooltipPosition::Bottom,
        TooltipPosition::Right,
        TooltipPosition::Left,
    ]
    .into_iter()
    .find(|&s| has_room(s, anchor, size, arrow, area))
    .unwrap_or(TooltipPosition::Top)
}

/// The box's corner: on `side`, centered across the anchor, one arrow cell
/// away, then pushed back inside the area
fn model_corner(
    side: TooltipPosition,
    (ax, ay): (u16, u16),
    (w, h): (u16, u16),
    arrow: u16,
    (aw, ah): (u16, u16),
) -> (u16, u16) {
    let (x, y) = match side {
        TooltipPosition::Bottom => (ax.saturating_sub(w / 2), ay.saturating_add(1 + arrow)),
        TooltipPosition::Left => (ax.saturating_sub(w + arrow), ay.saturating_sub(h / 2)),
        TooltipPosition::Right => (ax.saturating_add(1 + arrow), ay.saturating_sub(h / 2)),
        _ => (ax.saturating_sub(w / 2), ay.saturating_sub(h + arrow)),
    };
    (x.min(aw.saturating_sub(w)), y.min(ah.saturating_sub(h)))
}

/// The arrow glyph pointing from the box on `side` toward the anchor
fn arrow_glyph(arrow: TooltipArrow, side: TooltipPosition) -> char {
    match (arrow, side) {
        (TooltipArrow::Simple, TooltipPosition::Bottom) => '^',
        (TooltipArrow::Simple, TooltipPosition::Left) => '>',
        (TooltipArrow::Simple, TooltipPosition::Right) => '<',
        (TooltipArrow::Simple, _) => 'v',
        (_, TooltipPosition::Bottom) => '▲',
        (_, TooltipPosition::Left) => '▶',
        (_, TooltipPosition::Right) => '◀',
        _ => '▼',
    }
}

#[derive(Clone, Debug)]
struct Case {
    text: String,
    title: Option<String>,
    max_width: u16,
    style: TooltipStyle,
    side: TooltipPosition,
    arrow: TooltipArrow,
    anchor: (u16, u16),
    area: (u16, u16),
}

impl Case {
    fn tooltip(&self) -> Tooltip {
        let mut t = Tooltip::new(self.text.clone())
            .max_width(self.max_width)
            .style(self.style)
            .position(self.side)
            .arrow(self.arrow)
            .anchor(self.anchor.0, self.anchor.1);
        if let Some(title) = &self.title {
            t = t.title(title.clone());
        }
        t
    }

    fn bordered(&self) -> bool {
        self.style != TooltipStyle::Plain
    }

    fn arrow_cells(&self) -> u16 {
        u16::from(self.arrow != TooltipArrow::None)
    }

    fn wrap_width(&self) -> usize {
        model_wrap_width(self.max_width, self.area.0, self.bordered())
    }

    fn lines(&self) -> Vec<String> {
        model_lines(&self.text, self.wrap_width())
    }

    fn size(&self) -> (u16, u16) {
        model_size(
            &self.lines(),
            self.title.as_deref(),
            self.bordered(),
            self.area.0,
        )
    }

    fn side_taken(&self) -> TooltipPosition {
        model_side(
            self.side,
            self.anchor,
            self.size(),
            self.arrow_cells(),
            self.area,
        )
    }

    fn render(&self) -> Buffer {
        let (aw, ah) = self.area;
        let mut buffer = Buffer::new(aw, ah);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, aw, ah));
        self.tooltip().render(&mut ctx);
        buffer
    }
}

/// An anchor coordinate below `near`, or with `far` sometimes anywhere up
/// to `u16::MAX` and sometimes within 2 of it, where adding the arrow
/// overflowed (#1054)
fn anchor(near: u16, far: bool) -> BoxedStrategy<u16> {
    if far {
        prop_oneof![18 => 0..near, 1 => near..=u16::MAX, 1 => u16::MAX - 2..=u16::MAX].boxed()
    } else {
        (0..near).boxed()
    }
}

/// Cases in areas of `widths` x `heights`, the anchor up to `past` cells
/// past the area, and, when `past` is not 0, sometimes far past it
fn cases(
    widths: std::ops::Range<u16>,
    heights: impl Strategy<Value = u16>,
    past: u16,
) -> impl Strategy<Value = Case> {
    (widths, heights)
        .prop_flat_map(move |(aw, ah)| {
            (
                Just((aw, ah)),
                anchor(aw + past, past > 0),
                anchor(ah + past, past > 0),
            )
        })
        .prop_flat_map(|(area, ax, ay)| {
            (
                (text(), title(), 0u16..25),
                (0..STYLES.len(), 0..SIDES.len(), 0..ARROWS.len()),
                Just(area),
                Just((ax, ay)),
            )
        })
        .prop_map(|((text, title, max_width), (s, p, a), area, anchor)| Case {
            text,
            title,
            max_width,
            style: STYLES[s],
            side: SIDES[p],
            arrow: ARROWS[a],
            anchor,
            area,
        })
}

/// Any area, the anchor inside it, up to 2 cells past it, or anywhere up
/// to `u16::MAX` (one case in ten). Areas of 1 to
/// 4 rows are half the cases, so Auto often finds no room above or below
/// and has to choose between right and left.
fn case() -> impl Strategy<Value = Case> {
    cases(1..45, prop_oneof![1u16..5, 5u16..16], 3)
}

proptest! {
    // Two properties hold only for some cases (room on the side, a box that
    // fits): allow the rejects that 10,000 cases bring.
    #![proptest_config(ProptestConfig {
        max_global_rejects: 100_000,
        ..ProptestConfig::default()
    })]

    /// The cells with a background are exactly the model's box, cut to the
    /// area; a bordered box that fits has its corners where the model puts
    /// them, and the first line starts inside the padding
    #[test]
    fn prop_box_matches_the_model(c in case()) {
        let buffer = c.render();
        let (w, h) = c.size();
        let (x, y) = model_corner(c.side_taken(), c.anchor, (w, h), c.arrow_cells(), c.area);
        let (aw, ah) = c.area;
        for cy in 0..ah {
            for cx in 0..aw {
                let inside = (x..x + w).contains(&cx) && (y..y + h).contains(&cy);
                prop_assert_eq!(buffer.get(cx, cy).unwrap().bg.is_some(), inside, "({}, {})", cx, cy);
            }
        }
        if c.bordered() && w <= aw && h <= ah {
            let corner = buffer.get(x, y).unwrap().symbol;
            prop_assert!(matches!(corner, '┌' | '╭'), "corner {:?}", corner);
            let corner = buffer.get(x + w - 1, y + h - 1).unwrap().symbol;
            prop_assert!(matches!(corner, '┘' | '╯'), "corner {:?}", corner);
        }
        let lines = c.lines();
        let first = lines[0].chars().find(|ch| !ch.is_whitespace());
        let lead = lines[0].chars().take_while(|ch| ch.is_whitespace()).count() as u16;
        let tx = x + if c.bordered() { 2 } else { 1 } + lead;
        let ty = y + if c.bordered() { 1 + u16::from(c.title.is_some()) } else { 0 };
        if let Some(first) = first {
            if tx < aw && ty < ah && w <= aw {
                prop_assert_eq!(buffer.get(tx, ty).unwrap().symbol, first);
            }
        }
    }

    /// Every line fits the width the box gives it (a single glyph wider
    /// than that has to go somewhere), and every word is kept, in order
    #[test]
    fn prop_lines_fit_and_keep_every_word(c in case()) {
        let lines = c.lines();
        for line in &lines {
            prop_assert!(
                width(line) <= c.wrap_width() || line.chars().count() == 1,
                "{:?} wider than {}", line, c.wrap_width()
            );
        }
        let kept: String = lines.concat().split_whitespace().collect();
        let text: String = c.text.split_whitespace().collect();
        prop_assert_eq!(kept, text);
    }

    /// With room on the side the box takes, the anchor's cell stays free
    /// and the arrow is in the cell next to it, pointing at it
    #[test]
    fn prop_the_anchor_stays_free_and_the_arrow_points_at_it(c in cases(16..60, 8u16..24, 0)) {
        let (ax, ay) = c.anchor;
        let side = c.side_taken();
        // Without room the box is pushed over the anchor: #1045.
        prop_assume!(has_room(side, c.anchor, c.size(), c.arrow_cells(), c.area));
        let buffer = c.render();
        let anchor = buffer.get(ax, ay).unwrap();
        prop_assert_eq!(anchor.symbol, ' ');
        prop_assert_eq!(anchor.bg, None);
        if c.arrow != TooltipArrow::None {
            let (x, y) = match side {
                TooltipPosition::Bottom => (ax, ay + 1),
                TooltipPosition::Left => (ax - 1, ay),
                TooltipPosition::Right => (ax + 1, ay),
                _ => (ax, ay - 1),
            };
            prop_assert_eq!(buffer.get(x, y).unwrap().symbol, arrow_glyph(c.arrow, side));
        }
    }

    /// A box that fits the area is drawn whole, whatever the anchor
    #[test]
    fn prop_a_box_that_fits_stays_inside_the_area(c in case()) {
        let (w, h) = c.size();
        prop_assume!(w <= c.area.0 && h <= c.area.1);
        let buffer = c.render();
        let n = (0..c.area.1)
            .flat_map(|y| (0..c.area.0).map(move |x| (x, y)))
            .filter(|&(x, y)| buffer.get(x, y).unwrap().bg.is_some())
            .count();
        prop_assert_eq!(n, (w * h) as usize);
    }

    /// At the origin the overlay layer draws the same cells as the inline
    /// fallback. (Not in an offset area: there the overlay ignores the
    /// area's position, #1053.)
    #[test]
    fn prop_overlay_at_the_origin_matches_inline(c in case()) {
        let inline = c.render();
        let (aw, ah) = c.area;
        let mut overlay = Buffer::new(aw, ah);
        let mut queue = OverlayQueue::new();
        {
            let mut ctx = RenderContext::new(&mut overlay, Rect::new(0, 0, aw, ah))
                .with_overlay_queue(&mut queue);
            c.tooltip().render(&mut ctx);
        }
        queue.render_to(&mut overlay);
        for y in 0..ah {
            for x in 0..aw {
                prop_assert_eq!(inline.get(x, y), overlay.get(x, y), "({}, {})", x, y);
            }
        }
    }
}
