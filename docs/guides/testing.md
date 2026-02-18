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
        .press_key(Key::Up)
        .press_key(Key::Up)
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

### Custom Size

```rust
let mut app = TestApp::with_size(MyWidget::new(), 120, 40);
```

## TestApp

`TestApp` wraps a `View` widget with a virtual terminal buffer for testing.

### Construction

```rust
TestApp::new(view)                          // 80x24 default
TestApp::with_size(view, 120, 40)           // Custom size
TestApp::with_config(view, config)          // Full config
```

### Event Handlers

Register custom event handlers for testing interactive widgets:

```rust
let app = TestApp::new(MyWidget::new())
    .on_key(|event, view| {
        view.handle_key(event);
        true
    })
    .on_mouse(|event, view| {
        view.handle_mouse(event);
        true
    });
```

### Buffer Access

```rust
let text = app.screen_text();               // Full screen as string
let line = app.get_line(0);                 // Single line
let ch = app.get_cell(5, 3);               // Single character
let found = app.find_text("hello");         // Find text position
let has = app.contains("world");            // Check text exists
```

## Simulating Input

### Keyboard

```rust
pilot
    .press_key(Key::Enter)           // Single key
    .press_key(Key::Char('a'))       // Character
    .type_text("hello world")        // Type string
    .press_ctrl(Key::Char('c'))      // Ctrl+C
    .press_alt(Key::Char('x'))       // Alt+X
    .press_enter()                   // Convenience methods
    .press_escape()
    .press_tab()
    .press_up()
    .press_down();
```

### Mouse

```rust
pilot
    .click(10, 5)                    // Click at position
    .double_click(10, 5)
    .scroll_up(10, 5, 1)
    .scroll_down(10, 5, 1);
```

### Click on Text

```rust
pilot.click_text("Submit");          // Find and click text
```

## Assertions

### Content Assertions

```rust
pilot
    .assert_contains("Hello")               // Text exists
    .assert_not_contains("Error")           // Text doesn't exist
    .assert_line_contains(0, "Title")       // Line contains text
    .assert_cell(0, 0, 'H');               // Specific cell character
```

### Screen Comparison

```rust
pilot.assert_screen("expected full screen content");
```

### Querying

```rust
let text = pilot.screen();                  // Full screen text
let line = pilot.line(3);                   // Specific line
let ch = pilot.cell(5, 2);                 // Specific cell
let found = pilot.find_text("hello");       // (x, y) position
let has = pilot.screen_contains("world");   // Boolean check
```

## Snapshot Testing

Compare widget output against saved snapshots:

```rust
#[test]
fn test_widget_snapshot() {
    let mut app = TestApp::new(MyWidget::new());
    let mut pilot = Pilot::new(&mut app);

    pilot.snapshot("my_widget_initial");

    pilot.press_key(Key::Enter);
    pilot.snapshot("my_widget_after_enter");
}
```

Snapshots are stored in `tests/snapshots/` by default.

Update snapshots when output intentionally changes:
```bash
REVUE_UPDATE_SNAPSHOTS=1 cargo test
```

### SnapshotManager

For direct snapshot control:

```rust
use revue::testing::SnapshotManager;

let manager = SnapshotManager::new();              // tests/snapshots/
let manager = SnapshotManager::with_dir("custom"); // Custom directory

manager.assert_snapshot("name", "content");
manager.assert_buffer_snapshot("name", &buffer);

// Query
manager.snapshot_exists("name");
manager.list_snapshots();
manager.delete_snapshot("name");
```

## Visual Regression Testing

Capture and compare visual output including styles and colors:

```rust
use revue::testing::{VisualTest, VisualTestConfig};

#[test]
fn test_visual_regression() {
    let test = VisualTest::new("dashboard_initial")
        .group("dashboard");

    let mut app = TestApp::new(Dashboard::new());
    let pilot = Pilot::new(&mut app);

    // Compare against golden file
    test.assert_matches(pilot.buffer());
}
```

### Configuration

```rust
let config = VisualTestConfig::with_dir("tests/golden")
    .tolerance(5)               // Color tolerance (0=exact, 255=any)
    .generate_diff(true)        // Generate diff output on failure
    .include_styles(true)       // Compare bold, italic, underline
    .include_colors(true);      // Compare fg/bg colors

let test = VisualTest::with_config("widget_test", config);
```

Update golden files when visuals intentionally change:
```bash
REVUE_UPDATE_VISUALS=1 cargo test
```

### Visual Capture

Capture detailed visual state for comparison:

```rust
use revue::testing::VisualCapture;

let capture = VisualCapture::from_buffer(&buffer, true, true);
let cell = capture.get(5, 3);  // CapturedCell with symbol, colors, styles

// Compare two captures
let diff = capture.diff(&other_capture, 0);
if diff.has_differences() {
    println!("{}", diff.summary());
}
```

## Action Sequences

Create reusable sequences of test actions:

```rust
use revue::testing::ActionSequence;

let login_sequence = ActionSequence::new()
    .type_text("admin")
    .press(Key::Tab)
    .type_text("password")
    .press(Key::Enter);

// Reuse across tests
let nav_sequence = ActionSequence::new()
    .press(Key::Down)
    .press(Key::Down)
    .press(Key::Enter);
```

### Action Types

```rust
use revue::testing::{Action, KeyAction, MouseAction};

// Key actions
KeyAction::press(Key::Enter)
KeyAction::press_ctrl(Key::Char('c'))
KeyAction::press_alt(Key::Char('x'))
KeyAction::type_text("hello")

// Mouse actions
MouseAction::click(10, 5)
MouseAction::double_click(10, 5)
MouseAction::right_click(10, 5)
MouseAction::drag(0, 0, 10, 10)
MouseAction::move_to(5, 5)

// Timing
Action::Wait(Duration::from_millis(100))
Action::Resize(120, 40)
```

