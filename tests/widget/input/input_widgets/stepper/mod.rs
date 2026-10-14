//! Stepper widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{
    step, stepper, Step, StepStatus, Stepper, StepperOrientation, StepperStyle, View,
};

mod view;

/// Each row of `s` drawn in a `w` x `h` area, trimmed at the end
fn rows(s: &Stepper, w: u16, h: u16) -> Vec<String> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    s.render(&mut ctx);
    (0..h)
        .map(|y| {
            (0..w)
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .filter(|&c| c != '\0')
                .collect::<String>()
                .trim_end()
                .to_string()
        })
        .collect()
}

/// The fg of the cell at (x, y)
fn fg_at(s: &Stepper, w: u16, h: u16, x: u16, y: u16) -> Option<Color> {
    let mut buffer = Buffer::new(w, h);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, w, h));
    s.render(&mut ctx);
    buffer.get(x, y).unwrap().fg
}

// =============================================================================
// StepStatus Tests
// =============================================================================

#[test]
fn test_step_status_default() {
    let status = StepStatus::default();
    assert_eq!(status, StepStatus::Pending);
}

#[test]
fn test_step_status_clone() {
    let status = StepStatus::Active;
    let cloned = status;
    assert_eq!(status, cloned);
}

#[test]
fn test_step_status_copy() {
    let status1 = StepStatus::Completed;
    let status2 = status1;
    assert_eq!(status1, StepStatus::Completed);
    assert_eq!(status2, StepStatus::Completed);
}

#[test]
fn test_step_status_partial_eq() {
    assert_eq!(StepStatus::Pending, StepStatus::Pending);
    assert_ne!(StepStatus::Pending, StepStatus::Active);
    assert_eq!(StepStatus::Error, StepStatus::Error);
}

#[test]
fn test_step_status_all_variants() {
    let variants = [
        StepStatus::Pending,
        StepStatus::Active,
        StepStatus::Completed,
        StepStatus::Error,
        StepStatus::Skipped,
    ];
    assert_eq!(variants.len(), 5);
}

#[test]
fn test_step_status_icons() {
    // Each status has its own icon
    for (status, icon) in [
        (StepStatus::Pending, '○'),
        (StepStatus::Active, '●'),
        (StepStatus::Completed, '✓'),
        (StepStatus::Error, '✗'),
        (StepStatus::Skipped, '⊘'),
    ] {
        let s = Stepper::new().step(Step::new("X").status(status));
        assert_eq!(rows(&s, 10, 2)[0], icon.to_string(), "{status:?}");
    }
}

// =============================================================================
// Step Tests
// =============================================================================

#[test]
fn test_step_new() {
    let s = Step::new("Step 1");
    assert_eq!(s.title, "Step 1");
    assert_eq!(s.status, StepStatus::Pending);
}

#[test]
fn test_step_new_with_string() {
    let s = Step::new("Test Title".to_string());
    assert_eq!(s.title, "Test Title");
}

#[test]
fn test_step_description() {
    let s = Step::new("Title").description("Description");
    assert_eq!(s.description, Some("Description".to_string()));

    let step = revue::widget::Step::new("Step 1").description("Do something");
    assert_eq!(step.description, Some("Do something".to_string()));
}

#[test]
fn test_step_description_string() {
    let s = Step::new("Title").description("Description".to_string());
    assert_eq!(s.description, Some("Description".to_string()));
}

#[test]
fn test_step_status_builder() {
    let s = Step::new("Title").status(StepStatus::Active);
    assert_eq!(s.status, StepStatus::Active);
}

#[test]
fn test_step_icon() {
    let s = Step::new("Title").icon('🔧');
    assert_eq!(s.icon, Some('🔧'));

    let step = revue::widget::Step::new("Step 1").icon('1');
    assert_eq!(step.icon, Some('1'));
}

#[test]
fn test_step_complete() {
    let s = Step::new("Title").complete();
    assert_eq!(s.status, StepStatus::Completed);

    let step = revue::widget::Step::new("Step 1");
    assert_eq!(step.status, StepStatus::Pending);

    let step = step.complete();
    assert_eq!(step.status, StepStatus::Completed);
}

#[test]
fn test_step_active() {
    let s = Step::new("Title").active();
    assert_eq!(s.status, StepStatus::Active);

    let step = revue::widget::Step::new("Step 1");
    assert_eq!(step.status, StepStatus::Pending);

    let step = step.active();
    assert_eq!(step.status, StepStatus::Active);
}

