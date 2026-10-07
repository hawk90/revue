//! The draw pipeline: DOM, styles, layout tree, buffers and terminal output

use super::App;
use crate::render::Terminal;
use crate::widget::View;

impl App {
    /// Draw the UI to the terminal
    pub(crate) fn draw<V: View, W: std::io::Write>(
        &mut self,
        view: &V,
        terminal: &mut Terminal<W>,
        force_redraw: bool,
    ) -> crate::Result<()> {
        let (width, height) = self.get_buffer_size();

        let dom_from_render = self.dom.dom_from_render();
        if !dom_from_render {
            let root_dom_id = self.update_dom_and_get_root(view)?;
            if self.layout_engine {
                self.update_layout_tree(root_dom_id, width, height);
            }
        }

        let new_buffer_idx = self.swap_buffers();
        self.render_to_buffer(view, new_buffer_idx);

        if dom_from_render && self.layout_engine {
            // The render pass built this frame's DOM, so lay it out now rather
            // than a frame late. Nothing paints from the result (containers
            // place their own children), so running after the paint is safe.
            if self.dom.take_structure_dirty() {
                self.needs_layout_rebuild = true;
            }
            if let Some(root_dom_id) = self.dom.tree().root_id() {
                self.update_layout_tree(root_dom_id, width, height);
            }
        }
        self.draw_to_terminal(terminal, new_buffer_idx, force_redraw)?;

        // Clear dirty flags after rendering
        self.dom.tree_mut().clear_dirty_flags();

        Ok(())
    }

    /// Update DOM and return the root DOM ID
    fn update_dom_and_get_root<V: View>(&mut self, view: &V) -> crate::Result<crate::dom::DomId> {
        // With reconciliation off, the DOM is built once and then never follows
        // the view again - so a widget added after the first frame is invisible
        // to CSS, to layout and to devtools until something forces a rebuild.
        //
        // With it on, `build` reconciles every frame: nodes that still match
        // keep their DomId, their state and their cached style.
        if self.needs_dom_rebuild || self.incremental_dom {
            self.dom.build(view);
            self.needs_dom_rebuild = false;
            // Only a change in the *shape* of the DOM invalidates layout.
            // Rebuilding it on every reconciled frame would cost more than the
            // full rebuild reconciliation replaces.
            if self.dom.take_structure_dirty() {
                self.needs_layout_rebuild = true;
            }
        }

        // Always compute styles (has internal dirty checking optimization)
        self.dom.compute_styles_with_inheritance();

        self.dom.tree().root_id().ok_or_else(|| {
            crate::Error::Other(anyhow::anyhow!(
                "Root DOM node not found. DOM may not have been built."
            ))
        })
    }

    /// Get the current buffer size
    pub(super) fn get_buffer_size(&self) -> (u16, u16) {
        (
            self.buffers[self.current_buffer].width(),
            self.buffers[self.current_buffer].height(),
        )
    }

    /// Update layout tree with the given dimensions
    fn update_layout_tree(&mut self, root_dom_id: crate::dom::DomId, width: u16, height: u16) {
        // Only rebuild layout tree if needed (e.g., on resize or structural changes)
        if self.needs_layout_rebuild {
            self.layout.clear();
            self.build_layout_tree(root_dom_id);
            self.needs_layout_rebuild = false;
        } else {
            // Incremental update: only update nodes that changed
            self.update_layout_tree_incremental(root_dom_id);
        }

        // Compute layout for the given dimensions
        if let Err(e) = self.layout.compute(root_dom_id, width, height) {
            crate::log_warn!("Layout compute failed for {:?}: {}", root_dom_id, e);
        }
    }

    /// Swap buffers and return the new buffer index
    fn swap_buffers(&mut self) -> usize {
        1 - self.current_buffer
    }

    /// Render the view into the back buffer.
    ///
    /// Always clears and renders the whole view. Painting into a buffer is
    /// memory traffic; the expensive part of a frame is what goes down the wire
    /// to the terminal, and the buffer diff in [`draw_to_terminal`](Self::draw_to_terminal) already
    /// reduces that to exactly the cells that changed.
    ///
    /// This used to skip rendering when the DOM reported no dirty nodes, and to
    /// clear only the dirty regions otherwise. Both were unsound: a widget's
    /// content is not part of `WidgetMeta`, so an ordinary state change marks
    /// nothing dirty - and the app simply stopped repainting.
    fn render_to_buffer<V: View>(&mut self, view: &V, buffer_idx: usize) {
        let new_buffer = &mut self.buffers[buffer_idx];
        let area = crate::layout::Rect::new(0, 0, new_buffer.width(), new_buffer.height());

        new_buffer.clear();
        self.dom.render(view, new_buffer, area);
    }

