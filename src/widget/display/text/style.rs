//! What the text paints with: the builder's settings over the stylesheet's

use super::{Alignment, Text};
use crate::widget::display::richtext::{RichText, Span, Style};
use crate::widget::traits::RenderContext;

impl Text {
    /// The attributes (bold, italic, ...) to paint with, without the colors.
    ///
    /// Both drawing paths read them here: `to_rich_text_with_ctx` and the
    /// justified path, which assembles its own cells.
    pub(super) fn attributes_with_ctx(&self, ctx: &RenderContext) -> Style {
        let mut style = Style::new();

        // These are booleans whose `false` means both "off" and "not
        // specified", so the builder can only turn them on and a stylesheet
        // fills in what it did not mention. Same reading `gap: 0` gets.
        if self.bold || ctx.css_bold() {
            style = style.bold();
        }
        if self.italic {
            style = style.italic();
        }
        if self.underline || ctx.css_underline() {
            style = style.underline();
        }
        if ctx.css_line_through() {
            style = style.strikethrough();
        }
        if self.dim {
            style = style.dim();
        }
        if self.reverse {
            style = style.reverse();
        }
        style
    }

    /// One line of the text as RichText, for rendering with CSS support
    pub(super) fn to_rich_text_with_ctx(&self, ctx: &RenderContext, line: &str) -> RichText {
        let mut style = self.attributes_with_ctx(ctx);

        // Get foreground color: inline > CSS > none
        let fg = self.fg.or_else(|| ctx.css_color_if_set());
        if let Some(fg) = fg {
            style = style.fg(fg);
        }

        // Get background color: inline > CSS > none
        let bg = self.bg.or_else(|| ctx.css_background_if_set());
        if let Some(bg) = bg {
            style = style.bg(bg);
        }

        let mut span = Span::styled(line.to_string(), style);
        if let Some(url) = &self.hyperlink {
            span = span.href(url.clone());
        }
        RichText::new().span(span)
    }

    /// The alignment to paint with: the builder's if it named one, else the
    /// stylesheet's, else left.
    ///
    /// This is the precedence `docs/guides/styling.md` records - builder,
    /// then author stylesheet, then the widget's own default - and it needs
    /// the builder's "said nothing" to be a distinct state, which is why the
    /// field is an `Option`.
    pub(super) fn align_with_css(&self, ctx: &RenderContext) -> Alignment {
        if let Some(builder) = self.align {
            return builder;
        }
        match ctx.css_text_align() {
            crate::style::TextAlign::Left => Alignment::Left,
            crate::style::TextAlign::Center => Alignment::Center,
            crate::style::TextAlign::Right => Alignment::Right,
        }
    }
}
