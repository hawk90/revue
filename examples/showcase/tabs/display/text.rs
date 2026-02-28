//! Text widget demos

use crate::theme_colors;
use revue::prelude::*;

pub fn render() -> impl View {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Text Styles ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Normal text").fg(text))
                            .child(Text::new("Bold text").bold().fg(text))
                            .child(Text::new("Italic text").italic().fg(text))
                            .child(Text::new("Underlined text").underline().fg(text))
                            .child(Text::new("Dim text").dim().fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Combined:").fg(primary))
                            .child(Text::new("Bold italic underline").bold().italic().underline().fg(success)),
                    ),
                )
                .child(
                    Border::rounded().title(" Text Colors ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Colored text:").fg(primary))
                            .child(Text::new("Primary").fg(primary))
                            .child(Text::new("Success").fg(success))
                            .child(Text::new("Warning").fg(warning))
                            .child(Text::new("Error").fg(error))
                            .child(Text::new("Info").fg(info))
                            .child(Text::new("Muted").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Backgrounds:").fg(primary))
                            .child(Text::new(" Highlight ").bg(primary).fg(text)),
                    ),
                )
                .child(
                    Border::rounded().title(" Big Text ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Large ASCII art:").fg(primary))
                            .child(BigText::new("REVUE", 2))
                            .child(Text::new(""))
                            .child(Text::new("• ASCII art text rendering").fg(muted))
                            .child(Text::new("• Customizable font styles").fg(muted))
                            .child(Text::new("• Good for titles/banners").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Rich Text ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Formatted content:").fg(primary))
                            .child(RichText::markup("This is [bold]bold[/], this is [italic]italic[/], and this is [green]colored[/]."))
                            .child(Text::new(""))
                            .child(RichText::markup("Links: [link=https://example.com]Click here[/]"))
                            .child(Text::new(""))
                            .child(Text::new("• Inline formatting").fg(muted))
                            .child(Text::new("• Mixed styles").fg(muted))
                            .child(Text::new("• Links and colors").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Markdown ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Markdown rendering:").fg(primary))
                            .child(Markdown::new("
# Heading
**Bold** and *italic* text.

- Item 1
- Item 2
- Item 3

> Blockquote

`inline code`
                            "))
                            .child(Text::new(""))
                            .child(Text::new("• Headings and lists").fg(muted))
                            .child(Text::new("• Code blocks").fg(muted))
                            .child(Text::new("• Links and images").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Unicode ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Korean: 안녕하세요!").fg(text))
                            .child(Text::new("Japanese: こんにちは").fg(text))
                            .child(Text::new("Chinese: 你好世界").fg(text))
                            .child(Text::new("Emoji: 🎉 👍 🚀 ✨ 💻").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("Math: ∑ ∞ π √ ∫ ∂").fg(text))
                            .child(Text::new("Arrows: ← → ↑ ↓ ↔ ↕").fg(text))
                            .child(Text::new("Box: ┌ ┐ └ ┘ ├ ┤ ┬ ┴ ┼").fg(text))
                            .child(Text::new("Block: █ ▓ ▒ ░ ▀ ▄").fg(text))
                            .child(Text::new("Shapes: ● ○ ■ □ ▲ ▼").fg(text)),
                    ),
                ),
        )
}
