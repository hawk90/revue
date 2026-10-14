use super::Popover;

/// Helper function to create a popover
pub fn popover(content: impl Into<String>) -> Popover {
    Popover::new(content)
}
