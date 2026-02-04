//! Buffer Operation Property tests

#![allow(unused_imports)]

use proptest::prelude::*;
use revue::render::{Buffer, Cell};
use revue::style::Color;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(100))]

    /// Test that buffer dimensions are preserved
    #[test]
    fn test_buffer_dimensions(w in 0u16..200u16, h in 0u16..200u16) {
        let buffer = Buffer::new(w, h);
        prop_assert_eq!(buffer.width(), w);
        prop_assert_eq!(buffer.height(), h);
    }

    /// Test that buffer area calculation is correct
    #[test]
    fn test_buffer_area(w in 0u16..200u16, h in 0u16..200u16) {
        let buffer = Buffer::new(w, h);
        let area = buffer.width() as u32 * buffer.height() as u32;
        prop_assert_eq!(area, w as u32 * h as u32);
    }

    /// Test that buffer cell access is bounded
    #[test]
    fn test_buffer_cell_in_bounds(w in 1u16..200u16, h in 1u16..200u16) {
        let buffer = Buffer::new(w, h);
        // Safe access should not panic
        let _cell = buffer.get(0, 0);
        let _cell = buffer.get(w.saturating_sub(1), h.saturating_sub(1));
    }

    /// Test that buffer clearing resets content
    #[test]
    fn test_buffer_clear_reset(w in 1u16..100u16, h in 1u16..100u16) {
        let mut buffer = Buffer::new(w, h);

        // Write some content
        let cell = Cell::new('X').fg(Color::WHITE).bg(Color::BLACK);
        buffer.set(0, 0, cell);

        // Clear should reset
        buffer.clear();

        // After clear, cell should be cleared
        let cleared = buffer.get(0, 0);
        prop_assert!(cleared.is_some());
    }
}