#[test]
fn test_step_builder_chain() {
    let s = Step::new("Install")
        .description("Installing packages")
        .status(StepStatus::Active)
        .icon('📦');

    assert_eq!(s.description, Some("Installing packages".to_string()));
    assert_eq!(s.status, StepStatus::Active);
    assert_eq!(s.icon, Some('📦'));
}

#[test]
fn test_step_display_icon_with_custom() {
    // display_icon is private; verify indirectly: a step with a custom icon
    // renders without panic and the icon field is set
    let s = Step::new("Title").icon('🔧');
    assert_eq!(s.icon, Some('🔧'));

    let mut buf = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buf, area);
    Stepper::new().step(s).render(&mut ctx);
}

#[test]
fn test_step_display_icon_default() {
    // display_icon is private; verify indirectly: completed step renders without panic
    let s = Step::new("Title").status(StepStatus::Completed);
    assert_eq!(s.status, StepStatus::Completed);
    assert_eq!(s.icon, None);

    let mut buf = Buffer::new(10, 2);
    let area = Rect::new(0, 0, 10, 2);
    let mut ctx = RenderContext::new(&mut buf, area);
    Stepper::new().step(s).render(&mut ctx);
}

#[test]
fn test_step_clone() {
    let s1 = Step::new("Title")
        .description("Desc")
        .status(StepStatus::Active)
        .icon('🔧');
    let s2 = s1.clone();
    assert_eq!(s1.title, s2.title);
    assert_eq!(s1.description, s2.description);
    assert_eq!(s1.status, s2.status);
    assert_eq!(s1.icon, s2.icon);
}

#[test]
fn test_step_debug() {
    let s = Step::new("Test");
    let debug_str = format!("{:?}", s);
    assert!(debug_str.contains("Test"));
}

// =============================================================================
// StepperOrientation Tests
// =============================================================================

#[test]
fn test_stepper_orientation_default() {
    let orientation = StepperOrientation::default();
    assert_eq!(orientation, StepperOrientation::Horizontal);
}

#[test]
fn test_stepper_orientation_clone() {
    let orientation = StepperOrientation::Vertical;
    let cloned = orientation;
    assert_eq!(orientation, cloned);
}

#[test]
fn test_stepper_orientation_copy() {
    let orientation1 = StepperOrientation::Horizontal;
    let orientation2 = orientation1;
    assert_eq!(orientation1, StepperOrientation::Horizontal);
    assert_eq!(orientation2, StepperOrientation::Horizontal);
}

#[test]
fn test_stepper_orientation_partial_eq() {
    assert_eq!(
        StepperOrientation::Horizontal,
        StepperOrientation::Horizontal
    );
    assert_ne!(StepperOrientation::Horizontal, StepperOrientation::Vertical);
}

#[test]
fn test_stepper_orientation_all_variants() {
    let variants = [StepperOrientation::Horizontal, StepperOrientation::Vertical];
    assert_eq!(variants.len(), 2);
}

// =============================================================================
// StepperStyle Tests
// =============================================================================

#[test]
fn test_stepper_style_default() {
    let style = StepperStyle::default();
    assert_eq!(style, StepperStyle::Dots);
}

#[test]
fn test_stepper_style_clone() {
    let style = StepperStyle::Numbered;
    let cloned = style;
    assert_eq!(style, cloned);
}

#[test]
fn test_stepper_style_copy() {
    let style1 = StepperStyle::Connected;
    let style2 = style1;
    assert_eq!(style1, StepperStyle::Connected);
    assert_eq!(style2, StepperStyle::Connected);
}

#[test]
fn test_stepper_style_partial_eq() {
    assert_eq!(StepperStyle::Dots, StepperStyle::Dots);
    assert_ne!(StepperStyle::Dots, StepperStyle::Numbered);
}

#[test]
fn test_stepper_style_all_variants() {
    let variants = [
        StepperStyle::Dots,
        StepperStyle::Numbered,
        StepperStyle::Connected,
        StepperStyle::Progress,
    ];
    assert_eq!(variants.len(), 4);
}

// =============================================================================
// Stepper Constructor Tests
// =============================================================================

