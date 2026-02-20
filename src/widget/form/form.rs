//! Form and FormField widgets for automated form rendering
//!
//! These widgets provide automatic two-way binding with FormState,
//! reducing form creation boilerplate by 50-70%.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::prelude::*;
//! use revue::patterns::form::FormState;
//! use revue::widget::{Form, FormField};
//!
//! let form_state = FormState::new()
//!     .field("email", |f| f.label("Email").required().email())
//!     .field("password", |f| f.label("Password").required().min_length(8))
//!     .build();
//!
//! Form::new(form_state.clone())
//!     .on_submit(|data| {
//!         println!("Form submitted: {:?}", data);
//!     })
//!     .child(FormField::new("email").placeholder("Enter email"))
//!     .child(FormField::new("password").input_type(InputType::Password))
//!     .child(Button::new("Submit").submit());
//! ```

use crate::impl_props_builders;
use crate::patterns::form::FormState;
use crate::render::{Cell, Modifier};
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use std::collections::HashMap;
use std::sync::Arc;

/// Type alias for form submit callback
type SubmitCallback = Arc<dyn Fn(HashMap<String, String>)>;

/// Input type for FormField
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum InputType {
    /// Standard text input
    #[default]
    Text,
    /// Password input (masked)
    Password,
    /// Email input
    Email,
    /// Number input
    Number,
}

/// Form widget for automated form rendering with FormState binding
pub struct Form {
    /// Form state for two-way binding
    form_state: FormState,
    /// Submit callback
    on_submit: Option<SubmitCallback>,
    /// Form widget properties
    props: WidgetProps,
    /// Custom submit button text (None = default)
    submit_text: Option<String>,
    /// Whether to show validation errors inline
    show_errors: bool,
    /// Error display style
    error_style: ErrorDisplayStyle,
}

/// How to display validation errors
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ErrorDisplayStyle {
    /// Show errors below each field
    #[default]
    Inline,
    /// Show errors at the bottom of the form
    Summary,
    /// Show both inline and summary
    Both,
}

impl Form {
    /// Create a new Form with FormState binding
    pub fn new(form_state: FormState) -> Self {
        Self {
            form_state,
            on_submit: None,
            props: WidgetProps::default(),
            submit_text: None,
            show_errors: true,
            error_style: ErrorDisplayStyle::default(),
        }
    }

    /// Set submit callback
    pub fn on_submit(mut self, callback: SubmitCallback) -> Self {
        self.on_submit = Some(callback);
        self
    }

    /// Set custom submit button text
    pub fn submit_text(mut self, text: impl Into<String>) -> Self {
        self.submit_text = Some(text.into());
        self
    }

    /// Set whether to show validation errors inline
    pub fn show_errors(mut self, show: bool) -> Self {
        self.show_errors = show;
        self
    }

    /// Set error display style
    pub fn error_style(mut self, style: ErrorDisplayStyle) -> Self {
        self.error_style = style;
        self
    }

    /// Get the form state
    pub fn form_state(&self) -> &FormState {
        &self.form_state
    }

    /// Check if form is valid
    pub fn is_valid(&self) -> bool {
        self.form_state.is_valid()
    }

    /// Get the number of errors in the form
    pub fn error_count(&self) -> usize {
        self.form_state.errors().len()
    }

    /// Get submit button text
    pub fn get_submit_text(&self) -> Option<&String> {
        self.submit_text.as_ref()
    }

    /// Get show_errors flag
    pub fn get_show_errors(&self) -> bool {
        self.show_errors
    }

    /// Get error display style
    pub fn get_error_style(&self) -> ErrorDisplayStyle {
        self.error_style
    }

    /// Submit the form (triggers callback if valid)
    pub fn submit(&self) {
        if self.is_valid() {
            if let Some(ref callback) = self.on_submit {
                let data = self.form_state.values();
                callback(data);
            }
        }
    }

