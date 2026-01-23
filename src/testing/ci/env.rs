//! CI environment detection implementation

use super::types::{CiEnvironment, CiProvider};
use std::fs;
use std::path::PathBuf;

impl CiEnvironment {
    /// Detect current CI environment
    pub fn detect() -> Self {
        let is_ci = std::env::var("CI").is_ok() || std::env::var("CONTINUOUS_INTEGRATION").is_ok();

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
            CiProvider::GitHubActions => std::env::var("GITHUB_HEAD_REF")
                .or_else(|_| std::env::var("GITHUB_REF_NAME"))
                .ok(),
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
            CiProvider::GitHubActions => std::env::var("GITHUB_EVENT_NAME")
                .ok()
                .filter(|e| e == "pull_request")
                .and_then(|_| {
                    std::env::var("GITHUB_REF")
                        .ok()
                        .and_then(|r| r.split('/').nth(2).map(String::from))
                }),
            CiProvider::GitLabCi => std::env::var("CI_MERGE_REQUEST_IID").ok(),
            CiProvider::CircleCi => std::env::var("CIRCLE_PULL_REQUEST")
                .ok()
                .and_then(|url| url.split('/').next_back().map(String::from)),
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
            CiProvider::GitHubActions => std::env::var("GITHUB_WORKSPACE")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("test-artifacts"),
            CiProvider::GitLabCi => PathBuf::from("test-artifacts"),
            CiProvider::CircleCi => std::env::var("CIRCLE_ARTIFACTS")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("test-artifacts")),
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
            CiProvider::GitLabCi => println!(
                "\x1b[0Ksection_start:{}:{}\r\x1b[0K{}",
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
            CiProvider::GitLabCi => println!(
                "\x1b[0Ksection_end:{}:{}\r\x1b[0K",
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
