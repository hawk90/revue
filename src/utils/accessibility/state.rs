//! State of an accessible element

/// State of an accessible element
#[derive(Clone, Debug, Default)]
pub struct AccessibleState {
    /// Element is disabled
    pub disabled: bool,
    /// Element is expanded (for trees, menus)
    pub expanded: Option<bool>,
    /// Element is selected
    pub selected: bool,
    /// Element is checked (for checkboxes, radios)
    pub checked: Option<bool>,
    /// Element is pressed (for toggle buttons)
    pub pressed: Option<bool>,
    /// Element has focus
    pub focused: bool,
    /// Element is hidden
    pub hidden: bool,
    /// Current value (for progress, sliders)
    pub value_now: Option<f64>,
    /// Minimum value
    pub value_min: Option<f64>,
    /// Maximum value
    pub value_max: Option<f64>,
    /// Text value
    pub value_text: Option<String>,
    /// Position in set (1-indexed)
    pub pos_in_set: Option<usize>,
    /// Set size
    pub set_size: Option<usize>,
    /// Level (for headings, trees)
    pub level: Option<usize>,
    /// Error message
    pub error_message: Option<String>,
}

impl AccessibleState {
    /// Create new empty state
    pub fn new() -> Self {
        Self::default()
    }

    /// Set disabled state
    pub fn disabled(mut self, value: bool) -> Self {
        self.disabled = value;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, value: bool) -> Self {
        self.expanded = Some(value);
        self
    }

    /// Set selected state
    pub fn selected(mut self, value: bool) -> Self {
        self.selected = value;
        self
    }

    /// Set checked state
    pub fn checked(mut self, value: bool) -> Self {
        self.checked = Some(value);
        self
    }

    /// Set pressed state
    pub fn pressed(mut self, value: bool) -> Self {
        self.pressed = Some(value);
        self
    }

    /// Set focused state
    pub fn focused(mut self, value: bool) -> Self {
        self.focused = value;
        self
    }

    /// Set value range
    pub fn value_range(mut self, now: f64, min: f64, max: f64) -> Self {
        self.value_now = Some(now);
        self.value_min = Some(min);
        self.value_max = Some(max);
        self
    }

    /// Set position in set
    pub fn position(mut self, pos: usize, size: usize) -> Self {
        self.pos_in_set = Some(pos);
        self.set_size = Some(size);
        self
    }

    /// Set level
    pub fn level(mut self, level: usize) -> Self {
        self.level = Some(level);
        self
    }

    /// Set error message
    pub fn error(mut self, message: impl Into<String>) -> Self {
        self.error_message = Some(message.into());
        self
    }
}
