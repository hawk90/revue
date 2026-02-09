//! Box plot rendering logic

use super::group::BoxGroup;
use super::types::WhiskerStyle;
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

/// Box plot rendering state
pub struct BoxPlotRender<'a> {
    /// Box groups
    pub groups: &'a [BoxGroup],
    /// Value to screen coordinate mapping state
    pub bounds: (f64, f64),
    pub chart_area: Rect,
    pub box_width: f64,
    pub whisker_style: WhiskerStyle,
    pub show_outliers: bool,
    pub group_count: usize,
}

impl<'a> BoxPlotRender<'a> {
    /// Create new render state
    pub fn new(
        groups: &'a [BoxGroup],
        bounds: (f64, f64),
        chart_area: Rect,
        box_width: f64,
        whisker_style: WhiskerStyle,
        show_outliers: bool,
    ) -> Self {
        Self {
            groups,
            bounds,
            chart_area,
            box_width,
            whisker_style,
            show_outliers,
            group_count: groups.len(),
        }
    }

    /// Map value to screen coordinate
    pub fn value_to_screen(&self, value: f64, length: u16) -> u16 {
        let (min, max) = self.bounds;
        let range = (max - min).max(1.0);
        ((value - min) / range * (length as f64 - 1.0)) as u16
    }

    /// Get color for group at index
    pub fn group_color(
        &self,
        index: usize,
        colors: &crate::widget::data::chart::chart_common::ColorScheme,
    ) -> Color {
        self.groups
            .get(index)
            .and_then(|g| g.color)
            .unwrap_or_else(|| colors.get(index))
    }

