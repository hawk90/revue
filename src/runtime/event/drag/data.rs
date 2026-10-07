//! The payload carried by a drag

use std::any::Any;
use std::fmt;

/// Data payload for drag operations
///
/// Wraps any data type for drag-and-drop operations.
/// Use the typed constructors for common cases.
#[derive(Debug)]
pub struct DragData {
    /// Type identifier for matching drop targets
    pub type_id: &'static str,
    /// The actual data (type-erased)
    data: Box<dyn Any + Send + Sync>,
    /// Optional display label
    pub label: Option<String>,
}

impl DragData {
    /// Create drag data with a custom type
    pub fn new<T: Any + Send + Sync + fmt::Debug>(type_id: &'static str, data: T) -> Self {
        Self {
            type_id,
            data: Box::new(data),
            label: None,
        }
    }

    /// Create drag data for text
    pub fn text(value: impl Into<String>) -> Self {
        let s: String = value.into();
        Self {
            type_id: "text",
            label: Some(s.clone()),
            data: Box::new(s),
        }
    }

    /// Create drag data for a file path
    pub fn file(path: impl Into<String>) -> Self {
        let p: String = path.into();
        Self {
            type_id: "file",
            label: Some(p.clone()),
            data: Box::new(p),
        }
    }

    /// Create drag data for a list item index
    pub fn list_item(index: usize, label: impl Into<String>) -> Self {
        Self {
            type_id: "list_item",
            label: Some(label.into()),
            data: Box::new(index),
        }
    }

    /// Create drag data for a tree node
    pub fn tree_node(node_id: impl Into<String>, label: impl Into<String>) -> Self {
        let id: String = node_id.into();
        Self {
            type_id: "tree_node",
            label: Some(label.into()),
            data: Box::new(id),
        }
    }

    /// Set display label
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Get the data as a specific type
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.data.downcast_ref::<T>()
    }

    /// Get text data if this is a text drag
    pub fn as_text(&self) -> Option<&str> {
        if self.type_id == "text" || self.type_id == "file" || self.type_id == "tree_node" {
            self.get::<String>().map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Get list item index if this is a list item drag
    pub fn as_list_index(&self) -> Option<usize> {
        if self.type_id == "list_item" {
            self.get::<usize>().copied()
        } else {
            None
        }
    }

    /// Check if this drag data matches a type
    pub fn is_type(&self, type_id: &str) -> bool {
        self.type_id == type_id
    }

    /// Get display label for rendering
    pub fn display_label(&self) -> &str {
        self.label.as_deref().unwrap_or("...")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // DragData tests
    // =========================================================================

    #[test]
    fn test_drag_data_new() {
        let data = DragData::new("custom", (42i32,));
        assert_eq!(data.type_id, "custom");
        assert!(data.label.is_none());
    }

    #[test]
    fn test_drag_data_text() {
        let data = DragData::text("Hello");
        assert_eq!(data.type_id, "text");
        assert_eq!(data.as_text(), Some("Hello"));
        assert_eq!(data.display_label(), "Hello");
    }

    #[test]
    fn test_drag_data_file() {
        let data = DragData::file("/path/to/file.txt");
        assert_eq!(data.type_id, "file");
        assert_eq!(data.as_text(), Some("/path/to/file.txt"));
    }

    #[test]
    fn test_drag_data_list_item() {
        let data = DragData::list_item(5, "Item 5");
        assert_eq!(data.type_id, "list_item");
        assert_eq!(data.as_list_index(), Some(5));
    }

    #[test]
    fn test_drag_data_tree_node() {
        let data = DragData::tree_node("node123", "Label");
        assert_eq!(data.type_id, "tree_node");
        assert_eq!(data.as_text(), Some("node123"));
    }

    #[test]
    fn test_drag_data_with_label() {
        let data = DragData::new("custom", 123).with_label("Custom Label");
        assert_eq!(data.display_label(), "Custom Label");
    }

    #[test]
    fn test_drag_data_is_type() {
        let data = DragData::text("test");
        assert!(data.is_type("text"));
        assert!(!data.is_type("file"));
    }

    #[test]
    fn test_drag_data_get() {
        let data = DragData::new("custom", 42i32);
        assert!(data.get::<i32>().is_some());
        assert!(data.get::<String>().is_none());
    }
}
