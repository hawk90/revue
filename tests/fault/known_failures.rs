//! Known failures of the fault-injection layers - the ratchet.
//!
//! A layer fails on any failure not listed here, and on any listed failure
//! that no longer happens: fix the library, then delete the entry. The list
//! only shrinks. Case keys are layer-specific; each layer documents its own.

/// A group of known failures of one layer sharing one root cause.
pub struct Known {
    pub layer: &'static str,
    /// One-line root cause.
    pub reason: &'static str,
    pub cases: &'static [&'static str],
}

pub const KNOWN: &[Known] = &[
    // The remaining entry needs a design decision, not a fix; see
    // docs/refactor/findings-fault-injection.md, "남긴 것".
    Known {
        layer: "concurrency",
        reason: "design decision: TaskRunner and WorkerPool join running tasks on drop, so a \
                 task that never finishes blocks the drop for good - keep the join, or detach \
                 like PooledTaskRunner and WorkerHandle?",
        cases: &["runner drop-running", "pool drop-running"],
    },
];
