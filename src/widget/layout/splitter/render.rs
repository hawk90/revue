//! Drawing the dividers between panes

use super::{SplitOrientation, Splitter, SplitterStyle};
use crate::render::Cell;
use crate::widget::theme::DARK_GRAY;
use crate::widget::traits::{RenderContext, View};

impl SplitterStyle {
    fn char(&self, orientation: SplitOrientation) -> char {
        match (self, orientation) {
            (SplitterStyle::Line, SplitOrientation::Horizontal) => '│',
            (SplitterStyle::Line, SplitOrientation::Vertical) => '─',
            (SplitterStyle::Double, SplitOrientation::Horizontal) => '║',
            (SplitterStyle::Double, SplitOrientation::Vertical) => '═',
            (SplitterStyle::Thick, SplitOrientation::Horizontal) => '┃',
            (SplitterStyle::Thick, SplitOrientation::Vertical) => '━',
            (SplitterStyle::Hidden, _) => ' ',
        }
    }
}

impl View for Splitter {
    crate::impl_view_meta!("Splitter");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let idle_color = self.color.unwrap_or_else(|| ctx.css_color(DARK_GRAY));
        let areas = self.pane_areas(area);

        // Draw splitters between panes
        for (i, (_, pane_area)) in areas.iter().enumerate().take(areas.len().saturating_sub(1)) {
            let is_active = self.active_divider == Some(i);
            // The idle divider takes `color`; the one being dragged keeps its
            // highlight, which a rule cannot address separately.
            let color = if is_active {
                self.active_color
            } else {
                idle_color
            };
            let ch = self.style.char(self.orientation);

            match self.orientation {
                SplitOrientation::Horizontal => {
                    let x = pane_area.x + pane_area.width - area.x;
                    for y in 0..area.height {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        ctx.set(x, y, cell);
                    }
                }
                SplitOrientation::Vertical => {
                    let y = pane_area.y + pane_area.height - area.y;
                    for x in 0..area.width {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        ctx.set(x, y, cell);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::Pane;
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_splitter_style_char() {
        assert_eq!(SplitterStyle::Line.char(SplitOrientation::Horizontal), '│');
        assert_eq!(SplitterStyle::Line.char(SplitOrientation::Vertical), '─');
        assert_eq!(
            SplitterStyle::Double.char(SplitOrientation::Horizontal),
            '║'
        );
        assert_eq!(
            SplitterStyle::Hidden.char(SplitOrientation::Horizontal),
            ' '
        );
    }

    #[test]
    fn test_splitter_render_no_panic() {
        let mut buf = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buf, area);
        let s = Splitter::new()
            .pane(Pane::new("a").ratio(0.5))
            .pane(Pane::new("b").ratio(0.5));
        s.render(&mut ctx);
    }

    /// The divider columns, and which of them is highlighted.
    fn dividers(s: &Splitter) -> (Vec<u16>, Option<u16>) {
        let mut buf = Buffer::new(40, 1);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 40, 1));
        s.render(&mut ctx);
        let xs: Vec<u16> = (0..40)
            .filter(|&x| buf.get(x, 0).unwrap().symbol == '│')
            .collect();
        let active = xs
            .iter()
            .copied()
            .find(|&x| buf.get(x, 0).unwrap().fg == Some(s.active_color));
        (xs, active)
    }

    #[test]
    fn resize_moves_the_highlighted_divider_past_a_collapsed_pane() {
        let pane = |id| Pane::new(id).ratio(0.3).min_size(0);
        let mut s = Splitter::new()
            .pane(pane("a"))
            .pane(pane("b").collapsible())
            .pane(pane("c"))
            .pane(pane("d"));
        s.toggle_pane(1);
        // Visible: a | c | d - divider 1 sits between c and d.
        s.start_resize(1);
        let (before, active) = dividers(&s);
        assert_eq!(before.len(), 2);
        assert_eq!(
            active,
            Some(before[1]),
            "divider 1 is not the highlighted one"
        );

        s.resize(10);
        let (after, active) = dividers(&s);
        assert_eq!(
            after[0], before[0],
            "a divider other than the highlighted one moved"
        );
        assert!(
            after[1] > before[1],
            "the highlighted divider did not move right"
        );
        assert_eq!(active, Some(after[1]));
    }
}
