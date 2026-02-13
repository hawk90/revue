//! Diff Viewer widget tests for public APIs

use revue::widget::diff::{DiffViewer, DiffMode, ChangeType, DiffColors};
use revue::layout::Rect;
use revue::render::Buffer;

// =========================================================================
// DiffMode enum trait tests
// =========================================================================

#[test]
fn test_diff_mode_default() {
    assert_eq!(DiffMode::default(), DiffMode::Split);
}

#[test]
fn test_diff_mode_clone() {
    let mode = DiffMode::Unified;
    assert_eq!(mode, mode.clone());
}

#[test]
fn test_diff_mode_copy() {
    let mode1 = DiffMode::Inline;
    let mode2 = mode1;
    assert_eq!(mode1, DiffMode::Inline);
    assert_eq!(mode2, DiffMode::Inline);
}

#[test]
fn test_diff_mode_equality() {
    assert_eq!(DiffMode::Split, DiffMode::Split);
    assert_eq!(DiffMode::Unified, DiffMode::Unified);
    assert_ne!(DiffMode::Split, DiffMode::Inline);
}

#[test]
fn test_diff_mode_debug() {
    let debug_str = format!("{:?}", DiffMode::Split);
    assert!(debug_str.contains("Split"));
}

// =========================================================================
// ChangeType enum trait tests
// =========================================================================

#[test]
fn test_change_type_clone() {
    let change = ChangeType::Added;
    assert_eq!(change, change.clone());
}

#[test]
fn test_change_type_copy() {
    let change1 = ChangeType::Removed;
    let change2 = change1;
    assert_eq!(change1, ChangeType::Removed);
    assert_eq!(change2, ChangeType::Removed);
}

#[test]
fn test_change_type_equality() {
    assert_eq!(ChangeType::Equal, ChangeType::Equal);
    assert_eq!(ChangeType::Added, ChangeType::Added);
    assert_ne!(ChangeType::Added, ChangeType::Removed);
}

#[test]
fn test_change_type_debug() {
    let debug_str = format!("{:?}", ChangeType::Modified);
    assert!(debug_str.contains("Modified"));
}

// =========================================================================
// DiffLine struct tests
// =========================================================================

#[test]
fn test_diff_line_clone() {
    let line = DiffLine {
        left_num: Some(1),
        right_num: Some(2),
        left: "hello".to_string(),
        right: "world".to_string(),
        change: ChangeType::Modified,
    };
    let cloned = line.clone();
    assert_eq!(line.left_num, cloned.left_num);
    assert_eq!(line.change, cloned.change);
}

#[test]
fn test_diff_line_debug() {
    let line = DiffLine {
        left_num: None,
        right_num: Some(1),
        left: String::new(),
        right: "new line".to_string(),
        change: ChangeType::Added,
    };
    let debug_str = format!("{:?}", line);
    assert!(debug_str.contains("DiffLine"));
}

// =========================================================================
// DiffColors struct tests
// =========================================================================

#[test]
fn test_diff_colors_default() {
    let colors = DiffColors::default();
    assert_eq!(colors.added_bg, revue::style::Color::rgb(30, 60, 30));
    assert_eq!(colors.removed_bg, revue::style::Color::rgb(60, 30, 30));
    assert_eq!(colors.modified_bg, revue::style::Color::rgb(60, 60, 30));
}

#[test]
fn test_diff_colors_clone() {
    let colors1 = DiffColors::default();
    let colors2 = colors1.clone();
    assert_eq!(colors1.added_bg, colors2.added_bg);
    assert_eq!(colors1.separator, colors2.separator);
}

#[test]
fn test_diff_colors_debug() {
    let colors = DiffColors::default();
    let debug_str = format!("{:?}", colors);
    assert!(debug_str.contains("DiffColors"));
}

#[test]
fn test_diff_colors_github() {
    let colors = DiffColors::github();
    assert_eq!(colors.added_bg, revue::style::Color::rgb(35, 134, 54));
    assert_eq!(colors.removed_bg, revue::style::Color::rgb(218, 54, 51));
}

// =========================================================================
// DiffViewer constructor tests
// =========================================================================

