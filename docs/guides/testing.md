# Testing Guide

Revue provides comprehensive testing utilities for TUI applications.

## Pilot Testing Framework

Pilot allows you to test widgets by simulating user interactions.

### Basic Setup

```rust
use revue::prelude::*;
use revue::testing::{Pilot, TestApp, TestConfig};

#[test]
fn test_counter() {
    // Create test app with your widget
    let mut app = TestApp::new(Counter::new());
    let mut pilot = Pilot::new(&mut app);

    // Simulate interactions
    pilot
        .press(Key::Up)
        .press(Key::Up)
        .assert_contains("2");
}
```

### Configuration

```rust
let config = TestConfig {
    width: 80,
    height: 24,
    ..Default::default()
};

let mut app = TestApp::with_config(MyWidget::new(), config);
```

## Simulating Input

### Keyboard

```rust
pilot
    .press(Key::Enter)           // Single key
    .press(Key::Char('a'))       // Character
    .type_text("hello world")    // Type string
    .press_ctrl('c')             // Ctrl+C
    .press_alt('x');             // Alt+X
```

### Mouse

```rust
pilot
    .click(10, 5)                // Click at position
    .double_click(10, 5)
    .right_click(10, 5)
    .scroll_up(10, 5)
    .scroll_down(10, 5);
```

## Assertions

### Content Assertions

```rust
pilot
    .assert_contains("Hello")           // Text exists
    .assert_not_contains("Error")       // Text doesn't exist
    .assert_line(0, "Title")            // Specific line
    .assert_cell(0, 0, 'H');            // Specific cell
```

### State Assertions

```rust
pilot
    .assert_focused(".input")           // Focus check
    .assert_visible("#modal")           // Visibility check
    .assert_selected(2);                // Selection index
```

## Snapshot Testing

Compare widget output against saved snapshots:

```rust
#[test]
fn test_widget_snapshot() {
    let mut app = TestApp::new(MyWidget::new());
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("my_widget_initial");

    pilot.press(Key::Enter);
    pilot.snapshot("my_widget_after_enter");
}
```

Update snapshots with:
```bash
REVUE_UPDATE_SNAPSHOTS=1 cargo test
```

## Visual Regression Testing

Capture and compare visual output:

```rust
use revue::testing::{VisualTest, VisualTestConfig};

#[test]
fn test_visual_regression() {
    let config = VisualTestConfig::new("tests/golden");
    let mut test = VisualTest::new(config);

    let mut app = TestApp::new(Dashboard::new());
    let mut pilot = Pilot::new(&mut app);

    // Capture visual state
    let capture = pilot.capture();

    // Compare against golden file
    test.assert_matches("dashboard_initial", &capture);
}
```

Update golden files:
```bash
REVUE_UPDATE_VISUALS=1 cargo test
```

## CI Integration

Revue detects CI environments automatically:

```rust
use revue::testing::{CiEnvironment, TestReport};

#[test]
fn test_with_ci_report() {
    let ci = CiEnvironment::detect();

    // Run tests
    let results = run_tests();

    // Generate report
    let report = TestReport::new(results);

    if ci.is_github_actions() {
        // Outputs GitHub Actions annotations
        report.github_annotations();
    }

    report.save_markdown("test-report.md");
}
```

## Testing Patterns

### Testing Event Handlers

```rust
#[test]
fn test_navigation() {
    let mut app = TestApp::new(List::new(items));
    let mut pilot = Pilot::new(&mut app);

    // Initial state
    pilot.assert_selected(0);

    // Navigate down
    pilot.press(Key::Down);
    pilot.assert_selected(1);

    // Navigate up
    pilot.press(Key::Up);
    pilot.assert_selected(0);
}
```

### Testing Forms

```rust
#[test]
fn test_form_validation() {
    let mut app = TestApp::new(LoginForm::new());
    let mut pilot = Pilot::new(&mut app);

    // Empty submit shows error
    pilot.press(Key::Enter);
    pilot.assert_contains("Required");

    // Fill form
    pilot
        .type_text("user@example.com")
        .press(Key::Tab)
        .type_text("password123")
        .press(Key::Enter);

    pilot.assert_contains("Success");
}
```

### Testing Async Operations

```rust
#[test]
fn test_async_loading() {
    let mut app = TestApp::new(DataLoader::new());
    let mut pilot = Pilot::new(&mut app);

    // Shows loading state
    pilot.assert_contains("Loading");

    // Wait for data
    pilot.wait_ms(1000);

    // Shows loaded data
    pilot.assert_contains("Data loaded");
}
```

## DevTools for Debugging

Enable devtools during development:

```rust
let mut app = App::builder()
    .devtools(true)  // F12 to toggle
    .build();
```

DevTools panels:
- **Inspector**: Widget tree viewer
- **State Debugger**: Signal values
- **Style Inspector**: Applied CSS
- **Event Logger**: Event stream

## Best Practices

### 1. Test Behavior, Not Implementation

```rust
// Good: Test what user sees
pilot.assert_contains("Item added");

// Bad: Test internal state directly
assert_eq!(app.state.items.len(), 1);
```

### 2. Use Descriptive Snapshot Names

```rust
// Good
pilot.snapshot("todo_list_with_3_items_first_selected");

// Bad
pilot.snapshot("test1");
```

### 3. Test Edge Cases

```rust
#[test]
fn test_empty_list() {
    let mut app = TestApp::new(List::new(vec![]));
    let mut pilot = Pilot::new(&mut app);

    pilot.assert_contains("No items");
}

#[test]
fn test_long_text_truncation() {
    let long_text = "a".repeat(1000);
    let mut app = TestApp::new(Text::new(long_text));
    // ...
}
```

### 4. Isolate Tests

Each test should start with fresh state:

```rust
fn create_test_app() -> TestApp<MyWidget> {
    TestApp::new(MyWidget::new())
}

#[test]
fn test_a() {
    let mut app = create_test_app();
    // ...
}

#[test]
fn test_b() {
    let mut app = create_test_app();
    // ...
}
```
