//! Design invariants that must hold before *and* after the 3.0 refactor.
//!
//! Unlike render snapshots, these do not encode current output - they encode
//! rules the architecture must never break. That makes them the right net for
//! a refactor whose whole point is that some output changes.
//!
//! IDs refer to `docs/anti-patterns/catalog.yaml` (`invariants:` section).

use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::{RenderContext, View};

/// Minimal leaf widget with a stable element id.
struct Leaf {
    id: String,
    classes: Vec<String>,
}

impl Leaf {
    fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            classes: vec!["leaf".to_string()],
        }
    }
}

impl View for Leaf {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.id, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "Leaf"
    }
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }
    fn classes(&self) -> &[String] {
        &self.classes
    }
}

/// Container holding a variable number of children.
struct Container {
    children: Vec<Box<dyn View>>,
}

impl Container {
    fn with_ids(ids: &[&str]) -> Self {
        Self {
            children: ids
                .iter()
                .map(|id| Box::new(Leaf::new(id)) as Box<dyn View>)
                .collect(),
        }
    }
}

impl View for Container {
    fn render(&self, ctx: &mut RenderContext) {
        for child in &self.children {
            child.render(ctx);
        }
    }
    fn widget_type(&self) -> &'static str {
        "Container"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

// ---------------------------------------------------------------------------
// INV-02: 하나의 focus scope에는 최대 하나의 focused widget만 존재한다
// ---------------------------------------------------------------------------

#[test]
fn inv02_at_most_one_node_is_focused() {
    let view = Container::with_ids(&["a", "b", "c"]);
    let mut h = PipelineHarness::new(30, 6);
    h.draw(&view);

    assert_eq!(h.focused_count(), 0, "nothing focused before any request");

    h.focus(Some("a"));
    assert_eq!(h.focused_count(), 1);
    assert_eq!(h.focused_ids(), vec!["a".to_string()]);

    // Moving focus must not leave the previous holder focused.
    h.focus(Some("b"));
    assert_eq!(
        h.focused_count(),
        1,
        "focus moved but {:?} are focused",
        h.focused_ids()
    );
    assert_eq!(h.focused_ids(), vec!["b".to_string()]);

    h.focus(None);
    assert_eq!(h.focused_count(), 0);
}

#[test]
fn inv02_holds_across_redraws() {
    let view = Container::with_ids(&["a", "b"]);
    let mut h = PipelineHarness::new(30, 6);
    h.draw(&view);
    h.focus(Some("a"));

    h.draw(&view);
    h.draw(&view);

    assert_eq!(
        h.focused_count(),
        1,
        "redraw duplicated focus: {:?}",
        h.focused_ids()
    );
}

// ---------------------------------------------------------------------------
// INV-03: 제거된 widget은 effect, task, timer, focus를 유지하지 않는다
// (여기서 검증 가능한 범위: 노드와 그 상태가 트리에서 사라진다)
// ---------------------------------------------------------------------------

#[test]
fn inv03_removed_node_leaves_no_state_behind() {
    let mut h = PipelineHarness::new(30, 6);

    let full = Container::with_ids(&["a", "b", "c"]);
    h.draw(&full);
    assert!(h.node_state("b").is_some(), "b should exist initially");

    // "b" disappears from the view.
    let shrunk = Container::with_ids(&["a", "c"]);
    h.request_dom_rebuild().draw(&shrunk);

    assert!(
        h.node_state("b").is_none(),
        "state for a removed widget is still reachable (zombie state)"
    );
    assert!(h.node_state("a").is_some());
    assert!(h.node_state("c").is_some());
}

#[test]
fn inv03_removing_the_focused_node_drops_its_focus() {
    let mut h = PipelineHarness::new(30, 6);

    let full = Container::with_ids(&["a", "b"]);
    h.draw(&full);
    h.focus(Some("b"));
    assert_eq!(h.focused_count(), 1);

    let shrunk = Container::with_ids(&["a"]);
    h.request_dom_rebuild().draw(&shrunk);

    assert_eq!(
        h.focused_count(),
        0,
        "focus survived the removal of its holder: {:?}",
        h.focused_ids()
    );
}

// ---------------------------------------------------------------------------
// INV-06: 변경되지 않은 subtree는 style/layout/paint되지 않는다
// ---------------------------------------------------------------------------

#[test]
fn inv06_unchanged_tree_is_not_dirty_after_draw() {
    let view = Container::with_ids(&["a", "b", "c"]);
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&view);
    h.draw(&view);

