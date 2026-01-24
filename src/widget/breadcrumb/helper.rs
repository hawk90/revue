//! Breadcrumb navigation widget - helper functions

use super::core::Breadcrumb;
use super::types::BreadcrumbItem;

/// Helper to create a breadcrumb
pub fn breadcrumb() -> Breadcrumb {
    Breadcrumb::new()
}

/// Helper to create a breadcrumb item
pub fn crumb(label: impl Into<String>) -> BreadcrumbItem {
    BreadcrumbItem::new(label)
}
