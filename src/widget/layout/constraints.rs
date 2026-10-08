//! The `min_width` / `min_height` / `max_width` / `max_height` builders that
//! many layout widgets share: the rect they allow, and drawing inside it.

use crate::layout::Rect;
use crate::widget::traits::RenderContext;

/// `area` resized to the constraints, keeping its top-left corner. A max of
/// 0 means no limit; a max below its min counts as the min.
pub(crate) fn constrain(area: Rect, min_w: u16, min_h: u16, max_w: u16, max_h: u16) -> Rect {
    let max_w = if max_w > 0 {
        max_w.max(min_w)
    } else {
        u16::MAX
    };
    let max_h = if max_h > 0 {
        max_h.max(min_h)
    } else {
        u16::MAX
    };
    Rect::new(
        area.x,
        area.y,
        area.width.clamp(min_w, max_w),
        area.height.clamp(min_h, max_h),
    )
}

/// Run `draw` with `ctx.area` set to `area`, then put the area back.
///
/// Unlike [`RenderContext::sub_ctx`] this keeps the render pass, so children
/// drawn inside still register in the DOM.
pub(crate) fn within(ctx: &mut RenderContext, area: Rect, draw: impl FnOnce(&mut RenderContext)) {
    let outer = std::mem::replace(&mut ctx.area, area);
    draw(ctx);
    ctx.area = outer;
}
