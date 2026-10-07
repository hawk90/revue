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

pub const KNOWN: &[Known] = &[];
