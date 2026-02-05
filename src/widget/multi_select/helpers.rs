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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multi_select_function() {
        let select = multi_select();
        let _ = select;
    }

    #[test]
    fn test_multi_select_from_function() {
        let fruits = vec!["Apple", "Banana", "Cherry"];
        let select = multi_select_from(fruits);
        let _ = select;
    }

    #[test]
    fn test_multi_select_from_vec() {
        let fruits = vec!["Apple", "Banana"];
        let select = multi_select_from(fruits.clone());
        let _ = select;
    }

    #[test]
    fn test_multi_select_from_iterator() {
        let fruits = vec!["Apple", "Banana"];
        let select = multi_select_from(fruits.iter().copied());
        let _ = select;
    }

    #[test]
    fn test_multi_select_from_empty() {
        let items: Vec<&str> = vec![];
        let select = multi_select_from(items);
        let _ = select;
    }
}