#[test]
fn test_diff_viewer_new() {
    let viewer = DiffViewer::new();
    assert_eq!(viewer.change_count(), 0);
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_viewer_default() {
    let viewer = DiffViewer::default();
    assert_eq!(viewer.change_count(), 0);
}

// =========================================================================
// Builder method tests
// =========================================================================

#[test]
fn test_left() {
    let viewer = DiffViewer::new().left("left content");
    // Verify builder compiles and returns self
    let _ = viewer.left("other");
}

#[test]
fn test_right() {
    let viewer = DiffViewer::new().right("right content");
    let _ = viewer.right("other");
}

#[test]
fn test_left_name() {
    let viewer = DiffViewer::new().left_name("original.txt");
    let _ = viewer.left_name("new.txt");
}

#[test]
fn test_right_name() {
    let viewer = DiffViewer::new().right_name("modified.txt");
    let _ = viewer.right_name("new.txt");
}

#[test]
fn test_compare() {
    let viewer = DiffViewer::new().compare("A\nB", "A\nC");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_mode_unified() {
    let viewer = DiffViewer::new().mode(DiffMode::Unified);
    let _ = viewer.mode(DiffMode::Split);
}

#[test]
fn test_mode_inline() {
    let viewer = DiffViewer::new().mode(DiffMode::Inline);
    let _ = viewer.mode(DiffMode::Split);
}

#[test]
fn test_mode_split() {
    let viewer = DiffViewer::new().mode(DiffMode::Split);
    let _ = viewer.mode(DiffMode::Unified);
}

#[test]
fn test_colors() {
    let custom_colors = DiffColors::github();
    let viewer = DiffViewer::new().colors(custom_colors.clone());
    let _ = viewer.colors(DiffColors::default());
}

#[test]
fn test_line_numbers() {
    let viewer = DiffViewer::new().line_numbers(false);
    let _ = viewer.line_numbers(true);
}

#[test]
fn test_context() {
    let viewer = DiffViewer::new().context(5);
    let _ = viewer.context(3);
}

// =========================================================================
// State-changing method tests
// =========================================================================

#[test]
fn test_set_scroll() {
    let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5");
    viewer.set_scroll(2);
    viewer.set_scroll(0);
    // Just verify methods work without panicking
}

#[test]
fn test_set_scroll_clamps() {
    let mut viewer = diff("1\n2", "1\n2");
    viewer.set_scroll(100);
    // Should clamp internally
}

#[test]
fn test_scroll_down() {
    let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5\n6");
    viewer.scroll_down(2);
    viewer.scroll_down(1);
    // Just verify it doesn't panic
}

#[test]
fn test_scroll_up() {
    let mut viewer = diff("1\n2\n3\n4\n5", "1\n2\n3\n4\n5\n6");
    viewer.scroll_down(5);
    viewer.scroll_up(2);
    viewer.scroll_up(10); // Should saturate
                          // Just verify it doesn't panic
}

// Helper function for tests
fn diff(left: impl Into<String>, right: impl Into<String>) -> DiffViewer {
    DiffViewer::new().compare(left, right)
}

// =========================================================================
// Getter method tests
// =========================================================================

#[test]
fn test_change_count() {
    let viewer = diff("A\nB\nC", "A\nX\nC");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_change_count_all_different() {
    let viewer = diff("A", "B");
    assert!(viewer.change_count() >= 1);
}

#[test]
fn test_change_count_identical() {
    let viewer = diff("Same\nContent", "Same\nContent");
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_line_count() {
    let viewer = diff("1\n2\n3", "1\n2\n3");
    assert!(viewer.line_count() > 0);
}

#[test]
fn test_line_count_empty() {
    let viewer = diff("", "");
    assert_eq!(viewer.line_count(), 0);
}

// =========================================================================
// Diff computation tests
// =========================================================================

#[test]
fn test_diff_addition() {
    let viewer = diff("Line 1\nLine 2", "Line 1\nLine 2\nNew Line");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_deletion() {
    let viewer = diff("Line 1\nLine 2\nLine 3", "Line 1\nLine 3");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_modification() {
    let viewer = diff("Hello World", "Hello Rust");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_empty_strings() {
    let viewer = diff("", "");
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_diff_one_empty() {
    let viewer = diff("", "content");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_multiline() {
    let viewer = diff(
        "Line 1\nLine 2\nLine 3\nLine 4",
        "Line 1\nChanged\nLine 3\nLine 4",
    );
    assert!(viewer.change_count() > 0);
}

// =========================================================================
// Render tests
// =========================================================================

#[test]
fn test_diff_render_split() {
    let viewer = diff("Original\nLine", "Modified\nLine");

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = revue::render::RenderContext::new(&mut buffer, area);

    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_unified() {
    let viewer = diff("A\nB", "A\nC").mode(DiffMode::Unified);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = revue::render::RenderContext::new(&mut buffer, area);

    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_inline() {
    let viewer = diff("test", "toast").mode(DiffMode::Inline);

    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = revue::render::RenderContext::new(&mut buffer, area);

    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_small_area() {
    let viewer = diff("A", "B");

    let mut buffer = Buffer::new(10, 5);
    let area = Rect::new(0, 0, 10, 5);
    let mut ctx = revue::render::RenderContext::new(&mut buffer, area);

    viewer.render(&mut ctx); // Should not panic
}

// =========================================================================
// Helper function tests
// =========================================================================

#[test]
fn test_diff_viewer_helper() {
    let viewer = revue::widget::diff::diff_viewer();
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_diff_helper() {
    let viewer = diff("left", "right");
    assert!(viewer.line_count() > 0);
}

// =========================================================================
// Builder chain tests
// =========================================================================

#[test]
fn test_builder_chain() {
    let colors = DiffColors::github();
    let _viewer = DiffViewer::new()
        .left("original")
        .right("modified")
        .left_name("old.txt")
        .right_name("new.txt")
        .mode(DiffMode::Unified)
        .colors(colors)
        .line_numbers(false)
        .context(2);
    // If it compiles, the chain works
}

// =========================================================================
// Edge case tests
// =========================================================================

#[test]
fn test_unicode_content() {
    let viewer = diff("Hello 世界", "Hello 世界!");
    assert!(viewer.line_count() > 0);
}

#[test]
fn test_long_lines() {
    let long_line = "A".repeat(1000);
    let viewer = diff(&long_line, &long_line);
    assert!(viewer.line_count() > 0);
}

#[test]
fn test_many_lines() {
    let left = (1..100)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let right = left.clone();
    let viewer = diff(&left, &right);
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_whitespace_changes() {
    let viewer = diff("Line 1\nLine 2", "Line 1  \nLine 2");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_trailing_newlines() {
    let viewer = diff("Line 1\n", "Line 1");
    assert!(viewer.line_count() > 0);
}

#[test]
fn test_only_newlines() {
    let viewer = diff("\n\n\n", "\n\n");
    assert!(viewer.line_count() > 0);
}