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
    // ── resources ──
    // ── concurrency ──
    Known {
        layer: "concurrency",
        reason: "TaskRunner and WorkerPool join running tasks on drop (design: wait or detach?)",
        cases: &["runner drop-running", "pool drop-running"],
    },
    Known {
        layer: "concurrency",
        reason: "a panicking plugin hook unwinds out of the registry (design: crash or isolate?)",
        cases: &[
            "plugin init-panic",
            "plugin mount-panic",
            "plugin tick-panic",
            "plugin unmount-panic",
        ],
    },
];
