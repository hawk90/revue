//! Render context for widget rendering

mod box_model;
mod css;
mod focus;
pub mod overlay;
mod progress;
mod relative;
mod segments;
mod shapes;
mod text;
mod types;

#[cfg(test)]
mod tests;

pub use overlay::{OverlayEntry, OverlayQueue};
pub use types::ProgressBarConfig;

use super::View;
use crate::dom::{CollectSink, DomId, NodeState};
use crate::layout::Rect;
use crate::render::Buffer;
use crate::style::{Display, Style};

/// One node as the paint pass sees it.
pub struct PaintNode<'a> {
    pub id: DomId,
    pub style: Option<&'a Style>,
    pub(crate) state: Option<&'a NodeState>,
    /// Nodes in this node's subtree, itself included.
    ///
    /// The collect pass records pre-order, so a subtree is contiguous. That
    /// makes this both the distance to the next sibling and the amount to skip
    /// for a `display: none` node.
    pub(crate) subtree_len: usize,
}

/// Which half of the frame this context belongs to.
///
/// A frame renders the view twice: once to discover the tree, once to paint it
/// with the styles that tree produced. See
/// [`dom::renderer::collect`](crate::dom::CollectSink).
///
/// `None` means neither - a context built directly rather than through
/// [`RenderContext::render_child`]. Those still render; they just do not
/// register a node, which is the pre-existing behavior.
pub enum RenderPass<'a> {
    /// Recording what each widget renders, in traversal order.
    Collect {
        sink: &'a mut CollectSink,
        /// Index of this widget in the sink; `None` at the root's parent.
        parent: Option<usize>,
    },
    /// Painting, with the nodes the collect pass produced.
    ///
    /// `next` is a shared cursor into `nodes`. The two traversals are the same
    /// walk, so a counter is enough to align them - and after each child it is
    /// reset to that child's subtree end, so a child that renders a different
    /// number of nodes than it did during collect cannot drag its siblings out
    /// of alignment.
    Paint {
        nodes: &'a [PaintNode<'a>],
        next: &'a mut usize,
        /// Apply each node's specified CSS box properties to the area its
        /// parent gave it. See [`AppBuilder::css_layout`](crate::app::AppBuilder::css_layout).
        css_layout: bool,
    },
}

/// Render context passed to widgets
pub struct RenderContext<'a> {
    /// Buffer to render into
    pub buffer: &'a mut Buffer,
    /// Available area for rendering
    pub area: Rect,
    /// Computed style from CSS cascade
    pub style: Option<&'a Style>,
    /// Current widget state
    pub state: Option<&'a NodeState>,
    /// Transition values for animations (property name -> current value)
    transitions: Option<&'a std::collections::HashMap<String, f32>>,
    /// Overlay queue for floating content (dropdowns, tooltips, toasts)
    overlays: Option<&'a mut OverlayQueue>,
    /// Clipping region for overflow: hidden (absolute coordinates)
    ///
    /// When set, all drawing operations are clipped to this rectangle.
    /// Content outside this area is not rendered.
    clip: Option<Rect>,
    /// Which half of the frame this context belongs to, if either.
    pub pass: Option<RenderPass<'a>>,
}

impl<'a> RenderContext<'a> {
    /// Create a basic render context (without style/state)
    pub fn new(buffer: &'a mut Buffer, area: Rect) -> Self {
        Self {
            buffer,
            area,
            style: None,
            state: None,
            transitions: None,
            overlays: None,
            clip: None,
            pass: None,
        }
    }

