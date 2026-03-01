//! Grid widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{positioned, Card, Grid};

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, _error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Grid",
            "Auto-wrapping grid with configurable columns",
            Border::rounded().title(" Grid ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("3-column grid:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(3)
                            .gap(1)
                            .child(Border::rounded().title("A").child(Text::new("1").fg(text)))
                            .child(Border::rounded().title("B").child(Text::new("2").fg(text)))
                            .child(Border::rounded().title("C").child(Text::new("3").fg(text)))
                            .child(Border::rounded().title("D").child(Text::new("4").fg(text)))
                            .child(Border::rounded().title("E").child(Text::new("5").fg(text)))
                            .child(Border::rounded().title("F").child(Text::new("6").fg(text))),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Auto-wrapping").fg(muted))
                    .child(Text::new("• Responsive columns").fg(muted))
                    .child(Text::new("• Gap control").fg(muted)),
            ),
        ),
        Example::new(
            "Grid Variations",
            "Different column counts and gap settings",
            Border::rounded().title(" Grid Variations ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("2-column:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(2)
                            .gap(1)
                            .child(Text::new("1,1").fg(text))
                            .child(Text::new("1,2").fg(text))
                            .child(Text::new("2,1").fg(text))
                            .child(Text::new("2,2").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("4-column:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(4)
                            .gap(0)
                            .child(Text::new("A").fg(text))
                            .child(Text::new("B").fg(text))
                            .child(Text::new("C").fg(text))
                            .child(Text::new("D").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Column count").fg(muted))
                    .child(Text::new("• Flexible layout").fg(muted)),
            ),
        ),
        Example::new(
            "Position",
            "Absolute positioning with x/y coordinates",
            Border::rounded().title(" Position ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Absolute positioning:").fg(primary))
                    .child(positioned(Text::new("(2,0)").fg(success)).x(2).y(0))
                    .child(positioned(Text::new("(0,1)").fg(info)).x(0).y(1))
                    .child(Text::new(""))
                    .child(Text::new("• x, y coordinates").fg(muted))
                    .child(Text::new("• Relative/absolute").fg(muted))
                    .child(Text::new("• Z-index support").fg(muted)),
            ),
        ),
        Example::new(
            "Card Grid",
            "Dashboard-style grid with card widgets",
            Border::rounded().title(" Card Grid ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Widget cards:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(3)
                            .gap(1)
                            .child(Card::new().title("CPU").body(Text::new("42%").fg(success)))
                            .child(Card::new().title("MEM").body(Text::new("67%").fg(info)))
                            .child(Card::new().title("DISK").body(Text::new("83%").fg(warning)))
                            .child(Card::new().title("NET").body(Text::new("15 MB/s").fg(text)))
                            .child(Card::new().title("LOAD").body(Text::new("1.25").fg(text)))
                            .child(
                                Card::new()
                                    .title("UPTIME")
                                    .body(Text::new("5d 12h").fg(text)),
                            ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Dashboard layout").fg(muted))
                    .child(Text::new("• Metric cards").fg(muted)),
            ),
        ),
        Example::new(
            "Span Cells",
            "Grid cells spanning multiple columns",
            Border::rounded().title(" Span Cells ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Cell spanning:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(3)
                            .gap(1)
                            .child(Text::new("1x1").fg(text))
                            .child(Text::new("Span 2").fg(success))
                            .child(Text::new("3x1").fg(text))
                            .child(Text::new("3x2").fg(text))
                            .child(Text::new("3x3").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Column span").fg(muted))
                    .child(Text::new("• Row span").fg(muted))
                    .child(Text::new("• Complex layouts").fg(muted)),
            ),
        ),
        Example::new(
            "Responsive",
            "Responsive grid adapting to terminal size",
            Border::rounded().title(" Responsive ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Responsive grid:").fg(primary))
                    .child(Text::new("• Auto-fit columns").fg(muted))
                    .child(Text::new("• Min/max widths").fg(muted))
                    .child(Text::new("• Breakpoints").fg(muted))
                    .child(Text::new(""))
                    .child(
                        Grid::new()
                            .cols(4)
                            .gap(1)
                            .child(Badge::new("A"))
                            .child(Badge::new("B"))
                            .child(Badge::new("C"))
                            .child(Badge::new("D"))
                            .child(Badge::new("E"))
                            .child(Badge::new("F"))
                            .child(Badge::new("G"))
                            .child(Badge::new("H")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Adaptive layout").fg(muted))
                    .child(Text::new("• Terminal resize").fg(muted)),
            ),
        ),
        Example::new(
            "Constrained Grid",
            "Grid with min/max width constraints",
            Border::rounded().title(" Constrained Grid ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Size-limited grid:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(3)
                            .gap(1)
                            .min_width(25)
                            .max_width(45)
                            .child(Badge::new("A"))
                            .child(Badge::new("B"))
                            .child(Badge::new("C"))
                            .child(Badge::new("D"))
                            .child(Badge::new("E"))
                            .child(Badge::new("F")),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• min_width() / max_width()").fg(muted))
                    .child(Text::new("• min_height() / max_height()").fg(muted))
                    .child(Text::new("• Responsive grid sizing").fg(muted)),
            ),
        ),
        Example::new(
            "Constrained Cards",
            "Cards with size constraints inside a grid",
            Border::rounded().title(" Constrained Cards ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Sized cards in grid:").fg(primary))
                    .child(
                        Grid::new()
                            .cols(2)
                            .gap(1)
                            .child(
                                Card::new()
                                    .title("Min 12w")
                                    .body(Text::new("Fixed min"))
                                    .min_width(12),
                            )
                            .child(
                                Card::new()
                                    .title("Max 15w")
                                    .body(Text::new("Fixed max"))
                                    .max_width(15),
                            ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Card sizing").fg(muted))
                    .child(Text::new("• Dashboard tiles").fg(muted))
                    .child(Text::new("• Consistent layout").fg(muted)),
            ),
        ),
        Example::new(
            "Positioned",
            "Positioned elements with size constraints",
            Border::rounded().title(" Positioned ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Constrained position:").fg(primary))
                    .child(
                        positioned(Text::new("Min 8w, Max 15w").fg(text))
                            .x(2)
                            .y(1)
                            .min_width(8)
                            .max_width(15),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Positioned constraints").fg(muted))
                    .child(Text::new("• Size limits").fg(muted))
                    .child(Text::new("• Overflow control").fg(muted)),
            ),
        ),
    ]
}
