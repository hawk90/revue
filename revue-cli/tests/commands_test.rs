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
        // Every file of the template, at its path in the template: main.rs
        // loads `styles/main.css` relative to the project root.
        let mut dirs = vec![revue_cli::templates::project_files(template).unwrap()];
        while let Some(dir) = dirs.pop() {
            for file in dir.files() {
                let path = file.path().strip_prefix(template).unwrap();
                let written = fs::read(project.join(path))
                    .unwrap_or_else(|e| panic!("{template}: {}: {e}", path.display()));
                assert_eq!(written, file.contents(), "{template}: {}", path.display());
            }
            dirs.extend(dir.dirs());
        }
        for path in ["src/main.rs", "src/app.rs", "styles/main.css"] {
            assert!(project.join(path).is_file(), "{template}: {path}");
        }
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
    let result = revue_cli::commands::install_theme("nonexistent-theme", false);
    assert!(result.is_err());
}

#[test]
fn add_component_unknown_type_returns_error() {
    let result = revue_cli::commands::add_component("nonexistent", None);
    assert!(result.is_err());
}
