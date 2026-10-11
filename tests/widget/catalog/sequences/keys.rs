//! Layer 2a - key sequences: every widget with a key handler driven with
//! random sequences of keys from [`KEYS`](super::KEYS), interleaved with the
//! widget's data mutations (clear, fewer items, set value, ...) and renders at
//! random sizes.
//!
//! Case keys: `"<widget> <shrunk sequence>"`.

use super::{catalog, run_layer, Weights};

/// Random sequences per widget.
const CASES: u32 = 512;

#[test]
fn stateful_widgets_survive_key_sequences() {
    run_layer(
        "keys",
        catalog::catalog(),
        Weights {
            key: 12,
            mutate: 2,
            render: 2,
            mouse: 0,
        },
        CASES,
    );
}
