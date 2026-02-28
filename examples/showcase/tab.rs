//! Tab definitions for two-level navigation

/// Main tab categories (Level 1)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MainTab {
    Input,
    Display,
    Chart,
    Data,
    Layout,
    Feedback,
    Developer,
}

impl MainTab {
    pub const ALL: [MainTab; 7] = [
        MainTab::Input,
        MainTab::Display,
        MainTab::Chart,
        MainTab::Data,
        MainTab::Layout,
        MainTab::Feedback,
        MainTab::Developer,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            MainTab::Input => "Input",
            MainTab::Display => "Display",
            MainTab::Chart => "Chart",
            MainTab::Data => "Data",
            MainTab::Layout => "Layout",
            MainTab::Feedback => "Feedback",
            MainTab::Developer => "Developer",
        }
    }

    pub fn key(&self) -> char {
        match self {
            MainTab::Input => '1',
            MainTab::Display => '2',
            MainTab::Chart => '3',
            MainTab::Data => '4',
            MainTab::Layout => '5',
            MainTab::Feedback => '6',
            MainTab::Developer => '7',
        }
    }

    #[allow(dead_code)]
    pub fn from_key(c: char) -> Option<Self> {
        match c {
            '1' => Some(MainTab::Input),
            '2' => Some(MainTab::Display),
            '3' => Some(MainTab::Chart),
            '4' => Some(MainTab::Data),
            '5' => Some(MainTab::Layout),
            '6' => Some(MainTab::Feedback),
            '7' => Some(MainTab::Developer),
            _ => None,
        }
    }

    pub fn sub_tabs(&self) -> &'static [SubTab] {
        match self {
            MainTab::Input => &SubTab::INPUT,
            MainTab::Display => &SubTab::DISPLAY,
            MainTab::Chart => &SubTab::CHART,
            MainTab::Data => &SubTab::DATA,
            MainTab::Layout => &SubTab::LAYOUT,
            MainTab::Feedback => &SubTab::FEEDBACK,
            MainTab::Developer => &SubTab::DEVELOPER,
        }
    }
}

/// Sub-tab categories (Level 2)
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SubTab {
    // Input
    Button,
    InputField,
    Toggle,
    Select,
    Slider,
    Form,
    Picker,
    Autocomplete,
    Number,
    Masked,

    // Display
    Text,
    Status,
    Badge,
    Alert,
    Progress,
    Media,
    Skeleton,
    Typography,

    // Chart
    Bar,
    Line,
    Pie,
    Spark,
    Time,
    Special,

    // Data
    Table,
    Tree,
    List,
    Calendar,
    Timeline,
    Viewer,

    // Layout
    Border,
    Stack,
    Grid,
    Split,
    Container,
    Nav,

    // Feedback
    Modal,
    Toast,
    Menu,
    Tooltip,
    Overlay,

    // Developer
    Code,
    Terminal,
    Http,
    Ai,
    Diff,
    Monitor,
}

impl SubTab {
    // Input sub-tabs
    pub const INPUT: [SubTab; 10] = [
        SubTab::Button,
        SubTab::InputField,
        SubTab::Toggle,
        SubTab::Select,
        SubTab::Slider,
        SubTab::Form,
        SubTab::Picker,
        SubTab::Autocomplete,
        SubTab::Number,
        SubTab::Masked,
    ];

    // Display sub-tabs
    pub const DISPLAY: [SubTab; 8] = [
        SubTab::Text,
        SubTab::Status,
        SubTab::Badge,
        SubTab::Alert,
        SubTab::Progress,
        SubTab::Media,
        SubTab::Skeleton,
        SubTab::Typography,
    ];

    // Chart sub-tabs
    pub const CHART: [SubTab; 6] = [
        SubTab::Bar,
        SubTab::Line,
        SubTab::Pie,
        SubTab::Spark,
        SubTab::Time,
        SubTab::Special,
    ];

    // Data sub-tabs
    pub const DATA: [SubTab; 6] = [
        SubTab::Table,
        SubTab::Tree,
        SubTab::List,
        SubTab::Calendar,
        SubTab::Timeline,
        SubTab::Viewer,
    ];

    // Layout sub-tabs
    pub const LAYOUT: [SubTab; 6] = [
        SubTab::Border,
        SubTab::Stack,
        SubTab::Grid,
        SubTab::Split,
        SubTab::Container,
        SubTab::Nav,
    ];

    // Feedback sub-tabs
    pub const FEEDBACK: [SubTab; 5] = [
        SubTab::Modal,
        SubTab::Toast,
        SubTab::Menu,
        SubTab::Tooltip,
        SubTab::Overlay,
    ];

    // Developer sub-tabs
    pub const DEVELOPER: [SubTab; 6] = [
        SubTab::Code,
        SubTab::Terminal,
        SubTab::Http,
        SubTab::Ai,
        SubTab::Diff,
        SubTab::Monitor,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            SubTab::Button => "Button",
            SubTab::InputField => "Input",
            SubTab::Toggle => "Toggle",
            SubTab::Select => "Select",
            SubTab::Slider => "Slider",
            SubTab::Form => "Form",
            SubTab::Picker => "Picker",
            SubTab::Autocomplete => "Autocomplete",
            SubTab::Number => "Number",
            SubTab::Masked => "Masked",

            SubTab::Text => "Text",
            SubTab::Status => "Status",
            SubTab::Badge => "Badge",
            SubTab::Alert => "Alert",
            SubTab::Progress => "Progress",
            SubTab::Media => "Media",
            SubTab::Skeleton => "Skeleton",
            SubTab::Typography => "Typography",

            SubTab::Bar => "Bar",
            SubTab::Line => "Line",
            SubTab::Pie => "Pie",
            SubTab::Spark => "Spark",
            SubTab::Time => "Time",
            SubTab::Special => "Special",

            SubTab::Table => "Table",
            SubTab::Tree => "Tree",
            SubTab::List => "List",
            SubTab::Calendar => "Calendar",
            SubTab::Timeline => "Timeline",
            SubTab::Viewer => "Viewer",

            SubTab::Border => "Border",
            SubTab::Stack => "Stack",
            SubTab::Grid => "Grid",
            SubTab::Split => "Split",
            SubTab::Container => "Container",
            SubTab::Nav => "Nav",

            SubTab::Modal => "Modal",
            SubTab::Toast => "Toast",
            SubTab::Menu => "Menu",
            SubTab::Tooltip => "Tooltip",
            SubTab::Overlay => "Overlay",

            SubTab::Code => "Code",
            SubTab::Terminal => "Terminal",
            SubTab::Http => "HTTP",
            SubTab::Ai => "AI",
            SubTab::Diff => "Diff",
            SubTab::Monitor => "Monitor",
        }
    }
}
