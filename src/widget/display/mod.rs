//! Display widgets - Visual presentation components
//!
//! Widgets for displaying information to users with visual emphasis and status indication.

pub mod avatar;
pub mod badge;
pub mod bigtext;
pub mod digits;
pub mod divider;
pub mod empty_state;
pub mod gauge;
pub mod progress;
pub mod richlog;
pub mod richtext;
pub mod skeleton;
pub mod spinner;
pub mod status_indicator;
pub mod tag;
pub mod text;

// Re-exports for convenience
// Some functions are exported for public API but may not be used internally
#[allow(unused_imports)]
pub use digits::{clock, digits, timer, DigitStyle, Digits};

pub use avatar::{avatar, avatar_icon, Avatar, AvatarShape, AvatarSize};
pub use badge::{badge, dot_badge, Badge, BadgeShape, BadgeVariant};
pub use bigtext::{bigtext, h1, h2, h3, BigText};
pub use divider::{divider, vdivider, Divider, DividerStyle, Orientation};
pub use empty_state::{
    empty_error, empty_state, first_use, no_results, EmptyState, EmptyStateType, EmptyStateVariant,
};
pub use gauge::{battery, gauge, percentage, Gauge, GaugeStyle, LabelPosition};
pub use progress::{progress, Progress, ProgressStyle};
pub use richlog::{log_entry, richlog, LogEntry, LogLevel, RichLog};
pub use richtext::{markup, rich_text, span, style, RichText, Span, Style};
pub use skeleton::{
    skeleton, skeleton_avatar, skeleton_paragraph, skeleton_text, Skeleton, SkeletonShape,
};
pub use spinner::{spinner, Spinner, SpinnerStyle};
pub use status_indicator::{
    away_indicator, busy_indicator, offline, online, status_indicator, Status, StatusIndicator,
    StatusSize, StatusStyle,
};
pub use tag::{chip, tag, Tag, TagStyle};
pub use text::{Alignment, Text};
