//! Element enum for rendered widgets

use super::view::View;

/// A rendered element
#[derive(Default)]
pub enum Element {
    /// Empty element
    #[default]
    Empty,
    /// View element
    View(Box<dyn View>),
}
