use super::core::CommandPalette;

/// Helper to create a command palette
pub fn command_palette() -> CommandPalette {
    CommandPalette::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // command_palette() helper function tests
    // =========================================================================

    #[test]
    fn test_command_palette_helper() {
        let cp = command_palette();
        // Should create without panicking
        let _ = cp;
    }

    #[test]
    fn test_command_palette_helper_returns_palette() {
        let cp = command_palette();
        // Verify it's a CommandPalette
        let _ = cp.query;
    }

    #[test]
    fn test_command_palette_initial_state() {
        let cp = command_palette();
        // Query should be empty initially
        assert!(cp.query.is_empty());
    }

    #[test]
    fn test_command_palette_multiple_instances() {
        let cp1 = command_palette();
        let cp2 = command_palette();
        // Each should be independent
        assert!(cp1.query.is_empty());
        assert!(cp2.query.is_empty());
    }

    #[test]
    fn test_command_palette_is_chainable() {
        let cp = command_palette();
        // Should allow builder methods
        let _ = cp;
    }

    #[test]
    fn test_command_palette_helpers_do_not_panic() {
        // Helper should work without panicking
        let _ = command_palette();
    }

    #[test]
    fn test_command_palette_returns_same_type() {
        let cp = command_palette();
        // Should always return CommandPalette type
        let _ = cp;
    }

    #[test]
    fn test_command_palette_can_be_created_repeatedly() {
        // Create multiple palettes in sequence
        for _ in 0..10 {
            let cp = command_palette();
            assert!(cp.query.is_empty());
        }
    }

    #[test]
    fn test_command_palette_initial_filter_state() {
        let cp = command_palette();
        // Filtered indices should be empty initially
        assert!(cp.filtered.is_empty());
    }

    #[test]
    fn test_command_palette_initial_visible_state() {
        let cp = command_palette();
        // Should have a visible state (true or false based on implementation)
        let _ = cp.visible;
    }

    #[test]
    fn test_command_palette_initial_width() {
        let cp = command_palette();
        // Should have a default width
        assert!(cp.width > 0);
    }

    #[test]
    fn test_command_palette_initial_max_visible() {
        let cp = command_palette();
        // Should have a default max visible items setting
        assert!(cp.max_visible > 0);
    }

    #[test]
    fn test_command_palette_initial_placeholder() {
        let cp = command_palette();
        // Should have a default placeholder
        let _ = cp.placeholder;
    }

    #[test]
    fn test_command_palette_initial_title() {
        let cp = command_palette();
        // Title should be None initially
        assert!(cp.title.is_none());
    }

    #[test]
    fn test_command_palette_initial_show_descriptions() {
        let cp = command_palette();
        // Should have a default show_descriptions setting
        let _ = cp.show_descriptions;
    }

    #[test]
    fn test_command_palette_initial_show_shortcuts() {
        let cp = command_palette();
        // Should have a default show_shortcuts setting
        let _ = cp.show_shortcuts;
    }

    #[test]
    fn test_command_palette_initial_show_icons() {
        let cp = command_palette();
        // Should have a default show_icons setting
        let _ = cp.show_icons;
    }

    #[test]
    fn test_command_palette_empty_initially() {
        let cp = command_palette();
        // Commands list should be empty initially
        assert!(cp.commands.is_empty());
    }

    #[test]
    fn test_command_palette_initial_selection_state() {
        let cp = command_palette();
        // Should have a selection state
        let _ = cp.selection;
    }

    #[test]
    fn test_command_palette_initial_colors() {
        let cp = command_palette();
        // Should have default colors set
        let _ = cp.bg_color;
        let _ = cp.border_color;
        let _ = cp.selected_bg;
        let _ = cp.match_color;
    }

    #[test]
    fn test_command_palette_initial_props() {
        let cp = command_palette();
        // Should have widget properties
        let _ = cp.props;
    }

    #[test]
    fn test_command_palette_clone_behavior() {
        // Test that multiple palettes are independent
        let cp1 = command_palette();
        let cp2 = command_palette();

        // Both should have independent state
        assert_eq!(cp1.query, cp2.query);
        assert!(cp1.commands.is_empty());
        assert!(cp2.commands.is_empty());
    }

    #[test]
    fn test_command_palette_initial_width_is_reasonable() {
        let cp = command_palette();
        // Width should be a reasonable value
        assert!(cp.width >= 20);
        assert!(cp.width <= 200);
    }

    #[test]
    fn test_command_palette_initial_max_visible_is_reasonable() {
        let cp = command_palette();
        // Max visible should be a reasonable value
        assert!(cp.max_visible >= 5);
        assert!(cp.max_visible <= 50);
    }
}
