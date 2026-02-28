//! Display widgets tab - Text, Status, Badge, Alert, Progress, Media, Skeleton, Typography

mod alert;
mod badge;
mod media;
mod progress;
mod skeleton;
mod status;
mod text;
mod typography;

pub use alert::render as render_alerts;
pub use badge::render as render_badges;
pub use media::render as render_media;
pub use progress::render as render_progress;
pub use skeleton::render as render_skeleton;
pub use status::render as render_status;
pub use text::render as render_text;
pub use typography::render as render_typography;
