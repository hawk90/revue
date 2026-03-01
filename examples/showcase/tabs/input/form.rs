//! Form widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{password_input, Input, RichTextEditor, Select, TextArea};

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Login Form",
            "Form validation, submit handling, and field grouping",
            Border::rounded().title(" Login Form ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Username:").fg(primary))
                    .child(Input::new().placeholder("Enter username..."))
                    .child(Text::new("Password:").fg(primary))
                    .child(password_input("••••••••"))
                    .child(Text::new(""))
                    .child(Checkbox::new("Remember me").checked(true))
                    .child(Text::new(""))
                    .child(Button::primary("Login"))
                    .child(Text::new(""))
                    .child(Text::new("• Form validation").fg(muted))
                    .child(Text::new("• Submit handling").fg(muted))
                    .child(Text::new("• Field grouping").fg(muted)),
            ),
        ),
        Example::new(
            "Form Fields",
            "Label, input, help text, and select combinations",
            Border::rounded().title(" Form Fields ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Email:").fg(primary))
                    .child(Input::new().placeholder("user@example.com"))
                    .child(Text::new("We'll never share your email.").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("Bio:").fg(primary))
                    .child(TextArea::new().placeholder("Tell us about yourself..."))
                    .child(Text::new(""))
                    .child(Text::new("Country:").fg(primary))
                    .child(
                        Select::new()
                            .option("United States")
                            .option("Canada")
                            .option("United Kingdom")
                            .placeholder("Select country..."),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Label + input combo").fg(muted))
                    .child(Text::new("• Help text support").fg(muted))
                    .child(Text::new("• Error state display").fg(muted)),
            ),
        ),
        Example::new(
            "Form Validation",
            "Success, error, and warning validation states",
            Border::rounded().title(" Form Validation ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Username:").fg(success))
                    .child(Input::new().value("validuser"))
                    .child(Text::new("Username is available!").fg(success))
                    .child(Text::new(""))
                    .child(Text::new("Email:").fg(error))
                    .child(Input::new().value("invalid-email"))
                    .child(Text::new("Please enter a valid email address.").fg(error))
                    .child(Text::new(""))
                    .child(Text::new("Password:").fg(warning))
                    .child(password_input("••••••••"))
                    .child(Text::new("Password is weak.").fg(warning))
                    .child(Text::new(""))
                    .child(Text::new("• Success state (green)").fg(muted))
                    .child(Text::new("• Error state (red)").fg(muted))
                    .child(Text::new("• Warning state (yellow)").fg(muted)),
            ),
        ),
        Example::new(
            "Rich Text Editor",
            "Formatted text input with toolbar controls",
            Border::rounded().title(" Rich Text Editor ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Formatted text input:").fg(primary))
                    .child(RichTextEditor::new())
                    .child(Text::new(""))
                    .child(
                        Text::new("Toolbar: [B] [I] [U] [S] [Link] [List] [Code] [Quote]")
                            .fg(muted),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Bold, italic, underline").fg(muted))
                    .child(Text::new("• Strikethrough support").fg(muted))
                    .child(Text::new("• Links and images").fg(muted))
                    .child(Text::new("• Lists and code blocks").fg(muted)),
            ),
        ),
        Example::new(
            "Field Layout",
            "Inline and stacked field arrangements",
            Border::rounded().title(" Field Layout ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Inline fields:").fg(primary))
                    .child(
                        hstack()
                            .gap(2)
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Text::new("First:").fg(muted))
                                    .child(Input::new().placeholder("John")),
                            )
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Text::new("Last:").fg(muted))
                                    .child(Input::new().placeholder("Doe")),
                            ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Stacked fields:").fg(primary))
                    .child(Input::new().placeholder("Address line 1"))
                    .child(Input::new().placeholder("Address line 2"))
                    .child(
                        hstack()
                            .gap(2)
                            .child(Input::new().placeholder("City"))
                            .child(Input::new().placeholder("ZIP")),
                    ),
            ),
        ),
    ]
}
