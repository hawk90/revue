//! Box plot render tests extracted from source files
//! Tests only use public APIs

mod box_plot_render_new {
    use revue::layout::Rect;
    use revue::widget::data::chart::boxplot::{group::BoxGroup, render::BoxPlotRender, types::WhiskerStyle};
    use revue::widget::data::chart::chart_common::{Axis, ColorScheme};

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
}

mod value_to_screen {
    use revue::widget::data::chart::boxplot::{group::BoxGroup, render::BoxPlotRender, types::WhiskerStyle};

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
}

mod group_color {
    use revue::style::Color;
    use revue::widget::data::chart::boxplot::{group::BoxGroup, render::BoxPlotRender, types::WhiskerStyle};
    use revue::widget::data::chart::chart_common::ColorScheme;

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
}

mod render_methods {
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::widget::data::chart::boxplot::{group::BoxGroup, render::BoxPlotRender, types::WhiskerStyle};
    use revue::widget::data::chart::chart_common::{Axis, ColorScheme};

    #[test]
    fn test_render_boxes_empty_groups() {
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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

        // Should not panic with empty groups
        render.render_boxes(&mut ctx, &ColorScheme::default_palette());
    }

    #[test]
    fn test_render_boxes_single_group() {
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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

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

    #[test]
    fn test_render_axes_empty_groups() {
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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

        let value_axis = Axis::default();
        let category_axis = Axis::default();

        // Should not panic with empty groups
        render.render_axes(&mut ctx, area, &value_axis, &category_axis);
    }

    #[test]
    fn test_render_axes_with_groups() {
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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

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
        let mut ctx = super::super::super::super::revue::widget::traits::RenderContext::new(&mut buffer, area);

        let value_axis = Axis::default();
        let category_axis = Axis::default();

        // Should not panic with long labels
        render.render_axes(&mut ctx, area, &value_axis, &category_axis);
    }
}

mod public_fields {
    use revue::layout::Rect;
    use revue::style::Color;
    use revue::widget::data::chart::boxplot::{group::BoxGroup, render::BoxPlotRender, types::WhiskerStyle};

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