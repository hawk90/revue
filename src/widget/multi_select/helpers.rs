//! Constructor functions for the multi-select widget

use super::types::MultiSelect;

/// Create a basic multi-select widget
///
/// # Example
/// ```rust,ignore
/// let select = multi_select()
///     .option("Apple")
///     .option("Banana");
/// ```
pub fn multi_select() -> MultiSelect {
    MultiSelect::new()
}

/// Create a multi-select from an iterable of strings
///
/// # Example
/// ```rust,ignore
/// let fruits = vec!["Apple", "Banana", "Cherry"];
/// let select = multi_select_from(fruits);
/// ```
pub fn multi_select_from<I, S>(items: I) -> MultiSelect
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    MultiSelect::new().options(items.into_iter().map(|s| s.into()).collect())
}
