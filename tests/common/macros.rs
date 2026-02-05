//! Custom test macros
//!
//! Provides assertion macros for testing panic conditions and
//! other custom test behaviors.

/// Macro for asserting that a function panics with a specific message
///
/// # Examples
///
/// ```rust
/// assert_panic!(panic!("test error"), "test error");
/// assert_panic!(panic!("error: 42"), "error:");
/// ```
#[macro_export]
macro_rules! assert_panic {
    ($expr:expr, $msg_pat:pat) => {{
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let result = catch_unwind(AssertUnwindSafe(|| {
            $expr;
        }));
        match result {
            Err(e) => {
                let panic_msg = if let Some(s) = e.downcast_ref::<String>() {
                    s.as_str()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    *s
                } else {
                    ""
                };
                assert!(matches!(panic_msg, $msg_pat),
                    "Expected panic message matching {:?}, got: {:?}",
                    std::stringify!($msg_pat), panic_msg);
            }
            Ok(_) => panic!("Expected panic but expression succeeded"),
        }
    }};
}

/// Macro for asserting that a function does NOT panic
///
/// # Examples
///
/// ```rust
/// assert_no_panic!(std::panic::catch_unwind(|| 1 + 1));
/// ```
#[macro_export]
macro_rules! assert_no_panic {
    ($expr:expr) => {{
        use std::panic::{catch_unwind, AssertUnwindSafe};
        let result = catch_unwind(AssertUnwindSafe(|| {
            $expr;
        }));
        assert!(result.is_ok(), "Expression unexpectedly panicked");
    }};
}
