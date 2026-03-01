//! Modal widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{ButtonVariant, Input, Popover};

pub fn examples() -> Vec<Example> {
    let (primary, _success, _warning, error, _info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Modal Dialog",
            "Basic modal with centered overlay and focus trap",
            Border::rounded().title(" Modal Dialog ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Basic modal:").fg(primary))
                    .child(Text::new(""))
                    .child(Button::new("Open Modal"))
                    .child(Text::new(""))
                    .child(Text::new("Modal features:").fg(primary))
                    .child(Text::new("• Centered overlay").fg(muted))
                    .child(Text::new("• Backdrop dimming").fg(muted))
                    .child(Text::new("• ESC to close").fg(muted))
                    .child(Text::new("• Focus trap").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("Common uses:").fg(primary))
                    .child(Text::new("• Confirmations").fg(muted))
                    .child(Text::new("• Forms").fg(muted))
                    .child(Text::new("• Information dialogs").fg(muted)),
            ),
        ),
        Example::new(
            "Confirm Dialog",
            "Delete confirmation with destructive action warning",
            Border::rounded().title(" Confirm Dialog ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Delete confirmation:").fg(primary))
                    .child(Text::new(""))
                    .child(
                        Border::double().title(" Confirm Delete ").child(
                            vstack()
                                .gap(1)
                                .child(
                                    Text::new("Are you sure you want to delete this item?")
                                        .fg(text),
                                )
                                .child(Text::new("This action cannot be undone.").fg(error))
                                .child(Text::new(""))
                                .child(
                                    hstack().gap(2).child(Button::new("Cancel")).child(
                                        Button::new("Delete").variant(ButtonVariant::Danger),
                                    ),
                                ),
                        ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Yes/No dialogs").fg(muted))
                    .child(Text::new("• Destructive actions").fg(muted))
                    .child(Text::new("• Clear messaging").fg(muted)),
            ),
        ),
        Example::new(
            "Form Modal",
            "Modal with input fields for data entry",
            Border::rounded().title(" Form Modal ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Input dialog:").fg(primary))
                    .child(Text::new(""))
                    .child(
                        Border::rounded().title(" Create Project ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Name:").fg(muted))
                                .child(Input::new().placeholder("Project name..."))
                                .child(Text::new("Description:").fg(muted))
                                .child(Input::new().placeholder("Brief description..."))
                                .child(Text::new(""))
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(Button::new("Cancel"))
                                        .child(Button::primary("Create")),
                                ),
                        ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Form inputs").fg(muted))
                    .child(Text::new("• Submit/Cancel").fg(muted))
                    .child(Text::new("• Validation").fg(muted)),
            ),
        ),
        Example::new(
            "Popover",
            "Context popup with positioned help text",
            Border::rounded().title(" Popover ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Context popup:").fg(primary))
                    .child(Text::new(""))
                    .child(
                        hstack().gap(2).child(Text::new("Hover me →")).child(
                            Popover::new("Help text here\nAdditional context")
                                .anchor(40, 10)
                                .open(true),
                        ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Hover/click trigger").fg(muted))
                    .child(Text::new("• Positioned popup").fg(muted))
                    .child(Text::new("• Context help").fg(muted)),
            ),
        ),
        Example::new(
            "Alert Modal",
            "Color-coded alert dialogs with action buttons",
            Border::rounded().title(" Alert Modal ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Alert types:").fg(primary))
                    .child(Text::new(""))
                    .child(
                        Border::rounded().title(" ⚠️ Warning ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Your session will expire in 5 minutes.").fg(text))
                                .child(
                                    hstack()
                                        .gap(2)
                                        .child(Button::new("Extend"))
                                        .child(Button::new("Dismiss")),
                                ),
                        ),
                    )
                    .child(Text::new(""))
                    .child(
                        Border::rounded().title(" ℹ️ Information ").child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Update available. Restart to apply.").fg(text))
                                .child(Button::new("Restart Now")),
                        ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Color-coded").fg(muted))
                    .child(Text::new("• Icon indicators").fg(muted))
                    .child(Text::new("• Action buttons").fg(muted)),
            ),
        ),
        Example::new(
            "Size Variants",
            "Modal sizes and positioning options",
            Border::rounded().title(" Size Variants ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Modal sizes:").fg(primary))
                    .child(Text::new(""))
                    .child(Text::new("• Small: Alert dialogs").fg(muted))
                    .child(Text::new("• Medium: Forms").fg(muted))
                    .child(Text::new("• Large: Content viewers").fg(muted))
                    .child(Text::new("• Full-screen: Complex UIs").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("Positioning:").fg(primary))
                    .child(Text::new("• Center (default)").fg(muted))
                    .child(Text::new("• Top/bottom").fg(muted))
                    .child(Text::new("• Side drawer").fg(muted))
                    .child(Text::new("• Custom offset").fg(muted)),
            ),
        ),
    ]
}
