//! Mutable id and class access for CSS styling

use super::view::View;

/// Extended View trait with styling support
///
/// This trait provides runtime mutable access to CSS styling properties.
/// Unlike the base `View` trait which returns immutable references,
/// `StyledView` allows modifying IDs and classes after widget creation.
///
/// # Example
///
/// ```ignore
/// struct MyWidget {
///     id: String,
///     classes: Vec<String>,
/// }
///
/// impl StyledView for MyWidget {
///     fn set_id(&mut self, id: impl Into<String>) {
///         self.id = id.into();
///     }
///
///     fn add_class(&mut self, class: impl Into<String>) {
///         self.classes.push(class.into());
///     }
///
///     fn remove_class(&mut self, class: &str) {
///         self.classes.retain(|c| c != class);
///     }
///
///     fn toggle_class(&mut self, class: &str) {
///         if self.has_class(class) {
///             self.remove_class(class);
///         } else {
///             self.add_class(class);
///         }
///     }
///
///     fn has_class(&self, class: &str) -> bool {
///         self.classes.iter().any(|c| c == class)
///     }
/// }
/// ```
pub trait StyledView: View {
    /// Set element ID
    fn set_id(&mut self, id: impl Into<String>);

    /// Add a CSS class
    fn add_class(&mut self, class: impl Into<String>);

    /// Remove a CSS class
    fn remove_class(&mut self, class: &str);

    /// Toggle a CSS class
    fn toggle_class(&mut self, class: &str);

    /// Check if has class
    fn has_class(&self, class: &str) -> bool;
}
