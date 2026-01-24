//! Terminal backend using crossterm

mod core;
mod helper;
mod render;
mod types;

pub use helper::stdout_terminal;
pub use types::Terminal;

#[cfg(test)]
mod tests {
    use crate::layout::Rect;
    use crate::render::cell::{Cell, Modifier};
    use crate::render::terminal::Terminal;
    use crate::render::Buffer;
    use crate::style::Color;
    use crossterm::style::Color as CrosstermColor;
    use std::io::{self, Write};

    // Helper function to convert Color to CrosstermColor (duplicated from render.rs)
    fn to_crossterm_color(color: Color) -> CrosstermColor {
        // Check if it's a named color (using helper methods)
        let Color { r, g, b, a: _ } = color;
        CrosstermColor::Rgb { r, g, b }
    }

    // Mock writer for testing
    struct MockWriter {
        buffer: Vec<u8>,
    }

    impl MockWriter {
        fn new() -> Self {
            Self { buffer: Vec::new() }
        }

        fn contents(&self) -> &[u8] {
            &self.buffer
        }

        fn as_string(&self) -> String {
            String::from_utf8_lossy(&self.buffer).to_string()
        }
    }

    impl Write for MockWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    // RenderState is internal, so we use a minimal test struct
    #[derive(Default)]
    struct RenderState {
        fg: Option<Color>,
        bg: Option<Color>,
        modifier: Modifier,
        hyperlink_id: Option<u16>,
        cursor: Option<(u16, u16)>,
    }

    // RenderState tests
    #[test]
    fn test_render_state_default() {
        let state = RenderState::default();
        assert!(state.fg.is_none());
        assert!(state.bg.is_none());
        assert!(state.modifier.is_empty());
        assert!(state.hyperlink_id.is_none());
    }

