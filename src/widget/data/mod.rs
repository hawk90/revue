//! Data widgets - Data display components
//!
//! Widgets for displaying and interacting with structured data.

pub mod csv_viewer;
pub mod datagrid;
pub mod filetree;
pub mod json_viewer;
pub mod list;
pub mod log_viewer;
pub mod table;
pub mod timeline;
pub mod tree;
pub mod virtuallist;

// Re-exports for convenience
pub use csv_viewer::{csv_viewer, CsvViewer, Delimiter, SortOrder as CsvSortOrder};
pub use datagrid::{datagrid, grid_column, grid_row, DataGrid, GridColumn, GridRow, SortDirection};
pub use filetree::{dir_entry, file_entry, file_tree, FileEntry, FileTree, FileType};
pub use json_viewer::{json_viewer, JsonNode, JsonType, JsonViewer, Search};
pub use list::{list, List};
pub use log_viewer::{
    log_entry as adv_log_entry, log_filter, log_parser, log_viewer, LogEntry as AdvLogEntry,
    LogFilter, LogLevel as AdvLogLevel, LogParser, LogViewer, SearchMatch, TimestampFormat,
};
pub use table::{column, table, Column, Table};
pub use timeline::{
    timeline, timeline_event, EventType, Timeline, TimelineEvent, TimelineOrientation,
    TimelineStyle,
};
pub use tree::{tree, tree_node, Tree, TreeNode};
pub use virtuallist::{virtual_list, ScrollAlignment, ScrollMode, VirtualList};
