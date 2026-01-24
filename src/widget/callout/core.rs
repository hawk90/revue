//! Callout widget core struct

use super::types::{CalloutType, CalloutVariant};
use crate::widget::traits::WidgetState;

/// A callout widget for highlighting important information
///
/// Provides predefined styles for notes, tips, warnings, and other
/// important information blocks commonly used in documentation.
#[derive(Clone)]
pub struct Callout {
    /// Main content text
    pub content: String,
    /// Optional title (defaults to type name)
    pub title: Option<String>,
    /// Callout type
    pub callout_type: CalloutType,
    /// Visual variant
    pub variant: CalloutVariant,
    /// Show icon
    pub show_icon: bool,
    /// Custom icon override
    pub custom_icon: Option<char>,
    /// Whether the callout is collapsible
    pub collapsible: bool,
    /// Whether the callout is expanded (when collapsible)
    pub expanded: bool,
    /// Icon when collapsed
    pub collapsed_icon: char,
    /// Icon when expanded
    pub expanded_icon: char,
    /// Widget state
    pub state: WidgetState,
    /// Widget properties
    pub props: crate::widget::traits::WidgetProps,
}
