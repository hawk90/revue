//! The grammars: one tree-sitter highlight configuration per supported
//! [`Language`], built once on first use

use std::collections::HashMap;
use std::sync::OnceLock;

use tree_sitter_highlight::HighlightConfiguration;

use super::captures::HIGHLIGHT_NAMES;
use crate::widget::syntax::Language;

/// Language configurations cache
pub(super) struct LanguageConfigs {
    configs: HashMap<Language, HighlightConfiguration>,
}

impl LanguageConfigs {
    fn new() -> Self {
        let mut configs = Self {
            configs: HashMap::new(),
        };

        // Rust
        configs.insert(
            Language::Rust,
            HighlightConfiguration::new(
                tree_sitter_rust::LANGUAGE.into(),
                "rust",
                tree_sitter_rust::HIGHLIGHTS_QUERY,
                tree_sitter_rust::INJECTIONS_QUERY,
                "",
            ),
        );

        // Python
        configs.insert(
            Language::Python,
            HighlightConfiguration::new(
                tree_sitter_python::LANGUAGE.into(),
                "python",
                tree_sitter_python::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // JavaScript
        configs.insert(
            Language::JavaScript,
            HighlightConfiguration::new(
                tree_sitter_javascript::LANGUAGE.into(),
                "javascript",
                tree_sitter_javascript::HIGHLIGHT_QUERY,
                tree_sitter_javascript::INJECTIONS_QUERY,
                tree_sitter_javascript::LOCALS_QUERY,
            ),
        );

        // JSON
        configs.insert(
            Language::Json,
            HighlightConfiguration::new(
                tree_sitter_json::LANGUAGE.into(),
                "json",
                tree_sitter_json::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // Go
        configs.insert(
            Language::Go,
            HighlightConfiguration::new(
                tree_sitter_go::LANGUAGE.into(),
                "go",
                tree_sitter_go::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // Bash/Shell
        configs.insert(
            Language::Shell,
            HighlightConfiguration::new(
                tree_sitter_bash::LANGUAGE.into(),
                "bash",
                tree_sitter_bash::HIGHLIGHT_QUERY,
                "",
                "",
            ),
        );

        // HTML
        configs.insert(
            Language::Html,
            HighlightConfiguration::new(
                tree_sitter_html::LANGUAGE.into(),
                "html",
                tree_sitter_html::HIGHLIGHTS_QUERY,
                tree_sitter_html::INJECTIONS_QUERY,
                "",
            ),
        );

        // CSS
        configs.insert(
            Language::Css,
            HighlightConfiguration::new(
                tree_sitter_css::LANGUAGE.into(),
                "css",
                tree_sitter_css::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // TOML
        configs.insert(
            Language::Toml,
            HighlightConfiguration::new(
                tree_sitter_toml_ng::LANGUAGE.into(),
                "toml",
                tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // YAML
        configs.insert(
            Language::Yaml,
            HighlightConfiguration::new(
                tree_sitter_yaml::LANGUAGE.into(),
                "yaml",
                tree_sitter_yaml::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // SQL (using tree-sitter-sequel)
        configs.insert(
            Language::Sql,
            HighlightConfiguration::new(
                tree_sitter_sequel::LANGUAGE.into(),
                "sql",
                tree_sitter_sequel::HIGHLIGHTS_QUERY,
                "",
                "",
            ),
        );

        // Markdown
        configs.insert(
            Language::Markdown,
            HighlightConfiguration::new(
                tree_sitter_md::LANGUAGE.into(),
                "markdown",
                tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
                tree_sitter_md::INJECTION_QUERY_BLOCK,
                "",
            ),
        );

        configs
    }

    /// Keep a grammar's configuration if its queries compiled; a grammar whose
    /// queries fail is left out, so the language reads as unsupported
    fn insert<E>(&mut self, language: Language, config: Result<HighlightConfiguration, E>) {
        if let Ok(mut config) = config {
            config.configure(HIGHLIGHT_NAMES);
            self.configs.insert(language, config);
        }
    }

    pub(super) fn get(&self, language: Language) -> Option<&HighlightConfiguration> {
        self.configs.get(&language)
    }
}

/// Get the global language configurations
pub(super) fn get_configs() -> &'static LanguageConfigs {
    static CONFIGS: OnceLock<LanguageConfigs> = OnceLock::new();
    CONFIGS.get_or_init(LanguageConfigs::new)
}