#[test]
fn test_stepper_new() {
    let s = Stepper::new();
    assert!(s.is_empty());
    // Verify default orientation and style by checking that horizontal rendering
    // is used (smoke test - should not panic with default settings)
    let mut buf = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    s.render(&mut ctx);

    let stepper = Stepper::new();

    assert_eq!(stepper.len(), 0);
    assert!(stepper.is_empty());
}

#[test]
fn test_stepper_default() {
    let s = Stepper::default();
    assert!(s.is_empty());
}

#[test]
fn test_stepper_with_step_object() {
    let s = Stepper::new().step(Step::new("Custom Step"));
    assert_eq!(s.len(), 1);
}

#[test]
fn test_stepper_add_step() {
    let s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(0);

    assert_eq!(s.len(), 3);
    // current(0) sets index 0 as Active
    assert_eq!(s.current_step().unwrap().status, StepStatus::Active);
    // No steps completed yet: progress is 0.0
    assert_eq!(s.progress(), 0.0);

    let stepper = Stepper::new().add_step("Step 1");

    assert_eq!(stepper.len(), 1);
}

#[test]
fn test_stepper_steps_builder() {
    let s = Stepper::new().steps(vec![Step::new("A"), Step::new("B"), Step::new("C")]);
    assert_eq!(s.len(), 3);
}

// =============================================================================
// Stepper Builder Tests
// =============================================================================

#[test]
fn test_stepper_orientation_horizontal() {
    // Side by side: half the width each, joined by the default connector,
    // titles on the row below
    let s = Stepper::new().horizontal().add_step("A").add_step("B");
    assert_eq!(rows(&s, 20, 2), vec!["○ ────────○", "A         B"]);
}

#[test]
fn test_stepper_orientation_vertical() {
    // One under the other, the title after the indicator, a connector
    // between
    let s = Stepper::new().vertical().add_step("A").add_step("B");
    assert_eq!(rows(&s, 20, 3), vec!["○  A", "│", "○  B"]);
}

#[test]
fn test_stepper_orientation_builder() {
    let s = Stepper::new().add_step("A").add_step("B");
    assert_eq!(
        rows(&s.clone().orientation(StepperOrientation::Vertical), 20, 3),
        rows(&s.clone().vertical(), 20, 3)
    );
    assert_eq!(
        rows(
            &s.clone().orientation(StepperOrientation::Horizontal),
            20,
            3
        ),
        rows(&s, 20, 3)
    );
}

#[test]
fn test_stepper_style_builder() {
    // Numbered: the step's number instead of its icon
    let s = Stepper::new().style(StepperStyle::Numbered).add_step("A");
    assert_eq!(rows(&s, 10, 2), vec!["1", "A"]);
}

#[test]
fn test_stepper_descriptions_show() {
    let s = Stepper::new()
        .descriptions(true)
        .step(Step::new("A").description("desc"));
    assert_eq!(rows(&s, 20, 3), vec!["○", "A", "desc"]);
}

#[test]
fn test_stepper_descriptions_hide() {
    let s = Stepper::new()
        .descriptions(false)
        .step(Step::new("A").description("desc"));
    assert_eq!(rows(&s, 20, 3), vec!["○", "A", ""]);
}

#[test]
fn test_stepper_numbers_show() {
    // Numbers show only in the Numbered style
    let s = Stepper::new().numbers(true).add_step("A");
    assert_eq!(rows(&s, 10, 2)[0], "○");
    assert_eq!(rows(&s.style(StepperStyle::Numbered), 10, 2)[0], "1");
}

#[test]
fn test_stepper_numbers_hide() {
    // Turned off, the Numbered style shows icons
    let s = Stepper::new()
        .numbers(false)
        .style(StepperStyle::Numbered)
        .add_step("A");
    assert_eq!(rows(&s, 10, 2)[0], "○");
}

#[test]
fn test_stepper_active_color() {
    let s = Stepper::new()
        .active_color(Color::MAGENTA)
        .add_step("A")
        .add_step("B")
        .current(0);
    // The active step's indicator and title; the next one is not active
    assert_eq!(fg_at(&s, 20, 2, 0, 0), Some(Color::MAGENTA));
    assert_eq!(fg_at(&s, 20, 2, 0, 1), Some(Color::MAGENTA));
    assert_ne!(fg_at(&s, 20, 2, 10, 0), Some(Color::MAGENTA));
}

#[test]
fn test_stepper_completed_color() {
    let s = Stepper::new()
        .completed_color(Color::MAGENTA)
        .step(Step::new("A").complete());
    assert_eq!(rows(&s, 10, 2)[0], "✓");
    assert_eq!(fg_at(&s, 10, 2, 0, 0), Some(Color::MAGENTA));
}

