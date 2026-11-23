//! A scrollbar on its own, for a view that keeps its own scroll offset

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Which way the bar runs
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Axis {
    Vertical,
    Horizontal,
}

/// A scrollbar for content the app scrolls itself (#954)
///
/// It takes the content's length, the offset of the first shown row (or
/// column) and how many are shown, and draws a track with a thumb. It always
/// takes its column (or row), also when everything fits and there is
/// nothing to scroll: then only the track is drawn. So the content beside it
/// keeps one width whether or not it scrolls.
///
/// ```rust
/// use revue::widget::Scrollbar;
///
/// // 200 lines, 20 shown, starting at line 40
/// let bar = Scrollbar::vertical()
///     .content_length(200)
///     .viewport_length(20)
///     .offset(40);
/// ```
#[derive(Clone)]
pub struct Scrollbar {
    axis: Axis,
    content_length: usize,
    offset: usize,
    viewport_length: Option<usize>,
    thumb_color: Option<Color>,
    track_color: Option<Color>,
    props: WidgetProps,
}

impl Scrollbar {
    /// A bar one column wide, as tall as its area
    pub fn vertical() -> Self {
        Self::with_axis(Axis::Vertical)
    }

    /// A bar one row tall, as wide as its area
    pub fn horizontal() -> Self {
        Self::with_axis(Axis::Horizontal)
    }

    fn with_axis(axis: Axis) -> Self {
        Self {
            axis,
            content_length: 0,
            offset: 0,
            viewport_length: None,
            thumb_color: None,
            track_color: None,
            props: WidgetProps::new(),
        }
    }

    /// The content's length, in rows (or columns)
    pub fn content_length(mut self, length: usize) -> Self {
        self.content_length = length;
        self
    }

    /// The first row (or column) shown. Past the last full page it reads as
    /// the last page.
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = offset;
        self
    }

    /// How many rows (or columns) are shown. Without it, the bar's own
    /// length: right when the bar is as tall (or wide) as the view it
    /// scrolls.
    pub fn viewport_length(mut self, length: usize) -> Self {
        self.viewport_length = Some(length);
        self
    }

    /// The thumb's color (default: the stylesheet's `color`, else white)
    pub fn thumb_color(mut self, color: Color) -> Self {
        self.thumb_color = Some(color);
        self
    }

    /// The track's color (default: the stylesheet's `background`, else dark
    /// gray)
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = Some(color);
        self
    }
}

/// Where the thumb goes on a track `track` cells long, as (start, length),
/// for `content` rows of which `viewport` are shown from `offset`; `None`
/// when everything is shown and there is nothing to scroll
///
/// The thumb is the shown share of the track (at least a cell), and it moves
/// from the track's start at offset 0 to its end at the last page. ScrollView
/// and Scrollbar draw it from here.
pub(crate) fn scrollbar_thumb(
    content: usize,
    viewport: usize,
    offset: usize,
    track: u16,
) -> Option<(u16, u16)> {
    if content <= viewport || track == 0 {
        return None;
    }
    let length = ((viewport as f64 / content as f64) * f64::from(track)) as u16;
    let length = length.clamp(1, track);
    let max_offset = content - viewport;
    let ratio = offset.min(max_offset) as f64 / max_offset as f64;
    let start = (f64::from(track - length) * ratio) as u16;
    Some((start.min(track - length), length))
}

impl View for Scrollbar {
    crate::impl_view_meta!("Scrollbar");

    /// One column as tall as offered (vertical), or one row as wide
    /// (horizontal): the bar keeps its cells whether or not there is
    /// anything to scroll.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        Some(match self.axis {
            Axis::Vertical => (1.min(max_width), max_height),
            Axis::Horizontal => (max_width, 1.min(max_height)),
        })
    }

    fn fills(&self) -> crate::widget::Fill {
        match self.axis {
            Axis::Vertical => crate::widget::Fill::HEIGHT,
            Axis::Horizontal => crate::widget::Fill::WIDTH,
        }
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 {
            return;
        }
        let track = match self.axis {
            Axis::Vertical => area.height,
            Axis::Horizontal => area.width,
        };
        let viewport = self.viewport_length.unwrap_or(usize::from(track));
        let thumb = scrollbar_thumb(self.content_length, viewport, self.offset, track);

        let track_fg = self
            .track_color
            .unwrap_or_else(|| ctx.css_background(Color::rgb(64, 64, 64)));
        let thumb_fg = self
            .thumb_color
            .unwrap_or_else(|| ctx.css_color(Color::WHITE));
        let (track_ch, thumb_ch) = match self.axis {
            Axis::Vertical => ('│', '█'),
            Axis::Horizontal => ('─', '█'),
        };
        for i in 0..track {
            let in_thumb = thumb.is_some_and(|(start, len)| i >= start && i < start + len);
            let cell = if in_thumb {
                Cell::new(thumb_ch).fg(thumb_fg)
            } else {
                Cell::new(track_ch).fg(track_fg)
            };
            match self.axis {
                Axis::Vertical => ctx.set(0, i, cell),
                Axis::Horizontal => ctx.set(i, 0, cell),
            }
        }
    }
}

impl_styled_view!(Scrollbar);
impl_props_builders!(Scrollbar);

/// A vertical scrollbar
pub fn scrollbar() -> Scrollbar {
    Scrollbar::vertical()
}

#[cfg(test)]
mod tests {
    use super::scrollbar_thumb;

    #[test]
    fn thumb_is_the_shown_share_and_reaches_both_ends() {
        // Nothing to scroll: no thumb
        assert_eq!(scrollbar_thumb(10, 10, 0, 10), None);
        assert_eq!(scrollbar_thumb(5, 10, 0, 10), None);
        assert_eq!(scrollbar_thumb(100, 10, 0, 0), None);
        // A tenth shown: a one-cell thumb at the start, and at the end on
        // the last page and past it
        assert_eq!(scrollbar_thumb(100, 10, 0, 10), Some((0, 1)));
        assert_eq!(scrollbar_thumb(100, 10, 90, 10), Some((9, 1)));
        assert_eq!(scrollbar_thumb(100, 10, 500, 10), Some((9, 1)));
        // Half shown on a 20-cell track: a 10-cell thumb
        assert_eq!(scrollbar_thumb(40, 20, 0, 20), Some((0, 10)));
        assert_eq!(scrollbar_thumb(40, 20, 20, 20), Some((10, 10)));
        // A huge content still gets a one-cell thumb inside the track
        assert_eq!(scrollbar_thumb(usize::MAX, 1, usize::MAX, 5), Some((4, 1)));
        for content in 1..60usize {
            for viewport in 0..content {
                for offset in 0..=content {
                    let (start, len) = scrollbar_thumb(content, viewport, offset, 12).unwrap();
                    assert!(
                        len >= 1 && start + len <= 12,
                        "{content} {viewport} {offset}"
                    );
                }
            }
        }
    }
}
