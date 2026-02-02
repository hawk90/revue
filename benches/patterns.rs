//! Pattern benchmarks
//!
//! Benchmarks for pattern operations like confirm, form state, etc.

use criterion::{criterion_group, criterion_main, Criterion};
use revue::patterns::{ConfirmAction, ConfirmState, FormState};

/// Benchmark confirm dialog pattern
fn bench_confirm_dialog(c: &mut Criterion) {
    let mut group = c.benchmark_group("confirm_dialog");

    group.bench_function("create_confirm", |b| {
        b.iter(|| {
            let _confirm = ConfirmState::new();
        });
    });

    group.bench_function("confirm_with_action", |b| {
        b.iter(|| {
            let mut confirm = ConfirmState::new();
            confirm.request(ConfirmAction::Delete);
        });
    });

    group.finish();
}

/// Benchmark form state management
fn bench_form_state(c: &mut Criterion) {
    let mut group = c.benchmark_group("form_state");

    group.bench_function("form_empty", |b| {
        b.iter(|| {
            let _form = FormState::new();
        });
    });

    group.bench_function("form_with_fields", |b| {
        b.iter(|| {
            FormState::new()
                .field("username", |f| f.label("Username"))
                .field("email", |f| f.label("Email"))
                .field("password", |f| f.label("Password"))
                .field("bio", |f| f.label("Bio"))
                .build();
        });
    });

    group.bench_function("form_with_validators", |b| {
        b.iter(|| {
            FormState::new()
                .field("username", |f| f.min_length(3))
                .field("age", |f| f.min(0.0).max(150.0))
                .field("email", |f| f.email())
                .build();
        });
    });

    group.finish();
}

/// Benchmark pattern composition
fn bench_pattern_composition(c: &mut Criterion) {
    let mut group = c.benchmark_group("pattern_composition");

    group.bench_function("form_with_confirm", |b| {
        b.iter(|| {
            let _form = FormState::new()
                .field("action", |f| f.label("Action"))
                .build();

            let _confirm = ConfirmState::new();

            (_form, _confirm)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_confirm_dialog,
    bench_form_state,
    bench_pattern_composition
);

criterion_main!(benches);