#[test]
fn test_stepper_current() {
    let s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    assert_eq!(s.current_step().unwrap().title, "Step 1");
    assert_eq!(s.current_step().unwrap().status, StepStatus::Active);

    let stepper = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    assert_eq!(
        stepper.current_step().map(|s| s.title.as_str()),
        Some("Step 1")
    );
}

#[test]
fn test_stepper_current_clamped() {
    let s = Stepper::new().add_step("A").add_step("B").current(100);
    // current is clamped to last valid index (1)
    assert_eq!(s.current_step().unwrap().title, "B");
}

// =============================================================================
// Stepper Navigation Tests
// =============================================================================

#[test]
fn test_stepper_next_step() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(0);

    assert!(s.next_step());
    // After next_step: current moves to Step 2, Step 1 becomes Completed
    assert_eq!(s.current_step().unwrap().title, "Step 2");
    assert_eq!(s.current_step().unwrap().status, StepStatus::Active);
    // Step 1 is completed: progress = 1/3
    assert!((s.progress() - 1.0 / 3.0).abs() < f64::EPSILON);
}

#[test]
fn test_stepper_next_step_at_end() {
    let mut s = Stepper::new().add_step("A").current(0);
    assert!(!s.next_step());
    assert_eq!(s.current_step().unwrap().title, "A");
}

#[test]
fn test_stepper_prev() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(1);

    assert!(s.prev());
    assert_eq!(s.current_step().unwrap().title, "Step 1");
}

#[test]
fn test_stepper_prev_at_start() {
    let mut s = Stepper::new().add_step("A").current(0);
    assert!(!s.prev());
    assert_eq!(s.current_step().unwrap().title, "A");
}

#[test]
fn test_stepper_go_to_valid() {
    let mut s = Stepper::new()
        .add_step("A")
        .add_step("B")
        .add_step("C")
        .current(0);
    s.go_to(2);
    assert_eq!(s.current_step().unwrap().title, "C");
}

#[test]
fn test_stepper_go_to_invalid() {
    let mut s = Stepper::new().add_step("A").add_step("B").current(0);
    s.go_to(100);
    assert_eq!(s.current_step().unwrap().title, "A");
}

#[test]
fn test_stepper_complete_current() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.complete_current();
    // Advances to Step 2
    assert_eq!(s.current_step().unwrap().title, "Step 2");
    // Step 1 is completed: progress = 1/2
    assert_eq!(s.progress(), 0.5);
}

#[test]
fn test_stepper_complete_current_at_end() {
    let mut s = Stepper::new().add_step("A").current(0);
    s.complete_current();
    // Still on A (last step), but it should be Completed
    assert_eq!(s.current_step().unwrap().title, "A");
    assert_eq!(s.current_step().unwrap().status, StepStatus::Completed);
}

#[test]
fn test_stepper_complete_current_empty() {
    let mut s = Stepper::new();
    s.complete_current();
    assert!(s.current_step().is_none());
}

// =============================================================================
// Stepper State Query Tests
// =============================================================================

#[test]
fn test_stepper_len_empty() {
    let s = Stepper::new();
    assert_eq!(s.len(), 0);
}

#[test]
fn test_stepper_len_multiple() {
    let s = Stepper::new().add_step("A").add_step("B").add_step("C");
    assert_eq!(s.len(), 3);
}

#[test]
fn test_stepper_is_empty_true() {
    let s = Stepper::new();
    assert!(s.is_empty());
}

#[test]
fn test_stepper_is_empty_false() {
    let s = Stepper::new().add_step("A");
    assert!(!s.is_empty());
}

#[test]
fn test_stepper_current_step_some() {
    let s = Stepper::new().add_step("A").current(0);
    assert!(s.current_step().is_some());
    assert_eq!(s.current_step().unwrap().title, "A");
}

#[test]
fn test_stepper_current_step_none() {
    let s = Stepper::new();
    assert!(s.current_step().is_none());
}

#[test]
fn test_stepper_is_completed_empty() {
    let s = Stepper::new();
    assert!(!s.is_completed());
}

#[test]
fn test_stepper_is_completed_last_not_completed() {
    let s = Stepper::new().add_step("A").add_step("B").current(0);
    assert!(!s.is_completed());
}

