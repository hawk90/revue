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

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // AccessibleState::new() and default tests
    // =========================================================================

    #[test]
    fn test_accessible_state_new() {
        let state = AccessibleState::new();
        assert!(!state.disabled);
        assert_eq!(state.expanded, None);
        assert!(!state.selected);
        assert_eq!(state.checked, None);
        assert_eq!(state.pressed, None);
        assert!(!state.focused);
        assert!(!state.hidden);
        assert_eq!(state.value_now, None);
        assert_eq!(state.value_min, None);
        assert_eq!(state.value_max, None);
        assert_eq!(state.value_text, None);
        assert_eq!(state.pos_in_set, None);
        assert_eq!(state.set_size, None);
        assert_eq!(state.level, None);
        assert_eq!(state.error_message, None);
    }

    #[test]
    fn test_accessible_state_default() {
        let state = AccessibleState::default();
        assert!(!state.disabled);
        assert!(!state.selected);
        assert!(!state.focused);
        assert!(!state.hidden);
    }

    // =========================================================================
    // AccessibleState::disabled() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_disabled_true() {
        let state = AccessibleState::new().disabled(true);
        assert!(state.disabled);
    }

    #[test]
    fn test_accessible_state_disabled_false() {
        let state = AccessibleState::new().disabled(false);
        assert!(!state.disabled);
    }

    // =========================================================================
    // AccessibleState::expanded() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_expanded_true() {
        let state = AccessibleState::new().expanded(true);
        assert_eq!(state.expanded, Some(true));
    }

    #[test]
    fn test_accessible_state_expanded_false() {
        let state = AccessibleState::new().expanded(false);
        assert_eq!(state.expanded, Some(false));
    }

    // =========================================================================
    // AccessibleState::selected() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_selected_true() {
        let state = AccessibleState::new().selected(true);
        assert!(state.selected);
    }

    #[test]
    fn test_accessible_state_selected_false() {
        let state = AccessibleState::new().selected(false);
        assert!(!state.selected);
    }

    // =========================================================================
    // AccessibleState::checked() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_checked_true() {
        let state = AccessibleState::new().checked(true);
        assert_eq!(state.checked, Some(true));
    }

    #[test]
    fn test_accessible_state_checked_false() {
        let state = AccessibleState::new().checked(false);
        assert_eq!(state.checked, Some(false));
    }

    // =========================================================================
    // AccessibleState::pressed() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_pressed_true() {
        let state = AccessibleState::new().pressed(true);
        assert_eq!(state.pressed, Some(true));
    }

    #[test]
    fn test_accessible_state_pressed_false() {
        let state = AccessibleState::new().pressed(false);
        assert_eq!(state.pressed, Some(false));
    }

    // =========================================================================
    // AccessibleState::focused() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_focused_true() {
        let state = AccessibleState::new().focused(true);
        assert!(state.focused);
    }

    #[test]
    fn test_accessible_state_focused_false() {
        let state = AccessibleState::new().focused(false);
        assert!(!state.focused);
    }

    // =========================================================================
    // AccessibleState::value_range() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_value_range() {
        let state = AccessibleState::new().value_range(50.0, 0.0, 100.0);
        assert_eq!(state.value_now, Some(50.0));
        assert_eq!(state.value_min, Some(0.0));
        assert_eq!(state.value_max, Some(100.0));
    }

    #[test]
    fn test_accessible_state_value_range_custom() {
        let state = AccessibleState::new().value_range(75.5, 10.0, 90.0);
        assert_eq!(state.value_now, Some(75.5));
        assert_eq!(state.value_min, Some(10.0));
        assert_eq!(state.value_max, Some(90.0));
    }

    // =========================================================================
    // AccessibleState::position() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_position() {
        let state = AccessibleState::new().position(2, 5);
        assert_eq!(state.pos_in_set, Some(2));
        assert_eq!(state.set_size, Some(5));
    }

    #[test]
    fn test_accessible_state_position_first() {
        let state = AccessibleState::new().position(1, 10);
        assert_eq!(state.pos_in_set, Some(1));
        assert_eq!(state.set_size, Some(10));
    }

    // =========================================================================
    // AccessibleState::level() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_level() {
        let state = AccessibleState::new().level(2);
        assert_eq!(state.level, Some(2));
    }

    #[test]
    fn test_accessible_state_level_multiple() {
        let state = AccessibleState::new().level(5);
        assert_eq!(state.level, Some(5));
    }

    // =========================================================================
    // AccessibleState::error() tests
    // =========================================================================

    #[test]
    fn test_accessible_state_error_str() {
        let state = AccessibleState::new().error("Invalid input");
        assert_eq!(state.error_message, Some("Invalid input".to_string()));
    }

    #[test]
    fn test_accessible_state_error_string() {
        let state = AccessibleState::new().error(String::from("Error occurred"));
        assert_eq!(state.error_message, Some("Error occurred".to_string()));
    }

    // =========================================================================
    // AccessibleState clone tests
    // =========================================================================

    #[test]
    fn test_accessible_state_clone() {
        let state = AccessibleState::new()
            .disabled(true)
            .expanded(true)
            .checked(true);
        let cloned = state.clone();
        assert_eq!(state.disabled, cloned.disabled);
        assert_eq!(state.expanded, cloned.expanded);
        assert_eq!(state.checked, cloned.checked);
    }

    // =========================================================================
    // AccessibleState builder chain tests
    // =========================================================================

    #[test]
    fn test_accessible_state_builder_chain() {
        let state = AccessibleState::new()
            .disabled(false)
            .expanded(true)
            .selected(true)
            .checked(true)
            .pressed(false)
            .focused(true)
            .position(1, 5)
            .level(2)
            .error("No error");

        assert!(!state.disabled);
        assert_eq!(state.expanded, Some(true));
        assert!(state.selected);
        assert_eq!(state.checked, Some(true));
        assert_eq!(state.pressed, Some(false));
        assert!(state.focused);
        assert!(!state.hidden);
        assert_eq!(state.pos_in_set, Some(1));
        assert_eq!(state.set_size, Some(5));
        assert_eq!(state.level, Some(2));
        assert_eq!(state.error_message, Some("No error".to_string()));
    }
}
