//! Stepper widget for multi-step processes
//!
//! Shows progress through a series of steps with status indicators.

use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Step status
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StepStatus {
    /// Step not started
    #[default]
    Pending,
    /// Step in progress
    Active,
    /// Step completed
    Completed,
    /// Step has error
    Error,
    /// Step skipped
    Skipped,
}

impl StepStatus {
    fn icon(&self) -> char {
        match self {
            StepStatus::Pending => '○',
            StepStatus::Active => '●',
            StepStatus::Completed => '✓',
            StepStatus::Error => '✗',
            StepStatus::Skipped => '⊘',
        }
    }
}

/// Step definition
#[derive(Clone, Debug)]
pub struct Step {
    /// Step title
    pub title: String,
    /// Step description
    pub description: Option<String>,
    /// Step status
    pub status: StepStatus,
    /// Custom icon
    pub icon: Option<char>,
}

impl Step {
    /// Create a new step
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            description: None,
            status: StepStatus::Pending,
            icon: None,
        }
    }

    /// Set description
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set status
    pub fn status(mut self, status: StepStatus) -> Self {
        self.status = status;
        self
    }

    /// Set custom icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Mark as completed
    pub fn complete(mut self) -> Self {
        self.status = StepStatus::Completed;
        self
    }

    /// Mark as active
    pub fn active(mut self) -> Self {
        self.status = StepStatus::Active;
        self
    }

    /// Get display icon
    fn display_icon(&self) -> char {
        self.icon.unwrap_or_else(|| self.status.icon())
    }
}

/// Stepper orientation
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StepperOrientation {
    /// Horizontal steps
    #[default]
    Horizontal,
    /// Vertical steps
    Vertical,
}

/// Stepper style
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum StepperStyle {
    /// Simple dots
    #[default]
    Dots,
    /// Numbered steps
    Numbered,
    /// With connector lines
    Connected,
    /// Progress bar style
    Progress,
}

/// Stepper widget
#[derive(Clone, Debug)]
pub struct Stepper {
    /// Steps
    steps: Vec<Step>,
    /// Current step index
    current: usize,
    /// Orientation
    orientation: StepperOrientation,
    /// Style
    style: StepperStyle,
    /// Show descriptions
    show_descriptions: bool,
    /// Active color
    active_color: Color,
    /// Completed color
    completed_color: Color,
    /// Pending color
    pending_color: Color,
    /// Error color
    error_color: Color,
    /// Connector color
    connector_color: Color,
    /// Show step numbers
    show_numbers: bool,
    /// Widget properties
    props: WidgetProps,
}

