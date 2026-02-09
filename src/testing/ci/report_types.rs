use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct TestResult {
    /// Test name
    pub name: String,
    /// Whether test passed
    pub passed: bool,
    /// Failure message (if failed)
    pub message: Option<String>,
    /// Diff file path (if failed)
    pub diff_path: Option<PathBuf>,
    /// Duration in milliseconds
    pub duration_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // TestResult tests
    // =========================================================================

    #[test]
    fn test_test_result_passed() {
        let result = TestResult {
            name: "test_example".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 100,
        };

        assert!(result.passed);
        assert_eq!(result.name, "test_example");
        assert!(result.message.is_none());
        assert!(result.diff_path.is_none());
    }

    #[test]
    fn test_test_result_failed() {
        let result = TestResult {
            name: "test_failed".to_string(),
            passed: false,
            message: Some("assertion failed".to_string()),
            diff_path: Some(PathBuf::from("/tmp/diff.txt")),
            duration_ms: 50,
        };

        assert!(!result.passed);
        assert_eq!(result.message, Some("assertion failed".to_string()));
        assert_eq!(result.diff_path, Some(PathBuf::from("/tmp/diff.txt")));
    }

    #[test]
    fn test_test_result_clone() {
        let result = TestResult {
            name: "test_clone".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 10,
        };

        let cloned = result.clone();
        assert_eq!(cloned.name, "test_clone");
        assert_eq!(cloned.passed, result.passed);
        assert_eq!(cloned.duration_ms, 10);
    }

    #[test]
    fn test_test_result_debug() {
        let result = TestResult {
            name: "test_debug".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 5,
        };

        let debug = format!("{:?}", result);
        assert!(debug.contains("TestResult"));
        assert!(debug.contains("test_debug"));
    }

    #[test]
    fn test_test_result_with_message_only() {
        let result = TestResult {
            name: "test_message_only".to_string(),
            passed: false,
            message: Some("error occurred".to_string()),
            diff_path: None,
            duration_ms: 0,
        };

        assert_eq!(result.message, Some("error occurred".to_string()));
        assert!(result.diff_path.is_none());
    }

    #[test]
    fn test_test_result_with_diff_path_only() {
        let result = TestResult {
            name: "test_diff_only".to_string(),
            passed: false,
            message: None,
            diff_path: Some(PathBuf::from("/path/to/diff.png")),
            duration_ms: 0,
        };

        assert!(result.message.is_none());
        assert_eq!(result.diff_path, Some(PathBuf::from("/path/to/diff.png")));
    }

    #[test]
    fn test_test_result_zero_duration() {
        let result = TestResult {
            name: "instant_test".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 0,
        };

        assert_eq!(result.duration_ms, 0);
    }

    #[test]
    fn test_test_result_long_duration() {
        let result = TestResult {
            name: "slow_test".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 60000,
        };

        assert_eq!(result.duration_ms, 60000);
    }

    #[test]
    fn test_test_result_empty_name() {
        let result = TestResult {
            name: String::new(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 1,
        };

        assert!(result.name.is_empty());
    }
}