    /// Render all box plots
    pub fn render_boxes(
        &self,
        ctx: &mut RenderContext,
        colors: &crate::widget::data::chart::chart_common::ColorScheme,
    ) {
        if self.groups.is_empty() {
            return;
        }

        let n_groups = self.group_count;
        let group_width = self.chart_area.width / n_groups as u16;
        let box_width = (group_width as f64 * self.box_width) as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let Some(stats) = group.get_stats(self.whisker_style) else {
                continue;
            };

            let color = self.group_color(i, colors);
            let group_center = self.chart_area.x + (i as u16 * group_width) + group_width / 2;
            let box_left = group_center.saturating_sub(box_width / 2);
            let box_right = box_left + box_width;

            // Calculate y positions (inverted because y increases downward)
            let y_whisker_low = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.whisker_low, self.chart_area.height);
            let y_q1 = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.q1, self.chart_area.height);
            let y_median = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.median, self.chart_area.height);
            let y_q3 = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.q3, self.chart_area.height);
            let y_whisker_high = self.chart_area.y + self.chart_area.height
                - 1
                - self.value_to_screen(stats.whisker_high, self.chart_area.height);

            // Draw whiskers (vertical line in center)
            for y in y_whisker_low.min(y_whisker_high)..=y_whisker_low.max(y_whisker_high) {
                if y >= self.chart_area.y && y < self.chart_area.y + self.chart_area.height {
                    let mut cell = Cell::new('│');
                    cell.fg = Some(color);
                    ctx.buffer.set(group_center, y, cell);
                }
            }

            // Draw whisker caps
            for x in box_left..=box_right {
                if x >= self.chart_area.x && x < self.chart_area.x + self.chart_area.width {
                    // Lower whisker cap
                    if y_whisker_low >= self.chart_area.y
                        && y_whisker_low < self.chart_area.y + self.chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y_whisker_low, cell);
                    }
                    // Upper whisker cap
                    if y_whisker_high >= self.chart_area.y
                        && y_whisker_high < self.chart_area.y + self.chart_area.height
                    {
                        let mut cell = Cell::new('─');
                        cell.fg = Some(color);
                        ctx.buffer.set(x, y_whisker_high, cell);
                    }
                }
            }

            // Draw box (Q1 to Q3)
            for y in y_q3.min(y_q1)..=y_q3.max(y_q1) {
                if y < self.chart_area.y || y >= self.chart_area.y + self.chart_area.height {
                    continue;
                }
                for x in box_left..=box_right {
                    if x < self.chart_area.x || x >= self.chart_area.x + self.chart_area.width {
                        continue;
                    }

                    let ch = if y == y_q1.min(y_q3) {
                        if x == box_left {
                            '┌'
                        } else if x == box_right {
                            '┐'
                        } else {
                            '─'
                        }
                    } else if y == y_q1.max(y_q3) {
                        if x == box_left {
                            '└'
                        } else if x == box_right {
                            '┘'
                        } else {
                            '─'
                        }
                    } else if x == box_left || x == box_right {
                        '│'
                    } else {
                        ' '
                    };

                    let mut cell = Cell::new(ch);
                    cell.fg = Some(color);
                    ctx.buffer.set(x, y, cell);
                }
            }

            // Draw median line
            for x in box_left..=box_right {
                if x >= self.chart_area.x
                    && x < self.chart_area.x + self.chart_area.width
                    && y_median >= self.chart_area.y
                    && y_median < self.chart_area.y + self.chart_area.height
                {
                    let ch = if x == box_left {
                        '├'
                    } else if x == box_right {
                        '┤'
                    } else {
                        '─'
                    };
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(Color::WHITE);
                    ctx.buffer.set(x, y_median, cell);
                }
            }

            // Draw outliers
            if self.show_outliers {
                for &outlier in &stats.outliers {
                    let y = self.chart_area.y + self.chart_area.height
                        - 1
                        - self.value_to_screen(outlier, self.chart_area.height);
                    if y >= self.chart_area.y
                        && y < self.chart_area.y + self.chart_area.height
                        && group_center >= self.chart_area.x
                        && group_center < self.chart_area.x + self.chart_area.width
                    {
                        let mut cell = Cell::new('○');
                        cell.fg = Some(color);
                        ctx.buffer.set(group_center, y, cell);
                    }
                }
            }
        }
    }

    /// Render axis labels
    pub fn render_axes(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        value_axis: &crate::widget::data::chart::chart_common::Axis,
        category_axis: &crate::widget::data::chart::chart_common::Axis,
    ) {
        if self.groups.is_empty() {
            return;
        }

        let (min, max) = self.bounds;

        // Value axis labels (left side)
        let y_label_width = 6u16;
        for i in 0..=4 {
            let value = max - (max - min) * i as f64 / 4.0;
            let label = value_axis.format_value(value);
            let y = area.y + 1 + (i as u16 * (area.height - 3) / 4);

            for (j, ch) in label.chars().take(y_label_width as usize - 1).enumerate() {
                let x = area.x + j as u16;
                if x < area.x + y_label_width && y < area.y + area.height {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(value_axis.color);
                    ctx.buffer.set(x, y, cell);
                }
            }
        }

        // Category axis labels (bottom)
        let n_groups = self.group_count;
        let chart_width = area.width.saturating_sub(y_label_width);
        let group_width = chart_width / n_groups as u16;

        for (i, group) in self.groups.iter().enumerate() {
            let x = area.x + y_label_width + (i as u16 * group_width) + group_width / 2;
            let y = area.y + area.height - 1;
            let label_start = x.saturating_sub(group.label.len() as u16 / 2);

            for (j, ch) in group.label.chars().enumerate() {
                let label_x = label_start + j as u16;
                if label_x >= area.x + y_label_width && label_x < area.x + area.width {
                    let mut cell = Cell::new(ch);
                    cell.fg = Some(category_axis.color);
                    ctx.buffer.set(label_x, y, cell);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::data::chart::chart_common::{Axis, ColorScheme};

    // =========================================================================
    // BoxPlotRender::new tests
    // =========================================================================

    #[test]
    fn test_box_plot_render_new() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let bounds = (0.0, 10.0);
        let chart_area = Rect::new(10, 5, 40, 20);
        let box_width = 0.6;
        let whisker_style = WhiskerStyle::IQR;
        let show_outliers = true;

        let render = BoxPlotRender::new(
            &groups,
            bounds,
            chart_area,
            box_width,
            whisker_style,
            show_outliers,
        );

        assert_eq!(render.groups.len(), groups.len());
        assert_eq!(render.bounds, bounds);
        assert_eq!(render.chart_area, chart_area);
        assert_eq!(render.box_width, box_width);
        assert_eq!(render.whisker_style, whisker_style);
        assert_eq!(render.show_outliers, show_outliers);
        assert_eq!(render.group_count, 1);
    }

    #[test]
    fn test_box_plot_render_new_empty_groups() {
        let groups: Vec<BoxGroup> = vec![];
        let bounds = (0.0, 10.0);
        let chart_area = Rect::new(0, 0, 40, 20);
        let render = BoxPlotRender::new(&groups, bounds, chart_area, 0.6, WhiskerStyle::IQR, true);
        assert_eq!(render.group_count, 0);
    }

    #[test]
    fn test_box_plot_render_new_multiple_groups() {
        let groups = vec![
            BoxGroup::new("A", &[1.0, 2.0, 3.0]),
            BoxGroup::new("B", &[4.0, 5.0, 6.0]),
            BoxGroup::new("C", &[7.0, 8.0, 9.0]),
        ];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );
        assert_eq!(render.group_count, 3);
    }

    // =========================================================================
    // BoxPlotRender::value_to_screen tests
    // =========================================================================

    #[test]
    fn test_value_to_screen_min_value() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 100.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let result = render.value_to_screen(0.0, 100);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_value_to_screen_max_value() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 100.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let result = render.value_to_screen(100.0, 100);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_value_to_screen_mid_value() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 100.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let result = render.value_to_screen(50.0, 100);
        assert_eq!(result, 49);
    }

    #[test]
    fn test_value_to_screen_zero_range() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (50.0, 50.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let result = render.value_to_screen(50.0, 100);
        // When range is 0, max(1.0) is used, so result is 0
        assert_eq!(result, 0);
    }

    #[test]
    fn test_value_to_screen_negative_bounds() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (-100.0, 100.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let result = render.value_to_screen(0.0, 200);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_value_to_screen_custom_length() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let result = render.value_to_screen(5.0, 20);
        assert_eq!(result, 9);
    }

    // =========================================================================
    // BoxPlotRender::group_color tests
    // =========================================================================

    #[test]
    fn test_group_color_with_custom_color() {
        let mut group = BoxGroup::new("A", &[1.0, 2.0, 3.0]);
        group = group.color(Color::RED);

        let groups = vec![group];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );
        let colors = ColorScheme::default_palette();

        let result = render.group_color(0, &colors);
        assert_eq!(result, Color::RED);
    }

    #[test]
    fn test_group_color_from_scheme() {
        let group = BoxGroup::new("A", &[1.0, 2.0, 3.0]);

        let groups = vec![group];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );
        let colors = ColorScheme::default_palette();

        let result = render.group_color(0, &colors);
        // Should get the first color from the scheme
        assert!(result.r < 255 || result.g < 255 || result.b < 255);
    }

    #[test]
    fn test_group_color_multiple_groups() {
        let groups = vec![
            BoxGroup::new("A", &[1.0, 2.0, 3.0]),
            BoxGroup::new("B", &[4.0, 5.0, 6.0]),
        ];

        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );
        let colors = ColorScheme::default_palette();

        let color0 = render.group_color(0, &colors);
        let color1 = render.group_color(1, &colors);
        // Different indices should get different colors from scheme
        assert!((color0.r != color1.r) || (color0.g != color1.g) || (color0.b != color1.b));
    }

    #[test]
    fn test_group_color_out_of_bounds() {
        let group = BoxGroup::new("A", &[1.0, 2.0, 3.0]);

        let groups = vec![group];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );
        let colors = ColorScheme::default_palette();

        // Index out of bounds should still return a color from the scheme
        let result = render.group_color(10, &colors);
        assert!(result.r < 255 || result.g < 255 || result.b < 255);
    }

    // =========================================================================
    // BoxPlotRender::render_boxes tests
    // =========================================================================

    #[test]
    fn test_render_boxes_empty_groups() {
        use crate::render::Buffer;
        let groups: Vec<BoxGroup> = vec![];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with empty groups
        render.render_boxes(&mut ctx, &ColorScheme::default_palette());
    }

    #[test]
    fn test_render_boxes_single_group() {
        use crate::render::Buffer;
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0, 4.0, 5.0])];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::MinMax,
            false,
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render.render_boxes(&mut ctx, &ColorScheme::default_palette());

        // Should render box characters
        let mut has_box = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '┌'
                        || cell.symbol == '┐'
                        || cell.symbol == '└'
                        || cell.symbol == '┘'
                        || cell.symbol == '│'
                        || cell.symbol == '─'
                    {
                        has_box = true;
                        break;
                    }
                }
            }
        }
        assert!(has_box);
    }

    #[test]
    fn test_render_boxes_with_outliers() {
        use crate::render::Buffer;
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        data.push(100.0); // outlier
        let groups = vec![BoxGroup::new("A", &data)];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 100.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render.render_boxes(&mut ctx, &ColorScheme::default_palette());

        // Should render outlier circle
        let mut has_outlier = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '○' {
                        has_outlier = true;
                        break;
                    }
                }
            }
        }
        assert!(has_outlier);
    }

    #[test]
    fn test_render_boxes_no_outliers_when_disabled() {
        use crate::render::Buffer;
        let mut data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        data.push(100.0); // outlier
        let groups = vec![BoxGroup::new("A", &data)];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 100.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            false, // show_outliers = false
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        render.render_boxes(&mut ctx, &ColorScheme::default_palette());

        // Should not render outlier circles
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    assert_ne!(cell.symbol, '○');
                }
            }
        }
    }

    // =========================================================================
    // BoxPlotRender::render_axes tests
    // =========================================================================

    #[test]
    fn test_render_axes_empty_groups() {
        use crate::render::Buffer;
        let groups: Vec<BoxGroup> = vec![];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let value_axis = Axis::default();
        let category_axis = Axis::default();

        // Should not panic with empty groups
        render.render_axes(&mut ctx, area, &value_axis, &category_axis);
    }

    #[test]
    fn test_render_axes_with_groups() {
        use crate::render::Buffer;
        let groups = vec![
            BoxGroup::new("A", &[1.0, 2.0, 3.0]),
            BoxGroup::new("B", &[4.0, 5.0, 6.0]),
        ];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let value_axis = Axis::default();
        let category_axis = Axis::default();

        render.render_axes(&mut ctx, area, &value_axis, &category_axis);

        // Should render without panic
        // Verify some content was written
        let mut has_content = false;
        for y in 0..20 {
            for x in 0..40 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol != ' ' {
                        has_content = true;
                        break;
                    }
                }
            }
        }
        assert!(has_content);
    }

    #[test]
    fn test_render_axes_long_labels() {
        use crate::render::Buffer;
        let groups = vec![BoxGroup::new("VeryLongGroupName", &[1.0, 2.0, 3.0])];
        let render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        let mut buffer = Buffer::new(40, 20);
        let area = Rect::new(0, 0, 40, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let value_axis = Axis::default();
        let category_axis = Axis::default();

        // Should not panic with long labels
        render.render_axes(&mut ctx, area, &value_axis, &category_axis);
    }

    // =========================================================================
    // Public field tests
    // =========================================================================

    #[test]
    fn test_box_plot_render_public_fields() {
        let groups = vec![BoxGroup::new("A", &[1.0, 2.0, 3.0])];
        let mut render = BoxPlotRender::new(
            &groups,
            (0.0, 10.0),
            Rect::new(0, 0, 40, 20),
            0.6,
            WhiskerStyle::IQR,
            true,
        );

        // All fields are public and can be modified
        render.bounds = (5.0, 15.0);
        render.box_width = 0.8;
        render.whisker_style = WhiskerStyle::MinMax;
        render.show_outliers = false;

        assert_eq!(render.bounds, (5.0, 15.0));
        assert_eq!(render.box_width, 0.8);
        assert_eq!(render.whisker_style, WhiskerStyle::MinMax);
        assert!(!render.show_outliers);
    }
}
