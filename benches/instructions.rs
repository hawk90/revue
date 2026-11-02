//! Instruction counts of the hot paths, under Callgrind (#1168).
//!
//! Criterion's times only compare on one machine in one session, so CI cannot
//! gate on them. Instruction counts are the same on every run of the same
//! code, so the Instructions job in CI compares a PR with its base and fails
//! past a set regression. Criterion stays for local wall-clock measurement.
//!
//! Runs on Linux with Valgrind and `gungraun-runner` of the same version as
//! the `gungraun` dev-dependency:
//!
//! ```text
//! cargo bench --bench instructions
//! ```
//!
//! Each benchmark does one operation: setup builds the state it starts from,
//! and is not counted.

use gungraun::prelude::*;
use revue::dom::{DomId, DomRenderer};
use revue::layout::LayoutEngine;
use revue::prelude::*;
use revue::style::{parse_css, Style};
use revue::testing::{PipelineHarness, TestApp};
use std::hint::black_box;

/// A list of rows; `tick` changes every row's text.
struct Rows {
    n: usize,
    tick: usize,
}

impl View for Rows {
    fn render(&self, ctx: &mut RenderContext) {
        let mut stack = vstack().element_id("root");
        for i in 0..self.n {
            stack = stack.child(
                Text::new(format!("row {i} value {}", self.tick))
                    .element_id(format!("r{i}"))
                    .class(if i % 2 == 0 { "even" } else { "odd" }),
            );
        }
        stack.render(ctx);
    }
}

const ROWS_CSS: &str = "
    .even { color: rgb(200, 200, 200); }
    .odd { color: rgb(150, 150, 150); background: rgb(20, 20, 20); }
    .even + .odd { padding: 0; }
    Text:focus { color: yellow; }
";

