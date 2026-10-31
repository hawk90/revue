//! Builders, the widget-state setters and getters, and CSS classes

use crate::event::drag::DragData;
use crate::style::Color;

use super::{DropZone, DropZoneStyle};

impl<F> DropZone<F>
where
    F: FnMut(DragData) -> bool,
{
    /// Set accepted data types
    pub fn accepts(mut self, types: &[&'static str]) -> Self {
        self.accepts = types.to_vec();
        self
    }

    /// Accept all data types
    pub fn accepts_all(mut self) -> Self {
        self.accepts.clear();
        self
    }

    /// Set visual style
    pub fn style(mut self, style: DropZoneStyle) -> Self {
        self.style = style;
        self
    }

    /// Set border color
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Set hover color
    ///
    /// Used for the border, the minimal indicator and the hint text while a
    /// drag the zone would accept hovers over it. A drag that would be
    /// rejected keeps the reject color, so the verdict still reads.
    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }

    /// The fewest rows to ask the layout for, and the zone still fills any more
    /// there is, like CSS `min-height`. A content-sized stack makes room by
    /// shrinking its measured children down to a row each, but does not push a
    /// sibling off: with too few rows the zone draws into what is left.
    pub fn min_height(mut self, height: u16) -> Self {
        self.min_height = height;
        self
    }

    /// Set drop handler
    pub fn on_drop<G>(self, handler: G) -> DropZone<G>
    where
        G: FnMut(DragData) -> bool,
    {
        DropZone {
            id: self.id,
            placeholder: self.placeholder,
            accepts: self.accepts,
            style: self.style,
            on_drop: Some(handler),
            hovered: self.hovered,
            can_accept_current: self.can_accept_current,
            border_color: self.border_color,
            hover_color: self.hover_color,
            reject_color: self.reject_color,
            state: self.state,
            props: self.props,
            min_height: self.min_height,
        }
    }
}

// Builder methods (manually implemented due to generic type parameter)
impl DropZone<fn(DragData) -> bool> {
    /// Set the focused state
    pub fn focused(mut self, focused: bool) -> Self {
        self.state.focused = focused;
        self
    }

    /// Set the disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.state.disabled = disabled;
        self
    }

    /// Set the foreground color
    pub fn fg(mut self, color: Color) -> Self {
        self.state.fg = Some(color);
        self
    }

    /// Set the background color
    pub fn bg(mut self, color: Color) -> Self {
        self.state.bg = Some(color);
        self
    }

    /// Check if the widget is focused
    pub fn is_focused(&self) -> bool {
        self.state.focused
    }

    /// Check if the widget is disabled
    pub fn is_disabled(&self) -> bool {
        self.state.disabled
    }

    /// Set the focused state (mutable)
    pub fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    /// Get hovered state for testing
    #[doc(hidden)]
    pub fn is_hovered(&self) -> bool {
        self.hovered
    }

    /// Get can_accept_current state for testing
    #[doc(hidden)]
    pub fn can_accept_current(&self) -> bool {
        self.can_accept_current
    }

    /// Get min_height for testing
    #[doc(hidden)]
    pub fn get_min_height(&self) -> u16 {
        self.min_height
    }
}

// StyledView trait for CSS class management
impl crate::widget::traits::StyledView for DropZone<fn(DragData) -> bool> {
    fn set_id(&mut self, id: impl Into<String>) {
        self.props.id = Some(id.into());
    }

    fn add_class(&mut self, class: impl Into<String>) {
        let class_str = class.into();
        if !self.props.classes.contains(&class_str) {
            self.props.classes.push(class_str);
        }
    }

    fn remove_class(&mut self, class: &str) {
        self.props.classes.retain(|c| c != class);
    }

    fn toggle_class(&mut self, class: &str) {
        if self.props.classes.contains(&class.to_string()) {
            self.remove_class(class);
        } else {
            self.add_class(class);
        }
    }

    fn has_class(&self, class: &str) -> bool {
        self.props.classes.contains(&class.to_string())
    }
}