    /// Create a render context with style
    pub fn with_style(buffer: &'a mut Buffer, area: Rect, style: &'a Style) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: None,
            transitions: None,
            overlays: None,
            clip: None,
            pass: None,
        }
    }

    /// Create a full render context
    pub fn full(
        buffer: &'a mut Buffer,
        area: Rect,
        style: &'a Style,
        state: &'a NodeState,
    ) -> Self {
        Self {
            buffer,
            area,
            style: Some(style),
            state: Some(state),
            transitions: None,
            overlays: None,
            clip: None,
            pass: None,
        }
    }

    /// Render a child widget into `area`, registering it in the DOM.
    ///
    /// Container widgets should route child rendering through this rather than
    /// building a [`RenderContext`] by hand. It is what makes the DOM describe
    /// the application: the render traversal is the real widget tree, and a
    /// child constructed inside `render` is invisible to `View::children`.
    ///
    /// In exchange the child is handed the computed style and state of its own
    /// node, so CSS reaches it. A context built directly still renders - it just
    /// registers nothing and receives no style, which is what every container
    /// did before.
    pub fn render_child(&mut self, child: &dyn View, area: Rect) {
        let clip = self.clip;
        self.render_child_with_overflow(child, area, false, clip);
    }

    /// [`render_child`](Self::render_child), with the overflow and clip handling
    /// that [`child_ctx_with_overflow`](Self::child_ctx_with_overflow) applies.
    pub fn render_child_with_overflow(
        &mut self,
        child: &dyn View,
        mut area: Rect,
        overflow_hidden: bool,
        parent_clip: Option<Rect>,
    ) {
        // Destructured so the buffer and the pass can be borrowed at once.
        let RenderContext { buffer, pass, .. } = self;

        match pass {
            None => {
                let mut ctx = RenderContext::child_ctx_with_overflow(
                    buffer,
                    area,
                    overflow_hidden,
                    parent_clip,
                );
                child.render(&mut ctx);
            }
            Some(RenderPass::Collect { sink, parent }) => {
                let me = sink.push(child.meta(), *parent);
                let mut ctx = RenderContext::child_ctx_with_overflow(
                    buffer,
                    area,
                    overflow_hidden,
                    parent_clip,
                );
                ctx.pass = Some(RenderPass::Collect {
                    sink,
                    parent: Some(me),
                });
                child.render(&mut ctx);
            }
            Some(RenderPass::Paint {
                nodes,
                next,
                css_layout,
            }) => {
                // Pre-order, exactly as the collect pass pushed.
                let idx = **next;
                **next += 1;
                let node = nodes.get(idx);

                if *css_layout {
                    if let Some(style) = node.and_then(|n| n.style) {
                        if style.layout.display == Display::None {
                            // The subtree is not painted, but it still exists -
                            // the collect pass walked it, so the cursor has to
                            // step over all of it.
                            **next = idx + node.map_or(1, |n| n.subtree_len);
                            return;
                        }
                        if box_model::specifies_anything(style) {
                            area = box_model::apply(style, area);
                        }
                    }
                }

                let mut ctx = RenderContext::child_ctx_with_overflow(
                    buffer,
                    area,
                    overflow_hidden,
                    parent_clip,
                );
                if let Some(node) = node {
                    ctx.style = node.style;
                    ctx.state = node.state;
                }
                let css_layout = *css_layout;
                ctx.pass = Some(RenderPass::Paint {
                    nodes,
                    next,
                    css_layout,
                });
                child.render(&mut ctx);

                // Resynchronize. The child rendered with an area the collect
                // pass never saw, so it may have produced a different number of
                // nodes; without this, every later sibling would be handed the
                // wrong node's style.
                if let Some(node) = nodes.get(idx) {
                    **next = idx + node.subtree_len;
                }
            }
        }
    }

    /// Are CSS box and gap properties being applied this frame?
    ///
    /// See [`AppBuilder::css_layout`](crate::app::AppBuilder::css_layout). A
    /// container should gate any CSS-derived geometry on this, so that turning
    /// the flag off really does restore the previous behavior.
    pub fn css_layout(&self) -> bool {
        matches!(
            self.pass,
            Some(RenderPass::Paint {
                css_layout: true,
                ..
            })
        )
    }

    /// Attach an overlay queue to this context
    pub fn with_overlay_queue(mut self, queue: &'a mut OverlayQueue) -> Self {
        self.overlays = Some(queue);
        self
    }

    /// Set transition values for this render context
    pub fn with_transitions(
        mut self,
        transitions: &'a std::collections::HashMap<String, f32>,
    ) -> Self {
        self.transitions = Some(transitions);
        self
    }

    /// Get current transition value for a property
    pub fn transition(&self, property: &str) -> Option<f32> {
        self.transitions.and_then(|t| t.get(property).copied())
    }

    /// Get transition value with a default fallback
    pub fn transition_or(&self, property: &str, default: f32) -> f32 {
        self.transition(property).unwrap_or(default)
    }

    /// Queue an overlay to render after the main pass.
    ///
    /// Overlays render at absolute screen coordinates, bypassing parent
    /// clipping. Use this for dropdowns, tooltips, and toasts.
    ///
    /// Returns true if the overlay was queued, false if no overlay queue
    /// is available (e.g., in test contexts).
    pub fn queue_overlay(&mut self, entry: OverlayEntry) -> bool {
        if let Some(ref mut queue) = self.overlays {
            queue.push(entry);
            true
        } else {
            false
        }
    }

    /// Get absolute screen position of this context's area
    pub fn absolute_position(&self) -> (u16, u16) {
        (self.area.x, self.area.y)
    }

    /// Check if overlay queue is available
    pub fn has_overlay_support(&self) -> bool {
        self.overlays.is_some()
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.state.map(|s| s.focused).unwrap_or(false)
    }

    /// Check if hovered
    pub fn is_hovered(&self) -> bool {
        self.state.map(|s| s.hovered).unwrap_or(false)
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.state.map(|s| s.disabled).unwrap_or(false)
    }

    /// Set a clipping region (absolute coordinates)
    ///
    /// When a clipping region is set, all drawing operations (`set`, `put_str`, etc.)
    /// will be restricted to this rectangle. Content outside is silently discarded.
    /// Used by containers with `overflow: hidden`.
    pub fn with_clip(mut self, clip: Rect) -> Self {
        self.clip = Some(clip);
        self
    }

    /// Get the current clipping region
    pub fn clip(&self) -> Option<Rect> {
        self.clip
    }

    /// Check if an absolute coordinate is within the clipping region
    #[inline]
    pub fn is_clipped(&self, abs_x: u16, abs_y: u16) -> bool {
        if let Some(clip) = &self.clip {
            abs_x < clip.x
                || abs_y < clip.y
                || abs_x >= clip.x.saturating_add(clip.width)
                || abs_y >= clip.y.saturating_add(clip.height)
        } else {
            false
        }
    }

    /// Create a child `Rect` from relative position and size.
    ///
    /// Input `x`/`y` are relative to this area; the returned `Rect` contains
    /// absolute buffer coordinates suitable for constructing a child context:
    /// ```ignore
    /// let inner = ctx.sub_area(1, 1, w - 2, h - 2);
    /// let mut child_ctx = RenderContext::new(ctx.buffer, inner);
    /// ```
    pub fn sub_area(&self, x: u16, y: u16, w: u16, h: u16) -> Rect {
        Rect::new(
            self.area.x.saturating_add(x),
            self.area.y.saturating_add(y),
            w.min(self.area.width.saturating_sub(x)),
            h.min(self.area.height.saturating_sub(y)),
        )
    }
}
