//! CSS style merging
//!
//! Implements the StyleMerge trait for combining styles.

use crate::style::{
    AlignItems, BorderStyle, Color, Display, FlexDirection, JustifyContent, Size, Spacing, Style,
};

/// Trait for styles that can be merged
pub trait StyleMerge {
    /// Merge another style into this one (other values override)
    fn merge(&self, other: &Self) -> Self;
}

impl StyleMerge for Style {
    fn merge(&self, other: &Self) -> Self {
        let mut result = self.clone();

        // Merge layout (display)
        if other.layout.display != Display::default() {
            result.layout.display = other.layout.display;
        }

        // Merge flex properties
        if other.layout.flex_direction != FlexDirection::default() {
            result.layout.flex_direction = other.layout.flex_direction;
        }
        if other.layout.justify_content != JustifyContent::default() {
            result.layout.justify_content = other.layout.justify_content;
        }
        if other.layout.align_items != AlignItems::default() {
            result.layout.align_items = other.layout.align_items;
        }
        if other.layout.gap != 0 {
            result.layout.gap = other.layout.gap;
        }

        // Merge size
        if other.sizing.width != Size::default() {
            result.sizing.width = other.sizing.width;
        }
        if other.sizing.height != Size::default() {
            result.sizing.height = other.sizing.height;
        }
        if other.sizing.min_width != Size::default() {
            result.sizing.min_width = other.sizing.min_width;
        }
        if other.sizing.min_height != Size::default() {
            result.sizing.min_height = other.sizing.min_height;
        }
        if other.sizing.max_width != Size::default() {
            result.sizing.max_width = other.sizing.max_width;
        }
        if other.sizing.max_height != Size::default() {
            result.sizing.max_height = other.sizing.max_height;
        }

        // Merge spacing
        if other.spacing.margin != Spacing::default() {
            result.spacing.margin = other.spacing.margin;
        }
        if other.spacing.padding != Spacing::default() {
            result.spacing.padding = other.spacing.padding;
        }

        // Merge colors (non-black means it was set)
        if other.visual.color != Color::default() {
            result.visual.color = other.visual.color;
        }
        if other.visual.background != Color::default() {
            result.visual.background = other.visual.background;
        }

        // Merge border
        if other.visual.border_style != BorderStyle::default() {
            result.visual.border_style = other.visual.border_style;
        }
        if other.visual.border_color != Color::default() {
            result.visual.border_color = other.visual.border_color;
        }

        // Merge visual
        if other.visual.opacity != 1.0 {
            result.visual.opacity = other.visual.opacity;
        }
        if !other.visual.visible {
            result.visual.visible = other.visual.visible;
        }

        result
    }
}
