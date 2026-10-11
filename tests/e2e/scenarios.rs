//! End-to-end scenario tests
//!
//! These tests simulate real application patterns to verify
//! that widgets, styling, state, and rendering work together correctly.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::{RenderContext, Text, View};

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 1: Counter app (Signal + View + render cycle)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_counter_app_render_cycle() {
    use revue::prelude::*;

    struct CounterView {
        count: Signal<i32>,
    }

    impl CounterView {
        fn new() -> Self {
            Self { count: signal(0) }
        }
    }

    impl View for CounterView {
        fn render(&self, ctx: &mut RenderContext) {
            let text = format!("Count: {}", self.count.get());
            Text::new(text).render(ctx);
        }
        fn meta(&self) -> WidgetMeta {
            WidgetMeta::new("CounterView")
        }
    }

    let view = CounterView::new();

    // Initial render
    let mut buf = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    {
        let mut ctx = RenderContext::new(&mut buf, area);
        view.render(&mut ctx);
    }
    assert_eq!(buf.get(0, 0).unwrap().symbol, 'C');
    assert_eq!(buf.get(7, 0).unwrap().symbol, '0');

    // Update state
    view.count.set(42);

    // Re-render
    let mut buf2 = Buffer::new(40, 10);
    {
        let mut ctx = RenderContext::new(&mut buf2, area);
        view.render(&mut ctx);
    }
    // "Count: 42"
    assert_eq!(buf2.get(7, 0).unwrap().symbol, '4');
    assert_eq!(buf2.get(8, 0).unwrap().symbol, '2');
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 2: Nested layout (Stack > Border > Text)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_nested_layout_rendering() {
    use revue::widget::{hstack, vstack, Border};

    let layout = vstack()
        .gap(0)
        .child(
            hstack()
                .child(Border::single().child(Text::new("A")))
                .child(Border::single().child(Text::new("B"))),
        )
        .child(Text::new("Footer"));

    let mut buf = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    {
        let mut ctx = RenderContext::new(&mut buf, area);
        layout.render(&mut ctx);
    }
    // Border corners should be rendered
    assert_eq!(buf.get(0, 0).unwrap().symbol, '┌');
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 3: CSS styling (parse → apply → render with style)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_css_parse_and_apply() {
    let css = r#"
        :root {
            --primary: hsl(220, 90%, 56%);
        }
        .title {
            text-align: center;
            font-weight: bold;
            color: var(--primary);
        }
        .muted {
            color: slategray;
            opacity: 0.8;
        }
        .error {
            color: crimson;
            text-decoration: underline;
        }
    "#;

    let sheet = revue::style::parse_css(css).unwrap();
    let style = sheet.apply(".title", &revue::style::Style::default());

    assert_eq!(style.visual.text_align, revue::style::TextAlign::Center);
    assert_eq!(style.visual.font_weight, revue::style::FontWeight::Bold);
    // HSL(220, 90%, 56%) should produce a blue-ish color
    assert!(style.visual.color != revue::style::Color::default());

    let muted = sheet.apply(".muted", &revue::style::Style::default());
    assert_eq!(muted.visual.opacity, 0.8);
    // slategray = rgb(112, 128, 144)
    assert_eq!(muted.visual.color, revue::style::Color::rgb(112, 128, 144));

    let error = sheet.apply(".error", &revue::style::Style::default());
    assert!(error.visual.text_decoration.underline);
    // crimson = rgb(220, 20, 60)
    assert_eq!(error.visual.color, revue::style::Color::rgb(220, 20, 60));
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 4: CSS variables with fallback
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_css_variable_fallback_chain() {
    let css = r#"
        :root {
            --defined: orange;
        }
        .a { color: var(--defined); }
        .b { color: var(--undefined, teal); }
        .c { color: var(--undefined, var(--defined)); }
    "#;

    let sheet = revue::style::parse_css(css).unwrap();

    let a = sheet.apply(".a", &revue::style::Style::default());
    assert_eq!(a.visual.color, revue::style::Color::rgb(255, 165, 0)); // orange

    let b = sheet.apply(".b", &revue::style::Style::default());
    assert_eq!(b.visual.color, revue::style::Color::rgb(0, 128, 128)); // teal
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 5: CsvViewer full workflow
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_csv_viewer_full_workflow() {
    use revue::widget::CsvViewer;

    // Parse
    let csv = "Name,Age,City\nAlice,30,Seoul\nBob,25,Tokyo\nCharlie,35,Beijing";
    let mut viewer = CsvViewer::from_content(csv);

    // Navigate
    assert_eq!(viewer.selected_value(), Some("Alice"));
    viewer.select_down();
    assert_eq!(viewer.selected_value(), Some("Bob"));
    viewer.select_right();
    assert_eq!(viewer.get_cell(1, 1), Some("25"));

    // Sort
    viewer.sort_by(1); // Sort by Age ascending
    assert_eq!(
        viewer.sort_order,
        revue::widget::csv_viewer::SortOrder::Ascending
    );

    // Search
    viewer.search("tokyo");
    assert_eq!(viewer.match_count(), 1);

    // Render without panic
    let mut buf = Buffer::new(60, 20);
    let area = Rect::new(0, 0, 60, 20);
    {
        let mut ctx = RenderContext::new(&mut buf, area);
        viewer.render(&mut ctx);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 6: Tabs navigation
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_tabs_navigation_and_render() {
    use revue::widget::Tabs;

    let mut tabs = Tabs::new().tab("Home").tab("Settings").tab("About");

    assert_eq!(tabs.selected_label(), Some("Home"));
    tabs.select_next();
    assert_eq!(tabs.selected_label(), Some("Settings"));
    tabs.select_next();
    assert_eq!(tabs.selected_label(), Some("About"));
    tabs.select_next(); // wraps
    assert_eq!(tabs.selected_label(), Some("Home"));

    // Handle key
    tabs.handle_key(&revue::event::Key::End);
    assert_eq!(tabs.selected_label(), Some("About"));

    // Render
    let mut buf = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 3);
    {
        let mut ctx = RenderContext::new(&mut buf, area);
        tabs.render(&mut ctx);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 7: ScrollView paging
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_scroll_view_paging_workflow() {
    let mut scroll = revue::widget::scroll_view().content_height(100);

    // Page through content
    let viewport = 20u16;
    assert_eq!(scroll.offset(), 0);
    assert_eq!(scroll.scroll_percentage(viewport), 0.0);

    scroll.page_down(viewport);
    assert_eq!(scroll.offset(), 19); // viewport - 1

    scroll.page_down(viewport);
    assert_eq!(scroll.offset(), 38);

    scroll.scroll_to_bottom(viewport);
    assert_eq!(scroll.offset(), 80); // 100 - 20
    assert_eq!(scroll.scroll_percentage(viewport), 1.0);

    scroll.scroll_to_top();
    assert_eq!(scroll.offset(), 0);
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 8: Command palette search and execute
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_command_palette_workflow() {
    use revue::widget::{Command, CommandPalette};

    let mut palette = CommandPalette::new()
        .command(Command::new("save", "Save File").shortcut("Ctrl+S"))
        .command(Command::new("open", "Open File").shortcut("Ctrl+O"))
        .command(Command::new("quit", "Quit Application"));

    // Show and search
    palette.show();
    assert!(palette.is_visible());
    assert_eq!(palette.filtered.len(), 3);

    // Type to filter
    palette.set_query("save");
    assert_eq!(palette.selected_id(), Some("save"));

    // Execute
    let id = palette.execute();
    assert_eq!(id, Some("save".to_string()));
    assert!(!palette.is_visible()); // hidden after execute
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 9: Sortable list reorder
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_sortable_list_reorder_workflow() {
    use revue::widget::SortableList;

    let mut list = SortableList::new(vec!["First", "Second", "Third", "Fourth"]);

    // Select and move
    list.set_selected(Some(0));
    list.move_down();
    assert_eq!(list.items()[0].label, "Second");
    assert_eq!(list.items()[1].label, "First");
    assert_eq!(list.selected(), Some(1));

    // Move back
    list.move_up();
    assert_eq!(list.items()[0].label, "First");
    assert_eq!(list.selected(), Some(0));

    // Check order tracking
    assert_eq!(list.order(), vec![0, 1, 2, 3]);
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 10: Collapsible toggle and render
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_collapsible_toggle_render() {
    use revue::widget::Collapsible;

    let mut c = Collapsible::new("Details")
        .content("Line 1\nLine 2\nLine 3")
        .expanded(false);

    assert_eq!(c.height(), 1); // collapsed

    c.handle_key(&revue::event::Key::Enter);
    assert!(c.is_expanded());
    assert!(c.height() > 1);

    // Render expanded
    let mut buf = Buffer::new(30, 10);
    let area = Rect::new(0, 0, 30, 10);
    {
        let mut ctx = RenderContext::new(&mut buf, area);
        c.render(&mut ctx);
    }
    assert_eq!(buf.get(0, 0).unwrap().symbol, '▼'); // expanded icon

    // Collapse
    c.handle_key(&revue::event::Key::Enter);
    assert!(!c.is_expanded());

    let mut buf2 = Buffer::new(30, 10);
    {
        let mut ctx = RenderContext::new(&mut buf2, area);
        c.render(&mut ctx);
    }
    assert_eq!(buf2.get(0, 0).unwrap().symbol, '▶'); // collapsed icon
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 11: Modal with focus trap
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_modal_focus_trap_workflow() {
    use revue::event::{FocusManager, Key};
    use revue::widget::Modal;

    let mut fm = FocusManager::new();
    fm.register(1); // background
    fm.register(10); // modal btn 1
    fm.register(11); // modal btn 2
    fm.focus(1);

    let mut modal = Modal::new()
        .title("Confirm")
        .content("Are you sure?")
        .yes_no();

    // Show with trap
    modal.show_with_focus_trap(&mut fm, 100, &[10, 11]);
    assert!(modal.is_visible());
    assert!(fm.is_trapped());
    assert_eq!(fm.current(), Some(10));

    // Navigate buttons
    modal.handle_key(&Key::Right);
    assert_eq!(modal.selected_button(), 1);

    // Confirm and release
    let result = modal.handle_key_with_focus(&Key::Enter, &mut fm);
    assert_eq!(result, Some(1)); // "No" button
    assert!(!fm.is_trapped());
    assert_eq!(fm.current(), Some(1)); // restored
}

// ─────────────────────────────────────────────────────────────────────────────
// Scenario 12: Full render pipeline (buffer → diff)
// ─────────────────────────────────────────────────────────────────────────────

#[test]
fn test_buffer_diff_pipeline() {
    use revue::render::{diff, Cell};

    let old = Buffer::new(20, 5);
    let mut new = Buffer::new(20, 5);

    // Make some changes
    new.set(5, 2, Cell::new('X'));
    new.set(6, 2, Cell::new('Y'));

    let rect = Rect::new(0, 0, 20, 5);
    let changes = diff(&old, &new, &[rect]);

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].x, 5);
    assert_eq!(changes[0].y, 2);
    assert_eq!(changes[0].cell.symbol, 'X');
}
