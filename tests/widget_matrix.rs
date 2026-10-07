//! The widget matrix: every public widget rendered across boundary sizes,
//! edge contents, focus states and render paths, checked for two invariants -
//! no panic, and nothing written outside the widget's area.
//!
//! Three layers, one file each, so a failure names its layer:
//!
//! - `matrix/sizes.rs` - boundary areas, one ordinary content;
//! - `matrix/contents.rs` - edge contents, one ordinary area;
//! - `matrix/pairwise.rs` - all pairs of {size, content, focus, path}.
//!
//! Known failures live in `matrix/known_failures.rs`; the list may only shrink.
//! See `docs/refactor/findings-widget-matrix.md`.

mod matrix;
