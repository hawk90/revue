//! JSON Viewer types

#![allow(missing_docs)]

/// JSON value type for styling
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonType {
    /// JSON object `{}`
    Object,
    /// JSON array `[]`
    Array,
    /// JSON string value
    String,
    /// JSON number value
    Number,
    /// JSON boolean (true/false)
    Boolean,
    /// JSON null value
    Null,
}

/// A node in the JSON tree
#[derive(Clone, Debug)]
pub struct JsonNode {
    /// Key name (empty for root or array elements)
    pub key: String,
    /// JSON path to this node
    pub path: String,
    /// Value type
    pub value_type: JsonType,
    /// String representation of value (for leaf nodes)
    pub value: Option<String>,
    /// Child nodes (for objects and arrays)
    pub children: Vec<JsonNode>,
    /// Depth in tree
    pub depth: usize,
    /// Index in flattened list (set during render)
    pub index: usize,
}

impl JsonNode {
    pub fn new(
        key: impl Into<String>,
        path: impl Into<String>,
        value_type: JsonType,
        depth: usize,
    ) -> Self {
        Self {
            key: key.into(),
            path: path.into(),
            value_type,
            value: None,
            children: Vec::new(),
            depth,
            index: 0,
        }
    }

    pub fn with_value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    #[allow(dead_code)]
    pub fn with_children(mut self, children: Vec<JsonNode>) -> Self {
        self.children = children;
        self
    }

    pub fn is_container(&self) -> bool {
        matches!(self.value_type, JsonType::Object | JsonType::Array)
    }

    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}
