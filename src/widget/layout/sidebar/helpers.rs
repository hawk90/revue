//! Sidebar helper functions

use super::types::{SidebarItem, SidebarSection};

/// Helper function to create a sidebar
pub fn sidebar() -> super::Sidebar {
    super::Sidebar::new()
}

/// Helper function to create a sidebar item
pub fn sidebar_item(id: impl Into<String>, label: impl Into<String>) -> SidebarItem {
    SidebarItem::new(id, label)
}

/// Helper function to create a sidebar section
pub fn sidebar_section(items: Vec<SidebarItem>) -> SidebarSection {
    SidebarSection::new(items)
}

/// Helper function to create a titled sidebar section
pub fn sidebar_section_titled(title: impl Into<String>, items: Vec<SidebarItem>) -> SidebarSection {
    SidebarSection::titled(title, items)
}

// All tests extracted to tests/widget/layout/sidebar_helpers.rs
