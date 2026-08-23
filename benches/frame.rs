//! What a frame costs now that every draw renders and diffs in full.
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::prelude::*;
use revue::testing::PipelineHarness;

struct Rows {
    n: usize,
    tick: usize,
}
impl View for Rows {
    fn render(&self, ctx: &mut RenderContext) {
        let mut stack = vstack().element_id("root");
        for i in 0..self.n {
            stack = stack.child(
                Text::new(format!("row {} value {}", i, self.tick)).element_id(format!("r{}", i)),
            );
        }
        stack.render(ctx);
    }
    fn widget_type(&self) -> &'static str {
        "Rows"
    }
    fn id(&self) -> Option<&str> {
        Some("app")
    }
}

fn bench_frame(c: &mut Criterion) {
    let mut group = c.benchmark_group("frame");
    for rows in [10usize, 50, 200] {
        group.bench_with_input(BenchmarkId::new("changed", rows), &rows, |b, &rows| {
            let mut h = PipelineHarness::new(120, 40).incremental_dom(true);
            h.draw(&Rows { n: rows, tick: 0 });
            let mut tick = 0usize;
            b.iter(|| {
                tick += 1;
                h.draw(&Rows { n: rows, tick });
                std::hint::black_box(h.frame_count());
            });
        });
        group.bench_with_input(
            BenchmarkId::new("dom_from_render", rows),
            &rows,
            |b, &rows| {
                let mut h = PipelineHarness::new(120, 40).dom_from_render(true);
                h.draw(&Rows { n: rows, tick: 0 });
                let mut tick = 0usize;
                b.iter(|| {
                    tick += 1;
                    h.draw(&Rows { n: rows, tick });
                    std::hint::black_box(h.frame_count());
                });
            },
        );
        group.bench_with_input(BenchmarkId::new("unchanged", rows), &rows, |b, &rows| {
            let mut h = PipelineHarness::new(120, 40).incremental_dom(true);
            h.draw(&Rows { n: rows, tick: 0 });
            b.iter(|| {
                h.draw(&Rows { n: rows, tick: 0 });
                std::hint::black_box(h.frame_count());
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_frame);
criterion_main!(benches);
