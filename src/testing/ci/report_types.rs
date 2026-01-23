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
