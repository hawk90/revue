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
}
