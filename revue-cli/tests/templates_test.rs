//! Tests for CLI template generation functions

use revue_cli::templates;

// =============================================================================
// Core project templates
// =============================================================================

#[test]
fn cargo_toml_contains_project_name() {
    let content = templates::cargo_toml("my-app");
    assert!(content.contains("my-app"));
    assert!(content.contains("[package]"));
    assert!(content.contains("[dependencies]"));
    assert!(content.contains("revue"));
}

#[test]
fn cargo_toml_depends_on_revue_from_crates_io() {
    let manifest: toml::Table = templates::cargo_toml("my-app").parse().unwrap();
    let revue = &manifest["dependencies"]["revue"];
    assert_eq!(revue.as_str(), Some(templates::REVUE_VERSION));
}

/// The generated projects must ask for the release line this repository's
/// `revue` belongs to: `REVUE_VERSION` (a caret requirement such as "3.0")
/// must match the workspace crate's version.
#[test]
fn revue_version_matches_the_revue_crate() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../Cargo.toml");
    let manifest: toml::Table = std::fs::read_to_string(path).unwrap().parse().unwrap();
    let crate_version = manifest["package"]["version"].as_str().unwrap();

    let numbers = |v: &str| -> Vec<u64> { v.split('.').map(|n| n.parse().unwrap()).collect() };
    let required = numbers(templates::REVUE_VERSION);
    let actual = numbers(crate_version);
    assert!(
        actual[0] == required[0] && actual[1] >= required[1],
        "templates ask for revue {}, but the crate is {}; port the templates \
         (`revue new`) and update REVUE_VERSION",
        templates::REVUE_VERSION,
        crate_version
    );
}

#[test]
fn cargo_toml_different_names() {
    let a = templates::cargo_toml("alpha");
    let b = templates::cargo_toml("beta");
    assert!(a.contains("alpha"));
    assert!(b.contains("beta"));
    assert!(!a.contains("beta"));
}

#[test]
fn gitignore_not_empty() {
    let content = templates::gitignore();
    assert!(!content.is_empty());
    assert!(content.contains("target"));
}

#[test]
fn default_style_not_empty() {
    let content = templates::default_style();
    assert!(!content.is_empty());
}

// =============================================================================
// Project type templates
// =============================================================================

#[test]
fn basic_templates_valid() {
    let main = templates::basic_main();
    let app = templates::basic_app();
    assert!(main.contains("fn main"));
    assert!(!app.is_empty());
}

#[test]
fn dashboard_templates_valid() {
    let main = templates::dashboard_main();
    let app = templates::dashboard_app();
    assert!(main.contains("fn main"));
    assert!(!app.is_empty());
}

#[test]
fn todo_templates_valid() {
    let main = templates::todo_main();
    let app = templates::todo_app();
    assert!(main.contains("fn main"));
    assert!(!app.is_empty());
}

#[test]
fn chat_templates_valid() {
    let main = templates::chat_main();
    let app = templates::chat_app();
    assert!(main.contains("fn main"));
    assert!(!app.is_empty());
}

// =============================================================================
// Theme templates
// =============================================================================

#[test]
fn theme_dracula_valid_css() {
    let css = templates::theme_dracula();
    assert!(!css.is_empty());
    assert!(css.contains("dracula") || css.contains("Dracula") || css.contains('#'));
}

#[test]
fn theme_nord_valid_css() {
    let css = templates::theme_nord();
    assert!(!css.is_empty());
}

#[test]
fn theme_monokai_valid_css() {
    let css = templates::theme_monokai();
    assert!(!css.is_empty());
}

#[test]
fn theme_gruvbox_valid_css() {
    let css = templates::theme_gruvbox();
    assert!(!css.is_empty());
}

#[test]
fn theme_catppuccin_valid_css() {
    let css = templates::theme_catppuccin();
    assert!(!css.is_empty());
}

// =============================================================================
// Component templates
// =============================================================================

#[test]
fn component_search_valid() {
    let content = templates::component_search();
    assert!(content.contains("struct"));
    assert!(content.contains("fn"));
}

#[test]
fn component_form_valid() {
    let content = templates::component_form();
    assert!(content.contains("struct"));
}

#[test]
fn component_navigation_valid() {
    let content = templates::component_navigation();
    assert!(content.contains("struct"));
}

#[test]
fn component_modal_valid() {
    let content = templates::component_modal();
    assert!(content.contains("struct"));
}

#[test]
fn component_toast_valid() {
    let content = templates::component_toast();
    assert!(content.contains("struct"));
}

#[test]
fn component_command_palette_valid() {
    let content = templates::component_command_palette();
    assert!(content.contains("struct"));
}

#[test]
fn component_table_valid() {
    let content = templates::component_table();
    assert!(content.contains("struct"));
}

#[test]
fn component_tabs_valid() {
    let content = templates::component_tabs();
    assert!(content.contains("struct"));
}
