//! Dialog rendering for RichTextEditor

use super::{DialogType, RichTextEditor};
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::RenderContext;

impl RichTextEditor {
    /// Render dialog
    pub(crate) fn render_dialog(
        &self,
        ctx: &mut RenderContext,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) {
        // Calculate dialog position (centered)
        let dialog_width = 40.min(width.saturating_sub(4));
        let dialog_height = 7;
        let dialog_x = x + (width.saturating_sub(dialog_width)) / 2;
        let dialog_y = y + (height.saturating_sub(dialog_height)) / 2;

        let bg = Color::rgb(49, 50, 68);
        let fg = Color::rgb(205, 214, 244);

        // Draw dialog background
        for row in 0..dialog_height {
            for col in 0..dialog_width {
                ctx.buffer
                    .set(dialog_x + col, dialog_y + row, Cell::new(' ').bg(bg));
            }
        }

        // Draw border
        ctx.buffer
            .set(dialog_x, dialog_y, Cell::new('┌').fg(fg).bg(bg));
        ctx.buffer.set(
            dialog_x + dialog_width - 1,
            dialog_y,
            Cell::new('┐').fg(fg).bg(bg),
        );
        ctx.buffer.set(
            dialog_x,
            dialog_y + dialog_height - 1,
            Cell::new('└').fg(fg).bg(bg),
        );
        ctx.buffer.set(
            dialog_x + dialog_width - 1,
            dialog_y + dialog_height - 1,
            Cell::new('┘').fg(fg).bg(bg),
        );
        for col in 1..dialog_width - 1 {
            ctx.buffer
                .set(dialog_x + col, dialog_y, Cell::new('─').fg(fg).bg(bg));
            ctx.buffer.set(
                dialog_x + col,
                dialog_y + dialog_height - 1,
                Cell::new('─').fg(fg).bg(bg),
            );
        }
        for row in 1..dialog_height - 1 {
            ctx.buffer
                .set(dialog_x, dialog_y + row, Cell::new('│').fg(fg).bg(bg));
            ctx.buffer.set(
                dialog_x + dialog_width - 1,
                dialog_y + row,
                Cell::new('│').fg(fg).bg(bg),
            );
        }

        match &self.dialog {
            DialogType::InsertLink { text, url, field } => {
                // Title
                let title = "Insert Link";
                let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;
                for (i, ch) in title.chars().enumerate() {
                    ctx.buffer.set(
                        title_x + i as u16,
                        dialog_y + 1,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }

                // Text field
                let label = "Text: ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in text.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 3,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }

                // URL field
                let label = "URL:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in url.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 4,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }
            }
            DialogType::InsertImage { alt, src, field } => {
                // Title
                let title = "Insert Image";
                let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;
                for (i, ch) in title.chars().enumerate() {
                    ctx.buffer.set(
                        title_x + i as u16,
                        dialog_y + 1,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }

                // Alt field
                let label = "Alt:  ";
                let input_bg = if *field == 0 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 3,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in alt.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 3,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }

                // Src field
                let label = "Src:  ";
                let input_bg = if *field == 1 { self.selection_bg } else { bg };
                for (i, ch) in label.chars().enumerate() {
                    ctx.buffer.set(
                        dialog_x + 2 + i as u16,
                        dialog_y + 4,
                        Cell::new(ch).fg(fg).bg(bg),
                    );
                }
                for (i, ch) in src.chars().enumerate() {
                    if dialog_x + 8 + (i as u16) < dialog_x + dialog_width - 2 {
                        ctx.buffer.set(
                            dialog_x + 8 + i as u16,
                            dialog_y + 4,
                            Cell::new(ch).fg(fg).bg(input_bg),
                        );
                    }
                }
            }
            DialogType::None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_render_dialog_none_no_render() {
        // Arrange
        let editor = RichTextEditor::new();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - DialogType::None should not render any dialog content
        // Check that center area doesn't have dialog border
        let center_x = 40;
        let center_y = 12;
        let cell = buffer.get(center_x, center_y).unwrap();
        assert_eq!(cell.symbol, ' ');
    }

    #[test]
    fn test_render_dialog_insert_link_shows_dialog() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check for dialog border
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check top-left corner
        let cell = buffer.get(dialog_x, dialog_y).unwrap();
        assert_eq!(cell.symbol, '┌');

        // Check top-right corner
        let cell = buffer.get(dialog_x + dialog_width - 1, dialog_y).unwrap();
        assert_eq!(cell.symbol, '┐');
    }

    #[test]
    fn test_render_dialog_insert_image_shows_dialog() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_image_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check for dialog border
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check bottom-left corner
        let cell = buffer.get(dialog_x, dialog_y + dialog_height - 1).unwrap();
        assert_eq!(cell.symbol, '└');
    }

    #[test]
    fn test_render_dialog_insert_link_title() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check title "Insert Link" is rendered
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;
        let title = "Insert Link";
        let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;

        // Check first character of title
        let cell = buffer.get(title_x, dialog_y + 1).unwrap();
        assert_eq!(cell.symbol, 'I');

        // Check last character of title
        let cell = buffer
            .get(title_x + title.len() as u16 - 1, dialog_y + 1)
            .unwrap();
        assert_eq!(cell.symbol, 'k');
    }

