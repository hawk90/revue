//! Convert Revue styles to taffy styles

use crate::style::{
    Style, Display, FlexDirection, JustifyContent, AlignItems, Size, Spacing,
    Position, GridTemplate, GridTrack, GridPlacement,
};
use taffy::prelude::*;

/// Convert Revue style to taffy style
pub fn to_taffy_style(style: &Style) -> taffy::Style {
    let mut taffy_style = taffy::Style {
        display: convert_display(style.layout.display),
        position: convert_position(style.layout.position),
        flex_direction: convert_flex_direction(style.layout.flex_direction),
        justify_content: Some(convert_justify_content(style.layout.justify_content)),
        align_items: Some(convert_align_items(style.layout.align_items)),
        gap: taffy::Size {
            width: length(style.layout.column_gap.unwrap_or(style.layout.gap) as f32),
            height: length(style.layout.row_gap.unwrap_or(style.layout.gap) as f32),
        },
        padding: convert_spacing(&style.spacing.padding),
        margin: convert_spacing_auto(&style.spacing.margin),
        size: taffy::Size {
            width: convert_size(style.sizing.width),
            height: convert_size(style.sizing.height),
        },
        min_size: taffy::Size {
            width: convert_size(style.sizing.min_width),
            height: convert_size(style.sizing.min_height),
        },
        max_size: taffy::Size {
            width: convert_size(style.sizing.max_width),
            height: convert_size(style.sizing.max_height),
        },
        ..Default::default()
    };

    // Grid properties
    if style.layout.display == Display::Grid {
        taffy_style.grid_template_columns = convert_grid_template(&style.layout.grid_template_columns);
        taffy_style.grid_template_rows = convert_grid_template(&style.layout.grid_template_rows);
    }

    // Grid item placement
    if style.layout.grid_column != GridPlacement::default() {
        taffy_style.grid_column = convert_grid_placement(style.layout.grid_column);
    }
    if style.layout.grid_row != GridPlacement::default() {
        taffy_style.grid_row = convert_grid_placement(style.layout.grid_row);
    }

    // Position offsets
    taffy_style.inset = taffy::Rect {
        top: convert_inset(style.spacing.top),
        right: convert_inset(style.spacing.right),
        bottom: convert_inset(style.spacing.bottom),
        left: convert_inset(style.spacing.left),
    };

    taffy_style
}

fn convert_display(display: Display) -> taffy::Display {
    match display {
        Display::Flex => taffy::Display::Flex,
        Display::Block => taffy::Display::Block,
        Display::Grid => taffy::Display::Grid,
        Display::None => taffy::Display::None,
    }
}

fn convert_position(position: Position) -> taffy::Position {
    match position {
        Position::Static => taffy::Position::Relative,  // taffy doesn't have Static
        Position::Relative => taffy::Position::Relative,
        Position::Absolute => taffy::Position::Absolute,
        Position::Fixed => taffy::Position::Absolute,  // Fixed is like absolute in terminal
    }
}

fn convert_grid_template(template: &GridTemplate) -> Vec<taffy::TrackSizingFunction> {
    use taffy::style_helpers::*;

    template.tracks.iter().map(|track| {
        match track {
            GridTrack::Fixed(v) => taffy::TrackSizingFunction::Single(
                minmax(length(*v as f32), length(*v as f32))
            ),
            GridTrack::Fr(fr_val) => taffy::TrackSizingFunction::Single(
                minmax(auto(), fr(*fr_val))
            ),
            GridTrack::Auto => taffy::TrackSizingFunction::Single(
                minmax(auto(), auto())
            ),
            GridTrack::MinContent => taffy::TrackSizingFunction::Single(
                minmax(min_content(), min_content())
            ),
            GridTrack::MaxContent => taffy::TrackSizingFunction::Single(
                minmax(max_content(), max_content())
            ),
        }
    }).collect()
}

fn convert_grid_placement(placement: GridPlacement) -> taffy::Line<taffy::GridPlacement> {
    use taffy::style_helpers::*;

    if placement.start == 0 && placement.end == 0 {
        // Auto placement
        taffy::Line {
            start: taffy::GridPlacement::Auto,
            end: taffy::GridPlacement::Auto,
        }
    } else if placement.end < 0 {
        // Span
        taffy::Line {
            start: if placement.start == 0 {
                taffy::GridPlacement::Auto
            } else {
                line(placement.start)
            },
            end: span((-placement.end) as u16),
        }
    } else if placement.end == 0 {
        // Single line
        taffy::Line {
            start: line(placement.start),
            end: taffy::GridPlacement::Auto,
        }
    } else {
        // Start to end
        taffy::Line {
            start: line(placement.start),
            end: line(placement.end),
        }
    }
}

