//! Data widgets tab - Table, Tree, List, Calendar, Timeline, Viewer

mod calendar;
mod list;
mod table;
mod timeline;
mod tree;
mod viewer;

pub use calendar::render as render_calendar;
pub use list::render as render_list;
pub use table::render as render_table;
pub use timeline::render as render_timeline;
pub use tree::render as render_tree;
pub use viewer::render as render_viewer;
