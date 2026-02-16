//! Stepper widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::{step, stepper, Step, StepStatus, Stepper, StepperOrientation, StepperStyle, View};

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
    let cloned = status.clone();
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
    assert_eq!(StepStatus::Pending.icon(), '○');
    assert_eq!(StepStatus::Active.icon(), '●');
    assert_eq!(StepStatus::Completed.icon(), '✓');
    assert_eq!(StepStatus::Error.icon(), '✗');
    assert_eq!(StepStatus::Skipped.icon(), '⊘');
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
}

#[test]
fn test_step_complete() {
    let s = Step::new("Title").complete();
    assert_eq!(s.status, StepStatus::Completed);
}

#[test]
fn test_step_active() {
    let s = Step::new("Title").active();
    assert_eq!(s.status, StepStatus::Active);
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
    let s = Step::new("Title").icon('🔧');
    assert_eq!(s.display_icon(), '🔧');
}

#[test]
fn test_step_display_icon_default() {
    let s = Step::new("Title").status(StepStatus::Completed);
    assert_eq!(s.display_icon(), '✓');
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
    let cloned = orientation.clone();
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
    assert_eq!(StepperOrientation::Horizontal, StepperOrientation::Horizontal);
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
    let cloned = style.clone();
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
    assert_eq!(s.current, 0);
    assert_eq!(s.orientation, StepperOrientation::Horizontal);
    assert_eq!(s.style, StepperStyle::Connected);
}

#[test]
fn test_stepper_default() {
    let s = Stepper::default();
    assert!(s.is_empty());
    assert_eq!(s.current, 0);
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
    assert_eq!(s.steps[0].status, StepStatus::Active);
    assert_eq!(s.steps[1].status, StepStatus::Pending);
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
    let s = Stepper::new().horizontal();
    assert_eq!(s.orientation, StepperOrientation::Horizontal);
}

#[test]
fn test_stepper_orientation_vertical() {
    let s = Stepper::new().vertical();
    assert_eq!(s.orientation, StepperOrientation::Vertical);
}

#[test]
fn test_stepper_orientation_builder() {
    let s = Stepper::new().orientation(StepperOrientation::Vertical);
    assert_eq!(s.orientation, StepperOrientation::Vertical);
}

#[test]
fn test_stepper_style_builder() {
    let s = Stepper::new().style(StepperStyle::Numbered);
    assert_eq!(s.style, StepperStyle::Numbered);
}

#[test]
fn test_stepper_descriptions_show() {
    let s = Stepper::new().descriptions(true);
    assert!(s.show_descriptions);
}

#[test]
fn test_stepper_descriptions_hide() {
    let s = Stepper::new().descriptions(false);
    assert!(!s.show_descriptions);
}

#[test]
fn test_stepper_numbers_show() {
    let s = Stepper::new().numbers(true);
    assert!(s.show_numbers);
}

#[test]
fn test_stepper_numbers_hide() {
    let s = Stepper::new().numbers(false);
    assert!(!s.show_numbers);
}

#[test]
fn test_stepper_active_color() {
    let color = Color::MAGENTA;
    let s = Stepper::new().active_color(color);
    assert_eq!(s.active_color, color);
}

#[test]
fn test_stepper_completed_color() {
    let color = Color::BLUE;
    let s = Stepper::new().completed_color(color);
    assert_eq!(s.completed_color, color);
}

#[test]
fn test_stepper_current() {
    let s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    assert_eq!(s.current, 0);
    assert_eq!(s.steps[0].status, StepStatus::Active);
}

#[test]
fn test_stepper_current_clamped() {
    let s = Stepper::new().add_step("A").add_step("B").current(100);
    assert_eq!(s.current, 1);
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
    assert_eq!(s.current, 1);
    assert_eq!(s.steps[0].status, StepStatus::Completed);
    assert_eq!(s.steps[1].status, StepStatus::Active);
}

#[test]
fn test_stepper_next_step_at_end() {
    let mut s = Stepper::new().add_step("A").current(0);
    assert!(!s.next_step());
    assert_eq!(s.current, 0);
}

#[test]
fn test_stepper_prev() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(1);

    assert!(s.prev());
    assert_eq!(s.current, 0);
}

#[test]
fn test_stepper_prev_at_start() {
    let mut s = Stepper::new().add_step("A").current(0);
    assert!(!s.prev());
    assert_eq!(s.current, 0);
}

#[test]
fn test_stepper_go_to_valid() {
    let mut s = Stepper::new()
        .add_step("A")
        .add_step("B")
        .add_step("C")
        .current(0);
    s.go_to(2);
    assert_eq!(s.current, 2);
}

#[test]
fn test_stepper_go_to_invalid() {
    let mut s = Stepper::new().add_step("A").add_step("B").current(0);
    s.go_to(100);
    assert_eq!(s.current, 0);
}

#[test]
fn test_stepper_complete_current() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.complete_current();
    assert_eq!(s.current, 1);
    assert_eq!(s.steps[0].status, StepStatus::Completed);
}

#[test]
fn test_stepper_complete_current_at_end() {
    let mut s = Stepper::new().add_step("A").current(0);
    s.complete_current();
    assert_eq!(s.current, 0);
    assert_eq!(s.steps[0].status, StepStatus::Completed);
}

#[test]
fn test_stepper_complete_current_empty() {
    let mut s = Stepper::new();
    s.complete_current();
    assert_eq!(s.current, 0);
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
    assert_eq!(s.steps[1].status, StepStatus::Error);
}

