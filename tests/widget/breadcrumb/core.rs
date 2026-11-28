//! Breadcrumb core tests

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::render::Modifier;
use revue::style::Color;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{breadcrumb, Breadcrumb, BreadcrumbItem, SeparatorStyle};

/// Render `bc` into a `width` x 1 buffer.
fn render(bc: &Breadcrumb, width: u16) -> Buffer {
    let mut buffer = Buffer::new(width, 1);
    let area = Rect::new(0, 0, width, 1);
    let mut ctx = RenderContext::new(&mut buffer, area);
    bc.render(&mut ctx);
    buffer
}

/// The rendered line, with trailing blanks trimmed.
fn line(bc: &Breadcrumb, width: u16) -> String {
    let buffer = render(bc, width);
    (0..width)
        .filter_map(|x| buffer.get(x, 0))
        .filter(|c| !c.is_continuation())
        .map(|c| c.symbol)
        .collect::<String>()
        .trim_end()
        .to_string()
}

// =============================================================================
// Breadcrumb Constructor Tests
// =============================================================================

#[test]
fn test_breadcrumb_new() {
    let bc = Breadcrumb::new();
    assert!(bc.is_empty());
    assert_eq!(bc.len(), 0);
    assert_eq!(bc.selected(), 0);
    // The home icon is shown by default
    let buffer = render(&bc.push("Docs"), 20);
    // 🏠 takes columns 0-1, then a space, the separator and a space
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '🏠');
    assert!(buffer.get(1, 0).unwrap().is_continuation());
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '›');
    assert_eq!(buffer.get(5, 0).unwrap().symbol, 'D');
}

#[test]
fn test_breadcrumb_collapses_by_default() {
    let bc = Breadcrumb::new()
        .home(false)
        .push("Alpha")
        .push("Beta")
        .push("Gamma")
        .push("Delta");
    assert_eq!(line(&bc, 27), "Alpha › … › Gamma › Delta");
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
    let bc = Breadcrumb::new()
        .home(false)
        .item(BreadcrumbItem::new("Home").icon('*'));
    assert_eq!(bc.items()[0].icon, Some('*'));
    assert_eq!(line(&bc, 20), "* Home");
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
        .home(false)
        .separator(SeparatorStyle::Arrow)
        .push("A")
        .push("B");
    assert_eq!(line(&bc, 20), "A > B");
}

#[test]
fn test_breadcrumb_default_separator_is_chevron() {
    let bc = Breadcrumb::new().home(false).push("A").push("B");
    assert_eq!(line(&bc, 20), "A › B");
}

#[test]
fn test_breadcrumb_item_color() {
    let mut bc = Breadcrumb::new()
        .home(false)
        .item_color(Color::RED)
        .push("A")
        .push("B");
    bc.set_selected(1);
    let buffer = render(&bc, 10);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_ne!(buffer.get(4, 0).unwrap().fg, Some(Color::RED));
}

#[test]
fn test_breadcrumb_selected_color() {
    let bc = Breadcrumb::new()
        .home(false)
        .selected_color(Color::GREEN)
        .push("A")
        .push("B");
    let buffer = render(&bc, 10);
    // The last item is selected: drawn bold in the selected color
    let cell = buffer.get(4, 0).unwrap();
    assert_eq!(cell.fg, Some(Color::GREEN));
    assert!(cell.modifier.contains(Modifier::BOLD));
    let first = buffer.get(0, 0).unwrap();
    assert_ne!(first.fg, Some(Color::GREEN));
    assert!(!first.modifier.contains(Modifier::BOLD));
}

#[test]
fn test_breadcrumb_separator_color() {
    let bc = Breadcrumb::new()
        .home(false)
        .separator_color(Color::rgb(128, 128, 128))
        .push("A")
        .push("B");
    let buffer = render(&bc, 10);
    let sep = buffer.get(2, 0).unwrap();
    assert_eq!(sep.symbol, '›');
    assert_eq!(sep.fg, Some(Color::rgb(128, 128, 128)));
}

#[test]
fn test_breadcrumb_home_enabled() {
    let bc = Breadcrumb::new().home(true).home_icon('~').push("Test");
    assert_eq!(line(&bc, 20), "~ › Test");
}