    /// Render form border
    fn render_border(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 2 || area.height < 2 {
            return;
        }

        let border_color = if self.is_valid() {
            Color::rgb(100, 100, 100)
        } else {
            Color::rgb(200, 80, 80) // Red for invalid
        };

        // Draw horizontal borders
        for x in area.x..area.x + area.width {
            let mut top_cell = Cell::new('─');
            top_cell.fg = Some(border_color);
            ctx.buffer.set(x, area.y, top_cell);

            let mut bottom_cell = Cell::new('─');
            bottom_cell.fg = Some(border_color);
            ctx.buffer.set(x, area.y + area.height - 1, bottom_cell);
        }

        // Draw vertical borders
        for y in area.y..area.y + area.height {
            let mut left_cell = Cell::new('│');
            left_cell.fg = Some(border_color);
            ctx.buffer.set(area.x, y, left_cell);

            let mut right_cell = Cell::new('│');
            right_cell.fg = Some(border_color);
            ctx.buffer.set(area.x + area.width - 1, y, right_cell);
        }

        // Draw corners
        let corners = [
            ('┌', area.x, area.y),
            ('┐', area.x + area.width - 1, area.y),
            ('└', area.x, area.y + area.height - 1),
            ('┘', area.x + area.width - 1, area.y + area.height - 1),
        ];

        for &(ch, x, y) in &corners {
            let mut cell = Cell::new(ch);
            cell.fg = Some(border_color);
            ctx.buffer.set(x, y, cell);
        }
    }

    /// Render form title
    fn render_title(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width < 4 {
            return;
        }

        let title = "Form";
        let title_x = area.x + 2;

        for (i, ch) in title.chars().enumerate() {
            if title_x + (i as u16) < area.x + area.width - 1 {
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::WHITE);
                cell.bg = Some(Color::BLACK);
                ctx.buffer.set(title_x + i as u16, area.y, cell);
            }
        }
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::new(FormState::new().build())
    }
}

impl View for Form {
    crate::impl_view_meta!("Form");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Render border
        self.render_border(ctx);

        // Render title
        self.render_title(ctx);

        // Render form fields inside the border
        // Each field takes 3 rows: label (row 0), value (row 1), helper/error (row 2)
        // Plus 1 row gap between fields
        let content_x = area.x + 2;
        if area.width <= 4 {
            return;
        }
        let content_width = area.width - 4;
        let max_x = content_x + content_width;
        let mut current_y = area.y + 1;
        if area.height <= 2 {
            return;
        }
        let max_y = area.y + area.height - 2;
        let show_inline = self.error_style == ErrorDisplayStyle::Inline
            || self.error_style == ErrorDisplayStyle::Both;

        for (_name, field) in self.form_state.iter() {
            if current_y >= max_y {
                break;
            }

            // Row 0: Label
            let label = &field.label;
            if !label.is_empty() {
                for (i, ch) in label.chars().enumerate() {
                    let x = content_x + i as u16;
                    if x < max_x {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(200, 200, 200));
                        ctx.buffer.set(x, current_y, cell);
                    }
                }
            }
            current_y += 1;
            if current_y >= max_y {
                break;
            }

            // Row 1: Value or placeholder
            let value = field.value();
            let (display_text, text_color) = if value.is_empty() {
                (field.placeholder.clone(), Color::rgb(120, 120, 120))
            } else {
                (value, Color::WHITE)
            };

            for (i, ch) in display_text.chars().enumerate() {
                let x = content_x + i as u16;
                if x < max_x {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(text_color);
                    ctx.buffer.set(x, current_y, cell);
                }
            }
            current_y += 1;
            if current_y >= max_y {
                break;
            }

