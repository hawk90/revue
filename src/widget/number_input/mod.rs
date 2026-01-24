//! Number input widget with increment/decrement controls
//!
//! Provides a numeric input field with:
//! - Up/Down arrow key controls for increment/decrement
//! - Direct numeric entry
//! - Min/max value constraints
//! - Configurable step size and precision
//! - Optional prefix/suffix display (e.g., "$", "%")

mod core;
mod helper;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_input_new() {
        let input = NumberInput::new();
        assert_eq!(input.get_value(), 0.0);
        assert!(!input.is_editing());
    }

    #[test]
    fn test_number_input_value() {
        let input = number_input().value(42.0);
        assert_eq!(input.get_value(), 42.0);
    }

    #[test]
    fn test_number_input_min_max() {
        let input = number_input().value(150.0).min(0.0).max(100.0);
        assert_eq!(input.get_value(), 100.0); // Clamped to max

        let input = number_input().value(-50.0).min(0.0).max(100.0);
        assert_eq!(input.get_value(), 0.0); // Clamped to min
    }

    #[test]
    fn test_number_input_increment() {
        let mut input = number_input().value(10.0).step(5.0);
        input.increment();
        assert_eq!(input.get_value(), 15.0);

        input.decrement();
        assert_eq!(input.get_value(), 10.0);
    }

    #[test]
    fn test_number_input_increment_clamped() {
        let mut input = number_input().value(95.0).step(10.0).max(100.0);
        input.increment();
        assert_eq!(input.get_value(), 100.0); // Clamped
    }

    #[test]
    fn test_number_input_large_step() {
        let mut input = number_input().value(50.0).step(1.0);
        input.increment_large();
        assert_eq!(input.get_value(), 60.0); // 10x step
    }

    #[test]
    fn test_number_input_format() {
        let input = number_input().value(42.123).precision(2);
        assert_eq!(input.format_value(), "42.12");

        let input = number_input().value(42.0).precision(0);
        assert_eq!(input.format_value(), "42");
    }

    #[test]
    fn test_number_input_display_string() {
        let input = number_input().value(19.99).prefix("$").precision(2);
        assert_eq!(input.display_string(), "$19.99");

        let input = number_input().value(75.0).suffix("%").precision(0);
        assert_eq!(input.display_string(), "75%");
    }

    #[test]
    fn test_number_input_editing() {
        let mut input = number_input().value(10.0);
        input.state.focused = true;

        input.start_editing();
        assert!(input.is_editing());
        assert_eq!(input.input_buffer, "10");

        input.handle_key(&Key::Char('5'));
        assert_eq!(input.input_buffer, "105");

        input.commit_edit();
        assert!(!input.is_editing());
        assert_eq!(input.get_value(), 105.0);
    }

    #[test]
    fn test_number_input_cancel_edit() {
        let mut input = number_input().value(10.0);
        input.state.focused = true;

        input.start_editing();
        input.handle_key(&Key::Char('9'));
        input.handle_key(&Key::Char('9'));
        assert_eq!(input.input_buffer, "1099");

        input.cancel_edit();
        assert!(!input.is_editing());
        assert_eq!(input.get_value(), 10.0); // Original value preserved
    }

    #[test]
    fn test_number_input_key_increment() {
        let mut input = number_input().value(10.0).step(1.0);
        input.state.focused = true;

        input.handle_key(&Key::Up);
        assert_eq!(input.get_value(), 11.0);

        input.handle_key(&Key::Down);
        assert_eq!(input.get_value(), 10.0);

        input.handle_key(&Key::Char('k'));
        assert_eq!(input.get_value(), 11.0);

        input.handle_key(&Key::Char('j'));
        assert_eq!(input.get_value(), 10.0);
    }

    #[test]
    fn test_number_input_page_up_down() {
        let mut input = number_input().value(50.0).step(1.0);
        input.state.focused = true;

        input.handle_key(&Key::PageUp);
        assert_eq!(input.get_value(), 60.0);

        input.handle_key(&Key::PageDown);
        assert_eq!(input.get_value(), 50.0);
    }

    #[test]
    fn test_number_input_home_end() {
        let mut input = number_input().value(50.0).min(0.0).max(100.0);
        input.state.focused = true;

        input.handle_key(&Key::Home);
        assert_eq!(input.get_value(), 0.0);

        input.handle_key(&Key::End);
        assert_eq!(input.get_value(), 100.0);
    }

    #[test]
    fn test_number_input_disabled() {
        let mut input = number_input().value(10.0).disabled(true);

        let handled = input.handle_key(&Key::Up);
        assert!(!handled);
        assert_eq!(input.get_value(), 10.0); // Unchanged
    }

    #[test]
    fn test_number_input_decimal_validation() {
        let mut input = number_input().value(0.0).precision(2);
        input.state.focused = true;
        input.start_editing();
        input.input_buffer.clear();
        input.cursor = 0;

        input.handle_key(&Key::Char('1'));
        input.handle_key(&Key::Char('.'));
        input.handle_key(&Key::Char('5'));
        assert_eq!(input.input_buffer, "1.5");

        // Second decimal should be ignored
        input.handle_key(&Key::Char('.'));
        assert_eq!(input.input_buffer, "1.5");
    }

    #[test]
    fn test_number_input_negative() {
        let mut input = number_input().value(0.0).min(-100.0);
        input.state.focused = true;
        input.start_editing();
        input.input_buffer.clear();
        input.cursor = 0;

        input.handle_key(&Key::Char('-'));
        input.handle_key(&Key::Char('5'));
        assert_eq!(input.input_buffer, "-5");

        // Second minus should be ignored
        input.handle_key(&Key::Char('-'));
        assert_eq!(input.input_buffer, "-5");

        input.commit_edit();
        assert_eq!(input.get_value(), -5.0);
    }

    #[test]
    fn test_integer_input() {
        let input = integer_input().value(42.7);
        assert_eq!(input.format_value(), "43"); // Rounded
        assert_eq!(input.get_int(), 43);
    }

    #[test]
    fn test_currency_input() {
        let input = currency_input("$").value(19.99);
        assert_eq!(input.display_string(), "$19.99");
        assert_eq!(input.get_value(), 19.99);
    }

    #[test]
    fn test_percentage_input() {
        let mut input = percentage_input().value(150.0);
        assert_eq!(input.get_value(), 100.0); // Clamped to max
        assert_eq!(input.display_string(), "100%");

        input.set_value(-10.0);
        assert_eq!(input.get_value(), 0.0); // Clamped to min
    }

    #[test]
    fn test_number_input_render() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let input = number_input().value(42.0).precision(0).focused(true);
        input.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '4');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '2');
    }

    #[test]
    fn test_number_input_render_with_prefix_suffix() {
        let mut buffer = Buffer::new(20, 1);
        let area = Rect::new(0, 0, 20, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let input = currency_input("$").value(9.99).focused(true);
        input.render(&mut ctx);

        assert_eq!(buffer.get(0, 0).unwrap().symbol, '$');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, '9');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, '.');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, '9');
        assert_eq!(buffer.get(4, 0).unwrap().symbol, '9');
    }
}

// Re-exports
pub use core::NumberInput;
pub use helper::{currency_input, integer_input, number_input, percentage_input};
