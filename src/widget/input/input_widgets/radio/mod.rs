//! Radio button widget for single selection from options

mod render;

use crate::event::Key;
use crate::style::Color;
use crate::utils::Selection;
use crate::widget::traits::WidgetProps;
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
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self::new(Vec::<String>::new())
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
