//! CSS benchmarks
//!
//! Benchmarks for CSS parsing and style resolution.

use criterion::{criterion_group, criterion_main, Criterion};
use revue::style::{parse_css, Color};

/// Benchmark CSS parsing
fn bench_css_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("css_parse");

    // Simple CSS
    let simple_css = r#"
        .button {
            color: #ffffff;
            background: #333333;
        }
    "#;

    group.bench_function("simple", |b| {
        b.iter(|| {
            std::hint::black_box(parse_css(simple_css).unwrap());
        });
    });

    // Medium CSS with multiple rules
    let medium_css = r#"
        :root {
            --primary: #3b82f6;
            --secondary: #6b7280;
            --success: #22c55e;
            --danger: #ef4444;
        }

        .container {
            display: flex;
            flex-direction: column;
            padding: 8px;
            gap: 4px;
        }

        .button {
            color: var(--primary);
            padding: 4px 8px;
            border: 1px solid var(--primary);
        }

        .button:hover {
            background: var(--primary);
            color: white;
        }

        .input {
            border: 1px solid var(--secondary);
            padding: 4px;
        }

        .input:focus {
            border-color: var(--primary);
        }
    "#;

    group.bench_function("medium", |b| {
        b.iter(|| {
            std::hint::black_box(parse_css(medium_css).unwrap());
        });
    });

    // Large CSS with many rules
    let large_css = generate_large_css(100);

    group.bench_function("large_100_rules", |b| {
        b.iter(|| {
            std::hint::black_box(parse_css(&large_css).unwrap());
        });
    });

    group.finish();
}

/// Generate large CSS for benchmarking
fn generate_large_css(rule_count: usize) -> String {
    let mut css = String::new();
    for i in 0..rule_count {
        css.push_str(&format!(
            r#"
            .widget-{} {{
                color: #{:02x}{:02x}{:02x};
                background: #{:02x}{:02x}{:02x};
                padding: {}px;
                margin: {}px;
            }}
            "#,
            i,
            (i * 3) % 256,
            (i * 5) % 256,
            (i * 7) % 256,
            (i * 11) % 256,
            (i * 13) % 256,
            (i * 17) % 256,
            i % 20,
            i % 10,
        ));
    }
    css
}

/// Benchmark style application
fn bench_style_apply(c: &mut Criterion) {
    let mut group = c.benchmark_group("style_apply");

    let css = r#"
        .button {
            color: #ffffff;
            background: #333333;
            padding: 4px 8px;
            border: 1px solid #666666;
        }
    "#;

    let sheet = parse_css(css).unwrap();
    let base_style = revue::style::Style::default();

    group.bench_function("single_rule", |b| {
        b.iter(|| {
            std::hint::black_box(sheet.apply(".button", &base_style));
        });
    });

    group.finish();
}

/// Benchmark color operations
fn bench_color_ops(c: &mut Criterion) {
    let mut group = c.benchmark_group("color_ops");

    group.bench_function("rgb", |b| {
        b.iter(|| {
            std::hint::black_box(Color::rgb(255, 128, 64));
        });
    });

    group.bench_function("hex_u32", |b| {
        b.iter(|| {
            std::hint::black_box(Color::hex(0xFFFFFF));
        });
    });

    group.bench_function("rgba", |b| {
        b.iter(|| {
            std::hint::black_box(Color::rgba(255, 128, 64, 200));
        });
    });

    group.finish();
}

/// Benchmark selector matching
fn bench_selector_match(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_match");

    let css = r#"
        button { color: red; }
        .primary { color: blue; }
        #submit { color: green; }
        button.primary { color: purple; }
        button.primary:hover { color: orange; }
    "#;

    let sheet = parse_css(css).unwrap();

    group.bench_function("simple_element", |b| {
        let base = revue::style::Style::default();
        b.iter(|| {
            std::hint::black_box(sheet.apply("button", &base));
        });
    });

    group.bench_function("class", |b| {
        let base = revue::style::Style::default();
        b.iter(|| {
            std::hint::black_box(sheet.apply(".primary", &base));
        });
    });

    group.bench_function("id", |b| {
        let base = revue::style::Style::default();
        b.iter(|| {
            std::hint::black_box(sheet.apply("#submit", &base));
        });
    });

    group.finish();
}

/// Benchmark selector matching with many rules
/// Tests the performance impact of selector indexing (PR #345)
fn bench_selector_indexing(c: &mut Criterion) {
    let mut group = c.benchmark_group("selector_indexing");

    // Generate CSS with many rules to test indexing benefits
    let css = generate_large_css(100);

    let sheet = parse_css(&css).unwrap();
    let base = revue::style::Style::default();

    // Matching a specific class should be fast with indexing
    group.bench_function("match_class_in_100_rules", |b| {
        b.iter(|| {
            std::hint::black_box(sheet.apply(".widget-50", &base));
        });
    });

    // Matching element should be fast
    group.bench_function("match_element_in_100_rules", |b| {
        b.iter(|| {
            // Note: This tests the old API, uses internal matching
            std::hint::black_box(sheet.apply("button", &base));
        });
    });

    // Test with different selector types
    for rule_count in [10, 50, 100, 500].iter() {
        let css = generate_large_css(*rule_count);
        let sheet = parse_css(&css).unwrap();

        group.bench_with_input(
            criterion::BenchmarkId::new("match_specific", rule_count),
            rule_count,
            |b, &_count| {
                b.iter(|| {
                    std::hint::black_box(sheet.apply(".widget-0", &base));
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_css_parse,
    bench_style_apply,
    bench_color_ops,
    bench_selector_match,
    bench_selector_indexing,
);

criterion_main!(benches);
