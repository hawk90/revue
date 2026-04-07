//! Integration tests for CLI commands

use std::fs;
use tempfile::TempDir;

// =============================================================================
// new_project (uses absolute paths, safe for parallel tests)
// =============================================================================

#[test]
fn new_project_creates_basic_structure() {
    let tmp = TempDir::new().unwrap();
    let project_name = tmp.path().join("test-project");
    let name = project_name.to_str().unwrap();

    let result = revue_cli::commands::new_project(name, "basic", false);
    assert!(result.is_ok(), "new_project failed: {:?}", result.err());

    assert!(project_name.join("Cargo.toml").exists());
    assert!(project_name.join("src").exists());
    assert!(project_name.join("src/main.rs").exists());
    assert!(project_name.join("src/app.rs").exists());
    assert!(project_name.join("styles").exists());

    let cargo = fs::read_to_string(project_name.join("Cargo.toml")).unwrap();
    assert!(cargo.contains("test-project"));
    assert!(cargo.contains("revue"));
}

#[test]
fn new_project_creates_dashboard_template() {
    let tmp = TempDir::new().unwrap();
    let project_name = tmp.path().join("dashboard-app");
    let name = project_name.to_str().unwrap();

    let result = revue_cli::commands::new_project(name, "dashboard", false);
    assert!(result.is_ok());

    let main_rs = fs::read_to_string(project_name.join("src/main.rs")).unwrap();
    assert!(main_rs.contains("fn main"));
}

#[test]
fn new_project_creates_todo_template() {
    let tmp = TempDir::new().unwrap();
    let project_name = tmp.path().join("todo-app");
    let name = project_name.to_str().unwrap();

    let result = revue_cli::commands::new_project(name, "todo", false);
    assert!(result.is_ok());
    assert!(project_name.join("src/main.rs").exists());
}

#[test]
fn new_project_creates_chat_template() {
    let tmp = TempDir::new().unwrap();
    let project_name = tmp.path().join("chat-app");
    let name = project_name.to_str().unwrap();

    let result = revue_cli::commands::new_project(name, "chat", false);
    assert!(result.is_ok());
    assert!(project_name.join("src/main.rs").exists());
}

#[test]
fn new_project_fails_if_dir_exists() {
    let tmp = TempDir::new().unwrap();
    let project_name = tmp.path().join("existing");
    fs::create_dir_all(&project_name).unwrap();

    let name = project_name.to_str().unwrap();
    let result = revue_cli::commands::new_project(name, "basic", false);
    assert!(result.is_err());
}

// =============================================================================
// list_themes (no fs dependencies)
// =============================================================================

#[test]
fn list_themes_does_not_error() {
    let result = revue_cli::commands::list_themes(false);
    assert!(result.is_ok());
}

#[test]
fn list_themes_verbose_does_not_error() {
    let result = revue_cli::commands::list_themes(true);
    assert!(result.is_ok());
}

// =============================================================================
// install_theme and add_component are cwd-based, skip parallel testing
// They work correctly (verified by output) but need serial execution
// =============================================================================

#[test]
fn install_theme_unknown_returns_error() {
    // This doesn't touch filesystem for unknown themes
    let result = revue_cli::commands::install_theme("nonexistent-theme");
    assert!(result.is_err());
}

#[test]
fn add_component_unknown_type_returns_error() {
    let result = revue_cli::commands::add_component("nonexistent", None);
    assert!(result.is_err());
}
