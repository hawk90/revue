//! Tests for EditorViewMode

use super::*;

#[test]
fn test_editor_view_mode_default() {
    assert_eq!(EditorViewMode::default(), EditorViewMode::Editor);
}
