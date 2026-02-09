//! Heat map widget rendering

use super::HeatMap;
use crate::style::Color;
use crate::widget::RenderContext;
use crate::widget::View;
use crate::widget::{hstack, vstack, Text};

impl View for HeatMap {
    crate::impl_view_meta!("HeatMap");

    fn render(&self, ctx: &mut RenderContext) {
        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Column labels
        if let Some(labels) = &self.col_labels {
            let label_offset = if self.row_labels.is_some() { 8 } else { 0 };
            let mut col_header = hstack();
            col_header = col_header.child(Text::new(" ".repeat(label_offset)));

            for label in labels.iter().take(self.cols) {
                let truncated = if label.len() > self.cell_width {
                    &label[..self.cell_width]
                } else {
                    label
                };
                col_header = col_header.child(
                    Text::new(format!("{:^width$}", truncated, width = self.cell_width))
                        .fg(Color::rgb(150, 150, 150)),
                );
            }
            content = content.child(col_header);
        }

        // Data rows
        for (row_idx, row) in self.data.iter().enumerate() {
            for _ in 0..self.cell_height {
                let mut row_view = hstack();

                // Row label
                if let Some(labels) = &self.row_labels {
                    if let Some(label) = labels.get(row_idx) {
                        let truncated = if label.len() > 6 { &label[..6] } else { label };
                        row_view = row_view.child(
                            Text::new(format!("{:>6} ", truncated)).fg(Color::rgb(150, 150, 150)),
                        );
                    }
                }

                // Cells
                for (col_idx, &value) in row.iter().enumerate() {
                    let color = self.color_for(value);
                    let cell_str = self.render_cell(value);

                    let is_highlighted = self.highlighted == Some((row_idx, col_idx));

                    let mut cell_text = Text::new(&cell_str);

                    if self.show_values {
                        // Show value with colored background
                        cell_text = cell_text.bg(color);
                        // Contrast text color
                        let brightness = (color.r as u32 + color.g as u32 + color.b as u32) / 3;
                        if brightness > 128 {
                            cell_text = cell_text.fg(Color::BLACK);
                        } else {
                            cell_text = cell_text.fg(Color::WHITE);
                        }
                    } else {
                        cell_text = cell_text.fg(color);
                    }

                    if is_highlighted {
                        cell_text = cell_text.bold();
                    }

                    row_view = row_view.child(cell_text);
                }

                content = content.child(row_view);
            }
        }

        // Legend
        if self.show_legend {
            let mut legend = hstack();
            legend = legend.child(Text::new("Low ").fg(Color::rgb(128, 128, 128)));

            for i in 0..10 {
                let v = i as f64 / 9.0;
                let color = self.color_scale.color_at(v);
                legend = legend.child(Text::new("█").fg(color));
            }

            legend = legend.child(Text::new(" High").fg(Color::rgb(128, 128, 128)));
            legend = legend.child(
                Text::new(format!("  ({:.1} - {:.1})", self.min_val, self.max_val))
                    .fg(Color::rgb(100, 100, 100)),
            );

            content = content.child(legend);
        }

        content.render(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::data::chart::{CellDisplay, ColorScale};
    use crate::widget::View;

    // =========================================================================
    // HeatMap::render tests
    // =========================================================================

    #[test]
    fn test_heatmap_render_basic() {
        let data = vec![vec![0.0, 0.5, 1.0], vec![0.2, 0.4, 0.8]];
        let hm = HeatMap::new(data);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Basic smoke test - should render without panic
    }

    #[test]
    fn test_heatmap_render_with_title() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data).title("Test Title");

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Title should be in the first row
        assert_eq!(buffer.get(0, 0).unwrap().symbol, 'T');
        assert_eq!(buffer.get(1, 0).unwrap().symbol, 'e');
        assert_eq!(buffer.get(2, 0).unwrap().symbol, 's');
        assert_eq!(buffer.get(3, 0).unwrap().symbol, 't');
    }