#[test]
fn test_stepper_is_completed_true() {
    let mut s = Stepper::new().add_step("Step 1").current(0);
    assert!(!s.is_completed());

    s.complete_current();
    assert!(s.is_completed());
}

#[test]
fn test_stepper_progress_empty() {
    let s = Stepper::new();
    assert_eq!(s.progress(), 0.0);

    let stepper = Stepper::new();
    assert_eq!(stepper.progress(), 0.0);
}

#[test]
fn test_stepper_progress_none_completed() {
    let s = Stepper::new()
        .add_step("A")
        .add_step("B")
        .add_step("C")
        .current(0);
    assert_eq!(s.progress(), 0.0);
}

#[test]
fn test_stepper_progress_half_completed() {
    let s = Stepper::new()
        .step(Step::new("A").complete())
        .step(Step::new("B").complete())
        .step(Step::new("C"))
        .step(Step::new("D"));

    assert_eq!(s.progress(), 0.5);
}

#[test]
fn test_stepper_progress_all_completed() {
    let s = Stepper::new()
        .step(Step::new("A").complete())
        .step(Step::new("B").complete())
        .step(Step::new("C").complete());

    assert_eq!(s.progress(), 1.0);

    let steps = vec![
        revue::widget::Step::new("Step 1").complete(),
        revue::widget::Step::new("Step 2").complete(),
        revue::widget::Step::new("Step 3").complete(),
    ];
    let stepper = Stepper::new().steps(steps);

    assert_eq!(stepper.progress(), 1.0);
}

// =============================================================================
// Stepper Status Management Tests
// =============================================================================

#[test]
fn test_stepper_mark_error() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.mark_error(1);
    s.go_to(1);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Error);
}

#[test]
fn test_stepper_mark_error_out_of_bounds() {
    let mut s = Stepper::new().add_step("A");
    s.mark_error(100);
    // Nothing changes
    assert_eq!(s.len(), 1);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Pending);
}

#[test]
fn test_stepper_skip() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.skip(1);
    s.go_to(1);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Skipped);
}

#[test]
fn test_stepper_skip_out_of_bounds() {
    let mut s = Stepper::new().add_step("A");
    s.skip(100);
    assert_eq!(s.len(), 1);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Pending);
}

#[test]
fn test_stepper_update_statuses_preserves_error() {
    // Error status is preserved through current() and go_to() calls
    let mut s = Stepper::new()
        .step(Step::new("A"))
        .step(Step::new("B").status(StepStatus::Error))
        .current(0);

    s.go_to(1);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Error);
}

#[test]
fn test_stepper_update_statuses_preserves_skipped() {
    // Skipped status is preserved through current() and go_to() calls
    let mut s = Stepper::new()
        .step(Step::new("A"))
        .step(Step::new("B").status(StepStatus::Skipped))
        .current(0);

    s.go_to(1);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Skipped);
}

// =============================================================================
// Stepper Clone and Debug Tests
// =============================================================================

#[test]
fn test_stepper_clone() {
    let s1 = Stepper::new().add_step("A").add_step("B").current(1);
    let s2 = s1.clone();
    assert_eq!(s1.len(), s2.len());
    assert_eq!(
        s1.current_step().map(|s| &s.title),
        s2.current_step().map(|s| &s.title)
    );
}

#[test]
fn test_stepper_debug() {
    let s = Stepper::new().add_step("Test");
    let debug_str = format!("{:?}", s);
    assert!(debug_str.contains("Stepper"));
}

// =============================================================================
// Helper Function Tests
// =============================================================================

#[test]
fn test_stepper_helper() {
    let s = stepper();
    assert!(s.is_empty());
    // Default orientation is Horizontal; smoke test rendering
    let mut buf = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buf, area);
    s.render(&mut ctx);
}

#[test]
fn test_step_helper_str() {
    let s = step("Title");
    assert_eq!(s.title, "Title");
}

#[test]
fn test_step_helper_string() {
    let s = step("Title".to_string());
    assert_eq!(s.title, "Title");
}

