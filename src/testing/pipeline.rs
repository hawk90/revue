//! Full-pipeline test harness.
//!
//! [`TestApp`](super::TestApp) and [`Snapshot`](crate::core::app::Snapshot) render a
//! view by calling [`View::render`] against a bare
//! [`RenderContext`](crate::widget::RenderContext). That path never builds the
//! DOM, never runs the CSS cascade, never computes layout, and leaves
//! `RenderContext::state` as `None` - so it cannot observe anything that
//! depends on [`NodeState`], computed style, or dirty tracking.
//!
//! [`PipelineHarness`] instead drives the *real* [`App::draw`] against an
//! in-memory writer. Every frame goes through the same sequence as a running
//! application:
//!
//! ```text
//! update_dom_and_get_root -> compute_styles_with_inheritance
//!   -> update_layout_tree -> render_to_buffer
//!   -> diff -> terminal
//! ```
//!
//! Use this for any test that must not drift from production behavior:
//! reconciliation, node identity, state ownership, invalidation scope.
//!
//! # Example
//!
//! ```ignore
//! use revue::testing::PipelineHarness;
//!
//! let mut h = PipelineHarness::with_css("button { color: cyan; }", 40, 10);
//! h.draw(&my_view);
//! assert!(h.screen_text().contains("Save"));
//! assert!(h.node_state("save-btn").is_some());
//! ```

use crate::core::app::App;
use crate::dom::{DomId, NodeState, Query};
use crate::layout::Rect;
use crate::render::{Buffer, Terminal};
use crate::style::parse_css;
use crate::widget::View;

/// Drives the real [`App`] draw pipeline against an in-memory terminal.
pub struct PipelineHarness {
    app: App,
    terminal: Terminal<Vec<u8>>,
    width: u16,
    height: u16,
    frames: usize,
}

impl PipelineHarness {
    /// Create a harness with no stylesheet.
    pub fn new(width: u16, height: u16) -> Self {
        Self::build(App::builder().size(width, height).build(), width, height)
    }

    /// Create a harness with a stylesheet parsed from `css`.
    ///
    /// # Panics
    ///
    /// Panics if `css` fails to parse - in a test that is what you want.
    pub fn with_css(css: &str, width: u16, height: u16) -> Self {
        let stylesheet = parse_css(css).expect("PipelineHarness: stylesheet failed to parse");
        let mut app = App::builder().size(width, height).build();
        app.dom_renderer().set_stylesheet(stylesheet);
        Self::build(app, width, height)
    }

    /// Turn per-frame DOM reconciliation on or off for this harness.
    ///
    /// Mirrors [`AppBuilder::incremental_dom`](crate::app::AppBuilder::incremental_dom),
    /// which is off by default. A test that exercises reconciliation must turn
    /// it on explicitly - otherwise the DOM is built once and never follows the
    /// view again, and the test silently passes against a frozen tree.
    pub fn incremental_dom(mut self, enabled: bool) -> Self {
        self.app.set_incremental_dom(enabled);
        self
    }

    /// Build the DOM from the render traversal for this harness.
    ///
    /// Mirrors [`AppBuilder::dom_from_render`](crate::app::AppBuilder::dom_from_render),
    /// which is off by default.
    pub fn dom_from_render(mut self, enabled: bool) -> Self {
        self.app.set_dom_from_render(enabled);
        self
    }

    /// Let CSS box properties override container-computed geometry.
    ///
    /// Mirrors [`AppBuilder::css_layout`](crate::app::AppBuilder::css_layout),
    /// which is off by default and inert without `dom_from_render`.
    pub fn css_layout(mut self, enabled: bool) -> Self {
        self.app.set_css_layout(enabled);
        self
    }

