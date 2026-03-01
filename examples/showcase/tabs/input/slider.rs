//! Slider widget demos (Slider, Rating, Stepper)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{slider_range, step, Rating, Slider, StepStatus, Stepper};

pub fn examples(slider_val: f64, rating_val: u8) -> Vec<Example> {
    let (primary, _success, _warning, _, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Slider",
            "Continuous value selection with range constraints",
            Border::rounded().title(" Slider ").child(
                vstack()
                    .gap(1)
                    .child(Text::new(format!("Volume: {:.0}%", slider_val)).fg(primary))
                    .child(Slider::new().value(slider_val).range(0.0, 100.0).label(""))
                    .child(Text::new(""))
                    .child(Text::new("[←/→] adjust slider").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("Percentage slider:").fg(primary))
                    .child(Slider::new().value(65.0).range(0.0, 100.0))
                    .child(Text::new(""))
                    .child(Text::new("• Continuous value selection").fg(muted))
                    .child(Text::new("• Range constraint").fg(muted))
                    .child(Text::new("• Step increment support").fg(muted)),
            ),
        ),
        Example::new(
            "Rating",
            "Discrete value selection with star ratings",
            Border::rounded().title(" Rating ").child(
                vstack()
                    .gap(1)
                    .child(Text::new(format!("Rating: {} / 5", rating_val)).fg(primary))
                    .child(Rating::new().value(rating_val as f32).max_value(5))
                    .child(Text::new(""))
                    .child(Text::new("[-/+] change rating").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("Star rating:").fg(primary))
                    .child(Rating::new().value(3.5).max_value(5))
                    .child(Text::new(""))
                    .child(Text::new("Heart rating:").fg(primary))
                    .child(Rating::new().value(4.0).max_value(5))
                    .child(Text::new(""))
                    .child(Text::new("• Discrete value selection").fg(muted))
                    .child(Text::new("• Custom icons").fg(muted))
                    .child(Text::new("• Half-star support").fg(muted)),
            ),
        ),
        Example::new(
            "Stepper",
            "Sequential process steps with status indicators",
            Border::rounded().title(" Stepper ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Process steps:").fg(primary))
                    .child(
                        Stepper::new()
                            .step(step("Initialize").status(StepStatus::Completed))
                            .step(step("Load config").status(StepStatus::Completed))
                            .step(step("Connect database").status(StepStatus::Active))
                            .step(step("Start server").status(StepStatus::Pending))
                            .step(step("Health check").status(StepStatus::Pending)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Increment/decrement").fg(muted))
                    .child(Text::new("• Status indicators").fg(muted))
                    .child(Text::new("• Sequential process").fg(muted)),
            ),
        ),
        Example::new(
            "Range Slider",
            "Two-handle slider for min/max range selection",
            Border::rounded().title(" Range Slider ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Price range:").fg(primary))
                    .child(slider_range(20.0, 80.0).range(0.0, 100.0))
                    .child(Text::new(""))
                    .child(Text::new("Date range:").fg(primary))
                    .child(slider_range(5.0, 15.0).range(1.0, 31.0))
                    .child(Text::new(""))
                    .child(Text::new("• Two-handle slider").fg(muted))
                    .child(Text::new("• Min and max selection").fg(muted))
                    .child(Text::new("• Range filtering").fg(muted)),
            ),
        ),
        Example::new(
            "Volume Control",
            "Slider with icon decorators for audio controls",
            Border::rounded().title(" Volume Control ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Audio volume:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new("🔈"))
                            .child(Slider::new().value(30.0).range(0.0, 100.0))
                            .child(Text::new("🔊")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("Mic level:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(Text::new("🎤"))
                            .child(Slider::new().value(75.0).range(0.0, 100.0))
                            .child(Text::new("")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Icon decorators").fg(muted))
                    .child(Text::new("• Horizontal layout").fg(muted)),
            ),
        ),
    ]
}
