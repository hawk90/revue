//! Every catalog widget through the whole pipeline, read back by a terminal
//! emulator: the first frame (a full redraw) and a second with other
//! content and focus (a diff) both show what the buffer holds (#1164)

use revue::testing::PipelineHarness;

use super::matrix::catalog::catalog;
use super::matrix::Content;
use crate::emulator::{difference, emulate};

/// Widgets whose screen does not read back yet, each with its issue. The
/// list may only shrink: a listed widget that reads back fails the test.
const KNOWN: &[(&str, &str)] = &[
    // Its tree labels are drawn one cell per char (#1285)
    ("DevToolsInspector", "#1285"),
];

#[test]
fn every_widget_reads_back_through_a_terminal_emulator() {
    let (w, h) = (40, 12);
    let mut failures = Vec::new();
    let mut fixed = Vec::new();
    for entry in catalog() {
        let known = KNOWN.iter().find(|(name, _)| *name == entry.name);
        let mut failed = false;
        let mut harness = PipelineHarness::new(w, h);
        let frames = [
            (Content::ordinary(), false),
            (Content::text("wide", "한글 😀 é and more text"), true),
        ];
        for (frame, (content, focused)) in frames.into_iter().enumerate() {
            let view = (entry.build)(&content, focused);
            harness.draw(&view);
            let parser = emulate(harness.terminal_output(), w, h);
            if let Some(diff) = difference(harness.buffer(), parser.screen()) {
                failed = true;
                if known.is_none() {
                    failures.push(format!("{} frame {frame}: {diff}", entry.name));
                }
                break;
            }
        }
        if let (Some((name, issue)), false) = (known, failed) {
            fixed.push(format!(
                "{name} ({issue}) reads back now: remove it from KNOWN"
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
    assert!(fixed.is_empty(), "{}", fixed.join("\n"));
}
