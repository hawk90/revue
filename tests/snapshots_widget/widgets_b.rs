//! Widget snapshot tests part B (Grid, Autocomplete, TextArea, VirtualList, Menu, Tooltip)

#![allow(unused_imports)]

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};
use revue::widget::{Accordion, Breadcrumb, Calendar, Gauge, Grid, Rating, Slider, Switch};

#[test]
fn test_grid_basic() {
    use revue::widget::TrackSize;

    let view = Grid::new()
        .columns(vec![
            TrackSize::Fr(1.0),
            TrackSize::Fr(1.0),
            TrackSize::Fr(1.0),
        ])
        .child(text("1"))
        .child(text("2"))
        .child(text("3"))
        .child(text("4"))
        .child(text("5"))
        .child(text("6"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("grid_basic");
}

// =============================================================================
// Autocomplete Widget Tests
// =============================================================================

#[test]
fn test_autocomplete_basic() {
    use revue::widget::Autocomplete;

    let view = Autocomplete::new().placeholder("Search...").suggestions([
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
    ]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("autocomplete_basic");
}

#[test]
fn test_autocomplete_with_value() {
    use revue::widget::Autocomplete;

    let view = Autocomplete::new()
        .placeholder("Search fruits...")
        .suggestions(["Apple", "Apricot", "Avocado"])
        .value("Ap");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("autocomplete_with_value");
}

// =============================================================================
// TextArea Widget Tests
// =============================================================================

#[test]
fn test_textarea_basic() {
    use revue::widget::TextArea;

    let view =
        TextArea::new().content("Hello, World!\nThis is a multi-line text area.\nLine 3 here.");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("textarea_basic");
}

#[test]
fn test_textarea_with_line_numbers() {
    use revue::widget::TextArea;

    let view = TextArea::new()
        .content("fn main() {\n    println!(\"Hello\");\n}")
        .line_numbers(true);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("textarea_line_numbers");
}

#[test]
fn test_textarea_with_placeholder() {
    use revue::widget::TextArea;

    let view = TextArea::new().placeholder("Enter your code here...");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("textarea_placeholder");
}

// =============================================================================
// VirtualList Widget Tests
// =============================================================================

#[test]
fn test_virtuallist_basic() {
    use revue::widget::VirtualList;

    let items: Vec<String> = (0..100).map(|i| format!("Item {}", i)).collect();
    let view = VirtualList::new(items).item_height(1).selected(5);

    let config = TestConfig::with_size(40, 10);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("virtuallist_basic");
}

#[test]
fn test_virtuallist_with_scrollbar() {
    use revue::widget::VirtualList;

    let items: Vec<String> = (0..50).map(|i| format!("Row {}", i)).collect();
    let view = VirtualList::new(items)
        .item_height(1)
        .show_scrollbar(true)
        .selected(10);

    let config = TestConfig::with_size(30, 8);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("virtuallist_scrollbar");
}

// =============================================================================
// Menu Widget Tests
// =============================================================================

#[test]
fn test_menubar_basic() {
    use revue::widget::{Menu, MenuBar, MenuItem};

    let view = MenuBar::new()
        .menu(
            Menu::new("File")
                .item(MenuItem::new("New"))
                .item(MenuItem::new("Open"))
                .item(MenuItem::separator())
                .item(MenuItem::new("Save"))
                .item(MenuItem::new("Exit")),
        )
        .menu(
            Menu::new("Edit")
                .item(MenuItem::new("Cut"))
                .item(MenuItem::new("Copy")),
        )
        .menu(Menu::new("Help").item(MenuItem::new("About")));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("menubar_basic");
}

#[test]
fn test_menubar_with_shortcuts() {
    use revue::widget::{Menu, MenuBar, MenuItem};

    let view = MenuBar::new()
        .menu(
            Menu::new("File")
                .item(MenuItem::new("New").shortcut("Ctrl+N"))
                .item(MenuItem::new("Open").shortcut("Ctrl+O"))
                .item(MenuItem::new("Save").shortcut("Ctrl+S")),
        )
        .menu(
            Menu::new("Edit")
                .item(MenuItem::new("Cut").shortcut("Ctrl+X"))
                .item(MenuItem::new("Copy").shortcut("Ctrl+C"))
                .item(MenuItem::new("Paste").shortcut("Ctrl+V")),
        );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("menubar_shortcuts");
}

// =============================================================================
// Tooltip Widget Tests
// =============================================================================

#[test]
fn test_tooltip_basic() {
    use revue::widget::Tooltip;

    let view = Tooltip::new("This is helpful information")
        .visible(true)
        .anchor(10, 5);

    let config = TestConfig::with_size(50, 10);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tooltip_basic");
}

#[test]
fn test_tooltip_variants() {
    use revue::widget::Tooltip;

    let view = vstack()
        .gap(2)
        .child(Tooltip::info("Info tooltip").visible(true).anchor(5, 1))
        .child(
            Tooltip::warning("Warning tooltip")
                .visible(true)
                .anchor(5, 4),
        )
        .child(Tooltip::error("Error tooltip").visible(true).anchor(5, 7));

    let config = TestConfig::with_size(50, 12);
    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tooltip_variants");
}