    /// Draw the buffer to the terminal
    fn draw_to_terminal<W: std::io::Write>(
        &mut self,
        terminal: &mut Terminal<W>,
        buffer_idx: usize,
        force_redraw: bool,
    ) -> crate::Result<()> {
        let old_buffer = &self.buffers[self.current_buffer];
        let new_buffer = &self.buffers[buffer_idx];

        if force_redraw || self.needs_force_redraw {
            terminal.force_redraw(new_buffer)?;
            self.needs_force_redraw = false;
        } else {
            // Compare the whole buffer. Masking this to a region is only safe
            // when the region provably covers everything that was painted, and
            // nothing in the pipeline establishes that today - a change outside
            // the mask lands in the buffer and never reaches the terminal, after
            // which the two buffers agree with each other and no later diff can
            // repair it.
            let changes = crate::render::diff(old_buffer, new_buffer, &[]);
            terminal.draw_changes(changes, new_buffer)?;
        }

        // Swap to the new buffer
        self.current_buffer = buffer_idx;
        Ok(())
    }

    /// Recursively build the layout tree from the DOM tree.
    ///
    /// **Post-order.** [`LayoutEngine::create_node_with_children`] links only
    /// the children that already exist, so a parent built first ends up with no
    /// children at all - and a layout tree with no edges computes a rect for
    /// the root and leaves every other node at 0x0. That is what this used to
    /// do, which is why nothing could read the engine's output.
    fn build_layout_tree(&mut self, dom_id: crate::dom::DomId) {
        // Clone children to own the Vec - necessary because we need mutable access to self
        // during recursion, and holding a slice reference would prevent that.
        // DomId (u64) is Copy, so this is just copying IDs, not deep cloning.
        let children = self
            .dom
            .tree()
            .get(dom_id)
            .map(|node| node.children.clone())
            .unwrap_or_default();

        for child_dom_id in &children {
            self.build_layout_tree(*child_dom_id);
        }

        // Use default style if computation fails (defensive programming)
        let style = match self.dom.style_for_with_inheritance(dom_id) {
            Some(s) => s,
            None => {
                crate::log_warn!("Style not found for DOM node {:?}, using default", dom_id);
                crate::style::Style::default()
            }
        };
        if let Err(e) = self
            .layout
            .create_node_with_children(dom_id, &style, &children)
        {
            crate::log_warn!("Layout node creation failed for {:?}: {}", dom_id, e);
        }
    }

    /// Incrementally update layout tree (only update changed nodes)
    ///
    /// Works with the incremental DOM build to only update dirty nodes.
    fn update_layout_tree_incremental(&mut self, dom_id: crate::dom::DomId) {
        // Check if this node exists in layout
        let node_exists = self.layout.layout(dom_id).is_ok();

        if !node_exists {
            // Node doesn't exist, need full rebuild
            self.needs_layout_rebuild = true;
            return;
        }

        // Get node state to check if dirty
        let is_dirty = self
            .dom
            .tree()
            .get(dom_id)
            .map(|n| n.state.dirty)
            .unwrap_or(false);

        // Only update style if node is dirty
        if is_dirty {
            if let Some(style) = self.dom.style_for_with_inheritance(dom_id) {
                if let Err(e) = self.layout.update_style(dom_id, &style) {
                    crate::log_warn!("Layout style update failed for {:?}: {}", dom_id, e);
                }
            }
        }

        // Recursively update children - clone to own the Vec
        // Necessary because we need mutable access to self during recursion.
        // DomId (u64) is Copy, so this is just copying IDs, not deep cloning.
        let children = self
            .dom
            .tree()
            .get(dom_id)
            .map(|n| n.children.clone())
            .unwrap_or_default();

        for child_id in children {
            self.update_layout_tree_incremental(child_id);
        }
    }
}