    /// Mirrors [`AppBuilder::tab_navigation`](crate::app::AppBuilder::tab_navigation),
    /// which is off by default.
    ///
    /// With it on, sending a `Tab` key through [`send`](Self::send) moves
    /// `:focus` exactly as the event loop does.
    pub fn tab_navigation(mut self, enabled: bool) -> Self {
        self.app.set_tab_navigation(enabled);
        self
    }

    fn build(app: App, width: u16, height: u16) -> Self {
        Self {
            app,
            // Never initialized: no raw mode, no alternate screen, no TTY.
            terminal: Terminal::with_size(Vec::new(), width, height),
            width,
            height,
            frames: 0,
        }
    }

    /// Draw one frame through the full pipeline.
    ///
    /// The first frame is a forced full redraw, matching `App::run`, which
    /// draws once before entering the event loop. Subsequent frames go through
    /// dirty-region tracking.
    ///
    /// # Panics
    ///
    /// Panics if the draw fails. A failing draw in a test is a defect, not a
    /// condition to be handled.
    pub fn draw<V: View>(&mut self, view: &V) -> &mut Self {
        let force = self.frames == 0;
        self.app
            .draw(view, &mut self.terminal, force)
            .expect("PipelineHarness: draw failed");
        self.frames += 1;
        self
    }

    /// Number of frames drawn so far.
    pub fn frame_count(&self) -> usize {
        self.frames
    }

    /// Terminal dimensions.
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    /// The buffer most recently presented to the terminal.
    pub fn buffer(&self) -> &Buffer {
        self.app.presented_buffer()
    }

    /// Rendered screen as text, trailing whitespace and blank lines trimmed.
    pub fn screen_text(&self) -> String {
        let buffer = self.buffer();
        let mut lines: Vec<String> = (0..buffer.height())
            .map(|y| {
                let mut line = String::new();
                for x in 0..buffer.width() {
                    line.push(buffer.get(x, y).map(|c| c.symbol).unwrap_or(' '));
                }
                line.trim_end().to_string()
            })
            .collect();

        while lines.last().is_some_and(|l| l.is_empty()) {
            lines.pop();
        }
        lines.join("\n")
    }

    /// Bytes written to the terminal so far.
    ///
    /// Useful for asserting that an unchanged frame produces no output at all
    /// (`REV-RENDER-001`).
    pub fn terminal_output(&self) -> &[u8] {
        self.terminal.writer()
    }

    /// Whether the rendered screen contains `needle`.
    pub fn contains(&self, needle: &str) -> bool {
        self.screen_text().contains(needle)
    }

    /// [`NodeState`] of the DOM node with the given element id.
    ///
    /// Returns `None` when no node carries that id - which after a draw means
    /// the widget did not report it via `WidgetMeta`.
    pub fn node_state(&self, element_id: &str) -> Option<&NodeState> {
        self.app.dom().get_by_id(element_id).map(|node| &node.state)
    }

    /// [`DomId`] of the node with the given element id.
    ///
    /// Stable across frames for a node that survives reconciliation - this is
    /// the identity contract the 3.0 refactor rests on.
    pub fn node_id(&self, element_id: &str) -> Option<DomId> {
        self.app.dom().get_by_id(element_id).map(|node| node.id)
    }

    /// Number of nodes in the DOM tree.
    pub fn node_count(&self) -> usize {
        self.app.dom().tree().len()
    }

    /// Set the focused node by element id (`None` clears focus).
    ///
    /// Names a node directly. To exercise the paths the runtime actually uses,
    /// click through [`send`](Self::send) with a mouse event, or send a `Tab`
    /// key with [`tab_navigation`](Self::tab_navigation) on.
    pub fn focus(&mut self, element_id: Option<&str>) -> &mut Self {
        self.app.dom_renderer().set_focus(element_id);
        self
    }

    /// Set the hovered node by element id (`None` clears hover).
    pub fn hover(&mut self, element_id: Option<&str>) -> &mut Self {
        self.app.dom_renderer().set_hover(element_id);
        self
    }

