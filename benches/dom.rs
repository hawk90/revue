//! DOM benchmarks
//!
//! Benchmarks for DOM tree operations and incremental updates.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::dom::DomRenderer;
use revue::widget::{Stack, Text};

/// Benchmark DOM build operations
fn bench_dom_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("dom_build");

    // Simple view - single node
    group.bench_function("simple", |b| {
        let view = Text::new("Hello").element_id("greeting");
        b.iter(|| {
            let mut renderer = DomRenderer::new();
            renderer.build(&view);
            std::hint::black_box(&renderer);
        });
    });

    // Medium view - nested structure
    group.bench_function("nested_5_levels", |b| {
        let view = Stack::new().element_id("root").child(
            Stack::new().element_id("l1").child(
                Stack::new().element_id("l2").child(
                    Stack::new().element_id("l3").child(
                        Stack::new()
                            .element_id("l4")
                            .child(Text::new("Deep").element_id("leaf")),
                    ),
                ),
            ),
        );

        b.iter(|| {
            let mut renderer = DomRenderer::new();
            renderer.build(&view);
            std::hint::black_box(&renderer);
        });
    });

    group.finish();
}

/// Benchmark incremental DOM updates
fn bench_dom_incremental(c: &mut Criterion) {
    let mut group = c.benchmark_group("dom_incremental");

    // Create initial view
    let create_view = |suffix: &str| {
        Stack::new()
            .element_id("root")
            .child(Text::new(format!("Item 1 {}", suffix)).element_id("item1"))
            .child(Text::new(format!("Item 2 {}", suffix)).element_id("item2"))
            .child(Text::new(format!("Item 3 {}", suffix)).element_id("item3"))
    };

    // The view is built once and reused by every arm below. Constructing it
    // inside `iter` would charge view-construction cost to whichever arm did
    // it, making the fresh/incremental comparison meaningless - which is
    // exactly the gate Phase 1 has to pass.
    let stable_view = create_view("");

    // Benchmark: rebuild from scratch each time
    group.bench_function("fresh_build", |b| {
        b.iter(|| {
            let mut renderer = DomRenderer::new();
            renderer.build(&stable_view);
            std::hint::black_box(&renderer);
        });
    });

    // Benchmark: incremental update, structure and content unchanged.
    // This is the cost of an idle frame once reconciliation runs every frame.
    group.bench_function("incremental_same", |b| {
        let mut renderer = DomRenderer::new();
        renderer.build(&stable_view);

        b.iter(|| {
            renderer.build(&stable_view);
            std::hint::black_box(&renderer);
        });
    });

    // Benchmark: incremental update with minor changes
    let view_a = create_view("A");
    let view_b = create_view("B");
    group.bench_function("incremental_text_change", |b| {
        let mut renderer = DomRenderer::new();
        renderer.build(&view_a);
        let mut counter = 0;

        b.iter(|| {
            counter += 1;
            let view = if counter % 2 == 0 { &view_a } else { &view_b };
            renderer.build(view);
            std::hint::black_box(&renderer);
        });
    });

    group.finish();
}

/// Benchmark DOM with many children
fn bench_dom_many_children(c: &mut Criterion) {
    let mut group = c.benchmark_group("dom_children");

    for count in [10, 50, 100].iter() {
        // Create view with N children
        let create_view = || {
            let mut stack = Stack::new().element_id("root");
            for i in 0..*count {
                stack =
                    stack.child(Text::new(format!("Item {}", i)).element_id(format!("item{}", i)));
            }
            stack
        };

        group.bench_with_input(BenchmarkId::new("fresh", count), count, |b, _| {
            b.iter(|| {
                let mut renderer = DomRenderer::new();
                let view = create_view();
                renderer.build(&view);
                std::hint::black_box(&renderer);
            });
        });

        group.bench_with_input(BenchmarkId::new("incremental", count), count, |b, _| {
            let mut renderer = DomRenderer::new();
            let view = create_view();
            renderer.build(&view);

            b.iter(|| {
                let view = create_view();
                renderer.build(&view);
                std::hint::black_box(&renderer);
            });
        });
    }

    group.finish();
}

/// Benchmark invalidate and rebuild
fn bench_dom_invalidate(c: &mut Criterion) {
    let mut group = c.benchmark_group("dom_invalidate");

    let view = Stack::new()
        .element_id("root")
        .child(Text::new("Hello").element_id("text1"))
        .child(Text::new("World").element_id("text2"));

    // Normal incremental rebuild
    group.bench_function("incremental", |b| {
        let mut renderer = DomRenderer::new();
        renderer.build(&view);

        b.iter(|| {
            renderer.build(&view);
            std::hint::black_box(&renderer);
        });
    });

    // Invalidate + rebuild
    group.bench_function("invalidate_rebuild", |b| {
        let mut renderer = DomRenderer::new();
        renderer.build(&view);

        b.iter(|| {
            renderer.invalidate();
            renderer.build(&view);
            std::hint::black_box(&renderer);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_dom_build,
    bench_dom_incremental,
    bench_dom_many_children,
    bench_dom_invalidate,
);

criterion_main!(benches);
