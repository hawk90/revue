//! Render benchmarks
//!
//! Benchmarks for the rendering pipeline.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::prelude::*;
use revue::render::Buffer;
use revue::testing::TestApp;

/// Benchmark simple text rendering
fn bench_text_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_render");

    for size in [10, 100, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let text_content: String = "Hello World! ".repeat(size / 13 + 1);
            let view = text(&text_content);
            let mut app = TestApp::new(view);

            b.iter(|| {
                app.render();
                black_box(&app);
            });
        });
    }

    group.finish();
}

/// Benchmark nested layout rendering
fn bench_nested_layout(c: &mut Criterion) {
    let mut group = c.benchmark_group("nested_layout");

    for depth in [1, 3, 5].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(depth), depth, |b, &depth| {
            // Create a nested vstack structure
            let mut view = vstack().child(text("Leaf"));
            for i in 0..depth {
                view = vstack().child(text(&format!("Level {}", i))).child(view);
            }

            let mut app = TestApp::new(view);

            b.iter(|| {
                app.render();
                black_box(&app);
            });
        });
    }

    group.finish();
}

/// Benchmark list widget with many items
fn bench_list_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("list_render");

    for item_count in [10, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(item_count),
            item_count,
            |b, &count| {
                let items: Vec<String> = (0..count).map(|i| format!("Item {}", i)).collect();
                let view = List::new(items);
                let mut app = TestApp::new(view);

                b.iter(|| {
                    app.render();
                    black_box(&app);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark table rendering
fn bench_table_render(c: &mut Criterion) {
    let mut group = c.benchmark_group("table_render");

    for row_count in [10, 50, 100].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(row_count),
            row_count,
            |b, &count| {
                let mut table = Table::new(vec![
                    Column::new("ID").width(10),
                    Column::new("Name").width(20),
                    Column::new("Value").width(15),
                ]);

                for i in 0..count {
                    table = table.row(vec![
                        format!("{}", i),
                        format!("Item {}", i),
                        format!("{:.2}", i as f64 * 1.5),
                    ]);
                }

                let mut app = TestApp::new(table);

                b.iter(|| {
                    app.render();
                    black_box(&app);
                });
            },
        );
    }

    group.finish();
}

/// Benchmark buffer operations
fn bench_buffer_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_ops");

    // Buffer creation
    group.bench_function("create_80x24", |b| {
        b.iter(|| {
            black_box(Buffer::new(80, 24));
        });
    });

    // Buffer clear
    group.bench_function("clear_80x24", |b| {
        let mut buffer = Buffer::new(80, 24);
        b.iter(|| {
            buffer.clear();
            black_box(&buffer);
        });
    });

    // Buffer resize
    group.bench_function("resize", |b| {
        let mut buffer = Buffer::new(80, 24);
        b.iter(|| {
            buffer.resize(120, 40);
            buffer.resize(80, 24);
            black_box(&buffer);
        });
    });

    // put_str performance
    group.bench_function("put_str_short", |b| {
        let mut buffer = Buffer::new(80, 24);
        b.iter(|| {
            buffer.put_str(0, 0, "Hello, World!");
            black_box(&buffer);
        });
    });

    group.bench_function("put_str_long", |b| {
        let mut buffer = Buffer::new(80, 24);
        let long_str = "X".repeat(80);
        b.iter(|| {
            buffer.put_str(0, 0, &long_str);
            black_box(&buffer);
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_text_render,
    bench_nested_layout,
    bench_list_render,
    bench_table_render,
    bench_buffer_ops,
);

criterion_main!(benches);