    /// Move the pointer to `(x, y)`, exactly as the event loop does.
    ///
    /// Returns `true` if `:hover` moved. Unlike [`hover`](Self::hover), which
    /// names a node directly, this goes through the hit test - so it observes
    /// where the last frame actually painted things. Draw first.
    pub fn mouse_at(&mut self, x: u16, y: u16) -> bool {
        self.app.track_hover_for_test(x, y)
    }

    /// Push an event through the real `App::handle_event`, with a user handler
    /// that does nothing.
    ///
    /// Returns whether the app asked for a redraw. Use this rather than
    /// [`mouse_at`](Self::mouse_at) when the point of the test is that the
    /// event loop *reacts* to the event at all.
    pub fn send<V: View>(&mut self, event: crate::event::Event, view: &mut V) -> bool {
        self.app.dispatch_for_test(event, view)
    }

    /// Element id of the node under `(x, y)` in the last painted frame.
    pub fn element_at(&self, x: u16, y: u16) -> Option<String> {
        self.app.dom().element_at(x, y).map(str::to_owned)
    }

    /// Element id of the node a click at `(x, y)` would focus.
    pub fn focus_target_at(&self, x: u16, y: u16) -> Option<String> {
        self.app
            .dom()
            .focus_target_at(x, y)
            .and_then(|id| self.app.dom().tree().get(id))
            .and_then(|node| node.meta.id.clone())
    }

    /// Whether the node with this element id is a focusable *kind* of widget.
    pub fn is_focusable(&self, element_id: &str) -> bool {
        self.app
            .dom()
            .get_by_id(element_id)
            .is_some_and(|node| node.meta.focusable)
    }

    /// Element id of the node currently hovered, if it has one.
    pub fn hovered_element(&self) -> Option<String> {
        self.app
            .dom()
            .hovered_node()
            .and_then(|id| self.app.dom().tree().get(id))
            .and_then(|node| node.meta.id.clone())
    }

    /// Where the last frame painted the node with this element id.
    ///
    /// This is what the user saw, which is not what
    /// [`layout_rect`](Self::layout_rect) reports - see
    /// `docs/refactor/findings-layout.md`.
    pub fn painted_rect(&self, element_id: &str) -> Option<Rect> {
        self.node_id(element_id)
            .and_then(|id| self.app.dom().painted_rect(id))
    }

    /// The `color` the cascade computed for a node, if it set one.
    ///
    /// What the *stylesheet* decided, as distinct from what reached the screen -
    /// a widget can still paint something else, and several do.
    pub fn computed_color(&self, element_id: &str) -> Option<crate::style::Color> {
        let id = self.node_id(element_id)?;
        let style = self.app.dom().computed_style(id)?;
        let color = style.visual.color;
        (color != crate::style::Color::default()).then_some(color)
    }

    /// Element ids of every node currently marked focused.
    ///
    /// More than one entry is an `INV-02` violation.
    pub fn focused_ids(&self) -> Vec<String> {
        self.app
            .dom()
            .tree()
            .nodes()
            .filter(|n| n.state.focused)
            .filter_map(|n| n.meta.id.clone())
            .collect()
    }

    /// How many nodes are currently marked focused, id or not.
    pub fn focused_count(&self) -> usize {
        self.app
            .dom()
            .tree()
            .nodes()
            .filter(|n| n.state.focused)
            .count()
    }

    /// Nodes currently flagged dirty.
    ///
    /// Cleared at the end of every `draw`, so read this *between* mutations
    /// and the next draw.
    pub fn dirty_count(&self) -> usize {
        self.app
            .dom()
            .tree()
            .nodes()
            .filter(|n| n.state.dirty)
            .count()
    }

