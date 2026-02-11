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