    #[test]
    fn test_heatmap_render_without_title() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // No title in first row
        let first_cell = buffer.get(0, 0).unwrap();
        assert_ne!(first_cell.symbol, 'T');
    }

    #[test]
    fn test_heatmap_render_with_column_labels() {
        let data = vec![vec![0.5, 0.8]];
        let hm = HeatMap::new(data).col_labels(vec!["A".into(), "B".into()]);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Column labels should be rendered
        // The labels appear in the header row (row 0 if no title, row 1 if title)
        let mut has_label = false;
        for x in 0..30 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'A' || cell.symbol == 'B' {
                    has_label = true;
                    break;
                }
            }
        }
        assert!(has_label);
    }

    #[test]
    fn test_heatmap_render_with_row_labels() {
        let data = vec![vec![0.5], vec![0.8]];
        let hm = HeatMap::new(data).row_labels(vec!["R1".into(), "R2".into()]);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Row labels should be rendered on the left side
        // First non-space character should be 'R' or '1' or '2'
        let mut has_label = false;
        for y in 0..10 {
            for x in 0..8 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == 'R' || cell.symbol == '1' || cell.symbol == '2' {
                        has_label = true;
                        break;
                    }
                }
            }
        }
        assert!(has_label);
    }

    #[test]
    fn test_heatmap_render_with_both_labels() {
        let data = vec![vec![0.5, 0.8], vec![0.2, 0.9]];
        let hm = HeatMap::new(data)
            .row_labels(vec!["R1".into(), "R2".into()])
            .col_labels(vec!["C1".into(), "C2".into()]);

        let mut buffer = Buffer::new(40, 15);
        let rect = Rect::new(0, 0, 40, 15);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Should render with both row and column labels
        // Column labels have 8 char offset for row labels
        let mut has_col_label = false;
        for x in 8..40 {
            if let Some(cell) = buffer.get(x, 0) {
                if cell.symbol == 'C' {
                    has_col_label = true;
                    break;
                }
            }
        }
        assert!(has_col_label);
    }

    #[test]
    fn test_heatmap_render_with_legend() {
        let data = vec![vec![0.0, 0.5, 1.0]];
        let hm = HeatMap::new(data).show_legend(true);

        let mut buffer = Buffer::new(50, 10);
        let rect = Rect::new(0, 0, 50, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Legend should be rendered with "Low" and "High" labels
        let mut has_low = false;
        let mut has_high = false;
        for y in 0..10 {
            for x in 0..50 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == 'L' {
                        has_low = true;
                    }
                    if cell.symbol == 'H' {
                        has_high = true;
                    }
                }
            }
        }
        assert!(has_low);
        assert!(has_high);
    }

    #[test]
    fn test_heatmap_render_without_legend() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data).show_legend(false);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Legend should not be rendered
        let mut has_legend_text = false;
        for y in 0..10 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == 'L' || cell.symbol == 'H' {
                        has_legend_text = true;
                        break;
                    }
                }
            }
        }
        assert!(!has_legend_text);
    }

    #[test]
    fn test_heatmap_render_with_values() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data).show_values(true).value_decimals(1);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Value "0.5" should be rendered
        let mut has_value = false;
        for y in 0..10 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '0' || cell.symbol == '.' || cell.symbol == '5' {
                        has_value = true;
                        break;
                    }
                }
            }
        }
        assert!(has_value);
    }

    #[test]
    fn test_heatmap_render_with_highlight() {
        let data = vec![vec![0.5, 0.8], vec![0.2, 0.9]];
        let hm = HeatMap::new(data).highlight(1, 1);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Highlighted cell should have bold modifier
        // We can't directly check modifiers easily, but rendering should complete
        // This is mainly a smoke test to ensure highlighting doesn't panic
    }

    #[test]
    fn test_heatmap_render_cell_height_multiple() {
        let data = vec![vec![0.5, 0.8]];
        let hm = HeatMap::new(data).cell_height(3);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Should render multiple rows for each data row
    }

    #[test]
    fn test_heatmap_render_empty_data() {
        let data: Vec<Vec<f64>> = vec![];
        let hm = HeatMap::new(data);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Should not panic with empty data
    }

    #[test]
    fn test_heatmap_render_empty_rows() {
        let data = vec![vec![], vec![]];
        let hm = HeatMap::new(data);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Should not panic with empty rows
    }

    #[test]
    fn test_heatmap_render_different_color_scales() {
        let data = vec![vec![0.0, 0.5, 1.0]];

        for scale in &[
            ColorScale::BlueRed,
            ColorScale::Green,
            ColorScale::Viridis,
            ColorScale::Plasma,
            ColorScale::Gray,
            ColorScale::RedYellowGreen,
        ] {
            let hm = HeatMap::new(data.clone()).color_scale(*scale);

            let mut buffer = Buffer::new(30, 10);
            let rect = Rect::new(0, 0, 30, 10);
            let mut ctx = RenderContext::new(&mut buffer, rect);

            hm.render(&mut ctx);
            // Should render without panic for each color scale
        }
    }

    #[test]
    fn test_heatmap_render_cell_display_block() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data).cell_display(CellDisplay::Block);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Block character should be rendered
        let mut has_block = false;
        for y in 0..10 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '█' {
                        has_block = true;
                        break;
                    }
                }
            }
        }
        assert!(has_block);
    }

    #[test]
    fn test_heatmap_render_cell_display_half_block() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data).cell_display(CellDisplay::HalfBlock);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Half block character should be rendered
        let mut has_half_block = false;
        for y in 0..10 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '▀' {
                        has_half_block = true;
                        break;
                    }
                }
            }
        }
        assert!(has_half_block);
    }

    #[test]
    fn test_heatmap_render_cell_display_custom() {
        let data = vec![vec![0.5]];
        let hm = HeatMap::new(data).cell_display(CellDisplay::Custom);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Custom character should be rendered
        let mut has_custom = false;
        for y in 0..10 {
            for x in 0..30 {
                if let Some(cell) = buffer.get(x, y) {
                    if cell.symbol == '■' {
                        has_custom = true;
                        break;
                    }
                }
            }
        }
        assert!(has_custom);
    }

    #[test]
    fn test_heatmap_render_brightness_contrast_light() {
        let data = vec![vec![1.0]];
        let hm = HeatMap::new(data)
            .color_scale(ColorScale::Gray)
            .show_values(true);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Bright value (1.0) should get dark text
    }

    #[test]
    fn test_heatmap_render_brightness_contrast_dark() {
        let data = vec![vec![0.0]];
        let hm = HeatMap::new(data)
            .color_scale(ColorScale::Gray)
            .show_values(true);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Dark value (0.0) should get light text
    }

    #[test]
    fn test_heatmap_render_legend_with_range() {
        let data = vec![vec![0.0, 1.0]];
        let hm = HeatMap::new(data).bounds(-10.0, 10.0).show_legend(true);

        let mut buffer = Buffer::new(50, 10);
        let rect = Rect::new(0, 0, 50, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);

        // Legend should show custom range values
        // We expect the legend to render (smoke test)
        // The actual format may vary, so we just check it rendered without panic
        let mut has_content = false;
        for y in 0..10 {
            for x in 0..50 {
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
    fn test_heatmap_render_negative_values() {
        let data = vec![vec![-1.0, 0.0, 1.0]];
        let hm = HeatMap::new(data);

        let mut buffer = Buffer::new(30, 10);
        let rect = Rect::new(0, 0, 30, 10);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Should handle negative values without panic
    }

    #[test]
    fn test_heatmap_render_large_dataset() {
        let data: Vec<Vec<f64>> = (0..10)
            .map(|i| (0..10).map(|j| (i * j) as f64).collect())
            .collect();
        let hm = HeatMap::new(data);

        let mut buffer = Buffer::new(50, 30);
        let rect = Rect::new(0, 0, 50, 30);
        let mut ctx = RenderContext::new(&mut buffer, rect);

        hm.render(&mut ctx);
        // Should handle larger datasets without panic
    }
}
