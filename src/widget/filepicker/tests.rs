use super::*;
use std::path::Path;

#[test]
fn test_file_picker_new() {
    let picker = FilePicker::new();
    assert!(picker.current_dir.exists());
    assert_eq!(picker.mode, PickerMode::Open);
}

#[test]
fn test_file_picker_modes() {
    let open = FilePicker::new();
    assert_eq!(open.mode, PickerMode::Open);

    let save = FilePicker::save();
    assert_eq!(save.mode, PickerMode::Save);

    let dir = FilePicker::directory();
    assert_eq!(dir.mode, PickerMode::Directory);

    let multi = FilePicker::multi_select();
    assert_eq!(multi.mode, PickerMode::MultiSelect);
}

#[test]
fn test_file_filter_extensions() {
    let filter = FileFilter::extensions(&["rs", "toml"]);

    assert!(filter.matches(Path::new("main.rs")));
    assert!(filter.matches(Path::new("Cargo.toml")));
    assert!(!filter.matches(Path::new("readme.md")));
}

#[test]
fn test_file_filter_pattern() {
    let filter = FileFilter::pattern("*.rs");
    assert!(filter.matches(Path::new("main.rs")));
    assert!(!filter.matches(Path::new("main.py")));

    let filter2 = FileFilter::pattern("test*");
    assert!(filter2.matches(Path::new("test_main.rs")));
    assert!(!filter2.matches(Path::new("main_test.rs")));
}

#[test]
fn test_picker_entry_format_size() {
    let mut entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 1024,
        selected: false,
    };

    assert_eq!(entry.format_size(), "1.0 KB");

    entry.size = 1024 * 1024;
    assert_eq!(entry.format_size(), "1.0 MB");

    entry.is_dir = true;
    assert_eq!(entry.format_size(), "<DIR>");
}

#[test]
fn test_navigation() {
    let mut picker = FilePicker::new();
    let _initial_dir = picker.current_dir.clone();

    // These tests depend on filesystem, so just check basic operations
    picker.highlight_next();
    picker.highlight_previous();

    assert!(!picker.history.is_empty());
}

#[test]
fn test_save_mode_input() {
    let mut picker = FilePicker::save();
    picker.input_char('t');
    picker.input_char('e');
    picker.input_char('s');
    picker.input_char('t');
    assert_eq!(picker.input_name, "test");

    picker.input_backspace();
    assert_eq!(picker.input_name, "tes");
}

#[test]
fn test_helper_functions() {
    let fp = file_picker();
    assert_eq!(fp.mode, PickerMode::Open);

    let sp = save_picker();
    assert_eq!(sp.mode, PickerMode::Save);

    let dp = dir_picker();
    assert_eq!(dp.mode, PickerMode::Directory);
}

// Security tests for path traversal

#[test]
fn test_reject_double_dot_slash() {
    let picker = FilePicker::new();
    let result = picker.try_set_start_dir("../../../etc/passwd");
    assert!(result.is_err());
    if let Err(FilePickerError::PathTraversal(_)) = result {
        // Expected
    } else {
        panic!("Expected PathTraversal error");
    }
}

#[test]
fn test_reject_double_dot_backslash() {
    let picker = FilePicker::new();
    // On Unix, backslash is a valid filename character, not a path separator
    // On Windows, this would be detected as path traversal
    let result = picker.try_set_start_dir("..\\..\\system32");
    // On Unix: this is a valid path with weird backslashes, might not exist
    // On Windows: should be rejected as traversal
    #[cfg(windows)]
    {
        assert!(result.is_err());
        if let Err(FilePickerError::PathTraversal(_)) = result {
            // Expected
        } else {
            panic!("Expected PathTraversal error");
        }
    }
    #[cfg(not(windows))]
    {
        // On Unix, backslash is just a character - the path validation
        // will catch it via component check (.. is a ParentDir component)
        // We just check the function doesn't panic
        let _ = result;
    }
}

#[test]
fn test_reject_parent_dir_component() {
    let picker = FilePicker::new();
    let mut picker = picker;
    let path = PathBuf::from("..").join("etc");
    let result = picker.navigate_to(&path);
    assert!(result.is_err());
}

#[test]
fn test_navigate_to_rejects_traversal() {
    let mut picker = FilePicker::new();
    let traversal_path = Path::new("../../../etc");
    let result = picker.navigate_to(traversal_path);
    assert!(result.is_err());
}

#[test]
fn test_allow_valid_paths() {
    let picker = FilePicker::new();
    // Valid absolute path should work (if it exists)
    if let Ok(current) = std::env::current_dir() {
        let result = picker.try_set_start_dir(&current);
        assert!(result.is_ok());
    }
}

#[test]
fn test_allow_current_directory() {
    let picker = FilePicker::new();
    let result = picker.try_set_start_dir(".");
    assert!(result.is_ok());
}

#[test]
fn test_go_up_stays_safe() {
    let mut picker = FilePicker::new();
    let _initial_dir = picker.current_dir().to_path_buf();
    picker.go_up();
    // go_up should always navigate to parent, which should be valid
    // (may or may not change directory depending on where we started)
}

