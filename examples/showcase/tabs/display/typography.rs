//! Typography widget demos (BigText, Digits, Headings)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{bigtext, digits, h1, h2, h3, BigText, Digits};

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Big Text",
            "ASCII art text for terminal banners and title screens",
            Border::rounded()
                .title(" Big Text ")
                .min_width(30)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("ASCII art text:").fg(primary))
                        .child(BigText::new("REVUE", 2))
                        .child(Text::new(""))
                        .child(Text::new("Smaller scale:").fg(primary))
                        .child(bigtext("RUST", 1))
                        .child(Text::new(""))
                        .child(Text::new("• Terminal banners").fg(muted))
                        .child(Text::new("• Title screens").fg(muted))
                        .child(Text::new("• ASCII art headers").fg(muted)),
                ),
        ),
        Example::new(
            "Digits",
            "Large digit display for scores, counters, and statistics",
            Border::rounded()
                .title(" Digits ")
                .min_width(30)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Large digit display:").fg(primary))
                        .child(Digits::new(12345))
                        .child(Text::new(""))
                        .child(Text::new("Styled digits:").fg(primary))
                        .child(digits(42).fg(success))
                        .child(Text::new(""))
                        .child(Text::new("With color:").fg(primary))
                        .child(digits(9876543).fg(primary))
                        .child(Text::new(""))
                        .child(Text::new("• Score displays").fg(muted))
                        .child(Text::new("• Counters").fg(muted))
                        .child(Text::new("• Statistics").fg(muted)),
                ),
        ),
        Example::new(
            "Headings",
            "Heading levels for visual text hierarchy",
            Border::rounded()
                .title(" Headings ")
                .min_width(30)
                .min_height(12)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Heading levels:").fg(primary))
                        .child(h1("Heading 1").fg(primary))
                        .child(h2("Heading 2").fg(success))
                        .child(h3("Heading 3").fg(info))
                        .child(Text::new(""))
                        .child(Text::new("• H1: Largest").fg(muted))
                        .child(Text::new("• H2: Medium").fg(muted))
                        .child(Text::new("• H3: Smallest").fg(muted)),
                ),
        ),
        Example::new(
            "Big Text Examples",
            "Status messages rendered as large ASCII art",
            Border::rounded()
                .title(" Big Text Examples ")
                .min_width(30)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Status messages:").fg(primary))
                        .child(bigtext("OK", 1).fg(success))
                        .child(Text::new(""))
                        .child(bigtext("ERROR", 1).fg(error))
                        .child(Text::new(""))
                        .child(bigtext("DONE", 1).fg(success))
                        .child(Text::new(""))
                        .child(Text::new("• Status indicators").fg(muted))
                        .child(Text::new("• Alert messages").fg(muted))
                        .child(Text::new("• Completion states").fg(muted)),
                ),
        ),
        Example::new(
            "Digit Patterns",
            "Time display and counter patterns using large digits",
            Border::rounded()
                .title(" Digit Patterns ")
                .min_width(30)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Time display:").fg(primary))
                        .child(
                            hstack()
                                .gap(0)
                                .child(digits(12).fg(text))
                                .child(Text::new(":"))
                                .child(digits(34).fg(text))
                                .child(Text::new(":"))
                                .child(digits(56).fg(text)),
                        )
                        .child(Text::new(""))
                        .child(Text::new("Counter:").fg(primary))
                        .child(digits(0).fg(warning))
                        .child(Text::new(""))
                        .child(Text::new("• Clock displays").fg(muted))
                        .child(Text::new("• Timers").fg(muted))
                        .child(Text::new("• Leaderboards").fg(muted)),
                ),
        ),
        Example::new(
            "Typography Styles",
            "Combined headings, digits, and statistics layout",
            Border::rounded()
                .title(" Typography Styles ")
                .min_width(40)
                .min_height(14)
                .child(
                    vstack()
                        .gap(1)
                        .child(Text::new("Combination example:").fg(primary))
                        .child(h1("TUI Framework").fg(primary))
                        .child(h2("v2.52.2").fg(muted))
                        .child(h3("Build Terminal UIs Fast").fg(info))
                        .child(Text::new(""))
                        .child(Text::new("Statistics:").fg(primary))
                        .child(
                            hstack()
                                .gap(3)
                                .child(
                                    vstack()
                                        .gap(0)
                                        .child(digits(92).fg(primary))
                                        .child(Text::new("Widgets").fg(muted)),
                                )
                                .child(
                                    vstack()
                                        .gap(0)
                                        .child(digits(7).fg(success))
                                        .child(Text::new("Categories").fg(muted)),
                                )
                                .child(
                                    vstack()
                                        .gap(0)
                                        .child(digits(100).fg(info))
                                        .child(Text::new("% Rust").fg(muted)),
                                ),
                        )
                        .child(Text::new(""))
                        .child(Text::new("• Combined layouts").fg(muted))
                        .child(Text::new("• Visual hierarchy").fg(muted))
                        .child(Text::new("• Dashboard stats").fg(muted)),
                ),
        ),
    ]
}
