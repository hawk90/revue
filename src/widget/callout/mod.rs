//! Callout widget for highlighting important information blocks

mod callout_type;
mod core;
mod helpers;
mod impls;
#[cfg(test)]
mod tests {
    use super::*;

    use crate::event::Key;
    use crate::render::Buffer;
    use crate::widget::callout::core::Callout;
    use crate::widget::callout::helpers::*;
    use crate::widget::callout::types::CalloutType;
    use crate::widget::callout::CalloutVariant;
    use crate::widget::traits::{RenderContext, View};

    #[test]
    fn test_callout_new() {
        let c = Callout::new("Test content");
        assert_eq!(c.content, "Test content");
        assert_eq!(c.callout_type, CalloutType::Note);
        assert!(c.title.is_none());
        assert!(!c.collapsible);
        assert!(c.expanded);
    }

    #[test]
    fn test_callout_type_helpers() {
        assert_eq!(Callout::note("msg").callout_type, CalloutType::Note);
        assert_eq!(Callout::tip("msg").callout_type, CalloutType::Tip);
        assert_eq!(
            Callout::important("msg").callout_type,
            CalloutType::Important
        );
        assert_eq!(Callout::warning("msg").callout_type, CalloutType::Warning);
        assert_eq!(Callout::danger("msg").callout_type, CalloutType::Danger);
        assert_eq!(Callout::info("msg").callout_type, CalloutType::Info);
    }

    #[test]
    fn test_callout_builders() {
        let c = Callout::new("Content")
            .title("Custom Title")
            .callout_type(CalloutType::Warning)
            .variant(CalloutVariant::LeftBorder)
            .collapsible(true)
            .expanded(false)
            .icon(false);

        assert_eq!(c.title, Some("Custom Title".to_string()));
        assert_eq!(c.callout_type, CalloutType::Warning);
        assert_eq!(c.variant, CalloutVariant::LeftBorder);
        assert!(c.collapsible);
        assert!(!c.expanded);
        assert!(!c.show_icon);
    }

    #[test]
    fn test_callout_toggle() {
        let mut c = Callout::new("Test").collapsible(true);
        assert!(c.is_expanded());

        c.toggle();
        assert!(!c.is_expanded());

        c.toggle();
        assert!(c.is_expanded());
    }

    #[test]
    fn test_callout_toggle_not_collapsible() {
        let mut c = Callout::new("Test").collapsible(false);
        assert!(c.is_expanded());

        c.toggle(); // Should not change
        assert!(c.is_expanded());
    }

    #[test]
    fn test_callout_expand_collapse() {
        let mut c = Callout::new("Test").collapsible(true).expanded(false);

        c.expand();
        assert!(c.is_expanded());

        c.collapse();
        assert!(!c.is_expanded());
    }

    #[test]
    fn test_callout_height() {
        // Collapsed
        let collapsed = Callout::new("Content").collapsible(true).expanded(false);
        assert_eq!(collapsed.height(), 1);

        // Filled with single line content
        let filled = Callout::new("Single line").variant(CalloutVariant::Filled);
        assert_eq!(filled.height(), 4); // border + title + content + border

        // Filled with multi-line content
        let multi = Callout::new(
            "Line 1
Line 2
Line 3",
        )
        .variant(CalloutVariant::Filled);
        assert_eq!(multi.height(), 6); // border + title + 3 content lines + border

        // Left border variant
        let left = Callout::new("Content").variant(CalloutVariant::LeftBorder);
        assert_eq!(left.height(), 2); // title + content

        // Minimal variant
        let minimal = Callout::new("Content").variant(CalloutVariant::Minimal);
        assert_eq!(minimal.height(), 2); // title + content
    }

