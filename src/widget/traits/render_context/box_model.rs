//! Applying a node's CSS box properties to the area its parent gave it.
//!
//! Container widgets compute their children's geometry themselves - that is the
//! whole point of a builder-first widget library, and it is what makes
//! `vstack().child(..)` mean anything without a stylesheet. CSS layout
//! properties therefore act as an **override** on top of that geometry rather
//! than replacing it: the container decides the flow, the stylesheet adjusts
//! the box.
//!
//! "Specified" is read off the value itself. The cascade merges into a `Style`
//! full of defaults, so `Size::Auto` means *not specified* and a zero margin
//! means *not specified*. That is the same test the cascade already uses to
//! decide whether one rule overrides another.
//!
//! What this deliberately does **not** apply:
//!
//! - `padding` - it insets a widget's *content*, and a widget that draws its
//!   own border (`Border`, `Card`) paints on the outer box. Shrinking the area
//!   here would move the border inward instead of the content.
//! - `gap`, `flex-*`, `grid-*` - flow properties. They belong to the container,
//!   which is the only thing that knows how its children are arranged.

use crate::layout::Rect;
use crate::style::{Size, Style};

/// Apply the specified box properties of `style` to `area`.
///
/// Order follows CSS: margin insets the box, then an explicit size replaces the
/// resulting dimension, then min/max clamp it. The origin stays where the
/// container put it - an explicit size shrinks toward the top-left rather than
/// re-centering, because re-centering is an alignment decision and alignment
/// belongs to the container.
pub(crate) fn apply(style: &Style, area: Rect) -> Rect {
    let margin = style.spacing.margin;

    let x = area.x.saturating_add(margin.left);
    let y = area.y.saturating_add(margin.top);
    let mut width = area
        .width
        .saturating_sub(margin.left)
        .saturating_sub(margin.right);
    let mut height = area
        .height
        .saturating_sub(margin.top)
        .saturating_sub(margin.bottom);

    // The percentage basis is the area the container offered, before margins -
    // the same basis the container itself used.
    if let Some(w) = resolve(style.sizing.width, area.width) {
        width = w;
    }
    if let Some(h) = resolve(style.sizing.height, area.height) {
        height = h;
    }

    width = clamp(
        width,
        resolve(style.sizing.min_width, area.width),
        resolve(style.sizing.max_width, area.width),
    );
    height = clamp(
        height,
        resolve(style.sizing.min_height, area.height),
        resolve(style.sizing.max_height, area.height),
    );

    Rect::new(x, y, width, height)
}

/// Does `style` specify anything this function would act on?
///
/// Lets the caller skip the work - and, more importantly, leave the area
/// untouched - for the overwhelmingly common node that styles only paint
/// properties.
pub(crate) fn specifies_anything(style: &Style) -> bool {
    let s = &style.sizing;
    style.spacing.margin != crate::style::Spacing::default()
        || s.width != Size::Auto
        || s.height != Size::Auto
        || s.min_width != Size::Auto
        || s.max_width != Size::Auto
        || s.min_height != Size::Auto
        || s.max_height != Size::Auto
}

/// `None` for `Size::Auto`, which is the cascade's "not specified".
fn resolve(size: Size, basis: u16) -> Option<u16> {
    match size {
        Size::Auto => None,
        Size::Fixed(v) => Some(v),
        Size::Percent(pct) => {
            Some(((basis as f32) * pct / 100.0).clamp(0.0, u16::MAX as f32) as u16)
        }
    }
}

fn clamp(value: u16, min: Option<u16>, max: Option<u16>) -> u16 {
    let value = max.map_or(value, |m| value.min(m));
    min.map_or(value, |m| value.max(m))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn area() -> Rect {
        Rect::new(10, 20, 40, 12)
    }

    #[test]
    fn an_unstyled_node_keeps_its_area() {
        let style = Style::default();
        assert!(!specifies_anything(&style));
        assert_eq!(apply(&style, area()), area());
    }

    #[test]
    fn a_fixed_size_replaces_the_dimension() {
        let mut style = Style::default();
        style.sizing.width = Size::Fixed(7);
        assert_eq!(apply(&style, area()), Rect::new(10, 20, 7, 12));
    }

    #[test]
    fn a_percentage_resolves_against_the_offered_area() {
        let mut style = Style::default();
        style.sizing.height = Size::Percent(50.0);
        assert_eq!(apply(&style, area()), Rect::new(10, 20, 40, 6));
    }

    #[test]
    fn a_margin_insets_the_box() {
        let mut style = Style::default();
        style.spacing.margin = crate::style::Spacing::all(2);
        assert_eq!(apply(&style, area()), Rect::new(12, 22, 36, 8));
    }

    #[test]
    fn min_and_max_clamp_the_result() {
        let mut style = Style::default();
        style.sizing.width = Size::Fixed(100);
        style.sizing.max_width = Size::Fixed(30);
        style.sizing.min_height = Size::Fixed(20);
        assert_eq!(apply(&style, area()), Rect::new(10, 20, 30, 20));
    }

    /// A margin wider than the area must not wrap around.
    #[test]
    fn an_oversized_margin_saturates_to_zero() {
        let mut style = Style::default();
        style.spacing.margin = crate::style::Spacing::all(50);
        let r = apply(&style, area());
        assert_eq!((r.width, r.height), (0, 0));
    }

    #[test]
    fn padding_alone_specifies_nothing() {
        let mut style = Style::default();
        style.spacing.padding = crate::style::Spacing::all(3);
        assert!(
            !specifies_anything(&style),
            "padding is the container's business - see the module docs"
        );
    }
}
