//! ComputedProperty implementations

use super::types::ComputedProperty;
use super::types::PropertySource;

impl ComputedProperty {
    /// Create a new computed property
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            source: PropertySource::Computed,
            overridden: false,
        }
    }

    /// Set source
    pub fn source(mut self, source: PropertySource) -> Self {
        self.source = source;
        self
    }

    /// Mark as overridden
    pub fn overridden(mut self) -> Self {
        self.overridden = true;
        self
    }
}
