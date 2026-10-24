//! Widget trees: random trees of stacks, borders, text, lists, progress
//! bars and scroll views with random stylesheets, drawn through the real
//! pipeline (`targets.rs`, which the `fuzz` crate runs under libFuzzer
//! too).
//!
//! Invariants: nothing panics, and drawing one tree after another gives the
//! screen a fresh app gives the second tree (the DOM is reconciled from the
//! first to the second without leaving anything of the first behind). Then
//! events go through the app's dispatch with a draw after each.
//!
//! The byte streams come from a fixed xorshift seed. Case keys:
//! `"trees random"`.

use std::collections::BTreeSet;

use super::{catch, ratchet, Failure};

mod targets;

const LAYER: &str = "trees";
const CASES: u64 = 1000;
const BYTES: usize = 256;

/// `len` bytes from a xorshift stream started at `seed`
fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x as u8
        })
        .collect()
}

#[test]
fn widget_trees() {
    let mut failures = Vec::new();
    for seed in 0..CASES {
        let data = bytes(seed, BYTES);
        if let Err(why) = catch(|| targets::trees(&data)) {
            failures.push(Failure::new(
                "trees random",
                format!("{why} on seed {seed} (bytes(seed, {BYTES}))"),
            ));
            break;
        }
    }
    let cases = BTreeSet::from(["trees random".to_string()]);
    ratchet(LAYER, &cases, failures);
}