    #[test]
    fn test_render_dialog_insert_image_title() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_image_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check title "Insert Image" is rendered
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;
        let title = "Insert Image";
        let title_x = dialog_x + (dialog_width - title.len() as u16) / 2;

        // Check first character of title
        let cell = buffer.get(title_x, dialog_y + 1).unwrap();
        assert_eq!(cell.symbol, 'I');
    }

    #[test]
    fn test_render_dialog_insert_link_field_0_selected() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Field 0 should have default background when empty
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check text field (field 0) has dialog background at position 8 (when empty)
        let cell = buffer.get(dialog_x + 8, dialog_y + 3).unwrap();
        assert_eq!(cell.bg, Some(Color::rgb(49, 50, 68)));
    }

    #[test]
    fn test_render_dialog_insert_link_text_field_label() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check "Text: " label is present
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check 'T' of "Text:"
        let cell = buffer.get(dialog_x + 2, dialog_y + 3).unwrap();
        assert_eq!(cell.symbol, 'T');
    }

    #[test]
    fn test_render_dialog_insert_link_url_field_label() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check "URL:  " label is present
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check 'U' of "URL:"
        let cell = buffer.get(dialog_x + 2, dialog_y + 4).unwrap();
        assert_eq!(cell.symbol, 'U');
    }

    #[test]
    fn test_render_dialog_insert_image_alt_field_label() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_image_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check "Alt:  " label is present
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check 'A' of "Alt:"
        let cell = buffer.get(dialog_x + 2, dialog_y + 3).unwrap();
        assert_eq!(cell.symbol, 'A');
    }

    #[test]
    fn test_render_dialog_insert_image_src_field_label() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_image_dialog();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check "Src:  " label is present
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check 'S' of "Src:"
        let cell = buffer.get(dialog_x + 2, dialog_y + 4).unwrap();
        assert_eq!(cell.symbol, 'S');
    }

    #[test]
    fn test_render_dialog_small_area_width_limit() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(50, 24);
        let area = Rect::new(0, 0, 50, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act - Render in small area (width 50, dialog limited to 46 after subtracting 4)
        editor.render_dialog(&mut ctx, 0, 0, 50, 24);

        // Assert - Dialog should be rendered with limited width
        // Dialog width should be min(40, 50-4) = 40
        let dialog_width = 40;
        let dialog_x = (50 - dialog_width) / 2;
        let dialog_y = (24 - 7) / 2;

        let cell = buffer.get(dialog_x, dialog_y).unwrap();
        assert_eq!(cell.symbol, '┌');
    }

    #[test]
    fn test_render_dialog_very_small_area() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(20, 20);
        let area = Rect::new(0, 0, 20, 20);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act - Render in very small area (width 20, dialog limited to 16)
        editor.render_dialog(&mut ctx, 0, 0, 20, 20);

        // Assert - Should not panic
        // Dialog width should be min(40, 20-4) = 16
        let dialog_width = 16;
        let dialog_x = (20 - dialog_width) / 2;
        let dialog_y = (20 - 7) / 2;

        let cell = buffer.get(dialog_x, dialog_y).unwrap();
        assert_eq!(cell.symbol, '┌');
    }

    #[test]
    fn test_render_dialog_minimal_area() {
        // Arrange
        let mut editor = RichTextEditor::new();
        editor.open_link_dialog();
        let mut buffer = Buffer::new(44, 10);
        let area = Rect::new(0, 0, 44, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Act - Render in minimal area (width 44, dialog width limited to 40)
        editor.render_dialog(&mut ctx, 0, 0, 44, 10);

        // Assert - Should not panic with minimal area
        // This tests edge case handling
        let cell = buffer.get(2, 1).unwrap();
        // Dialog should be rendered
        assert_eq!(cell.symbol, '┌');
    }

    #[test]
    fn test_render_dialog_with_content() {
        // Arrange
        let mut editor = RichTextEditor::new().content("Test Content");
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        editor.open_link_dialog();

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Dialog should render
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check that dialog is rendered
        let cell = buffer.get(dialog_x, dialog_y).unwrap();
        assert_eq!(cell.symbol, '┌');
    }

    #[test]
    fn test_render_dialog_background_color() {
        // Arrange
        let mut editor = RichTextEditor::new();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        editor.open_link_dialog();

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check background color is set correctly
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check a cell in the middle of the dialog has the correct background
        let cell = buffer.get(dialog_x + 1, dialog_y + 1).unwrap();
        assert_eq!(cell.bg, Some(Color::rgb(49, 50, 68)));
    }

    #[test]
    fn test_render_dialog_foreground_color() {
        // Arrange
        let mut editor = RichTextEditor::new();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        editor.open_link_dialog();

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check foreground color is set correctly
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check border has correct foreground color
        let cell = buffer.get(dialog_x, dialog_y).unwrap();
        assert_eq!(cell.fg, Some(Color::rgb(205, 214, 244)));
    }

    #[test]
    fn test_render_dialog_border_characters() {
        // Arrange
        let mut editor = RichTextEditor::new();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        editor.open_link_dialog();

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check all border characters
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Top border
        assert_eq!(buffer.get(dialog_x, dialog_y).unwrap().symbol, '┌');
        assert_eq!(
            buffer
                .get(dialog_x + dialog_width - 1, dialog_y)
                .unwrap()
                .symbol,
            '┐'
        );

        // Bottom border
        assert_eq!(
            buffer
                .get(dialog_x, dialog_y + dialog_height - 1)
                .unwrap()
                .symbol,
            '└'
        );
        assert_eq!(
            buffer
                .get(dialog_x + dialog_width - 1, dialog_y + dialog_height - 1)
                .unwrap()
                .symbol,
            '┘'
        );

        // Side borders
        assert_eq!(buffer.get(dialog_x, dialog_y + 1).unwrap().symbol, '│');
        assert_eq!(
            buffer
                .get(dialog_x + dialog_width - 1, dialog_y + 1)
                .unwrap()
                .symbol,
            '│'
        );
    }

    #[test]
    fn test_render_dialog_horizontal_border() {
        // Arrange
        let mut editor = RichTextEditor::new();
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        editor.open_link_dialog();

        // Act
        editor.render_dialog(&mut ctx, 0, 0, 80, 24);

        // Assert - Check horizontal border characters
        let dialog_width = 40;
        let dialog_height = 7;
        let dialog_x = (80 - dialog_width) / 2;
        let dialog_y = (24 - dialog_height) / 2;

        // Check horizontal borders on top and bottom
        assert_eq!(buffer.get(dialog_x + 1, dialog_y).unwrap().symbol, '─');
        assert_eq!(
            buffer
                .get(dialog_x + dialog_width - 2, dialog_y)
                .unwrap()
                .symbol,
            '─'
        );
        assert_eq!(
            buffer
                .get(dialog_x + 1, dialog_y + dialog_height - 1)
                .unwrap()
                .symbol,
            '─'
        );
    }
}
