//! Parser fuzzing: every text entry point fed three kinds of input.
//!
//! - `unicode` - arbitrary Unicode strings (proptest, up to 256 chars);
//! - `soup` - "token soup": random sequences of the format's own tokens
//!   (CSS-ish tokens, markdown markers, `[`/`]` markup, escape sequences, ...),
//!   which reach much deeper into a parser than random characters do;
//! - `corpus` - a fixed list of nasty inputs: empty, huge, 10 000 nested
//!   brackets, unterminated constructs, NUL, lossy-decoded random bytes, very
//!   long lines, CRLF, combining-only, RTL+LTR mixes.
//!
//! Invariants: nothing panics (a parser that returns `Result` returns `Err`),
//! and nothing hangs - every input is bounded, a single input taking more than
//! [`SLOW`] is a failure, and a target that does not finish within [`HANG`]
//! is reported and abandoned.
//!
//! The proptest runs are deterministic: a fixed ChaCha seed and an explicit
//! config, so neither `PROPTEST_*` variables nor a regressions file change
//! what runs.
//!
//! Case keys: `"<target> <unicode|soup|corpus>"`.

use std::collections::BTreeSet;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestCaseError, TestRng, TestRunner};

use super::{catch, preview, ratchet, Failure};

mod targets;

use targets::corpus;

/// A target of this layer, from [`targets::targets`]
struct Target {
    name: &'static str,
    run: fn(&str),
    tokens: &'static [&'static str],
}

fn targets() -> Vec<Target> {
    targets::targets()
        .into_iter()
        .map(|(name, run, tokens)| Target { name, run, tokens })
        .collect()
}

const LAYER: &str = "parsers";

/// The case kinds of every target: its three inputs, and whether it finished.
/// With the target name, the case key.
const KINDS: [&str; 4] = ["unicode", "soup", "corpus", "hang"];

/// Cases per proptest strategy and target.
const CASES: u32 = 128;
/// One input taking longer than this is a failure.
const SLOW: Duration = Duration::from_secs(3);
/// A target that has not finished after this is reported as hung.
const HANG: Duration = Duration::from_secs(60);

/// Run `f` on `input`; `Err` on a panic or an input that took too long.
fn guarded(f: fn(&str), input: &str) -> Result<(), String> {
    let start = Instant::now();
    catch(|| f(input))?;
    let took = start.elapsed();
    if took > SLOW {
        return Err(format!("slow: {took:?} on one input"));
    }
    Ok(())
}

fn runner() -> TestRunner {
    let config = Config {
        cases: CASES,
        failure_persistence: None,
        max_shrink_iters: 512,
        rng_algorithm: RngAlgorithm::ChaCha,
        ..Config::default()
    };
    TestRunner::new_with_rng(config, TestRng::deterministic_rng(RngAlgorithm::ChaCha))
}

/// Run a proptest strategy against a target; `Some(detail)` on failure, with
/// the shrunk input.
fn fuzz(f: fn(&str), strategy: impl Strategy<Value = String>) -> Option<String> {
    let mut runner = runner();
    match runner.run(&strategy, |s| guarded(f, &s).map_err(TestCaseError::fail)) {
        Ok(()) => None,
        Err(proptest::test_runner::TestError::Fail(why, input)) => {
            Some(format!("{why} on {}", preview(&input)))
        }
        Err(e) => Some(e.to_string()),
    }
}

/// Every case of one target: `(case key, failure detail)`.
fn run_target(target: &Target, corpus: &[(&'static str, String)]) -> Vec<Failure> {
    let mut failures = Vec::new();
    let mut push = |kind: &str, detail: Option<String>| {
        if let Some(detail) = detail {
            failures.push(Failure::new(format!("{} {kind}", target.name), detail));
        }
    };

    push("unicode", fuzz(target.run, "(?s).{0,256}"));
    let soup = proptest::collection::vec(proptest::sample::select(target.tokens), 0..64)
        .prop_map(|tokens| tokens.concat());
    push("soup", fuzz(target.run, soup));

    let corpus_failure = corpus.iter().find_map(|(name, input)| {
        guarded(target.run, input)
            .err()
            .map(|why| format!("{why} on corpus entry {name:?}"))
    });
    push("corpus", corpus_failure);

    failures
}

#[test]
fn parser_fuzzing() {
    let corpus = corpus();
    let mut cases = BTreeSet::new();
    let mut failures = Vec::new();

    // Each target runs on its own thread, so one that hangs is reported and
    // abandoned instead of stalling the layer.
    let (tx, rx) = mpsc::channel();
    let names: Vec<&'static str> = targets().iter().map(|t| t.name).collect();
    for (i, target) in targets().into_iter().enumerate() {
        let tx = tx.clone();
        let corpus = corpus.clone();
        std::thread::Builder::new()
            .name(format!("fuzz-{}", target.name))
            .stack_size(8 << 20)
            .spawn(move || {
                let _ = tx.send((i, run_target(&target, &corpus)));
            })
            .expect("spawn fuzz thread");
    }
    drop(tx);

    let deadline = Instant::now() + HANG;
    let mut done = vec![false; names.len()];
    while done.iter().any(|d| !d) {
        match rx.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            Ok((i, mut f)) => {
                done[i] = true;
                failures.append(&mut f);
            }
            Err(_) => break,
        }
    }
    for (i, name) in names.iter().enumerate() {
        for kind in KINDS {
            cases.insert(format!("{name} {kind}"));
        }
        if !done[i] {
            failures.push(Failure::new(
                format!("{name} hang"),
                format!("did not finish within {HANG:?}"),
            ));
        }
    }
    failures.sort_by(|a, b| a.case.cmp(&b.case));
    ratchet(LAYER, &cases, failures);
}
