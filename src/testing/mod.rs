//! Pilot testing framework for automated UI tests.
//!
//! Test your TUI applications with simulated user interactions,
//! assertions on rendered output, and snapshot testing.
//!
//! # Features
//!
//! | Feature | Description |
//! |---------|-------------|
//! | **Key Simulation** | Simulate keyboard input |
//! | **Text Search** | Assert on rendered text |
//! | **Snapshot Testing** | Compare against golden files |
//! | **Visual Regression** | Color & style comparison |
//! | **CI Integration** | GitHub Actions, GitLab CI |
//! | **Async Support** | Test async operations |
//! | **Action Sequences** | Chain multiple actions |
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use revue::testing::{Pilot, TestApp};
//! use revue::event::Key;
//!
//! #[test]
//! fn test_counter() {
//!     let mut app = TestApp::new(Counter::new());
//!     let mut pilot = Pilot::new(&mut app);
//!
//!     pilot
//!         .press(Key::Up)
//!         .press(Key::Up)
//!         .assert_contains("Count: 2")
//!         .snapshot("counter_at_2");
//! }
//! ```
//!
//! # Pilot API
//!
//! ## Key Simulation
//!
//! ```rust,ignore
//! pilot
//!     .press(Key::Enter)           // Press Enter
//!     .press(Key::Char('a'))       // Type 'a'
//!     .type_text("hello")          // Type string
//!     .press(Key::Escape);         // Press Escape
//! ```
//!
//! ## Assertions
//!
//! ```rust,ignore
//! pilot
//!     .assert_contains("Welcome")          // Text exists
//!     .assert_not_contains("Error")        // Text doesn't exist
//!     .assert_focused(".input")            // Element is focused
//!     .assert_visible(".modal");           // Element is visible
//! ```
//!
//! ## Snapshot Testing
//!
//! ```rust,ignore
//! pilot
//!     .snapshot("initial_state")           // Save/compare snapshot
//!     .press(Key::Enter)
//!     .snapshot("after_enter");
//! ```
//!
//! Snapshots are stored in `tests/snapshots/` and can be updated with:
//! ```bash
//! REVUE_UPDATE_SNAPSHOTS=1 cargo test
//! ```
//!
//! ## Waiting
//!
//! ```rust,ignore
//! pilot
//!     .wait_ms(100)                        // Wait 100ms
//!     .wait_until(|screen| {               // Wait for condition
//!         screen.contains("Loaded")
//!     });
//! ```
//!
//! # TestApp
//!
//! [`TestApp`] wraps your view for testing:
//!
//! ```rust,ignore
//! use revue::testing::{TestApp, TestConfig};
//!
//! // Default 80x24 terminal
//! let app = TestApp::new(MyView::new());
//!
//! // Custom size
//! let config = TestConfig {
//!     width: 120,
//!     height: 40,
//!     ..Default::default()
//! };
//! let app = TestApp::with_config(MyView::new(), config);
//! ```
//!
//! # Action Sequences
//!
//! Build reusable action sequences:
//!
//! ```rust,ignore
//! use revue::testing::{ActionSequence, Action};
//!
//! let login_sequence = ActionSequence::new()
//!     .action(Action::Type("admin".into()))
//!     .action(Action::Press(Key::Tab))
//!     .action(Action::Type("password".into()))
//!     .action(Action::Press(Key::Enter));
//!
//! pilot.run_sequence(&login_sequence);
//! ```
//!
//! # Async Testing
//!
//! For testing async operations:
//!
//! ```rust,ignore
//! use revue::testing::AsyncPilot;
//!
//! #[tokio::test]
//! async fn test_async_load() {
//!     let app = TestApp::new(MyAsyncView::new());
//!
//!     AsyncPilot::run(app, |pilot| async move {
//!         pilot.press(Key::Enter).await;
//!         pilot.wait_until_async(|s| s.contains("Loaded")).await;
//!         pilot.assert_contains("Data loaded");
//!     }).await;
//! }
//! ```
//!
//! # Visual Regression Testing
//!
//! For pixel-perfect UI testing with color and style comparison:
//!
//! ```rust,ignore
//! use revue::testing::{VisualTest, VisualTestConfig};
//!
//! #[test]
//! fn test_button_styles() {
//!     let test = VisualTest::new("button_normal")
//!         .group("buttons");
//!
//!     let buffer = render_my_widget();
//!     test.assert_matches(&buffer);
//! }
//! ```
//!
//! Update golden files:
//! ```bash
//! REVUE_UPDATE_VISUALS=1 cargo test
//! ```
//!
//! # CI Integration
//!
//! Detect CI environments and generate reports:
//!
//! ```rust,ignore
//! use revue::testing::{CiEnvironment, TestReport};
//!
//! let ci = CiEnvironment::detect();
//! let mut report = TestReport::new();
//!
//! // Run tests...
//! report.add_passed("button_test");
//! report.add_failed("modal_test", "Size mismatch");
//!
//! // Generate CI-specific output
//! report.write_summary(&ci);
//! report.save_artifacts(&ci).ok();
//! ```
//!
//! # Best Practices
//!
//! 1. **Name snapshots descriptively**: `"login_form_with_error"` not `"test1"`
//! 2. **Test user flows**: Simulate realistic user interactions
//! 3. **Keep tests focused**: One behavior per test
//! 4. **Use wait_until for async**: Don't rely on fixed delays
//! 5. **Use visual tests for styling**: Catch color and layout regressions
//! 6. **Run in CI**: Use `CiEnvironment` for portable test reports

mod pilot;
mod test_app;
mod assertions;
mod actions;
mod snapshot;
pub mod visual;
pub mod ci;

pub use pilot::{Pilot, AsyncPilot};
pub use test_app::TestApp;
pub use assertions::{Assertion, AssertionResult};
pub use actions::{Action, KeyAction, MouseAction, ActionSequence};
pub use snapshot::SnapshotManager;

// Visual regression testing
pub use visual::{
    VisualTest, VisualTestConfig, VisualTestResult,
    VisualCapture, VisualDiff, CapturedCell, CellDiff,
};

// CI integration
pub use ci::{CiEnvironment, CiProvider, TestReport, TestResult};

/// Test configuration
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// Screen width
    pub width: u16,
    /// Screen height
    pub height: u16,
    /// Timeout for async operations (ms)
    pub timeout_ms: u64,
    /// Enable debug output
    pub debug: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            width: 80,
            height: 24,
            timeout_ms: 5000,
            debug: false,
        }
    }
}

impl TestConfig {
    /// Create with custom size
    pub fn with_size(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            ..Default::default()
        }
    }
}
