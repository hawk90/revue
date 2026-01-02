//! Stepper widget for multi-step processes
//!
//! Shows progress through a series of steps with status indicators.

use super::traits::{View, RenderContext, WidgetProps};
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::{impl_styled_view, impl_props_builders};

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
        self.steps.last().is_some_and(|s| s.status == StepStatus::Completed)
    }

    /// Get progress as percentage
    pub fn progress(&self) -> f64 {
        if self.steps.is_empty() {
            return 0.0;
        }
        let completed = self.steps.iter().filter(|s| s.status == StepStatus::Completed).count();
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
            if matches!(self.style, StepperStyle::Connected | StepperStyle::Progress) && i < step_count - 1 {
                let connector_start = x + 2;
                let connector_end = area.x + ((i + 1) * step_width) as u16;

                for cx in connector_start..connector_end {
                    let ch = if matches!(self.style, StepperStyle::Progress) && step.status == StepStatus::Completed {
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
    use crate::render::Buffer;
    use crate::layout::Rect;

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
        let mut s = Stepper::new()
            .add_step("Step 1")
            .current(0);

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
        let s = stepper()
            .step(step("Test").description("Testing"));

        assert_eq!(s.len(), 1);
    }
}
