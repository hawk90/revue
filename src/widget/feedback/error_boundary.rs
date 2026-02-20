//! Error boundary widget for catching panics in child rendering
//!
//! Unlike crash propagation, ErrorBoundary catches panics during `render()` and
//! displays a fallback UI instead of crashing the application.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::widget::{ErrorBoundary, error_boundary};
//!
//! // Basic error boundary
//! ErrorBoundary::new()
//!     .child(potentially_failing_widget());
//!
//! // With custom fallback
//! error_boundary()
//!     .child(potentially_failing_widget())
//!     .fallback(text("Something went wrong").fg(Color::RED));
//! ```

use std::cell::RefCell;
use std::panic;

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Error boundary that catches panics during child rendering
///
/// Wraps a child widget and catches panics during `render()`,
/// displaying a fallback UI instead of crashing the application.
///
/// # Example
///
/// ```rust,ignore
/// use revue::prelude::*;
///
/// let safe_ui = ErrorBoundary::new()
///     .child(potentially_failing_widget())
///     .fallback(text("Something went wrong").fg(Color::RED));
/// ```
pub struct ErrorBoundary {
    child: Option<Box<dyn View>>,
    fallback: Option<Box<dyn View>>,
    has_error: std::cell::Cell<bool>,
    error_message: RefCell<Option<String>>,
    props: WidgetProps,
}

impl ErrorBoundary {
    /// Create a new error boundary
    pub fn new() -> Self {
        Self {
            child: None,
            fallback: None,
            has_error: std::cell::Cell::new(false),
            error_message: RefCell::new(None),
            props: WidgetProps::new(),
        }
    }

    /// Set the child widget to protect
    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    /// Set a custom fallback widget to show on error
    pub fn fallback<V: View + 'static>(mut self, fallback: V) -> Self {
        self.fallback = Some(Box::new(fallback));
        self
    }

    /// Reset the error state, allowing the child to render again
    pub fn reset(&self) {
        self.has_error.set(false);
        *self.error_message.borrow_mut() = None;
    }

    /// Check if an error has occurred
    pub fn has_error(&self) -> bool {
        self.has_error.get()
    }

    /// Get the error message, if any
    pub fn error_message(&self) -> Option<String> {
        self.error_message.borrow().clone()
    }

    /// Render the default fallback (red-bordered error box)
    fn render_default_fallback(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 || area.height < 3 {
            return;
        }

        let border_color = Color::rgb(200, 60, 60);
        let text_color = Color::rgb(200, 60, 60);
        let dim_color = Color::rgb(140, 60, 60);

        // Draw top and bottom borders
        for x in area.x..area.x + area.width {
            ctx.buffer.set(x, area.y, Cell::new('─').fg(border_color));
            ctx.buffer
                .set(x, area.y + area.height - 1, Cell::new('─').fg(border_color));
        }
        // Draw left and right borders
        for y in area.y..area.y + area.height {
            ctx.buffer.set(area.x, y, Cell::new('│').fg(border_color));
            ctx.buffer
                .set(area.x + area.width - 1, y, Cell::new('│').fg(border_color));
        }
        // Corners
        ctx.buffer
            .set(area.x, area.y, Cell::new('┌').fg(border_color));
        ctx.buffer.set(
            area.x + area.width - 1,
            area.y,
            Cell::new('┐').fg(border_color),
        );
        ctx.buffer.set(
            area.x,
            area.y + area.height - 1,
            Cell::new('└').fg(border_color),
        );
        ctx.buffer.set(
            area.x + area.width - 1,
            area.y + area.height - 1,
            Cell::new('┘').fg(border_color),
        );

        // Title
        let title = " Error ";
        let title_x = area.x + 2;
        for (i, ch) in title.chars().enumerate() {
            let x = title_x + i as u16;
            if x < area.x + area.width - 1 {
                ctx.buffer.set(x, area.y, Cell::new(ch).fg(text_color));
            }
        }

        // Error message
        let msg = self.error_message.borrow();
        let display_msg = msg.as_deref().unwrap_or("A rendering error occurred");
        let inner_width = (area.width.saturating_sub(4)) as usize;
        let truncated: String = display_msg.chars().take(inner_width).collect();
        let msg_y = area.y + 1;
        for (i, ch) in truncated.chars().enumerate() {
            let x = area.x + 2 + i as u16;
            if x < area.x + area.width - 1 {
                ctx.buffer.set(x, msg_y, Cell::new(ch).fg(dim_color).dim());
            }
        }
    }
}

impl Default for ErrorBoundary {
    fn default() -> Self {
        Self::new()
    }
}

impl View for ErrorBoundary {
    crate::impl_view_meta!("ErrorBoundary");

    fn render(&self, ctx: &mut RenderContext) {
        // If already in error state, show fallback
        if self.has_error.get() {
            if let Some(ref fallback) = self.fallback {
                fallback.render(ctx);
            } else {
                self.render_default_fallback(ctx);
            }
            return;
        }

        // Try rendering child with panic catch
        if let Some(ref child) = self.child {
            let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
                child.render(ctx);
            }));

            if let Err(panic_info) = result {
                self.has_error.set(true);
                let msg = if let Some(s) = panic_info.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = panic_info.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Unknown panic".to_string()
                };
                *self.error_message.borrow_mut() = Some(msg);

                // Render fallback
                if let Some(ref fallback) = self.fallback {
                    fallback.render(ctx);
                } else {
                    self.render_default_fallback(ctx);
                }
            }
        }
    }
}

impl_styled_view!(ErrorBoundary);
impl_props_builders!(ErrorBoundary);

/// Create an error boundary widget
pub fn error_boundary() -> ErrorBoundary {
    ErrorBoundary::new()
}
