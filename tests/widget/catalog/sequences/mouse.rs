//! Layer 2b - mouse sequences: every widget with a mouse handler driven with
//! random clicks, drags, scrolls and moves at coordinates in and out of its
//! area (its edges, one past them, 0 and `u16::MAX`), interleaved with a few
//! keys, its data mutations and renders at random sizes.
//!
//! Case keys: `"<widget> <shrunk sequence>"`.

use super::{catalog, run_layer, Weights};

/// Random sequences per widget.
const CASES: u32 = 512;

#[test]
fn stateful_widgets_survive_mouse_sequences() {
    let entries = catalog::catalog().into_iter().filter(has_mouse).collect();
    run_layer(
        "mouse",
        entries,
        Weights {
            key: 2,
            mutate: 1,
            render: 2,
            mouse: 10,
        },
        CASES,
    );
}

fn has_mouse(entry: &catalog::Entry) -> bool {
    (entry.build)().has_mouse()
}
