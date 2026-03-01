//! Button widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;

pub fn examples() -> Vec<Example> {
    let (primary, _success, _warning, _error, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Button Variants",
            "Standard button styles",
            Border::rounded()
                .title(" Button Variants ")
                .min_width(35)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Standard button styles:").fg(primary))
                        .child(Text::new(""))
                        .child(
                            hstack()
                                .gap(2)
                                .child(Button::primary("Primary"))
                                .child(Button::new("Default"))
                                .child(Button::new("Danger").variant(ButtonVariant::Danger)),
                        )
                        .child(
                            hstack()
                                .gap(2)
                                .child(Button::new("Success").variant(ButtonVariant::Success))
                                .child(Button::new("Danger").variant(ButtonVariant::Danger))
                                .child(Button::new("Ghost").variant(ButtonVariant::Ghost)),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Primary: Main call-to-action").fg(muted))
                        .child(Text::new("• Default: Standard actions").fg(muted))
                        .child(Text::new("• Danger: Destructive actions").fg(muted))
                        .child(Text::new("• Success: Positive actions").fg(muted))
                        .child(Text::new("• Ghost: Subtle/minimal actions").fg(muted)),
                ),
        ),
        Example::new(
            "Button Sizes",
            "Different sizes for various contexts",
            Border::rounded()
                .title(" Button Sizes ")
                .min_width(30)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Different sizes:").fg(primary))
                        .child(Text::new(""))
                        .child(
                            hstack()
                                .gap(2)
                                .child(Button::new("Small"))
                                .child(Button::new("Medium"))
                                .child(Button::new("Large")),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Compact UIs").fg(muted))
                        .child(Text::new("• Standard size").fg(muted))
                        .child(Text::new("• Emphasis/importance").fg(muted)),
                ),
        ),
        Example::new(
            "Icon Buttons",
            "Buttons with icons and symbols",
            Border::rounded()
                .title(" Icon Buttons ")
                .min_width(35)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Buttons with icons:").fg(primary))
                        .child(Text::new(""))
                        .child(
                            hstack()
                                .gap(2)
                                .child(Button::new("♥ Like"))
                                .child(Button::new("★ Star"))
                                .child(Button::new("✓ Done"))
                                .child(Button::new("✕ Cancel")),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Combine text with symbols").fg(muted))
                        .child(Text::new("• Use Unicode characters").fg(muted))
                        .child(Text::new("• Icon-only buttons supported").fg(muted)),
                ),
        ),
        Example::new(
            "Interactive States",
            "Button states and interactions",
            Border::rounded()
                .title(" Interactive States ")
                .min_width(35)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Button states:").fg(primary))
                        .child(Text::new(""))
                        .child(
                            hstack()
                                .gap(2)
                                .child(Button::new("Normal"))
                                .child(Button::new("Disabled").disabled(true)),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Normal: Default interactive").fg(muted))
                        .child(Text::new("• Hover: When focused (see focus ring)").fg(muted))
                        .child(Text::new("• Disabled: Non-interactive").fg(muted))
                        .child(Text::new("• Pressed: During click action").fg(muted)),
                ),
        ),
        Example::new(
            "Button Groups",
            "Grouped and connected buttons",
            Border::rounded()
                .title(" Button Groups ")
                .min_width(40)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Grouped buttons:").fg(primary))
                        .child(Text::new(""))
                        .child(
                            hstack()
                                .gap(0)
                                .child(Button::new("Left"))
                                .child(Button::new("Center"))
                                .child(Button::new("Right")),
                        )
                        .child(Text::new(""))
                        .child(
                            hstack()
                                .gap(1)
                                .child(Button::new("◄ Prev"))
                                .child(Button::new("1"))
                                .child(Button::new("2"))
                                .child(Button::new("3"))
                                .child(Button::new("Next ►")),
                        ),
                ),
        ),
    ]
}
