//! Drawing the option list: title, options, separators, groups and scroll markers

use super::{OptionEntry, OptionList};
use crate::style::Color;
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

        let mut option_idx = 0;
        let mut visible_count = 0;
        let mut skipped = 0;

        for entry in &self.entries {
            // Handle scrolling
            if let OptionEntry::Option(_) = entry {
                if option_idx < self.scroll_offset {
                    option_idx += 1;
                    skipped += 1;
                    continue;
                }
                if visible_count >= self.max_visible {
                    break;
                }
            }

            match entry {
                OptionEntry::Option(item) => {
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
                    let padding = width.saturating_sub(main_text.len() + hint.len());

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

                    content = content.child(text);

                    // Show description
                    if self.show_descriptions {
                        if let Some(desc) = &item.description {
                            content = content
                                .child(Text::new(format!("    {}", desc)).fg(PLACEHOLDER_FG));
                        }
                    }

                    option_idx += 1;
                    visible_count += 1;
                }
                OptionEntry::Separator => {
                    let line = self.separator_char().repeat(width);
                    content = content.child(Text::new(line).fg(DARK_GRAY));
                }
                OptionEntry::Group(name) => {
                    content = content.child(Text::new(name).fg(MUTED_TEXT).bold());
                }
            }
        }

        // Scroll indicators
        if skipped > 0 {
            content = vstack()
                .child(Text::new("↑").fg(DISABLED_FG))
                .child(content);
        }

        let remaining = self.option_count() - (self.scroll_offset + visible_count);
        if remaining > 0 {
            content = content.child(Text::new("↓").fg(DISABLED_FG));
        }

        content.render(ctx);
    }
}
