//! CSS style merging
//!
//! Implements the StyleMerge trait for combining styles.

use crate::style::{LayoutStyle, SizingStyle, SpacingStyle, Style, VisualStyle};

/// Trait for styles that can be merged
pub trait StyleMerge {
    /// Merge another style into this one (other values override)
    fn merge(&self, other: &Self) -> Self;
}

impl StyleMerge for Style {
    /// Every field of `other` that differs from its default replaces the
    /// field in `self`. A field equal to its default reads as "not set", so
    /// it cannot override (#1139, fixed in 4.0).
    fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();
        merge_layout(&mut result.layout, &other.layout);
        merge_spacing(&mut result.spacing, &other.spacing);
        merge_sizing(&mut result.sizing, &other.sizing);
        merge_visual(&mut result.visual, &other.visual);
        result
    }
}

/// Copy `value` into `slot` unless it is the default.
fn set<T: PartialEq + Clone>(slot: &mut T, value: &T, default: &T) {
    if value != default {
        *slot = value.clone();
    }
}

// Each group is destructured without `..`, so a field added to it does not
// compile until it is merged here too.

fn merge_layout(result: &mut LayoutStyle, other: &LayoutStyle) {
    let d = LayoutStyle::default();
    let LayoutStyle {
        display,
        position,
        flex_direction,
        justify_content,
        align_items,
        flex_grow,
        flex_wrap,
        align_self,
        order,
        gap,
        column_gap,
        row_gap,
        grid_template_columns,
        grid_template_rows,
        grid_column,
        grid_row,
    } = other;
    set(&mut result.display, display, &d.display);
    set(&mut result.position, position, &d.position);
    set(
        &mut result.flex_direction,
        flex_direction,
        &d.flex_direction,
    );
    set(
        &mut result.justify_content,
        justify_content,
        &d.justify_content,
    );
    set(&mut result.align_items, align_items, &d.align_items);
    set(&mut result.flex_grow, flex_grow, &d.flex_grow);
    set(&mut result.flex_wrap, flex_wrap, &d.flex_wrap);
    set(&mut result.align_self, align_self, &d.align_self);
    set(&mut result.order, order, &d.order);
    set(&mut result.gap, gap, &d.gap);
    set(&mut result.column_gap, column_gap, &d.column_gap);
    set(&mut result.row_gap, row_gap, &d.row_gap);
    set(
        &mut result.grid_template_columns,
        grid_template_columns,
        &d.grid_template_columns,
    );
    set(
        &mut result.grid_template_rows,
        grid_template_rows,
        &d.grid_template_rows,
    );
    set(&mut result.grid_column, grid_column, &d.grid_column);
    set(&mut result.grid_row, grid_row, &d.grid_row);
}

fn merge_spacing(result: &mut SpacingStyle, other: &SpacingStyle) {
    let d = SpacingStyle::default();
    let SpacingStyle {
        padding,
        margin,
        top,
        right,
        bottom,
        left,
    } = other;
    set(&mut result.padding, padding, &d.padding);
    set(&mut result.margin, margin, &d.margin);
    set(&mut result.top, top, &d.top);
    set(&mut result.right, right, &d.right);
    set(&mut result.bottom, bottom, &d.bottom);
    set(&mut result.left, left, &d.left);
}

fn merge_sizing(result: &mut SizingStyle, other: &SizingStyle) {
    let d = SizingStyle::default();
    let SizingStyle {
        width,
        height,
        min_width,
        max_width,
        min_height,
        max_height,
    } = other;
    set(&mut result.width, width, &d.width);
    set(&mut result.height, height, &d.height);
    set(&mut result.min_width, min_width, &d.min_width);
    set(&mut result.max_width, max_width, &d.max_width);
    set(&mut result.min_height, min_height, &d.min_height);
    set(&mut result.max_height, max_height, &d.max_height);
}

fn merge_visual(result: &mut VisualStyle, other: &VisualStyle) {
    let d = VisualStyle::default();
    let VisualStyle {
        border_style,
        border_color,
        color,
        background,
        opacity,
        visible,
        z_index,
        text_align,
        font_weight,
        text_decoration,
        overflow,
    } = other;
    set(&mut result.border_style, border_style, &d.border_style);
    set(&mut result.border_color, border_color, &d.border_color);
    set(&mut result.color, color, &d.color);
    set(&mut result.background, background, &d.background);
    set(&mut result.opacity, opacity, &d.opacity);
    set(&mut result.visible, visible, &d.visible);
    set(&mut result.z_index, z_index, &d.z_index);
    set(&mut result.text_align, text_align, &d.text_align);
    set(&mut result.font_weight, font_weight, &d.font_weight);
    set(
        &mut result.text_decoration,
        text_decoration,
        &d.text_decoration,
    );
    set(&mut result.overflow, overflow, &d.overflow);
}
