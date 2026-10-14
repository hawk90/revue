//! Drawing a lone FormField: its name, placeholder and helper text

use super::render::put_text;
use super::{FormFieldWidget, InputType};
use crate::patterns::form::FormState;
use crate::render::Modifier;
use crate::style::Color;
use crate::widget::theme::{SECONDARY_TEXT, SUBTLE_GRAY};
use crate::widget::traits::{RenderContext, View};

#[allow(dead_code)]
impl FormFieldWidget {
    /// Render the field label at the current area position
    fn render_label(&self, form_state: &FormState, ctx: &mut RenderContext) {
        let area = ctx.area;
        if let Some(field) = form_state.get(&self.name) {
            let label = &field.label;

            put_text(
                ctx,
                0,
                0,
                label,
                area.width,
                SECONDARY_TEXT,
                Modifier::empty(),
            );
        }
    }

    /// Render the field value at the current area position
    fn render_value(&self, form_state: &FormState, ctx: &mut RenderContext) {
        let area = ctx.area;
        let value = form_state.value(&self.name).unwrap_or_default();

        // Display value or placeholder
        let display_text = if value.is_empty() {
            self.placeholder.clone()
        } else {
            match self.input_type {
                InputType::Password => "•".repeat(value.chars().count().min(20)),
                _ => value.clone(),
            }
        };

        let text_color = if value.is_empty() {
            SUBTLE_GRAY // Gray for placeholder
        } else {
            Color::WHITE
        };

        put_text(
            ctx,
            0,
            0,
            &display_text,
            area.width,
            text_color,
            Modifier::empty(),
        );
    }

    /// Render helper text below the field (gray, dim)
    fn render_helper_text(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if self.helper_text.is_empty() {
            return;
        }

        let helper_color = Color::rgb(140, 140, 140);

        put_text(
            ctx,
            0,
            0,
            &self.helper_text,
            area.width,
            helper_color,
            Modifier::DIM,
        );
    }

    /// Render validation errors at the current area position
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

        put_text(
            ctx,
            0,
            0,
            &error_msg,
            area.width,
            error_color,
            Modifier::DIM,
        );
    }
}

impl View for FormFieldWidget {
    crate::impl_view_meta!("FormField");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;

        // Row 0: Label (field name). The label takes `color`, as a
        // `Checkbox`'s does; the placeholder and helper text keep their grays,
        // which say "nothing entered" and "aside".
        let label_fg = ctx.css_color(SECONDARY_TEXT);
        if self.show_label && area.height >= 1 && area.width > 0 {
            put_text(
                ctx,
                0,
                0,
                &self.name,
                area.width,
                label_fg,
                Modifier::empty(),
            );
        }

        // Row 1: Value/placeholder
        if area.height >= 2 {
            let display_text = if self.placeholder.is_empty() {
                &self.name
            } else {
                &self.placeholder
            };

            let text_color = SUBTLE_GRAY;
            put_text(
                ctx,
                0,
                1,
                display_text,
                area.width,
                text_color,
                Modifier::empty(),
            );
        }

        // Row 2: Helper text (gray, dim)
        if area.height >= 3 && !self.helper_text.is_empty() {
            let helper_color = Color::rgb(140, 140, 140);
            put_text(
                ctx,
                0,
                2,
                &self.helper_text,
                area.width,
                helper_color,
                Modifier::DIM,
            );
        }
    }
}