#[test]
fn test_breadcrumb_home_without_items() {
    // No separator after the home icon when the trail is empty
    let bc = Breadcrumb::new().home_icon('~');
    assert_eq!(line(&bc, 20), "~");
}

#[test]
fn test_breadcrumb_home_disabled() {
    let bc = Breadcrumb::new().home(false).push("Test");
    assert_eq!(line(&bc, 20), "Test");
}

#[test]
fn test_breadcrumb_home_icon() {
    let bc = Breadcrumb::new().home_icon('🏡').home(true).push("Test");
    assert_eq!(render(&bc, 20).get(0, 0).unwrap().symbol, '🏡');
}

#[test]
fn test_breadcrumb_max_width() {
    // max_width clips the trail even when the area is wider
    let bc = Breadcrumb::new()
        .home(false)
        .collapse(false)
        .max_width(6)
        .push("Documents");
    assert_eq!(line(&bc, 20), "Docume");
}

#[test]
fn test_breadcrumb_max_width_zero() {
    // 0 means no limit beyond the area
    let bc = Breadcrumb::new().home(false).max_width(0).push("Documents");
    assert_eq!(line(&bc, 20), "Documents");
    assert_eq!(line(&bc, 4), "Docu");
}

#[test]
fn test_breadcrumb_collapse_enabled() {
    let bc = Breadcrumb::new()
        .home(false)
        .collapse(true)
        .max_width(27)
        .push("Alpha")
        .push("Beta")
        .push("Gamma")
        .push("Delta");
    assert_eq!(line(&bc, 40), "Alpha › … › Gamma › Delta");
}

