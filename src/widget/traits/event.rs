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
    // EventResult tests
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
    fn test_event_result_or_both_ignored() {
        let result = EventResult::Ignored.or(EventResult::Ignored);
        assert_eq!(result, EventResult::Ignored);
    }

    #[test]
    fn test_event_result_or_one_consumed() {
        let result = EventResult::Ignored.or(EventResult::Consumed);
        assert_eq!(result, EventResult::Consumed);

        let result = EventResult::Consumed.or(EventResult::Ignored);
        assert_eq!(result, EventResult::Consumed);
    }

    #[test]
    fn test_event_result_or_consumed_and_render_wins() {
        // ConsumedAndRender always wins
        let result = EventResult::Ignored.or(EventResult::ConsumedAndRender);
        assert_eq!(result, EventResult::ConsumedAndRender);

        let result = EventResult::ConsumedAndRender.or(EventResult::Ignored);
        assert_eq!(result, EventResult::ConsumedAndRender);

        let result = EventResult::Consumed.or(EventResult::ConsumedAndRender);
        assert_eq!(result, EventResult::ConsumedAndRender);

        let result = EventResult::ConsumedAndRender.or(EventResult::Consumed);
        assert_eq!(result, EventResult::ConsumedAndRender);
    }

    #[test]
    fn test_event_result_from_bool() {
        let result: EventResult = true.into();
        assert_eq!(result, EventResult::ConsumedAndRender);

        let result: EventResult = false.into();
        assert_eq!(result, EventResult::Ignored);
    }

    // =========================================================================
    // FocusStyle tests
    // =========================================================================

    #[test]
    fn test_focus_style_default() {
        let style = FocusStyle::default();
        assert_eq!(style, FocusStyle::Solid);
    }

    #[test]
    fn test_focus_style_variants() {
        // Just verify all variants exist and are distinct
        let styles = [
            FocusStyle::Solid,
            FocusStyle::Rounded,
            FocusStyle::Double,
            FocusStyle::Dotted,
            FocusStyle::Bold,
            FocusStyle::Ascii,
        ];

        // All should be different from each other
        for i in 0..styles.len() {
            for j in (i + 1)..styles.len() {
                assert_ne!(styles[i], styles[j]);
            }
        }
    }

    #[test]
    fn test_focus_style_clone() {
        let style = FocusStyle::Rounded;
        let cloned = style.clone();
        assert_eq!(style, cloned);
    }

    #[test]
    fn test_focus_style_copy() {
        let style = FocusStyle::Double;
        let copied = style;
        assert_eq!(style, copied);
    }

    #[test]
    fn test_focus_style_debug() {
        let style = FocusStyle::Bold;
        let debug = format!("{:?}", style);
        assert!(debug.contains("Bold"));
    }
}
