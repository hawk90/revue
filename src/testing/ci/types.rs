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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // CiProvider tests
    // =========================================================================

    #[test]
    fn test_ci_provider_variants() {
        let _ = CiProvider::GitHubActions;
        let _ = CiProvider::GitLabCi;
        let _ = CiProvider::CircleCi;
        let _ = CiProvider::TravisCi;
        let _ = CiProvider::Jenkins;
        let _ = CiProvider::AzurePipelines;
        let _ = CiProvider::Generic;
        let _ = CiProvider::Local;
    }

    #[test]
    fn test_ci_provider_clone() {
        let provider = CiProvider::GitHubActions;
        let cloned = provider.clone();
        assert_eq!(cloned, CiProvider::GitHubActions);
    }

    #[test]
    fn test_ci_provider_equality() {
        assert_eq!(CiProvider::GitHubActions, CiProvider::GitHubActions);
        assert_ne!(CiProvider::GitHubActions, CiProvider::GitLabCi);
    }

    #[test]
    fn test_ci_provider_debug() {
        let provider = CiProvider::Local;
        let debug = format!("{:?}", provider);
        assert!(debug.contains("Local"));
    }

    // =========================================================================
    // CiEnvironment tests
    // =========================================================================

    #[test]
    fn test_ci_environment_new() {
        let env = CiEnvironment {
            provider: CiProvider::Local,
            is_ci: false,
            branch: None,
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::from("/tmp/artifacts"),
        };

        assert!(!env.is_ci);
        assert_eq!(env.provider, CiProvider::Local);
    }

    #[test]
    fn test_ci_environment_with_branch() {
        let env = CiEnvironment {
            provider: CiProvider::GitHubActions,
            is_ci: true,
            branch: Some("main".to_string()),
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::new(),
        };

        assert_eq!(env.branch, Some("main".to_string()));
    }

    #[test]
    fn test_ci_environment_with_commit() {
        let env = CiEnvironment {
            provider: CiProvider::GitLabCi,
            is_ci: true,
            branch: None,
            commit: Some("abc123".to_string()),
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::new(),
        };

        assert_eq!(env.commit, Some("abc123".to_string()));
    }

    #[test]
    fn test_ci_environment_with_pr_number() {
        let env = CiEnvironment {
            provider: CiProvider::CircleCi,
            is_ci: true,
            branch: None,
            commit: None,
            pr_number: Some("42".to_string()),
            build_number: None,
            artifacts_dir: PathBuf::new(),
        };

        assert_eq!(env.pr_number, Some("42".to_string()));
    }

    #[test]
    fn test_ci_environment_with_build_number() {
        let env = CiEnvironment {
            provider: CiProvider::TravisCi,
            is_ci: true,
            branch: None,
            commit: None,
            pr_number: None,
            build_number: Some("100".to_string()),
            artifacts_dir: PathBuf::new(),
        };

        assert_eq!(env.build_number, Some("100".to_string()));
    }

    #[test]
    fn test_ci_environment_with_artifacts_dir() {
        let env = CiEnvironment {
            provider: CiProvider::Local,
            is_ci: false,
            branch: None,
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::from("/custom/path"),
        };

        assert_eq!(env.artifacts_dir, PathBuf::from("/custom/path"));
    }

    #[test]
    fn test_ci_environment_clone() {
        let env = CiEnvironment {
            provider: CiProvider::AzurePipelines,
            is_ci: true,
            branch: Some("main".to_string()),
            commit: Some("def456".to_string()),
            pr_number: Some("10".to_string()),
            build_number: Some("200".to_string()),
            artifacts_dir: PathBuf::from("/artifacts"),
        };

        let cloned = env.clone();
        assert_eq!(cloned.provider, CiProvider::AzurePipelines);
        assert_eq!(cloned.branch, Some("main".to_string()));
    }

    #[test]
    fn test_ci_environment_debug() {
        let env = CiEnvironment {
            provider: CiProvider::Jenkins,
            is_ci: true,
            branch: None,
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::new(),
        };

        let debug = format!("{:?}", env);
        assert!(debug.contains("CiEnvironment"));
        assert!(debug.contains("Jenkins"));
    }

    #[test]
    fn test_ci_environment_all_providers() {
        let providers = vec![
            CiProvider::GitHubActions,
            CiProvider::GitLabCi,
            CiProvider::CircleCi,
            CiProvider::TravisCi,
            CiProvider::Jenkins,
            CiProvider::AzurePipelines,
            CiProvider::Generic,
            CiProvider::Local,
        ];

        for provider in providers {
            let env = CiEnvironment {
                provider: provider.clone(),
                is_ci: matches!(
                    provider,
                    CiProvider::Generic | CiProvider::GitHubActions | CiProvider::GitLabCi
                ),
                branch: None,
                commit: None,
                pr_number: None,
                build_number: None,
                artifacts_dir: PathBuf::new(),
            };
            assert_eq!(env.provider, provider);
        }
    }

    #[test]
    fn test_ci_environment_generic() {
        let env = CiEnvironment {
            provider: CiProvider::Generic,
            is_ci: true,
            branch: Some("develop".to_string()),
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::new(),
        };

        assert_eq!(env.provider, CiProvider::Generic);
        assert!(env.is_ci);
    }

    #[test]
    fn test_ci_environment_local_not_ci() {
        let env = CiEnvironment {
            provider: CiProvider::Local,
            is_ci: false,
            branch: Some("feature-branch".to_string()),
            commit: None,
            pr_number: None,
            build_number: None,
            artifacts_dir: PathBuf::new(),
        };

        assert_eq!(env.provider, CiProvider::Local);
        assert!(!env.is_ci);
    }
}
