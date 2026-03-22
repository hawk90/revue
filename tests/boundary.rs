//! Boundary condition tests
//!
//! Tests for edge cases and boundary values including:
//! - Numeric limits (zero, max, overflow)
//! - Text boundaries (empty, unicode, long strings)
//! - Collection boundaries (empty, single item, index limits)

#[path = "boundary/collection_tests.rs"]
mod collection_tests;
#[path = "boundary/numeric_tests.rs"]
mod numeric_tests;
#[path = "boundary/text_tests.rs"]
mod text_tests;
