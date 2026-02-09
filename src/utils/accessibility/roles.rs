//! ARIA-like roles for widgets

/// ARIA-like roles for widgets
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Role {
    /// Generic container
    Generic,
    /// Button that can be clicked
    Button,
    /// Checkbox (checked/unchecked)
    Checkbox,
    /// Radio button in a group
    Radio,
    /// Text input field
    TextInput,
    /// Multi-line text area
    TextArea,
    /// Dropdown select
    Select,
    /// List of items
    List,
    /// Item in a list
    ListItem,
    /// Tree view
    Tree,
    /// Item in a tree
    TreeItem,
    /// Tab in a tab list
    Tab,
    /// Tab panel content
    TabPanel,
    /// Menu
    Menu,
    /// Menu item
    MenuItem,
    /// Dialog/modal
    Dialog,
    /// Alert message
    Alert,
    /// Status message
    Status,
    /// Progress indicator
    Progress,
    /// Slider control
    Slider,
    /// Navigation region
    Navigation,
    /// Main content region
    Main,
    /// Header region
    Header,
    /// Footer region
    Footer,
    /// Search region
    Search,
    /// Form
    Form,
    /// Table
    Table,
    /// Row in a table
    Row,
    /// Cell in a table
    Cell,
    /// Column header
    ColumnHeader,
    /// Row header
    RowHeader,
    /// Group of related items
    Group,
    /// Tooltip
    Tooltip,
    /// Image
    Image,
    /// Link
    Link,
    /// Separator
    Separator,
    /// Toolbar
    Toolbar,
}

