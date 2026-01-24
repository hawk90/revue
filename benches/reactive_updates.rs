//! Reactive system benchmarks
//!
//! Benchmarks for the signal/computed reactive system.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::reactive::{computed, effect, signal};

/// Benchmark signal creation and updates
fn bench_signal_updates(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_updates");

    // Basic signal update
    group.bench_function("single", |b| {
        let sig = signal(0);
        b.iter(|| {
            sig.set(black_box(42));
        });
    });

    // Multiple signal updates
    group.bench_function("batch_100", |b| {
        let sig = signal(0);
        b.iter(|| {
            for i in 0..100 {
                sig.set(black_box(i));
            }
        });
    });

    group.finish();
}

/// Benchmark signal with subscribers
fn bench_signal_subscribers(c: &mut Criterion) {
    let mut group = c.benchmark_group("signal_subscribers");

    for sub_count in [1, 10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("update_with_subs", sub_count),
            sub_count,
            |b, &count| {
                let sig = signal(0);
                let mut _subs = Vec::new();

                // Create subscribers
                for _ in 0..count {
                    let sig_clone = sig.clone();
                    _subs.push(sig_clone.subscribe(|| {}));
                }

                b.iter(|| {
                    sig.set(black_box(42));
                });
            },
        );
    }

    group.finish();
}

/// Benchmark computed values
fn bench_computed(c: &mut Criterion) {
    let mut group = c.benchmark_group("computed");

    // Simple computed
    group.bench_function("simple", |b| {
        let source = signal(10);
        let source_c = source.clone();
        let comp = computed(move || source_c.get() * 2);

        b.iter(|| {
            source.set(black_box(20));
            black_box(comp.get());
        });
    });

    // Computed chain
    group.bench_function("chain_3", |b| {
        let s1 = signal(1);
        let s1_c = s1.clone();
        let c1 = computed(move || s1_c.get() + 1);

        let s1_c2 = s1.clone();
        let c1_c = c1.clone();
        let c2 = computed(move || c1_c.get() + s1_c2.get());

        let c2_c = c2.clone();
        let _c3 = computed(move || c2_c.get() * 2);

        b.iter(|| {
            s1.set(black_box(5));
            let _val = c2.get();
        });
    });

    // Multiple computed from same source
    group.bench_function("multiple_sources", |b| {
        let source = signal(10);
        let source_c = source.clone();
        let comp1 = computed(move || source_c.get() + 1);

        let source_c2 = source.clone();
        let comp2 = computed(move || source_c2.get() * 2);

        let source_c3 = source.clone();
        let comp3 = computed(move || source_c3.get() - 1);

        b.iter(|| {
            source.set(black_box(20));
            black_box(comp1.get());
            black_box(comp2.get());
            black_box(comp3.get());
        });
    });

    group.finish();
}

/// Benchmark effects
fn bench_effects(c: &mut Criterion) {
    let mut group = c.benchmark_group("effects");

    // Basic effect
    group.bench_function("basic", |b| {
        let sig = signal(42);
        let _counter = std::sync::atomic::AtomicUsize::new(0);

        let sig_c = sig.clone();
        let _eff = effect(move || {
            let _val = sig_c.get();
            // counter.fetch_add(1, Ordering::Relaxed);
        });

        b.iter(|| {
            sig.set(black_box(43));
        });
    });

    // Effect with computed
    group.bench_function("with_computed", |b| {
        let source = signal(10);
        let source_c = source.clone();
        let comp = computed(move || source_c.get() * 2);

        let comp_c = comp.clone();
        let _eff = effect(move || {
            let _val = comp_c.get();
        });

        b.iter(|| {
            source.set(black_box(20));
        });
    });

    group.finish();
}

/// Benchmark complex derived state
fn bench_derived_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("derived_state");

    // Map operation
    group.bench_function("map", |b| {
        let source = signal(vec![1, 2, 3, 4, 5]);
        let source_c = source.clone();
        let mapped = computed(move || source_c.get().iter().map(|x| x * 2).collect::<Vec<_>>());

        b.iter(|| {
            black_box(mapped.get());
        });
    });

    // Filter operation
    group.bench_function("filter", |b| {
        let source = signal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let source_c = source.clone();
        let filtered = computed(move || {
            source_c
                .get()
                .iter()
                .filter(|x| *x % 2 == 0)
                .cloned()
                .collect::<Vec<_>>()
        });

        b.iter(|| {
            black_box(filtered.get());
        });
    });

    // Combined map and filter
    group.bench_function("map_filter", |b| {
        let source = signal(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let source_c = source.clone();
        let transformed = computed(move || {
            source_c
                .get()
                .iter()
                .filter(|x| *x % 2 == 0)
                .map(|x| x * x)
                .collect::<Vec<_>>()
        });

        b.iter(|| {
            black_box(transformed.get());
        });
    });

    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory");

    // Repeated signal creation
    group.bench_function("create_signals", |b| {
        b.iter(|| {
            for i in 0..100 {
                black_box(signal(i));
            }
        });
    });

    // Signal cloning
    group.bench_function("clone_signal", |b| {
        let sig = signal(42);
        b.iter(|| {
            for _ in 0..10 {
                black_box(sig.clone());
            }
        });
    });

    group.finish();
}

/// Benchmark notification propagation
fn bench_propagation(c: &mut Criterion) {
    let mut group = c.benchmark_group("propagation");

    // Single level propagation
    group.bench_function("single_level", |b| {
        let source = signal(0);
        let source_c = source.clone();
        let _derived = computed(move || source_c.get() + 1);

        b.iter(|| {
            source.set(black_box(1));
        });
    });

    // Multi-level propagation (source -> c1 -> c2 -> c3)
    group.bench_function("three_levels", |b| {
        let source = signal(0);
        let s_c = source.clone();

        let c1 = computed(move || s_c.get() + 1);
        let c1_c = c1.clone();

        let c2 = computed(move || c1_c.get() + 1);
        let c2_c = c2.clone();

        let _c3 = computed(move || c2_c.get() + 1);

        b.iter(|| {
            source.set(black_box(1));
        });
    });

    // Fan-out propagation (one source, many derived)
    group.bench_function("fan_out_10", |b| {
        let source = signal(0);
        let mut derived = Vec::new();

        for _ in 0..10 {
            let s = source.clone();
            derived.push(computed(move || s.get() + 1));
        }

        b.iter(|| {
            source.set(black_box(1));
            for d in &derived {
                black_box(d.get());
            }
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_signal_updates,
    bench_signal_subscribers,
    bench_computed,
    bench_effects,
    bench_derived_state,
    bench_memory,
    bench_propagation,
);

criterion_main!(benches);