## Async Testing

### AsyncPilot

Test async views:

```rust
use revue::testing::AsyncPilot;

#[tokio::test]
async fn test_async_view() {
    AsyncPilot::run(MyAsyncView::new(), |mut pilot| async move {
        pilot.press_key(Key::Enter);
        pilot.wait_until_async(|buf| buf.contains("Loaded")).await;
        pilot.assert_contains("Data loaded");
    }).await;
}
```

### Waiting

```rust
// Wait fixed duration
pilot.wait_ms(500);

// Wait until condition
pilot.wait_until(|buffer| buffer.contains("Ready"));
```

## Mock Utilities

### MockTerminal

Virtual terminal for testing without a real terminal:

```rust
use revue::testing::mock_terminal;

let terminal = mock_terminal(80, 24);
assert_eq!(terminal.size(), (80, 24));
let buffer = terminal.buffer();
terminal.resize(120, 40);
```

### MockTime

Control time progression in tests:

```rust
use revue::testing::mock_time;

let time = mock_time();
assert_eq!(time.elapsed_ms(), 0);

time.advance_ms(500);
assert_eq!(time.elapsed_ms(), 500);

time.advance_secs(2);
assert_eq!(time.elapsed_ms(), 2500);

time.reset();
assert_eq!(time.elapsed_ms(), 0);
```

### MockState

Track state changes:

```rust
use revue::testing::MockState;

let state = MockState::new(0);
assert_eq!(state.value(), 0);

state.set(42);
assert_eq!(state.value(), 42);
assert_eq!(state.change_count(), 1);

state.update(|v| *v += 1);
assert_eq!(state.value(), 43);
assert_eq!(state.change_count(), 2);

state.reset_count();
assert_eq!(state.change_count(), 0);
```

### EventSimulator

Build event sequences fluently:

```rust
use revue::testing::simulate_user;

let mut sim = simulate_user()
    .type_text("hello")
    .tab()
    .type_text("world")
    .enter()
    .click(10, 5)
    .wait_ms(100);

while let Some(event) = sim.poll_event() {
    // Process simulated events
}
```

### RenderCapture

Capture and inspect render output:

```rust
use revue::testing::capture_render;

let mut capture = capture_render(80, 24);
// ... render into capture.buffer_mut() ...

assert!(capture.contains("Hello"));
let pos = capture.find("World");
let ch = capture.char_at(0, 0);

// Diff two captures
let changes = capture.diff(&other);
```

## CI Integration

Revue detects CI environments automatically:

```rust
use revue::testing::{CiEnvironment, TestReport};

let ci = CiEnvironment::detect();
// ci.provider: GitHubActions, GitLabCi, CircleCi, etc.
// ci.is_ci: true in CI, false locally
// ci.branch: current branch name
// ci.commit: commit SHA
// ci.pr_number: PR number if applicable
```

### Test Reports

Generate reports for CI:

```rust
let mut report = TestReport::new();

report.add_passed("widget_render");
report.add_failed("modal_close", "Expected 'Closed' but found 'Open'");
report.add_metadata("duration", "1.5s");

println!("{}", report.summary());
// Passed: 1, Failed: 1, Total: 2

// Output as markdown
let md = report.to_markdown();

// CI-specific output
let ci = CiEnvironment::detect();
report.write_summary(&ci);
report.save_artifacts(&ci).ok();
```

## Testing Patterns

### Testing Event Handlers

```rust
#[test]
fn test_navigation() {
    let mut app = TestApp::new(List::new(items));
    let mut pilot = Pilot::new(&mut app);

    pilot.press_down();
    pilot.assert_contains("Item 2");  // Second item highlighted

    pilot.press_up();
    pilot.assert_contains("Item 1");  // Back to first
}
```

### Testing Forms

```rust
#[test]
fn test_form_validation() {
    let mut app = TestApp::new(LoginForm::new());
    let mut pilot = Pilot::new(&mut app);

    // Empty submit shows error
    pilot.press_enter();
    pilot.assert_contains("Required");

    // Fill form
    pilot
        .type_text("user@example.com")
        .press_tab()
        .type_text("password123")
        .press_enter();

    pilot.assert_contains("Success");
}
```

### Testing Async Operations

```rust
#[test]
fn test_async_loading() {
    let mut app = TestApp::new(DataLoader::new());
    let mut pilot = Pilot::new(&mut app);

    pilot.assert_contains("Loading");
    pilot.wait_ms(1000);
    pilot.assert_contains("Data loaded");
}
```

### Testing Resize

```rust
#[test]
fn test_responsive_layout() {
    let mut app = TestApp::new(Dashboard::new());
    let mut pilot = Pilot::new(&mut app);

    // Full width
    pilot.assert_contains("Sidebar");

    // Narrow width - sidebar collapses
    pilot.resize(40, 24);
    pilot.assert_not_contains("Sidebar");
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

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `REVUE_UPDATE_SNAPSHOTS=1` | Update snapshot files instead of comparing |
| `REVUE_UPDATE_VISUALS=1` | Update visual golden files instead of comparing |

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

### 5. Use MockTime for Timing-Dependent Tests

```rust
#[test]
fn test_animation() {
    let time = mock_time();
    let mut app = TestApp::new(AnimatedWidget::new(&time));
    let mut pilot = Pilot::new(&mut app);

    pilot.assert_contains("Frame 1");
    time.advance_ms(500);
    pilot.assert_contains("Frame 2");
}
```

## See Also

- [State Management Guide](state.md) - Signals and reactive primitives
- [Store Guide](store.md) - Testing stores with MockState
