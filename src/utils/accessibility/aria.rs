//! ARIA attribute generation utilities
//!
//! Converts accessibility information to ARIA attributes for semantic markup.
//!
//! # Example
//!
//! ```rust
//! use revue::utils::accessibility::{Role, AccessibleNode};
//! use revue::utils::accessibility::aria::{AriaAttribute, AriaBuilder};
//!
//! let node = AccessibleNode::new(Role::Button)
//!     .label("Submit")
//!     .description("Submit the form");
//!
//! let aria = AriaBuilder::new()
//!     .from_node(&node)
//!     .build();
//!
//! assert_eq!(aria.get("role"), Some(&"button".to_string()));
//! assert_eq!(aria.get("aria-label"), Some(&"Submit".to_string()));
//! ```

use super::AccessibleNode;
use std::collections::HashMap;

/// ARIA attribute
#[derive(Clone, Debug, PartialEq)]
pub enum AriaAttribute {
    /// Role attribute
    Role(String),
    /// aria-label
    Label(String),
    /// aria-labelledby
    LabelledBy(String),
    /// aria-describedby
    DescribedBy(String),
    /// aria-live (polite, assertive, off)
    Live(String),
    /// aria-atomic
    Atomic(bool),
    /// aria-hidden
    Hidden(bool),
    /// aria-expanded
    Expanded(Option<bool>),
    /// aria-checked
    Checked(Option<bool>),
    /// aria-selected
    Selected(bool),
    /// aria-pressed
    Pressed(Option<bool>),
    /// aria-disabled
    Disabled(bool),
    /// aria-readonly
    ReadOnly(bool),
    /// aria-required
    Required(bool),
    /// aria-invalid
    Invalid(bool),
    /// aria-modal
    Modal(bool),
    /// aria-current
    Current(Option<String>),
    /// aria-haspopup
    HasPopup(Option<String>),
    /// aria-orientation
    Orientation(String),
    /// aria-valuemin
    ValueMin(f64),
    /// aria-valuemax
    ValueMax(f64),
    /// aria-valuenow
    ValueNow(f64),
    /// aria-valuetext
    ValueText(String),
    /// aria-posinset
    PosInSet(usize),
    /// aria-setsize
    SetSize(usize),
    /// aria-level
    Level(usize),
    /// aria-errormessage
    ErrorMessage(String),
    /// tabindex
    TabIndex(i16),
    /// alt text for images
    Alt(String),
    /// Custom attribute
    Custom(String, String),
}

impl AriaAttribute {
    /// Get attribute name
    pub fn name(&self) -> &str {
        match self {
            AriaAttribute::Role(_) => "role",
            AriaAttribute::Label(_) => "aria-label",
            AriaAttribute::LabelledBy(_) => "aria-labelledby",
            AriaAttribute::DescribedBy(_) => "aria-describedby",
            AriaAttribute::Live(_) => "aria-live",
            AriaAttribute::Atomic(_) => "aria-atomic",
            AriaAttribute::Hidden(_) => "aria-hidden",
            AriaAttribute::Expanded(_) => "aria-expanded",
            AriaAttribute::Checked(_) => "aria-checked",
            AriaAttribute::Selected(_) => "aria-selected",
            AriaAttribute::Pressed(_) => "aria-pressed",
            AriaAttribute::Disabled(_) => "aria-disabled",
            AriaAttribute::ReadOnly(_) => "aria-readonly",
            AriaAttribute::Required(_) => "aria-required",
            AriaAttribute::Invalid(_) => "aria-invalid",
            AriaAttribute::Modal(_) => "aria-modal",
            AriaAttribute::Current(_) => "aria-current",
            AriaAttribute::HasPopup(_) => "aria-haspopup",
            AriaAttribute::Orientation(_) => "aria-orientation",
            AriaAttribute::ValueMin(_) => "aria-valuemin",
            AriaAttribute::ValueMax(_) => "aria-valuemax",
            AriaAttribute::ValueNow(_) => "aria-valuenow",
            AriaAttribute::ValueText(_) => "aria-valuetext",
            AriaAttribute::PosInSet(_) => "aria-posinset",
            AriaAttribute::SetSize(_) => "aria-setsize",
            AriaAttribute::Level(_) => "aria-level",
            AriaAttribute::ErrorMessage(_) => "aria-errormessage",
            AriaAttribute::TabIndex(_) => "tabindex",
            AriaAttribute::Alt(_) => "alt",
            AriaAttribute::Custom(name, _) => name,
        }
    }

