//! Radio button widget for single selection from options

use super::traits::{RenderContext, View, WidgetProps};
use crate::event::Key;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Radio button style variants
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioStyle {
    /// Parentheses with dot: (●) ( )
    #[default]
    Parentheses,
    /// Unicode radio: ◉ ○
    Unicode,
    /// Brackets with asterisk: \[*\] \[ \]
    Brackets,
    /// Diamond: ◆ ◇
    Diamond,
}

impl RadioStyle {
    /// Get the selected and unselected characters for this style
    fn chars(&self) -> (char, char) {
        match self {
            RadioStyle::Parentheses => ('●', ' '),
            RadioStyle::Unicode => ('◉', '○'),
            RadioStyle::Brackets => ('*', ' '),
            RadioStyle::Diamond => ('◆', '◇'),
        }
    }

    /// Get the bracket characters (if applicable)
    fn brackets(&self) -> (char, char) {
        match self {
            RadioStyle::Parentheses => ('(', ')'),
            RadioStyle::Brackets => ('[', ']'),
            _ => (' ', ' '),
        }
    }

    /// Whether this style uses brackets
    fn has_brackets(&self) -> bool {
        matches!(self, RadioStyle::Parentheses | RadioStyle::Brackets)
    }
}

/// Layout direction for radio group
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioLayout {
    /// Stack options vertically
    #[default]
    Vertical,
    /// Layout options horizontally
    Horizontal,
}

/// A radio button group widget for single selection
#[derive(Clone)]
pub struct RadioGroup {
    options: Vec<String>,
    selected: usize,
    focused: bool,
    disabled: bool,
    style: RadioStyle,
    layout: RadioLayout,
    gap: u16,
    fg: Option<Color>,
    selected_fg: Option<Color>,
    props: WidgetProps,
}

