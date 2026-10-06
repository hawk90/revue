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

    // `name` was a full path; the package is named after its last component.
    let cargo: toml::Table = fs::read_to_string(project_name.join("Cargo.toml"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(cargo["package"]["name"].as_str(), Some("test-project"));
    assert!(cargo["dependencies"]["revue"].is_str());
}

#[test]
fn new_project_writes_each_template() {
    for &template in revue_cli::templates::PROJECT_TEMPLATES {
        let tmp = TempDir::new().unwrap();
        let project = tmp.path().join(format!("{template}-app"));
        let result = revue_cli::commands::new_project(project.to_str().unwrap(), template, true);
        assert!(result.is_ok(), "{template}: {:?}", result.err());

        let read = |path: &str| {
            fs::read_to_string(project.join(path))
                .unwrap_or_else(|e| panic!("{template}: {path}: {e}"))
        };
        let files = revue_cli::templates::project_files(template).unwrap();
        assert_eq!(read("src/main.rs"), files.main_rs, "{template}");
        assert_eq!(read("src/app.rs"), files.app_rs, "{template}");
        // main.rs loads this path, relative to the project root.
        assert_eq!(read("styles/main.css"), files.style_css, "{template}");
        assert_eq!(read(".gitignore"), revue_cli::templates::gitignore());

        let cargo: toml::Table = read("Cargo.toml").parse().unwrap();
        assert_eq!(
            cargo["package"]["name"].as_str(),
            Some(format!("{template}-app").as_str())
        );
        assert_eq!(
            cargo["dependencies"]["revue"].as_str(),
            Some(revue_cli::templates::REVUE_VERSION),
            "{template}: revue comes from crates.io"
        );
    }
}

#[test]
fn new_project_rejects_an_unknown_template() {
    let tmp = TempDir::new().unwrap();
    let project = tmp.path().join("app");

    let result = revue_cli::commands::new_project(project.to_str().unwrap(), "nope", false);
    let error = result.unwrap_err().to_string();
    assert!(error.contains("nope"), "{error}");
    assert!(error.contains("dashboard"), "lists the templates: {error}");
    assert!(!project.exists());
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
