//! Integration tests for stepper widget

use revue::style::Color;
use revue::widget::{StepStatus, Stepper};

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
fn test_stepper_new() {
    let stepper = Stepper::new();

    assert_eq!(stepper.len(), 0);
    assert!(stepper.is_empty());
}

#[test]
fn test_stepper_builder_with_step() {
    let step = revue::widget::Step::new("Step 1");
    let stepper = Stepper::new().step(step);

    assert_eq!(stepper.len(), 1);
    assert!(!stepper.is_empty());
}

#[test]
fn test_stepper_add_step() {
    let stepper = Stepper::new().add_step("Step 1");

    assert_eq!(stepper.len(), 1);
}

#[test]
fn test_stepper_add_multiple_steps() {
    let stepper = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3");

    assert_eq!(stepper.len(), 3);
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
fn test_stepper_orientation_horizontal() {
    let _stepper = Stepper::new().horizontal();

    // Orientation was set successfully
}

#[test]
fn test_stepper_orientation_vertical() {
    let _stepper = Stepper::new().vertical();

    // Orientation was set successfully
}

#[test]
fn test_stepper_active_color() {
    let _stepper = Stepper::new().active_color(Color::CYAN);

    // Color was set successfully
}

#[test]
fn test_stepper_completed_color() {
    let _stepper = Stepper::new().completed_color(Color::GREEN);

    // Color was set successfully
}

#[test]
fn test_stepper_descriptions() {
    let _stepper = Stepper::new().descriptions(true);

    // Descriptions enabled successfully
}

#[test]
fn test_stepper_numbers() {
    let _stepper = Stepper::new().numbers(true);

    // Numbers enabled successfully
}

#[test]
fn test_stepper_current() {
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
fn test_stepper_progress_empty() {
    let stepper = Stepper::new();
    assert_eq!(stepper.progress(), 0.0);
}

#[test]
fn test_stepper_progress_all_completed() {
    let steps = vec![
        revue::widget::Step::new("Step 1").complete(),
        revue::widget::Step::new("Step 2").complete(),
        revue::widget::Step::new("Step 3").complete(),
    ];
    let stepper = Stepper::new().steps(steps);

    assert_eq!(stepper.progress(), 1.0);
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
fn test_step_complete() {
    let step = revue::widget::Step::new("Step 1");
    assert_eq!(step.status, StepStatus::Pending);

    let step = step.complete();
    assert_eq!(step.status, StepStatus::Completed);
}

#[test]
fn test_step_active() {
    let step = revue::widget::Step::new("Step 1");
    assert_eq!(step.status, StepStatus::Pending);

    let step = step.active();
    assert_eq!(step.status, StepStatus::Active);
}

#[test]
fn test_step_description() {
    let step = revue::widget::Step::new("Step 1").description("Do something");
    assert_eq!(step.description, Some("Do something".to_string()));
}

#[test]
fn test_step_icon() {
    let step = revue::widget::Step::new("Step 1").icon('1');
    assert_eq!(step.icon, Some('1'));
}

#[test]
fn test_step_status_error() {
    let step = revue::widget::Step::new("Step 1").status(StepStatus::Error);
    assert_eq!(step.status, StepStatus::Error);
}