#[test]
fn test_helpers_combined() {
    let s = stepper().step(step("Test").description("Testing"));

    assert_eq!(s.len(), 1);
    assert_eq!(s.current_step().unwrap().title, "Test");
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_horizontal() {
    let s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(1);
    // 20 columns each: done, active, pending, joined by connectors
    assert_eq!(
        &rows(&s, 60, 5)[..2],
        [
            format!("✓ {}● {}○", "─".repeat(18), "─".repeat(18)),
            format!("Step 1{}Step 2{}Step 3", " ".repeat(14), " ".repeat(14)),
        ]
    );
}

#[test]
fn test_render_vertical() {
    let s = Stepper::new()
        .vertical()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);
    assert_eq!(&rows(&s, 40, 10)[..4], ["●  Step 1", "│", "○  Step 2", ""]);
}

#[test]
fn test_render_horizontal_too_narrow() {
    // Under 3 columns, nothing is drawn
    let s = Stepper::new().add_step("Step 1").add_step("Step 2");
    assert!(rows(&s, 2, 5).iter().all(|r| r.is_empty()));
    // One column each: both icons, with no room between them (#1028)
    assert_eq!(rows(&s, 3, 5)[0].matches('○').count(), 2);
}

#[test]
fn test_render_horizontal_too_short() {
    let mut buffer = Buffer::new(60, 1);
    let area = Rect::new(0, 0, 60, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new().add_step("Step 1");

    s.render(&mut ctx);
    // A zero-height area draws nothing
    assert!((0..60).all(|x| buffer.get(x, 0).unwrap().symbol == ' '));
}

#[test]
fn test_render_horizontal_empty() {
    assert!(rows(&Stepper::new(), 60, 5).iter().all(|r| r.is_empty()));
}

#[test]
fn test_render_vertical_empty() {
    assert!(rows(&Stepper::new().vertical(), 40, 10)
        .iter()
        .all(|r| r.is_empty()));
}

#[test]
fn test_render_with_description() {
    let s = Stepper::new()
        .step(Step::new("Step 1").description("This is a description"))
        .current(0)
        .descriptions(true);
    let shown = rows(&s, 60, 5);
    assert_eq!(&shown[..3], ["●", "Step 1", "This is a description"]);
    // Grayer than the title
    assert_ne!(fg_at(&s, 60, 5, 0, 2), fg_at(&s, 60, 5, 0, 1));
}

#[test]
fn test_render_without_description() {
    let s = Stepper::new()
        .descriptions(false)
        .step(Step::new("Step 1").description("Hidden"))
        .current(0);
    assert!(rows(&s, 60, 5).iter().all(|r| !r.contains("Hidden")));
}

#[test]
fn test_render_vertical_with_description() {
    let s = Stepper::new()
        .vertical()
        .descriptions(true)
        .step(Step::new("Step 1").description("Description"))
        .add_step("Step 2")
        .current(0);
    // The description takes its own row, under the title
    assert_eq!(
        &rows(&s, 40, 10)[..4],
        ["●  Step 1", "   Description", "│", "○  Step 2"]
    );
}

#[test]
fn test_render_vertical_without_description() {
    let s = Stepper::new()
        .vertical()
        .descriptions(false)
        .step(Step::new("Step 1").description("Description"))
        .add_step("Step 2")
        .current(0);
    assert_eq!(&rows(&s, 40, 10)[..3], ["●  Step 1", "│", "○  Step 2"]);
}

// =============================================================================
// Style-Specific Rendering Tests
// =============================================================================

#[test]
fn test_render_style_dots() {
    let s = Stepper::new()
        .style(StepperStyle::Dots)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);
    // Icons, no connector
    assert_eq!(rows(&s, 20, 2)[0], "●         ○");
}

#[test]
fn test_render_style_numbered() {
    let s = Stepper::new()
        .style(StepperStyle::Numbered)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);
    assert_eq!(rows(&s, 20, 2)[0], "1         2");
}

#[test]
fn test_render_style_connected() {
    let s = Stepper::new()
        .style(StepperStyle::Connected)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);
    // A thin line from after the first indicator to the next step
    assert_eq!(rows(&s, 20, 2)[0], format!("● {}○", "─".repeat(8)));
}

#[test]
fn test_render_style_progress() {
    let s = Stepper::new()
        .style(StepperStyle::Progress)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(1);
    // Past a completed step the line is heavy, in the completed color
    assert_eq!(rows(&s, 20, 2)[0], format!("✓ {}●", "━".repeat(8)));
    assert_eq!(fg_at(&s, 20, 2, 3, 0), fg_at(&s, 20, 2, 0, 0));
}

// =============================================================================
// Integration Tests
// =============================================================================

