//! The fuzz targets: the parser entry points of the fault layer
//! (`tests/fault/parsers/targets.rs`, included here so both run the same
//! code), each fed libFuzzer's bytes as text; the editing sessions
//! (`tests/fault/sessions/targets.rs`), which read the bytes as steps; and
//! widget trees with stylesheets (`tests/fault/trees/targets.rs`), drawn
//! through the real pipeline.

#[path = "../../tests/fault/parsers/targets.rs"]
mod targets;

#[path = "../../tests/fault/sessions/targets.rs"]
mod sessions;

#[path = "../../tests/fault/trees/targets.rs"]
mod trees;

pub use sessions::editing;
pub use trees::trees;

pub use targets::{corpus, targets, Target};

/// Run the target named `name` on `data`, read as UTF-8 with invalid bytes
/// replaced
pub fn run(name: &str, data: &[u8]) {
    let (_, run, _) = targets()
        .into_iter()
        .find(|t| t.0 == name)
        .unwrap_or_else(|| panic!("no fuzz target {name:?} in this build"));
    run(&String::from_utf8_lossy(data));
}
