//! Steps, step statuses, orientation and visual styles

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