impl Role {
    /// Get role name as string
    pub fn name(&self) -> &'static str {
        match self {
            Role::Generic => "generic",
            Role::Button => "button",
            Role::Checkbox => "checkbox",
            Role::Radio => "radio",
            Role::TextInput => "textbox",
            Role::TextArea => "textbox",
            Role::Select => "combobox",
            Role::List => "list",
            Role::ListItem => "listitem",
            Role::Tree => "tree",
            Role::TreeItem => "treeitem",
            Role::Tab => "tab",
            Role::TabPanel => "tabpanel",
            Role::Menu => "menu",
            Role::MenuItem => "menuitem",
            Role::Dialog => "dialog",
            Role::Alert => "alert",
            Role::Status => "status",
            Role::Progress => "progressbar",
            Role::Slider => "slider",
            Role::Navigation => "navigation",
            Role::Main => "main",
            Role::Header => "banner",
            Role::Footer => "contentinfo",
            Role::Search => "search",
            Role::Form => "form",
            Role::Table => "table",
            Role::Row => "row",
            Role::Cell => "cell",
            Role::ColumnHeader => "columnheader",
            Role::RowHeader => "rowheader",
            Role::Group => "group",
            Role::Tooltip => "tooltip",
            Role::Image => "img",
            Role::Link => "link",
            Role::Separator => "separator",
            Role::Toolbar => "toolbar",
        }
    }

    /// Check if role is interactive (can receive focus)
    pub fn is_interactive(&self) -> bool {
        matches!(
            self,
            Role::Button
                | Role::Checkbox
                | Role::Radio
                | Role::TextInput
                | Role::TextArea
                | Role::Select
                | Role::ListItem
                | Role::TreeItem
                | Role::Tab
                | Role::MenuItem
                | Role::Slider
                | Role::Link
        )
    }

    /// Check if role is a landmark
    pub fn is_landmark(&self) -> bool {
        matches!(
            self,
            Role::Navigation | Role::Main | Role::Header | Role::Footer | Role::Search | Role::Form
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Role::name() tests
    // =========================================================================

    #[test]
    fn test_role_name_generic() {
        assert_eq!(Role::Generic.name(), "generic");
    }

    #[test]
    fn test_role_name_button() {
        assert_eq!(Role::Button.name(), "button");
    }

    #[test]
    fn test_role_name_checkbox() {
        assert_eq!(Role::Checkbox.name(), "checkbox");
    }

    #[test]
    fn test_role_name_radio() {
        assert_eq!(Role::Radio.name(), "radio");
    }

    #[test]
    fn test_role_name_text_input() {
        assert_eq!(Role::TextInput.name(), "textbox");
    }

    #[test]
    fn test_role_name_text_area() {
        assert_eq!(Role::TextArea.name(), "textbox");
    }

    #[test]
    fn test_role_name_select() {
        assert_eq!(Role::Select.name(), "combobox");
    }

    #[test]
    fn test_role_name_list() {
        assert_eq!(Role::List.name(), "list");
    }

    #[test]
    fn test_role_name_list_item() {
        assert_eq!(Role::ListItem.name(), "listitem");
    }

    #[test]
    fn test_role_name_tree() {
        assert_eq!(Role::Tree.name(), "tree");
    }

    #[test]
    fn test_role_name_tree_item() {
        assert_eq!(Role::TreeItem.name(), "treeitem");
    }

    #[test]
    fn test_role_name_tab() {
        assert_eq!(Role::Tab.name(), "tab");
    }

    #[test]
    fn test_role_name_tab_panel() {
        assert_eq!(Role::TabPanel.name(), "tabpanel");
    }

    #[test]
    fn test_role_name_menu() {
        assert_eq!(Role::Menu.name(), "menu");
    }

    #[test]
    fn test_role_name_menu_item() {
        assert_eq!(Role::MenuItem.name(), "menuitem");
    }

    #[test]
    fn test_role_name_dialog() {
        assert_eq!(Role::Dialog.name(), "dialog");
    }

    #[test]
    fn test_role_name_alert() {
        assert_eq!(Role::Alert.name(), "alert");
    }

    #[test]
    fn test_role_name_status() {
        assert_eq!(Role::Status.name(), "status");
    }

    #[test]
    fn test_role_name_progress() {
        assert_eq!(Role::Progress.name(), "progressbar");
    }

    #[test]
    fn test_role_name_slider() {
        assert_eq!(Role::Slider.name(), "slider");
    }

    #[test]
    fn test_role_name_navigation() {
        assert_eq!(Role::Navigation.name(), "navigation");
    }

    #[test]
    fn test_role_name_main() {
        assert_eq!(Role::Main.name(), "main");
    }

    #[test]
    fn test_role_name_header() {
        assert_eq!(Role::Header.name(), "banner");
    }

    #[test]
    fn test_role_name_footer() {
        assert_eq!(Role::Footer.name(), "contentinfo");
    }

    #[test]
    fn test_role_name_search() {
        assert_eq!(Role::Search.name(), "search");
    }

    #[test]
    fn test_role_name_form() {
        assert_eq!(Role::Form.name(), "form");
    }

    #[test]
    fn test_role_name_table() {
        assert_eq!(Role::Table.name(), "table");
    }

    #[test]
    fn test_role_name_row() {
        assert_eq!(Role::Row.name(), "row");
    }

    #[test]
    fn test_role_name_cell() {
        assert_eq!(Role::Cell.name(), "cell");
    }

    #[test]
    fn test_role_name_column_header() {
        assert_eq!(Role::ColumnHeader.name(), "columnheader");
    }

    #[test]
    fn test_role_name_row_header() {
        assert_eq!(Role::RowHeader.name(), "rowheader");
    }

    #[test]
    fn test_role_name_group() {
        assert_eq!(Role::Group.name(), "group");
    }

    #[test]
    fn test_role_name_tooltip() {
        assert_eq!(Role::Tooltip.name(), "tooltip");
    }

    #[test]
    fn test_role_name_image() {
        assert_eq!(Role::Image.name(), "img");
    }

    #[test]
    fn test_role_name_link() {
        assert_eq!(Role::Link.name(), "link");
    }

    #[test]
    fn test_role_name_separator() {
        assert_eq!(Role::Separator.name(), "separator");
    }

    #[test]
    fn test_role_name_toolbar() {
        assert_eq!(Role::Toolbar.name(), "toolbar");
    }

    // =========================================================================
    // Role::is_interactive() tests
    // =========================================================================

    #[test]
    fn test_role_button_is_interactive() {
        assert!(Role::Button.is_interactive());
    }

    #[test]
    fn test_role_checkbox_is_interactive() {
        assert!(Role::Checkbox.is_interactive());
    }

    #[test]
    fn test_role_radio_is_interactive() {
        assert!(Role::Radio.is_interactive());
    }

    #[test]
    fn test_role_text_input_is_interactive() {
        assert!(Role::TextInput.is_interactive());
    }

    #[test]
    fn test_role_text_area_is_interactive() {
        assert!(Role::TextArea.is_interactive());
    }

    #[test]
    fn test_role_select_is_interactive() {
        assert!(Role::Select.is_interactive());
    }

    #[test]
    fn test_role_list_item_is_interactive() {
        assert!(Role::ListItem.is_interactive());
    }

    #[test]
    fn test_role_tree_item_is_interactive() {
        assert!(Role::TreeItem.is_interactive());
    }

    #[test]
    fn test_role_tab_is_interactive() {
        assert!(Role::Tab.is_interactive());
    }

    #[test]
    fn test_role_menu_item_is_interactive() {
        assert!(Role::MenuItem.is_interactive());
    }

    #[test]
    fn test_role_slider_is_interactive() {
        assert!(Role::Slider.is_interactive());
    }

    #[test]
    fn test_role_link_is_interactive() {
        assert!(Role::Link.is_interactive());
    }

    #[test]
    fn test_role_generic_not_interactive() {
        assert!(!Role::Generic.is_interactive());
    }

    #[test]
    fn test_role_list_not_interactive() {
        assert!(!Role::List.is_interactive());
    }

    #[test]
    fn test_role_tree_not_interactive() {
        assert!(!Role::Tree.is_interactive());
    }

    #[test]
    fn test_role_tab_panel_not_interactive() {
        assert!(!Role::TabPanel.is_interactive());
    }

    #[test]
    fn test_role_menu_not_interactive() {
        assert!(!Role::Menu.is_interactive());
    }

    #[test]
    fn test_role_dialog_not_interactive() {
        assert!(!Role::Dialog.is_interactive());
    }

    #[test]
    fn test_role_alert_not_interactive() {
        assert!(!Role::Alert.is_interactive());
    }

    #[test]
    fn test_role_status_not_interactive() {
        assert!(!Role::Status.is_interactive());
    }

    #[test]
    fn test_role_progress_not_interactive() {
        assert!(!Role::Progress.is_interactive());
    }

    // =========================================================================
    // Role::is_landmark() tests
    // =========================================================================

    #[test]
    fn test_role_navigation_is_landmark() {
        assert!(Role::Navigation.is_landmark());
    }

    #[test]
    fn test_role_main_is_landmark() {
        assert!(Role::Main.is_landmark());
    }

    #[test]
    fn test_role_header_is_landmark() {
        assert!(Role::Header.is_landmark());
    }

    #[test]
    fn test_role_footer_is_landmark() {
        assert!(Role::Footer.is_landmark());
    }

    #[test]
    fn test_role_search_is_landmark() {
        assert!(Role::Search.is_landmark());
    }

    #[test]
    fn test_role_form_is_landmark() {
        assert!(Role::Form.is_landmark());
    }

    #[test]
    fn test_role_generic_not_landmark() {
        assert!(!Role::Generic.is_landmark());
    }

    #[test]
    fn test_role_button_not_landmark() {
        assert!(!Role::Button.is_landmark());
    }

    #[test]
    fn test_role_list_not_landmark() {
        assert!(!Role::List.is_landmark());
    }

    #[test]
    fn test_role_table_not_landmark() {
        assert!(!Role::Table.is_landmark());
    }

    // =========================================================================
    // Role trait implementation tests
    // =========================================================================

    #[test]
    fn test_role_partial_eq() {
        assert_eq!(Role::Button, Role::Button);
        assert_ne!(Role::Button, Role::Checkbox);
    }

    #[test]
    fn test_role_clone() {
        let role = Role::Button;
        let cloned = role.clone();
        assert_eq!(role, cloned);
    }

    #[test]
    fn test_role_copy() {
        let role = Role::Button;
        let copied = role;
        assert_eq!(role, Role::Button);
        assert_eq!(copied, Role::Button);
    }

    #[test]
    fn test_role_hash() {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        set.insert(Role::Button);
        set.insert(Role::Checkbox);
        set.insert(Role::Button);
        assert_eq!(set.len(), 2);
    }
}
