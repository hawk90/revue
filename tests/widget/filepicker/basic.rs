use revue::widget::filepicker;

#[test]
fn test_filepicker_new() {
    let picker = filepicker(".");
}

#[test]
fn test_filepicker_with_path() {
    let picker = filepicker("/home/user");
}
