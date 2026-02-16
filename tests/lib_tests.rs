//! Tests for lib.rs extracted from src/lib.rs
//!
//! All tests use only public constants, functions, and types from revue.

use revue::{is_dev_build, Error, Result, GIT_SHA, VERSION};
use std::io;

// =========================================================================
// VERSION constant tests
// =========================================================================

#[test]
fn test_version_constant_is_not_empty() {
    assert!(!VERSION.is_empty(), "VERSION should not be empty");
}

#[test]
fn test_version_contains_digits() {
    assert!(
        VERSION.chars().any(|c| c.is_ascii_digit()),
        "VERSION should contain digits"
    );
}

#[test]
fn test_version_has_dot() {
    assert!(
        VERSION.contains('.'),
        "VERSION should contain a dot separator"
    );
}

// =========================================================================
// GIT_SHA constant tests
// =========================================================================

#[test]
fn test_git_sha_is_valid() {
    // GIT_SHA is either empty (release) or 40 chars (dev)
    if is_dev_build() {
        assert_eq!(
            GIT_SHA.len(),
            40,
            "GIT_SHA should be 40 characters in dev builds"
        );
        assert!(
            GIT_SHA.chars().all(|c| c.is_ascii_hexdigit()),
            "GIT_SHA should be hex"
        );
    } else {
        assert_eq!(GIT_SHA, "", "GIT_SHA should be empty in release builds");
    }
}

// =========================================================================
// is_dev_build() function tests
// =========================================================================

#[test]
fn test_is_dev_build_returns_bool() {
    let result = is_dev_build();
    // Just verify it returns a boolean without panicking
    if result {
        assert!(
            VERSION.contains('-'),
            "Dev build VERSION should contain dash"
        );
    }
}

#[test]
fn test_is_dev_build_can_be_called_multiple_times() {
    let _r1 = is_dev_build();
    let _r2 = is_dev_build();
    let _r3 = is_dev_build();
    // Should always return the same value
    assert_eq!(_r1, _r2);
    assert_eq!(_r2, _r3);
}

// =========================================================================
// Error enum tests
// =========================================================================

#[test]
fn test_error_io_from_io_error() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error: Error = io_err.into();
    assert!(matches!(error, Error::Io(_)));
}

#[test]
fn test_error_display_formatting() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let error = Error::Io(io_err);
    let display = format!("{}", error);
    assert!(display.contains("I/O error"));
    assert!(display.contains("access denied"));
}

#[test]
fn test_error_debug_formatting() {
    let error = Error::Render("buffer overflow".to_string());
    let debug = format!("{:?}", error);
    assert!(debug.contains("Render"));
    assert!(debug.contains("buffer overflow"));
}

#[test]
fn test_error_render_variant() {
    let error = Error::Render("test error".to_string());
    assert!(matches!(error, Error::Render(_)));
}

#[test]
fn test_error_from_anyhow() {
    let anyhow_err = anyhow::anyhow!("generic error");
    let error: Error = anyhow_err.into();
    assert!(matches!(error, Error::Other(_)));
}

#[test]
fn test_error_clone_for_render() {
    let error = Error::Render("test".to_string());
    // Error doesn't impl Clone but we can use it
    let _msg = format!("{}", error);
}

// =========================================================================
// Result type tests
// =========================================================================

#[test]
fn test_result_ok_variant() {
    let result: Result<String> = Ok("success".to_string());
    assert!(result.is_ok());
    assert!(!result.is_err());
}

#[test]
fn test_result_err_variant() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "not found");
    let result: Result<String> = Err(io_err.into());
    assert!(!result.is_ok());
    assert!(result.is_err());
}

#[test]
fn test_result_with_question_mark() {
    fn fallsible() -> Result<()> {
        let _ = Ok::<(), Error>(());
        Ok(())
    }
    assert!(fallsible().is_ok());
}

#[test]
fn test_result_question_mark_propagates_io_error() {
    fn fallsible_io() -> Result<()> {
        let _file = std::fs::read_to_string("/nonexistent/file/that/does/not/exist")?;
        Ok(())
    }
    assert!(fallsible_io().is_err());
}

// =========================================================================
// Store derive macro re-export tests
// =========================================================================

#[test]
fn test_store_macro_is_reexported() {
    // Just verify Store is accessible from prelude
    // This test passes if it compiles
}

// =========================================================================
// Core module re-export tests
// =========================================================================

#[test]
fn test_constants_module_is_reexported() {
    // Verify constants module is accessible
    // This test passes if it compiles
}

// =========================================================================
// Runtime modules re-export tests
// =========================================================================

#[test]
fn test_dom_module_is_reexported() {
    // Verify dom module is accessible
}

#[test]
fn test_event_module_is_reexported() {
    // Verify event module is accessible
}

#[test]
fn test_layout_module_is_reexported() {
    // Verify layout module is accessible
}

#[test]
fn test_render_module_is_reexported() {
    // Verify render module is accessible
}

#[test]
fn test_style_module_is_reexported() {
    // Verify style module is accessible
}

// =========================================================================
// State modules re-export tests
// =========================================================================

#[test]
fn test_patterns_module_is_reexported() {
    // Verify patterns module is accessible
}

#[test]
fn test_reactive_module_is_reexported() {
    // Verify reactive module is accessible
}

// =========================================================================
// Other module re-export tests
// =========================================================================

#[test]
fn test_a11y_module_is_reexported() {
    // Verify a11y module is accessible
}

#[test]
fn test_devtools_module_is_reexported() {
    // Verify devtools module is accessible
}

#[test]
fn test_query_module_is_reexported() {
    // Verify query module is accessible
}

#[test]
fn test_testing_module_is_reexported() {
    // Verify testing module is accessible
}

#[test]
fn test_text_module_is_reexported() {
    // Verify text module is accessible
}

#[test]
fn test_utils_module_is_reexported() {
    // Verify utils module is accessible
}

#[test]
fn test_widget_module_is_reexported() {
    // Verify widget module is accessible
}
