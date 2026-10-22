//! Drawing the option list: title, options, separators, groups and scroll markers

use super::{OptionEntry, OptionList};
use crate::style::Color;
use crate::utils::display_width;
use crate::widget::theme::{DARK_GRAY, MUTED_TEXT, PLACEHOLDER_FG};
use crate::widget::traits::DISABLED_FG;
use crate::widget::{RenderContext, View};

impl View for OptionList {
    crate::impl_view_meta!("OptionList");

    fn render(&self, ctx: &mut RenderContext) {
        use crate::widget::stack::vstack;
        use crate::widget::Text;

        let width = self.width.unwrap_or(40) as usize;

        let mut content = vstack();

        // Title
        if let Some(title) = &self.title {
            content = content.child(Text::new(title).bold());
        }

        // Every entry's rows (an option with a description takes two), and
        // the rows of each option
        let mut rows = Vec::new();
        let mut option_rows = Vec::new();
        let mut option_idx = 0;

        for entry in &self.entries {
            match entry {
                OptionEntry::Option(item) => {
                    let first_row = rows.len();
                    let is_highlighted = option_idx == self.highlighted && self.focused;
                    let is_selected = self.selected == Some(option_idx);

                    // Build option text
                    let icon = if self.show_icons {
                        item.icon.as_deref().unwrap_or("")
                    } else {
                        ""
                    };

                    let prefix = if is_selected {
                        "▸ "
                    } else if is_highlighted {
                        "> "
                    } else {
                        "  "
                    };

                    let main_text = format!("{}{}{}", prefix, icon, item.text);

                    // Calculate padding for hint
                    let hint = item.hint.as_deref().unwrap_or("");
                    let padding =
                        width.saturating_sub(display_width(&main_text) + display_width(hint));

                    // Determine colors
                    let fg = if item.disabled {
                        self.disabled_fg.unwrap_or(DISABLED_FG)
                    } else if is_highlighted {
                        self.highlighted_fg.unwrap_or(Color::CYAN)
                    } else if is_selected {
                        self.selected_fg.unwrap_or(Color::GREEN)
                    } else {
                        self.fg.unwrap_or_else(|| ctx.css_color(Color::WHITE))
                    };

                    let bg = if is_highlighted {
                        self.highlighted_bg
                    } else {
                        self.bg
                    };

                    // Build row
                    let mut text =
                        Text::new(format!("{}{}{}", main_text, " ".repeat(padding), hint)).fg(fg);

                    if let Some(bg) = bg {
                        text = text.bg(bg);
                    }

                    if is_highlighted || is_selected {
                        text = text.bold();
                    }

                    rows.push(text);

                    // Show description
                    if self.show_descriptions {
                        if let Some(desc) = &item.description {
                            rows.push(Text::new(format!("    {}", desc)).fg(PLACEHOLDER_FG));
                        }
                    }

                    option_rows.push(first_row..rows.len());
                    option_idx += 1;
                }
                OptionEntry::Separator => {
                    let line = self.separator_char().repeat(width);
                    rows.push(Text::new(line).fg(DARK_GRAY));
                }
                OptionEntry::Group(name) => {
                    rows.push(Text::new(name).fg(MUTED_TEXT).bold());
                }
            }
        }

        // The rows scroll to keep the highlighted option in view (#973). The
        // window is the rows the area has left, or the most rows any
        // `max_visible` options in a row take with the entries between them;
        // when the rows overflow it, a row each is kept for the markers.
        let room = (ctx.area.height as usize).saturating_sub(usize::from(self.title.is_some()));
        let cap = if self.max_visible > 0 && option_rows.len() > self.max_visible {
            option_rows
                .windows(self.max_visible)
                .map(|run| run[run.len() - 1].end - run[0].start)
                .max()
                .unwrap_or(usize::MAX)
        } else {
            usize::MAX
        };
        let window = if rows.len() <= room.min(cap) {
            rows.len()
        } else {
            cap.min(room.saturating_sub(2)).max(1)
        };
        self.scroll.set_content(rows.len());
        self.scroll.set_viewport(window);
        if let Some(highlight) = option_rows.get(self.highlighted) {
            self.scroll.track(highlight.clone());
        }

        if self.scroll.has_above() {
            content = content.child(Text::new("↑").fg(DISABLED_FG));
        }
        let shown = self.scroll.visible();
        let below = rows.len() - shown.end;
        for row in rows.into_iter().skip(shown.start).take(shown.len()) {
            content = content.child(row);
        }
        if below > 0 {
            content = content.child(Text::new("↓").fg(DISABLED_FG));
        }

        content.render(ctx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::Rect;
    use crate::render::Buffer;

    fn rows(list: &OptionList) -> Vec<String> {
        let (w, h) = (12, 8);
        let mut buf = Buffer::new(w, h);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, w, h));
        list.render(&mut ctx);
        (0..h)
            .map(|y| {
                (0..w)
                    .map(|x| buf.get(x, y).unwrap().symbol)
                    .collect::<String>()
                    .trim_end()
                    .to_string()
            })
            .filter(|r| !r.is_empty())
            .collect()
    }

    /// `max_visible(2)` is the most rows two options in a row take with the
    /// entries between them: three here, so any two adjacent options fit
    #[test]
    fn the_window_holds_max_visible_options_with_the_entries_between() {
        let mut list = OptionList::new()
            .width(10)
            .max_visible(2)
            .group("G1")
            .option("a", "")
            .separator()
            .option("b", "")
            .group("G2")
            .option("c", "")
            .option("d", "")
            .group("G3")
            .option("e", "");
        for _ in 0..3 {
            list.highlight_next(); // d
        }
        assert_eq!(rows(&list), ["↑", "G2", "  c", "  d", "↓"]);
        list.highlight_previous(); // c, in view
        assert_eq!(rows(&list), ["↑", "G2", "  c", "  d", "↓"]);
        list.highlight_previous(); // b, above
        assert_eq!(rows(&list), ["↑", "  b", "G2", "  c", "↓"]);
    }

    #[test]
    fn hint_aligns_to_the_right_edge_by_display_width() {
        let list = OptionList::new()
            .width(12)
            .option("한글", "K")
            .option("ab", "K");
        let mut buf = Buffer::new(12, 2);
        let mut ctx = RenderContext::new(&mut buf, Rect::new(0, 0, 12, 2));
        list.render(&mut ctx);
        for y in 0..2 {
            assert_eq!(buf.get(11, y).unwrap().symbol, 'K', "row {y}");
        }
    }
}
