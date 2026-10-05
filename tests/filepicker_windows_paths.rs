//! A FilePicker accepts the paths the standard library hands back.
//!
//! On Windows `Path::canonicalize` returns the verbatim form `\\?\C:\...`.
//! The traversal check rejected any `\\?\` prefix shorter than ten
//! characters as a device path, so a canonicalized directory - the most
//! ordinary input there is - made `start_dir` panic. The prefix is now
//! judged by its kind: disks and UNC shares pass in either spelling, the
//! device namespace does not.

use revue::widget::file_picker;

#[test]
fn a_canonicalized_directory_is_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().canonicalize().unwrap();

    let picker = file_picker()
        .try_set_start_dir(&root)
        .expect("a canonicalized directory must be accepted");
    assert_eq!(picker.current_dir(), root);
}

#[cfg(windows)]
#[test]
fn a_verbatim_disk_path_is_accepted() {
    assert!(file_picker().try_set_start_dir(r"\\?\C:\Windows").is_ok());
    assert!(file_picker().try_set_start_dir(r"C:\Windows").is_ok());
}

#[cfg(windows)]
#[test]
fn the_device_namespace_is_still_rejected() {
    assert!(file_picker().try_set_start_dir(r"\\.\COM1").is_err());
    assert!(file_picker()
        .try_set_start_dir(r"\\.\PhysicalDrive0")
        .is_err());
}
