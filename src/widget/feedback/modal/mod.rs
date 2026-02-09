//! Modal/Dialog widget for displaying overlays

use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View, WidgetProps};
use crate::{impl_props_builders, impl_styled_view};

/// Button configuration for modal dialogs
///
/// This is distinct from the interactive `Button` widget.
/// `ModalButton` configures the appearance and label of buttons
/// shown at the bottom of modal dialogs.
#[derive(Clone)]
pub struct ModalButton {
    /// Button label
    pub label: String,
    /// Button style
    pub style: ModalButtonStyle,
}

/// Style preset for modal buttons
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ModalButtonStyle {
    /// Default neutral button
    #[default]
    Default,
    /// Primary action button (highlighted)
    Primary,
    /// Danger/destructive action button
    Danger,
}

impl ModalButton {
    /// Create a new button with default style
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Default,
        }
    }

    /// Create a primary action button
    pub fn primary(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Primary,
        }
    }

    /// Create a danger/destructive action button
    pub fn danger(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ModalButtonStyle::Danger,
        }
    }

    /// Set button style
    pub fn style(mut self, style: ModalButtonStyle) -> Self {
        self.style = style;
        self
    }
}

/// A modal dialog widget
pub struct Modal {
    title: String,
    /// Text content (for simple messages)
    content: Vec<String>,
    /// Child widget content (takes precedence over text content)
    body: Option<Box<dyn View>>,
    buttons: Vec<ModalButton>,
    selected_button: usize,
    visible: bool,
    width: u16,
    height: Option<u16>,
    title_fg: Option<Color>,
    border_fg: Option<Color>,
    props: WidgetProps,
}

impl Modal {
    /// Create a new modal dialog
    pub fn new() -> Self {
        Self {
            title: String::new(),
            content: Vec::new(),
            body: None,
            buttons: Vec::new(),
            selected_button: 0,
            visible: false,
            width: 40,
            height: None,
            title_fg: Some(Color::WHITE),
            border_fg: Some(Color::WHITE),
            props: WidgetProps::new(),
        }
    }

