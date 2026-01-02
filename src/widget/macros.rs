//! Declarative UI macros

/// Create a vertical stack with children
///
/// # Examples
///
/// ```ignore
/// use revue::prelude::*;
///
/// let view = vstack![
///     Text::heading("Title"),
///     Text::new("Content"),
/// ];
///
/// // With gap
/// let view = vstack![gap: 2;
///     Text::new("Line 1"),
///     Text::new("Line 2"),
/// ];
/// ```
#[macro_export]
macro_rules! vstack {
    // With gap
    (gap: $gap:expr; $($child:expr),* $(,)?) => {{
        $crate::widget::vstack()
            .gap($gap)
            $(.child($child))*
    }};
    // Without gap
    ($($child:expr),* $(,)?) => {{
        $crate::widget::vstack()
            $(.child($child))*
    }};
}

/// Create a horizontal stack with children
///
/// # Examples
///
/// ```ignore
/// use revue::prelude::*;
///
/// let view = hstack![
///     Text::new("Left"),
///     Text::new("Right"),
/// ];
///
/// // With gap
/// let view = hstack![gap: 1;
///     Text::new("A"),
///     Text::new("B"),
/// ];
/// ```
#[macro_export]
macro_rules! hstack {
    // With gap
    (gap: $gap:expr; $($child:expr),* $(,)?) => {{
        $crate::widget::hstack()
            .gap($gap)
            $(.child($child))*
    }};
    // Without gap
    ($($child:expr),* $(,)?) => {{
        $crate::widget::hstack()
            $(.child($child))*
    }};
}

/// Create a border with a child
///
/// # Examples
///
/// ```ignore
/// use revue::prelude::*;
///
/// let view = bordered![
///     Text::new("Content")
/// ];
///
/// // With title
/// let view = bordered!["My Panel";
///     Text::new("Content")
/// ];
///
/// // With type and title
/// let view = bordered![rounded, "Card Title";
///     Text::new("Card content")
/// ];
/// ```
#[macro_export]
macro_rules! bordered {
    // Border type + title + child
    ($border_type:ident, $title:expr; $child:expr) => {{
        $crate::widget::Border::$border_type()
            .title($title)
            .child($child)
    }};
    // Title + child
    ($title:expr; $child:expr) => {{
        $crate::widget::Border::single().title($title).child($child)
    }};
    // Just child
    ($child:expr) => {{
        $crate::widget::Border::single().child($child)
    }};
}

/// Create text with common styling
///
/// # Examples
///
/// ```ignore
/// use revue::prelude::*;
///
/// let t = text!("Hello");
/// let t = text!("Error!", red);
/// let t = text!("Success", green, bold);
/// ```
#[macro_export]
macro_rules! text {
    // Text with color and modifiers
    ($content:expr, $color:ident, bold) => {{
        $crate::widget::Text::new($content)
            .fg($crate::style::Color::$color)
            .bold()
    }};
    ($content:expr, $color:ident, italic) => {{
        $crate::widget::Text::new($content)
            .fg($crate::style::Color::$color)
            .italic()
    }};
    // Text with color
    ($content:expr, red) => {{
        $crate::widget::Text::error($content)
    }};
    ($content:expr, green) => {{
        $crate::widget::Text::success($content)
    }};
    ($content:expr, yellow) => {{
        $crate::widget::Text::warning($content)
    }};
    ($content:expr, cyan) => {{
        $crate::widget::Text::info($content)
    }};
    ($content:expr, $color:ident) => {{
        $crate::widget::Text::new($content).fg($crate::style::Color::$color)
    }};
    // Plain text
    ($content:expr) => {{
        $crate::widget::Text::new($content)
    }};
}

/// Build a complete UI layout declaratively
///
/// # Examples
///
/// ```ignore
/// use revue::prelude::*;
///
/// let ui = ui! {
///     vstack(gap: 1) {
///         Text::heading("Dashboard")
///         hstack {
///             bordered!["Stats"; Text::new("100")]
///             bordered!["Users"; Text::new("42")]
///         }
///         Text::muted("Press 'q' to quit")
///     }
/// };
/// ```
#[macro_export]
macro_rules! ui {
    // VStack with options
    (vstack(gap: $gap:expr) { $($child:tt)* }) => {{
        $crate::widget::vstack()
            .gap($gap)
            $(.child($crate::ui!(@child $child)))*
    }};
    // VStack without options
    (vstack { $($child:tt)* }) => {{
        $crate::widget::vstack()
            $(.child($crate::ui!(@child $child)))*
    }};
    // HStack with options
    (hstack(gap: $gap:expr) { $($child:tt)* }) => {{
        $crate::widget::hstack()
            .gap($gap)
            $(.child($crate::ui!(@child $child)))*
    }};
    // HStack without options
    (hstack { $($child:tt)* }) => {{
        $crate::widget::hstack()
            $(.child($crate::ui!(@child $child)))*
    }};
    // Child processing - recursive ui! call
    (@child vstack $($rest:tt)*) => {{
        $crate::ui!(vstack $($rest)*)
    }};
    (@child hstack $($rest:tt)*) => {{
        $crate::ui!(hstack $($rest)*)
    }};
    // Child processing - direct expression
    (@child $expr:expr) => {{
        $expr
    }};
}

#[cfg(test)]
mod tests {
    use crate::layout::Rect;
    use crate::render::Buffer;
    use crate::widget::{RenderContext, Text, View};

    #[test]
    fn test_vstack_macro() {
        let stack = vstack![Text::new("Line 1"), Text::new("Line 2"),];
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_vstack_macro_with_gap() {
        let stack = vstack![gap: 2;
            Text::new("A"),
            Text::new("B"),
        ];
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_hstack_macro() {
        let stack = hstack![Text::new("Left"), Text::new("Right"),];
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_text_macro() {
        let t = text!("Hello");
        assert_eq!(t.content(), "Hello");

        let t = text!("Error", red);
        assert_eq!(t.content(), "Error");
    }

    #[test]
    fn test_bordered_macro() {
        let b = bordered![Text::new("Content")];
        // Just verify it compiles and creates a border
        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        b.render(&mut ctx);
    }

    #[test]
    fn test_bordered_macro_with_title() {
        let b = bordered!["Title"; Text::new("Content")];
        let mut buffer = Buffer::new(20, 5);
        let area = Rect::new(0, 0, 20, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);
        b.render(&mut ctx);
    }

    #[test]
    fn test_nested_layout() {
        let layout = vstack![
            Text::heading("Title"),
            hstack![Text::new("Left"), Text::new("Right"),],
            Text::muted("Footer"),
        ];
        assert_eq!(layout.len(), 3);
    }
}
