//! Notification types

use crate::style::Color;

/// Notification level/severity
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NotificationLevel {
    /// Informational message
    #[default]
    Info,
    /// Success message
    Success,
    /// Warning message
    Warning,
    /// Error message
    Error,
    /// Debug message
    Debug,
}

impl NotificationLevel {
    /// Get icon for level
    pub(super) fn icon(&self) -> char {
        match self {
            NotificationLevel::Info => 'ℹ',
            NotificationLevel::Success => '✓',
            NotificationLevel::Warning => '⚠',
            NotificationLevel::Error => '✗',
            NotificationLevel::Debug => '⚙',
        }
    }

    /// Get color for level
    pub(super) fn color(&self) -> Color {
        match self {
            NotificationLevel::Info => Color::CYAN,
            NotificationLevel::Success => Color::GREEN,
            NotificationLevel::Warning => Color::YELLOW,
            NotificationLevel::Error => Color::RED,
            NotificationLevel::Debug => Color::MAGENTA,
        }
    }

    /// Get background color
    pub(super) fn bg_color(&self) -> Color {
        match self {
            NotificationLevel::Info => Color::rgb(20, 50, 60),
            NotificationLevel::Success => Color::rgb(20, 50, 30),
            NotificationLevel::Warning => Color::rgb(60, 50, 20),
            NotificationLevel::Error => Color::rgb(60, 20, 20),
            NotificationLevel::Debug => Color::rgb(40, 20, 50),
        }
    }
}

/// A single notification
#[derive(Clone, Debug)]
pub struct Notification {
    /// Unique ID
    pub id: u64,
    /// Notification title
    pub title: Option<String>,
    /// Notification message
    pub message: String,
    /// Severity level
    pub level: NotificationLevel,
    /// Duration in ticks (0 = permanent until dismissed)
    pub duration: u32,
    /// Current tick count
    pub tick: u32,
    /// Is dismissible
    pub dismissible: bool,
    /// Progress value (0.0-1.0, for progress notifications)
    pub progress: Option<f64>,
    /// Action text
    pub action: Option<String>,
    /// Created timestamp (tick when created)
    pub created_at: u64,
}

impl Notification {
    /// Create a new notification
    pub fn new(message: impl Into<String>) -> Self {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

        Self {
            id: COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            title: None,
            message: message.into(),
            level: NotificationLevel::Info,
            duration: 100, // Default ~3 seconds at 30fps
            tick: 0,
            dismissible: true,
            progress: None,
            action: None,
            created_at: 0,
        }
    }

    /// Set title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Set level
    pub fn level(mut self, level: NotificationLevel) -> Self {
        self.level = level;
        self
    }

    /// Set duration (ticks, 0 = permanent)
    pub fn duration(mut self, ticks: u32) -> Self {
        self.duration = ticks;
        self
    }

    /// Set dismissible
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Set progress
    pub fn progress(mut self, progress: f64) -> Self {
        self.progress = Some(progress.clamp(0.0, 1.0));
        self
    }

    /// Set action text
    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    /// Create info notification
    pub fn info(message: impl Into<String>) -> Self {
        Self::new(message).level(NotificationLevel::Info)
    }

    /// Create success notification
    pub fn success(message: impl Into<String>) -> Self {
        Self::new(message).level(NotificationLevel::Success)
    }

    /// Create warning notification
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new(message).level(NotificationLevel::Warning)
    }

    /// Create error notification
    pub fn error(message: impl Into<String>) -> Self {
        Self::new(message).level(NotificationLevel::Error)
    }

    /// Create debug notification
    pub fn debug(message: impl Into<String>) -> Self {
        Self::new(message).level(NotificationLevel::Debug)
    }

    /// Check if expired
    pub fn is_expired(&self) -> bool {
        self.duration > 0 && self.tick >= self.duration
    }

    /// Get remaining percentage
    pub fn remaining(&self) -> f64 {
        if self.duration == 0 {
            1.0
        } else {
            1.0 - (self.tick as f64 / self.duration as f64)
        }
    }
}

/// Notification position on screen
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NotificationPosition {
    /// Top right corner
    #[default]
    TopRight,
    /// Top left corner
    TopLeft,
    /// Top center
    TopCenter,
    /// Bottom right corner
    BottomRight,
    /// Bottom left corner
    BottomLeft,
    /// Bottom center
    BottomCenter,
}
