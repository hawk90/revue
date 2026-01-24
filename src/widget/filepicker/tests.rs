#[cfg(test)]
mod tests {
    use super::*;

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

        assert!(picker.history.len() >= 1);
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
}
