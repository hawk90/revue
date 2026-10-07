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
}
