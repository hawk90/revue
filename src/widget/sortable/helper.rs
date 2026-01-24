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