impl Stepper {
    /// Create a new stepper
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            current: 0,
            orientation: StepperOrientation::Horizontal,
            style: StepperStyle::Connected,
            show_descriptions: true,
            active_color: Color::CYAN,
            completed_color: Color::GREEN,
            pending_color: Color::rgb(100, 100, 100),
            error_color: Color::RED,
            connector_color: Color::rgb(60, 60, 60),
            show_numbers: true,
            props: WidgetProps::new(),
        }
    }

    /// Add a step
    pub fn step(mut self, step: Step) -> Self {
        self.steps.push(step);
        self
    }

    /// Add step from string
    pub fn add_step(mut self, title: impl Into<String>) -> Self {
        self.steps.push(Step::new(title));
        self
    }

    /// Set all steps
    pub fn steps(mut self, steps: Vec<Step>) -> Self {
        self.steps = steps;
        self
    }

    /// Set current step
    pub fn current(mut self, index: usize) -> Self {
        self.current = index.min(self.steps.len().saturating_sub(1));
        self.update_statuses();
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: StepperOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set horizontal orientation
    pub fn horizontal(mut self) -> Self {
        self.orientation = StepperOrientation::Horizontal;
        self
    }

    /// Set vertical orientation
    pub fn vertical(mut self) -> Self {
        self.orientation = StepperOrientation::Vertical;
        self
    }

    /// Set style
    pub fn style(mut self, style: StepperStyle) -> Self {
        self.style = style;
        self
    }

    /// Show/hide descriptions
    pub fn descriptions(mut self, show: bool) -> Self {
        self.show_descriptions = show;
        self
    }

    /// Show/hide step numbers
    pub fn numbers(mut self, show: bool) -> Self {
        self.show_numbers = show;
        self
    }

    /// Set active color
    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }

    /// Set completed color
    pub fn completed_color(mut self, color: Color) -> Self {
        self.completed_color = color;
        self
    }

    /// Update step statuses based on current index
    fn update_statuses(&mut self) {
        for (i, step) in self.steps.iter_mut().enumerate() {
            if step.status != StepStatus::Error && step.status != StepStatus::Skipped {
                step.status = if i < self.current {
                    StepStatus::Completed
                } else if i == self.current {
                    StepStatus::Active
                } else {
                    StepStatus::Pending
                };
            }
        }
    }

    /// Go to next step
    pub fn next_step(&mut self) -> bool {
        if self.current < self.steps.len().saturating_sub(1) {
            self.current += 1;
            self.update_statuses();
            true
        } else {
            false
        }
    }

    /// Go to previous step
    pub fn prev(&mut self) -> bool {
        if self.current > 0 {
            self.current -= 1;
            self.update_statuses();
            true
        } else {
            false
        }
    }

    /// Go to specific step
    pub fn go_to(&mut self, index: usize) {
        if index < self.steps.len() {
            self.current = index;
            self.update_statuses();
        }
    }

    /// Complete current step and advance
    pub fn complete_current(&mut self) {
        if let Some(step) = self.steps.get_mut(self.current) {
            step.status = StepStatus::Completed;
        }
        self.next_step();
    }

    /// Mark step as error
    pub fn mark_error(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.status = StepStatus::Error;
        }
    }

    /// Mark step as skipped
    pub fn skip(&mut self, index: usize) {
        if let Some(step) = self.steps.get_mut(index) {
            step.status = StepStatus::Skipped;
        }
    }

    /// Get current step
    pub fn current_step(&self) -> Option<&Step> {
        self.steps.get(self.current)
    }

    /// Get step count
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    /// Check if completed (on last step and it's completed)
    pub fn is_completed(&self) -> bool {
        self.steps
            .last()
            .is_some_and(|s| s.status == StepStatus::Completed)
    }

    /// Get progress as percentage
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        let completed = self
            .steps
            .iter()
            .filter(|s| s.status == StepStatus::Completed)
            .count();
        completed as f64 / self.steps.len() as f64
    }

    /// Get color for step
    fn step_color(&self, step: &Step) -> Color {
        match step.status {
            StepStatus::Active => self.active_color,
            StepStatus::Completed => self.completed_color,
            StepStatus::Error => self.error_color,
            StepStatus::Pending | StepStatus::Skipped => self.pending_color,
        }
    }
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Stepper {
    crate::impl_view_meta!("Stepper");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 3 || area.height < 1 || self.steps.is_empty() {
            return;
        }

        match self.orientation {
            StepperOrientation::Horizontal => self.render_horizontal(ctx),
            StepperOrientation::Vertical => self.render_vertical(ctx),
        }
    }
}