    assert_eq!(
        h.dirty_count(),
        0,
        "a redraw of an unchanged tree left nodes dirty"
    );
}

#[test]
fn inv06_focus_change_dirties_only_the_nodes_involved() {
    let view = Container::with_ids(&["a", "b", "c"]);
    let mut h = PipelineHarness::new(30, 6);
    h.draw(&view);

    h.focus(Some("a"));
    let after_first = h.dirty_count();
    assert!(
        after_first <= 2,
        "focusing one node dirtied {after_first} nodes; expected at most the \
         old and new focus holders"
    );

    h.draw(&view);
    h.focus(Some("b"));
    let after_move = h.dirty_count();
    assert!(
        after_move <= 2,
        "moving focus dirtied {after_move} nodes; expected at most 2"
    );
}

// ---------------------------------------------------------------------------
// Phase 1 계약: reconcile을 통과한 노드는 DomId를 보존한다
// ---------------------------------------------------------------------------

#[test]
fn node_id_survives_an_unchanged_redraw() {
    let view = Container::with_ids(&["a", "b"]);
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&view);
    let before = h.node_id("a").expect("a missing");

    h.draw(&view);
    assert_eq!(h.node_id("a"), Some(before));
}

/// Inserting a sibling must not re-identify the nodes that were already there.
///
/// This passes today because every child carries an element id, so
/// `update_children_internal` matches through its id map. The keyless case is
/// covered separately below.
#[test]
fn node_id_survives_sibling_insertion() {
    let mut h = PipelineHarness::new(30, 6);

    let before_view = Container::with_ids(&["a", "b"]);
    h.draw(&before_view);
    let a_before = h.node_id("a").expect("a missing");
    let b_before = h.node_id("b").expect("b missing");

    // A new child is prepended; "a" and "b" shift position but are the same
    // widgets and must keep their identity.
    let after_view = Container::with_ids(&["x", "a", "b"]);
    h.request_dom_rebuild().draw(&after_view);

    assert_eq!(h.node_id("a"), Some(a_before), "'a' was re-identified");
    assert_eq!(h.node_id("b"), Some(b_before), "'b' was re-identified");
}

/// Focus must ride along with identity, not position.
#[test]
fn focus_survives_sibling_insertion() {
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&Container::with_ids(&["a", "b"]));
    h.focus(Some("b"));

    h.request_dom_rebuild()
        .draw(&Container::with_ids(&["x", "a", "b"]));

    assert_eq!(
        h.focused_ids(),
        vec!["b".to_string()],
        "focus did not follow the widget across a structural change"
    );
}

// ---------------------------------------------------------------------------
// REV-TREE-003: identity by position
//
// Children with no element id fall back to positional matching, so inserting
// at the front shifts every widget onto its neighbour's node. This is the gap
// Phase 1 closes by introducing `WidgetMeta::key`.
// ---------------------------------------------------------------------------

/// Leaf with no element id - the common case inside a dynamic list.
struct AnonLeaf {
    label: String,
}

impl AnonLeaf {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}

impl View for AnonLeaf {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, &self.label, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "AnonLeaf"
    }
}

struct AnonContainer {
    children: Vec<Box<dyn View>>,
}

impl AnonContainer {
    fn with_labels(labels: &[&str]) -> Self {
        Self {
            children: labels
                .iter()
                .map(|l| Box::new(AnonLeaf::new(l)) as Box<dyn View>)
                .collect(),
        }
    }
}

impl View for AnonContainer {
    fn render(&self, ctx: &mut RenderContext) {
        for child in &self.children {
            child.render(ctx);
        }
    }
    fn widget_type(&self) -> &'static str {
        "AnonContainer"
    }
    fn id(&self) -> Option<&str> {
        Some("root")
    }
    fn children(&self) -> &[Box<dyn View>] {
        &self.children
    }
}

