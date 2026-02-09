//! Radio button widget for single selection from options

use crate::event::Key;
use crate::render::Cell;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::{RenderContext, View, WidgetProps};
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
    selection: Selection,
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
        let opts: Vec<String> = options.into_iter().map(|s| s.into()).collect();
        let len = opts.len();
        Self {
            options: opts,
            selection: Selection::new(len),
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
        self.selection.set(index);
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
        self.selection.index
    }

    /// Get selected option value
    pub fn selected_value(&self) -> Option<&str> {
        self.options.get(self.selection.index).map(|s| s.as_str())
    }

    /// Check if focused
    pub fn is_focused(&self) -> bool {
        self.focused
    }

    /// Check if disabled
    pub fn is_disabled(&self) -> bool {
        self.disabled
    }

    /// Select next option (wraps around)
    pub fn select_next(&mut self) {
        if !self.disabled {
            self.selection.next();
        }
    }

    /// Select previous option (wraps around)
    pub fn select_prev(&mut self) {
        if !self.disabled {
            self.selection.prev();
        }
    }

    /// Set focus state (mutable)
    pub fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }

    /// Set selected index (mutable)
    pub fn set_selected(&mut self, index: usize) {
        self.selection.set(index);
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
                // Safe: c is '0'..='9' after is_ascii_digit() check
                let index = (*c as u8 - b'0') as usize;
                if index > 0 && index <= self.options.len() {
                    self.selection.set(index - 1);
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

        let is_selected = self.selection.is_selected(index);
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

// Most tests moved to tests/widget_tests.rs
// Tests below access private fields and must stay inline

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // RadioGroup Constructor Tests
    // =========================================================================

    #[test]
    fn test_radio_group_new() {
        let rg = RadioGroup::new(vec!["Option 1", "Option 2", "Option 3"]);
        assert_eq!(rg.options.len(), 3);
        assert_eq!(rg.selected_index(), 0);
        assert!(!rg.focused);
        assert!(!rg.disabled);
    }

    #[test]
    fn test_radio_group_new_empty() {
        let rg = RadioGroup::new(vec![] as Vec<&str>);
        assert_eq!(rg.options.len(), 0);
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_new_single_option() {
        let rg = RadioGroup::new(vec!["Only Option"]);
        assert_eq!(rg.options.len(), 1);
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_new_with_strings() {
        let rg = RadioGroup::new(vec![
            String::from("A"),
            String::from("B"),
            String::from("C"),
        ]);
        assert_eq!(rg.options.len(), 3);
        assert_eq!(rg.options[0], "A");
    }

    #[test]
    fn test_radio_group_new_with_slice() {
        let options = ["X", "Y", "Z"];
        let rg = RadioGroup::new(options);
        assert_eq!(rg.options.len(), 3);
    }

    #[test]
    fn test_radio_group_default() {
        let rg = RadioGroup::default();
        assert_eq!(rg.options.len(), 0);
        assert_eq!(rg.selected_index(), 0);
    }

    // =========================================================================
    // RadioGroup Builder Tests
    // =========================================================================

    #[test]
    fn test_radio_group_builder() {
        let rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(1)
            .focused(true)
            .disabled(false)
            .style(RadioStyle::Unicode)
            .layout(RadioLayout::Horizontal)
            .gap(2);

        assert_eq!(rg.selected_index(), 1);
        assert!(rg.focused);
        assert!(!rg.disabled);
        assert_eq!(rg.style, RadioStyle::Unicode);
        assert_eq!(rg.layout, RadioLayout::Horizontal);
        assert_eq!(rg.gap, 2);
    }

    #[test]
    fn test_radio_group_selected() {
        let rg = RadioGroup::new(vec!["A", "B", "C"]).selected(2);
        assert_eq!(rg.selected_index(), 2);
    }

    #[test]
    fn test_radio_group_focused_true() {
        let rg = RadioGroup::new(vec!["A", "B"]).focused(true);
        assert!(rg.is_focused());
    }

    #[test]
    fn test_radio_group_focused_false() {
        let rg = RadioGroup::new(vec!["A", "B"]).focused(false);
        assert!(!rg.is_focused());
    }

    #[test]
    fn test_radio_group_disabled_true() {
        let rg = RadioGroup::new(vec!["A", "B"]).disabled(true);
        assert!(rg.is_disabled());
    }

    #[test]
    fn test_radio_group_disabled_false() {
        let rg = RadioGroup::new(vec!["A", "B"]).disabled(false);
        assert!(!rg.is_disabled());
    }

    #[test]
    fn test_radio_group_style_parentheses() {
        let rg = RadioGroup::new(vec!["A"]).style(RadioStyle::Parentheses);
        assert_eq!(rg.style, RadioStyle::Parentheses);
    }

    #[test]
    fn test_radio_group_style_unicode() {
        let rg = RadioGroup::new(vec!["A"]).style(RadioStyle::Unicode);
        assert_eq!(rg.style, RadioStyle::Unicode);
    }

    #[test]
    fn test_radio_group_style_brackets() {
        let rg = RadioGroup::new(vec!["A"]).style(RadioStyle::Brackets);
        assert_eq!(rg.style, RadioStyle::Brackets);
    }

    #[test]
    fn test_radio_group_style_diamond() {
        let rg = RadioGroup::new(vec!["A"]).style(RadioStyle::Diamond);
        assert_eq!(rg.style, RadioStyle::Diamond);
    }

    #[test]
    fn test_radio_group_layout_vertical() {
        let rg = RadioGroup::new(vec!["A"]).layout(RadioLayout::Vertical);
        assert_eq!(rg.layout, RadioLayout::Vertical);
    }

    #[test]
    fn test_radio_group_layout_horizontal() {
        let rg = RadioGroup::new(vec!["A"]).layout(RadioLayout::Horizontal);
        assert_eq!(rg.layout, RadioLayout::Horizontal);
    }

    #[test]
    fn test_radio_group_gap() {
        let rg = RadioGroup::new(vec!["A", "B"]).gap(3);
        assert_eq!(rg.gap, 3);
    }

    #[test]
    fn test_radio_group_fg() {
        let rg = RadioGroup::new(vec!["A"]).fg(Color::WHITE);
        assert_eq!(rg.fg, Some(Color::WHITE));
    }

    #[test]
    fn test_radio_group_selected_fg() {
        let rg = RadioGroup::new(vec!["A"]).selected_fg(Color::GREEN);
        assert_eq!(rg.selected_fg, Some(Color::GREEN));
    }

    // =========================================================================
    // RadioGroup Getter Tests
    // =========================================================================

    #[test]
    fn test_radio_group_selected_index() {
        let rg = RadioGroup::new(vec!["A", "B", "C"]).selected(1);
        assert_eq!(rg.selected_index(), 1);
    }

    #[test]
    fn test_radio_group_selected_value_some() {
        let rg = RadioGroup::new(vec!["Apple", "Banana", "Cherry"]).selected(1);
        assert_eq!(rg.selected_value(), Some("Banana"));
    }

    #[test]
    fn test_radio_group_selected_value_none() {
        let rg = RadioGroup::new(vec!["A", "B"]);
        assert_eq!(rg.selected_value(), Some("A")); // First option selected by default
    }

    #[test]
    fn test_radio_group_selected_value_empty() {
        let rg = RadioGroup::new(vec![] as Vec<&str>);
        assert_eq!(rg.selected_value(), None);
    }

    #[test]
    fn test_radio_group_is_focused_true() {
        let rg = RadioGroup::new(vec!["A"]).focused(true);
        assert!(rg.is_focused());
    }

    #[test]
    fn test_radio_group_is_focused_false() {
        let rg = RadioGroup::new(vec!["A"]);
        assert!(!rg.is_focused());
    }

    #[test]
    fn test_radio_group_is_disabled_true() {
        let rg = RadioGroup::new(vec!["A"]).disabled(true);
        assert!(rg.is_disabled());
    }

    #[test]
    fn test_radio_group_is_disabled_false() {
        let rg = RadioGroup::new(vec!["A"]);
        assert!(!rg.is_disabled());
    }

    // =========================================================================
    // RadioGroup State Mutation Tests
    // =========================================================================

    #[test]
    fn test_radio_group_set_focused() {
        let mut rg = RadioGroup::new(vec!["A"]);
        assert!(!rg.is_focused());
        rg.set_focused(true);
        assert!(rg.is_focused());
        rg.set_focused(false);
        assert!(!rg.is_focused());
    }

    #[test]
    fn test_radio_group_set_selected() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
        assert_eq!(rg.selected_index(), 0);
        rg.set_selected(2);
        assert_eq!(rg.selected_index(), 2);
        rg.set_selected(1);
        assert_eq!(rg.selected_index(), 1);
    }

    #[test]
    fn test_radio_group_select_next() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
        assert_eq!(rg.selected_index(), 0);
        rg.select_next();
        assert_eq!(rg.selected_index(), 1);
        rg.select_next();
        assert_eq!(rg.selected_index(), 2);
    }

    #[test]
    fn test_radio_group_select_next_wraps() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).selected(2);
        rg.select_next();
        assert_eq!(rg.selected_index(), 0); // Wraps to beginning
    }

    #[test]
    fn test_radio_group_select_next_when_disabled() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(0)
            .disabled(true);
        rg.select_next();
        assert_eq!(rg.selected_index(), 0); // Should not change
    }

    #[test]
    fn test_radio_group_select_prev() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).selected(2);
        rg.select_prev();
        assert_eq!(rg.selected_index(), 1);
        rg.select_prev();
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_select_prev_wraps() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
        rg.select_prev();
        assert_eq!(rg.selected_index(), 2); // Wraps to end
    }

    #[test]
    fn test_radio_group_select_prev_when_disabled() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(2)
            .disabled(true);
        rg.select_prev();
        assert_eq!(rg.selected_index(), 2); // Should not change
    }

    #[test]
    fn test_radio_group_select_single_option() {
        let mut rg = RadioGroup::new(vec!["Only"]);
        rg.select_next();
        assert_eq!(rg.selected_index(), 0);
        rg.select_prev();
        assert_eq!(rg.selected_index(), 0);
    }

    // =========================================================================
    // RadioGroup Key Handling Tests
    // =========================================================================

    #[test]
    fn test_radio_group_handle_key_up() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).selected(1);
        let handled = rg.handle_key(&Key::Up);
        assert!(handled);
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_handle_key_down() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
        let handled = rg.handle_key(&Key::Down);
        assert!(handled);
        assert_eq!(rg.selected_index(), 1);
    }

    #[test]
    fn test_radio_group_handle_key_k() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).selected(1);
        let handled = rg.handle_key(&Key::Char('k'));
        assert!(handled);
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_handle_key_j() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
        let handled = rg.handle_key(&Key::Char('j'));
        assert!(handled);
        assert_eq!(rg.selected_index(), 1);
    }

    #[test]
    fn test_radio_group_handle_key_left_horizontal() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(1)
            .layout(RadioLayout::Horizontal);
        let handled = rg.handle_key(&Key::Left);
        assert!(handled);
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_handle_key_right_horizontal() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).layout(RadioLayout::Horizontal);
        let handled = rg.handle_key(&Key::Right);
        assert!(handled);
        assert_eq!(rg.selected_index(), 1);
    }

    #[test]
    fn test_radio_group_handle_key_left_vertical() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(1)
            .layout(RadioLayout::Vertical);
        let handled = rg.handle_key(&Key::Left);
        assert!(!handled); // Left key ignored in vertical layout
        assert_eq!(rg.selected_index(), 1);
    }

    #[test]
    fn test_radio_group_handle_key_right_vertical() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).layout(RadioLayout::Vertical);
        let handled = rg.handle_key(&Key::Right);
        assert!(!handled); // Right key ignored in vertical layout
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_handle_key_digit() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C", "D"]);
        let handled = rg.handle_key(&Key::Char('2'));
        assert!(handled);
        assert_eq!(rg.selected_index(), 1); // Index 1 for digit '2'
    }

    #[test]
    fn test_radio_group_handle_key_digit_one() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]);
        let handled = rg.handle_key(&Key::Char('1'));
        assert!(handled);
        assert_eq!(rg.selected_index(), 0); // Index 0 for digit '1'
    }

    #[test]
    fn test_radio_group_handle_key_digit_nine() {
        let mut rg = RadioGroup::new(vec!["A"; 10]);
        let handled = rg.handle_key(&Key::Char('9'));
        assert!(handled);
        assert_eq!(rg.selected_index(), 8); // Index 8 for digit '9'
    }

    #[test]
    fn test_radio_group_handle_key_digit_zero() {
        let mut rg = RadioGroup::new(vec!["A", "B"]);
        let handled = rg.handle_key(&Key::Char('0'));
        assert!(!handled); // '0' is not valid (1-9 only)
    }

    #[test]
    fn test_radio_group_handle_key_digit_out_of_range() {
        let mut rg = RadioGroup::new(vec!["A", "B"]);
        let handled = rg.handle_key(&Key::Char('9'));
        assert!(!handled); // 9 > len(2)
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_handle_key_other_char() {
        let mut rg = RadioGroup::new(vec!["A", "B"]);
        let handled = rg.handle_key(&Key::Char('x'));
        assert!(!handled);
    }

    #[test]
    fn test_radio_group_handle_key_when_disabled() {
        let mut rg = RadioGroup::new(vec!["A", "B", "C"]).disabled(true);
        let handled = rg.handle_key(&Key::Down);
        assert!(!handled);
        assert_eq!(rg.selected_index(), 0);
    }

    #[test]
    fn test_radio_group_handle_key_enter() {
        let mut rg = RadioGroup::new(vec!["A", "B"]);
        let handled = rg.handle_key(&Key::Enter);
        assert!(!handled); // Enter not handled
    }

    #[test]
    fn test_radio_group_handle_key_escape() {
        let mut rg = RadioGroup::new(vec!["A", "B"]);
        let handled = rg.handle_key(&Key::Escape);
        assert!(!handled); // Escape not handled
    }

    // =========================================================================
    // RadioStyle Tests
    // =========================================================================

    #[test]
    fn test_radio_styles() {
        assert_eq!(RadioStyle::Parentheses.chars(), ('●', ' '));
        assert_eq!(RadioStyle::Unicode.chars(), ('◉', '○'));
        assert_eq!(RadioStyle::Brackets.chars(), ('*', ' '));
        assert_eq!(RadioStyle::Diamond.chars(), ('◆', '◇'));
    }

    #[test]
    fn test_radio_style_default() {
        assert_eq!(RadioStyle::default(), RadioStyle::Parentheses);
    }

    #[test]
    fn test_radio_style_parentheses_brackets() {
        assert_eq!(RadioStyle::Parentheses.brackets(), ('(', ')'));
    }

    #[test]
    fn test_radio_style_brackets_brackets() {
        assert_eq!(RadioStyle::Brackets.brackets(), ('[', ']'));
    }

    #[test]
    fn test_radio_style_unicode_brackets() {
        assert_eq!(RadioStyle::Unicode.brackets(), (' ', ' '));
    }

    #[test]
    fn test_radio_style_diamond_brackets() {
        assert_eq!(RadioStyle::Diamond.brackets(), (' ', ' '));
    }

    #[test]
    fn test_radio_style_has_brackets_parentheses() {
        assert!(RadioStyle::Parentheses.has_brackets());
    }

    #[test]
    fn test_radio_style_has_brackets_brackets() {
        assert!(RadioStyle::Brackets.has_brackets());
    }

    #[test]
    fn test_radio_style_has_brackets_unicode() {
        assert!(!RadioStyle::Unicode.has_brackets());
    }

    #[test]
    fn test_radio_style_has_brackets_diamond() {
        assert!(!RadioStyle::Diamond.has_brackets());
    }

    #[test]
    fn test_radio_style_clone() {
        let style1 = RadioStyle::Unicode;
        let style2 = style1;
        assert_eq!(style1, RadioStyle::Unicode);
        assert_eq!(style2, RadioStyle::Unicode);
    }

    #[test]
    fn test_radio_style_copy() {
        let style = RadioStyle::Diamond;
        let copied = style;
        assert_eq!(style, copied);
    }

    #[test]
    fn test_radio_style_partial_eq() {
        assert_eq!(RadioStyle::Parentheses, RadioStyle::Parentheses);
        assert_ne!(RadioStyle::Parentheses, RadioStyle::Unicode);
    }

    #[test]
    fn test_radio_style_debug() {
        let debug_str = format!("{:?}", RadioStyle::Unicode);
        assert!(debug_str.contains("Unicode"));
    }

    // =========================================================================
    // RadioLayout Tests
    // =========================================================================

    #[test]
    fn test_radio_layout_default() {
        assert_eq!(RadioLayout::default(), RadioLayout::Vertical);
    }

    #[test]
    fn test_radio_layout_clone() {
        let layout1 = RadioLayout::Horizontal;
        let layout2 = layout1;
        assert_eq!(layout1, RadioLayout::Horizontal);
        assert_eq!(layout2, RadioLayout::Horizontal);
    }

    #[test]
    fn test_radio_layout_copy() {
        let layout = RadioLayout::Vertical;
        let copied = layout;
        assert_eq!(layout, copied);
    }

    #[test]
    fn test_radio_layout_partial_eq() {
        assert_eq!(RadioLayout::Vertical, RadioLayout::Vertical);
        assert_ne!(RadioLayout::Vertical, RadioLayout::Horizontal);
    }

    #[test]
    fn test_radio_layout_debug() {
        let debug_str = format!("{:?}", RadioLayout::Horizontal);
        assert!(debug_str.contains("Horizontal"));
    }

    // =========================================================================
    // RadioGroup Clone Tests
    // =========================================================================

    #[test]
    fn test_radio_group_clone() {
        let rg1 = RadioGroup::new(vec!["A", "B", "C"])
            .selected(1)
            .focused(true)
            .style(RadioStyle::Unicode)
            .fg(Color::WHITE);
        let rg2 = rg1.clone();

        assert_eq!(rg1.options.len(), rg2.options.len());
        assert_eq!(rg1.selected_index(), rg2.selected_index());
        assert_eq!(rg1.is_focused(), rg2.is_focused());
        assert_eq!(rg1.style, rg2.style);
        assert_eq!(rg1.fg, rg2.fg);
    }

    // =========================================================================
    // Helper Function Tests
    // =========================================================================

    #[test]
    fn test_radio_group_helper() {
        let rg = radio_group(vec!["X", "Y"]);
        assert_eq!(rg.options.len(), 2);
    }

    #[test]
    fn test_radio_group_helper_with_strings() {
        let rg = radio_group(vec![String::from("P"), String::from("Q")]);
        assert_eq!(rg.options.len(), 2);
        assert_eq!(rg.options[0], "P");
    }

    // =========================================================================
    // Edge Case Tests
    // =========================================================================

    #[test]
    fn test_radio_group_custom_colors() {
        let rg = RadioGroup::new(vec!["A", "B"])
            .fg(Color::WHITE)
            .selected_fg(Color::GREEN);

        assert_eq!(rg.fg, Some(Color::WHITE));
        assert_eq!(rg.selected_fg, Some(Color::GREEN));
    }

    #[test]
    fn test_radio_group_with_unicode_options() {
        let rg = RadioGroup::new(vec!["사과", "바나나", "체리"]);
        assert_eq!(rg.options.len(), 3);
        assert_eq!(rg.options[0], "사과");
    }

    #[test]
    fn test_radio_group_with_emojis() {
        let rg = RadioGroup::new(vec!["🍎 Apple", "🍌 Banana", "🍒 Cherry"]);
        assert_eq!(rg.options.len(), 3);
        assert_eq!(rg.options[0], "🍎 Apple");
    }

    #[test]
    fn test_radio_group_with_empty_strings() {
        let rg = RadioGroup::new(vec!["", "", ""]);
        assert_eq!(rg.options.len(), 3);
        assert_eq!(rg.options[0], "");
    }

    #[test]
    fn test_radio_group_builder_chain() {
        let rg = RadioGroup::new(vec!["A", "B", "C"])
            .selected(2)
            .focused(true)
            .disabled(false)
            .style(RadioStyle::Diamond)
            .layout(RadioLayout::Horizontal)
            .gap(1)
            .fg(Color::CYAN)
            .selected_fg(Color::YELLOW);

        assert_eq!(rg.selected_index(), 2);
        assert!(rg.is_focused());
        assert!(!rg.is_disabled());
        assert_eq!(rg.style, RadioStyle::Diamond);
        assert_eq!(rg.layout, RadioLayout::Horizontal);
        assert_eq!(rg.gap, 1);
        assert_eq!(rg.fg, Some(Color::CYAN));
        assert_eq!(rg.selected_fg, Some(Color::YELLOW));
    }
}
