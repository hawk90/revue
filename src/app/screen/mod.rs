//! Screen system for multi-page TUI applications
//!
//! Provides a screen stack for managing multiple views/pages with transitions.

mod core;
mod state;
mod types;

pub use core::ScreenManager;
pub use state::{screen_manager, simple_screen, SimpleScreen};
pub use types::{
    Screen, ScreenConfig, ScreenData, ScreenEvent, ScreenId, ScreenMode, ScreenResult, Transition,
};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::RenderContext;
    use std::time::Duration;

    #[allow(dead_code)]
    struct TestScreen {
        id: ScreenId,
        events: Vec<ScreenEvent>,
    }

    impl TestScreen {
        #[allow(dead_code)]
        fn new(id: impl Into<ScreenId>) -> Self {
            Self {
                id: id.into(),
                events: Vec::new(),
            }
        }
    }

    impl Screen for TestScreen {
        fn id(&self) -> ScreenId {
            self.id.clone()
        }

        fn on_event(&mut self, event: ScreenEvent) {
            self.events.push(event);
        }

        fn render(&self, _ctx: &mut RenderContext) {}
    }

    // ScreenId tests
    #[test]
    fn test_screen_id_new() {
        let id = ScreenId::new("home");
        assert_eq!(id.as_str(), "home");
    }

    #[test]
    fn test_screen_id_display() {
        let id = ScreenId::new("settings");
        assert_eq!(format!("{}", id), "settings");
    }

    #[test]
    fn test_screen_id_from_string() {
        let id: ScreenId = "profile".into();
        assert_eq!(id.as_str(), "profile");
    }

    #[test]
    fn test_screen_id_clone() {
        let id = ScreenId::new("test");
        let cloned = id.clone();
        assert_eq!(id, cloned);
    }

    #[test]
    fn test_screen_id_eq() {
        let id1 = ScreenId::new("same");
        let id2 = ScreenId::new("same");
        let id3 = ScreenId::new("different");
        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    // Transition tests
    #[test]
    fn test_transition_default() {
        let t = Transition::default();
        assert_eq!(t, Transition::None);
    }

    #[test]
    fn test_transition_variants() {
        assert_eq!(Transition::None, Transition::None);
        assert_eq!(Transition::Fade, Transition::Fade);
        assert_eq!(Transition::SlideLeft, Transition::SlideLeft);
    }

    #[test]
    fn test_transition_clone() {
        let t = Transition::Fade;
        let cloned = t.clone();
        assert_eq!(t, cloned);
    }

    // ScreenMode tests
    #[test]
    fn test_screen_mode_default() {
        let mode = ScreenMode::default();
        assert_eq!(mode, ScreenMode::Fullscreen);
    }

    #[test]
    fn test_screen_mode_variants() {
        assert_eq!(ScreenMode::Fullscreen, ScreenMode::Fullscreen);
        assert_eq!(ScreenMode::Modal, ScreenMode::Modal);
        assert_eq!(ScreenMode::Popup, ScreenMode::Popup);
    }

    // ScreenEvent tests
    #[test]
    fn test_screen_event_clone() {
        let event = ScreenEvent::Mount;
        let cloned = event.clone();
        assert!(matches!(cloned, ScreenEvent::Mount));
    }

    #[test]
    fn test_screen_event_debug() {
        let events = vec![
            ScreenEvent::Mount,
            ScreenEvent::Unmount,
            ScreenEvent::Show,
            ScreenEvent::Hide,
            ScreenEvent::Focus,
            ScreenEvent::Blur,
            ScreenEvent::Resume,
            ScreenEvent::Suspend,
        ];
        for event in events {
            let debug = format!("{:?}", event);
            assert!(!debug.is_empty());
        }
    }

    // ScreenResult tests
    #[test]
    fn test_screen_result_default() {
        let result = ScreenResult::default();
        assert!(matches!(result, ScreenResult::Continue));
    }

    #[test]
    fn test_screen_result_clone() {
        let result = ScreenResult::Render;
        let cloned = result.clone();
        assert!(matches!(cloned, ScreenResult::Render));
    }

    #[test]
    fn test_screen_result_push() {
        let result = ScreenResult::Push(ScreenId::new("test"));
        assert!(matches!(result, ScreenResult::Push(_)));
    }

    // ScreenConfig tests
    #[test]
    fn test_screen_config_default() {
        let config = ScreenConfig::default();
        assert_eq!(config.mode, ScreenMode::Fullscreen);
        assert_eq!(config.enter_transition, Transition::None);
        assert_eq!(config.exit_transition, Transition::None);
        assert!(config.dismissable);
        assert!(config.title.is_none());
    }

    #[test]
    fn test_screen_config_fullscreen() {
        let config = ScreenConfig::fullscreen();
        assert_eq!(config.mode, ScreenMode::Fullscreen);
    }

    #[test]
    fn test_screen_config_modal() {
        let config = ScreenConfig::modal();
        assert_eq!(config.mode, ScreenMode::Modal);
        assert_eq!(config.enter_transition, Transition::Fade);
        assert_eq!(config.exit_transition, Transition::Fade);
    }

    #[test]
    fn test_screen_config_popup() {
        let config = ScreenConfig::popup();
        assert_eq!(config.mode, ScreenMode::Popup);
        assert_eq!(config.transition_duration, Duration::from_millis(100));
    }

    #[test]
    fn test_screen_config_builder() {
        let config = ScreenConfig::default()
            .mode(ScreenMode::Modal)
            .transitions(Transition::SlideLeft, Transition::SlideRight)
            .duration(Duration::from_millis(300))
            .dismissable(false)
            .title("Settings");

        assert_eq!(config.mode, ScreenMode::Modal);
        assert_eq!(config.enter_transition, Transition::SlideLeft);
        assert_eq!(config.exit_transition, Transition::SlideRight);
        assert_eq!(config.transition_duration, Duration::from_millis(300));
        assert!(!config.dismissable);
        assert_eq!(config.title, Some("Settings".to_string()));
    }

    // ScreenManager tests
    #[test]
    fn test_screen_manager_new() {
        let manager = ScreenManager::new();
        assert_eq!(manager.depth(), 0);
        assert!(!manager.can_pop());
    }

    #[test]
    fn test_screen_manager_default() {
        let manager = ScreenManager::default();
        assert_eq!(manager.depth(), 0);
    }

    // Helper function tests
    #[test]
    fn test_screen_manager_helper() {
        let manager = screen_manager();
        assert_eq!(manager.depth(), 0);
    }

    #[test]
    fn test_simple_screen_helper() {
        let screen = simple_screen("test", |_ctx| {});
        assert_eq!(screen.id().as_str(), "test");
    }

    #[test]
    fn test_simple_screen_with_config() {
        let config = ScreenConfig::modal();
        let screen = simple_screen("modal", |_ctx| {}).config(config);
        let screen_config = Screen::config(&screen);
        assert_eq!(screen_config.mode, ScreenMode::Modal);
    }
}
