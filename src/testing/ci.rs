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

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// =============================================================================
// CI Environment Detection
// =============================================================================

/// Detected CI environment
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CiProvider {
    /// GitHub Actions
    GitHubActions,
    /// GitLab CI
    GitLabCi,
    /// CircleCI
    CircleCi,
    /// Travis CI
    TravisCi,
    /// Jenkins
    Jenkins,
    /// Azure Pipelines
    AzurePipelines,
    /// Generic CI (detected but unknown provider)
    Generic,
    /// Local development (not CI)
    Local,
}

/// CI environment information
#[derive(Debug, Clone)]
pub struct CiEnvironment {
    /// CI provider
    pub provider: CiProvider,
    /// Whether running in CI
    pub is_ci: bool,
    /// Branch name (if available)
    pub branch: Option<String>,
    /// Commit SHA (if available)
    pub commit: Option<String>,
    /// Pull request number (if available)
    pub pr_number: Option<String>,
    /// Build number (if available)
    pub build_number: Option<String>,
    /// Artifacts directory
    pub artifacts_dir: PathBuf,
}

impl CiEnvironment {
    /// Detect current CI environment
    pub fn detect() -> Self {
        let is_ci = std::env::var("CI").is_ok()
            || std::env::var("CONTINUOUS_INTEGRATION").is_ok();

        let provider = if std::env::var("GITHUB_ACTIONS").is_ok() {
            CiProvider::GitHubActions
        } else if std::env::var("GITLAB_CI").is_ok() {
            CiProvider::GitLabCi
        } else if std::env::var("CIRCLECI").is_ok() {
            CiProvider::CircleCi
        } else if std::env::var("TRAVIS").is_ok() {
            CiProvider::TravisCi
        } else if std::env::var("JENKINS_URL").is_ok() {
            CiProvider::Jenkins
        } else if std::env::var("TF_BUILD").is_ok() {
            CiProvider::AzurePipelines
        } else if is_ci {
            CiProvider::Generic
        } else {
            CiProvider::Local
        };

        let branch = Self::detect_branch(&provider);
        let commit = Self::detect_commit(&provider);
        let pr_number = Self::detect_pr_number(&provider);
        let build_number = Self::detect_build_number(&provider);
        let artifacts_dir = Self::detect_artifacts_dir(&provider);

        Self {
            provider,
            is_ci,
            branch,
            commit,
            pr_number,
            build_number,
            artifacts_dir,
        }
    }

    fn detect_branch(provider: &CiProvider) -> Option<String> {
        match provider {
            CiProvider::GitHubActions => {
                std::env::var("GITHUB_HEAD_REF")
                    .or_else(|_| std::env::var("GITHUB_REF_NAME"))
                    .ok()
            }
            CiProvider::GitLabCi => std::env::var("CI_COMMIT_REF_NAME").ok(),
            CiProvider::CircleCi => std::env::var("CIRCLE_BRANCH").ok(),
            CiProvider::TravisCi => std::env::var("TRAVIS_BRANCH").ok(),
            _ => std::env::var("BRANCH_NAME")
                .or_else(|_| std::env::var("GIT_BRANCH"))
                .ok(),
        }
    }

    fn detect_commit(provider: &CiProvider) -> Option<String> {
        match provider {
            CiProvider::GitHubActions => std::env::var("GITHUB_SHA").ok(),
            CiProvider::GitLabCi => std::env::var("CI_COMMIT_SHA").ok(),
            CiProvider::CircleCi => std::env::var("CIRCLE_SHA1").ok(),
            CiProvider::TravisCi => std::env::var("TRAVIS_COMMIT").ok(),
            _ => std::env::var("GIT_COMMIT")
                .or_else(|_| std::env::var("COMMIT_SHA"))
                .ok(),
        }
    }

    fn detect_pr_number(provider: &CiProvider) -> Option<String> {
        match provider {
            CiProvider::GitHubActions => {
                std::env::var("GITHUB_EVENT_NAME")
                    .ok()
                    .filter(|e| e == "pull_request")
                    .and_then(|_| {
                        std::env::var("GITHUB_REF")
                            .ok()
                            .and_then(|r| r.split('/').nth(2).map(String::from))
                    })
            }
            CiProvider::GitLabCi => std::env::var("CI_MERGE_REQUEST_IID").ok(),
            CiProvider::CircleCi => std::env::var("CIRCLE_PULL_REQUEST")
                .ok()
                .and_then(|url| url.split('/').last().map(String::from)),
            CiProvider::TravisCi => std::env::var("TRAVIS_PULL_REQUEST")
                .ok()
                .filter(|pr| pr != "false"),
            _ => None,
        }
    }