            // Row 2: Error text (red, dim) if touched+errors, otherwise helper text (gray, dim)
            if show_inline && self.show_errors {
                let show_error = field.is_touched() && field.has_errors();
                if show_error {
                    if let Some(error_msg) = field.first_error() {
                        let error_color = Color::rgb(200, 80, 80);
                        for (i, ch) in error_msg.chars().enumerate() {
                            let x = content_x + i as u16;
                            if x < max_x {
                                let mut cell = Cell::new(ch);
                                cell.fg = Some(error_color);
                                cell.modifier |= Modifier::DIM;
                                ctx.buffer.set(x, current_y, cell);
                            }
                        }
                    }
                } else {
                    let helper = field.helper_text();
                    if !helper.is_empty() {
                        let helper_color = Color::rgb(140, 140, 140);
                        for (i, ch) in helper.chars().enumerate() {
                            let x = content_x + i as u16;
                            if x < max_x {
                                let mut cell = Cell::new(ch);
                                cell.fg = Some(helper_color);
                                cell.modifier |= Modifier::DIM;
                                ctx.buffer.set(x, current_y, cell);
                            }
                        }
                    }
                }
            }
            current_y += 1;

            // 1 row gap between fields
            current_y += 1;
        }

        // Render validation summary at bottom if Summary or Both style
        let show_summary = self.error_style == ErrorDisplayStyle::Summary
            || self.error_style == ErrorDisplayStyle::Both;

        let status_y = area.y + area.height - 2;
        if status_y > area.y && area.width > 4 {
            if self.is_valid() {
                let status_text = "Valid";
                let status_color = Color::rgb(80, 200, 80);
                for (i, ch) in status_text.chars().enumerate() {
                    let x = area.x + 2 + i as u16;
                    if x < area.x + area.width - 2 {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(status_color);
                        ctx.buffer.set(x, status_y, cell);
                    }
                }
            } else if show_summary {
                let error_count = self.error_count();
                let status_text = format!("{} error(s)", error_count);
                let status_color = Color::rgb(200, 80, 80);
                for (i, ch) in status_text.chars().enumerate() {
                    let x = area.x + 2 + i as u16;
                    if x < area.x + area.width - 2 {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(status_color);
                        ctx.buffer.set(x, status_y, cell);
                    }
                }
            }
        }
    }
}

impl_props_builders!(Form);

/// FormField widget for individual form field rendering
pub struct FormFieldWidget {
    /// Field name (key in FormState)
    name: String,
    /// Placeholder text
    placeholder: String,
    /// Helper text displayed below the field
    helper_text: String,
    /// Input type
    input_type: InputType,
    /// Widget properties
    props: WidgetProps,
    /// Whether to show label
    show_label: bool,
    /// Whether to show errors inline
    show_errors: bool,
}

