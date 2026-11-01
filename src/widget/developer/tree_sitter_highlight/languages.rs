//! The grammars: one tree-sitter highlight configuration per supported
//! [`Language`], each built the first time that language is used

use std::sync::OnceLock;

use tree_sitter_highlight::HighlightConfiguration;

use super::captures::HIGHLIGHT_NAMES;
use crate::widget::syntax::Language;

/// One slot for each variant of [`Language`], `Go` being the last.
const SLOTS: usize = Language::Go as usize + 1;

/// A language's configuration, built once on first use. `None` for a language
/// with no grammar, or one whose queries fail to compile: it reads as
/// unsupported.
pub(super) fn config(language: Language) -> Option<&'static HighlightConfiguration> {
    static CONFIGS: [OnceLock<Option<HighlightConfiguration>>; SLOTS] =
        [const { OnceLock::new() }; SLOTS];
    CONFIGS
        .get(language as usize)?
        .get_or_init(|| {
            let mut config = build(language)?;
            config.configure(HIGHLIGHT_NAMES);
            Some(config)
        })
        .as_ref()
}

fn build(language: Language) -> Option<HighlightConfiguration> {
    match language {
        Language::Rust => HighlightConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            "rust",
            tree_sitter_rust::HIGHLIGHTS_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            "",
        ),
        Language::Python => HighlightConfiguration::new(
            tree_sitter_python::LANGUAGE.into(),
            "python",
            tree_sitter_python::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::JavaScript => HighlightConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            "javascript",
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTIONS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
        ),
        Language::Json => HighlightConfiguration::new(
            tree_sitter_json::LANGUAGE.into(),
            "json",
            tree_sitter_json::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::Go => HighlightConfiguration::new(
            tree_sitter_go::LANGUAGE.into(),
            "go",
            tree_sitter_go::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::Shell => HighlightConfiguration::new(
            tree_sitter_bash::LANGUAGE.into(),
            "bash",
            tree_sitter_bash::HIGHLIGHT_QUERY,
            "",
            "",
        ),
        Language::Html => HighlightConfiguration::new(
            tree_sitter_html::LANGUAGE.into(),
            "html",
            tree_sitter_html::HIGHLIGHTS_QUERY,
            tree_sitter_html::INJECTIONS_QUERY,
            "",
        ),
        Language::Css => HighlightConfiguration::new(
            tree_sitter_css::LANGUAGE.into(),
            "css",
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::Toml => HighlightConfiguration::new(
            tree_sitter_toml_ng::LANGUAGE.into(),
            "toml",
            tree_sitter_toml_ng::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::Yaml => HighlightConfiguration::new(
            tree_sitter_yaml::LANGUAGE.into(),
            "yaml",
            tree_sitter_yaml::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::Sql => HighlightConfiguration::new(
            tree_sitter_sequel::LANGUAGE.into(),
            "sql",
            tree_sitter_sequel::HIGHLIGHTS_QUERY,
            "",
            "",
        ),
        Language::Markdown => HighlightConfiguration::new(
            tree_sitter_md::LANGUAGE.into(),
            "markdown",
            tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            tree_sitter_md::INJECTION_QUERY_BLOCK,
            "",
        ),
        _ => return None,
    }
    .ok()
}
