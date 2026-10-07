//! Stepper widget for multi-step processes
//!
//! Shows progress through a series of steps with status indicators.

mod navigation;
mod render;
mod types;

pub use types::{Step, StepStatus, StepperOrientation, StepperStyle};

use crate::style::Color;
use crate::widget::theme::SEPARATOR_COLOR;
use crate::widget::traits::WidgetProps;
use crate::{impl_props_builders, impl_styled_view};

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
    /// The color the builder named, if it named one - see #656.
    pending_color: Option<Color>,
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
            pending_color: None,
            error_color: Color::RED,
            connector_color: SEPARATOR_COLOR,
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
    ///
    /// Numbers replace the status icons only in the
    /// [`Numbered`](StepperStyle::Numbered) style; `numbers(false)` keeps the
    /// icons there too.
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
}

impl Default for Stepper {
    fn default() -> Self {
        Self::new()
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
