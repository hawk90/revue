use revue::widget::filepicker;

#[test]
fn test_filepicker_select() {
    let mut picker = filepicker(".");
    picker.select("test.txt");
}

#[test]
fn test_filepicker_extension() {
    let picker = filepicker(".").extension("rs");
}
