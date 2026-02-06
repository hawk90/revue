//! Helper function for creating sortable lists

use super::core::SortableList;

/// Create a sortable list
pub fn sortable_list<I, S>(items: I) -> SortableList
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    SortableList::new(items)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sortable_list_function() {
        let list = sortable_list(vec!["a", "b", "c"]);
        let _ = list;
    }

    #[test]
    fn test_sortable_list_from_vec() {
        let items = vec!["x", "y", "z"];
        let list = sortable_list(items);
        let _ = list;
    }

    #[test]
    fn test_sortable_list_from_iterator() {
        let list = sortable_list(["apple", "banana"].iter().copied());
        let _ = list;
    }

    #[test]
    fn test_sortable_list_empty() {
        let items: Vec<&str> = vec![];
        let list = sortable_list(items);
        let _ = list;
    }
}
