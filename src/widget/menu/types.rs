//! Menu widget types

/// Menu item action
pub type MenuAction = Box<dyn Fn() + 'static>;

/// Menu item
pub struct MenuItem {
    /// Item label
    pub label: String,
    /// Keyboard shortcut display
    pub shortcut: Option<String>,
    /// Item is disabled
    pub disabled: bool,
    /// Item is checked (for toggle items)
    pub checked: Option<bool>,
    /// Submenu items
    pub submenu: Vec<MenuItem>,
    /// Is a separator
    pub separator: bool,
    /// Action callback
    action: Option<MenuAction>,
}

impl MenuItem {
    /// Create a new menu item
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            disabled: false,
            checked: None,
            submenu: Vec::new(),
            separator: false,
            action: None,
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        Self {
            label: String::new(),
            shortcut: None,
            disabled: false,
            checked: None,
            submenu: Vec::new(),
            separator: true,
            action: None,
        }
    }

    /// Set keyboard shortcut display
    pub fn shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set checked state
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Add submenu items
    pub fn submenu(mut self, items: Vec<MenuItem>) -> Self {
        self.submenu = items;
        self
    }

    /// Set action callback
    pub fn on_select<F: Fn() + 'static>(mut self, action: F) -> Self {
        self.action = Some(Box::new(action));
        self
    }

    /// Execute action if available
    pub fn execute(&self) {
        if let Some(ref action) = self.action {
            if !self.disabled {
                action();
            }
        }
    }

    /// Has submenu
    pub fn has_submenu(&self) -> bool {
        !self.submenu.is_empty()
    }
}

/// Menu (top-level or submenu)
pub struct Menu {
    /// Menu title
    pub title: String,
    /// Menu items
    pub items: Vec<MenuItem>,
}

impl Menu {
    /// Create a new menu
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
        }
    }

    /// Add an item
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add multiple items
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Add a separator
    pub fn separator(mut self) -> Self {
        self.items.push(MenuItem::separator());
        self
    }
}