    /// Set modal title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set modal content
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = content.into().lines().map(|s| s.to_string()).collect();
        self
    }

    /// Add a line to content
    pub fn line(mut self, line: impl Into<String>) -> Self {
        self.content.push(line.into());
        self
    }

    /// Set buttons
    pub fn buttons(mut self, buttons: Vec<ModalButton>) -> Self {
        self.buttons = buttons;
        self
    }

    /// Add OK button
    pub fn ok(mut self) -> Self {
        self.buttons.push(ModalButton::primary("OK"));
        self
    }

    /// Add Cancel button
    pub fn cancel(mut self) -> Self {
        self.buttons.push(ModalButton::new("Cancel"));
        self
    }

    /// Add OK and Cancel buttons
    pub fn ok_cancel(mut self) -> Self {
        self.buttons.push(ModalButton::primary("OK"));
        self.buttons.push(ModalButton::new("Cancel"));
        self
    }

    /// Add Yes and No buttons
    pub fn yes_no(mut self) -> Self {
        self.buttons.push(ModalButton::primary("Yes"));
        self.buttons.push(ModalButton::new("No"));
        self
    }

    /// Add Yes, No, and Cancel buttons
    pub fn yes_no_cancel(mut self) -> Self {
        self.buttons.push(ModalButton::primary("Yes"));
        self.buttons.push(ModalButton::new("No"));
        self.buttons.push(ModalButton::new("Cancel"));
        self
    }

    /// Set modal width
    pub fn width(mut self, width: u16) -> Self {
        self.width = width;
        self
    }

    /// Set modal height (None = auto)
    pub fn height(mut self, height: u16) -> Self {
        self.height = Some(height);
        self
    }

    /// Set a child widget as body content
    ///
    /// When a body widget is set, it takes precedence over text content.
    /// The widget will be rendered inside the modal's content area.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use revue::prelude::*;
    ///
    /// let modal = Modal::new()
    ///     .title("User Form")
    ///     .body(
    ///         vstack()
    ///             .gap(1)
    ///             .child(Input::new().placeholder("Name"))
    ///             .child(Input::new().placeholder("Email"))
    ///     )
    ///     .ok_cancel();
    /// ```
    pub fn body(mut self, widget: impl View + 'static) -> Self {
        self.body = Some(Box::new(widget));
        self
    }

    /// Set title color
    pub fn title_fg(mut self, color: Color) -> Self {
        self.title_fg = Some(color);
        self
    }

    /// Set border color
    pub fn border_fg(mut self, color: Color) -> Self {
        self.border_fg = Some(color);
        self
    }

    /// Show the modal
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Hide the modal
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if modal is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Get selected button index
    pub fn selected_button(&self) -> usize {
        self.selected_button
    }

    /// Select next button
    pub fn next_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = (self.selected_button + 1) % self.buttons.len();
        }
    }

    /// Select previous button
    pub fn prev_button(&mut self) {
        if !self.buttons.is_empty() {
            self.selected_button = self
                .selected_button
                .checked_sub(1)
                .unwrap_or(self.buttons.len() - 1);
        }
    }

    /// Handle key input, returns Some(button_index) if button confirmed
    pub fn handle_key(&mut self, key: &crate::event::Key) -> Option<usize> {
        use crate::event::Key;

        match key {
            Key::Enter | Key::Char(' ') => {
                if !self.buttons.is_empty() {
                    Some(self.selected_button)
                } else {
                    None
                }
            }
            Key::Left | Key::Char('h') => {
                self.prev_button();
                None
            }
            Key::Right | Key::Char('l') => {
                self.next_button();
                None
            }
            Key::Tab => {
                self.next_button();
                None
            }
            Key::Escape => {
                self.hide();
                None
            }
            _ => None,
        }
    }

    /// Create alert dialog
    pub fn alert(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new().title(title).content(message).ok()
    }

    /// Create confirmation dialog
    pub fn confirm(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self::new().title(title).content(message).yes_no()
    }

    /// Create error dialog
    pub fn error(message: impl Into<String>) -> Self {
        Self::new()
            .title("Error")
            .title_fg(Color::RED)
            .border_fg(Color::RED)
            .content(message)
            .ok()
    }

    /// Create warning dialog
    pub fn warning(message: impl Into<String>) -> Self {
        Self::new()
            .title("Warning")
            .title_fg(Color::YELLOW)
            .border_fg(Color::YELLOW)
            .content(message)
            .ok()
    }

    /// Calculate required height
    fn required_height(&self) -> u16 {
        // If height is explicitly set, use it
        if let Some(h) = self.height {
            return h;
        }

        // For body widget, use a default content height of 5 lines
        let content_lines = if self.body.is_some() {
            5u16
        } else {
            self.content.len() as u16
        };

        let button_line = if self.buttons.is_empty() { 0 } else { 1 };
        // top border + title + title separator + content + padding + buttons + bottom border
        3 + content_lines + 1 + button_line + 1
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Modal {
    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible {
            return;
        }

        let area = ctx.area;
        let modal_width = self.width.min(area.width.saturating_sub(4));
        let modal_height = self.required_height().min(area.height.saturating_sub(2));

        // Center the modal
        let x = area.x + (area.width.saturating_sub(modal_width)) / 2;
        let y = area.y + (area.height.saturating_sub(modal_height)) / 2;

        // Draw border
        self.render_border(ctx, x, y, modal_width, modal_height);

        // Draw title
        if !self.title.is_empty() && modal_width > 4 {
            let title_x = x + 2;
            let title_width = modal_width.saturating_sub(4) as usize;
            let title: String = self.title.chars().take(title_width).collect();

            for (i, ch) in title.chars().enumerate() {
                let mut cell = Cell::new(ch);
                cell.fg = self.title_fg;
                cell.modifier |= crate::render::Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, y + 1, cell);
            }

            // Title separator
            for dx in 1..modal_width.saturating_sub(1) {
                ctx.buffer.set(x + dx, y + 2, Cell::new('─'));
            }
            ctx.buffer.set(x, y + 2, Cell::new('├'));
            ctx.buffer
                .set(x + modal_width.saturating_sub(1), y + 2, Cell::new('┤'));
        }

        // Draw content
        let content_y = y + 3;
        let content_width = modal_width.saturating_sub(4);
        let content_height = modal_height.saturating_sub(6); // title + separator + padding + buttons + borders

        if let Some(ref body_widget) = self.body {
            // Render child widget
            let content_area =
                crate::layout::Rect::new(x + 2, content_y, content_width, content_height);
            let mut body_ctx = RenderContext::new(ctx.buffer, content_area);
            body_widget.render(&mut body_ctx);
        } else {
            // Render text content
            for (i, line) in self.content.iter().enumerate() {
                let cy = content_y + i as u16;
                if cy >= y + modal_height - 2 {
                    break;
                }
                let truncated: String = line.chars().take(content_width as usize).collect();
                for (j, ch) in truncated.chars().enumerate() {
                    ctx.buffer.set(x + 2 + j as u16, cy, Cell::new(ch));
                }
            }
        }

        // Draw buttons
        if !self.buttons.is_empty() && modal_height > 2 {
            let button_y = y + modal_height.saturating_sub(2);
            let total_button_width: usize = self
                .buttons
                .iter()
                .map(|b| b.label.len() + 4) // [ label ]
                .sum::<usize>()
                + (self.buttons.len() - 1) * 2; // spacing

            // Skip drawing buttons if they don't fit
            if total_button_width as u16 > modal_width {
                return;
            }
            let start_x = x + (modal_width.saturating_sub(total_button_width as u16)) / 2;
            let mut bx = start_x;

            for (i, button) in self.buttons.iter().enumerate() {
                let is_selected = i == self.selected_button;
                let button_text = format!("[ {} ]", button.label);

                let (fg, bg) = if is_selected {
                    match button.style {
                        ModalButtonStyle::Primary => (Some(Color::WHITE), Some(Color::BLUE)),
                        ModalButtonStyle::Danger => (Some(Color::WHITE), Some(Color::RED)),
                        ModalButtonStyle::Default => (Some(Color::BLACK), Some(Color::WHITE)),
                    }
                } else {
                    (None, None)
                };

                for (j, ch) in button_text.chars().enumerate() {
                    let mut cell = Cell::new(ch);
                    cell.fg = fg;
                    cell.bg = bg;
                    ctx.buffer.set(bx + j as u16, button_y, cell);
                }

                bx += button_text.len() as u16 + 2;
            }
        }
    }

    crate::impl_view_meta!("Modal");
}