    /// Get attribute value
    pub fn value(&self) -> String {
        match self {
            AriaAttribute::Role(v) => v.clone(),
            AriaAttribute::Label(v) => v.clone(),
            AriaAttribute::LabelledBy(v) => v.clone(),
            AriaAttribute::DescribedBy(v) => v.clone(),
            AriaAttribute::Live(v) => v.clone(),
            AriaAttribute::Atomic(v) => v.to_string(),
            AriaAttribute::Hidden(v) => v.to_string(),
            AriaAttribute::Expanded(Some(v)) => v.to_string(),
            AriaAttribute::Expanded(None) => "undefined".to_string(),
            AriaAttribute::Checked(Some(v)) => v.to_string(),
            AriaAttribute::Checked(None) => "mixed".to_string(),
            AriaAttribute::Selected(v) => v.to_string(),
            AriaAttribute::Pressed(Some(v)) => v.to_string(),
            AriaAttribute::Pressed(None) => "mixed".to_string(),
            AriaAttribute::Disabled(v) => v.to_string(),
            AriaAttribute::ReadOnly(v) => v.to_string(),
            AriaAttribute::Required(v) => v.to_string(),
            AriaAttribute::Invalid(v) => v.to_string(),
            AriaAttribute::Modal(v) => v.to_string(),
            AriaAttribute::Current(Some(v)) => v.clone(),
            AriaAttribute::Current(None) => "false".to_string(),
            AriaAttribute::HasPopup(Some(v)) => v.clone(),
            AriaAttribute::HasPopup(None) => "false".to_string(),
            AriaAttribute::Orientation(v) => v.clone(),
            AriaAttribute::ValueMin(v) => v.to_string(),
            AriaAttribute::ValueMax(v) => v.to_string(),
            AriaAttribute::ValueNow(v) => v.to_string(),
            AriaAttribute::ValueText(v) => v.clone(),
            AriaAttribute::PosInSet(v) => v.to_string(),
            AriaAttribute::SetSize(v) => v.to_string(),
            AriaAttribute::Level(v) => v.to_string(),
            AriaAttribute::ErrorMessage(v) => v.clone(),
            AriaAttribute::TabIndex(v) => v.to_string(),
            AriaAttribute::Alt(v) => v.clone(),
            AriaAttribute::Custom(_, v) => v.clone(),
        }
    }
}

/// Live region politeness level
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum LiveRegion {
    /// Do not announce changes
    Off,
    /// Announce when user is idle (default)
    #[default]
    Polite,
    /// Announce immediately
    Assertive,
}

impl LiveRegion {
    /// Get the string representation of the live region setting
    pub fn as_str(&self) -> &'static str {
        match self {
            LiveRegion::Off => "off",
            LiveRegion::Polite => "polite",
            LiveRegion::Assertive => "assertive",
        }
    }
}

/// Builder for constructing ARIA attributes
pub struct AriaBuilder {
    attributes: Vec<AriaAttribute>,
    live_region: Option<LiveRegion>,
    atomic: bool,
}

impl Default for AriaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AriaBuilder {
    /// Create new builder
    pub fn new() -> Self {
        Self {
            attributes: Vec::new(),
            live_region: None,
            atomic: false,
        }
    }

