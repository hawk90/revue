//! Widget information for debug overlay tree display

use crate::layout::Rect;

/// Information about a widget for debugging
#[derive(Debug, Clone)]
pub struct WidgetInfo {
    /// Widget type name
    pub type_name: String,
    /// Widget ID (if any)
    pub id: Option<String>,
    /// CSS classes
    pub classes: Vec<String>,
    /// Bounding rect
    pub rect: Rect,
    /// Depth in tree
    pub depth: usize,
    /// Is focused
    pub focused: bool,
    /// Is hovered
    pub hovered: bool,
}

impl WidgetInfo {
    /// Create new widget info
    pub fn new(type_name: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            id: None,
            classes: Vec::new(),
            rect: Rect::default(),
            depth: 0,
            focused: false,
            hovered: false,
        }
    }

    /// Set widget ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Add CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Set bounding rect
    pub fn rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    /// Set depth
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Format as tree line
    pub fn tree_line(&self) -> String {
        let indent = "  ".repeat(self.depth);
        let mut line = format!("{}{}", indent, self.type_name);

        if let Some(ref id) = self.id {
            line.push_str(&format!(" #{}", id));
        }

        for class in &self.classes {
            line.push_str(&format!(" .{}", class));
        }

        if self.focused {
            line.push_str(" [focused]");
        }

        if self.hovered {
            line.push_str(" [hover]");
        }

        line
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_info_new_default_values() {
        let info = WidgetInfo::new("TestWidget");
        assert_eq!(info.type_name, "TestWidget");
        assert!(info.id.is_none());
        assert!(info.classes.is_empty());
        assert_eq!(info.depth, 0);
        assert!(!info.focused);
        assert!(!info.hovered);
    }

    #[test]
    fn test_widget_info_builder() {
        let info = WidgetInfo::new("Button")
            .id("submit")
            .class("primary")
            .depth(1);

        let line = info.tree_line();
        assert!(line.contains("Button"));
        assert!(line.contains("#submit"));
        assert!(line.contains(".primary"));
    }

    #[test]
    fn test_widget_info_rect() {
        let info = WidgetInfo::new("Test").rect(Rect::new(5, 10, 20, 30));
        assert_eq!(info.rect.x, 5);
        assert_eq!(info.rect.y, 10);
        assert_eq!(info.rect.width, 20);
        assert_eq!(info.rect.height, 30);
    }

    #[test]
    fn test_widget_info_tree_line_with_focused() {
        let mut info = WidgetInfo::new("Button");
        info.focused = true;
        let line = info.tree_line();
        assert!(line.contains("[focused]"));
    }

    #[test]
    fn test_widget_info_tree_line_with_hovered() {
        let mut info = WidgetInfo::new("Button");
        info.hovered = true;
        let line = info.tree_line();
        assert!(line.contains("[hover]"));
    }

    #[test]
    fn test_widget_info_tree_line_depth() {
        let info = WidgetInfo::new("Widget").depth(2);
        let line = info.tree_line();
        assert!(line.starts_with("    "));
    }
}
