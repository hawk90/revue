//! Editing sessions: random byte streams played as keys, pastes, new text,
//! resizes and draws on every editing widget (`targets.rs`, which the
//! `fuzz` crate runs under libFuzzer too).
//!
//! Invariants: nothing panics, no session takes longer than [`SLOW`], and
//! the cursors stay inside the text (checked in `targets.rs`).
//!
//! The streams come from a fixed xorshift seed, so the same sessions run
//! every time. Case keys: `"editing random"`.

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use super::{catch, ratchet, Failure};

mod targets;

const LAYER: &str = "sessions";
/// Sessions per run, and bytes in each
const SESSIONS: u64 = 200;
const BYTES: usize = 1500;
/// One session taking longer than this is a failure
const SLOW: Duration = Duration::from_secs(3);

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
fn editing_sessions() {
    let mut failures = Vec::new();
    for seed in 0..SESSIONS {
        let data = bytes(seed, BYTES);
        let start = Instant::now();
        let outcome = catch(|| targets::editing(&data));
        let took = start.elapsed();
        let why = match outcome {
            Err(panic) => Some(panic),
            Ok(()) if took > SLOW => Some(format!("slow: {took:?}")),
            Ok(()) => None,
        };
        if let Some(why) = why {
            failures.push(Failure::new(
                "editing random",
                format!("{why} on seed {seed} (bytes(seed, {BYTES}))"),
            ));
            break;
        }
    }
    let cases = BTreeSet::from(["editing random".to_string()]);
    ratchet(LAYER, &cases, failures);
}