    // Color conversion tests
    #[test]
    fn test_to_crossterm_color() {
        let color = Color::rgb(255, 128, 64);
        let ct_color = to_crossterm_color(color);

        match ct_color {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 128);
                assert_eq!(b, 64);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_constants_conversion() {
        let red = to_crossterm_color(Color::RED);
        match red {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 0);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_green_conversion() {
        let green = to_crossterm_color(Color::GREEN);
        match green {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_blue_conversion() {
        let blue = to_crossterm_color(Color::BLUE);
        match blue {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 0);
                assert_eq!(b, 255);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_white_conversion() {
        let white = to_crossterm_color(Color::WHITE);
        match white {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 255);
                assert_eq!(b, 255);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_black_conversion() {
        let black = to_crossterm_color(Color::BLACK);
        match black {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 0);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_cyan_conversion() {
        let cyan = to_crossterm_color(Color::CYAN);
        match cyan {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 0);
                assert_eq!(g, 255);
                assert_eq!(b, 255);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_magenta_conversion() {
        let magenta = to_crossterm_color(Color::MAGENTA);
        match magenta {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 0);
                assert_eq!(b, 255);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_yellow_conversion() {
        let yellow = to_crossterm_color(Color::YELLOW);
        match yellow {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 255);
                assert_eq!(g, 255);
                assert_eq!(b, 0);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    #[test]
    fn test_color_gray_conversion() {
        let gray = to_crossterm_color(Color::rgb(128, 128, 128));
        match gray {
            CrosstermColor::Rgb { r, g, b } => {
                assert_eq!(r, 128);
                assert_eq!(g, 128);
                assert_eq!(b, 128);
            }
            _ => panic!("Expected RGB color"),
        }
    }

    // Hyperlink escape sequence tests
    #[test]
    fn test_hyperlink_start_escape() {
        let mut writer = MockWriter::new();
        let url = "https://example.com";
        write!(writer, "\x1b]8;;{}\x1b\\", url).unwrap();
        let output = writer.as_string();
        assert!(output.contains("8;;"));
        assert!(output.contains("https://example.com"));
    }

    #[test]
    fn test_hyperlink_end_escape() {
        let mut writer = MockWriter::new();
        write!(writer, "\x1b]8;;\x1b\\").unwrap();
        let output = writer.as_string();
        assert!(output.contains("8;;"));
    }

    // MockWriter tests
    #[test]
    fn test_mock_writer_write() {
        let mut writer = MockWriter::new();
        let bytes_written = writer.write(b"hello").unwrap();
        assert_eq!(bytes_written, 5);
        assert_eq!(writer.contents(), b"hello");
    }

    #[test]
    fn test_mock_writer_multiple_writes() {
        let mut writer = MockWriter::new();
        writer.write(b"hello").unwrap();
        writer.write(b" ").unwrap();
        writer.write(b"world").unwrap();
        assert_eq!(writer.as_string(), "hello world");
    }

    #[test]
    fn test_mock_writer_flush() {
        let mut writer = MockWriter::new();
        assert!(writer.flush().is_ok());
    }

    // Modifier tests
    #[test]
    fn test_modifier_empty() {
        let modifier = Modifier::empty();
        assert!(modifier.is_empty());
        assert!(!modifier.contains(Modifier::BOLD));
        assert!(!modifier.contains(Modifier::ITALIC));
    }

    #[test]
    fn test_modifier_bold() {
        let modifier = Modifier::BOLD;
        assert!(!modifier.is_empty());
        assert!(modifier.contains(Modifier::BOLD));
    }

    #[test]
    fn test_modifier_combined() {
        let modifier = Modifier::BOLD | Modifier::ITALIC;
        assert!(modifier.contains(Modifier::BOLD));
        assert!(modifier.contains(Modifier::ITALIC));
        assert!(!modifier.contains(Modifier::UNDERLINE));
    }

    // =========================================================================
    // Issue #175: Cursor tracking tests
    // =========================================================================

    #[test]
    fn test_render_state_cursor_default() {
        let state = RenderState::default();
        assert!(state.cursor.is_none());
    }

    #[test]
    fn test_render_state_cursor_tracking() {
        let mut state = RenderState::default();

        // Initially no cursor position
        assert!(state.cursor.is_none());

        // After setting cursor position
        state.cursor = Some((5, 10));
        assert_eq!(state.cursor, Some((5, 10)));

        // After advancing cursor (simulating char print)
        state.cursor = Some((6, 10));
        assert_eq!(state.cursor, Some((6, 10)));
    }

    #[test]
    fn test_cursor_position_after_normal_char() {
        // Normal ASCII character has width 1
        let ch = 'A';
        let width = crate::utils::unicode::char_width(ch) as u16;
        assert_eq!(width, 1);

        // Cursor should advance by 1
        let x: u16 = 5;
        let new_x = x.saturating_add(width);
        assert_eq!(new_x, 6);
    }

    #[test]
    fn test_cursor_position_after_wide_char() {
        // CJK character has width 2
        let ch = '한'; // Korean character
        let width = crate::utils::unicode::char_width(ch) as u16;
        assert_eq!(width, 2);

        // Cursor should advance by 2
        let x: u16 = 5;
        let new_x = x.saturating_add(width);
        assert_eq!(new_x, 7);
    }

    #[test]
    fn test_cursor_position_saturating_add() {
        // Test that cursor position doesn't overflow
        let x: u16 = u16::MAX - 1;
        let width: u16 = 2;
        let new_x = x.saturating_add(width);
        assert_eq!(new_x, u16::MAX);
    }

    #[test]
    fn test_cursor_skip_moveto_same_position() {
        let state = RenderState {
            cursor: Some((5, 10)),
            ..RenderState::default()
        };

        // When cursor is at (5, 10) and we want to draw at (5, 10),
        // MoveTo should be skipped
        let target = (5u16, 10u16);
        let should_skip = state.cursor == Some(target);
        assert!(should_skip);
    }

    #[test]
    fn test_cursor_emit_moveto_different_position() {
        let state = RenderState {
            cursor: Some((5, 10)),
            ..RenderState::default()
        };

        // When cursor is at (5, 10) and we want to draw at (6, 10),
        // MoveTo should NOT be skipped (different x)
        let target = (6u16, 10u16);
        let should_skip = state.cursor == Some(target);
        assert!(!should_skip);

        // Different row also needs MoveTo
        let target_diff_row = (5u16, 11u16);
        let should_skip_row = state.cursor == Some(target_diff_row);
        assert!(!should_skip_row);
    }

    #[test]
    fn test_cursor_emit_moveto_when_none() {
        let state = RenderState::default();

        // When cursor is None, MoveTo should always be emitted
        let target = (5u16, 10u16);
        let should_skip = state.cursor == Some(target);
        assert!(!should_skip);
    }

    #[test]
    fn test_contiguous_cells_cursor_tracking() {
        // Simulate drawing "ABC" at (0, 0)
        let mut state = RenderState::default();

        // First char 'A' at (0, 0) - MoveTo needed (cursor is None)
        assert!(state.cursor != Some((0, 0)));
        state.cursor = Some((1, 0)); // After 'A', cursor at (1, 0)

        // Second char 'B' at (1, 0) - MoveTo NOT needed
        assert!(state.cursor == Some((1, 0)));
        state.cursor = Some((2, 0)); // After 'B', cursor at (2, 0)

        // Third char 'C' at (2, 0) - MoveTo NOT needed
        assert!(state.cursor == Some((2, 0)));
        // After 'C', cursor would be at (3, 0)
    }

    // =========================================================================
    // Issue #174: render_dirty tests
    // =========================================================================

    #[test]
    fn test_render_dirty_only_diffs_dirty_regions() {
        let buf1 = Buffer::new(20, 20);
        let mut buf2 = Buffer::new(20, 20);

        // Make changes at two different locations
        buf2.set(5, 5, Cell::new('X')); // Inside dirty rect
        buf2.set(15, 15, Cell::new('Y')); // Outside dirty rect

        // Only diff the region containing (5, 5)
        let dirty_rects = vec![Rect::new(0, 0, 10, 10)];
        let changes = crate::render::diff::diff(&buf1, &buf2, &dirty_rects);

        // Should only find the change at (5, 5), not (15, 15)
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 5);
        assert_eq!(changes[0].y, 5);
    }

    #[test]
    fn test_render_dirty_multiple_regions() {
        let buf1 = Buffer::new(20, 20);
        let mut buf2 = Buffer::new(20, 20);

        // Make changes in two different dirty regions
        buf2.set(2, 2, Cell::new('X'));
        buf2.set(15, 15, Cell::new('Y'));

        // Two dirty regions covering both changes
        let dirty_rects = vec![
            Rect::new(0, 0, 5, 5),   // Contains (2, 2)
            Rect::new(14, 14, 5, 5), // Contains (15, 15)
        ];
        let changes = crate::render::diff::diff(&buf1, &buf2, &dirty_rects);

        // Should find both changes
        assert_eq!(changes.len(), 2);
    }

    #[test]
    fn test_render_dirty_no_changes_in_dirty_region() {
        let buf1 = Buffer::new(20, 20);
        let mut buf2 = Buffer::new(20, 20);

        // Make change outside the dirty region
        buf2.set(15, 15, Cell::new('Z'));

        // Dirty region doesn't include the change
        let dirty_rects = vec![Rect::new(0, 0, 10, 10)];
        let changes = crate::render::diff::diff(&buf1, &buf2, &dirty_rects);

        // No changes detected (change is outside dirty region)
        assert!(changes.is_empty());
    }

    #[test]
    fn test_render_dirty_empty_dirty_rects_fallback() {
        // When no dirty rects provided, should fall back to full-screen diff
        let buf1 = Buffer::new(10, 10);
        let mut buf2 = Buffer::new(10, 10);
        buf2.set(5, 5, Cell::new('X'));

        let changes = crate::render::diff::diff(&buf1, &buf2, &[]);

        // Should find the change (full-screen fallback)
        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].x, 5);
        assert_eq!(changes[0].y, 5);
    }

    #[test]
    fn test_render_dirty_overlapping_regions() {
        let buf1 = Buffer::new(20, 20);
        let mut buf2 = Buffer::new(20, 20);
        buf2.set(5, 5, Cell::new('X'));

        // Overlapping dirty rects both containing (5, 5)
        let dirty_rects = vec![Rect::new(0, 0, 10, 10), Rect::new(3, 3, 10, 10)];
        let changes = crate::render::diff::diff(&buf1, &buf2, &dirty_rects);

        // Should only report the change once (no duplicates)
        assert_eq!(changes.len(), 1);
    }
}
