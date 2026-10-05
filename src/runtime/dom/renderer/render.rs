//! Rendering logic for DomRenderer

use crate::dom::renderer::types::DomRenderer;
use crate::dom::Query;
use crate::layout::Rect;
use crate::render::Buffer;
use crate::widget::{RenderContext, View};

impl DomRenderer {
    /// Render the view into `buffer`.
    ///
    /// With [`dom_from_render`](Self::dom_from_render) enabled the frame runs
    /// two traversals - collect, then paint - so the DOM describes what was
    /// actually rendered and every widget receives its own computed style. See
    /// [`CollectSink`](crate::dom::CollectSink).
    pub fn render<V: View>(&mut self, root: &V, buffer: &mut Buffer, area: Rect) {
        if self.dom_from_render {
            self.render_two_pass(root, buffer, area);
        } else {
            self.render_from_children(root, buffer, area);
        }
    }

    /// Collect the tree from the render traversal, reconcile, then paint it.
    fn render_two_pass<V: View>(&mut self, root: &V, buffer: &mut Buffer, area: Rect) {
        use crate::widget::traits::render_context::{PaintNode, RenderPass};

        // Pass 1: discover the tree. Whatever this paints is discarded by the
        // clear below - the buffer is reused rather than allocating a scratch
        // one, and widgets must see a real area or they would take different
        // branches than the paint pass.
        let mut sink = crate::dom::CollectSink::new();
        sink.push_root(root.meta());
        {
            let mut discard = crate::widget::OverlayQueue::new();
            let mut ctx = RenderContext::new(buffer, area);
            ctx.pass = Some(RenderPass::Collect {
                sink: &mut sink,
                parent: Some(0),
            });
            ctx = ctx.with_overlay_queue(&mut discard);
            root.render(&mut ctx);
        }

        let order = self.reconcile_collected(&sink);
        self.compute_styles_with_inheritance();

        // Only the CSS box path can make the two traversals diverge, and only
        // it needs to skip a subtree. With it off, every node's "subtree" is
        // itself, which is what the plain cursor already does.
        let subtree_lens = if self.css_layout {
            sink.subtree_lens()
        } else {
            Vec::new()
        };
        let painted: Vec<PaintNode<'_>> = order
            .iter()
            .enumerate()
            .map(|(i, &id)| PaintNode {
                id,
                style: self.styles.get(&id),
                state: self.tree.get(id).map(|n| &n.state),
                subtree_len: subtree_lens.get(i).copied().unwrap_or(1),
            })
            .collect();

        // Pass 2: paint, with each widget's own style and state.
        buffer.clear();
        let mut overlay_queue = crate::widget::OverlayQueue::new();
        let mut next = 1; // index 0 is the root, consumed here

        // Moved out so the paint pass can fill it while `painted` still borrows
        // the tree and the style cache. Its capacity survives the round trip.
        let mut hits = std::mem::take(&mut self.hit_map);
        hits.clear();
        if let Some(root_node) = painted.first() {
            hits.push((root_node.id, area));
        }
        {
            let mut ctx = RenderContext::new(buffer, area);
            if let Some(root_node) = painted.first() {
                ctx.style = root_node.style;
                ctx.state = root_node.state;
            }
            ctx.pass = Some(RenderPass::Paint {
                nodes: &painted,
                next: &mut next,
                css_layout: self.css_layout,
                hits: &mut hits,
            });
            ctx = ctx.with_overlay_queue(&mut overlay_queue);
            root.render(&mut ctx);
        }
        drop(painted);
        self.hit_map = hits;
        overlay_queue.render_to(buffer);
    }

    /// The original path: the DOM comes from `View::children`, and only the root
    /// widget is handed a computed style.
    fn render_from_children<V: View>(&mut self, root: &V, buffer: &mut Buffer, area: Rect) {
        // Only the paint pass records where nodes land, so on this path there
        // is nothing to hit-test against. Better empty than stale.
        self.hit_map.clear();

        // Compute styles with inheritance
        self.compute_styles_with_inheritance();

        // Get root style and state
        let root_id = self.tree.root_id();
        let (style, state) = if let Some(id) = root_id {
            let style = self.styles.get(&id);
            let state = self.tree.get(id).map(|n| &n.state);
            (style, state)
        } else {
            (None, None)
        };

        // Phase 1: Main widget tree render with overlay queue
        let mut overlay_queue = crate::widget::OverlayQueue::new();

        let mut ctx = if let (Some(style), Some(state)) = (style, state) {
            RenderContext::full(buffer, area, style, state)
        } else if let Some(style) = style {
            RenderContext::with_style(buffer, area, style)
        } else {
            RenderContext::new(buffer, area)
        };
        ctx = ctx.with_overlay_queue(&mut overlay_queue);

        root.render(&mut ctx);

        // Phase 2: Render overlays on top (sorted by z-index)
        // ctx must go out of scope so buffer borrow is released
        let _ = ctx;
        overlay_queue.render_to(buffer);
    }

    /// Query nodes by selector
    pub fn query(&self, selector: &str) -> Vec<&crate::dom::DomNode> {
        self.tree.query_all(selector).all().to_vec()
    }

    /// Query one node by selector
    pub fn query_one(&self, selector: &str) -> Option<&crate::dom::DomNode> {
        self.tree.query_one(selector)
    }

    /// Get node by element ID
    pub fn get_by_id(&self, id: &str) -> Option<&crate::dom::DomNode> {
        self.tree.get_by_id(id)
    }
}