impl Modal {
    fn render_border(&self, ctx: &mut RenderContext, x: u16, y: u16, width: u16, height: u16) {
        // Clear interior with spaces
        for dy in 1..height.saturating_sub(1) {
            for dx in 1..width.saturating_sub(1) {
                ctx.buffer.set(x + dx, y + dy, Cell::new(' '));
            }
        }

        // Top border
        let mut corner = Cell::new('┌');
        corner.fg = self.border_fg;
        ctx.buffer.set(x, y, corner);

        for dx in 1..width.saturating_sub(1) {
            let mut cell = Cell::new('─');
            cell.fg = self.border_fg;
            ctx.buffer.set(x + dx, y, cell);
        }

        let mut corner = Cell::new('┐');
        corner.fg = self.border_fg;
        ctx.buffer.set(x + width.saturating_sub(1), y, corner);

        // Sides
        for dy in 1..height.saturating_sub(1) {
            let mut cell = Cell::new('│');
            cell.fg = self.border_fg;
            ctx.buffer.set(x, y + dy, cell);
            ctx.buffer.set(x + width.saturating_sub(1), y + dy, cell);
        }

        // Bottom border
        let mut corner = Cell::new('└');
        corner.fg = self.border_fg;
        ctx.buffer.set(x, y + height.saturating_sub(1), corner);

        for dx in 1..width.saturating_sub(1) {
            let mut cell = Cell::new('─');
            cell.fg = self.border_fg;
            ctx.buffer.set(x + dx, y + height.saturating_sub(1), cell);
        }

        let mut corner = Cell::new('┘');
        corner.fg = self.border_fg;
        ctx.buffer.set(
            x + width.saturating_sub(1),
            y + height.saturating_sub(1),
            corner,
        );
    }
}

/// Helper function to create a modal
pub fn modal() -> Modal {
    Modal::new()
}

impl_styled_view!(Modal);
impl_props_builders!(Modal);

