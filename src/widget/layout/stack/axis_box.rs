//! A child's CSS box along one axis: its explicit size, bounds and margins

use crate::style::{Size, Style};
use crate::widget::traits::render_context::box_model;

/// What a child's CSS box says along one axis - see
/// [`Stack::content_size`](super::Stack::content_size). Empty for a child
/// with no box properties.
#[derive(Default)]
pub(super) struct AxisBox {
    /// Explicit `height` / `width`
    pub(super) size: Option<u16>,
    /// `min-height` / `min-width`
    pub(super) min: Option<u16>,
    /// `max-height` / `max-width`
    pub(super) max: Option<u16>,
    /// Both margins on the axis
    pub(super) margins: u16,
    /// Any of the three sizes is a percentage
    pub(super) percent: bool,
}

impl AxisBox {
    /// The box along the vertical axis if `vertical`, else the horizontal one.
    pub(super) fn of(style: Option<&Style>, vertical: bool) -> Self {
        let Some(style) = style.filter(|s| box_model::specifies_anything(s)) else {
            return Self::default();
        };
        let (sizing, margin) = (&style.sizing, &style.spacing.margin);
        let (size, min, max, before, after) = if vertical {
            (
                sizing.height,
                sizing.min_height,
                sizing.max_height,
                margin.top,
                margin.bottom,
            )
        } else {
            (
                sizing.width,
                sizing.min_width,
                sizing.max_width,
                margin.left,
                margin.right,
            )
        };
        let fixed = |s: Size| match s {
            Size::Fixed(v) => Some(v),
            _ => None,
        };
        Self {
            size: fixed(size),
            min: fixed(min),
            max: fixed(max),
            margins: before.saturating_add(after),
            percent: [size, min, max]
                .iter()
                .any(|s| matches!(s, Size::Percent(_))),
        }
    }

    /// `size` clamped by `max`, then `min` - the box model's order.
    pub(super) fn clamp(&self, size: u16) -> u16 {
        let size = self.max.map_or(size, |m| size.min(m));
        self.min.map_or(size, |m| size.max(m))
    }

    /// The space the box takes for `content`: clamped, margins added.
    pub(super) fn slot(&self, content: u16) -> u16 {
        self.clamp(content).saturating_add(self.margins)
    }

    /// Does the box model put the size back whatever slot it gets? An
    /// explicit size or minimum does.
    pub(super) fn pinned(&self) -> bool {
        self.size.is_some() || self.min.is_some()
    }
}
