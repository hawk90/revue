//! Stress tests for large-scale data and edge cases
//!
//! These tests verify that revue handles large datasets, deep nesting,
//! and extreme inputs without panicking or excessive memory usage.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{RenderContext, Text, View};

// ─────────────────────────────────────────────────────────────────────────────
// Large buffer stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_large_buffer_creation() {
    // 500x200 = 100,000 cells - reasonable large terminal
    let buf = Buffer::new(500, 200);
    assert_eq!(buf.width(), 500);
    assert_eq!(buf.height(), 200);
}

#[test]
fn test_buffer_fill_large_area() {
    let mut buf = Buffer::new(200, 100);
    buf.fill(0, 0, 200, 100, revue::render::Cell::new('X'));
    assert_eq!(buf.get(199, 99).unwrap().symbol, 'X');
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'X');
}

// ─────────────────────────────────────────────────────────────────────────────
// CSS parsing stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_css_many_rules() {
    // Generate 500 CSS rules
    let mut css = String::new();
    for i in 0..500 {
        css.push_str(&format!(
            ".class-{} {{ color: rgb({}, {}, {}); padding: {}; }}\n",
            i,
            i % 256,
            (i * 3) % 256,
            (i * 7) % 256,
            i % 20
        ));
    }
    let sheet = revue::style::parse_css(&css).unwrap();
    assert_eq!(sheet.rules.len(), 500);
}

#[test]
fn test_css_long_selector_chain() {
    // Deep descendant selector: .a .b .c .d .e .f .g .h
    let css = ".a .b .c .d .e .f .g .h { color: red; }";
    let sheet = revue::style::parse_css(css).unwrap();
    assert_eq!(sheet.rules.len(), 1);
}

#[test]
fn test_css_many_declarations() {
    // Single rule with many declarations
    let mut css = String::from(".heavy {\n");
    for i in 0..100 {
        css.push_str(&format!("  padding: {};\n", i));
    }
    css.push_str("}\n");
    let sheet = revue::style::parse_css(&css).unwrap();
    assert_eq!(sheet.rules[0].declarations.len(), 100);
}

#[test]
fn test_css_many_variables() {
    let mut css = String::from(":root {\n");
    for i in 0..100 {
        css.push_str(&format!(
            "  --color-{}: rgb({}, {}, {});\n",
            i,
            i,
            i * 2,
            i * 3
        ));
    }
    css.push_str("}\n");
    let sheet = revue::style::parse_css(&css).unwrap();
    assert_eq!(sheet.variables.len(), 100);
}

// ─────────────────────────────────────────────────────────────────────────────
// Widget rendering stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_render_many_text_widgets() {
    let mut buf = Buffer::new(80, 24);

    // Render 100 text widgets sequentially
    for i in 0..100 {
        let text = Text::new(format!("Line {}", i));
        let row_area = Rect::new(0, (i % 24) as u16, 80, 1);
        let mut ctx = RenderContext::new(&mut buf, row_area);
        text.render(&mut ctx);
    }
    // Should not panic
}

#[test]
fn test_deep_nested_stack() {
    use revue::widget::vstack;

    // Build deeply nested stack (10 levels)
    let mut widget: Box<dyn View> = Box::new(Text::new("Leaf"));
    for _ in 0..10 {
        widget = Box::new(vstack().child(widget));
    }

    let mut buf = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buf, area);
    widget.render(&mut ctx);
    // Should not stack overflow
}

// ─────────────────────────────────────────────────────────────────────────────
// Data widget stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_csv_viewer_large_dataset() {
    use revue::widget::CsvViewer;

    // Generate 1000-row CSV
    let mut csv = String::from("Name,Age,City,Score\n");
    for i in 0..1000 {
        csv.push_str(&format!(
            "User{},{},City{},{}\n",
            i,
            20 + i % 50,
            i % 100,
            i * 7 % 100
        ));
    }

    let viewer = CsvViewer::from_content(&csv);
    assert_eq!(viewer.row_count(), 1000);
    assert_eq!(viewer.column_count(), 4);
    assert_eq!(viewer.get_cell(999, 0), Some("User999"));
}

#[test]
fn test_csv_viewer_many_columns() {
    use revue::widget::CsvViewer;

    // 50 columns
    let header: Vec<String> = (0..50).map(|i| format!("Col{}", i)).collect();
    let row: Vec<String> = (0..50).map(|i| format!("Val{}", i)).collect();
    let csv = format!("{}\n{}", header.join(","), row.join(","));

    let viewer = CsvViewer::from_content(&csv);
    assert_eq!(viewer.column_count(), 50);
}

#[test]
fn test_csv_viewer_search_large() {
    use revue::widget::CsvViewer;

    let mut csv = String::from("Name,Value\n");
    for i in 0..500 {
        csv.push_str(&format!(
            "item{},{}\n",
            i,
            if i == 42 { "target" } else { "other" }
        ));
    }

    let mut viewer = CsvViewer::from_content(&csv);
    viewer.search("target");
    assert_eq!(viewer.match_count(), 1);
}

// ─────────────────────────────────────────────────────────────────────────────
// Scroll stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_scroll_view_large_content() {
    let mut scroll = revue::widget::scroll_view().content_height(10000);
    scroll.scroll_to_bottom(24);
    assert_eq!(scroll.offset(), 10000 - 24);

    scroll.scroll_to_top();
    assert_eq!(scroll.offset(), 0);

    // Rapid scrolling
    for _ in 0..1000 {
        scroll.scroll_down(10, 24);
    }
    assert!(scroll.offset() <= 10000 - 24);
}

// ─────────────────────────────────────────────────────────────────────────────
// Command palette stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_command_palette_many_commands() {
    use revue::widget::{Command, CommandPalette};

    let mut palette = CommandPalette::new();
    for i in 0..500 {
        palette.add_command(Command::new(
            format!("cmd_{}", i),
            format!("Command Number {}", i),
        ));
    }

    palette.show();
    assert_eq!(palette.filtered.len(), 500);

    // Filter should work efficiently
    palette.set_query("42");
    assert!(palette.filtered.len() < 500);
    assert!(palette.filtered.len() >= 1); // "Command Number 42" etc.
}

// ─────────────────────────────────────────────────────────────────────────────
// Sortable list stress tests
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_sortable_list_many_items() {
    use revue::widget::SortableList;

    let items: Vec<String> = (0..1000).map(|i| format!("Item {}", i)).collect();
    let mut list = SortableList::new(items);
    assert_eq!(list.items().len(), 1000);

    // Navigate to end
    for _ in 0..1000 {
        list.select_next();
    }
    assert_eq!(list.selected(), Some(999));

    // Move last item up repeatedly
    for _ in 0..10 {
        list.move_up();
    }
    assert_eq!(list.selected(), Some(989));
}

// ─────────────────────────────────────────────────────────────────────────────
// Edge case: empty/zero inputs
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_empty_css() {
    let sheet = revue::style::parse_css("").unwrap();
    assert!(sheet.rules.is_empty());
}

#[test]
fn test_whitespace_only_css() {
    let sheet = revue::style::parse_css("   \n\n\t  ").unwrap();
    assert!(sheet.rules.is_empty());
}

#[test]
fn test_render_zero_area_no_panic() {
    let mut buf = Buffer::new(1, 1);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buf, area);
    let text = Text::new("Should not render");
    text.render(&mut ctx);
}