impl RadioGroup {
    /// Create a new radio group with options
    pub fn new<I, S>(options: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            options: options.into_iter().map(|s| s.into()).collect(),
            selected: 0,
            focused: false,
            disabled: false,
            style: RadioStyle::default(),
            layout: RadioLayout::default(),
            gap: 0,
            fg: None,
            selected_fg: None,
            props: WidgetProps::new(),
        }
    }

    /// Set selected index
    pub fn selected(mut self, index: usize) -> Self {
        if index < self.options.len() {
            self.selected = index;
        }
        self
    }

    /// Set focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = focused;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set radio style
    pub fn style(mut self, style: RadioStyle) -> Self {
        self.style = style;
        self
    }

    /// Set layout direction
    pub fn layout(mut self, layout: RadioLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Set gap between options
    pub fn gap(mut self, gap: u16) -> Self {
        self.gap = gap;
        self
    }

    /// Set label color
    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set selected indicator color
    pub fn selected_fg(mut self, color: Color) -> Self {
        self.selected_fg = Some(color);
        self
    }

    /// Get selected index
    pub fn selected_index(&self) -> usize {
        self.selected
    }

    /// Get selected option value
    pub fn selected_value(&self) -> Option<&str> {
        self.options.get(self.selected).map(|s| s.as_str())
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Select next option
    pub fn select_next(&mut self) {
        if !self.disabled && !self.options.is_empty() {
            self.selected = (self.selected + 1) % self.options.len();
        }
    }

    /// Select previous option
    pub fn select_prev(&mut self) {
        if !self.disabled && !self.options.is_empty() {
            self.selected = if self.selected == 0 {
                self.options.len() - 1
            } else {
                self.selected - 1
            };
        }
    }

    /// Set focus state (mutable)
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Set selected index (mutable)
    pub fn set_selected(&mut self, index: usize) {
        if index < self.options.len() {
            self.selected = index;
        }
    }

    /// Handle key input, returns true if selection changed
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if self.disabled {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                self.select_prev();
                true
            }
            Key::Down | Key::Char('j') => {
                self.select_next();
                true
            }
            Key::Left if self.layout == RadioLayout::Horizontal => {
                self.select_prev();
                true
            }
            Key::Right if self.layout == RadioLayout::Horizontal => {
                self.select_next();
                true
            }
            Key::Char(c) if c.is_ascii_digit() => {
                let index = c.to_digit(10).unwrap() as usize;
                if index > 0 && index <= self.options.len() {
                    self.selected = index - 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Render a single radio option
    fn render_option(&self, ctx: &mut RenderContext, index: usize, x: u16, y: u16) -> u16 {
        let area = ctx.area;
        if x >= area.x + area.width || y >= area.y + area.height {
            return 0;
        }

        let is_selected = index == self.selected;
        let (selected_char, unselected_char) = self.style.chars();
        let (left_bracket, right_bracket) = self.style.brackets();
        let has_brackets = self.style.has_brackets();

        let label_fg = if self.disabled {
            Color::rgb(100, 100, 100)
        } else {
            self.fg.unwrap_or(Color::WHITE)
        };

        let indicator_fg = if self.disabled {
            Color::rgb(100, 100, 100)
        } else if is_selected {
            self.selected_fg.unwrap_or(Color::CYAN)
        } else {
            self.fg.unwrap_or(Color::rgb(150, 150, 150))
        };

        let mut current_x = x;

        // Render indicator
        if has_brackets {
            let mut left_cell = Cell::new(left_bracket);
            left_cell.fg = Some(label_fg);
            ctx.buffer.set(current_x, y, left_cell);
            current_x += 1;

            let indicator = if is_selected {
                selected_char
            } else {
                unselected_char
            };
            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(indicator_fg);
            ctx.buffer.set(current_x, y, ind_cell);
            current_x += 1;

            let mut right_cell = Cell::new(right_bracket);
            right_cell.fg = Some(label_fg);
            ctx.buffer.set(current_x, y, right_cell);
            current_x += 1;
        } else {
            let indicator = if is_selected {
                selected_char
            } else {
                unselected_char
            };
            let mut ind_cell = Cell::new(indicator);
            ind_cell.fg = Some(indicator_fg);
            ctx.buffer.set(current_x, y, ind_cell);
            current_x += 1;
        }

        // Space before label
        ctx.buffer.set(current_x, y, Cell::new(' '));
        current_x += 1;

        // Render label
        if let Some(option) = self.options.get(index) {
            for ch in option.chars() {
                if current_x >= area.x + area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(label_fg);
                if is_selected && self.focused && !self.disabled {
                    cell.modifier = crate::render::Modifier::BOLD;
                }
                ctx.buffer.set(current_x, y, cell);
                current_x += 1;
            }
        }

        current_x - x
    }
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self::new(Vec::<String>::new())
    }
}

impl View for RadioGroup {
    crate::impl_view_meta!("RadioGroup");

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        if area.width == 0 || area.height == 0 || self.options.is_empty() {
            return;
        }

        // Render focus indicator for the group
        let start_x = if self.focused && !self.disabled {
            let mut arrow = Cell::new('>');
            arrow.fg = Some(Color::CYAN);
            ctx.buffer.set(area.x, area.y, arrow);
            area.x + 2
        } else {
            area.x
        };

        match self.layout {
            RadioLayout::Vertical => {
                let mut y = area.y;
                for (i, _) in self.options.iter().enumerate() {
                    if y >= area.y + area.height {
                        break;
                    }
                    self.render_option(ctx, i, start_x, y);
                    y += 1 + self.gap;
                }
            }
            RadioLayout::Horizontal => {
                let mut x = start_x;
                for (i, _option) in self.options.iter().enumerate() {
                    if x >= area.x + area.width {
                        break;
                    }
                    let width = self.render_option(ctx, i, x, area.y);
                    x += width + 2 + self.gap; // 2 for spacing between options
                }
            }
        }
    }
}

impl_styled_view!(RadioGroup);
impl_props_builders!(RadioGroup);

/// Create a radio group
pub fn radio_group<I, S>(options: I) -> RadioGroup
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    RadioGroup::new(options)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    #[test]
    fn test_radio_group_new() {
        let rg = RadioGroup::new(vec!["Option 1", "Option 2", "Option 3"]);
        assert_eq!(rg.options.len(), 3);
        assert_eq!(rg.selected, 0);
        assert!(!rg.focused);
        assert!(!rg.disabled);
    }

    #[test]
    fn test_radio_group_builder() {
        let rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(1)
            .focused(true)
            .disabled(false)
            .style(RadioStyle::Unicode)
            .layout(RadioLayout::Horizontal)
            .gap(2);

        assert_eq!(rg.selected, 1);
        assert!(rg.focused);
        assert!(!rg.disabled);
        assert_eq!(rg.style, RadioStyle::Unicode);
        assert_eq!(rg.layout, RadioLayout::Horizontal);
        assert_eq!(rg.gap, 2);
    }

    #[test]
    fn test_radio_group_selection() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

        assert_eq!(rg.selected_index(), 0);
        assert_eq!(rg.selected_value(), Some("A"));

        rg.select_next();
        assert_eq!(rg.selected_index(), 1);
        assert_eq!(rg.selected_value(), Some("B"));

        rg.select_next();
        assert_eq!(rg.selected_index(), 2);

        rg.select_next();
        assert_eq!(rg.selected_index(), 0); // Wraps around

        rg.select_prev();
        assert_eq!(rg.selected_index(), 2); // Wraps around
    }

    #[test]
    fn test_radio_group_disabled_selection() {
        let mut rg = RadioGroup::new(vec!["A", "B"]).disabled(true);

        rg.select_next();
        assert_eq!(rg.selected_index(), 0); // Should not change
    }

    #[test]
    fn test_radio_group_handle_key() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);

        assert!(rg.handle_key(&Key::Down));
        assert_eq!(rg.selected_index(), 1);

        assert!(rg.handle_key(&Key::Up));
        assert_eq!(rg.selected_index(), 0);

        assert!(rg.handle_key(&Key::Char('j')));
        assert_eq!(rg.selected_index(), 1);

        assert!(rg.handle_key(&Key::Char('k')));
        assert_eq!(rg.selected_index(), 0);

        // Number keys
        assert!(rg.handle_key(&Key::Char('2')));
        assert_eq!(rg.selected_index(), 1);

        assert!(!rg.handle_key(&Key::Char('a'))); // Invalid key
    }

    #[test]
    fn test_radio_group_horizontal_keys() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).layout(RadioLayout::Horizontal);

        assert!(rg.handle_key(&Key::Right));
        assert_eq!(rg.selected_index(), 1);

        assert!(rg.handle_key(&Key::Left));
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_styles() {
        assert_eq!(RadioStyle::Parentheses.chars(), ('●', ' '));
        assert_eq!(RadioStyle::Unicode.chars(), ('◉', '○'));
        assert_eq!(RadioStyle::Brackets.chars(), ('*', ' '));
        assert_eq!(RadioStyle::Diamond.chars(), ('◆', '◇'));
    }

    #[test]
    fn test_radio_group_render() {
        let rg = RadioGroup::new(vec!["Option A", "Option B"]).selected(0);
        let mut buffer = Buffer::new(30, 5);
        let area = Rect::new(1, 1, 25, 3);
        let mut ctx = RenderContext::new(&mut buffer, area);

        rg.render(&mut ctx);
        // Should render radio options
    }

    #[test]
    fn test_radio_group_helper() {
        let rg = radio_group(vec!["X", "Y"]);
        assert_eq!(rg.options.len(), 2);
    }

    #[test]
    fn test_radio_group_empty() {
        let rg = RadioGroup::new(Vec::<String>::new());
        assert_eq!(rg.selected_value(), None);
    }

    #[test]
    fn test_radio_group_custom_colors() {
        let rg = RadioGroup::new(vec!["A", "B"])
            .fg(Color::WHITE)
            .selected_fg(Color::GREEN);

        assert_eq!(rg.fg, Some(Color::WHITE));
        assert_eq!(rg.selected_fg, Some(Color::GREEN));
    }
}
