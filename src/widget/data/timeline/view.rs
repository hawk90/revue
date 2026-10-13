//! Timeline `View`: draws the vertical or the horizontal layout

use super::{Timeline, TimelineOrientation};
use crate::widget::traits::{RenderContext, View};

impl View for Timeline {
    crate::impl_view_meta!("Timeline");

    fn render(&self, ctx: &mut RenderContext) {
        match self.orientation {
            TimelineOrientation::Vertical => self.render_vertical(ctx),
            TimelineOrientation::Horizontal => self.render_horizontal(ctx),
        }
    }
}