/// Documents current behaviour: keyless children are matched by position, so
/// prepending re-identifies every existing widget.
///
/// Phase 1 introduces `WidgetMeta::key`; once a key is supplied this must
/// become identity-preserving. The assertion is written against today's
/// behaviour on purpose - when Phase 1 changes it, this test fails loudly and
/// must be rewritten as the positive contract.
#[test]
fn keyless_children_are_identified_by_position_today() {
    let mut h = PipelineHarness::new(30, 6);

    h.draw(&AnonContainer::with_labels(&["a", "b"]));
    let before = h.child_ids("root");
    assert_eq!(before.len(), 2);

    h.request_dom_rebuild()
        .draw(&AnonContainer::with_labels(&["x", "a", "b"]));
    let after = h.child_ids("root");
    assert_eq!(after.len(), 3);

    // Positional matching: the node that used to render "a" now renders "x".
    assert_eq!(
        after[0], before[0],
        "expected slot 0 to be reused for the newly prepended child \
         (positional matching). If this now fails, keyed reconciliation \
         landed - rewrite this test as the identity-preserving contract."
    );
    assert_eq!(after[1], before[1]);
    assert!(
        !before.contains(&after[2]),
        "the trailing node should be newly created"
    );
}

// ---------------------------------------------------------------------------
// INV-05: paint는 layout bounds와 clipping bounds를 벗어나지 않는다
// ---------------------------------------------------------------------------

/// A widget that deliberately paints outside its own area in every direction.
struct Overflowing;

impl View for Overflowing {
    fn render(&self, ctx: &mut RenderContext) {
        // Far past the right edge, far past the bottom, and at coordinates that
        // only make sense if clipping is absent.
        ctx.draw_text(0, 0, &"X".repeat(400), Color::WHITE);
        for y in 0..40u16 {
            ctx.draw_text(0, y, "X", Color::WHITE);
        }
    }
    fn widget_type(&self) -> &'static str {
        "Overflowing"
    }
    fn id(&self) -> Option<&str> {
        Some("overflow")
    }
}

#[test]
fn inv05_paint_stays_inside_the_buffer() {
    let mut h = PipelineHarness::new(20, 6);
    h.draw(&Overflowing);

    // Nothing escaped the buffer: the harness still reports the size it was
    // built with, and every row is exactly that wide.
    assert_eq!(h.size(), (20, 6));
    assert_eq!(h.buffer().width(), 20);
    assert_eq!(h.buffer().height(), 6);

    for line in h.screen_text().lines() {
        assert!(
            line.chars().count() <= 20,
            "row wider than the buffer: {line:?}"
        );
    }
    assert!(
        h.screen_text().lines().count() <= 6,
        "more rows than the buffer height"
    );
}

