//! Test report implementation

use super::types::CiEnvironment;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

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
        if ci.provider == super::types::CiProvider::GitHubActions {
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
        let status = if self.all_passed() {
            "✅ Passed"
        } else {
            "❌ Failed"
        };
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