fn convert_inset(value: Option<i16>) -> LengthPercentageAuto {
    match value {
        Some(v) => LengthPercentageAuto::Length(v as f32),
        None => LengthPercentageAuto::Auto,
    }
}

fn convert_flex_direction(dir: FlexDirection) -> taffy::FlexDirection {
    match dir {
        FlexDirection::Row => taffy::FlexDirection::Row,
        FlexDirection::Column => taffy::FlexDirection::Column,
    }
}

fn convert_justify_content(jc: JustifyContent) -> taffy::JustifyContent {
    match jc {
        JustifyContent::Start => taffy::JustifyContent::Start,
        JustifyContent::Center => taffy::JustifyContent::Center,
        JustifyContent::End => taffy::JustifyContent::End,
        JustifyContent::SpaceBetween => taffy::JustifyContent::SpaceBetween,
        JustifyContent::SpaceAround => taffy::JustifyContent::SpaceAround,
    }
}

fn convert_align_items(ai: AlignItems) -> taffy::AlignItems {
    match ai {
        AlignItems::Start => taffy::AlignItems::Start,
        AlignItems::Center => taffy::AlignItems::Center,
        AlignItems::End => taffy::AlignItems::End,
        AlignItems::Stretch => taffy::AlignItems::Stretch,
    }
}

fn convert_size(size: Size) -> Dimension {
    match size {
        Size::Auto => Dimension::Auto,
        Size::Fixed(v) => length(v as f32),
        Size::Percent(p) => percent(p / 100.0),
    }
}

fn convert_spacing(spacing: &Spacing) -> taffy::Rect<LengthPercentage> {
    taffy::Rect {
        top: length(spacing.top as f32),
        right: length(spacing.right as f32),
        bottom: length(spacing.bottom as f32),
        left: length(spacing.left as f32),
    }
}

fn convert_spacing_auto(spacing: &Spacing) -> taffy::Rect<LengthPercentageAuto> {
    taffy::Rect {
        top: LengthPercentageAuto::Length(spacing.top as f32),
        right: LengthPercentageAuto::Length(spacing.right as f32),
        bottom: LengthPercentageAuto::Length(spacing.bottom as f32),
        left: LengthPercentageAuto::Length(spacing.left as f32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_display() {
        assert!(matches!(convert_display(Display::Flex), taffy::Display::Flex));
        assert!(matches!(convert_display(Display::Block), taffy::Display::Block));
        assert!(matches!(convert_display(Display::None), taffy::Display::None));
    }

    #[test]
    fn test_convert_flex_direction() {
        assert!(matches!(
            convert_flex_direction(FlexDirection::Row),
            taffy::FlexDirection::Row
        ));
        assert!(matches!(
            convert_flex_direction(FlexDirection::Column),
            taffy::FlexDirection::Column
        ));
    }

    #[test]
    fn test_convert_size() {
        assert!(matches!(convert_size(Size::Auto), Dimension::Auto));

        match convert_size(Size::Fixed(100)) {
            Dimension::Length(v) => assert!((v - 100.0).abs() < f32::EPSILON),
            _ => panic!("Expected Length"),
        }

        match convert_size(Size::Percent(50.0)) {
            Dimension::Percent(v) => assert!((v - 0.5).abs() < f32::EPSILON),
            _ => panic!("Expected Percent"),
        }
    }

    #[test]
    fn test_to_taffy_style() {
        let mut style = Style::default();
        style.layout.display = Display::Flex;
        style.layout.flex_direction = FlexDirection::Column;
        style.layout.justify_content = JustifyContent::Center;
        style.layout.align_items = AlignItems::Stretch;
        style.layout.gap = 4;
        style.spacing.padding = Spacing::all(2);

        let taffy_style = to_taffy_style(&style);

        assert!(matches!(taffy_style.display, taffy::Display::Flex));
        assert!(matches!(taffy_style.flex_direction, taffy::FlexDirection::Column));
    }
}
