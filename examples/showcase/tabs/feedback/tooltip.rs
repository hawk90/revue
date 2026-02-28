//! Tooltip widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{ButtonVariant, Input, Tooltip, TooltipStyle};

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Tooltip ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Hover for info:").fg(primary))
                            .child(Text::new(""))
                            .child(Button::primary("Save"))
                            .child(Text::new(""))
                            .child(Button::new("Delete").variant(ButtonVariant::Danger))
                            .child(Text::new(""))
                            .child(Text::new("• Hover trigger").fg(muted))
                            .child(Text::new("• Delay before show").fg(muted))
                            .child(Text::new("• Auto-hide").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Rich Tooltip ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Detailed help:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                Tooltip::new(
                                    "Keyboard Shortcut: Press Ctrl+S to save your changes.",
                                )
                                .title("Shortcut")
                                .style(TooltipStyle::Info),
                            )
                            .child(Text::new(""))
                            .child(
                                Tooltip::new("This action cannot be undone. Proceed with caution.")
                                    .title("Warning")
                                    .style(TooltipStyle::Warning),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Title + body").fg(muted))
                            .child(Text::new("• Multiple lines").fg(muted))
                            .child(Text::new("• Color variants").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Key Hint ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Keyboard hints:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Text::new("[Ctrl] + [S] = Save").fg(text)),
                            )
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Text::new("[Ctrl] + [C] = Copy").fg(text)),
                            )
                            .child(hstack().gap(2).child(Text::new("[Esc] = Cancel").fg(text)))
                            .child(Text::new(""))
                            .child(Text::new("• Key visualization").fg(muted))
                            .child(Text::new("• Shortcut display").fg(muted))
                            .child(Text::new("• Help text").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Position Options ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Tooltip positions:").fg(primary))
                            .child(Text::new(""))
                            .child(Text::new("• Top (default)").fg(muted))
                            .child(Text::new("• Bottom").fg(muted))
                            .child(Text::new("• Left").fg(muted))
                            .child(Text::new("• Right").fg(muted))
                            .child(Text::new("• Auto (best fit)").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("Positioning is automatic").fg(primary))
                            .child(Text::new("based on viewport.").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Help Text ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Inline help:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(Text::new("Password:"))
                                    .child(Input::new()),
                            )
                            .child(Text::new("  Must be 8+ characters with numbers").fg(muted))
                            .child(Text::new(""))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(Text::new("Email:"))
                                    .child(Input::new()),
                            )
                            .child(Text::new("  We'll never share your email").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Inline guidance").fg(muted))
                            .child(Text::new("• Form helpers").fg(muted))
                            .child(Text::new("• Contextual info").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Icon Labels ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Icon tooltips:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                hstack()
                                    .gap(4)
                                    .child(Text::new("(Settings)"))
                                    .child(Text::new("(Notifications)"))
                                    .child(Text::new("(Help)"))
                                    .child(Text::new("(Logout)")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Icon-only buttons").fg(muted))
                            .child(Text::new("• Compact UIs").fg(muted))
                            .child(Text::new("• Accessibility").fg(muted)),
                    ),
                ),
        )
}
