use super::core::VirtualList;

/// Helper function to create a virtual list
pub fn virtual_list<T: ToString + Clone>(items: Vec<T>) -> VirtualList<T> {
    VirtualList::new(items)
}
