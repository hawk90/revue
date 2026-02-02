//! Widget rendering benchmarks
//!
//! Benchmarks for widget rendering performance.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::prelude::*;
use revue::testing::TestApp;

/// Benchmark button rendering
fn bench_button_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("button_render");

    group.bench_function("button_simple", |b| {
        let button = button("Click me");
        let mut app = TestApp::new(button);

        b.iter(|| {
            app.render();
            std::hint::black_box(&app);
        });
    });

    group.finish();
}

/// Benchmark text rendering
fn bench_text_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_render");

    group.bench_function("text_short", |b| {
        let text_widget = text("Hello");
        let mut app = TestApp::new(text_widget);

        b.iter(|| {
            app.render();
            std::hint::black_box(&app);
        });
    });

    group.bench_function("text_long", |b| {
        let text_widget =
            text("This is a much longer text string that should take more time to render");
        let mut app = TestApp::new(text_widget);

        b.iter(|| {
            app.render();
            std::hint::black_box(&app);
        });
    });

    group.finish();
}

/// Benchmark list/table rendering
fn bench_list_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("list_render");

    for item_count in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(item_count),
            item_count,
            |b, item_count| {
                let items: Vec<String> = (0..*item_count).map(|i| format!("Item {}", i)).collect();

                let list = list(items).selected(0);
                let mut app = TestApp::new(list);

                b.iter(|| {
                    app.render();
                    std::hint::black_box(&app);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark table rendering
fn bench_table_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_render");

    for (rows, cols) in [(5, 3), (10, 5), (20, 10)] {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}x{}", rows, cols)),
            &(rows, cols),
            |b, (rows, cols)| {
                // Create columns
                let columns: Vec<revue::widget::Column> = (0..*cols)
                    .map(|c| revue::widget::column(format!("Col{}", c)))
                    .collect();
                let mut table = table(columns);
                for r in 0..*rows {
                    let row: Vec<String> = (0..*cols).map(|c| format!("R{}C{}", r, c)).collect();
                    table = table.row(row);
                }

                let mut app = TestApp::new(table);

                b.iter(|| {
                    app.render();
                    std::hint::black_box(&app);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark complex widget composition
fn bench_composite_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("composite_render");

    group.bench_function("vstack_10", |b| {
        let mut view = vstack();
        for i in 0..10 {
            view = view.child(text(format!("Item {}", i)));
        }
        let mut app = TestApp::new(view);

        b.iter(|| {
            app.render();
            std::hint::black_box(&app);
        });
    });

    group.bench_function("hstack_10", |b| {
        let mut view = hstack();
        for i in 0..10 {
            view = view.child(text(format!("{}", i)));
        }
        let mut app = TestApp::new(view);

        b.iter(|| {
            app.render();
            std::hint::black_box(&app);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_button_render,
    bench_text_render,
    bench_list_render,
    bench_table_render,
    bench_composite_render
);

criterion_main!(benches);
