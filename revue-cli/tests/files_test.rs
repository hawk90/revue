//! Commands that write into the project, run as the `revue` binary in a
//! project directory of their own (#1212)

use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

fn revue(dir: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_revue"))
        .args(args)
        .current_dir(dir)
        // the docs go where cargo puts them for this project, not where the
        // test run's own target dir is
        .env_remove("CARGO_TARGET_DIR")
        .output()
        .unwrap()
}

#[test]
fn installing_a_theme_writes_it() {
    let dir = TempDir::new().unwrap();
    let out = revue(dir.path(), &["theme", "dracula"]);
    assert!(out.status.success(), "{out:?}");
    let css = fs::read_to_string(dir.path().join("styles/dracula.css")).unwrap();
    assert_eq!(css, revue_cli::templates::theme_dracula());
}

#[test]
fn installing_a_theme_keeps_an_edited_file() {
    let dir = TempDir::new().unwrap();
    fs::create_dir(dir.path().join("styles")).unwrap();
    let path = dir.path().join("styles/dracula.css");
    fs::write(&path, "/* mine */").unwrap();

    let out = revue(dir.path(), &["theme", "dracula"]);
    assert!(!out.status.success());
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("--force"),
        "{out:?}"
    );
    assert_eq!(fs::read_to_string(&path).unwrap(), "/* mine */");

    let out = revue(dir.path(), &["theme", "dracula", "--force"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(
        fs::read_to_string(&path).unwrap(),
        revue_cli::templates::theme_dracula()
    );
}

/// A crate with nothing to build but its own docs
fn doc_crate() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"doc-demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    fs::create_dir(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("src/lib.rs"),
        "//! Demo\n\n/// A thing\npub fn thing() {}\n",
    )
    .unwrap();
    dir
}

#[test]
fn docs_go_into_the_output_directory() {
    let dir = doc_crate();
    let out = revue(dir.path(), &["docs", "--output", "site"]);
    assert!(out.status.success(), "{out:?}");
    assert!(dir.path().join("site/doc_demo/index.html").is_file());
    assert!(dir.path().join("site/doc_demo/fn.thing.html").is_file());
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("site/doc_demo/index.html"), "{stdout}");
}

#[test]
fn docs_without_an_output_stay_in_the_target_dir() {
    let dir = doc_crate();
    let out = revue(dir.path(), &["docs"]);
    assert!(out.status.success(), "{out:?}");
    assert!(dir.path().join("target/doc/doc_demo/index.html").is_file());
    // the old default, `docs`, is not written to
    assert!(!dir.path().join("docs").exists());
}

#[test]
fn the_version_is_the_crate_version() {
    let dir = TempDir::new().unwrap();
    let out = revue(dir.path(), &["--version"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(
        String::from_utf8_lossy(&out.stdout).trim(),
        format!("revue {}", env!("CARGO_PKG_VERSION"))
    );
}
