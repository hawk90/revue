//! Code that must not compile, with its error pinned (#1161). The `.stderr`
//! files next to the cases hold the expected messages; regenerate them with
//! `TRYBUILD=overwrite cargo test --test compile_fail` after a deliberate
//! change.

#[path = "support/alloc_cap.rs"]
mod alloc_cap;

#[test]
fn compile_fail() {
    trybuild::TestCases::new().compile_fail("tests/compile_fail/*.rs");
}