#[test]
fn test_breadcrumb_collapse_disabled() {
    let bc = Breadcrumb::new()
        .home(false)
        .collapse(false)
        .max_width(27)
        .push("Alpha")
        .push("Beta")
        .push("Gamma")
        .push("Delta");
    assert_eq!(line(&bc, 40), "Alpha › Beta › Gamma › Delt");
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
fn test_breadcrumb_fits_exactly_without_collapse() {
    // "Alpha › Beta › Gamma › Delta" is 28 columns
    let bc = Breadcrumb::new()
        .home(false)
        .max_width(28)
        .push("Alpha")
        .push("Beta")
        .push("Gamma")
        .push("Delta");
    assert_eq!(line(&bc, 40), "Alpha › Beta › Gamma › Delta");
}

#[test]
fn test_breadcrumb_home_counts_toward_collapse_width() {
    // "~ › Alpha › Beta › Gamma › Delta" is 32 columns: it fits in 32 ...
    let make = || {
        Breadcrumb::new()
            .home_icon('~')
            .push("Alpha")
            .push("Beta")
            .push("Gamma")
            .push("Delta")
    };
    assert_eq!(
        line(&make().max_width(32), 40),
        "~ › Alpha › Beta › Gamma › Delta"
    );
    // ... and collapses in 31 instead of being cut off
    assert_eq!(
        line(&make().max_width(31), 40),
        "~ › Alpha › … › Gamma › Delta"
    );
}

#[test]
fn test_breadcrumb_three_items_collapse_the_middle_one() {
    // With three items the middle one is left out for one `…`, and then
    // the first is shortened (#1601)
    let bc = Breadcrumb::new()
        .home(false)
        .push("Alpha")
        .push("Beta")
        .push("Gamma");
    assert_eq!(line(&bc, 20), "Alpha › Beta › Gamma");
    assert_eq!(line(&bc, 19), "Alpha › … › Gamma");
    assert_eq!(line(&bc, 17), "Alpha › … › Gamma");
    assert_eq!(line(&bc, 16), "Alp… › … › Gamma");
}

#[test]
fn test_breadcrumb_icon_counts_toward_collapse_width() {
    // "* Alpha › Beta › Gamma › Delta" is 30 columns
    let make = || {
        Breadcrumb::new()
            .home(false)
            .item(BreadcrumbItem::new("Alpha").icon('*'))
            .push("Beta")
            .push("Gamma")
            .push("Delta")
    };
    assert_eq!(
        line(&make().max_width(30), 40),
        "* Alpha › Beta › Gamma › Delta"
    );
    assert_eq!(
        line(&make().max_width(29), 40),
        "* Alpha › … › Gamma › Delta"
    );
}

// =============================================================================
// Render Tests
// =============================================================================

#[test]
fn test_breadcrumb_render_basic() {
    let bc = Breadcrumb::new().home(false).push("Home").push("Documents");
    assert_eq!(line(&bc, 40), "Home › Documents");
}

#[test]
fn test_breadcrumb_render_with_home() {
    let bc = Breadcrumb::new().home(true).push("Documents");
    let buffer = render(&bc, 40);
    assert_eq!(buffer.get(0, 0).unwrap().symbol, '🏠');
    assert_eq!(buffer.get(3, 0).unwrap().symbol, '›');
    let text: String = (5..14).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert_eq!(text, "Documents");
}

#[test]
fn test_breadcrumb_render_with_icons() {
    let bc = Breadcrumb::new()
        .home(false)
        .item(BreadcrumbItem::new("Documents").icon('D'))
        .item(BreadcrumbItem::new("Work").icon('W'));
    assert_eq!(line(&bc, 60), "D Documents › W Work");
}

#[test]
fn test_breadcrumb_render_empty() {
    let bc = Breadcrumb::new().home(false);
    assert_eq!(line(&bc, 40), "");
}

#[test]
fn test_breadcrumb_render_single_item() {
    let bc = Breadcrumb::new().home(false).push("Only");
    assert_eq!(line(&bc, 40), "Only");
}

#[test]
fn test_breadcrumb_render_long_path() {
    let bc = Breadcrumb::new()
        .home(false)
        .push("home")
        .push("user")
        .push("documents")
        .push("work")
        .push("projects")
        .push("revue");
    assert_eq!(
        line(&bc, 80),
        "home › user › documents › work › projects › revue"
    );
}

#[test]
fn test_breadcrumb_render_with_collapse() {
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
    // The last two would be cut: only the last is kept after the `…`
    // (#1601)
    assert_eq!(line(&bc, 30), "Very › … › Collapse");
}

/// #1581: the left-out items are one `…` cell, as in Pagination, so the
/// first item keeps the two columns three periods took
#[test]
fn test_breadcrumb_collapsed_items_are_one_ellipsis_cell() {
    let gray = Color::rgb(90, 90, 90);
    let bc = Breadcrumb::new()
        .home(false)
        .separator_color(gray)
        .push("Alphabetical")
        .push("B")
        .push("C")
        .push("D");
    // Even the first item, `…` and the last one are 20 columns: the first
    // label is shortened to the 4 columns " › … › D" leave (#1601)
    let buffer = render(&bc, 12);
    let cells: Vec<(char, Option<Color>)> = (0..12)
        .map(|x| {
            let c = buffer.get(x, 0).unwrap();
            (c.symbol, c.fg)
        })
        .collect();
    let item = Some(revue::widget::theme::LIGHT_GRAY);
    let sep = Some(gray);
    let expected = [
        ('A', item),
        ('l', item),
        ('p', item),
        ('…', item),
        (' ', None),
        ('›', sep),
        (' ', None),
        ('…', sep),
        (' ', None),
        ('›', sep),
        (' ', None),
        ('D', Some(Color::CYAN)),
    ];
    assert_eq!(cells, expected);
}

#[test]
fn test_breadcrumb_render_no_collapse() {
    let bc = Breadcrumb::new()
        .home(false)
        .collapse(false)
        .push("A")
        .push("B")
        .push("C");
    assert_eq!(line(&bc, 30), "A › B › C");
}

#[test]
fn test_breadcrumb_render_small_area() {
    // Areas narrower than three columns are left untouched
    let bc = Breadcrumb::new().home(false).push("Test");
    assert_eq!(line(&bc, 2), "");
    assert_eq!(line(&bc, 3), "Tes");
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
        (SeparatorStyle::Slash, "A / B"),
        (SeparatorStyle::Arrow, "A > B"),
        (SeparatorStyle::Chevron, "A › B"),
        (SeparatorStyle::DoubleArrow, "A » B"),
        (SeparatorStyle::Dot, "A • B"),
        (SeparatorStyle::Pipe, "A | B"),
        (SeparatorStyle::Custom('→'), "A → B"),
    ];

    for (style, expected) in styles {
        let bc = Breadcrumb::new()
            .home(false)
            .separator(style)
            .push("A")
            .push("B");
        assert_eq!(line(&bc, 40), expected, "{style:?}");
    }
}