#[test]
fn test_full_workflow() {
    let mut s = Stepper::new()
        .add_step("Install")
        .add_step("Configure")
        .add_step("Test")
        .add_step("Deploy")
        .current(0);

    assert_eq!(s.progress(), 0.0);
    assert!(!s.is_completed());

    s.complete_current();
    assert_eq!(s.current_step().unwrap().title, "Configure");
    assert_eq!(s.progress(), 0.25);

    s.complete_current();
    assert_eq!(s.current_step().unwrap().title, "Test");
    assert_eq!(s.progress(), 0.5);

    s.mark_error(3);
    s.go_to(3);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Error);

    s.go_to(1);
    assert_eq!(s.current_step().unwrap().title, "Configure");

    assert!(s.next_step());
    assert_eq!(s.current_step().unwrap().title, "Test");

    assert!(s.next_step());
    assert_eq!(s.current_step().unwrap().title, "Deploy");
}

#[test]
fn test_builder_pattern_chain() {
    let s = Stepper::new()
        .orientation(StepperOrientation::Vertical)
        .style(StepperStyle::Progress)
        .descriptions(false)
        .numbers(false)
        .active_color(Color::YELLOW)
        .completed_color(Color::GREEN)
        .add_step("Start")
        .add_step("Middle")
        .add_step("End")
        .current(0);

    assert_eq!(s.len(), 3);
    assert_eq!(s.current_step().unwrap().status, StepStatus::Active);

    // Smoke test: rendering should not panic
    let mut buf = Buffer::new(60, 10);
    let area = Rect::new(0, 0, 60, 10);
    let mut ctx = RenderContext::new(&mut buf, area);
    s.render(&mut ctx);
}

#[test]
fn test_step_builder() {
    let step = revue::widget::Step::new("Step 1")
        .description("First step")
        .status(StepStatus::Completed)
        .icon('✓');

    // Step was created successfully
    assert_eq!(step.title, "Step 1");
}

#[test]
fn test_stepper_builder_with_step() {
    let step = revue::widget::Step::new("Step 1");
    let stepper = Stepper::new().step(step);

    assert_eq!(stepper.len(), 1);
    assert!(!stepper.is_empty());
}

#[test]
fn test_stepper_add_multiple_steps() {
    let stepper = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3");

    // In order, each pending
    assert_eq!(stepper.current_step().unwrap().title, "Step 1");
    assert_eq!(stepper.current_step().unwrap().status, StepStatus::Pending);
    assert!((stepper.progress() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_stepper_with_steps_vec() {
    let steps = vec![
        revue::widget::Step::new("Step 1"),
        revue::widget::Step::new("Step 2"),
    ];
    let stepper = Stepper::new().steps(steps);

    assert_eq!(stepper.len(), 2);
}

#[test]
fn test_stepper_descriptions() {
    // On by default
    let s = Stepper::new().step(Step::new("A").description("d"));
    assert!(rows(&s, 10, 3).iter().any(|r| r == "d"));
    assert!(!rows(&s.descriptions(false), 10, 3).iter().any(|r| r == "d"));
}

#[test]
fn test_stepper_numbers() {
    // On by default, for the Numbered style
    let s = Stepper::new().style(StepperStyle::Numbered).add_step("A");
    assert_eq!(rows(&s.clone().numbers(true), 10, 2), rows(&s, 10, 2));
}

#[test]
fn test_stepper_is_empty() {
    let stepper = Stepper::new();
    assert!(stepper.is_empty());

    let stepper = stepper.add_step("Step 1");
    assert!(!stepper.is_empty());
}

#[test]
fn test_stepper_len() {
    let stepper = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3");

    assert_eq!(stepper.len(), 3);
}

#[test]
fn test_stepper_progress() {
    let stepper = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(1);

    // Progress at step 1 of 3 = 1/3 = ~0.33
    let progress = stepper.progress();
    assert!(progress > 0.0 && progress < 1.0);
}

#[test]
fn test_stepper_is_completed() {
    let stepper = Stepper::new().add_step("Step 1");
    assert!(!stepper.is_completed());

    let steps = vec![
        revue::widget::Step::new("Step 1").complete(),
        revue::widget::Step::new("Step 2").complete(),
    ];
    let stepper = Stepper::new().steps(steps);
    assert!(stepper.is_completed());
}

#[test]
fn test_step_status_error() {
    let step = revue::widget::Step::new("Step 1").status(StepStatus::Error);
    assert_eq!(step.status, StepStatus::Error);
}
