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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breadcrumb_function() {
        let bc = breadcrumb();
        let _ = bc;
    }

    #[test]
    fn test_crumb_function() {
        let item = crumb("Home");
        assert_eq!(item.label, "Home");
    }

    #[test]
    fn test_crumb_function_with_string() {
        let item = crumb("Folder".to_string());
        assert_eq!(item.label, "Folder");
    }

    #[test]
    fn test_crumb_function_chainable() {
        let item = crumb("File").icon('📄');
        assert_eq!(item.label, "File");
        assert_eq!(item.icon, Some('📄'));
    }

    // =========================================================================
    // Edge cases
    // =========================================================================

    #[test]
    fn test_crumb_empty_label() {
        let item = crumb("");
        assert_eq!(item.label, "");
    }

    #[test]
    fn test_crumb_long_label() {
        let long_label = "A".repeat(100);
        let item = crumb(&long_label);
        assert_eq!(item.label.len(), 100);
    }

    #[test]
    fn test_crumb_with_special_chars() {
        let item = crumb("Path/To/File");
        assert_eq!(item.label, "Path/To/File");
    }

    #[test]
    fn test_crumb_with_unicode() {
        let item = crumb("🏠 Home");
        assert_eq!(item.label, "🏠 Home");
    }

    #[test]
    fn test_breadcrumb_multiple_times() {
        let bc1 = breadcrumb();
        let bc2 = breadcrumb();
        let _ = bc1;
        let _ = bc2;
    }
}
