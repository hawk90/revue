//! Display widgets - Visual presentation components
//!
//! This module provides widgets for displaying information with visual emphasis,
//! loading states, status indicators, and formatted text presentation.
//!
//! # Widget Categories
//!
//! ## Text Display
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Text`] | Styled text | `text()` |
//! | [`RichText`] | Markup-styled text | [`rich_text()`], [`markup()`] |
//! | [`BigText`] | Large heading text | [`h1()`], [`h2()`], [`h3()`], [`bigtext()`] |
//!
//! ## Loading & Status
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Spinner`] | Loading indicator | [`spinner()`] |
//! | [`Progress`] | Progress bar | [`progress()`] |
//! | [`Skeleton`] | Loading placeholder | [`skeleton()`] |
//! | [`StatusIndicator`] | Online/offline status | [`online()`], [`offline()`] |
//! | [`Gauge`] | Gauge/meter display | [`gauge()`], [`percentage()`] |
//!
//! ## Labels & Tags
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`Badge`] | Status badge | [`badge()`], [`dot_badge()`] |
//! | [`Tag`] | Label tag/chip | [`tag()`], [`chip()`] |
//! | [`Avatar`] | User avatar | [`avatar()`], [`avatar_icon()`] |
//!
//! ## Special Display
//!
//! | Widget | Description | Constructor |
//! |--------|-------------|-------------|
//! | [`EmptyState`] | No-data placeholder | [`empty_state()`], [`no_results()`] |
//! | [`Divider`] | Horizontal/vertical divider | [`divider()`], [`vdivider()`] |
//! | [`GradientBox`] | Animated gradient background | [`gradient_box()`] |
//! | [`RichLog`] | Rich log viewer | [`richlog()`] |
//! | [`Digits`] | Digital clock/timer display | [`digits()`], [`clock()`], [`timer()`] |
//!
//! # Quick Start
//!
//! ## Text Display
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! text("Hello, World!")
//!     .bold()
//!     .fg(Color::Cyan);
//!
//! rich_text()
//!     .content("This is **bold** and this is *italic*")
//!     .fg(Color::White);
//! ```
//!
//! ## Progress Bar
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! progress()
//!     .value(50)
//!//!     .max(100)
//!     .width(40)
//!     .style(ProgressStyle::Determinate);
//! ```
//!
//! ## Spinner
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! spinner()
//!     .style(SpinnerStyle::Dots)
//!     .text("Loading...");
//! ```
//!
//! ## Badge & Tag
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! badge("New")
//!     .variant(BadgeVariant::Filled);
//!
//! tag("Rust")
//!     .style(TagStyle::Primary);
//! ```
//!
//! ## Empty State
//!
//! ```rust,ignore
//! use revue::prelude::*;
//!
//! empty_state()
//!     .icon("📂")
//!     .title("No files found")
//!     .description("Try adjusting your search");
//! ```

pub mod avatar;
pub mod badge;
pub mod bigtext;
pub mod digits;
pub mod divider;
pub mod empty_state;
pub mod gauge;
pub mod gradient_box;
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
pub use gradient_box::{gradient_box, GradientBox};
pub use progress::{progress, Progress, ProgressStyle};
pub use richlog::{log_entry, richlog, LogEntry, LogFormat, LogLevel, RichLog};
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