    fn detect_build_number(provider: &CiProvider) -> Option<String> {
        match provider {
            CiProvider::GitHubActions => std::env::var("GITHUB_RUN_NUMBER").ok(),
            CiProvider::GitLabCi => std::env::var("CI_PIPELINE_ID").ok(),
            CiProvider::CircleCi => std::env::var("CIRCLE_BUILD_NUM").ok(),
            CiProvider::TravisCi => std::env::var("TRAVIS_BUILD_NUMBER").ok(),
            CiProvider::Jenkins => std::env::var("BUILD_NUMBER").ok(),
            _ => None,
        }
    }

    fn detect_artifacts_dir(provider: &CiProvider) -> PathBuf {
        match provider {
            CiProvider::GitHubActions => {
                std::env::var("GITHUB_WORKSPACE")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join("test-artifacts")
            }
            CiProvider::GitLabCi => PathBuf::from("test-artifacts"),
            CiProvider::CircleCi => {
                std::env::var("CIRCLE_ARTIFACTS")
                    .map(PathBuf::from)
                    .unwrap_or_else(|_| PathBuf::from("test-artifacts"))
            }
            _ => PathBuf::from("test-artifacts"),
        }
    }

    /// Check if running in CI
    pub fn is_ci(&self) -> bool {
        self.is_ci
    }

    /// Get provider name
    pub fn provider_name(&self) -> &str {
        match self.provider {
            CiProvider::GitHubActions => "GitHub Actions",
            CiProvider::GitLabCi => "GitLab CI",
            CiProvider::CircleCi => "CircleCI",
            CiProvider::TravisCi => "Travis CI",
            CiProvider::Jenkins => "Jenkins",
            CiProvider::AzurePipelines => "Azure Pipelines",
            CiProvider::Generic => "CI",
            CiProvider::Local => "Local",
        }
    }

    /// Emit GitHub Actions annotation for error
    pub fn annotate_error(&self, file: &str, line: u32, message: &str) {
        if self.provider == CiProvider::GitHubActions {
            println!("::error file={},line={}::{}", file, line, message);
        }
    }

    /// Emit GitHub Actions annotation for warning
    pub fn annotate_warning(&self, file: &str, line: u32, message: &str) {
        if self.provider == CiProvider::GitHubActions {
            println!("::warning file={},line={}::{}", file, line, message);
        }
    }

    /// Start a collapsible group in CI output
    pub fn start_group(&self, name: &str) {
        match self.provider {
            CiProvider::GitHubActions => println!("::group::{}", name),
            CiProvider::GitLabCi => println!("\x1b[0Ksection_start:{}:{}\r\x1b[0K{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                name.replace(' ', "_"),
                name
            ),
            _ => println!("=== {} ===", name),
        }
    }

    /// End a collapsible group
    pub fn end_group(&self, name: &str) {
        match self.provider {
            CiProvider::GitHubActions => println!("::endgroup::"),
            CiProvider::GitLabCi => println!("\x1b[0Ksection_end:{}:{}\r\x1b[0K",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                name.replace(' ', "_")
            ),
            _ => {}
        }
    }

    /// Set output variable (GitHub Actions)
    pub fn set_output(&self, name: &str, value: &str) {
        if self.provider == CiProvider::GitHubActions {
            if let Ok(output_file) = std::env::var("GITHUB_OUTPUT") {
                let _ = fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output_file)
                    .and_then(|mut f| {
                        use std::io::Write;
                        writeln!(f, "{}={}", name, value)
                    });
            }
        }
    }
}

impl Default for CiEnvironment {
    fn default() -> Self {
        Self::detect()
    }
}

// =============================================================================
// Test Report
// =============================================================================

/// Visual test result
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

/// Test report for CI
#[derive(Debug, Default)]
pub struct TestReport {
    /// Test results
    results: Vec<TestResult>,
    /// Start time
    start_time: Option<std::time::Instant>,
    /// Metadata
    metadata: HashMap<String, String>,
}

