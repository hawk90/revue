//! Number input widget demos (NumberInput, IntegerInput, CurrencyInput, PercentageInput)

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{
    currency_input, integer_input, number_input, percentage_input, percentage_slider,
    volume_slider, NumberInput,
};

pub fn render() -> impl View {
    let (primary, _success, _warning, error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Number Input ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Basic number input:").fg(primary))
                                .child(number_input().value(42.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("With range constraints:").fg(primary))
                                .child(
                                    NumberInput::new()
                                        .value(50.0)
                                        .min(0.0)
                                        .max(100.0)
                                        .step(5.0)
                                        .width(20),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Decimal precision:").fg(primary))
                                .child(number_input().value(3.14159).precision(4).width(20))
                                .child(Text::new(""))
                                .child(Text::new("• Increment/decrement buttons").fg(muted))
                                .child(Text::new("• Keyboard Up/Down arrows").fg(muted))
                                .child(Text::new("• Configurable step size").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Integer Input ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Whole numbers only:").fg(primary))
                                .child(integer_input().value(100.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("With bounds:").fg(primary))
                                .child(integer_input().value(25.0).min(0.0).max(100.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("Quantity selector:").fg(primary))
                                .child(integer_input().value(1.0).min(1.0).max(99.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("• Integer-only validation").fg(muted))
                                .child(Text::new("• Min/max enforcement").fg(muted))
                                .child(Text::new("• Step by any integer").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Currency Input ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("USD format:").fg(primary))
                                .child(currency_input("$").value(1234.56).width(20))
                                .child(Text::new(""))
                                .child(Text::new("KRW (no decimals):").fg(primary))
                                .child(currency_input("₩").value(50000.0).precision(0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("EUR format:").fg(primary))
                                .child(currency_input("€").value(987.65).width(20))
                                .child(Text::new(""))
                                .child(Text::new("• Currency symbol prefix").fg(muted))
                                .child(Text::new("• Thousands separator").fg(muted))
                                .child(Text::new("• Locale-aware formatting").fg(muted)),
                        ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Percentage Input ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Basic percentage:").fg(primary))
                                .child(percentage_input().value(75.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("With limits (0-100):").fg(primary))
                                .child(percentage_input().value(50.0).min(0.0).max(100.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("Percentage slider:").fg(primary))
                                .child(percentage_slider().value(65.0))
                                .child(Text::new(""))
                                .child(Text::new("• % symbol suffix").fg(muted))
                                .child(Text::new("• Automatic formatting").fg(muted))
                                .child(Text::new("• Slider + input combo").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Specialized Sliders ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Volume slider:").fg(primary))
                                .child(volume_slider().value(0.7))
                                .child(Text::new(""))
                                .child(Text::new("Temperature range:").fg(primary))
                                .child(
                                    number_input()
                                        .value(22.5)
                                        .min(-20.0)
                                        .max(50.0)
                                        .precision(1)
                                        .suffix("°C")
                                        .width(20),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Weight input:").fg(primary))
                                .child(
                                    number_input()
                                        .value(75.5)
                                        .min(0.0)
                                        .max(500.0)
                                        .precision(1)
                                        .suffix(" kg")
                                        .width(20),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Icon integration").fg(muted))
                                .child(Text::new("• Custom suffixes").fg(muted))
                                .child(Text::new("• Domain-specific UX").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Number Input States ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Default:").fg(primary))
                                .child(number_input().value(0.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("Empty:").fg(primary))
                                .child(number_input().width(20))
                                .child(Text::new(""))
                                .child(Text::new("Error state (out of range):").fg(error))
                                .child(number_input().value(150.0).max(100.0).width(20))
                                .child(Text::new(""))
                                .child(Text::new("• Validation feedback").fg(muted))
                                .child(Text::new("• Error highlighting").fg(muted))
                                .child(Text::new("• Disabled state").fg(muted)),
                        ),
                ),
        )
}
