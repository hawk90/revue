//! Justified text: the words spread out so the row ends at both edges

use super::Text;
use crate::render::Cell;
use crate::utils::unicode::display_width;
use crate::widget::traits::RenderContext;

/// The words of `content`, each with the spaces that follow it, spread
/// across `width` columns.
///
/// `None` means justify falls back to left alignment: there are fewer than
/// two words, or the words alone fill the width.
pub(super) fn justify_words(content: &str, width: u16) -> Option<Vec<(&str, usize)>> {
    let words: Vec<&str> = content.split_whitespace().collect();

    // If no words or single word, fall back to left alignment
    if words.len() <= 1 {
        return None;
    }

    // Calculate total text width (without spaces)
    let text_width: usize = words.iter().map(|w| display_width(w)).sum();
    let available_width = width as usize;

    // If text is too wide, fall back to left alignment
    if text_width >= available_width {
        return None;
    }

    // Calculate space distribution
    let total_space = available_width - text_width;
    let gap_count = words.len() - 1;
    let base_space = total_space / gap_count;
    let extra_spaces = total_space % gap_count;

    // Spacing after each word (none after the last)
    Some(
        words
            .into_iter()
            .enumerate()
            .map(|(i, word)| {
                let spaces = if i < gap_count {
                    base_space + if i < extra_spaces { 1 } else { 0 }
                } else {
                    0
                };
                (word, spaces)
            })
            .collect(),
    )
}

impl Text {
    /// Draw one line on row `y`, its words spread across the area. Returns
    /// false, drawing nothing, when the line falls back to the left.
    pub(super) fn render_justified_line(
        &self,
        ctx: &mut RenderContext,
        y: u16,
        line: &str,
    ) -> bool {
        let area = ctx.area;
        let Some(words) = justify_words(line, area.width) else {
            return false;
        };

        // Same CSS reading as `to_rich_text_with_ctx` - this path assembles
        // its own cells, so a rule that reaches justified text has to be
        // honored twice.
        let modifier = self.attributes_with_ctx(ctx).to_modifier();
        let fg = self.fg.or_else(|| ctx.css_color_if_set());
        let bg = self.bg.or_else(|| ctx.css_background_if_set());

        let hyperlink_id = self
            .hyperlink
            .as_ref()
            .map(|url| ctx.buffer.register_hyperlink(url));

        let styled = |ch: char| {
            let mut cell = Cell::new(ch);
            cell.fg = fg;
            cell.bg = bg;
            cell.modifier = modifier;
            cell.hyperlink_id = hyperlink_id;
            cell
        };

        // The gaps are spaces of the text, in its style, as they are when it
        // is aligned any other way.
        let mut x: u16 = 0;
        for (word, spaces) in words {
            x = x.saturating_add(ctx.put_str_with(x, y, word, area.width, styled));
            for _ in 0..spaces {
                ctx.set(x, y, styled(' '));
                x = x.saturating_add(1);
            }
        }
        true
    }
}
