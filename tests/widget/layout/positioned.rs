//! Positioned widget tests

mod float_inputs {
    //! NaN, infinite, negative and out-of-range floats do not overflow the
    //! `f32 as u16` conversions.

    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::traits::{RenderContext, View};
    use revue::widget::Positioned;

    #[test]
    fn test_positioned_negative_percent() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Negative percentage should be handled
        let pos = Positioned::new(revue::widget::Text::new("Test")).percent(-50.0, -50.0);

        // Should not panic - negative percent calculations may produce 0
        pos.render(&mut ctx);
    }

    #[test]
    fn test_positioned_greater_than_100_percent() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Percentages > 100 should work
        let pos = Positioned::new(revue::widget::Text::new("Test"))
            .percent(150.0, 150.0)
            .size(10, 1);

        // Should not panic
        pos.render(&mut ctx);
    }

    #[test]
    fn test_positioned_nan_percent() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // NaN percentage - the f32 as u16 conversion could be problematic
        let pos = Positioned::new(revue::widget::Text::new("Test")).percent(f32::NAN, f32::NAN);

        // NaN * width/height = NaN, NaN as u16 is undefined (could be 0 or wrap)
        // The code should handle this via saturating_add which treats NaN as 0
        pos.render(&mut ctx);
    }

    #[test]
    fn test_positioned_infinity_percent() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Infinity percentage
        let pos =
            Positioned::new(revue::widget::Text::new("Test")).percent(f32::INFINITY, f32::INFINITY);

        // Infinity * value = Infinity, Infinity as u16 wraps
        // saturating_add should handle this
        pos.render(&mut ctx);
    }

    #[test]
    fn test_positioned_negative_position() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Negative absolute position
        let pos = Positioned::new(revue::widget::Text::new("Test")).at(-10, -5);

        // Should not panic - saturating_sub handles this
        pos.render(&mut ctx);
    }

    #[test]
    fn test_positioned_very_large_position() {
        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Very large positive position
        let pos = Positioned::new(revue::widget::Text::new("Test")).at(10000, 10000);

        // Should not panic - position will be outside area but no crash
        pos.render(&mut ctx);
    }
}