    /// Add attribute from accessible node
    pub fn from_node(mut self, node: &AccessibleNode) -> Self {
        // Role
        self.attributes
            .push(AriaAttribute::Role(node.role.name().to_string()));

        // Label
        if let Some(label) = &node.label {
            self.attributes.push(AriaAttribute::Label(label.clone()));
        }

        // Description
        if let Some(desc) = &node.description {
            self.attributes
                .push(AriaAttribute::DescribedBy(desc.clone()));
        }

        // State attributes
        let state = &node.state;
        if state.disabled {
            self.attributes.push(AriaAttribute::Disabled(true));
        }
        if state.hidden {
            self.attributes.push(AriaAttribute::Hidden(true));
        }
        if let Some(expanded) = state.expanded {
            self.attributes
                .push(AriaAttribute::Expanded(Some(expanded)));
        }
        if let Some(checked) = state.checked {
            self.attributes.push(AriaAttribute::Checked(Some(checked)));
        }
        if state.selected {
            self.attributes.push(AriaAttribute::Selected(true));
        }
        if let Some(pressed) = state.pressed {
            self.attributes.push(AriaAttribute::Pressed(Some(pressed)));
        }
        if let Some(now) = state.value_now {
            self.attributes.push(AriaAttribute::ValueNow(now));
        }
        if let Some(min) = state.value_min {
            self.attributes.push(AriaAttribute::ValueMin(min));
        }
        if let Some(max) = state.value_max {
            self.attributes.push(AriaAttribute::ValueMax(max));
        }
        if let Some(text) = &state.value_text {
            self.attributes.push(AriaAttribute::ValueText(text.clone()));
        }
        if let Some(pos) = state.pos_in_set {
            self.attributes.push(AriaAttribute::PosInSet(pos));
        }
        if let Some(size) = state.set_size {
            self.attributes.push(AriaAttribute::SetSize(size));
        }
        if let Some(level) = state.level {
            self.attributes.push(AriaAttribute::Level(level));
        }
        if let Some(err) = &state.error_message {
            self.attributes
                .push(AriaAttribute::ErrorMessage(err.clone()));
        }

        // Tabindex based on focusability
        if node.is_focusable() {
            self.attributes.push(AriaAttribute::TabIndex(0));
        } else if !node.role.is_interactive() {
            self.attributes.push(AriaAttribute::TabIndex(-1));
        }

        // Custom properties
        for (key, value) in &node.properties {
            if key.starts_with("aria-") {
                self.attributes
                    .push(AriaAttribute::Custom(key.clone(), value.clone()));
            }
        }

        self
    }

    /// Set live region
    pub fn live_region(mut self, live: LiveRegion) -> Self {
        self.live_region = Some(live);
        self.attributes
            .push(AriaAttribute::Live(live.as_str().to_string()));
        self
    }

    /// Set atomic (for live regions)
    pub fn atomic(mut self, atomic: bool) -> Self {
        self.atomic = atomic;
        self.attributes.push(AriaAttribute::Atomic(atomic));
        self
    }

    /// Add labelled by reference
    pub fn labelled_by(mut self, id: impl Into<String>) -> Self {
        self.attributes.push(AriaAttribute::LabelledBy(id.into()));
        self
    }

    /// Add described by reference
    pub fn described_by(mut self, id: impl Into<String>) -> Self {
        self.attributes.push(AriaAttribute::DescribedBy(id.into()));
        self
    }

    /// Add modal attribute
    pub fn modal(mut self, modal: bool) -> Self {
        self.attributes.push(AriaAttribute::Modal(modal));
        self
    }

    /// Add has popup attribute
    pub fn has_popup(mut self, has: bool, popup_type: Option<&str>) -> Self {
        let value = if has {
            Some(popup_type.unwrap_or("true").to_string())
        } else {
            None
        };
        self.attributes.push(AriaAttribute::HasPopup(value));
        self
    }

    /// Add orientation attribute
    pub fn orientation(mut self, orientation: &str) -> Self {
        self.attributes
            .push(AriaAttribute::Orientation(orientation.to_string()));
        self
    }

    /// Build into attribute map
    pub fn build(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for attr in self.attributes {
            map.insert(attr.name().to_string(), attr.value());
        }
        map
    }

    /// Build as vector of (name, value) pairs
    pub fn build_pairs(self) -> Vec<(String, String)> {
        self.attributes
            .into_iter()
            .map(|attr| (attr.name().to_string(), attr.value()))
            .collect()
    }

    /// Get live region setting
    pub fn get_live_region(&self) -> Option<LiveRegion> {
        self.live_region
    }

    /// Get atomic setting
    pub fn is_atomic(&self) -> bool {
        self.atomic
    }
}

/// Helper to generate ARIA attributes from an accessible node
pub fn aria_attributes(node: &AccessibleNode) -> HashMap<String, String> {
    AriaBuilder::new().from_node(node).build()
}

/// Helper to generate ARIA attribute pairs from an accessible node
pub fn aria_pairs(node: &AccessibleNode) -> Vec<(String, String)> {
    AriaBuilder::new().from_node(node).build_pairs()
}
