//! Size, layout and drawing: `measure`, `render` and what they read.

use super::draw::{draw_frame_from, styled};
use super::{Border, BorderType};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::{BorderStyle, Color};
use crate::utils::unicode::{char_width, truncate_to_width};
use crate::widget::traits::{RenderContext, View};

impl View for Border {
    /// The child's size plus the frame, and at least wide enough for the title.
    /// A child that fills (`None`) makes the border fill too.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let (w, h) = match &self.child {
            Some(child) => {
                let (w, h) =
                    child.measure(max_width.saturating_sub(2), max_height.saturating_sub(2))?;
                (w.saturating_add(2), h.saturating_add(2))
            }
            None => (2, 2),
        };
        // `─ title ─`: a corner, a dash and a space each side.
        let title = self.title.as_deref().map_or(0, |t| {
            crate::utils::unicode::display_width(t).min(u16::MAX as usize) as u16 + 4
        });
        let w = w.max(title).max(self.min_width);
        let h = h.max(self.min_height);
        let w = if self.max_width > 0 {
            w.min(self.max_width)
        } else {
            w
        };
        let h = if self.max_height > 0 {
            h.min(self.max_height)
        } else {
            h
        };
        Some((w.min(max_width), h.min(max_height)))
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = self.apply_constraints(ctx.area);
        if area.width < 2 || area.height < 2 {
            return;
        }

        let border_type = self.border_type_with_css(ctx);
        let chars = border_type.chars();
        let (fg, bg) = self.colors(ctx);

        // Top border
        ctx.set(0, 0, styled(chars.top_left, fg, bg));

        // Top horizontal line with optional title
        let title_start = match self.title {
            Some(ref title) => Self::draw_title(ctx, title, area.width, chars.horizontal, fg, bg),
            None => 1,
        };

        draw_frame_from(
            title_start,
            area.width,
            area.height,
            &chars,
            fg,
            bg,
            |x, y, cell| ctx.set(x, y, cell),
        );

        // Render child in inner area, respecting overflow style
        if let Some(ref child) = self.child {
            let overflow_hidden = ctx.css_overflow_hidden();
            let parent_clip = ctx.clip();
            let inner = ctx.sub_area(
                1,
                1,
                area.width.saturating_sub(2),
                area.height.saturating_sub(2),
            );
            ctx.render_child_with_overflow(child.as_ref(), inner, overflow_hidden, parent_clip);
        }
    }

    crate::impl_view_meta!("Border");
}

impl Border {
    /// Apply size constraints to the available area
    fn apply_constraints(&self, area: Rect) -> Rect {
        let eff_max_w = if self.max_width > 0 {
            self.max_width.max(self.min_width)
        } else {
            u16::MAX
        };
        let eff_max_h = if self.max_height > 0 {
            self.max_height.max(self.min_height)
        } else {
            u16::MAX
        };
        let width = area.width.clamp(self.min_width, eff_max_w);
        let height = area.height.clamp(self.min_height, eff_max_h);

        Rect::new(area.x, area.y, width, height)
    }

    /// The border to draw: the stylesheet's if it named one, else the builder's.
    ///
    /// `border-style`'s initial value is `none`, so a stylesheet that did not
    /// mention it leaves the builder's own choice alone. `border-style: none`
    /// is a different thing and removes the border, which it could not express
    /// while the property was a plain enum whose initial value *was* `None`.
    ///
    /// `dashed` has no box-drawing set in a terminal, so it renders as a single
    /// line. Documented in `docs/FEATURES.md` rather than silently mapped to
    /// something that looks unrelated.
    fn border_type_with_css(&self, ctx: &RenderContext) -> BorderType {
        match ctx.css_border_style() {
            None => self.border_type,
            Some(BorderStyle::None) => BorderType::None,
            Some(BorderStyle::Solid) | Some(BorderStyle::Dashed) => BorderType::Single,
            Some(BorderStyle::Double) => BorderType::Double,
            Some(BorderStyle::Rounded) => BorderType::Rounded,
        }
    }

    /// The frame's colors. The builder is the inline style and wins; the
    /// stylesheet fills in what it did not set. Same precedence as every other
    /// paint property.
    fn colors(&self, ctx: &RenderContext) -> (Option<Color>, Option<Color>) {
        let fg = self.fg.or_else(|| ctx.css_border_or_text_color());
        let bg = self.bg.or_else(|| {
            let c = ctx.style?.visual.background;
            (c != crate::style::Color::default()).then_some(c)
        });
        (fg, bg)
    }

    /// The title on the top edge after one `horizontal`, cut to the columns
    /// the corners leave. Returns the column the rest of the edge starts at.
    fn draw_title(
        ctx: &mut RenderContext,
        title: &str,
        width: u16,
        horizontal: char,
        fg: Option<Color>,
        bg: Option<Color>,
    ) -> u16 {
        // Everything here is in terminal columns: a wide glyph takes two
        // cells and a zero-width one (e.g. the VS16 in "⚙️") takes none.
        let max_title_width = (width as usize).saturating_sub(4);
        let display_title = truncate_to_width(title, max_title_width);

        // Draw horizontal before title
        ctx.set(1, 0, styled(horizontal, fg, bg));

        // Draw title, the way `Buffer::put_str_styled` lays text out
        let mut x = 2u16;
        for ch in display_title.chars() {
            let w = char_width(ch) as u16;
            if w == 0 {
                continue;
            }
            ctx.set(x, 0, styled(ch, fg, bg));
            for dx in 1..w {
                let mut cont = Cell::continuation();
                cont.fg = fg;
                cont.bg = bg;
                ctx.set(x + dx, 0, cont);
            }
            x += w;
        }

        x
    }
}
