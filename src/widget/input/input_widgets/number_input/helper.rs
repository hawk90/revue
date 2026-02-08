use super::core::NumberInput;

// =============================================================================
// Helper functions
// =============================================================================

/// Create a basic number input widget
///
/// # Example
/// ```rust,ignore
/// let input = number_input().value(42.0);
/// ```
pub fn number_input() -> NumberInput {
    NumberInput::new()
}

/// Create an integer-only number input (precision = 0, step = 1)
///
/// # Example
/// ```rust,ignore
/// let count = integer_input().value(10.0).min(0.0);
/// ```
pub fn integer_input() -> NumberInput {
    NumberInput::new().precision(0).step(1.0)
}

/// Create a currency input with specified symbol
///
/// # Example
/// ```rust,ignore
/// let price = currency_input("$").value(19.99);
/// let euro = currency_input("€").value(15.50);
/// ```
pub fn currency_input(symbol: &str) -> NumberInput {
    NumberInput::new()
        .precision(2)
        .step(0.01)
        .prefix(symbol)
        .min(0.0)
}

/// Create a percentage input (0-100 range with % suffix)
///
/// # Example
/// ```rust,ignore
/// let percent = percentage_input().value(75.0);
/// ```
pub fn percentage_input() -> NumberInput {
    NumberInput::new()
        .precision(0)
        .step(1.0)
        .suffix("%")
        .min(0.0)
        .max(100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // number_input tests
    // =========================================================================

    #[test]
    fn test_number_input() {
        let input = number_input();
        // Just verify it creates without panicking
        let _ = input;
    }

    #[test]
    fn test_number_input_with_value() {
        let input = number_input().value(42.0);
        // Verify builder chain works
        let _ = input;
    }

    // =========================================================================
    // integer_input tests
    // =========================================================================

    #[test]
    fn test_integer_input() {
        let input = integer_input();
        // Verify it creates
        let _ = input;
    }

    #[test]
    fn test_integer_input_with_value() {
        let input = integer_input().value(10.0);
        // Verify builder chain works
        let _ = input;
    }

    #[test]
    fn test_integer_input_chained() {
        let input = integer_input().value(5.0).min(0.0).max(100.0);
        // Verify full builder chain
        let _ = input;
    }

    // =========================================================================
    // currency_input tests
    // =========================================================================

    #[test]
    fn test_currency_input_dollar() {
        let input = currency_input("$");
        // Verify it creates
        let _ = input;
    }

    #[test]
    fn test_currency_input_euro() {
        let input = currency_input("€");
        // Verify it creates
        let _ = input;
    }

    #[test]
    fn test_currency_input_pound() {
        let input = currency_input("£");
        // Verify it creates
        let _ = input;
    }

    #[test]
    fn test_currency_input_yen() {
        let input = currency_input("¥");
        // Verify it creates
        let _ = input;
    }

    #[test]
    fn test_currency_input_with_value() {
        let input = currency_input("$").value(19.99);
        // Verify builder chain works
        let _ = input;
    }

    #[test]
    fn test_currency_input_chained() {
        let input = currency_input("€").value(15.50).min(0.0);
        // Verify full builder chain
        let _ = input;
    }

    #[test]
    fn test_currency_input_symbol() {
        let input1 = currency_input("$");
        let input2 = currency_input("€");
        let input3 = currency_input("£");
        // All should create without panicking
        let _ = (input1, input2, input3);
    }

    // =========================================================================
    // percentage_input tests
    // =========================================================================

    #[test]
    fn test_percentage_input() {
        let input = percentage_input();
        // Verify it creates
        let _ = input;
    }

    #[test]
    fn test_percentage_input_with_value() {
        let input = percentage_input().value(75.0);
        // Verify builder chain works
        let _ = input;
    }

    #[test]
    fn test_percentage_input_chained() {
        let input = percentage_input().value(50.0).min(0.0).max(100.0);
        // Verify full builder chain
        let _ = input;
    }

    // =========================================================================
    // Edge case tests
    // =========================================================================

    #[test]
    fn test_integer_input_negative() {
        let input = integer_input().value(-5.0);
        let _ = input;
    }

    #[test]
    fn test_integer_input_zero() {
        let input = integer_input().value(0.0);
        let _ = input;
    }

    #[test]
    fn test_integer_input_large() {
        let input = integer_input().value(1000000.0);
        let _ = input;
    }

    #[test]
    fn test_currency_input_zero() {
        let input = currency_input("$").value(0.0);
        let _ = input;
    }

    #[test]
    fn test_currency_input_small() {
        let input = currency_input("$").value(0.01);
        let _ = input;
    }

    #[test]
    fn test_currency_input_large() {
        let input = currency_input("$").value(999999.99);
        let _ = input;
    }

    #[test]
    fn test_percentage_input_zero() {
        let input = percentage_input().value(0.0);
        let _ = input;
    }

    #[test]
    fn test_percentage_input_fifty() {
        let input = percentage_input().value(50.0);
        let _ = input;
    }

    #[test]
    fn test_percentage_input_hundred() {
        let input = percentage_input().value(100.0);
        let _ = input;
    }

    #[test]
    fn test_percentage_input_boundary() {
        let input = percentage_input().min(0.0).max(100.0);
        // Verify boundaries can be set
        let _ = input;
    }

    // =========================================================================
    // Unicode symbol tests
    // =========================================================================

    #[test]
    fn test_currency_input_unicode_symbols() {
        let symbols = ["€", "£", "¥", "₹", "₽", "₩"];
        for symbol in symbols {
            let input = currency_input(symbol);
            // All should create without panicking
            let _ = input;
        }
    }

    #[test]
    fn test_currency_input_multi_char_symbol() {
        let input = currency_input("USD");
        let _ = input;
    }

    #[test]
    fn test_currency_input_empty_symbol() {
        let input = currency_input("");
        let _ = input;
    }
}
