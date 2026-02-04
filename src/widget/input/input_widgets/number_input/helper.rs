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