impl Stepper {
    fn render_horizontal(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let step_count = self.steps.len();
        let available_width = area.width as usize;

        // Calculate spacing
        let step_width = available_width / step_count.max(1);

        let y = area.y;

        for (i, step) in self.steps.iter().enumerate() {
            let x = area.x + (i * step_width) as u16;
            let color = self.step_color(step);

            // Step indicator
            match self.style {
                StepperStyle::Numbered => {
                    let num = format!("{}", i + 1);
                    for (j, ch) in num.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(color);
                        if step.status == StepStatus::Active {
                            cell.modifier |= Modifier::BOLD;
                        }
                        ctx.buffer.set(x + j as u16, y, cell);
                    }
                }
                _ => {
                    let mut cell = Cell::new(step.display_icon());
                    cell.fg = Some(color);
                    if step.status == StepStatus::Active {
                        cell.modifier |= Modifier::BOLD;
                    }
                    ctx.buffer.set(x, y, cell);
                }
            }

            // Connector (except last)
            if matches!(self.style, StepperStyle::Connected | StepperStyle::Progress)
                && i < step_count - 1
            {
                let connector_start = x + 2;
                let connector_end = area.x + ((i + 1) * step_width) as u16;

                for cx in connector_start..connector_end {
                    let ch = if matches!(self.style, StepperStyle::Progress)
                        && step.status == StepStatus::Completed
                    {
                        '━'
                    } else {
                        '─'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(if step.status == StepStatus::Completed {
                        self.completed_color
                    } else {
                        self.connector_color
                    });
                    ctx.buffer.set(cx, y, cell);
                }
            }

            // Title (below indicator)
            if y + 1 < area.y + area.height {
                let max_title_len = step_width.saturating_sub(1);
                let title = if step.title.len() > max_title_len {
                    format!("{}…", &step.title[..max_title_len.saturating_sub(1)])
                } else {
                    step.title.clone()
                };

                for (j, ch) in title.chars().enumerate() {
                    if x + j as u16 >= area.x + area.width {
                        break;
                    }
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    if step.status == StepStatus::Active {
                        cell.modifier |= Modifier::BOLD;
                    }
                    ctx.buffer.set(x + j as u16, y + 1, cell);
                }
            }

            // Description (if enabled and space available)
            if self.show_descriptions && y + 2 < area.y + area.height {
                if let Some(ref desc) = step.description {
                    let max_desc_len = step_width.saturating_sub(1);
                    let desc_str = if desc.len() > max_desc_len {
                        format!("{}…", &desc[..max_desc_len.saturating_sub(1)])
                    } else {
                        desc.clone()
                    };

                    for (j, ch) in desc_str.chars().enumerate() {
                        if x + j as u16 >= area.x + area.width {
                            break;
                        }
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(120, 120, 120));
                        ctx.buffer.set(x + j as u16, y + 2, cell);
                    }
                }
            }
        }
    }

    fn render_vertical(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let mut y = area.y;

        for (i, step) in self.steps.iter().enumerate() {
            if y >= area.y + area.height {
                break;
            }

            let color = self.step_color(step);
            let x = area.x;

            // Step indicator
            let indicator = if self.show_numbers {
                format!("{}", i + 1)
            } else {
                step.display_icon().to_string()
            };

            for (j, ch) in indicator.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                if step.status == StepStatus::Active {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(x + j as u16, y, cell);
            }

            // Title
            let title_x = x + 3;
            for (j, ch) in step.title.chars().enumerate() {
                if title_x + j as u16 >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(color);
                if step.status == StepStatus::Active {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(title_x + j as u16, y, cell);
            }

            y += 1;

            // Description
            if self.show_descriptions {
                if let Some(ref desc) = step.description {
                    if y < area.y + area.height {
                        let desc_x = x + 3;
                        for (j, ch) in desc.chars().enumerate() {
                            if desc_x + j as u16 >= area.x + area.width {
                                break;
                            }
                            let mut cell = Cell::new(ch);
                            cell.fg = Some(Color::rgb(120, 120, 120));
                            ctx.buffer.set(desc_x + j as u16, y, cell);
                        }
                        y += 1;
                    }
                }
            }

            // Connector (except last)
            if matches!(self.style, StepperStyle::Connected)
                && i < self.steps.len() - 1
                && y < area.y + area.height
            {
                let mut cell = Cell::new('│');
                cell.fg = Some(if step.status == StepStatus::Completed {
                    self.completed_color
                } else {
                    self.connector_color
                });
                ctx.buffer.set(x, y, cell);
                y += 1;
            }
        }
    }
}

impl_styled_view!(Stepper);
impl_props_builders!(Stepper);

/// Helper to create a stepper
pub fn stepper() -> Stepper {
    Stepper::new()
}

