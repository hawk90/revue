use super::core::VirtualList;

impl<T: ToString + Clone + Default> Default for VirtualList<T> {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

// Manual implementations for generic type
impl<T: ToString + Clone> crate::widget::StyledView for VirtualList<T> {
    fn set_id(&mut self, id: impl Into<String>) {
        self.props.id = Some(id.into());
    }

    fn add_class(&mut self, class: impl Into<String>) {
        let class_str = class.into();
        if !self.props.classes.iter().any(|c| c == &class_str) {
            self.props.classes.push(class_str);
        }
    }

    fn remove_class(&mut self, class: &str) {
        self.props.classes.retain(|c| c != class);
    }

    fn toggle_class(&mut self, class: &str) {
        if self.props.classes.iter().any(|c| c == class) {
            self.props.classes.retain(|c| c != class);
        } else {
            self.props.classes.push(class.to_string());
        }
    }

    fn has_class(&self, class: &str) -> bool {
        self.props.classes.iter().any(|c| c == class)
    }
}

impl<T: ToString + Clone> VirtualList<T> {
    /// Set element ID for CSS selector (#id)
    pub fn element_id(mut self, id: impl Into<String>) -> Self {
        self.props.id = Some(id.into());
        self
    }

    /// Add a CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        let class_str = class.into();
        if !self.props.classes.iter().any(|c| c == &class_str) {
            self.props.classes.push(class_str);
        }
        self
    }

    /// Add multiple CSS classes
    pub fn classes<I, S>(mut self, classes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        for class in classes {
            let class_str = class.into();
            if !self.props.classes.iter().any(|c| c == &class_str) {
                self.props.classes.push(class_str);
            }
        }
        self
    }
}
