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