    #[test]
    fn test_callout_handle_key() {
        let mut c = Callout::new("Test").collapsible(true);

        assert!(c.handle_key(&Key::Enter));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Char(' ')));
        assert!(c.is_expanded());

        assert!(c.handle_key(&Key::Left));
        assert!(!c.is_expanded());

        assert!(c.handle_key(&Key::Right));
        assert!(c.is_expanded());

        assert!(!c.handle_key(&Key::Up)); // Not handled
    }

    #[test]
    fn test_callout_handle_key_not_collapsible() {
        let mut c = Callout::new("Test").collapsible(false);

        assert!(!c.handle_key(&Key::Enter));
        assert!(c.is_expanded()); // Should not change
    }

    #[test]
    fn test_callout_handle_key_disabled() {
        let mut c = Callout::new("Test").collapsible(true).disabled(true);

        assert!(!c.handle_key(&Key::Enter));
        assert!(c.is_expanded());
    }

    #[test]
    fn test_callout_type_icons() {
        assert_eq!(CalloutType::Note.icon(), '📝');
        assert_eq!(CalloutType::Tip.icon(), '💡');
        assert_eq!(CalloutType::Important.icon(), '❗');
        assert_eq!(CalloutType::Warning.icon(), '⚠');
        assert_eq!(CalloutType::Danger.icon(), '🔴');
        assert_eq!(CalloutType::Info.icon(), 'ℹ');
    }

    #[test]
    fn test_callout_type_default_titles() {
        assert_eq!(CalloutType::Note.default_title(), "Note");
        assert_eq!(CalloutType::Tip.default_title(), "Tip");
        assert_eq!(CalloutType::Important.default_title(), "Important");
        assert_eq!(CalloutType::Warning.default_title(), "Warning");
        assert_eq!(CalloutType::Danger.default_title(), "Danger");
        assert_eq!(CalloutType::Info.default_title(), "Info");
    }

    #[test]
    fn test_callout_custom_icon() {
        let c = Callout::new("Test").custom_icon('★');
        assert_eq!(c.get_icon(), '★');
        assert!(c.show_icon);
    }

    #[test]
    fn test_callout_get_title() {
        let default_title = Callout::note("Test");
        assert_eq!(default_title.get_title(), "Note");

        let custom_title = Callout::note("Test").title("Custom");
        assert_eq!(custom_title.get_title(), "Custom");
    }

    #[test]
    fn test_callout_collapse_icons() {
        let c = Callout::new("Test")
            .collapsible(true)
            .collapse_icons('+', '-');

        assert_eq!(c.collapsed_icon, '+');
        assert_eq!(c.expanded_icon, '-');
        assert_eq!(c.collapse_icon(), '-'); // expanded by default
    }

    #[test]
    fn test_callout_render_filled() {
        let mut buffer = Buffer::new(50, 5);
        let area = Rect::new(0, 0, 50, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::note("Test content").variant(CalloutVariant::Filled);
        c.render(&mut ctx);

        // Check left accent border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    }

    #[test]
    fn test_callout_render_left_border() {
        let mut buffer = Buffer::new(50, 3);
        let area = Rect::new(0, 0, 50, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::tip("Test").variant(CalloutVariant::LeftBorder);
        c.render(&mut ctx);

        // Check left accent border
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    }

    #[test]
    fn test_callout_render_minimal() {
        let mut buffer = Buffer::new(50, 3);
        let area = Rect::new(0, 0, 50, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::warning("Test").variant(CalloutVariant::Minimal);
        c.render(&mut ctx);

        // Check icon
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '⚠');
    }

    #[test]
    fn test_callout_render_collapsed() {
        let mut buffer = Buffer::new(50, 5);
        let area = Rect::new(0, 0, 50, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let c = Callout::note("Hidden content")
            .collapsible(true)
            .expanded(false)
            .variant(CalloutVariant::Filled);
        c.render(&mut ctx);

        // Only header should be rendered
        assert_eq!(buffer.get(0, 0).unwrap().symbol, '┃');
    }

    #[test]
    fn test_callout_helpers() {
        let c = callout("msg");
        assert_eq!(c.content, "msg");

        let n = note("note");
        assert_eq!(n.callout_type, CalloutType::Note);

        let t = tip("tip");
        assert_eq!(t.callout_type, CalloutType::Tip);

        let i = important("important");
        assert_eq!(i.callout_type, CalloutType::Important);

        let w = warning_callout("warning");
        assert_eq!(w.callout_type, CalloutType::Warning);

        let d = danger("danger");
        assert_eq!(d.callout_type, CalloutType::Danger);

        let info = info_callout("info");
        assert_eq!(info.callout_type, CalloutType::Info);
    }

    #[test]
    fn test_callout_default() {
        let c = Callout::default();
        assert_eq!(c.content, "Callout");
    }
}
mod types;
mod view;

pub use core::Callout;
pub use helpers::*;
pub use types::{CalloutType, CalloutVariant};
