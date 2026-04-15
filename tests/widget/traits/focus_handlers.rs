//! Tests for impl_focus_handlers! macro
//!
//! Verifies the state/direct/no_blur variants generate correct methods.

use revue::event::KeyEvent;
use revue::widget::traits::{EventResult, Interactive, RenderContext, View, WidgetState};

// --- Test widget using `state` variant ---

struct StateWidget {
    state: WidgetState,
    props: revue::widget::traits::WidgetProps,
}

impl StateWidget {
    fn new() -> Self {
        Self {
            state: WidgetState::new(),
            props: revue::widget::traits::WidgetProps::new(),
        }
    }
}

impl View for StateWidget {
    fn render(&self, _ctx: &mut RenderContext) {}
    revue::impl_view_meta!("StateWidget");
}

impl Interactive for StateWidget {
    fn handle_key(&mut self, _event: &KeyEvent) -> EventResult {
        EventResult::Ignored
    }

    revue::impl_focus_handlers!(state);
}

#[test]
fn test_state_focusable_when_enabled() {
    let w = StateWidget::new();
    assert!(w.focusable());
}

#[test]
fn test_state_not_focusable_when_disabled() {
    let mut w = StateWidget::new();
    w.state.disabled = true;
    assert!(!w.focusable());
}

#[test]
fn test_state_on_focus_sets_focused() {
    let mut w = StateWidget::new();
    assert!(!w.state.focused);
    w.on_focus();
    assert!(w.state.focused);
}

#[test]
fn test_state_on_blur_clears_focused() {
    let mut w = StateWidget::new();
    w.on_focus();
    w.on_blur();
    assert!(!w.state.focused);
}

// --- Test widget using `direct` variant ---

struct DirectWidget {
    disabled: bool,
    focused: bool,
    props: revue::widget::traits::WidgetProps,
}

impl DirectWidget {
    fn new() -> Self {
        Self {
            disabled: false,
            focused: false,
            props: revue::widget::traits::WidgetProps::new(),
        }
    }
}

impl View for DirectWidget {
    fn render(&self, _ctx: &mut RenderContext) {}
    revue::impl_view_meta!("DirectWidget");
}

impl Interactive for DirectWidget {
    fn handle_key(&mut self, _event: &KeyEvent) -> EventResult {
        EventResult::Ignored
    }

    revue::impl_focus_handlers!(direct);
}

#[test]
fn test_direct_focusable_when_enabled() {
    let w = DirectWidget::new();
    assert!(w.focusable());
}

#[test]
fn test_direct_not_focusable_when_disabled() {
    let w = DirectWidget {
        disabled: true,
        ..DirectWidget::new()
    };
    assert!(!w.focusable());
}

#[test]
fn test_direct_focus_blur_cycle() {
    let mut w = DirectWidget::new();
    w.on_focus();
    assert!(w.focused);
    w.on_blur();
    assert!(!w.focused);
}

// --- Test widget using `no_blur` variant ---

struct NoBlurWidget {
    disabled: bool,
    focused: bool,
    blur_called: bool,
    props: revue::widget::traits::WidgetProps,
}

impl NoBlurWidget {
    fn new() -> Self {
        Self {
            disabled: false,
            focused: false,
            blur_called: false,
            props: revue::widget::traits::WidgetProps::new(),
        }
    }
}

impl View for NoBlurWidget {
    fn render(&self, _ctx: &mut RenderContext) {}
    revue::impl_view_meta!("NoBlurWidget");
}

impl Interactive for NoBlurWidget {
    fn handle_key(&mut self, _event: &KeyEvent) -> EventResult {
        EventResult::Ignored
    }

    revue::impl_focus_handlers!(direct, no_blur);

    fn on_blur(&mut self) {
        self.focused = false;
        self.blur_called = true;
    }
}

#[test]
fn test_no_blur_custom_on_blur() {
    let mut w = NoBlurWidget::new();
    w.on_focus();
    assert!(w.focused);
    w.on_blur();
    assert!(!w.focused);
    assert!(w.blur_called);
}

#[test]
fn test_no_blur_focusable_still_works() {
    let w = NoBlurWidget::new();
    assert!(w.focusable());
}
