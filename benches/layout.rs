//! Layout benchmarks
//!
//! Benchmarks for the Taffy-based layout engine.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::dom::DomId;
use revue::layout::{LayoutEngine, Rect};
use revue::style::Style;
use std::sync::atomic::{AtomicU64, Ordering};

/// Counter for unique DomIds
static DOM_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a unique DomId
fn next_dom_id() -> DomId {
    DomId::new(DOM_ID_COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Benchmark layout engine creation
fn bench_layout_engine(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_engine");

    group.bench_function("create", |b| {
        b.iter(|| {
            std::hint::black_box(LayoutEngine::new());
        });
    });

    group.bench_function("create_single_node", |b| {
        b.iter(|| {
            let mut engine = LayoutEngine::new();
            engine.create_node(next_dom_id(), &Style::default());
            std::hint::black_box(engine);
        });
    });

    group.finish();
}

/// Benchmark layout computation with varying child counts
fn bench_layout_children(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout_children");

    for child_count in [3, 10, 50].iter() {
        group.bench_with_input(
            BenchmarkId::new("children", child_count),
            child_count,
            |b, &count| {
                b.iter_batched(
                    || {
                        let mut engine = LayoutEngine::new();
                        let mut children = Vec::new();

                        // Create children
                        for _ in 0..count {
                            let id = next_dom_id();
                            engine.create_node(id, &Style::default());
                            children.push(id);
                        }

                        // Create parent
                        let parent_id = next_dom_id();
                        engine.create_node_with_children(parent_id, &Style::default(), &children);

                        (engine, parent_id)
                    },
                    |(mut engine, parent_id)| {
                        engine.compute(parent_id, 80, 24);
                        std::hint::black_box(engine);
                    },
                    criterion::BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}

/// Benchmark nested layout computation
fn bench_nested_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_layout");

    for depth in [2, 5, 10].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            b.iter_batched(
                || {
                    let mut engine = LayoutEngine::new();
                    let mut current_id = next_dom_id();
                    engine.create_node(current_id, &Style::default());
                    let root_id = current_id;

                    for _ in 0..depth {
                        let child_id = next_dom_id();
                        engine.create_node(child_id, &Style::default());
                        engine.add_child(current_id, child_id);
                        current_id = child_id;
                    }

                    (engine, root_id)
                },
                |(mut engine, root_id)| {
                    engine.compute(root_id, 80, 24);
                    std::hint::black_box(engine);
                },
                criterion::BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

/// Benchmark Rect operations
fn bench_rect_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("rect_ops");

    let rect1 = Rect::new(10, 10, 50, 30);
    let rect2 = Rect::new(30, 20, 40, 40);

    group.bench_function("contains", |b| {
        b.iter(|| {
            std::hint::black_box(rect1.contains(25, 25));
        });
    });

    group.bench_function("intersects", |b| {
        b.iter(|| {
            std::hint::black_box(rect1.intersects(&rect2));
        });
    });

    group.bench_function("intersection", |b| {
        b.iter(|| {
            std::hint::black_box(rect1.intersection(&rect2));
        });
    });

    group.bench_function("union", |b| {
        b.iter(|| {
            std::hint::black_box(rect1.union(&rect2));
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_layout_engine,
    bench_layout_children,
    bench_nested_layout,
    bench_rect_ops,
);

criterion_main!(benches);
