#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Basic Tests
    // =========================================================================

    #[test]
    fn test_new() {
        let buf = TextBuffer::new();
        assert!(buf.is_empty());
        assert_eq!(buf.cursor(), 0);
        assert_eq!(buf.char_count(), 0);
    }

    #[test]
    fn test_with_content() {
        let buf = TextBuffer::with_content("Hello");
        assert_eq!(buf.text(), "Hello");
        assert_eq!(buf.cursor(), 5);
        assert_eq!(buf.char_count(), 5);
    }

    // =========================================================================
    // UTF-8 Tests
    // =========================================================================

    #[test]
    fn test_utf8_emoji() {
        let buf = TextBuffer::with_content("Hi 🎉!");
        assert_eq!(buf.char_count(), 5); // Not 8 bytes
        assert_eq!(buf.char_at(3), Some('🎉'));
        assert_eq!(buf.substring(0, 2), "Hi");
        assert_eq!(buf.substring(3, 4), "🎉");
    }

    #[test]
    fn test_utf8_korean() {
        let buf = TextBuffer::with_content("안녕하세요");
        assert_eq!(buf.char_count(), 5);
        assert_eq!(buf.char_at(0), Some('안'));
        assert_eq!(buf.substring(1, 3), "녕하");
    }

    #[test]
    fn test_char_to_byte() {
        let buf = TextBuffer::with_content("A🎉B");
        assert_eq!(buf.char_to_byte(0), 0); // 'A' starts at 0
        assert_eq!(buf.char_to_byte(1), 1); // '🎉' starts at 1
        assert_eq!(buf.char_to_byte(2), 5); // 'B' starts at 5 (1 + 4 bytes for emoji)
        assert_eq!(buf.char_to_byte(3), 6); // End
        assert_eq!(buf.char_to_byte(100), 6); // Beyond end
    }

    // =========================================================================
    // Insert/Delete Tests
    // =========================================================================

    #[test]
    fn test_insert_char() {
        let mut buf = TextBuffer::new();
        buf.insert_char('H');
        buf.insert_char('i');
        assert_eq!(buf.text(), "Hi");
        assert_eq!(buf.cursor(), 2);
    }

    #[test]
    fn test_insert_str() {
        let mut buf = TextBuffer::new();
        buf.insert_str("Hello");
        assert_eq!(buf.text(), "Hello");
        assert_eq!(buf.cursor(), 5);

        buf.set_cursor(0);
        buf.insert_str("Say ");
        assert_eq!(buf.text(), "Say Hello");
    }

    #[test]
    fn test_insert_emoji() {
        let mut buf = TextBuffer::new();
        buf.insert_str("Hi ");
        buf.insert_char('🎉');
        assert_eq!(buf.text(), "Hi 🎉");
        assert_eq!(buf.cursor(), 4);
    }

    #[test]
    fn test_delete_char_before() {
        let mut buf = TextBuffer::with_content("Hello");
        let deleted = buf.delete_char_before();
        assert_eq!(deleted, Some('o'));
        assert_eq!(buf.text(), "Hell");
        assert_eq!(buf.cursor(), 4);
    }

    #[test]
    fn test_delete_char_before_emoji() {
        let mut buf = TextBuffer::with_content("Hi🎉");
        let deleted = buf.delete_char_before();
        assert_eq!(deleted, Some('🎉'));
        assert_eq!(buf.text(), "Hi");
        assert_eq!(buf.cursor(), 2);
    }

    #[test]
    fn test_delete_char_at() {
        let mut buf = TextBuffer::with_content("Hello");
        buf.set_cursor(1);
        let deleted = buf.delete_char_at();
        assert_eq!(deleted, Some('e'));
        assert_eq!(buf.text(), "Hllo");
    }

    #[test]
    fn test_delete_range() {
        let mut buf = TextBuffer::with_content("Hello World");
        let deleted = buf.delete_range(0, 6);
        assert_eq!(deleted, "Hello ");
        assert_eq!(buf.text(), "World");
    }

    // =========================================================================
    // Cursor Movement Tests
    // =========================================================================

    #[test]
    fn test_move_left_right() {
        let mut buf = TextBuffer::with_content("Hello");

        buf.set_cursor(3);
        assert!(buf.move_left());
        assert_eq!(buf.cursor(), 2);

        assert!(buf.move_right());
        assert_eq!(buf.cursor(), 3);

        buf.set_cursor(0);
        assert!(!buf.move_left()); // Can't move left from 0

        buf.set_cursor(5);
        assert!(!buf.move_right()); // Can't move right from end
    }

    #[test]
    fn test_move_word() {
        let mut buf = TextBuffer::with_content("hello world test");
        buf.set_cursor(0);

        buf.move_word_right();
        assert_eq!(buf.cursor(), 6); // After "hello "

        buf.move_word_right();
        assert_eq!(buf.cursor(), 12); // After "world "

        buf.move_word_left();
        assert_eq!(buf.cursor(), 6);

        buf.move_word_left();
        assert_eq!(buf.cursor(), 0);
    }

    // =========================================================================
    // Selection Tests
    // =========================================================================

    #[test]
    fn test_selection() {
        let mut buf = TextBuffer::with_content("Hello World");
        buf.set_cursor(0);
        buf.start_selection();
        buf.set_cursor(5);

        assert!(buf.has_selection());
        assert_eq!(buf.selection(), Some((0, 5)));
        assert_eq!(buf.selected_text(), Some("Hello"));
    }

    #[test]
    fn test_selection_reverse() {
        let mut buf = TextBuffer::with_content("Hello World");
        buf.set_cursor(5);
        buf.start_selection();
        buf.set_cursor(0);

        assert!(buf.has_selection());
        assert_eq!(buf.selection(), Some((0, 5))); // Normalized
        assert_eq!(buf.selected_text(), Some("Hello"));
    }

    #[test]
    fn test_select_all() {
        let mut buf = TextBuffer::with_content("Hello");
        buf.select_all();

        assert!(buf.has_selection());
        assert_eq!(buf.selection(), Some((0, 5)));
        assert_eq!(buf.selected_text(), Some("Hello"));
    }

    #[test]
    fn test_delete_selection() {
        let mut buf = TextBuffer::with_content("Hello World");
        buf.set_cursor(0);
        buf.start_selection();
        buf.set_cursor(6);

        let deleted = buf.delete_selection();
        assert_eq!(deleted, Some("Hello ".to_string()));
        assert_eq!(buf.text(), "World");
        assert_eq!(buf.cursor(), 0);
    }

    // =========================================================================
    // Word Operation Tests
    // =========================================================================

    #[test]
    fn test_delete_word_before() {
        let mut buf = TextBuffer::with_content("hello world");
        buf.set_cursor(11); // End

        let deleted = buf.delete_word_before();
        assert_eq!(deleted, "world");
        assert_eq!(buf.text(), "hello ");
    }

    #[test]
    fn test_word_at_cursor() {
        let buf = TextBuffer::with_content("hello world");

        let mut buf2 = buf.clone();
        buf2.set_cursor(2);
        assert_eq!(buf2.word_at_cursor(), (0, 5)); // "hello"

        let mut buf3 = buf.clone();
        buf3.set_cursor(7);
        assert_eq!(buf3.word_at_cursor(), (6, 11)); // "world"
    }

    #[test]
    fn test_select_word() {
        let mut buf = TextBuffer::with_content("hello world");
        buf.set_cursor(2);
        buf.select_word();

        assert_eq!(buf.selection(), Some((0, 5)));
        assert_eq!(buf.selected_text(), Some("hello"));
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_empty_operations() {
        let mut buf = TextBuffer::new();

        assert_eq!(buf.delete_char_before(), None);
        assert_eq!(buf.delete_char_at(), None);
        assert!(!buf.move_left());
        assert!(!buf.move_right());
        assert!(!buf.has_selection());
    }

    #[test]
    fn test_set_cursor_clamped() {
        let mut buf = TextBuffer::with_content("Hello");
        buf.set_cursor(100);
        assert_eq!(buf.cursor(), 5); // Clamped to length
    }

    #[test]
    fn test_clear() {
        let mut buf = TextBuffer::with_content("Hello");
        buf.start_selection();
        buf.clear();

        assert!(buf.is_empty());
        assert_eq!(buf.cursor(), 0);
        assert!(!buf.has_selection());
    }
}
