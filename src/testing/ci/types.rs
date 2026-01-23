//! CI environment detection types

use std::path::PathBuf;

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
