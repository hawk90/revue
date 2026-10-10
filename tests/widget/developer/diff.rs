//! Diff Viewer widget integration tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{diff, diff_viewer, DiffColors, DiffMode, DiffViewer, View};

// ─────────────────────────────────────────────────────────────────────────
// Constructor Tests (생성자 테스트)
// ─────────────────────────────────────────────────────────────────────────

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
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_viewer_helper() {
    let viewer = diff_viewer();
    assert_eq!(viewer.change_count(), 0);
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_helper_function() {
    let viewer = diff("Hello", "World");
    assert!(viewer.change_count() > 0);
}

// ─────────────────────────────────────────────────────────────────────────
// Builder Method Tests (빌더 메서드 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_left_builder() {
    let viewer = DiffViewer::new().left("Original content");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_right_builder() {
    let viewer = DiffViewer::new().right("Modified content");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_compare_builder() {
    let viewer = DiffViewer::new().compare("Left", "Right");
    assert!(viewer.line_count() > 0);
}

#[test]
fn test_diff_left_name_builder() {
    let viewer = DiffViewer::new()
        .left("A")
        .right("B")
        .left_name("Original.txt");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_right_name_builder() {
    let viewer = DiffViewer::new()
        .left("A")
        .right("B")
        .right_name("Modified.txt");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_both_names_builder() {
    let viewer = DiffViewer::new()
        .left("A")
        .right("B")
        .left_name("old.rs")
        .right_name("new.rs");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_mode_split() {
    let viewer = DiffViewer::new().mode(DiffMode::Split);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_mode_unified() {
    let viewer = DiffViewer::new().mode(DiffMode::Unified);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_mode_inline() {
    let viewer = DiffViewer::new().mode(DiffMode::Inline);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_line_numbers_show() {
    let viewer = DiffViewer::new()
        .left("A\nB")
        .right("A\nC")
        .line_numbers(true);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_line_numbers_hide() {
    let viewer = DiffViewer::new()
        .left("A\nB")
        .right("A\nC")
        .line_numbers(false);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_context_builder() {
    let viewer = DiffViewer::new()
        .left("A\nB\nC\nD\nE")
        .right("A\nX\nC\nD\nE")
        .context(2);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_colors_custom() {
    let colors = DiffColors {
        added_bg: Color::rgb(0, 50, 0),
        added_fg: Color::rgb(100, 255, 100),
        removed_bg: Color::rgb(50, 0, 0),
        removed_fg: Color::rgb(255, 100, 100),
        modified_bg: Color::rgb(50, 50, 0),
        line_number: Color::rgb(80, 80, 80),
        separator: Color::rgb(40, 40, 40),
        header_bg: Color::rgb(30, 30, 50),
    };
    let viewer = DiffViewer::new().left("A").right("B").colors(colors);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_colors_github() {
    let colors = DiffColors::github();
    let viewer = DiffViewer::new().left("A").right("B").colors(colors);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Content Comparison Tests (내용 비교 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_equal_content() {
    let viewer = diff("Same content", "Same content");
    assert_eq!(viewer.change_count(), 0);
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_diff_empty_both() {
    let viewer = diff("", "");
    assert_eq!(viewer.change_count(), 0);
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_empty_left() {
    let viewer = diff("", "Added content");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_empty_right() {
    let viewer = diff("Removed content", "");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_single_line_addition() {
    let viewer = diff("Line 1", "Line 1\nLine 2");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_single_line_removal() {
    let viewer = diff("Line 1\nLine 2", "Line 1");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_single_line_modification() {
    let viewer = diff("Hello World", "Hello Rust");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_multiple_lines_addition() {
    let viewer = diff("Line 1\nLine 2", "Line 1\nLine 2\nLine 3\nLine 4");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_multiple_lines_removal() {
    let viewer = diff("Line 1\nLine 2\nLine 3\nLine 4", "Line 1\nLine 2");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_multiple_lines_mixed() {
    let viewer = diff(
        "Line 1\nLine 2\nLine 3\nLine 4",
        "Line 1\nModified\nLine 3\nLine 5",
    );
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_multiline_content() {
    let left = "fn main() {\n    println!(\"Hello\");\n}";
    let right = "fn main() {\n    println!(\"World\");\n}";
    let viewer = diff(left, right);
    assert!(viewer.change_count() > 0);
    // Line 1 is equal (1 line), line 2 creates 2 diff lines (delete + insert),
    // line 3 is equal (1 line) = total 4 lines in diff
    assert_eq!(viewer.line_count(), 4);
}

#[test]
fn test_diff_large_content() {
    let left = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
    let right = "Line 1\nLine 2\nModified\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10";
    let viewer = diff(left, right);
    assert!(viewer.change_count() > 0);
    // Lines 1-2 equal (2), line 3 creates 2 diff lines (delete + insert),
    // lines 4-10 equal (7) = total 11 lines in diff
    assert_eq!(viewer.line_count(), 11);
}

#[test]
fn test_diff_unicode_content() {
    let viewer = diff("Hello 世界", "Hello 🌍");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_whitespace_changes() {
    let viewer = diff("Hello  World", "Hello World");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_trailing_newlines() {
    let viewer = diff("Line 1\nLine 2\n", "Line 1\nLine 2");
    assert!(viewer.change_count() > 0);
}

// ─────────────────────────────────────────────────────────────────────────
// Rendering Tests (렌더링 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_render_split_mode() {
    let viewer = diff("Original\nContent", "Modified\nContent").mode(DiffMode::Split);
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_unified_mode() {
    let viewer = diff("Original\nContent", "Modified\nContent").mode(DiffMode::Unified);
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_inline_mode() {
    let viewer = diff("Original\nContent", "Modified\nContent").mode(DiffMode::Inline);
    let mut buffer = Buffer::new(80, 20);
    let area = Rect::new(0, 0, 80, 20);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_small_area() {
    let viewer = diff("A\nB", "A\nC");
    let mut buffer = Buffer::new(20, 5);
    let area = Rect::new(0, 0, 20, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_large_area() {
    let viewer = diff("A\nB", "A\nC");
    let mut buffer = Buffer::new(120, 40);
    let area = Rect::new(0, 0, 120, 40);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_with_line_numbers() {
    let viewer = diff("Line 1\nLine 2\nLine 3", "Line 1\nLine 2\nLine 3").line_numbers(true);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_without_line_numbers() {
    let viewer = diff("Line 1\nLine 2\nLine 3", "Line 1\nLine 2\nLine 3").line_numbers(false);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_with_headers() {
    let viewer = diff("A\nB", "A\nC")
        .left_name("original.txt")
        .right_name("modified.txt");
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_minimal_area() {
    let viewer = diff("A", "B");
    let mut buffer = Buffer::new(10, 3);
    let area = Rect::new(0, 0, 10, 3);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_empty_viewer() {
    let viewer = DiffViewer::new();
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Scrolling Tests (스크롤 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_scroll_down() {
    let mut viewer = diff(
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6",
    );
    viewer.scroll_down(2);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_scroll_up() {
    let mut viewer = diff(
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6",
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
    );
    viewer.scroll_down(3);
    viewer.scroll_up(1);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_scroll_boundaries() {
    let mut viewer = diff("A\nB\nC", "A\nB\nC");

    // Scroll beyond end should clamp
    viewer.scroll_down(100);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);

    // Scroll up from 0 should stay at 0
    viewer.scroll_up(10);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_set_scroll() {
    let mut viewer = diff(
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
    );
    viewer.set_scroll(2);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_scroll_with_scrolling_content() {
    let content = (1..=50)
        .map(|i| format!("Line {}", i))
        .collect::<Vec<_>>()
        .join("\n");
    let mut viewer = diff(&content, &content);
    viewer.scroll_down(10);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Edge Cases Tests (엣지 케이스 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_identical_multiline() {
    let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
    let viewer = diff(content, content);
    assert_eq!(viewer.change_count(), 0);
    assert_eq!(viewer.line_count(), 5);
}

#[test]
fn test_diff_completely_different() {
    let viewer = diff("AAAA\nBBBB", "CCCC\nDDDD");
    // All lines should be marked as changes
    assert!(viewer.change_count() >= 4);
}

#[test]
fn test_diff_only_one_difference() {
    let viewer = diff(
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
        "Line 1\nLine 2\nCHANGED\nLine 4\nLine 5",
    );
    assert!(viewer.change_count() >= 1);
}

#[test]
fn test_diff_very_long_lines() {
    let left = "A".repeat(200);
    let right = "B".repeat(200);
    let viewer = diff(&left, &right);
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_many_empty_lines() {
    let left = "\n\n\n\n\n";
    let right = "\n\n\n\n\n";
    let viewer = diff(left, right);
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_diff_mixed_empty_and_content() {
    let viewer = diff("\n\nContent\n\n", "\n\nDifferent\n\n");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_single_character() {
    let viewer = diff("A", "B");
    assert!(viewer.change_count() > 0);
    // "A" deleted (1 line), "B" inserted (1 line) = total 2 lines in diff
    assert_eq!(viewer.line_count(), 2);
}

#[test]
fn test_diff_newlines_only() {
    let viewer = diff("\n\n\n", "\n\n\n");
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_diff_carriage_return() {
    let viewer = diff("Line 1\r\nLine 2", "Line 1\r\nLine 2");
    assert_eq!(viewer.change_count(), 0);
}

#[test]
fn test_diff_tabs_in_content() {
    let viewer = diff("Line\t1\tIndented", "Line\t2\tIndented");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_special_characters() {
    let viewer = diff("!@#$%^&*()", "()(*&^%$#@!");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_ordering_difference() {
    let viewer = diff("Line 1\nLine 2\nLine 3", "Line 3\nLine 2\nLine 1");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_case_sensitive() {
    let viewer = diff("Hello", "hello");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_repeated_content() {
    let viewer = diff("A\nA\nA\nA", "A\nA\nB\nA");
    assert!(viewer.change_count() > 0);
}

// ─────────────────────────────────────────────────────────────────────────
// DiffColors Tests (DiffColors 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_colors_default() {
    let colors = DiffColors::default();
    // Check that default colors are set
    assert_eq!(colors.added_bg, Color::rgb(30, 60, 30));
    assert_eq!(colors.added_fg, Color::rgb(150, 255, 150));
    assert_eq!(colors.removed_bg, Color::rgb(60, 30, 30));
    assert_eq!(colors.removed_fg, Color::rgb(255, 150, 150));
    assert_eq!(colors.modified_bg, Color::rgb(60, 60, 30));
    assert_eq!(colors.line_number, Color::rgb(100, 100, 100));
    assert_eq!(colors.separator, Color::rgb(60, 60, 60));
    assert_eq!(colors.header_bg, Color::rgb(40, 40, 60));
}

#[test]
fn test_diff_colors_github_values() {
    let colors = DiffColors::github();
    assert_eq!(colors.added_bg, Color::rgb(35, 134, 54));
    assert_eq!(colors.added_fg, Color::WHITE);
    assert_eq!(colors.removed_bg, Color::rgb(218, 54, 51));
    assert_eq!(colors.removed_fg, Color::WHITE);
    assert_eq!(colors.modified_bg, Color::rgb(210, 153, 34));
    assert_eq!(colors.line_number, Color::rgb(140, 140, 140));
    assert_eq!(colors.separator, Color::rgb(48, 54, 61));
    assert_eq!(colors.header_bg, Color::rgb(22, 27, 34));
}

#[test]
fn test_diff_mode_default() {
    let mode = DiffMode::default();
    assert_eq!(mode, DiffMode::Split);
}

#[test]
fn test_diff_mode_equality() {
    assert_eq!(DiffMode::Split, DiffMode::Split);
    assert_eq!(DiffMode::Unified, DiffMode::Unified);
    assert_eq!(DiffMode::Inline, DiffMode::Inline);
}

#[test]
fn test_diff_mode_inequality() {
    assert_ne!(DiffMode::Split, DiffMode::Unified);
    assert_ne!(DiffMode::Unified, DiffMode::Inline);
    assert_ne!(DiffMode::Split, DiffMode::Inline);
}

// ─────────────────────────────────────────────────────────────────────────
// Rendering with Custom Colors Tests (사용자 정의 색상 렌더링 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_render_github_theme() {
    let viewer = diff("Original", "Modified")
        .colors(DiffColors::github())
        .mode(DiffMode::Split);
    let mut buffer = Buffer::new(60, 10);
    let area = Rect::new(0, 0, 60, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_render_high_contrast_theme() {
    let colors = DiffColors {
        added_bg: Color::BLACK,
        added_fg: Color::GREEN,
        removed_bg: Color::BLACK,
        removed_fg: Color::RED,
        modified_bg: Color::BLACK,
        line_number: Color::YELLOW,
        separator: Color::WHITE,
        header_bg: Color::BLUE,
    };
    let viewer = diff("A\nB", "A\nC").colors(colors);
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Line Count and Change Count Tests (줄 수 및 변경 수 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_line_count_empty() {
    let viewer = diff("", "");
    assert_eq!(viewer.line_count(), 0);
}

#[test]
fn test_diff_line_count_single() {
    let viewer = diff("Single line", "Single line");
    assert_eq!(viewer.line_count(), 1);
}

#[test]
fn test_diff_line_count_multiple() {
    let viewer = diff("A\nB\nC", "A\nB\nC");
    assert_eq!(viewer.line_count(), 3);
}

#[test]
fn test_diff_change_count_calculation() {
    let viewer = diff("A\nB\nC", "A\nX\nC");
    // Should detect at least one change (line 2)
    assert!(viewer.change_count() >= 1);
}

#[test]
fn test_diff_change_count_with_additions() {
    let viewer = diff("A\nB", "A\nB\nC\nD");
    // Should detect additions
    assert!(viewer.change_count() >= 2);
}

#[test]
fn test_diff_change_count_with_removals() {
    let viewer = diff("A\nB\nC\nD", "A\nB");
    // Should detect removals
    assert!(viewer.change_count() >= 2);
}

// ─────────────────────────────────────────────────────────────────────────
// Code-like Diff Tests (코드와 유사한 diff 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_rust_code() {
    let left = r#"
fn main() {
    let x = 5;
    println!("Hello");
}
"#;

    let right = r#"
fn main() {
    let x = 10;
    println!("World");
}
"#;

    let viewer = diff(left, right);
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_json_content() {
    let left = r#"{"name": "old", "value": 1}"#;
    let right = r#"{"name": "new", "value": 2}"#;
    let viewer = diff(left, right);
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_list_like_content() {
    let left = "- Item 1\n- Item 2\n- Item 3";
    let right = "- Item 1\n- Item 2\n- Item 4";
    let viewer = diff(left, right);
    assert!(viewer.change_count() > 0);
}

// ─────────────────────────────────────────────────────────────────────────
// Performance and Stress Tests (성능 및 스트레스 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_large_content_performance() {
    let left: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
    let right: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
    let viewer = diff(left.join("\n"), right.join("\n"));
    assert_eq!(viewer.change_count(), 0);
    assert_eq!(viewer.line_count(), 100);
}

#[test]
fn test_diff_large_content_with_changes() {
    let left: Vec<String> = (1..=100).map(|i| format!("Line {}", i)).collect();
    let right: Vec<String> = (1..=100)
        .map(|i| {
            if i == 50 {
                "MODIFIED".to_string()
            } else {
                format!("Line {}", i)
            }
        })
        .collect();
    let viewer = diff(left.join("\n"), right.join("\n"));
    assert!(viewer.change_count() >= 1);
    // Lines 1-49 equal (49), line 50 creates 2 diff lines (delete + insert),
    // lines 51-100 equal (50) = total 101 lines in diff
    assert_eq!(viewer.line_count(), 101);
}

// ─────────────────────────────────────────────────────────────────────────
// Chaining Builder Methods Tests (체이닝 빌더 메서드 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_chained_builders() {
    let viewer = DiffViewer::new()
        .left("Original content\nLine 2")
        .right("Modified content\nLine 2")
        .left_name("old.txt")
        .right_name("new.txt")
        .mode(DiffMode::Split)
        .line_numbers(true)
        .context(3)
        .colors(DiffColors::github());

    let mut buffer = Buffer::new(60, 15);
    let area = Rect::new(0, 0, 60, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

#[test]
fn test_diff_chained_with_compare() {
    let viewer = DiffViewer::new()
        .compare("A\nB\nC", "A\nX\nC")
        .mode(DiffMode::Unified)
        .line_numbers(false);

    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    viewer.render(&mut ctx);
}

// ─────────────────────────────────────────────────────────────────────────
// Multi-byte and UTF-8 Tests (멀티바이트 및 UTF-8 테스트)
// ─────────────────────────────────────────────────────────────────────────

#[test]
fn test_diff_emoji_content() {
    let viewer = diff("Hello 👋\nWorld 🌍", "Hello 🙏\nWorld 🌎");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_chinese_characters() {
    let viewer = diff("你好\n世界", "你好\n中国");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_korean_characters() {
    let viewer = diff("안녕하세요\n세계", "안녕하세요\n한국");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_japanese_characters() {
    let viewer = diff("こんにちは\n世界", "こんにちは\n日本");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_arabic_characters() {
    let viewer = diff("مرحبا\nالعالم", "مرحبا\nالسلام");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_cyrillic_characters() {
    let viewer = diff("Привет\nМир", "Привет\nРоссия");
    assert!(viewer.change_count() > 0);
}

#[test]
fn test_diff_mixed_scripts() {
    let viewer = diff("Hello 你好\nWorld", "Hello 안녕\nWorld");
    assert!(viewer.change_count() > 0);
}
