//! Breadcrumb core tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::breadcrumb::{breadcrumb, Breadcrumb, BreadcrumbItem, SeparatorStyle};

// =============================================================================
// Breadcrumb Constructor Tests
// =============================================================================

#[test]
fn test_breadcrumb_new() {
    let bc = Breadcrumb::new();
    assert!(bc.is_empty());
    assert_eq!(bc.len(), 0);
    assert_eq!(bc.selected(), 0);
    assert!(bc.show_home);
    assert!(bc.collapse);
}

#[test]
fn test_breadcrumb_default() {
    let bc = Breadcrumb::default();
    assert!(bc.is_empty());
}

// =============================================================================
// Breadcrumb Builder Methods Tests - Item Management
// =============================================================================

#[test]
fn test_breadcrumb_item_single() {
    let bc = Breadcrumb::new().item(BreadcrumbItem::new("Home"));
    assert_eq!(bc.len(), 1);
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_item_multiple() {
    let bc = Breadcrumb::new()
        .item(BreadcrumbItem::new("Home"))
        .item(BreadcrumbItem::new("Documents"))
        .item(BreadcrumbItem::new("Work"));

    assert_eq!(bc.len(), 3);
    assert_eq!(bc.selected(), 2);
}

#[test]
fn test_breadcrumb_item_with_icon() {
    let bc = Breadcrumb::new().item(BreadcrumbItem::new("Home").icon('🏠'));
    assert_eq!(bc.items()[0].icon, Some('🏠'));
}

#[test]
fn test_breadcrumb_item_not_clickable() {
    let bc = Breadcrumb::new().item(BreadcrumbItem::new("Locked").clickable(false));
    assert!(!bc.items()[0].clickable);
}

#[test]
fn test_breadcrumb_push_single() {
    let bc = Breadcrumb::new().push("Home");
    assert_eq!(bc.len(), 1);
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_push_multiple() {
    let bc = Breadcrumb::new()
        .push("Home")
        .push("Documents")
        .push("Work");

    assert_eq!(bc.len(), 3);
    assert_eq!(bc.selected(), 2);
}

#[test]
fn test_breadcrumb_push_from_string() {
    let bc = Breadcrumb::new().push(String::from("Test"));
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_path_basic() {
    let bc = Breadcrumb::new().path("/home/user/documents");
    assert_eq!(bc.len(), 3);
    assert_eq!(bc.path_string(), "home/user/documents");
}

#[test]
fn test_breadcrumb_path_with_leading_slash() {
    let bc = Breadcrumb::new().path("home/user/documents");
    assert_eq!(bc.len(), 3);
}

#[test]
fn test_breadcrumb_path_with_empty_segments() {
    let bc = Breadcrumb::new().path("//home//user//documents//");
    assert_eq!(bc.len(), 3);
}

#[test]
fn test_breadcrumb_path_empty() {
    let bc = Breadcrumb::new().path("");
    assert!(bc.is_empty());
}

#[test]
fn test_breadcrumb_path_single_segment() {
    let bc = Breadcrumb::new().path("home");
    assert_eq!(bc.len(), 1);
}

// =============================================================================
// Breadcrumb Builder Methods Tests - Configuration
// =============================================================================

#[test]
fn test_breadcrumb_separator() {
    let bc = Breadcrumb::new()
        .separator(SeparatorStyle::Arrow)
        .push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_item_color() {
    let bc = Breadcrumb::new().item_color(Color::RED).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_selected_color() {
    let bc = Breadcrumb::new().selected_color(Color::CYAN).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_separator_color() {
    let bc = Breadcrumb::new()
        .separator_color(Color::rgb(128, 128, 128))
        .push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_home_enabled() {
    let bc = Breadcrumb::new().home(true).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_home_disabled() {
    let bc = Breadcrumb::new().home(false).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_home_icon() {
    let bc = Breadcrumb::new().home_icon('🏡').home(true).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_max_width() {
    let bc = Breadcrumb::new().max_width(50).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_max_width_zero() {
    let bc = Breadcrumb::new().max_width(0).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_collapse_enabled() {
    let bc = Breadcrumb::new().collapse(true).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_collapse_disabled() {
    let bc = Breadcrumb::new().collapse(false).push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_full_builder_chain() {
    let bc = Breadcrumb::new()
        .separator(SeparatorStyle::Chevron)
        .item_color(Color::rgb(150, 150, 150))
        .selected_color(Color::CYAN)
        .separator_color(Color::rgb(80, 80, 80))
        .home(true)
        .home_icon('🏠')
        .max_width(100)
        .collapse(true)
        .push("Home")
        .push("Documents")
        .push("Work");

    assert_eq!(bc.len(), 3);
    assert_eq!(bc.selected(), 2);
}

// =============================================================================
// Navigation Tests - Selection Management
// =============================================================================

#[test]
fn test_breadcrumb_select_next() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.set_selected(0);
    assert_eq!(bc.selected(), 0);

    bc.select_next();
    assert_eq!(bc.selected(), 1);

    bc.select_next();
    assert_eq!(bc.selected(), 2);
}

#[test]
fn test_breadcrumb_select_next_at_end() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(1);
    bc.select_next();
    assert_eq!(bc.selected(), 1); // Can't go past end
}

#[test]
fn test_breadcrumb_select_prev() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.set_selected(2);
    assert_eq!(bc.selected(), 2);

    bc.select_prev();
    assert_eq!(bc.selected(), 1);

    bc.select_prev();
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_select_prev_at_start() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(0);
    bc.select_prev();
    assert_eq!(bc.selected(), 0); // Can't go below 0
}

#[test]
fn test_breadcrumb_set_selected() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.set_selected(1);
    assert_eq!(bc.selected(), 1);

    bc.set_selected(0);
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_set_selected_last() {
    let bc = Breadcrumb::new().push("A").push("B").push("C");
    assert_eq!(bc.selected(), 2); // Default selects last
}

#[test]
fn test_breadcrumb_navigation_cycle() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    // Start at 0
    bc.set_selected(0);

    // Go forward
    bc.select_next();
    assert_eq!(bc.selected(), 1);

    // Go back
    bc.select_prev();
    assert_eq!(bc.selected(), 0);

    // Go forward twice
    bc.select_next();
    bc.select_next();
    assert_eq!(bc.selected(), 2);

    // Try to go past end
    bc.select_next();
    assert_eq!(bc.selected(), 2);

    // Go back to start
    bc.select_prev();
    bc.select_prev();
    assert_eq!(bc.selected(), 0);
}

// =============================================================================
// Query Methods Tests
// =============================================================================

#[test]
fn test_breadcrumb_selected_item() {
    let bc = Breadcrumb::new().push("First").push("Second").push("Third");

    let mut bc = bc;
    bc.set_selected(1);
    let item = bc.selected_item();
    assert!(item.is_some());
    assert_eq!(item.unwrap().label, "Second");
}

#[test]
fn test_breadcrumb_selected_item_first() {
    let mut bc = Breadcrumb::new().push("Home").push("Docs");

    bc.set_selected(0);
    let item = bc.selected_item();
    assert_eq!(item.unwrap().label, "Home");
}

#[test]
fn test_breadcrumb_selected_item_last() {
    let bc = Breadcrumb::new().push("Home").push("Docs");

    let item = bc.selected_item();
    assert_eq!(item.unwrap().label, "Docs");
}

#[test]
fn test_breadcrumb_selected_item_empty() {
    let bc = Breadcrumb::new();
    let item = bc.selected_item();
    assert!(item.is_none());
}

#[test]
fn test_breadcrumb_len_empty() {
    let bc = Breadcrumb::new();
    assert_eq!(bc.len(), 0);
}

#[test]
fn test_breadcrumb_len_single() {
    let bc = Breadcrumb::new().push("Test");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_len_multiple() {
    let bc = Breadcrumb::new().push("A").push("B").push("C").push("D");
    assert_eq!(bc.len(), 4);
}

#[test]
fn test_breadcrumb_is_empty_true() {
    let bc = Breadcrumb::new();
    assert!(bc.is_empty());
}

#[test]
fn test_breadcrumb_is_empty_false() {
    let bc = Breadcrumb::new().push("Test");
    assert!(!bc.is_empty());
}

#[test]
fn test_breadcrumb_path_string_empty() {
    let bc = Breadcrumb::new();
    assert_eq!(bc.path_string(), "");
}

#[test]
fn test_breadcrumb_path_string_single() {
    let bc = Breadcrumb::new().push("Home");
    assert_eq!(bc.path_string(), "Home");
}

#[test]
fn test_breadcrumb_path_string_multiple() {
    let bc = Breadcrumb::new()
        .push("home")
        .push("user")
        .push("documents");

    assert_eq!(bc.path_string(), "home/user/documents");
}

// =============================================================================
// Keyboard Handling Tests
// =============================================================================

#[test]
fn test_breadcrumb_handle_key_left() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(1);
    assert!(bc.handle_key(&Key::Left));
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_handle_key_right() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(0);
    assert!(bc.handle_key(&Key::Right));
    assert_eq!(bc.selected(), 1);
}

#[test]
fn test_breadcrumb_handle_key_char_h() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(1);
    assert!(bc.handle_key(&Key::Char('h')));
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_handle_key_char_l() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(0);
    assert!(bc.handle_key(&Key::Char('l')));
    assert_eq!(bc.selected(), 1);
}

#[test]
fn test_breadcrumb_handle_key_unhandled() {
    let mut bc = Breadcrumb::new().push("Test");

    assert!(!bc.handle_key(&Key::Enter));
    assert!(!bc.handle_key(&Key::Escape));
    assert!(!bc.handle_key(&Key::Char('x')));
    assert!(!bc.handle_key(&Key::Up));
    assert!(!bc.handle_key(&Key::Down));
}

#[test]
fn test_breadcrumb_handle_key_at_bounds() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    bc.set_selected(0);
    assert!(bc.handle_key(&Key::Left)); // Returns true but stays at 0
    assert_eq!(bc.selected(), 0);

    bc.set_selected(1);
    assert!(bc.handle_key(&Key::Right)); // Returns true but stays at 1
    assert_eq!(bc.selected(), 1);
}

#[test]
fn test_breadcrumb_handle_key_multiple() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C").push("D");

    bc.set_selected(0);

    assert!(bc.handle_key(&Key::Right));
    assert_eq!(bc.selected(), 1);

    assert!(bc.handle_key(&Key::Char('l')));
    assert_eq!(bc.selected(), 2);

    assert!(bc.handle_key(&Key::Right));
    assert_eq!(bc.selected(), 3);

    assert!(bc.handle_key(&Key::Left));
    assert_eq!(bc.selected(), 2);

    assert!(bc.handle_key(&Key::Char('h')));
    assert_eq!(bc.selected(), 1);
}

// =============================================================================
// Modification Methods Tests
// =============================================================================

#[test]
fn test_breadcrumb_pop_single() {
    let mut bc = Breadcrumb::new().push("A").push("B");

    let item = bc.pop();
    assert!(item.is_some());
    assert_eq!(item.unwrap().label, "B");
    assert_eq!(bc.len(), 1);
}

#[test]
fn test_breadcrumb_pop_multiple() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.pop();
    assert_eq!(bc.len(), 2);

    bc.pop();
    assert_eq!(bc.len(), 1);

    bc.pop();
    assert!(bc.is_empty());
}

#[test]
fn test_breadcrumb_pop_empty() {
    let mut bc = Breadcrumb::new();
    let item = bc.pop();
    assert!(item.is_none());
}

#[test]
fn test_breadcrumb_pop_returned_item() {
    let mut bc = Breadcrumb::new();
    bc = bc.item(BreadcrumbItem::new("Test").icon('📁'));

    let item = bc.pop().unwrap();
    assert_eq!(item.label, "Test");
    assert_eq!(item.icon, Some('📁'));
}

#[test]
fn test_breadcrumb_navigate_to_middle() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C").push("D");

    bc.navigate_to(1);
    assert_eq!(bc.len(), 2);
    assert_eq!(bc.selected(), 1);
    assert_eq!(bc.path_string(), "A/B");
}

#[test]
fn test_breadcrumb_navigate_to_first() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.navigate_to(0);
    assert_eq!(bc.len(), 1);
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_navigate_to_last() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.navigate_to(2);
    assert_eq!(bc.len(), 3);
    assert_eq!(bc.selected(), 2);
}

#[test]
fn test_breadcrumb_navigate_to_out_of_bounds() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.navigate_to(10);
    assert_eq!(bc.len(), 3); // No change
}

#[test]
fn test_breadcrumb_navigate_to_empty() {
    let mut bc = Breadcrumb::new();
    bc.navigate_to(0);
    assert!(bc.is_empty());
}

// =============================================================================
// Breadcrumb::total_width() tests
// =============================================================================

#[test]
fn test_breadcrumb_total_width_empty() {
    let bc = Breadcrumb::new();
    // Home icon
    assert_eq!(bc.total_width(), 2);
}

#[test]
fn test_breadcrumb_total_width_no_home() {
    let bc = Breadcrumb::new().home(false).push("Home");
    assert_eq!(bc.total_width(), 4); // "Home" = 4 chars
}

#[test]
fn test_breadcrumb_total_width_with_separator() {
    let bc = Breadcrumb::new().home(false).push("Home").push("Folder");
    // Home(4) + sep(3) + Folder(6) = 13
    assert_eq!(bc.total_width(), 13);
}

#[test]
fn test_breadcrumb_total_width_with_icon() {
    let bc = Breadcrumb::new()
        .home(false)
        .item(BreadcrumbItem::new("Home").icon('🏠'));
    // icon(2) + Home(4) = 6
    assert_eq!(bc.total_width(), 6);
}

// =============================================================================
// Render Tests
// =============================================================================

#[test]
fn test_breadcrumb_render_basic() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().home(false).push("Home").push("Documents");
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_with_home() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().home(true).push("Documents");
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_with_icons() {
    let mut buffer = Buffer::new(60, 3);
    let area = Rect::new(0, 0, 60, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new()
        .home(false)
        .item(BreadcrumbItem::new("Documents").icon('📁'))
        .item(BreadcrumbItem::new("Work").icon('💼'));

    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_empty() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().home(false);
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_single_item() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().home(false).push("Only");
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_long_path() {
    let mut buffer = Buffer::new(80, 3);
    let area = Rect::new(0, 0, 80, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new()
        .home(false)
        .push("home")
        .push("user")
        .push("documents")
        .push("work")
        .push("projects")
        .push("revue");

    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_with_collapse() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new()
        .home(false)
        .max_width(25)
        .collapse(true)
        .push("Very")
        .push("Long")
        .push("Path")
        .push("That")
        .push("Needs")
        .push("Collapse");

    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_no_collapse() {
    let mut buffer = Buffer::new(30, 3);
    let area = Rect::new(0, 0, 30, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new()
        .home(false)
        .collapse(false)
        .push("A")
        .push("B")
        .push("C");

    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_small_area() {
    let mut buffer = Buffer::new(2, 1);
    let area = Rect::new(0, 0, 2, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().push("Test");
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_zero_area() {
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().push("Test");
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_zero_height() {
    let mut buffer = Buffer::new(40, 0);
    let area = Rect::new(0, 0, 40, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new().push("Test");
    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_all_separator_styles() {
    let styles = [
        SeparatorStyle::Slash,
        SeparatorStyle::Arrow,
        SeparatorStyle::Chevron,
        SeparatorStyle::DoubleArrow,
        SeparatorStyle::Dot,
        SeparatorStyle::Pipe,
    ];

    for style in styles {
        let mut buffer = Buffer::new(40, 3);
        let area = Rect::new(0, 0, 40, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let bc = Breadcrumb::new()
            .home(false)
            .separator(style)
            .push("A")
            .push("B");

        bc.render(&mut ctx);
    }
}

#[test]
fn test_breadcrumb_render_with_custom_colors() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new()
        .home(false)
        .item_color(Color::RED)
        .selected_color(Color::CYAN)
        .separator_color(Color::rgb(128, 128, 128))
        .push("Home")
        .push("Documents");

    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_render_selected_item() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut bc = Breadcrumb::new()
        .home(false)
        .push("First")
        .push("Second")
        .push("Third");

    bc.set_selected(1);
    bc.render(&mut ctx);
}

// =============================================================================
// CSS/Styling Tests
// =============================================================================

#[test]
fn test_breadcrumb_element_id() {
    let bc = Breadcrumb::new().element_id("nav-breadcrumb");
    assert_eq!(View::id(&bc), Some("nav-breadcrumb"));
}

#[test]
fn test_breadcrumb_classes() {
    let bc = Breadcrumb::new().class("nav").class("interactive");

    assert!(bc.has_class("nav"));
    assert!(bc.has_class("interactive"));
    assert!(!bc.has_class("hidden"));
}

#[test]
fn test_breadcrumb_classes_from_view_trait() {
    let bc = Breadcrumb::new().class("breadcrumb").class("nav");

    let classes = View::classes(&bc);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"breadcrumb".to_string()));
    assert!(classes.contains(&"nav".to_string()));
}

#[test]
fn test_breadcrumb_meta() {
    let bc = Breadcrumb::new().element_id("test-breadcrumb").class("nav");

    let meta = bc.meta();
    assert_eq!(meta.widget_type, "Breadcrumb");
    assert_eq!(meta.id, Some("test-breadcrumb".to_string()));
    assert!(meta.classes.contains("nav"));
}

// =============================================================================
// Edge Cases and Complex Scenarios
// =============================================================================

#[test]
fn test_breadcrumb_empty_label() {
    let bc = Breadcrumb::new().push("").push("Valid");
    assert_eq!(bc.len(), 2);
}

#[test]
fn test_breadcrumb_very_long_label() {
    let long_label = "This is a very long breadcrumb item label that exceeds normal width";
    let bc = Breadcrumb::new().push(long_label);

    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    bc.render(&mut ctx);
}

#[test]
fn test_breadcrumb_unicode_labels() {
    let bc = Breadcrumb::new().push("홈").push("문서").push("작업");

    assert_eq!(bc.len(), 3);
    assert_eq!(bc.path_string(), "홈/문서/작업");
}

#[test]
fn test_breadcrumb_special_characters() {
    let bc = Breadcrumb::new()
        .push("home & office")
        .push("docs/files")
        .push("project <test>");

    assert_eq!(bc.len(), 3);
}

#[test]
fn test_breadcrumb_navigation_chain() {
    let mut bc = Breadcrumb::new()
        .push("A")
        .push("B")
        .push("C")
        .push("D")
        .push("E");

    // Navigate back to C
    bc.navigate_to(2);
    assert_eq!(bc.len(), 3);
    assert_eq!(bc.selected(), 2);

    // Navigate back to A
    bc.navigate_to(0);
    assert_eq!(bc.len(), 1);
    assert_eq!(bc.selected(), 0);
}

#[test]
fn test_breadcrumb_pop_and_rebuild() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    // Pop everything
    bc.pop();
    bc.pop();
    bc.pop();
    assert!(bc.is_empty());

    // Rebuild
    bc = bc.push("X").push("Y");
    assert_eq!(bc.len(), 2);
}

#[test]
fn test_breadcrumb_select_after_pop() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.pop();
    assert_eq!(bc.len(), 2);

    // Selection should be valid
    bc.select_next();
    bc.select_prev();
}

#[test]
fn test_breadcrumb_path_with_special_separators() {
    let bc = Breadcrumb::new().path("home/user/documents/work");
    assert_eq!(bc.len(), 4);
}

#[test]
fn test_breadcrumb_path_with_consecutive_separators() {
    let bc = Breadcrumb::new().path("home///user///documents");
    // Empty segments should be filtered
    assert_eq!(bc.len(), 3);
}

#[test]
fn test_breadcrumb_multiple_items_same_label() {
    let bc = Breadcrumb::new()
        .push("folder")
        .push("folder")
        .push("folder");

    assert_eq!(bc.len(), 3);
    assert_eq!(bc.path_string(), "folder/folder/folder");
}

#[test]
fn test_breadcrumb_item_with_all_properties() {
    let bc = Breadcrumb::new()
        .home(false)
        .item(BreadcrumbItem::new("Complete").icon('✓').clickable(true))
        .item(BreadcrumbItem::new("Disabled").icon('🔒').clickable(false));

    assert_eq!(bc.len(), 2);
}

#[test]
fn test_breadcrumb_render_multiple_times() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);

    let bc = Breadcrumb::new().home(false).push("A").push("B");

    // Render multiple times - shouldn't crash
    for _ in 0..3 {
        let mut ctx = RenderContext::new(&mut buffer, area);
        bc.render(&mut ctx);
    }
}

#[test]
fn test_breadcrumb_all_builder_chains() {
    // Test different builder chain combinations
    let bc1 = Breadcrumb::new()
        .push("A")
        .push("B")
        .separator(SeparatorStyle::Arrow);

    let bc2 = Breadcrumb::new()
        .separator(SeparatorStyle::Chevron)
        .push("A")
        .push("B");

    let bc3 = breadcrumb().home(false).collapse(false).max_width(50);

    assert_eq!(bc1.len(), 2);
    assert_eq!(bc2.len(), 2);
    assert!(bc3.is_empty());
}

#[test]
fn test_breadcrumb_custom_home_icon_renders() {
    let mut buffer = Buffer::new(40, 3);
    let area = Rect::new(0, 0, 40, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let bc = Breadcrumb::new()
        .home(true)
        .home_icon('⌂')
        .push("Documents");

    bc.render(&mut ctx);

    // Just verify it doesn't crash
}

#[test]
fn test_breadcrumb_vim_keys_navigation() {
    let mut bc = Breadcrumb::new().push("A").push("B").push("C");

    bc.set_selected(0);

    // Vim 'l' key
    assert!(bc.handle_key(&Key::Char('l')));
    assert_eq!(bc.selected(), 1);

    // Vim 'h' key
    assert!(bc.handle_key(&Key::Char('h')));
    assert_eq!(bc.selected(), 0);
}