impl FormFieldWidget {
    /// Create a new FormField widget
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            placeholder: String::new(),
            helper_text: String::new(),
            input_type: InputType::Text,
            props: WidgetProps::default(),
            show_label: true,
            show_errors: true,
        }
    }

    /// Set placeholder text
    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = text.into();
        self
    }

    /// Set helper text displayed below the field
    pub fn helper_text(mut self, text: impl Into<String>) -> Self {
        self.helper_text = text.into();
        self
    }

    /// Set input type
    pub fn input_type(mut self, input_type: InputType) -> Self {
        self.input_type = input_type;
        self
    }

    /// Set whether to show label
    pub fn show_label(mut self, show: bool) -> Self {
        self.show_label = show;
        self
    }

    /// Set whether to show errors inline
    pub fn show_errors(mut self, show: bool) -> Self {
        self.show_errors = show;
        self
    }

    /// Get the field name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get placeholder
    pub fn get_placeholder(&self) -> Option<&String> {
        if self.placeholder.is_empty() {
            None
        } else {
            Some(&self.placeholder)
        }
    }

    /// Get input type
    pub fn get_input_type(&self) -> InputType {
        self.input_type
    }

    /// Get show_label flag
    pub fn get_show_label(&self) -> bool {
        self.show_label
    }

    /// Get show_errors flag
    pub fn get_show_errors(&self) -> bool {
        self.show_errors
    }

    /// Render the field label at the current area position
    #[allow(dead_code)]
    fn render_label(&self, form_state: &FormState, ctx: &mut RenderContext) {
        let area = ctx.area;
        if let Some(field) = form_state.get(&self.name) {
            let label = &field.label;

            for (i, ch) in label.chars().enumerate() {
                if area.x + (i as u16) < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(200, 200, 200));
                    ctx.buffer.set(area.x + i as u16, area.y, cell);
                }
            }
        }
    }

    /// Render the field value at the current area position
    #[allow(dead_code)]
    fn render_value(&self, form_state: &FormState, ctx: &mut RenderContext) {
        let area = ctx.area;
        let value = form_state.value(&self.name).unwrap_or_default();

        // Display value or placeholder
        let display_text = if value.is_empty() {
            self.placeholder.clone()
        } else {
            match self.input_type {
                InputType::Password => "•".repeat(value.len().min(20)),
                _ => value.clone(),
            }
        };

        let text_color = if value.is_empty() {
            Color::rgb(120, 120, 120) // Gray for placeholder
        } else {
            Color::WHITE
        };

        for (i, ch) in display_text.chars().enumerate() {
            let x = area.x + i as u16;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(text_color);
                ctx.buffer.set(x, area.y, cell);
            }
        }
    }

    /// Render helper text below the field (gray, dim)
    #[allow(dead_code)]
    fn render_helper_text(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if self.helper_text.is_empty() {
            return;
        }

        let helper_color = Color::rgb(140, 140, 140);

        for (i, ch) in self.helper_text.chars().enumerate() {
            let x = area.x + i as u16;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(helper_color);
                cell.modifier |= Modifier::DIM;
                ctx.buffer.set(x, area.y, cell);
            }
        }
    }

    /// Render validation errors at the current area position
    #[allow(dead_code)]
    fn render_errors(&self, form_state: &FormState, ctx: &mut RenderContext) {
        if !self.show_errors {
            return;
        }

        let field = match form_state.get(&self.name) {
            Some(f) => f,
            None => return,
        };

        let error_msg = match field.first_error() {
            Some(err) => err,
            None => return,
        };

        let area = ctx.area;
        let error_color = Color::rgb(200, 80, 80);

        for (i, ch) in error_msg.chars().enumerate() {
            let x = area.x + i as u16;
            if x < area.x + area.width {
                let mut cell = Cell::new(ch);
                cell.fg = Some(error_color);
                cell.modifier |= Modifier::DIM;
                ctx.buffer.set(x, area.y, cell);
            }
        }
    }
}

impl Default for FormFieldWidget {
    fn default() -> Self {
        Self::new("")
    }
}

impl View for FormFieldWidget {
    crate::impl_view_meta!("FormField");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Row 0: Label (field name)
        if self.show_label && area.height >= 1 && area.width > 0 {
            for (i, ch) in self.name.chars().enumerate() {
                let x = area.x + i as u16;
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::rgb(200, 200, 200));
                    ctx.buffer.set(x, area.y, cell);
                }
            }
        }

        // Row 1: Value/placeholder
        if area.height >= 2 {
            let display_text = if self.placeholder.is_empty() {
                &self.name
            } else {
                &self.placeholder
            };

            let text_color = Color::rgb(120, 120, 120);
            for (i, ch) in display_text.chars().enumerate() {
                let x = area.x + i as u16;
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(text_color);
                    ctx.buffer.set(x, area.y + 1, cell);
                }
            }
        }

        // Row 2: Helper text (gray, dim)
        if area.height >= 3 && !self.helper_text.is_empty() {
            let helper_color = Color::rgb(140, 140, 140);
            for (i, ch) in self.helper_text.chars().enumerate() {
                let x = area.x + i as u16;
                if x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(helper_color);
                    cell.modifier |= Modifier::DIM;
                    ctx.buffer.set(x, area.y + 2, cell);
                }
            }
        }
    }
}

impl_props_builders!(FormFieldWidget);

/// Convenience function to create a Form
pub fn form(form_state: FormState) -> Form {
    Form::new(form_state)
}

/// Convenience function to create a FormField
pub fn form_field(name: impl Into<String>) -> FormFieldWidget {
    FormFieldWidget::new(name)
}

// Re-export FormField from patterns module for convenience
pub use crate::patterns::form::FormField;
