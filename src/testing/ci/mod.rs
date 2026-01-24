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
mod tests {
    use super::super::{CiEnvironment, CiProvider, TestReport, TestResult};
    use std::path::PathBuf;

    #[test]
    fn test_ci_environment_detect() {
        // CI detection should work without panicking
        let ci = CiEnvironment::detect();
        // Provider name should be non-empty
        assert!(!ci.provider_name().is_empty());
        // Artifacts dir should be set
        assert!(!ci.artifacts_dir.as_os_str().is_empty());
    }

    #[test]
    fn test_ci_provider_name() {
        let ci = CiEnvironment {
            provider: CiProvider::GitHubActions,
            is_ci: true,
            branch: None,
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::from("artifacts"),
        };
        assert_eq!(ci.provider_name(), "GitHub Actions");
    }

    #[test]
    fn test_report_new() {
        let report = TestReport::new();
        assert_eq!(report.total(), 0);
        assert_eq!(report.passed(), 0);
        assert_eq!(report.failed(), 0);
    }

    #[test]
    fn test_report_add_passed() {
        let mut report = TestReport::new();
        report.add_passed("test1");
        report.add_passed("test2");

        assert_eq!(report.total(), 2);
        assert_eq!(report.passed(), 2);
        assert_eq!(report.failed(), 0);
        assert!(report.all_passed());
    }

    #[test]
    fn test_report_add_failed() {
        let mut report = TestReport::new();
        report.add_passed("test1");
        report.add_failed("test2", "Something went wrong");

        assert_eq!(report.total(), 2);
        assert_eq!(report.passed(), 1);
        assert_eq!(report.failed(), 1);
        assert!(!report.all_passed());
    }

    #[test]
    fn test_report_summary() {
        let mut report = TestReport::new();
        report.add_passed("test1");
        report.add_failed("test2", "Error message");

        let summary = report.summary();
        assert!(summary.contains("1 passed"));
        assert!(summary.contains("1 failed"));
        assert!(summary.contains("test2"));
    }

    #[test]
    fn test_report_markdown() {
        let mut report = TestReport::new();
        report.add_passed("test1");
        report.add_metadata("version", "0.5.0");

        let markdown = report.to_markdown();
        assert!(markdown.contains("# Visual Regression Test Results"));
        assert!(markdown.contains("✅ Passed"));
        assert!(markdown.contains("version"));
    }

    // =========================================================================
    // CiProvider tests
    // =========================================================================

    #[test]
    fn test_ci_provider_equality() {
        assert_eq!(CiProvider::GitHubActions, CiProvider::GitHubActions);
        assert_ne!(CiProvider::GitHubActions, CiProvider::GitLabCi);
    }

    #[test]
    fn test_ci_provider_clone() {
        let provider = CiProvider::CircleCi;
        let cloned = provider.clone();
        assert_eq!(provider, cloned);
    }

    #[test]
    fn test_ci_provider_all_variants() {
        // Just verify all variants exist
        let variants = [
            CiProvider::GitHubActions,
            CiProvider::GitLabCi,
            CiProvider::CircleCi,
            CiProvider::TravisCi,
            CiProvider::Jenkins,
            CiProvider::AzurePipelines,
            CiProvider::Generic,
            CiProvider::Local,
        ];
        assert_eq!(variants.len(), 8);
    }

    // =========================================================================
    // CiEnvironment tests
    // =========================================================================

    #[test]
    fn test_ci_environment_default() {
        let ci = CiEnvironment::default();
        assert!(!ci.provider_name().is_empty());
    }

    #[test]
    fn test_ci_environment_is_ci() {
        let ci = CiEnvironment {
            provider: CiProvider::Local,
            is_ci: false,
            branch: None,
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::from("artifacts"),
        };
        assert!(!ci.is_ci());
    }

    #[test]
    fn test_ci_environment_provider_names() {
        let test_cases = [
            (CiProvider::GitHubActions, "GitHub Actions"),
            (CiProvider::GitLabCi, "GitLab CI"),
            (CiProvider::CircleCi, "CircleCI"),
            (CiProvider::TravisCi, "Travis CI"),
            (CiProvider::Jenkins, "Jenkins"),
            (CiProvider::AzurePipelines, "Azure Pipelines"),
            (CiProvider::Generic, "CI"),
            (CiProvider::Local, "Local"),
        ];

        for (provider, expected_name) in test_cases {
            let ci = CiEnvironment {
                provider,
                is_ci: false,
                branch: None,
                commit: None,
                pr_number: None,
                build_number: None,
                artifacts_dir: PathBuf::from("test"),
            };
            assert_eq!(ci.provider_name(), expected_name);
        }
    }