#[cfg(test)]
mod tests {
    use super::*;

    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_modal_new() {
        let m = Modal::new();
        assert!(!m.is_visible());
        assert!(m.title.is_empty());
        assert!(m.content.is_empty());
        assert!(m.buttons.is_empty());
    }

    #[test]
    fn test_modal_builder() {
        let m = Modal::new()
            .title("Test")
            .content(
                "Hello
World",
            )
            .ok_cancel();

        assert_eq!(m.title, "Test");
        assert_eq!(m.content.len(), 2);
        assert_eq!(m.buttons.len(), 2);
    }

    #[test]
    fn test_modal_visibility() {
        let mut m = Modal::new();
        assert!(!m.is_visible());

        m.show();
        assert!(m.is_visible());

        m.hide();
        assert!(!m.is_visible());

        m.toggle();
        assert!(m.is_visible());
    }

    #[test]
    fn test_modal_button_navigation() {
        let mut m = Modal::new().ok_cancel();

        assert_eq!(m.selected_button(), 0);

        m.next_button();
        assert_eq!(m.selected_button(), 1);

        m.next_button(); // Wraps around
        assert_eq!(m.selected_button(), 0);

        m.prev_button(); // Wraps around
        assert_eq!(m.selected_button(), 1);
    }

    #[test]
    fn test_modal_handle_key() {
        use crate::event::Key;

        let mut m = Modal::new().yes_no();
        m.show();

        // Navigate buttons
        m.handle_key(&Key::Right);
        assert_eq!(m.selected_button(), 1);

        m.handle_key(&Key::Left);
        assert_eq!(m.selected_button(), 0);

        // Confirm selection
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, Some(0));

