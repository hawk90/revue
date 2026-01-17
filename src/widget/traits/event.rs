//! Event handling types

/// Result of handling an event
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EventResult {
    /// Event was not handled, should propagate to parent
    #[default]
    Ignored,
    /// Event was consumed, stop propagation
    Consumed,
    /// Event was consumed and widget needs re-render
    ConsumedAndRender,
}

impl EventResult {
    /// Check if the event was handled (consumed)
    #[inline]
    pub fn is_consumed(&self) -> bool {
        !matches!(self, EventResult::Ignored)
    }

    /// Check if re-render is needed
    #[inline]
    pub fn needs_render(&self) -> bool {
        matches!(self, EventResult::ConsumedAndRender)
    }

    /// Combine with another result (takes the "most impactful")
    pub fn or(self, other: EventResult) -> EventResult {
        match (self, other) {
            (EventResult::ConsumedAndRender, _) | (_, EventResult::ConsumedAndRender) => {
                EventResult::ConsumedAndRender
            }
            (EventResult::Consumed, _) | (_, EventResult::Consumed) => EventResult::Consumed,
            _ => EventResult::Ignored,
        }
    }
}

impl From<bool> for EventResult {
    /// Convert from bool: true = ConsumedAndRender, false = Ignored
    fn from(handled: bool) -> Self {
        if handled {
            EventResult::ConsumedAndRender
        } else {
            EventResult::Ignored
        }
    }
}

/// Focus indicator style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FocusStyle {
    /// Single line border (┌─┐)
    #[default]
    Solid,
    /// Rounded corners (╭─╮)
    Rounded,
    /// Double line border (╔═╗)
    Double,
    /// Dotted line border (┌┄┐)
    Dotted,
    /// Bold line border (┏━┓)
    Bold,
    /// ASCII compatible (+--+)
    Ascii,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // EventResult Tests
    // =========================================================================

    #[test]
    fn test_event_result_default() {
        let result = EventResult::default();
        assert_eq!(result, EventResult::Ignored);
    }

    #[test]
    fn test_event_result_is_consumed() {
        assert!(!EventResult::Ignored.is_consumed());
        assert!(EventResult::Consumed.is_consumed());
        assert!(EventResult::ConsumedAndRender.is_consumed());
    }

    #[test]
    fn test_event_result_needs_render() {
        assert!(!EventResult::Ignored.needs_render());
        assert!(!EventResult::Consumed.needs_render());
        assert!(EventResult::ConsumedAndRender.needs_render());
    }

    #[test]
    fn test_event_result_or_consumed_and_render() {
        // ConsumedAndRender wins over everything
        assert_eq!(
            EventResult::ConsumedAndRender.or(EventResult::Ignored),
            EventResult::ConsumedAndRender
        );
        assert_eq!(
            EventResult::Ignored.or(EventResult::ConsumedAndRender),
            EventResult::ConsumedAndRender
        );
        assert_eq!(
            EventResult::Consumed.or(EventResult::ConsumedAndRender),
            EventResult::ConsumedAndRender
        );
    }

    #[test]
    fn test_event_result_or_consumed() {
        // Consumed wins over Ignored
        assert_eq!(
            EventResult::Consumed.or(EventResult::Ignored),
            EventResult::Consumed
        );
        assert_eq!(
            EventResult::Ignored.or(EventResult::Consumed),
            EventResult::Consumed
        );
    }

    #[test]
    fn test_event_result_or_ignored() {
        // Ignored + Ignored = Ignored
        assert_eq!(
            EventResult::Ignored.or(EventResult::Ignored),
            EventResult::Ignored
        );
    }

    #[test]
    fn test_event_result_from_bool_true() {
        let result: EventResult = true.into();
        assert_eq!(result, EventResult::ConsumedAndRender);
    }

    #[test]
    fn test_event_result_from_bool_false() {
        let result: EventResult = false.into();
        assert_eq!(result, EventResult::Ignored);
    }

    #[test]
    fn test_event_result_clone() {
        let result = EventResult::ConsumedAndRender;
        let cloned = result;
        assert_eq!(cloned, EventResult::ConsumedAndRender);
    }

    #[test]
    fn test_event_result_debug() {
        let result = EventResult::Consumed;
        let debug = format!("{:?}", result);
        assert!(debug.contains("Consumed"));
    }

    #[test]
    fn test_event_result_copy() {
        let result = EventResult::Consumed;
        let copied = result;
        assert_eq!(result, copied);
    }

    // =========================================================================
    // FocusStyle Tests
    // =========================================================================

    #[test]
    fn test_focus_style_default() {
        let style = FocusStyle::default();
        assert_eq!(style, FocusStyle::Solid);
    }

    #[test]
    fn test_focus_style_variants() {
        assert_eq!(FocusStyle::Solid, FocusStyle::Solid);
        assert_eq!(FocusStyle::Rounded, FocusStyle::Rounded);
        assert_eq!(FocusStyle::Double, FocusStyle::Double);
        assert_eq!(FocusStyle::Dotted, FocusStyle::Dotted);
        assert_eq!(FocusStyle::Bold, FocusStyle::Bold);
        assert_eq!(FocusStyle::Ascii, FocusStyle::Ascii);
    }

    #[test]
    fn test_focus_style_ne() {
        assert_ne!(FocusStyle::Solid, FocusStyle::Rounded);
        assert_ne!(FocusStyle::Double, FocusStyle::Dotted);
        assert_ne!(FocusStyle::Bold, FocusStyle::Ascii);
    }

    #[test]
    fn test_focus_style_clone() {
        let style = FocusStyle::Rounded;
        let cloned = style;
        assert_eq!(cloned, FocusStyle::Rounded);
    }

    #[test]
    fn test_focus_style_debug() {
        let style = FocusStyle::Double;
        let debug = format!("{:?}", style);
        assert!(debug.contains("Double"));
    }

    #[test]
    fn test_focus_style_copy() {
        let style = FocusStyle::Bold;
        let copied = style;
        assert_eq!(style, copied);
    }
}