    #[test]
    fn test_ci_environment_clone() {
        let ci = CiEnvironment {
            provider: CiProvider::GitHubActions,
            is_ci: true,
            branch: Some("main".to_string()),
            commit: Some("abc123".to_string()),
            pr_number: Some("42".to_string()),
            build_number: Some("100".to_string()),
            artifacts_dir: PathBuf::from("artifacts"),
        };
        let cloned = ci.clone();
        assert_eq!(cloned.provider, CiProvider::GitHubActions);
        assert_eq!(cloned.branch, Some("main".to_string()));
    }

    // =========================================================================
    // TestResult tests
    // =========================================================================

    #[test]
    fn test_test_result_passed() {
        let result = TestResult {
            name: "my_test".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 100,
        };
        assert!(result.passed);
        assert_eq!(result.name, "my_test");
    }

    #[test]
    fn test_test_result_failed() {
        let result = TestResult {
            name: "failing_test".to_string(),
            passed: false,
            message: Some("Assertion failed".to_string()),
            diff_path: Some(PathBuf::from("test.diff")),
            duration_ms: 50,
        };
        assert!(!result.passed);
        assert!(result.message.is_some());
        assert!(result.diff_path.is_some());
    }

    #[test]
    fn test_test_result_clone() {
        let result = TestResult {
            name: "test".to_string(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 10,
        };
        let cloned = result.clone();
        assert_eq!(cloned.name, "test");
    }

    // =========================================================================
    // TestReport tests
    // =========================================================================

    #[test]
    fn test_report_default() {
        let report = TestReport::default();
        assert!(report.all_passed()); // Empty report has all passed
        assert_eq!(report.total(), 0);
    }

    #[test]
    fn test_report_add_metadata() {
        let mut report = TestReport::new();
        report.add_metadata("key1", "value1");
        report.add_metadata("key2", "value2");

        let markdown = report.to_markdown();
        assert!(markdown.contains("key1"));
        assert!(markdown.contains("value1"));
    }

    #[test]
    fn test_report_add_passed_with_duration() {
        let mut report = TestReport::new();
        report.add_passed_with_duration("fast_test", 10);
        report.add_passed_with_duration("slow_test", 1000);

        assert_eq!(report.passed(), 2);
    }

    #[test]
    fn test_report_add_failed_with_diff() {
        let mut report = TestReport::new();
        report.add_failed_with_diff("visual_test", "Size mismatch", "diff.png");

        assert_eq!(report.failed(), 1);

        let failures: Vec<_> = report.failures().collect();
        assert_eq!(failures.len(), 1);
        assert!(failures[0].diff_path.is_some());
    }

    #[test]
    fn test_report_failures_iterator() {
        let mut report = TestReport::new();
        report.add_passed("pass1");
        report.add_failed("fail1", "error1");
        report.add_passed("pass2");
        report.add_failed("fail2", "error2");

        let failures: Vec<_> = report.failures().collect();
        assert_eq!(failures.len(), 2);
    }

    #[test]
    fn test_report_duration() {
        let report = TestReport::new();
        // Duration should be non-zero after creation (start_time is set)
        std::thread::sleep(std::time::Duration::from_millis(1));
        assert!(report.duration() >= std::time::Duration::from_millis(1));
    }

    #[test]
    fn test_report_summary_all_passed() {
        let mut report = TestReport::new();
        report.add_passed("test1");
        report.add_passed("test2");

        let summary = report.summary();
        assert!(summary.contains("2 passed"));
        assert!(summary.contains("0 failed"));
    }

    #[test]
    fn test_report_summary_with_failures() {
        let mut report = TestReport::new();
        report.add_failed("broken_test", "Something broke");

        let summary = report.summary();
        assert!(summary.contains("Failed tests"));
        assert!(summary.contains("broken_test"));
        assert!(summary.contains("Something broke"));
    }

    #[test]
    fn test_report_markdown_failed() {
        let mut report = TestReport::new();
        report.add_failed("test1", "Error message");

        let markdown = report.to_markdown();
        assert!(markdown.contains("❌ Failed"));
        assert!(markdown.contains("## Failed Tests"));
        assert!(markdown.contains("test1"));
    }

    #[test]
    fn test_report_markdown_with_diff() {
        let mut report = TestReport::new();
        report.add_failed_with_diff("visual_test", "Mismatch", "path/to/diff.png");

        let markdown = report.to_markdown();
        assert!(markdown.contains("Diff:"));
        assert!(markdown.contains("diff.png"));
    }
}
