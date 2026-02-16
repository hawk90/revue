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
