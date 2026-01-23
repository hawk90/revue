//! CI integration helpers for visual regression testing
//!
//! Provides utilities for running visual tests in CI environments
//! like GitHub Actions, GitLab CI, and others.
//!
//! # Features
//!
//! - Automatic CI detection
//! - Artifact generation for failures
//! - GitHub Actions annotations
//! - Summary report generation
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::testing::ci::{CiEnvironment, TestReport};
//!
//! let ci = CiEnvironment::detect();
//! let mut report = TestReport::new();
//!
//! // Run tests...
//! report.add_passed("button_test");
//! report.add_failed("modal_test", "Size mismatch");
//!
//! // Generate summary
//! report.write_summary(&ci);
//! ```

mod env;
mod report;
mod types;

pub use report::{TestReport, TestResult};
pub use types::{CiEnvironment, CiProvider};

// Include tests from tests.rs
#[cfg(test)]
mod tests;
