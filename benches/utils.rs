//! Utility function benchmarks
//!
//! Benchmarks for utility functions like path manipulation, text processing, etc.

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use revue::utils::path::{abbreviate_path, home_relative, shorten_path};

/// Benchmark path abbreviation
fn bench_path_abbreviation(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_abbreviation");

    let short_path = "/Users/john/Documents/file.txt";
    let medium_path = "/Users/john/Documents/Projects/rust-project/src/main.rs";
    let long_path = "/very/long/path/to/some/deeply/nested/directory/structure/file.txt";

    group.bench_function("short_path", |b| {
        b.iter(|| abbreviate_path(short_path));
    });

    group.bench_function("medium_path", |b| {
        b.iter(|| abbreviate_path(medium_path));
    });

    group.bench_function("long_path", |b| {
        b.iter(|| abbreviate_path(long_path));
    });

    group.finish();
}

/// Benchmark path shortening
fn bench_path_shortening(c: &mut Criterion) {
    let mut group = c.benchmark_group("path_shortening");

    let path = "/a/b/c/d/e/f/g/h/i/j/k/l/m/n/o/p/q/r/s/t/u/v/w/x/y/z/file.txt";

    for width in [20, 40, 60, 80].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(width), width, |b, width| {
            b.iter(|| shorten_path(path, *width));
        });
    }

    group.finish();
}

/// Benchmark home path replacement
fn bench_home_replacement(c: &mut Criterion) {
    let mut group = c.benchmark_group("home_replacement");

    let paths = vec![
        "/Users/john/Documents/file.txt",
        "/home/user/projects/rust/src/main.rs",
        "/Users/verylong/path/to/some/file.txt",
        "/home/user/.config/app/config.json",
    ];

    for (i, path) in paths.iter().enumerate() {
        group.bench_with_input(BenchmarkId::from_parameter(i), &i, |b, _| {
            b.iter(|| home_relative(path));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_path_abbreviation,
    bench_path_shortening,
    bench_home_replacement
);

criterion_main!(benches);
