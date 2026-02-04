//! Custom assertion helpers
//!
//! Provides additional assertion utilities for testing.

/// Asserts that two optional values are equal, considering None == None
pub fn assert_option_eq<T: std::fmt::Debug + PartialEq>(
    left: Option<T>,
    right: Option<T>,
) {
    match (left, right) {
        (Some(l), Some(r)) => assert_eq!(l, r),
        (None, None) => {}
        (l, r) => panic!("Options not equal: {:?} != {:?}", l, r),
    }
}

/// Asserts that a result is Ok and returns the value
pub fn assert_ok<T: std::fmt::Debug, E: std::fmt::Debug>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => panic!("Expected Ok, got Err: {:?}", e),
    }
}

/// Asserts that a result is Err and returns the error
pub fn assert_err<T: std::fmt::Debug, E: std::fmt::Debug>(result: Result<T, E>) -> E {
    match result {
        Ok(value) => panic!("Expected Err, got Ok: {:?}", value),
        Err(e) => e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_option_eq() {
        assert_option_eq::<i32>(Some(1), Some(1));
        assert_option_eq::<i32>(None, None);
    }

    #[test]
    fn test_assert_ok() {
        let result: Result<i32, &str> = Ok(42);
        assert_eq!(assert_ok(result), 42);
    }

    #[test]
    fn test_assert_err() {
        let result: Result<i32, &str> = Err("error");
        assert_eq!(assert_err(result), "error");
    }
}