#[test]
fn inv05_out_of_bounds_paint_does_not_wrap_onto_the_next_row() {
    // If an over-long string were written without clipping, cell N would spill
    // into row 1 - the classic symptom. Row 1 must stay empty.
    struct LongLine;
    impl View for LongLine {
        fn render(&self, ctx: &mut RenderContext) {
            ctx.draw_text(0, 0, &"A".repeat(200), Color::WHITE);
        }
        fn widget_type(&self) -> &'static str {
            "LongLine"
        }
        fn id(&self) -> Option<&str> {
            Some("longline")
        }
    }

    let mut h = PipelineHarness::new(10, 4);
    h.draw(&LongLine);

    let rows: Vec<String> = h.screen_text().lines().map(|l| l.to_string()).collect();
    for (i, row) in rows.iter().enumerate().skip(1) {
        assert!(
            !row.contains('A'),
            "row {i} received spill-over from row 0: {row:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// INV-07: 외부 입력은 terminal control sequence로 직접 출력되지 않는다
// ---------------------------------------------------------------------------

/// A widget whose text content is attacker-controlled.
struct Untrusted(&'static str);

impl View for Untrusted {
    fn render(&self, ctx: &mut RenderContext) {
        ctx.draw_text(0, 0, self.0, Color::WHITE);
    }
    fn widget_type(&self) -> &'static str {
        "Untrusted"
    }
    fn id(&self) -> Option<&str> {
        Some("untrusted")
    }
}

#[test]
fn inv07_escape_sequences_in_content_are_not_emitted() {
    // A colour change, a window-title OSC, and a device-status query. The last
    // one is the dangerous case: a terminal answers it on stdin, so echoing it
    // turns display content into synthetic keystrokes.
    let payload = "hi\x1b[31mred\x1b]0;pwned\x07\x1b[6n!";

    let mut h = PipelineHarness::new(40, 3);
    h.draw(&Untrusted(payload));

    let out = h.terminal_output();

    // Guard against a vacuous pass: the harmless part of the payload must
    // actually have been rendered, otherwise "no escape sequence" is trivial.
    assert!(
        contains_subslice(out, b"hi"),
        "nothing was emitted - the assertions below would pass vacuously"
    );

    assert!(
        !contains_subslice(out, b"\x1b]0;pwned"),
        "OSC window-title sequence from content reached the terminal"
    );
    assert!(
        !contains_subslice(out, b"\x1b[6n"),
        "cursor-position query from content reached the terminal - \
         the reply would arrive as fake user input"
    );
    assert!(
        !contains_subslice(out, b"\x07"),
        "BEL from content reached the terminal"
    );
}

#[test]
fn inv07_control_characters_do_not_reach_the_buffer_verbatim() {
    let mut h = PipelineHarness::new(40, 3);
    h.draw(&Untrusted("a\x1b[31mb\x07c"));

    let screen = h.screen_text();
    assert!(
        screen.contains('a') && screen.contains('c'),
        "nothing rendered: {screen:?}"
    );
    assert!(
        !screen.contains('\x1b'),
        "ESC survived into the cell buffer: {screen:?}"
    );
    assert!(
        !screen.contains('\x07'),
        "BEL survived into the cell buffer: {screen:?}"
    );
}

fn contains_subslice(haystack: &[u8], needle: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|w| w == needle)
}

// ---------------------------------------------------------------------------
// INV-09: panic 또는 정상 종료 후 terminal state는 복구된다
// ---------------------------------------------------------------------------

/// End-to-end: a process that entered TUI mode and then panics must still emit
/// the sequence that leaves the alternate screen and shows the cursor.
///
/// This re-executes the test binary rather than panicking in-process, because
/// the thing under test is what the *process* writes on its way out. The child
/// arms revue's hook and panics; the parent inspects the child's stdout.
///
/// Note on `panic = "abort"` (`Cargo.toml` release profile): `Drop` does not run
/// on abort, which is exactly why this is a panic hook and not a `Drop` impl.
/// The test profile unwinds, so this exercises the unwind path; the abort path
/// is covered by the language guarantee that hooks run before the abort.
#[test]
fn inv09_terminal_is_restored_when_the_process_panics() {
    const CHILD_ENV: &str = "REVUE_INV09_CHILD";

    if std::env::var_os(CHILD_ENV).is_some() {
        // Child: pretend we entered TUI mode, then die.
        revue::render::install_panic_hook();
        panic!("inv09 deliberate panic");
    }

    let exe = std::env::current_exe().expect("test binary path");
    let out = std::process::Command::new(exe)
        .args([
            "inv09_terminal_is_restored_when_the_process_panics",
            "--exact",
            "--nocapture",
        ])
        .env(CHILD_ENV, "1")
        .output()
        .expect("re-exec the test binary");

    assert!(
        !out.status.success(),
        "child was supposed to fail with a panic"
    );

    assert!(
        contains_subslice(&out.stdout, b"\x1b[?1049l"),
        "panicking process never left the alternate screen; \
         stdout was {:?}",
        String::from_utf8_lossy(&out.stdout)
    );
    assert!(
        contains_subslice(&out.stdout, b"\x1b[?25h"),
        "panicking process left the cursor hidden"
    );
    assert!(
        contains_subslice(&out.stdout, b"\x1b[?1000l"),
        "panicking process left mouse capture enabled"
    );

    // The panic message must survive the restore - a hook that swallows it
    // trades a broken terminal for an unexplained exit.
    assert!(
        contains_subslice(&out.stderr, b"inv09 deliberate panic"),
        "the chained hook lost the panic message; stderr was {:?}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// A process that never entered TUI mode must not spray escape sequences when
/// it panics. The hook is armed by entering TUI mode, not by linking revue.
#[test]
fn inv09_hook_is_silent_for_a_process_that_never_entered_tui_mode() {
    const CHILD_ENV: &str = "REVUE_INV09_SILENT_CHILD";

    if std::env::var_os(CHILD_ENV).is_some() {
        panic!("inv09 unrelated panic");
    }

    let exe = std::env::current_exe().expect("test binary path");
    let out = std::process::Command::new(exe)
        .args([
            "inv09_hook_is_silent_for_a_process_that_never_entered_tui_mode",
            "--exact",
            "--nocapture",
        ])
        .env(CHILD_ENV, "1")
        .output()
        .expect("re-exec the test binary");

    assert!(!out.status.success(), "child was supposed to panic");
    assert!(
        !contains_subslice(&out.stdout, b"\x1b[?1049l"),
        "restore sequence emitted by a process that never entered TUI mode"
    );
}
