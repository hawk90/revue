//! Tests for the pure parts of `revue dev`: what a changed path means, when a
//! burst of changes has settled, how CSS changes are handled, and how the
//! binary to run is found.

use revue_cli::dev::{
    built_binaries, cargo_build_args, choose_binary, classify, honors_hot_reload_env,
    revue_dependency, revue_package_dir, BuiltBinary, ChangeKind, CssMode, Debouncer,
};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

// =============================================================================
// classify
// =============================================================================

#[test]
fn rust_sources_and_the_manifest_rebuild() {
    for path in [
        "src/main.rs",
        "src/widgets/list.rs",
        "Cargo.toml",
        "build.rs",
        "src/data.json",
        "./src/app.rs",
    ] {
        assert_eq!(classify(Path::new(path)), ChangeKind::Rebuild, "{path}");
    }
}

#[test]
fn stylesheets_are_css() {
    for path in [
        "styles/main.css",
        "styles/themes/dark.css",
        "src/widget.CSS",
        "config/extra.css",
        // An editor's temp file next to a stylesheet is still a stylesheet change.
        "styles/main.css.tmp1234",
        "styles/4913x",
    ] {
        assert_eq!(classify(Path::new(path)), ChangeKind::Css, "{path}");
    }
}

#[test]
fn build_output_vcs_and_editor_files_are_ignored() {
    for path in [
        "target/debug/demo",
        "target/debug/.fingerprint/demo/output",
        "src/target/x.rs",
        ".git/index",
        "src/.main.rs.swp",
        "src/main.rs~",
        "src/4913",
        "src/main.rs.swp",
        "Cargo.lock",
        ".DS_Store",
        "",
    ] {
        assert_eq!(classify(Path::new(path)), ChangeKind::Ignore, "{path:?}");
    }
}

#[test]
fn a_file_named_styles_at_the_root_is_not_a_stylesheet() {
    assert_eq!(classify(Path::new("styles")), ChangeKind::Rebuild);
}

// =============================================================================
// Debouncer
// =============================================================================

const QUIET: Duration = Duration::from_millis(250);
const MAX: Duration = Duration::from_secs(2);

#[test]
fn nothing_is_due_without_a_change() {
    let d = Debouncer::new(QUIET, MAX);
    let now = Instant::now();
    assert!(!d.is_pending());
    assert!(!d.is_due(now + Duration::from_secs(60)));
    assert_eq!(d.time_left(now), None);
}

#[test]
fn a_burst_is_due_once_quiet() {
    let mut d = Debouncer::new(QUIET, MAX);
    let t0 = Instant::now();
    d.push("src/a.rs".into(), t0);
    d.push("src/b.rs".into(), t0 + Duration::from_millis(100));
    d.push("src/a.rs".into(), t0 + Duration::from_millis(200));

    assert!(d.is_pending());
    assert!(!d.is_due(t0 + Duration::from_millis(400)));
    assert_eq!(
        d.time_left(t0 + Duration::from_millis(400)),
        Some(Duration::from_millis(50))
    );
    assert!(d.is_due(t0 + Duration::from_millis(450)));

    // The first path of the burst is reported, then the burst is cleared.
    assert_eq!(d.take(), Some(PathBuf::from("src/a.rs")));
    assert!(!d.is_pending());
    assert!(!d.is_due(t0 + Duration::from_secs(10)));
}

#[test]
fn a_burst_that_never_settles_is_due_after_the_cap() {
    let mut d = Debouncer::new(QUIET, MAX);
    let t0 = Instant::now();
    let mut t = t0;
    while t < t0 + MAX {
        d.push("src/gen.rs".into(), t);
        assert!(!d.is_due(t), "due too early at {:?}", t - t0);
        t += Duration::from_millis(100);
    }
    d.push("src/gen.rs".into(), t);
    assert!(d.is_due(t));
}

// =============================================================================
// CssMode
// =============================================================================

const MANIFEST: &str = r#"
[package]
name = "demo"
version = "0.1.0"
edition = "2021"

[dependencies]
revue = "3.0"
"#;

#[test]
fn hot_reload_when_revue_honors_the_env_var() {
    let mode = CssMode::detect(MANIFEST, Some(true));
    assert_eq!(
        mode,
        CssMode::HotReload {
            dep: "revue".into()
        }
    );
    assert!(!mode.restarts_on(ChangeKind::Css));
    assert!(mode.restarts_on(ChangeKind::Rebuild));
    assert!(!mode.restarts_on(ChangeKind::Ignore));
}

#[test]
fn restart_on_css_when_revue_is_older_or_unknown() {
    for honors_env in [Some(false), None] {
        let mode = CssMode::detect(MANIFEST, honors_env);
        assert!(matches!(mode, CssMode::Restart(_)), "{honors_env:?}");
        assert!(mode.restarts_on(ChangeKind::Css));
        assert!(mode.restarts_on(ChangeKind::Rebuild));
    }
}

