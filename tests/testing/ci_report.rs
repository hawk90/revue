//! `TestReport::save_artifacts`: the report and the diff files of failures
//! land in the artifacts directory

use revue::testing::{CiEnvironment, CiProvider, TestReport};
use std::fs;
use std::path::Path;

fn ci_with_artifacts(dir: &Path) -> CiEnvironment {
    CiEnvironment {
        provider: CiProvider::Local,
        is_ci: false,
        branch: None,
        commit: None,
        pr_number: None,
        build_number: None,
        artifacts_dir: dir.to_path_buf(),
    }
}

/// File names in `dir`, sorted
fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

#[test]
fn test_save_artifacts_copies_diff_files() {
    let src = tempfile::tempdir().unwrap();
    let out = tempfile::tempdir().unwrap();
    let diff = src.path().join("button.diff");
    fs::write(&diff, "diff body").unwrap();

    let mut report = TestReport::new();
    report.add_passed("ok");
    report.add_failed_with_diff("button", "mismatch", &diff);
    // A missing diff file is skipped
    report.add_failed_with_diff("gone", "mismatch", src.path().join("gone.diff"));
    report
        .save_artifacts(&ci_with_artifacts(out.path()))
        .unwrap();

    assert_eq!(names(out.path()), ["button.diff", "visual-test-report.md"]);
    assert_eq!(
        fs::read_to_string(out.path().join("button.diff")).unwrap(),
        "diff body"
    );
}

#[test]
fn test_save_artifacts_skips_an_existing_diff_path_without_a_file_name() {
    // `dir/..` and `/` exist but have no file name to copy to (#1584)
    let src = tempfile::tempdir().unwrap();
    let out = tempfile::tempdir().unwrap();
    let diff = src.path().join("table.diff");
    fs::write(&diff, "table").unwrap();
    let parent = src.path().join("sub");
    fs::create_dir(&parent).unwrap();

    let mut report = TestReport::new();
    report.add_failed_with_diff("dotdot", "mismatch", parent.join(".."));
    report.add_failed_with_diff("root", "mismatch", "/");
    report.add_failed_with_diff("table", "mismatch", &diff);
    report
        .save_artifacts(&ci_with_artifacts(out.path()))
        .unwrap();

    // The paths without a name are skipped; the one after them is copied
    assert_eq!(names(out.path()), ["table.diff", "visual-test-report.md"]);
    assert_eq!(
        fs::read_to_string(out.path().join("table.diff")).unwrap(),
        "table"
    );
}

#[test]
fn test_save_artifacts_writes_nothing_when_all_passed() {
    let out = tempfile::tempdir().unwrap();
    let artifacts = out.path().join("artifacts");
    let mut report = TestReport::new();
    report.add_passed("ok");
    report
        .save_artifacts(&ci_with_artifacts(&artifacts))
        .unwrap();
    assert!(!artifacts.exists());
}
