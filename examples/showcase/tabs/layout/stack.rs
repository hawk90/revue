//! Stack widget demos

use crate::theme_colors;
use revue::prelude::*;

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" VStack ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Vertical stack:").fg(primary))
                            .child(Text::new("Item 1").fg(text))
                            .child(Text::new("Item 2").fg(text))
                            .child(Text::new("Item 3").fg(text))
                            .child(Text::new("Item 4").fg(text))
                            .child(Text::new(""))
                            .child(Text::new("• Vertical arrangement").fg(muted))
                            .child(Text::new("• Top to bottom").fg(muted))
                            .child(Text::new("• Gap control").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" HStack ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Horizontal stack:").fg(primary))
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Text::new("A").fg(text))
                                    .child(Text::new("B").fg(text))
                                    .child(Text::new("C").fg(text))
                                    .child(Text::new("D").fg(text)),
                            )
                            .child(Text::new(""))
                            .child(
                                hstack()
                                    .gap(1)
                                    .child(Button::new("1"))
                                    .child(Button::new("2"))
                                    .child(Button::new("3")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Horizontal arrangement").fg(muted))
                            .child(Text::new("• Left to right").fg(muted))
                            .child(Text::new("• Responsive sizing").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Layers ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Overlapping layers:").fg(primary))
                            .child(
                                Layers::new()
                                    .child(
                                        Border::rounded()
                                            .title("Back")
                                            .child(Text::new("Layer 1").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("Middle")
                                            .child(Text::new("Layer 2").fg(text)),
                                    )
                                    .child(
                                        Border::rounded()
                                            .title("Front")
                                            .child(Text::new("Layer 3").fg(text)),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Z-index stacking").fg(muted))
                            .child(Text::new("• Overlay support").fg(muted))
                            .child(Text::new("• Modal patterns").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Gap Variations ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Gap = 0:").fg(primary))
                            .child(
                                vstack()
                                    .gap(0)
                                    .child(Text::new("A"))
                                    .child(Text::new("B"))
                                    .child(Text::new("C")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Gap = 1:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .child(Text::new("A"))
                                    .child(Text::new("B"))
                                    .child(Text::new("C")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Gap = 2:").fg(primary))
                            .child(
                                vstack()
                                    .gap(2)
                                    .child(Text::new("A"))
                                    .child(Text::new("B"))
                                    .child(Text::new("C")),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Spacing control").fg(muted))
                            .child(Text::new("• Tight to loose").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Alignment ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("VStack alignment:").fg(primary))
                            .child(
                                vstack()
                                    .child(Text::new("Left").fg(text))
                                    .child(Text::new("Aligned").fg(text)),
                            )
                            .child(
                                vstack()
                                    .child(Text::new("Center").fg(text))
                                    .child(Text::new("Aligned").fg(text)),
                            )
                            .child(
                                vstack()
                                    .child(Text::new("Right").fg(text))
                                    .child(Text::new("Aligned").fg(text)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Text alignment").fg(muted))
                            .child(Text::new("• Child positioning").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Spacing ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Space between:").fg(primary))
                            .child(
                                hstack()
                                    .gap(4)
                                    .child(Text::new("Left").fg(text))
                                    .child(Text::new("Right").fg(text)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("Space around:").fg(primary))
                            .child(
                                hstack()
                                    .gap(2)
                                    .child(Text::new("A").fg(text))
                                    .child(Text::new("B").fg(text))
                                    .child(Text::new("C").fg(text)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Distribution").fg(muted))
                            .child(Text::new("• Flexible spacing").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Constrained Stack ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Size-limited stack:").fg(primary))
                            .child(
                                vstack()
                                    .gap(1)
                                    .min_width(15)
                                    .max_width(25)
                                    .child(Text::new("Min 15w, Max 25w").fg(text))
                                    .child(Text::new("Constrained width").fg(muted)),
                            )
                            .child(Text::new(""))
                            .child(
                                hstack()
                                    .gap(1)
                                    .min_height(2)
                                    .max_height(3)
                                    .child(Text::new("A").fg(text))
                                    .child(Text::new("B").fg(text))
                                    .child(Text::new("Min 2h, Max 3h").fg(muted)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• min_width() / min_height()").fg(muted))
                            .child(Text::new("• max_width() / max_height()").fg(muted))
                            .child(Text::new("• Responsive sizing").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Constrained Layers ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Size-limited layers:").fg(primary))
                            .child(
                                Layers::new()
                                    .min_size(20, 3)
                                    .max_size(35, 5)
                                    .child(Border::rounded().child(Text::new("Layer 1")))
                                    .child(Border::rounded().child(Text::new("Layer 2"))),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Min/max dimensions").fg(muted))
                            .child(Text::new("• Overlay constraints").fg(muted))
                            .child(Text::new("• Prevent overflow").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" constrain() Helper ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Set all at once:").fg(primary))
                            .child(
                                vstack()
                                    .gap(0)
                                    .constrain(10, 2, 30, 5)
                                    .child(Text::new("10-30w, 2-5h").fg(text))
                                    .child(Text::new("Constrained stack").fg(muted)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• .constrain(min_w, min_h, max_w, max_h)").fg(muted))
                            .child(Text::new("• One call for all limits").fg(muted))
                            .child(Text::new("• Cleaner code").fg(muted)),
                    ),
                ),
        )
}