    /// Child [`DomId`]s of the node with the given element id, in order.
    ///
    /// Lets a test observe identity for children that carry no element id of
    /// their own - which is exactly where positional matching loses track of
    /// widgets (`REV-TREE-003`).
    pub fn child_ids(&self, parent_element_id: &str) -> Vec<DomId> {
        self.app
            .dom()
            .get_by_id(parent_element_id)
            .map(|node| node.children.clone())
            .unwrap_or_default()
    }

    /// Element ids of the nodes matching a CSS selector, in tree order.
    ///
    /// Reads through the DOM's id/class/type indices, so it also catches an
    /// index left stale by reconciliation - a node whose class changed but
    /// whose entry in `class_index` still points at the old value.
    pub fn query_ids(&self, selector: &str) -> Vec<String> {
        let mut ids: Vec<String> = self
            .app
            .dom()
            .tree()
            .query_all(selector)
            .iter()
            .filter_map(|n| n.meta.id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// How many nodes match a CSS selector.
    pub fn query_count(&self, selector: &str) -> usize {
        self.app.dom().tree().query_all(selector).len()
    }

    /// Element ids of the nodes the DOM's *class index* lists under `class`.
    ///
    /// [`query_ids`](Self::query_ids) walks every node and re-reads its
    /// metadata, so it is always right. This goes through `class_index`, the
    /// cache reconciliation has to maintain. When the two disagree, the index
    /// is stale - and `get_by_class` is what CSS matching and `Query` users
    /// actually call.
    pub fn indexed_class_ids(&self, class: &str) -> Vec<String> {
        let mut ids: Vec<String> = self
            .app
            .dom()
            .tree()
            .get_by_class(class)
            .iter()
            .filter_map(|n| n.meta.id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Structural state of the node with the given element id, as
    /// `(child_index, sibling_count, first_child, last_child)`.
    ///
    /// This is what `:first-child` and `:nth-child` read. Reconciliation
    /// reorders siblings, so it has to be recomputed - a test that only checks
    /// `DomId` stability will not notice when it is not.
    pub fn structural_state(&self, element_id: &str) -> Option<(usize, usize, bool, bool)> {
        self.app.dom().get_by_id(element_id).map(|n| {
            (
                n.state.child_index,
                n.state.sibling_count,
                n.state.first_child,
                n.state.last_child,
            )
        })
    }

    /// Force the next draw to repaint the whole screen.
    ///
    /// Mirrors [`App::request_redraw`]. Useful as a control: if a change only
    /// reaches the terminal after this, the fault is in dirty-rect computation
    /// rather than in rendering or diffing.
    pub fn request_redraw(&mut self) -> &mut Self {
        self.app.request_redraw();
        self
    }

    /// Force the next draw to rebuild the DOM from scratch.
    pub fn request_dom_rebuild(&mut self) -> &mut Self {
        self.app.request_dom_rebuild();
        self
    }

    /// Computed layout rect of the node with the given element id.
    ///
    /// Coordinates are **relative to the parent's content box**, not to the
    /// screen - that is what `LayoutEngine` produces.
    ///
    /// Nothing in the render path reads these yet; widgets still compute their
    /// own geometry. See `docs/refactor/findings-layout.md`.
    pub fn layout_rect(&self, element_id: &str) -> Option<Rect> {
        self.node_id(element_id)
            .and_then(|id| self.app.layout_rect(id))
    }

    /// Element ids of a node's children **in the layout tree**.
    ///
    /// The layout tree must mirror the DOM tree. It did not: every node was
    /// created before its children existed, so it was linked to none of them.
    pub fn layout_child_ids(&self, element_id: &str) -> Vec<String> {
        let Some(id) = self.node_id(element_id) else {
            return Vec::new();
        };
        self.app
            .layout_children(id)
            .into_iter()
            .filter_map(|child| {
                self.app
                    .dom()
                    .tree()
                    .get(child)
                    .and_then(|n| n.meta.id.clone())
            })
            .collect()
    }

    /// Escape hatch for assertions this harness does not cover yet.
    pub fn app(&self) -> &App {
        &self.app
    }
}
