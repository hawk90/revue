//! Snapshot tests for individual widgets
//!
//! Tests that verify widget rendering output matches expected snapshots.
//! Run with: cargo test
//! Update snapshots: REVUE_UPDATE_SNAPSHOTS=1 cargo test

use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};
use revue::widget::{Accordion, Breadcrumb, Calendar, Gauge, Grid, Rating, Slider, Switch};

// =============================================================================
// Text Widget Tests
// =============================================================================

#[test]
fn test_text_simple() {
    let view = text("Hello, World!");
    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_simple");
}

#[test]
fn test_text_multiline() {
    let view = vstack()
        .child(text("Line 1"))
        .child(text("Line 2"))
        .child(text("Line 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_multiline");
}

#[test]
fn test_text_formatting() {
    let view = vstack()
        .child(Text::new("Normal text"))
        .child(Text::heading("Heading"))
        .child(Text::muted("Muted text"))
        .child(Text::success("Success!"))
        .child(Text::error("Error!"))
        .child(Text::info("Info"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_formatting");
}

#[test]
fn test_text_alignment() {
    let config = TestConfig::with_size(40, 10);
    let view = vstack()
        .child(Text::new("Left aligned").align(Alignment::Left))
        .child(Text::new("Centered").align(Alignment::Center))
        .child(Text::new("Right aligned").align(Alignment::Right));

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("text_alignment");
}

// =============================================================================
// Stack Widget Tests
// =============================================================================

#[test]
fn test_vstack_basic() {
    let view = vstack()
        .child(text("Item 1"))
        .child(text("Item 2"))
        .child(text("Item 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("vstack_basic");
}

#[test]
fn test_hstack_basic() {
    let view = hstack().child(text("A")).child(text("B")).child(text("C"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("hstack_basic");
}

#[test]
fn test_vstack_with_gap() {
    let view = vstack()
        .gap(1)
        .child(text("Item 1"))
        .child(text("Item 2"))
        .child(text("Item 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("vstack_with_gap");
}

#[test]
fn test_nested_stacks() {
    let view = vstack()
        .child(text("Header"))
        .child(
            hstack()
                .child(text("Left"))
                .child(text(" | "))
                .child(text("Right")),
        )
        .child(text("Footer"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("nested_stacks");
}

// =============================================================================
// Border Widget Tests
// =============================================================================

#[test]
fn test_border_single() {
    let view = Border::single().child(text("Bordered content"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_single");
}

#[test]
fn test_border_double() {
    let view = Border::double().child(text("Double border"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_double");
}

#[test]
fn test_border_rounded() {
    let view = Border::rounded().child(text("Rounded corners"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_rounded");
}

#[test]
fn test_border_with_title() {
    let view = Border::single()
        .title("My Title")
        .child(text("Content with title"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_with_title");
}

#[test]
fn test_border_panel() {
    let view = Border::panel()
        .title("Panel")
        .child(vstack().child(text("Line 1")).child(text("Line 2")));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("border_panel");
}

// =============================================================================
// Progress Widget Tests
// =============================================================================

#[test]
fn test_progress_0_percent() {
    let view = progress(0.0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_0_percent");
}

#[test]
fn test_progress_50_percent() {
    let view = progress(0.5);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_50_percent");
}

#[test]
fn test_progress_100_percent() {
    let view = progress(1.0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_100_percent");
}

#[test]
fn test_progress_with_label() {
    let view = vstack()
        .child(text("Download Progress"))
        .child(progress(0.75));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("progress_with_label");
}

// =============================================================================
// Combined Widget Tests
// =============================================================================

#[test]
fn test_card_layout() {
    let view = Border::panel().title("Card Title").child(
        vstack()
            .gap(1)
            .child(Text::heading("Welcome"))
            .child(text("This is a card with multiple elements."))
            .child(
                hstack()
                    .child(text("[OK]"))
                    .child(text(" "))
                    .child(text("[Cancel]")),
            ),
    );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("card_layout");
}

#[test]
fn test_form_layout() {
    let view = Border::single().title("Login Form").child(
        vstack()
            .gap(1)
            .child(text("Username: "))
            .child(Border::single().child(text("admin")))
            .child(text("Password: "))
            .child(Border::single().child(text("****")))
            .child(text(""))
            .child(text("[Login]")),
    );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("form_layout");
}

#[test]
fn test_dashboard_layout() {
    let config = TestConfig::with_size(60, 20);
    let view = vstack()
        .child(
            Border::double()
                .title("Dashboard")
                .child(text("Application Status: Running")),
        )
        .child(
            hstack()
                .child(
                    Border::single().title("Stats").child(
                        vstack()
                            .child(text("CPU: 45%"))
                            .child(text("Memory: 2.1GB"))
                            .child(text("Uptime: 2h 15m")),
                    ),
                )
                .child(
                    Border::single().title("Logs").child(
                        vstack()
                            .child(Text::info("[INFO] Server started"))
                            .child(Text::success("[OK] Connected"))
                            .child(Text::error("[ERR] Failed to load")),
                    ),
                ),
        );

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("dashboard_layout");
}

// =============================================================================
// Edge Case Tests
// =============================================================================

#[test]
fn test_empty_vstack() {
    let view = vstack();

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("empty_vstack");
}

#[test]
fn test_empty_border() {
    let view = Border::single();

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("empty_border");
}

#[test]
fn test_deeply_nested() {
    let view = vstack().child(vstack().child(vstack().child(vstack().child(text("Deep")))));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("deeply_nested");
}

#[test]
fn test_long_text() {
    let long_text = "This is a very long line of text that might wrap or get truncated depending on the terminal width.";
    let view = text(long_text);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("long_text");
}

#[test]
fn test_special_characters() {
    let view = vstack()
        .child(text("ASCII: ABC abc 123"))
        .child(text("Symbols: !@#$%^&*()"))
        .child(text("Unicode: ✓ ✗ → ← ↑ ↓"))
        .child(text("Box: ┌─┐ │ │ └─┘"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("special_characters");
}

// =============================================================================
// Size Variation Tests
// =============================================================================

#[test]
fn test_small_terminal() {
    let config = TestConfig::with_size(20, 10);
    let view = Border::single()
        .title("Small")
        .child(text("Fits in small space"));

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("small_terminal");
}

#[test]
fn test_large_terminal() {
    let config = TestConfig::with_size(120, 40);
    let view = Border::double().title("Large Terminal").child(
        vstack()
            .child(text("This is a large terminal with plenty of space."))
            .child(text("We can fit much more content here."))
            .child(text("Line 3"))
            .child(text("Line 4"))
            .child(text("Line 5")),
    );

    let mut app = TestApp::with_config(view, config);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("large_terminal");
}

// =============================================================================
// Interactive State Tests (for future stateful widgets)
// =============================================================================

#[test]
fn test_focused_state() {
    // Placeholder for future focused state testing
    let view = Border::single().child(text("[Focused Element]"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("focused_state");
}

#[test]
fn test_disabled_state() {
    // Placeholder for future disabled state testing
    let view = vstack()
        .child(text("[Enabled Button]"))
        .child(Text::muted("[Disabled Button]"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("disabled_state");
}

// =============================================================================
// Button Widget Tests
// =============================================================================

#[test]
fn test_button_basic() {
    let view = button("Click Me");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("button_basic");
}

#[test]
fn test_button_variants() {
    use revue::widget::Button;
    let view = vstack()
        .gap(1)
        .child(Button::primary("Primary"))
        .child(Button::ghost("Ghost"))
        .child(Button::success("Success"))
        .child(Button::danger("Danger"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("button_variants");
}

// =============================================================================
// Badge Widget Tests
// =============================================================================

#[test]
fn test_badge_basic() {
    let view = hstack()
        .gap(1)
        .child(badge("New"))
        .child(badge("5"))
        .child(badge("Beta"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("badge_basic");
}

#[test]
fn test_badge_variants() {
    let view = vstack()
        .gap(1)
        .child(badge("Success").success())
        .child(badge("Error").error())
        .child(badge("Warning").warning())
        .child(badge("Info").info());

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("badge_variants");
}

// =============================================================================
// Checkbox Widget Tests
// =============================================================================

#[test]
fn test_checkbox_basic() {
    let view = vstack()
        .child(checkbox("Option 1"))
        .child(checkbox("Option 2"))
        .child(checkbox("Option 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("checkbox_basic");
}

#[test]
fn test_checkbox_checked() {
    let view = vstack()
        .child(checkbox("Unchecked"))
        .child(Checkbox::new("Checked").checked(true))
        .child(Checkbox::new("Disabled").disabled(true));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("checkbox_checked");
}

// =============================================================================
// Radio Widget Tests
// =============================================================================

#[test]
fn test_radio_group() {
    let view = RadioGroup::new(["Option A", "Option B", "Option C"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("radio_group");
}

// =============================================================================
// Switch Widget Tests
// =============================================================================

#[test]
fn test_switch_basic() {
    let view = vstack()
        .gap(1)
        .child(hstack().child(Switch::new()).child(text(" Enable feature")))
        .child(
            hstack()
                .child(Switch::new().on(true))
                .child(text(" Dark mode")),
        );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("switch_basic");
}

// =============================================================================
// Input Widget Tests
// =============================================================================

#[test]
fn test_input_basic() {
    let view = vstack()
        .gap(1)
        .child(Input::new().placeholder("Enter text..."))
        .child(Input::new().value("Hello World"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("input_basic");
}

#[test]
fn test_input_with_label() {
    let view = vstack()
        .gap(1)
        .child(text("Username:"))
        .child(Input::new().placeholder("Enter username"))
        .child(text("Password:"))
        .child(Input::new().placeholder("Enter password"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("input_with_label");
}

// =============================================================================
// Slider Widget Tests
// =============================================================================

#[test]
fn test_slider_basic() {
    let view = vstack()
        .gap(1)
        .child(Slider::new().value(50.0))
        .child(Slider::new().value(25.0))
        .child(Slider::new().value(75.0));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("slider_basic");
}

// =============================================================================
// Select Widget Tests
// =============================================================================

#[test]
fn test_select_basic() {
    let view = Select::new()
        .options(vec!["Option 1", "Option 2", "Option 3"])
        .selected(0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("select_basic");
}

// =============================================================================
// Table Widget Tests
// =============================================================================

#[test]
fn test_table_basic() {
    let view = Table::new(vec![
        Column::new("Name"),
        Column::new("Age"),
        Column::new("City"),
    ])
    .row(vec!["Alice", "30", "NYC"])
    .row(vec!["Bob", "25", "LA"])
    .row(vec!["Charlie", "35", "Chicago"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("table_basic");
}

#[test]
fn test_table_with_header() {
    let view = Table::new(vec![
        Column::new("ID"),
        Column::new("Product"),
        Column::new("Price"),
    ])
    .row(vec!["1", "Widget", "$9.99"])
    .row(vec!["2", "Gadget", "$19.99"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("table_with_header");
}

// =============================================================================
// Tabs Widget Tests
// =============================================================================

#[test]
fn test_tabs_basic() {
    let view = Tabs::new().tab("Home").tab("Settings").tab("About");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tabs_basic");
}

// =============================================================================
// List Widget Tests
// =============================================================================

#[test]
fn test_list_basic() {
    let view = List::new(vec!["Item 1", "Item 2", "Item 3", "Item 4", "Item 5"]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("list_basic");
}

#[test]
fn test_list_selected() {
    let view = List::new(vec!["First", "Second", "Third"]).selected(1);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("list_selected");
}

// =============================================================================
// Tree Widget Tests
// =============================================================================

#[test]
fn test_tree_basic() {
    let view = Tree::new().node(
        TreeNode::new("Root")
            .expanded(true)
            .child(TreeNode::new("Child 1"))
            .child(
                TreeNode::new("Child 2")
                    .expanded(true)
                    .child(TreeNode::new("Grandchild 1"))
                    .child(TreeNode::new("Grandchild 2")),
            )
            .child(TreeNode::new("Child 3")),
    );

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tree_basic");
}

// =============================================================================
// Divider Widget Tests
// =============================================================================

#[test]
fn test_divider_horizontal() {
    let view = vstack()
        .child(text("Above"))
        .child(Divider::new())
        .child(text("Below"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("divider_horizontal");
}

#[test]
fn test_divider_with_label() {
    let view = vstack()
        .child(text("Section 1"))
        .child(Divider::new().label("OR"))
        .child(text("Section 2"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("divider_with_label");
}

// =============================================================================
// Gauge Widget Tests
// =============================================================================

#[test]
fn test_gauge_basic() {
    let view = vstack()
        .gap(1)
        .child(Gauge::new().value(0.25).label("CPU"))
        .child(Gauge::new().value(0.75).label("Memory"))
        .child(Gauge::new().value(0.50).label("Disk"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("gauge_basic");
}

// =============================================================================
// Sparkline Widget Tests
// =============================================================================

#[test]
fn test_sparkline_basic() {
    let view = sparkline([1.0, 4.0, 2.0, 8.0, 3.0, 6.0, 5.0, 7.0]);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("sparkline_basic");
}

// =============================================================================
// Spinner Widget Tests
// =============================================================================

#[test]
fn test_spinner_basic() {
    let view = hstack()
        .gap(1)
        .child(Spinner::new())
        .child(text("Loading..."));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("spinner_basic");
}

// =============================================================================
// Tag Widget Tests
// =============================================================================

#[test]
fn test_tag_basic() {
    let view = hstack()
        .gap(1)
        .child(Tag::new("Rust"))
        .child(Tag::new("TUI"))
        .child(Tag::new("CSS"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tag_basic");
}

#[test]
fn test_tag_colors() {
    let view = hstack()
        .gap(1)
        .child(Tag::new("Success").green())
        .child(Tag::new("Warning").yellow())
        .child(Tag::new("Error").red());

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("tag_colors");
}

// =============================================================================
// Toast Widget Tests
// =============================================================================

#[test]
fn test_toast_variants() {
    let view = vstack()
        .gap(1)
        .child(Toast::success("Operation completed!"))
        .child(Toast::error("An error occurred"))
        .child(Toast::warning("Please check your input"))
        .child(Toast::info("New updates available"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("toast_variants");
}

// =============================================================================
// Avatar Widget Tests
// =============================================================================

#[test]
fn test_avatar_basic() {
    let view = hstack()
        .gap(2)
        .child(Avatar::new("JD"))
        .child(Avatar::new("AB"))
        .child(Avatar::new("XY"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("avatar_basic");
}

// =============================================================================
// Breadcrumb Widget Tests
// =============================================================================

#[test]
fn test_breadcrumb_basic() {
    let view = Breadcrumb::new()
        .push("Home")
        .push("Products")
        .push("Electronics")
        .push("Phones");

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("breadcrumb_basic");
}

// =============================================================================
// Rating Widget Tests
// =============================================================================

#[test]
fn test_rating_basic() {
    let view = vstack()
        .gap(1)
        .child(Rating::new().max_value(5).value(3.0))
        .child(Rating::new().max_value(5).value(5.0))
        .child(Rating::new().max_value(5).value(0.0));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("rating_basic");
}

// =============================================================================
// Accordion Widget Tests
// =============================================================================

#[test]
fn test_accordion_basic() {
    use revue::widget::AccordionSection;

    let view = Accordion::new()
        .section(AccordionSection::new("Section 1").content("Content for section 1"))
        .section(AccordionSection::new("Section 2").content("Content for section 2"))
        .section(AccordionSection::new("Section 3").content("Content for section 3"));

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("accordion_basic");
}

// =============================================================================
// Calendar Widget Tests
// =============================================================================

#[test]
fn test_calendar_basic() {
    let view = Calendar::new(2024, 6); // June 2024

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("calendar_basic");
}

// =============================================================================
// BarChart Widget Tests
// =============================================================================

#[test]
fn test_barchart_basic() {
    let view = BarChart::new()
        .bar("Mon", 10.0)
        .bar("Tue", 20.0)
        .bar("Wed", 15.0)
        .bar("Thu", 25.0)
        .bar("Fri", 18.0);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("barchart_basic");
}

// =============================================================================
// Notification Widget Tests
// =============================================================================

#[test]
fn test_notification_basic() {
    use revue::widget::NotificationCenter;

    // NotificationCenter is the widget, Notification is a data struct
    let view = NotificationCenter::new();

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("notification_basic");
}

// =============================================================================
// Modal Widget Tests
// =============================================================================

#[test]
fn test_modal_basic() {
    let mut modal = Modal::new()
        .title("Confirm Action")
        .content("Are you sure you want to proceed?")
        .ok();
    modal.show();

    let mut app = TestApp::new(modal);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("modal_basic");
}

// =============================================================================
// Pagination Widget Tests
// =============================================================================

#[test]
fn test_pagination_basic() {
    let view = Pagination::new(10).current(3);

    let mut app = TestApp::new(view);
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("pagination_basic");
}

// =============================================================================
// Grid Layout Tests
// =============================================================================

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