#[test]
fn test_breadcrumb_render_with_custom_colors() {
    let bc = Breadcrumb::new()
        .home(false)
        .item_color(Color::RED)
        .selected_color(Color::CYAN)
        .separator_color(Color::rgb(128, 128, 128))
        .push("Home")
        .push("Documents");
    let buffer = render(&bc, 40);
    assert_eq!(buffer.get(0, 0).unwrap().fg, Some(Color::RED));
    assert_eq!(
        buffer.get(5, 0).unwrap().fg,
        Some(Color::rgb(128, 128, 128))
    );
    assert_eq!(buffer.get(7, 0).unwrap().fg, Some(Color::CYAN));
}

#[test]
fn test_breadcrumb_render_selected_item() {
    let mut bc = Breadcrumb::new()
        .home(false)
        .selected_color(Color::CYAN)
        .push("First")
        .push("Second")
        .push("Third");
    bc.set_selected(1);
    let buffer = render(&bc, 40);
    // "First › Second › Third": Second starts at column 8
    assert_eq!(buffer.get(8, 0).unwrap().symbol, 'S');
    assert_eq!(buffer.get(8, 0).unwrap().fg, Some(Color::CYAN));
    assert!(buffer.get(8, 0).unwrap().modifier.contains(Modifier::BOLD));
    assert_ne!(buffer.get(0, 0).unwrap().fg, Some(Color::CYAN));
    assert_ne!(buffer.get(17, 0).unwrap().fg, Some(Color::CYAN));
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
    let bc = Breadcrumb::new().home(false).push("").push("Valid");
    assert_eq!(bc.len(), 2);
    assert_eq!(line(&bc, 20), " › Valid");
}

#[test]
fn test_breadcrumb_very_long_label() {
    let long_label = "This is a very long breadcrumb item label that exceeds normal width";
    let bc = Breadcrumb::new().home(false).push(long_label);
    assert_eq!(line(&bc, 40), long_label[..40]);
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
    assert_eq!(bc.path_string(), "home & office/docs/files/project <test>");
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

    // Selection is clamped to the remaining items
    bc.select_next();
    assert_eq!(bc.selected_item().unwrap().label, "B");
    bc.select_prev();
    assert_eq!(bc.selected_item().unwrap().label, "A");
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
        .item(BreadcrumbItem::new("Complete").icon('+').clickable(true))
        .item(BreadcrumbItem::new("Disabled").icon('x').clickable(false));

    assert_eq!(bc.len(), 2);
    assert!(bc.items()[0].clickable);
    assert!(!bc.items()[1].clickable);
    assert_eq!(line(&bc, 40), "+ Complete › x Disabled");
}

#[test]
fn test_breadcrumb_render_multiple_times() {
    let mut buffer = Buffer::new(40, 1);
    let area = Rect::new(0, 0, 40, 1);

    let bc = Breadcrumb::new().home(false).push("A").push("B");

    // Rendering is idempotent
    for _ in 0..3 {
        let mut ctx = RenderContext::new(&mut buffer, area);
        bc.render(&mut ctx);
    }
    let text: String = (0..5).map(|x| buffer.get(x, 0).unwrap().symbol).collect();
    assert_eq!(text, "A › B");
}

#[test]
fn test_breadcrumb_all_builder_chains() {
    // Builder order does not matter
    let bc1 = Breadcrumb::new()
        .home(false)
        .push("A")
        .push("B")
        .separator(SeparatorStyle::Arrow);

    let bc2 = Breadcrumb::new()
        .separator(SeparatorStyle::Arrow)
        .push("A")
        .push("B")
        .home(false);

    let bc3 = breadcrumb().home(false).collapse(false).max_width(50);

    assert_eq!(line(&bc1, 20), "A > B");
    assert_eq!(line(&bc2, 20), "A > B");
    assert!(bc3.is_empty());
}

