//! Masked input widget demos (MaskedInput, PinInput, CreditCardInput)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{credit_card_input, masked_input, password_input, pin_input, MaskStyle};

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Masked Input",
            "Password-style input with configurable mask characters",
            Border::rounded()
                .title(" Masked Input ")
                .min_width(30)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Password-style input:").fg(primary))
                        .child(masked_input().placeholder("Enter password...").width(25))
                        .child(Text::new(""))
                        .child(Text::new("With custom mask char:").fg(primary))
                        .child(
                            masked_input()
                                .mask_char('*')
                                .placeholder("Secret code...")
                                .width(25),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Mask character display").fg(muted))
                        .child(Text::new("• Secure text entry").fg(muted))
                        .child(Text::new("• Configurable masking").fg(muted)),
                ),
        ),
        Example::new(
            "PIN Input",
            "Individual digit boxes with auto-focus",
            Border::rounded()
                .title(" PIN Input ")
                .min_width(30)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("4-digit PIN:").fg(primary))
                        .child(pin_input(4).width(20))
                        .child(Text::new(""))
                        .child(Text::new("6-digit code:").fg(primary))
                        .child(pin_input(6).width(25))
                        .child(Text::new(""))
                        .child(Text::new("8-digit OTP:").fg(primary))
                        .child(pin_input(8).width(30))
                        .child(Text::new(""))
                        .child(Text::new("• Individual digit boxes").fg(muted))
                        .child(Text::new("• Auto-focus next box").fg(muted))
                        .child(Text::new("• Secure mode option").fg(muted)),
                ),
        ),
        Example::new(
            "Credit Card Input",
            "Card number input with type detection",
            Border::rounded()
                .title(" Credit Card Input ")
                .min_width(35)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Card number:").fg(primary))
                        .child(credit_card_input().width(30))
                        .child(Text::new(""))
                        .child(Text::new("Card types detected:").fg(info))
                        .child(Text::new("• Visa (4XXX)").fg(muted))
                        .child(Text::new("• Mastercard (5XXX)").fg(muted))
                        .child(Text::new("• Amex (34XX/37XX)").fg(muted)),
                ),
        ),
        Example::new(
            "Password Input",
            "Masked password with strength indicator",
            Border::rounded()
                .title(" Password Input ")
                .min_width(30)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Basic password:").fg(primary))
                        .child(password_input("Enter password").width(25))
                        .child(Text::new(""))
                        .child(Text::new("With label:").fg(primary))
                        .child(password_input("Password").width(25))
                        .child(Text::new(""))
                        .child(Text::new("Password strength:").fg(primary))
                        .child(Text::new("Weak: ••••••••").fg(error))
                        .child(Text::new("Medium: ••••••••••").fg(warning))
                        .child(Text::new("Strong: ••••••••••••").fg(success))
                        .child(Text::new(""))
                        .child(Text::new("• Masked by default").fg(muted))
                        .child(Text::new("• Show/hide toggle").fg(muted))
                        .child(Text::new("• Strength indicator").fg(muted)),
                ),
        ),
        Example::new(
            "Mask Styles",
            "Full, ShowLast, and ShowFirst mask display modes",
            Border::rounded()
                .title(" Mask Styles ")
                .min_width(30)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Full mask (default):").fg(primary))
                        .child(
                            masked_input()
                                .mask_style(MaskStyle::Full)
                                .value("secret123")
                                .width(25),
                        )
                        .child(Text::new(""))
                        .child(Text::new("Show last 4:").fg(primary))
                        .child(
                            masked_input()
                                .mask_style(MaskStyle::ShowLast(4))
                                .value("1234567890")
                                .width(25),
                        )
                        .child(Text::new(""))
                        .child(Text::new("Show first 2:").fg(primary))
                        .child(
                            masked_input()
                                .mask_style(MaskStyle::ShowFirst(2))
                                .value("password")
                                .width(25),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Full, ShowLast, ShowFirst").fg(muted))
                        .child(Text::new("• Peek mode (brief reveal)").fg(muted))
                        .child(Text::new("• Hidden (empty)").fg(muted)),
                ),
        ),
        Example::new(
            "Validation States",
            "Min/max length validation and error display",
            Border::rounded()
                .title(" Validation States ")
                .min_width(30)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("With min length:").fg(primary))
                        .child(
                            masked_input()
                                .placeholder("Min 8 chars...")
                                .min_length(8)
                                .width(25),
                        )
                        .child(Text::new(""))
                        .child(Text::new("With max length:").fg(primary))
                        .child(
                            masked_input()
                                .placeholder("Max 20 chars...")
                                .max_length(20)
                                .width(25),
                        )
                        .child(Text::new(""))
                        .child(Text::new(""))
                        .child(Text::new(""))
                        .child(Text::new("• Min/max length validation").fg(muted))
                        .child(Text::new("• Custom validation rules").fg(muted))
                        .child(Text::new("• Error state display").fg(muted)),
                ),
        ),
    ]
}