/// Helper to create a step
pub fn step(title: impl Into<String>) -> Step {
    Step::new(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_step_new() {
        let s = Step::new("Step 1");
        assert_eq!(s.title, "Step 1");
        assert_eq!(s.status, StepStatus::Pending);
    }

    #[test]
    fn test_step_builder() {
        let s = Step::new("Install")
            .description("Installing packages")
            .status(StepStatus::Active)
            .icon('📦');

        assert_eq!(s.description, Some("Installing packages".to_string()));
        assert_eq!(s.status, StepStatus::Active);
        assert_eq!(s.icon, Some('📦'));
    }

    #[test]
    fn test_step_status() {
        assert_eq!(StepStatus::Pending.icon(), '○');
        assert_eq!(StepStatus::Completed.icon(), '✓');
        assert_eq!(StepStatus::Active.icon(), '●');
    }

    #[test]
    fn test_stepper_new() {
        let s = Stepper::new();
        assert!(s.is_empty());
    }

    #[test]
    fn test_stepper_steps() {
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
    fn test_stepper_navigation() {
        let mut s = Stepper::new()
            .add_step("Step 1")
            .add_step("Step 2")
            .add_step("Step 3")
            .current(0);

        assert!(s.next_step());
        assert_eq!(s.current, 1);
        assert_eq!(s.steps[0].status, StepStatus::Completed);
        assert_eq!(s.steps[1].status, StepStatus::Active);

        assert!(s.prev());
        assert_eq!(s.current, 0);
    }

    #[test]
    fn test_stepper_complete() {
        let mut s = Stepper::new()
            .add_step("Step 1")
            .add_step("Step 2")
            .current(0);

        s.complete_current();
        assert_eq!(s.current, 1);
        assert_eq!(s.steps[0].status, StepStatus::Completed);
    }

    #[test]
    fn test_stepper_progress() {
        let s = Stepper::new()
            .step(Step::new("A").complete())
            .step(Step::new("B").complete())
            .step(Step::new("C"))
            .step(Step::new("D"));

        assert_eq!(s.progress(), 0.5);
    }

    #[test]
    fn test_stepper_error() {
        let mut s = Stepper::new()
            .add_step("Step 1")
            .add_step("Step 2")
            .current(0);

        s.mark_error(1);
        assert_eq!(s.steps[1].status, StepStatus::Error);
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
    fn test_stepper_is_completed() {
        let mut s = Stepper::new().add_step("Step 1").current(0);

        assert!(!s.is_completed());

        s.complete_current();
        assert!(s.is_completed());
    }

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
        // Smoke test
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
        // Smoke test
    }

    #[test]
    fn test_helpers() {
        let s = stepper().step(step("Test").description("Testing"));

        assert_eq!(s.len(), 1);
    }

    // StepStatus enum tests
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

    // StepperOrientation enum tests
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

    // StepperStyle enum tests
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

    // Step struct additional tests
    #[test]
    fn test_step_new_with_string() {
        let s = Step::new("Test Title".to_string());
        assert_eq!(s.title, "Test Title");
        assert!(s.description.is_none());
        assert_eq!(s.status, StepStatus::Pending);
        assert!(s.icon.is_none());
    }

    #[test]
    fn test_step_new_with_str() {
        let s = Step::new("Test Title");
        assert_eq!(s.title, "Test Title");
    }

    #[test]
    fn test_step_description_str() {
        let s = Step::new("Title").description("Description");
        assert_eq!(s.description, Some("Description".to_string()));
    }

    #[test]
    fn test_step_description_string() {
        let s = Step::new("Title").description("Description".to_string());
        assert_eq!(s.description, Some("Description".to_string()));
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

    // Stepper struct additional tests
    #[test]
    fn test_stepper_default() {
        let s = Stepper::default();
        assert!(s.is_empty());
        assert_eq!(s.current, 0);
        assert_eq!(s.orientation, StepperOrientation::Horizontal);
        assert_eq!(s.style, StepperStyle::Connected);
    }

    #[test]
    fn test_stepper_with_step_object() {
        let s = Stepper::new().step(Step::new("Custom Step"));
        assert_eq!(s.len(), 1);
        assert_eq!(s.steps[0].title, "Custom Step");
    }

    #[test]
    fn test_stepper_steps_builder() {
        let s = Stepper::new().steps(vec![Step::new("A"), Step::new("B"), Step::new("C")]);
        assert_eq!(s.len(), 3);
    }

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
    fn test_stepper_current_clamped() {
        let s = Stepper::new().add_step("A").add_step("B").current(100); // Out of bounds
        assert_eq!(s.current, 1); // Clamped to max
    }

    #[test]
    fn test_stepper_next_step_at_end() {
        let mut s = Stepper::new().add_step("A").current(0);
        assert!(!s.next_step()); // Should return false
        assert_eq!(s.current, 0); // Should not advance
    }

    #[test]
    fn test_stepper_prev_at_start() {
        let mut s = Stepper::new().add_step("A").current(0);
        assert!(!s.prev()); // Should return false
        assert_eq!(s.current, 0); // Should not go back
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
        assert_eq!(s.steps[2].status, StepStatus::Active);
    }

    #[test]
    fn test_stepper_go_to_invalid() {
        let mut s = Stepper::new().add_step("A").add_step("B").current(0);
        s.go_to(100); // Out of bounds
        assert_eq!(s.current, 0); // Should not change
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
    fn test_stepper_progress_all_completed() {
        let s = Stepper::new()
            .step(Step::new("A").complete())
            .step(Step::new("B").complete())
            .step(Step::new("C").complete());
        assert_eq!(s.progress(), 1.0);
    }

    #[test]
    fn test_stepper_mark_error_out_of_bounds() {
        let mut s = Stepper::new().add_step("A");
        s.mark_error(100); // Out of bounds
                           // Should not panic, just do nothing
    }

    #[test]
    fn test_stepper_skip_out_of_bounds() {
        let mut s = Stepper::new().add_step("A");
        s.skip(100); // Out of bounds
                     // Should not panic, just do nothing
    }

    #[test]
    fn test_stepper_complete_current_at_end() {
        let mut s = Stepper::new().add_step("A").current(0);
        s.complete_current();
        assert_eq!(s.current, 0); // Can't advance past end
        assert_eq!(s.steps[0].status, StepStatus::Completed);
    }

    #[test]
    fn test_stepper_complete_current_empty() {
        let mut s = Stepper::new();
        s.complete_current(); // Should not panic
        assert_eq!(s.current, 0);
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

    // Helper function tests
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

    // Rendering edge cases
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

    // Style-specific rendering tests
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

    // Integration tests
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
}