#[test]
fn revue_package_dir_reads_cargo_metadata() {
    let metadata = r#"{"packages":[
        {"name":"demo","manifest_path":"/p/demo/Cargo.toml"},
        {"name":"revue","manifest_path":"/registry/revue-3.0.1/Cargo.toml"}
    ]}"#;
    assert_eq!(
        revue_package_dir(metadata),
        Some(PathBuf::from("/registry/revue-3.0.1"))
    );
    assert_eq!(revue_package_dir(r#"{"packages":[]}"#), None);
    assert_eq!(revue_package_dir("not json"), None);
}

#[test]
fn honors_hot_reload_env_looks_for_the_variable_in_the_source() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src/core/app");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("mod.rs"), "pub fn hot_reload() {}\n").unwrap();
    assert!(!honors_hot_reload_env(dir.path()));

    std::fs::write(
        src.join("builder.rs"),
        "const HOT_RELOAD_ENV: &str = \"REVUE_HOT_RELOAD\";\n",
    )
    .unwrap();
    assert!(honors_hot_reload_env(dir.path()));

    assert!(!honors_hot_reload_env(&dir.path().join("missing")));
}

/// The library this CLI ships with honors the variable.
#[test]
fn the_revue_in_this_repository_honors_the_env_var() {
    let repo = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    assert!(honors_hot_reload_env(repo));
}

#[test]
fn restart_on_css_without_a_revue_dependency() {
    let manifest = "[package]\nname = \"x\"\n\n[dependencies]\nserde = \"1\"\n";
    assert!(matches!(
        CssMode::detect(manifest, None),
        CssMode::Restart(_)
    ));
    // A dev-dependency cannot take `--features revue/hot-reload`.
    let manifest = "[package]\nname = \"x\"\n\n[dev-dependencies]\nrevue = \"3\"\n";
    assert!(matches!(
        CssMode::detect(manifest, None),
        CssMode::Restart(_)
    ));
}

#[test]
fn revue_dependency_finds_tables_and_renames() {
    assert_eq!(revue_dependency(MANIFEST).as_deref(), Some("revue"));
    assert_eq!(
        revue_dependency("[dependencies.revue]\npath = \"../revue\"\n").as_deref(),
        Some("revue")
    );
    assert_eq!(
        revue_dependency("[dependencies]\ntui = { package = \"revue\", version = \"3\" }\n")
            .as_deref(),
        Some("tui")
    );
    assert_eq!(revue_dependency("not toml ["), None);
}

// =============================================================================
// cargo build arguments and the binary to run
// =============================================================================

#[test]
fn build_args_enable_hot_reload_and_select_a_binary() {
    let hot = CssMode::HotReload { dep: "tui".into() };
    assert_eq!(
        cargo_build_args(&hot, Some("demo")),
        [
            "build",
            "--message-format=json-render-diagnostics",
            "--features",
            "tui/hot-reload",
            "--bin",
            "demo"
        ]
    );
    let restart = CssMode::Restart(String::new());
    assert_eq!(
        cargo_build_args(&restart, None),
        ["build", "--message-format=json-render-diagnostics"]
    );
}

const MESSAGES: &str = r#"{"reason":"compiler-artifact","package_id":"revue 3.0.0","target":{"kind":["lib"],"name":"revue"},"executable":null}
{"reason":"compiler-artifact","package_id":"demo 0.1.0","target":{"kind":["custom-build"],"name":"build-script-build"},"executable":null}
not json
{"reason":"compiler-artifact","package_id":"demo 0.1.0","target":{"kind":["bin"],"name":"demo"},"executable":"/p/target/debug/demo"}
{"reason":"compiler-artifact","package_id":"demo 0.1.0","target":{"kind":["bin"],"name":"tool"},"executable":"/p/target/debug/tool"}
{"reason":"build-finished","success":true}
"#;

fn bin(name: &str) -> BuiltBinary {
    BuiltBinary {
        name: name.into(),
        path: PathBuf::from(format!("/p/target/debug/{name}")),
    }
}

#[test]
fn built_binaries_keeps_only_executables() {
    assert_eq!(built_binaries(MESSAGES), [bin("demo"), bin("tool")]);
}

#[test]
fn choose_binary_picks_the_only_one() {
    assert_eq!(
        choose_binary(&[bin("demo")], None, None),
        Ok(PathBuf::from("/p/target/debug/demo"))
    );
}

#[test]
fn choose_binary_needs_a_name_when_there_are_several() {
    let bins = [bin("demo"), bin("tool")];
    let err = choose_binary(&bins, None, None).unwrap_err();
    assert!(err.contains("--bin") && err.contains("demo, tool"), "{err}");

    assert_eq!(
        choose_binary(&bins, Some("tool"), None),
        Ok(PathBuf::from("/p/target/debug/tool"))
    );
    assert_eq!(
        choose_binary(&bins, None, Some("demo")),
        Ok(PathBuf::from("/p/target/debug/demo"))
    );
    // --bin wins over default-run.
    assert_eq!(
        choose_binary(&bins, Some("tool"), Some("demo")),
        Ok(PathBuf::from("/p/target/debug/tool"))
    );
    assert!(choose_binary(&bins, Some("nope"), None).is_err());
    assert!(choose_binary(&[], None, None).is_err());
}