#[test]
fn test_enter_directory_safe() {
    let mut picker = FilePicker::new();
    // Navigate within the picker should be safe
    picker.highlight_next();
    if let Some(entry) = picker.highlighted_entry() {
        if entry.is_dir {
            let _initial_dir = picker.current_dir().to_path_buf();
            picker.enter();
            // Either we navigated or stayed, but shouldn't panic
        }
    }
}

#[test]
fn test_path_traversal_error_message() {
    let picker = FilePicker::new();
    let result = picker.try_set_start_dir("../../../etc/passwd");
    if let Err(e) = result {
        let msg = format!("{}", e);
        assert!(
            msg.contains("traversal") || msg.contains("parent"),
            "Error message should mention traversal: {}",
            msg
        );
    } else {
        panic!("Expected error for path traversal");
    }
}

#[test]
fn test_outside_directory_error() {
    let mut picker = FilePicker::new();
    // Try to navigate to a path outside the current directory tree
    // This test is platform-dependent, so we just check the function doesn't panic
    let outside_path = Path::new("/tmp/revue_test_nonexistent_outside");
    let _result = picker.navigate_to(outside_path);
    // Should either succeed (if path exists) or fail with appropriate error
    // but should not panic
}

// Additional security hardening tests

#[test]
fn test_reject_null_byte_in_path() {
    let picker = FilePicker::new();
    // Path with null byte should be rejected
    let null_path = Path::new("test.txt\0malicious.exe");
    let result = picker.try_set_start_dir(null_path);
    assert!(result.is_err());
    if let Err(FilePickerError::InvalidCharacters) = result {
        // Expected
    } else {
        panic!("Expected InvalidCharacters error for null byte");
    }
}

#[test]
fn test_reject_windows_reserved_names() {
    // Test Windows reserved device names
    #[cfg(windows)]
    {
        let reserved_names = ["CON", "PRN", "AUX", "NUL", "COM1", "LPT1"];
        for name in reserved_names {
            let picker = FilePicker::new();
            let path = PathBuf::from("/tmp").join(name);
            // try_set_start_dir returns Result - just verify it doesn't panic
            // The validation should handle reserved names gracefully
            let _ = picker.try_set_start_dir(&path);
        }
    }
    #[cfg(not(windows))]
    {
        // On non-Windows, this test is not applicable
        // Just verify that FilePicker creation doesn't panic
        let _picker = FilePicker::new();
    }
}

#[test]
fn test_truncate_string_safe_utf8() {
    use validation::truncate_string_safe;

    // Test UTF-8 safe truncation
    let ascii = "hello_world_test.txt";
    let truncated = truncate_string_safe(ascii, 10);
    // truncate_string_safe adds "..." so result will be at most max_len + 3
    assert!(truncated.ends_with("..."));
    assert!(truncated.len() <= 13); // 10 + "..."
}

#[test]
fn test_truncate_string_single_emoji() {
    use validation::truncate_string_safe;

    // Emoji is 4 bytes in UTF-8
    let emoji = "😀😀😀😀😀";
    let truncated = truncate_string_safe(emoji, 10);
    // Should handle gracefully without panic
    assert!(truncated.ends_with("..."));
}

#[test]
fn test_multiselect_preserves_paths_on_canonicalize_failure() {
    let mut picker = FilePicker::multi_select();
    picker
        .selected
        .push(PathBuf::from("/nonexistent/path/file1.txt"));
    picker
        .selected
        .push(PathBuf::from("/nonexistent/path/file2.txt"));

    let result = picker.confirm();
    match result {
        PickerResult::Multiple(paths) => {
            // Should return the original paths even if canonicalize fails
            assert_eq!(paths.len(), 2);
        }
        _ => panic!("Expected Multiple with preserved paths"),
    }
}

#[cfg(unix)]
#[test]
fn navigate_to_an_unreadable_directory_is_an_error_and_stays_put() {
    use std::os::unix::fs::PermissionsExt;
    let dir = tempfile::tempdir().unwrap();
    let locked = dir.path().join("locked");
    std::fs::create_dir(&locked).unwrap();
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::read_dir(&locked).is_ok() {
        // Running as root: nothing is denied.
        std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o755)).unwrap();
        return;
    }
    let mut picker = FilePicker::new().start_dir(dir.path());
    let before = picker.current_dir().to_path_buf();
    let result = picker.navigate_to(&locked);
    std::fs::set_permissions(&locked, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(matches!(result, Err(FilePickerError::IoError(_))));
    assert_eq!(picker.current_dir(), before);
}

#[cfg(unix)]
#[test]
fn an_entry_whose_name_is_not_utf8_is_listed() {
    use std::os::unix::ffi::OsStrExt;
    // No file needed: the name alone used to make from_path give up.
    let path = Path::new("/nowhere").join(std::ffi::OsStr::from_bytes(b"bad-\xff-name.txt"));
    let entry = PickerEntry::from_path(&path).expect("entry left out of the listing");
    assert_eq!(entry.name, "bad-\u{fffd}-name.txt");
    assert_eq!(entry.path, path);
}