impl TestReport {
    /// Create new report
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
            start_time: Some(std::time::Instant::now()),
            metadata: HashMap::new(),
        }
    }

    /// Add metadata
    pub fn add_metadata(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.metadata.insert(key.into(), value.into());
    }

    /// Add passed test
    pub fn add_passed(&mut self, name: impl Into<String>) {
        self.results.push(TestResult {
            name: name.into(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms: 0,
        });
    }

    /// Add passed test with duration
    pub fn add_passed_with_duration(&mut self, name: impl Into<String>, duration_ms: u64) {
        self.results.push(TestResult {
            name: name.into(),
            passed: true,
            message: None,
            diff_path: None,
            duration_ms,
        });
    }

    /// Add failed test
    pub fn add_failed(&mut self, name: impl Into<String>, message: impl Into<String>) {
        self.results.push(TestResult {
            name: name.into(),
            passed: false,
            message: Some(message.into()),
            diff_path: None,
            duration_ms: 0,
        });
    }

    /// Add failed test with diff
    pub fn add_failed_with_diff(
        &mut self,
        name: impl Into<String>,
        message: impl Into<String>,
        diff_path: impl Into<PathBuf>,
    ) {
        self.results.push(TestResult {
            name: name.into(),
            passed: false,
            message: Some(message.into()),
            diff_path: Some(diff_path.into()),
            duration_ms: 0,
        });
    }

    /// Get total test count
    pub fn total(&self) -> usize {
        self.results.len()
    }

    /// Get passed test count
    pub fn passed(&self) -> usize {
        self.results.iter().filter(|r| r.passed).count()
    }

    /// Get failed test count
    pub fn failed(&self) -> usize {
        self.results.iter().filter(|r| !r.passed).count()
    }

    /// Check if all tests passed
    pub fn all_passed(&self) -> bool {
        self.results.iter().all(|r| r.passed)
    }

    /// Get failed tests
    pub fn failures(&self) -> impl Iterator<Item = &TestResult> {
        self.results.iter().filter(|r| !r.passed)
    }

    /// Get total duration
    pub fn duration(&self) -> std::time::Duration {
        self.start_time
            .map(|s| s.elapsed())
            .unwrap_or(std::time::Duration::ZERO)
    }

    /// Generate summary string
    pub fn summary(&self) -> String {
        let mut output = String::new();

        output.push_str(&format!(
            "Visual Tests: {} passed, {} failed, {} total\n",
            self.passed(),
            self.failed(),
            self.total()
        ));

        if !self.all_passed() {
            output.push_str("\nFailed tests:\n");
            for result in self.failures() {
                output.push_str(&format!("  - {}", result.name));
                if let Some(ref msg) = result.message {
                    output.push_str(&format!(": {}", msg));
                }
                output.push('\n');
            }
        }

        output.push_str(&format!("\nDuration: {:?}\n", self.duration()));

        output
    }

    /// Write summary to CI
    pub fn write_summary(&self, ci: &CiEnvironment) {
        // Print summary
        println!("\n{}", self.summary());

        // GitHub Actions specific
        if ci.provider == CiProvider::GitHubActions {
            // Write to step summary if available
            if let Ok(summary_file) = std::env::var("GITHUB_STEP_SUMMARY") {
                let markdown = self.to_markdown();
                let _ = fs::write(summary_file, markdown);
            }

            // Set outputs
            ci.set_output("passed", &self.passed().to_string());
            ci.set_output("failed", &self.failed().to_string());
            ci.set_output("total", &self.total().to_string());

            // Annotate failures
            for result in self.failures() {
                if let Some(ref msg) = result.message {
                    println!("::error title=Visual Test Failed: {}::{}", result.name, msg);
                }
            }
        }
    }

    /// Generate markdown report
    pub fn to_markdown(&self) -> String {
        let mut output = String::new();

        // Header
        output.push_str("# Visual Regression Test Results\n\n");

        // Summary badge
        let status = if self.all_passed() { "✅ Passed" } else { "❌ Failed" };
        output.push_str(&format!("**Status:** {}\n\n", status));

        // Stats table
        output.push_str("| Metric | Value |\n");
        output.push_str("|--------|-------|\n");
        output.push_str(&format!("| Total | {} |\n", self.total()));
        output.push_str(&format!("| Passed | {} |\n", self.passed()));
        output.push_str(&format!("| Failed | {} |\n", self.failed()));
        output.push_str(&format!("| Duration | {:?} |\n", self.duration()));
        output.push('\n');

        // Failures
        if !self.all_passed() {
            output.push_str("## Failed Tests\n\n");
            for result in self.failures() {
                output.push_str(&format!("### {}\n\n", result.name));
                if let Some(ref msg) = result.message {
                    output.push_str(&format!("**Error:** {}\n\n", msg));
                }
                if let Some(ref diff) = result.diff_path {
                    output.push_str(&format!("**Diff:** `{}`\n\n", diff.display()));
                }
            }
        }

        // Metadata
        if !self.metadata.is_empty() {
            output.push_str("## Metadata\n\n");
            for (key, value) in &self.metadata {
                output.push_str(&format!("- **{}:** {}\n", key, value));
            }
        }

        output
    }

    /// Save report to file
    pub fn save(&self, path: &Path) -> std::io::Result<()> {
        let content = self.to_markdown();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, content)
    }

    /// Save artifacts for failed tests
    pub fn save_artifacts(&self, ci: &CiEnvironment) -> std::io::Result<()> {
        if self.all_passed() {
            return Ok(());
        }

        // Create artifacts directory
        fs::create_dir_all(&ci.artifacts_dir)?;

        // Save report
        let report_path = ci.artifacts_dir.join("visual-test-report.md");
        self.save(&report_path)?;

        // Copy diff files
        for result in self.failures() {
            if let Some(ref diff_path) = result.diff_path {
                if diff_path.exists() {
                    let dest = ci.artifacts_dir.join(diff_path.file_name().unwrap());
                    fs::copy(diff_path, dest)?;
                }
            }
        }

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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
}