#[test]
fn test_stepper_mark_error_out_of_bounds() {
    let mut s = Stepper::new().add_step("A");
    s.mark_error(100);
    // Should not panic
}

#[test]
fn test_stepper_skip() {
    let mut s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.skip(1);
    assert_eq!(s.steps[1].status, StepStatus::Skipped);
}

#[test]
fn test_stepper_skip_out_of_bounds() {
    let mut s = Stepper::new().add_step("A");
    s.skip(100);
    // Should not panic
}

#[test]
fn test_stepper_update_statuses_preserves_error() {
    let s = Stepper::new()
        .step(Step::new("A"))
        .step(Step::new("B").status(StepStatus::Error))
        .current(0);

    assert_eq!(s.steps[1].status, StepStatus::Error);
}

#[test]
fn test_stepper_update_statuses_preserves_skipped() {
    let s = Stepper::new()
        .step(Step::new("A"))
        .step(Step::new("B").status(StepStatus::Skipped))
        .current(0);

    assert_eq!(s.steps[1].status, StepStatus::Skipped);
}

// =============================================================================
// Stepper Clone and Debug Tests
// =============================================================================

#[test]
fn test_stepper_clone() {
    let s1 = Stepper::new().add_step("A").add_step("B").current(1);
    let s2 = s1.clone();
    assert_eq!(s1.len(), s2.len());
    assert_eq!(s1.current, s2.current);
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
    assert_eq!(s.orientation, StepperOrientation::Horizontal);
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
    assert_eq!(s.steps[0].title, "Test");
}

// =============================================================================
// Rendering Tests
// =============================================================================

#[test]
fn test_render_horizontal() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .add_step("Step 1")
        .add_step("Step 2")
        .add_step("Step 3")
        .current(1);

    s.render(&mut ctx);
    // Smoke test - should not panic
}

#[test]
fn test_render_vertical() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .vertical()
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.render(&mut ctx);
    // Smoke test - should not panic
}

#[test]
fn test_render_horizontal_too_narrow() {
    let mut buffer = Buffer::new(2, 5);
    let area = Rect::new(0, 0, 2, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new().add_step("Step 1").add_step("Step 2");

    s.render(&mut ctx);
    // Should not panic with width < 3
}

#[test]
fn test_render_horizontal_too_short() {
    let mut buffer = Buffer::new(60, 0);
    let area = Rect::new(0, 0, 60, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new().add_step("Step 1");

    s.render(&mut ctx);
    // Should not panic with height < 1
}

#[test]
fn test_render_horizontal_empty() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new();

    s.render(&mut ctx);
    // Should not panic with empty steps
}

#[test]
fn test_render_vertical_empty() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new().vertical();

    s.render(&mut ctx);
    // Should not panic with empty steps
}

#[test]
fn test_render_with_description() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .step(Step::new("Step 1").description("This is a description"))
        .current(0);

    s.render(&mut ctx);
    // Smoke test with descriptions
}

#[test]
fn test_render_without_description() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .descriptions(false)
        .step(Step::new("Step 1").description("Hidden"))
        .current(0);

    s.render(&mut ctx);
    // Smoke test without descriptions
}

#[test]
fn test_render_vertical_with_description() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .vertical()
        .step(Step::new("Step 1").description("Description"))
        .current(0);

    s.render(&mut ctx);
    // Smoke test vertical with description
}

#[test]
fn test_render_vertical_without_description() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .vertical()
        .descriptions(false)
        .add_step("Step 1")
        .current(0);

    s.render(&mut ctx);
    // Smoke test vertical without description
}

// =============================================================================
// Style-Specific Rendering Tests
// =============================================================================

#[test]
fn test_render_style_dots() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .style(StepperStyle::Dots)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.render(&mut ctx);
    // Smoke test for Dots style
}

#[test]
fn test_render_style_numbered() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .style(StepperStyle::Numbered)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.render(&mut ctx);
    // Smoke test for Numbered style
}

#[test]
fn test_render_style_connected() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .style(StepperStyle::Connected)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(0);

    s.render(&mut ctx);
    // Smoke test for Connected style
}

#[test]
fn test_render_style_progress() {
    let mut buffer = Buffer::new(60, 5);
    let area = Rect::new(0, 0, 60, 5);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let s = Stepper::new()
        .style(StepperStyle::Progress)
        .add_step("Step 1")
        .add_step("Step 2")
        .current(1);

    s.render(&mut ctx);
    // Smoke test for Progress style
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
    assert_eq!(s.current, 1);
    assert_eq!(s.progress(), 0.25);

    s.complete_current();
    assert_eq!(s.current, 2);
    assert_eq!(s.progress(), 0.5);

    s.mark_error(3);
    assert_eq!(s.steps[3].status, StepStatus::Error);

    s.go_to(1);
    assert_eq!(s.current, 1);

    assert!(s.next_step());
    assert_eq!(s.current, 2);

    assert!(s.next_step());
    assert_eq!(s.current, 3);
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

    assert_eq!(s.orientation, StepperOrientation::Vertical);
    assert_eq!(s.style, StepperStyle::Progress);
    assert!(!s.show_descriptions);
    assert!(!s.show_numbers);
    assert_eq!(s.active_color, Color::YELLOW);
    assert_eq!(s.completed_color, Color::GREEN);
    assert_eq!(s.len(), 3);
}