        // Escape closes
        m.handle_key(&Key::Escape);
        assert!(!m.is_visible());
    }

    #[test]
    fn test_modal_presets() {
        let alert = Modal::alert("Title", "Message");
        assert_eq!(alert.title, "Title");
        assert_eq!(alert.buttons.len(), 1);

        let confirm = Modal::confirm("Title", "Question?");
        assert_eq!(confirm.buttons.len(), 2);

        let error = Modal::error("Something went wrong");
        assert_eq!(error.title, "Error");
    }

    #[test]
    fn test_modal_render_hidden() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let m = Modal::new().title("Test");
        m.render(&mut ctx);

        // Hidden modal shouldn't render anything special
        assert_eq!(buffer.get(0, 0).unwrap().symbol, ' ');
    }

    #[test]
    fn test_modal_render_visible() {
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("Test Dialog").content("Hello").ok();
        m.show();
        m.render(&mut ctx);

        // Modal should render centered - check for border characters
        // The exact position depends on centering calculation
        let center_x = (80 - 40) / 2;
        let center_y = (24 - m.required_height()) / 2;

        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_button_styles() {
        let btn = ModalButton::new("Test");
        assert!(matches!(btn.style, ModalButtonStyle::Default));

        let btn = ModalButton::primary("OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));

        let btn = ModalButton::danger("Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_helper() {
        let m = modal().title("Quick").ok();

        assert_eq!(m.title, "Quick");
    }

    #[test]
    fn test_modal_with_body() {
        use crate::widget::Text;

        let m = Modal::new()
            .title("Form")
            .body(Text::new("Custom content"))
            .height(10)
            .ok();

        assert!(m.body.is_some());
        assert_eq!(m.height, Some(10));
    }

    #[test]
    fn test_modal_body_render() {
        use crate::widget::Text;

        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new()
            .title("Body Test")
            .body(Text::new("Widget content"))
            .width(50)
            .height(12)
            .ok();
        m.show();
        m.render(&mut ctx);

        // Modal with body should render
        let center_x = (80 - 50) / 2;
        let center_y = (24 - 12) / 2;
        assert_eq!(buffer.get(center_x, center_y).unwrap().symbol, '┌');
    }

    #[test]
    fn test_modal_render_small_area_no_panic() {
        // Test that rendering in very small areas doesn't panic
        // This is the fix for issue #154

        // Width = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Width = 1
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 1, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Width = 2
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 2, 24);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 1
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 1);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Height = 2
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 80, 2);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic

        // Both width and height = 0
        let mut buffer = Buffer::new(80, 24);
        let area = Rect::new(0, 0, 0, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);
        let mut m = Modal::new().title("Test").ok();
        m.show();
        m.render(&mut ctx); // Should not panic
    }

    #[test]
    fn test_modal_render_width_2_border() {
        // Specific test for width=2 which was mentioned in the issue
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 4, 10); // Small width after subtracting 4 for margins
        let mut ctx = RenderContext::new(&mut buffer, area);

        let mut m = Modal::new().title("X").width(2).height(4);
        m.show();
        m.render(&mut ctx); // Should not panic
    }

    // =========================================================================
    // ModalButtonStyle enum tests
    // =========================================================================

    #[test]
    fn test_modal_button_style_default() {
        let style = ModalButtonStyle::default();
        assert!(matches!(style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_style_clone() {
        let style1 = ModalButtonStyle::Primary;
        let style2 = style1.clone();
        assert_eq!(style1, style2);
    }

    #[test]
    fn test_modal_button_style_copy() {
        let style1 = ModalButtonStyle::Danger;
        let style2 = style1;
        assert_eq!(style2, ModalButtonStyle::Danger);
        // style1 is still valid because of Copy
        assert_eq!(style1, ModalButtonStyle::Danger);
    }

    #[test]
    fn test_modal_button_style_partial_eq() {
        assert_eq!(ModalButtonStyle::Default, ModalButtonStyle::Default);
        assert_eq!(ModalButtonStyle::Primary, ModalButtonStyle::Primary);
        assert_eq!(ModalButtonStyle::Danger, ModalButtonStyle::Danger);

        assert_ne!(ModalButtonStyle::Default, ModalButtonStyle::Primary);
        assert_ne!(ModalButtonStyle::Primary, ModalButtonStyle::Danger);
        assert_ne!(ModalButtonStyle::Danger, ModalButtonStyle::Default);
    }

    #[test]
    fn test_modal_button_style_all_variants() {
        let styles = [
            ModalButtonStyle::Default,
            ModalButtonStyle::Primary,
            ModalButtonStyle::Danger,
        ];

        for (i, style1) in styles.iter().enumerate() {
            for (j, style2) in styles.iter().enumerate() {
                if i == j {
                    assert_eq!(style1, style2);
                } else {
                    assert_ne!(style1, style2);
                }
            }
        }
    }

    // =========================================================================
    // ModalButton Clone trait tests
    // =========================================================================

    #[test]
    fn test_modal_button_clone() {
        let btn1 = ModalButton::new("Test").style(ModalButtonStyle::Primary);
        let btn2 = btn1.clone();

        assert_eq!(btn1.label, btn2.label);
        assert_eq!(btn1.style, btn2.style);
    }

    // =========================================================================
    // ModalButton builder method tests
    // =========================================================================

    #[test]
    fn test_modal_button_new_with_string() {
        let label = String::from("Owned Label");
        let btn = ModalButton::new(label);
        assert_eq!(btn.label, "Owned Label");
        assert!(matches!(btn.style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_new_with_str() {
        let btn = ModalButton::new("Test Label");
        assert_eq!(btn.label, "Test Label");
        assert!(matches!(btn.style, ModalButtonStyle::Default));
    }

    #[test]
    fn test_modal_button_empty_label() {
        let btn = ModalButton::new("");
        assert_eq!(btn.label, "");
    }

    #[test]
    fn test_modal_button_primary_with_string() {
        let label = String::from("Submit");
        let btn = ModalButton::primary(label);
        assert_eq!(btn.label, "Submit");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));
    }

    #[test]
    fn test_modal_button_primary_with_str() {
        let btn = ModalButton::primary("OK");
        assert_eq!(btn.label, "OK");
        assert!(matches!(btn.style, ModalButtonStyle::Primary));
    }

    #[test]
    fn test_modal_button_danger_with_string() {
        let label = String::from("Delete");
        let btn = ModalButton::danger(label);
        assert_eq!(btn.label, "Delete");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_button_danger_with_str() {
        let btn = ModalButton::danger("Cancel");
        assert_eq!(btn.label, "Cancel");
        assert!(matches!(btn.style, ModalButtonStyle::Danger));
    }

    #[test]
    fn test_modal_button_all_distinct() {
        let default_btn = ModalButton::new("Default");
        let primary_btn = ModalButton::primary("Primary");
        let danger_btn = ModalButton::danger("Danger");

        assert!(matches!(default_btn.style, ModalButtonStyle::Default));
        assert!(matches!(primary_btn.style, ModalButtonStyle::Primary));
        assert!(matches!(danger_btn.style, ModalButtonStyle::Danger));
    }

    // =========================================================================
    // Modal builder method tests for edge cases
    // =========================================================================

    #[test]
    fn test_modal_empty_title() {
        let m = Modal::new().title("");
        assert_eq!(m.title, "");
    }

    #[test]
    fn test_modal_empty_content() {
        let m = Modal::new().content("");
        assert!(m.content.is_empty());
    }

    #[test]
    fn test_modal_content_with_multiline() {
        let m = Modal::new().content("Line 1\nLine 2\nLine 3");
        assert_eq!(m.content.len(), 3);
        assert_eq!(m.content[0], "Line 1");
        assert_eq!(m.content[1], "Line 2");
        assert_eq!(m.content[2], "Line 3");
    }

    #[test]
    fn test_modal_line_multiple() {
        let m = Modal::new().line("Line 1").line("Line 2").line("Line 3");
        assert_eq!(m.content.len(), 3);
    }

    #[test]
    fn test_modal_buttons_empty() {
        let m = Modal::new().buttons(vec![]);
        assert!(m.buttons.is_empty());
    }

    #[test]
    fn test_modal_width_zero() {
        let m = Modal::new().width(0);
        assert_eq!(m.width, 0);
    }

    #[test]
    fn test_modal_height_zero() {
        let m = Modal::new().height(0);
        assert_eq!(m.height, Some(0));
    }

    #[test]
    fn test_modal_title_colors() {
        let m = Modal::new().title_fg(Color::CYAN);
        assert_eq!(m.title_fg, Some(Color::CYAN));
    }

    #[test]
    fn test_modal_border_colors() {
        let m = Modal::new().border_fg(Color::MAGENTA);
        assert_eq!(m.border_fg, Some(Color::MAGENTA));
    }

    #[test]
    fn test_modal_selected_button_initial() {
        let m = Modal::new();
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_next_button_empty() {
        let mut m = Modal::new();
        m.next_button(); // Should not panic
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_prev_button_empty() {
        let mut m = Modal::new();
        m.prev_button(); // Should not panic
        assert_eq!(m.selected_button(), 0);
    }

    #[test]
    fn test_modal_handle_key_no_buttons() {
        use crate::event::Key;

        let mut m = Modal::new();
        let result = m.handle_key(&Key::Enter);
        assert_eq!(result, None);
    }

    #[test]
    fn test_modal_handle_key_unknown() {
        use crate::event::Key;

        let mut m = Modal::new().ok();
        let result = m.handle_key(&Key::Char('x'));
        assert_eq!(result, None);
    }

    // =========================================================================
    // Modal builder chain tests
    // =========================================================================

    #[test]
    fn test_modal_builder_chain_full() {
        let m = Modal::new()
            .title("Chain Title")
            .content("Chain content")
            .width(60)
            .height(10)
            .title_fg(Color::YELLOW)
            .border_fg(Color::GREEN);

        assert_eq!(m.title, "Chain Title");
        assert_eq!(m.content.len(), 1);
        assert_eq!(m.content[0], "Chain content");
        assert_eq!(m.width, 60);
        assert_eq!(m.height, Some(10));
        assert_eq!(m.title_fg, Some(Color::YELLOW));
        assert_eq!(m.border_fg, Some(Color::GREEN));
    }

    #[test]
    fn test_modal_buttons_builder_chain() {
        let buttons = vec![
            ModalButton::new("One"),
            ModalButton::primary("Two"),
            ModalButton::danger("Three"),
        ];
        let m = Modal::new().buttons(buttons.clone());

        assert_eq!(m.buttons.len(), 3);
        assert_eq!(m.buttons[0].label, "One");
        assert_eq!(m.buttons[1].label, "Two");
        assert_eq!(m.buttons[2].label, "Three");
    }
}
