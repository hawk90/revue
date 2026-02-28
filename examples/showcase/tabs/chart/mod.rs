//! Chart widgets tab - Bar, Line, Pie, Spark, Time, Special

mod bar;
mod line;
mod pie;
mod spark;
mod special;
mod time;

pub use bar::render as render_bar;
pub use line::render as render_line;
pub use pie::render as render_pie;
pub use spark::render as render_spark;
pub use special::render as render_special;
pub use time::render as render_time;