/// A stylesheet of `n` class rules.
fn large_css(n: usize) -> String {
    (0..n)
        .map(|i| {
            format!(
                ".widget-{i} {{ color: rgb({}, {}, {}); padding: {}; }}\n",
                i * 3 % 256,
                i * 5 % 256,
                i * 7 % 256,
                i % 3
            )
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Frame: the whole pipeline, as an app draws it

/// A harness past its first frames, the way a real app is configured
/// (`dom_from_render`, CSS layout, no layout engine).
fn settled(rows: usize) -> (PipelineHarness, usize) {
    let mut harness = PipelineHarness::with_css(ROWS_CSS, 120, 60).layout_engine(false);
    harness.draw(&Rows { n: rows, tick: 0 });
    harness.draw(&Rows { n: rows, tick: 0 });
    (harness, rows)
}

#[library_benchmark]
#[bench::rows_10(args = (10), setup = settled)]
#[bench::rows_50(args = (50), setup = settled)]
fn frame_unchanged((mut harness, rows): (PipelineHarness, usize)) -> PipelineHarness {
    harness.draw(black_box(&Rows { n: rows, tick: 0 }));
    harness
}

/// The changed text reached the screen: the frame did the work it is
/// counted for (REV-PERF-007). Teardown is not counted.
fn shows_tick_1(harness: PipelineHarness) {
    assert!(
        harness.contains("row 0 value 1"),
        "{}",
        harness.screen_text()
    );
}

/// A repaint sends the whole screen again.
fn wrote_the_screen(harness: PipelineHarness) {
    let output = String::from_utf8_lossy(harness.terminal_output());
    assert!(output.matches("row 0 value 0").count() >= 2, "no repaint");
}

#[library_benchmark]
#[bench::rows_10(args = (10), setup = settled, teardown = shows_tick_1)]
#[bench::rows_50(args = (50), setup = settled, teardown = shows_tick_1)]
fn frame_changed((mut harness, rows): (PipelineHarness, usize)) -> PipelineHarness {
    harness.draw(black_box(&Rows { n: rows, tick: 1 }));
    harness
}

// A full repaint of styled rows, as after a resize or a stylesheet reload.
#[library_benchmark]
#[bench::rows_50(args = (50), setup = settled, teardown = wrote_the_screen)]
fn frame_repaint((mut harness, rows): (PipelineHarness, usize)) -> PipelineHarness {
    harness.request_redraw();
    harness.draw(black_box(&Rows { n: rows, tick: 0 }));
    harness
}

library_benchmark_group!(
    name = frame,
    benchmarks = [frame_unchanged, frame_changed, frame_repaint]
);

// ---------------------------------------------------------------------------
// DOM: build and reconcile

/// The same rows as a widget tree the DOM builder walks: a custom view's
/// `render` is not seen by `DomRenderer::build`, so `Rows` would build one node.
fn rows_tree(rows: usize, tick: usize) -> Stack {
    let mut stack = vstack().element_id("root");
    for i in 0..rows {
        stack = stack.child(
            Text::new(format!("row {i} value {tick}"))
                .element_id(format!("r{i}"))
                .class(if i % 2 == 0 { "even" } else { "odd" }),
        );
    }
    stack
}

fn tree(rows: usize) -> Stack {
    rows_tree(rows, 0)
}

fn built(rows: usize) -> (DomRenderer, Stack) {
    let mut renderer = DomRenderer::new();
    renderer.build(&rows_tree(rows, 0));
    (renderer, rows_tree(rows, 1))
}

/// Every row became a node: the build walked the tree (REV-PERF-007).
fn has_all_rows(renderer: DomRenderer) {
    assert!(
        renderer.tree().len() > 50,
        "{} nodes",
        renderer.tree().len()
    );
}

#[library_benchmark]
#[bench::rows_50(args = (50), setup = tree, teardown = has_all_rows)]
fn dom_build(view: Stack) -> DomRenderer {
    let mut renderer = DomRenderer::new();
    renderer.build(black_box(&view));
    renderer
}

#[library_benchmark]
#[bench::rows_50(args = (50), setup = built, teardown = has_all_rows)]
fn dom_reconcile_text_change((mut renderer, view): (DomRenderer, Stack)) -> DomRenderer {
    renderer.build(black_box(&view));
    renderer
}

library_benchmark_group!(
    name = dom,
    benchmarks = [dom_build, dom_reconcile_text_change]
);

// ---------------------------------------------------------------------------
// CSS: parse and cascade

#[library_benchmark]
#[bench::rules_100(args = (100), setup = large_css)]
fn css_parse(css: String) -> revue::style::StyleSheet {
    parse_css(black_box(&css)).unwrap()
}

fn sheet(rules: usize) -> (revue::style::StyleSheet, Style) {
    (parse_css(&large_css(rules)).unwrap(), Style::default())
}

#[library_benchmark]
#[bench::rules_100(args = (100), setup = sheet)]
fn css_apply((sheet, base): (revue::style::StyleSheet, Style)) -> Style {
    sheet.apply(black_box(".widget-50"), &base)
}

library_benchmark_group!(name = css, benchmarks = [css_parse, css_apply]);

// ---------------------------------------------------------------------------
// Layout

fn flat_tree(children: usize) -> (LayoutEngine, DomId) {
    let mut engine = LayoutEngine::new();
    let ids: Vec<DomId> = (1..=children as u64).map(DomId::new).collect();
    for &id in &ids {
        let _ = engine.create_node(id, &Style::default());
    }
    let parent = DomId::new(children as u64 + 1);
    let _ = engine.create_node_with_children(parent, &Style::default(), &ids);
    (engine, parent)
}

#[library_benchmark]
#[bench::children_50(args = (50), setup = flat_tree)]
fn layout_compute((mut engine, parent): (LayoutEngine, DomId)) -> LayoutEngine {
    let _ = engine.compute(black_box(parent), 80, 24);
    engine
}

library_benchmark_group!(name = layout, benchmarks = [layout_compute]);

// ---------------------------------------------------------------------------
// Widgets: one render of each

fn list_app(items: usize) -> TestApp<List<String>> {
    let items: Vec<String> = (0..items).map(|i| format!("Item {i}")).collect();
    TestApp::new(list(items).selected(0))
}

fn table_app(rows: usize) -> TestApp<Table> {
    let mut table = table((0..5).map(|c| column(format!("Col{c}"))).collect());
    for r in 0..rows {
        table = table.row((0..5).map(|c| format!("R{r}C{c}")).collect());
    }
    TestApp::new(table)
}

fn text_app(words: usize) -> TestApp<Text> {
    let text: String = (0..words).map(|i| format!("word{i} ")).collect();
    TestApp::new(Text::new(text))
}

#[library_benchmark]
#[bench::items_100(args = (100), setup = list_app)]
fn widget_list(mut app: TestApp<List<String>>) -> TestApp<List<String>> {
    app.render();
    app
}

#[library_benchmark]
#[bench::rows_20(args = (20), setup = table_app)]
fn widget_table(mut app: TestApp<Table>) -> TestApp<Table> {
    app.render();
    app
}

#[library_benchmark]
#[bench::words_200(args = (200), setup = text_app)]
fn widget_text(mut app: TestApp<Text>) -> TestApp<Text> {
    app.render();
    app
}

library_benchmark_group!(
    name = widgets,
    benchmarks = [widget_list, widget_table, widget_text]
);

main!(library_benchmark_groups = frame, dom, css, layout, widgets);