#[test]
fn test_breadcrumb_custom_home_icon_renders() {
    let bc = Breadcrumb::new()
        .home(true)
        .home_icon('⌂')
        .push("Documents");
    assert_eq!(line(&bc, 40), "⌂ › Documents");
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

// =============================================================================
// #1601: a collapsed trail that still does not fit collapses further, then
// shortens its first item, so the last item is drawn whole
// =============================================================================

mod collapse_further {
    use revue::layout::Rect;
    use revue::render::{Buffer, Modifier};
    use revue::style::Color;
    use revue::widget::theme::{DARK_GRAY, LIGHT_GRAY};
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::{Breadcrumb, BreadcrumbItem};

    /// A cell: its glyph (`'>'` for the second half of a wide glyph, with no
    /// color) and its color
    type Drawn = (char, Option<Color>);

    const ITEM: Option<Color> = Some(LIGHT_GRAY);
    const SEP: Option<Color> = Some(DARK_GRAY);
    const SELECTED: Option<Color> = Some(Color::CYAN);

    /// The cells of a `width`-column area at column `x` of a buffer `x + 2`
    /// columns wider, the whole buffer row
    fn cells_at(bc: &Breadcrumb, x: u16, width: u16) -> Vec<Drawn> {
        let mut buffer = Buffer::new(x + width + 2, 1);
        bc.render(&mut RenderContext::new(
            &mut buffer,
            Rect::new(x, 0, width, 1),
        ));
        (0..x + width + 2)
            .map(|cx| {
                let c = buffer.get(cx, 0).unwrap();
                if c.is_continuation() {
                    ('>', None)
                } else {
                    (c.symbol, c.fg)
                }
            })
            .collect()
    }

    fn cells(bc: &Breadcrumb, width: u16) -> Vec<Drawn> {
        let mut all = cells_at(bc, 0, width);
        all.truncate(usize::from(width));
        all
    }

    /// The cells `parts` draw, glyph by glyph, padded with blank cells to
    /// `width`
    fn expect(parts: &[(&str, Option<Color>)], width: usize) -> Vec<Drawn> {
        let mut out = Vec::new();
        for (text, color) in parts {
            for ch in text.chars() {
                let color = if ch == ' ' { None } else { *color };
                out.push((ch, color));
                if revue::utils::char_width(ch) == 2 {
                    out.push(('>', None));
                }
            }
        }
        out.resize(width, (' ', None));
        out
    }

    /// " › " between entries
    const S: (&str, Option<Color>) = (" › ", SEP);
    /// The `…` for the items left out
    const E: (&str, Option<Color>) = ("…", SEP);

    fn four(first: &str, last: &str) -> Breadcrumb {
        Breadcrumb::new()
            .home(false)
            .push(first)
            .push("a")
            .push("b")
            .push(last)
    }

    #[test]
    fn the_last_two_that_fit_are_kept() {
        // 32 columns in full; "Documents › … › c › Settings" is 28
        let bc = Breadcrumb::new()
            .home(false)
            .push("Documents")
            .push("a")
            .push("b")
            .push("c")
            .push("Settings");
        assert_eq!(
            cells(&bc, 28),
            expect(
                &[
                    ("Documents", ITEM),
                    S,
                    E,
                    S,
                    ("c", ITEM),
                    S,
                    ("Settings", SELECTED)
                ],
                28
            )
        );
    }

    #[test]
    fn the_last_one_alone_when_the_last_two_do_not_fit() {
        // 27 columns: "b" goes too, "Documents › … › Settings" is 24
        let bc = four("Documents", "Settings");
        for width in [27, 24] {
            assert_eq!(
                cells(&bc, width),
                expect(
                    &[("Documents", ITEM), S, E, S, ("Settings", SELECTED)],
                    usize::from(width)
                ),
                "width {width}"
            );
        }
    }

    #[test]
    fn the_first_item_is_shortened_so_the_last_one_fits() {
        let bc = four("Documents", "Settings");
        // " › … › Settings" is 15 columns: the first label gets the rest
        for (width, first) in [(23, "Documen…"), (18, "Do…"), (16, "…")] {
            assert_eq!(
                cells(&bc, width),
                expect(
                    &[(first, ITEM), S, E, S, ("Settings", SELECTED)],
                    usize::from(width)
                ),
                "width {width}"
            );
        }
    }

    #[test]
    fn a_selected_first_item_keeps_its_style_when_shortened() {
        let mut bc = four("Documents", "Settings");
        bc.set_selected(0);
        let mut buffer = Buffer::new(18, 1);
        bc.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 18, 1)));
        for x in 0..3 {
            let c = buffer.get(x, 0).unwrap();
            assert_eq!((c.symbol, c.fg), (['D', 'o', '…'][x as usize], SELECTED));
            assert!(c.modifier.contains(Modifier::BOLD), "x {x}");
        }
        assert_eq!(
            buffer.get(11, 0).unwrap().fg,
            ITEM,
            "the last item is not selected"
        );
    }

    #[test]
    fn the_first_items_icon_stays_when_its_label_is_shortened() {
        let bc = Breadcrumb::new()
            .home(false)
            .item(BreadcrumbItem::new("Documents").icon('D'))
            .push("a")
            .push("b")
            .push("Settings");
        assert_eq!(
            cells(&bc, 20),
            expect(
                &[
                    ("D", ITEM),
                    (" ", None),
                    ("Do…", ITEM),
                    S,
                    E,
                    S,
                    ("Settings", SELECTED)
                ],
                20
            )
        );
    }

    #[test]
    fn two_items_shorten_the_first_with_nothing_left_out() {
        let bc = Breadcrumb::new()
            .home(false)
            .push("Documents")
            .push("Settings");
        assert_eq!(
            cells(&bc, 20),
            expect(&[("Documents", ITEM), S, ("Settings", SELECTED)], 20)
        );
        assert_eq!(
            cells(&bc, 15),
            expect(&[("Doc…", ITEM), S, ("Settings", SELECTED)], 15)
        );
    }

    #[test]
    fn the_home_icon_counts_toward_the_room_left() {
        let bc = Breadcrumb::new()
            .home_icon('~')
            .push("Documents")
            .push("a")
            .push("b")
            .push("Settings");
        // "~ › " takes 4 columns before the first item
        assert_eq!(
            cells(&bc, 22),
            expect(
                &[
                    ("~", ITEM),
                    S,
                    ("Do…", ITEM),
                    S,
                    E,
                    S,
                    ("Settings", SELECTED)
                ],
                22
            )
        );
    }

    #[test]
    fn a_wide_first_label_is_not_split() {
        let bc = four("문서함", "설정");
        // " › … › 설정" is 11 columns: 3 left hold one syllable and `…`
        assert_eq!(
            cells(&bc, 14),
            expect(&[("문…", ITEM), S, E, S, ("설정", SELECTED)], 14)
        );
        // 2 left: a syllable and `…` do not fit, so `…` alone, and the
        // last column stays blank
        assert_eq!(
            cells(&bc, 13),
            expect(&[("…", ITEM), S, E, S, ("설정", SELECTED)], 13)
        );
    }

    #[test]
    fn a_last_item_wider_than_the_area_is_cut_at_the_edge() {
        // The first item is shortened to `…`, and the last is cut where the
        // area ends
        let bc = Breadcrumb::new()
            .home(false)
            .push("AB")
            .push("LongLastItem");
        assert_eq!(
            cells(&bc, 8),
            expect(&[("…", ITEM), S, ("Long", SELECTED)], 8)
        );
    }

    #[test]
    fn an_area_at_an_offset_draws_inside_it() {
        let bc = four("Documents", "Settings");
        let mut want = vec![(' ', None); 3];
        want.extend(expect(
            &[("Do…", ITEM), S, E, S, ("Settings", SELECTED)],
            18,
        ));
        want.extend([(' ', None); 2]);
        assert_eq!(cells_at(&bc, 3, 18), want);
    }
}
