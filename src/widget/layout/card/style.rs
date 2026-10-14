//! The colors and border to paint: builder, stylesheet, variant and focus

use super::super::border::BorderType;
use super::types::CardVariant;
use super::Card;
use crate::style::Color;
use crate::widget::theme::DARK_BG;
use crate::widget::traits::RenderContext;

impl Card {
    /// [`effective_colors`](Self::effective_colors), with the stylesheet
    /// filling in whatever the builder left unset.
    ///
    /// The builder is the inline style and wins; a variant's defaults are the
    /// bottom row, below the stylesheet. Same precedence as every other paint
    /// property in the crate.
    pub(super) fn effective_colors_with_css(
        &self,
        ctx: &RenderContext,
    ) -> (Option<Color>, Color, Color) {
        let (bg, border, title) = self.effective_colors();

        let css_bg = ctx
            .style
            .map(|s| s.visual.background)
            .filter(|c| *c != Color::default());
        let bg = self.bg_color.or(css_bg).or(bg);

        let border = match (self.border_color, ctx.css_border_or_text_color()) {
            (Some(explicit), _) => explicit,
            (None, Some(css)) => css,
            (None, None) => border,
        };

        (bg, border, title)
    }

    /// The border to draw: the stylesheet's if it named one, else the builder's.
    ///
    /// See `Border::border_type_with_css` - a stylesheet that said nothing
    /// leaves the builder's choice alone, and `border-style: none` removes the
    /// border.
    pub(super) fn border_type_with_css(&self, ctx: &RenderContext) -> BorderType {
        match ctx.css_border_style() {
            None => self.border,
            Some(crate::style::BorderStyle::None) => BorderType::None,
            Some(crate::style::BorderStyle::Solid) | Some(crate::style::BorderStyle::Dashed) => {
                BorderType::Single
            }
            Some(crate::style::BorderStyle::Double) => BorderType::Double,
            Some(crate::style::BorderStyle::Rounded) => BorderType::Rounded,
        }
    }

    /// Get effective colors based on variant and state
    fn effective_colors(&self) -> (Option<Color>, Color, Color) {
        let default_bg: Option<Color> = match self.variant {
            CardVariant::Outlined => None,
            CardVariant::Filled => Some(Color::rgb(30, 30, 35)),
            CardVariant::Elevated => Some(Color::rgb(35, 35, 40)),
            CardVariant::Flat => None,
        };

        let default_border = match self.variant {
            CardVariant::Outlined => Color::rgb(60, 60, 70),
            CardVariant::Filled => Color::rgb(50, 50, 60),
            CardVariant::Elevated => Color::rgb(70, 70, 80),
            CardVariant::Flat => DARK_BG,
        };

        let default_title = Color::WHITE;

        let bg = self.bg_color.or(default_bg);
        let border = self.border_color.unwrap_or(default_border);
        let title = self.title_color.unwrap_or(default_title);

        // Adjust for focus state
        if self.state.focused && self.clickable {
            (bg, Color::CYAN, title)
        } else {
            (bg, border, title)
        }
    }
}
