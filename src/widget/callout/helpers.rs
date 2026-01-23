//! Helper functions for creating callouts

use super::core::Callout;

/// Helper function to create a Callout
pub fn callout(content: impl Into<String>) -> Callout {
    Callout::new(content)
}

/// Helper function to create a note Callout
pub fn note(content: impl Into<String>) -> Callout {
    Callout::note(content)
}

/// Helper function to create a tip Callout
pub fn tip(content: impl Into<String>) -> Callout {
    Callout::tip(content)
}

/// Helper function to create an important Callout
pub fn important(content: impl Into<String>) -> Callout {
    Callout::important(content)
}

/// Helper function to create a warning Callout
pub fn warning_callout(content: impl Into<String>) -> Callout {
    Callout::warning(content)
}

/// Helper function to create a danger Callout
pub fn danger(content: impl Into<String>) -> Callout {
    Callout::danger(content)
}

/// Helper function to create an info Callout
pub fn info_callout(content: impl Into<String>) -> Callout {
    Callout::info(content)
}
