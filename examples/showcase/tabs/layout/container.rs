//! Container widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Accordion, AccordionSection, Card, Collapsible};

pub fn render() -> impl View {
    let (primary, _success, warning, error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Card ").child(
                        vstack()
                            .gap(1)
                            .child(Card::new()
                                .title("Card Title")
                                .body(Text::new("Card body content goes here. Cards are useful for grouping related content."))
                                .footer(Text::new("Footer").fg(muted)))
                            .child(Text::new(""))
                            .child(Text::new("• Header + body + footer").fg(muted))
                            .child(Text::new("• Grouped content").fg(muted))
                            .child(Text::new("• Shadow/border options").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Collapsible ").child(
                        vstack()
                            .gap(1)
                            .child(Collapsible::new("Section 1")
                                .expanded(true)
                                .content("This section is expanded by default."))
                            .child(Collapsible::new("Section 2")
                                .expanded(false)
                                .content("This section is collapsed."))
                            .child(Collapsible::new("Section 3")
                                .expanded(false)
                                .content("Another collapsed section."))
                            .child(Text::new(""))
                            .child(Text::new("• Expand/collapse").fg(muted))
                            .child(Text::new("• Toggle visibility").fg(muted))
                            .child(Text::new("• Animated transition").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Accordion ").child(
                        vstack()
                            .gap(1)
                            .child(Accordion::new()
                                .section(
                                    AccordionSection::new("FAQ 1: What is Revue?")
                                        .content("A Rust TUI framework with 92+ widgets.")
                                )
                                .section(
                                    AccordionSection::new("FAQ 2: How many widgets?")
                                        .content("92+ widgets available!")
                                )
                                .section(
                                    AccordionSection::new("FAQ 3: Is it fast?")
                                        .content("Yes, extremely fast!")
                                )
                                .section(
                                    AccordionSection::new("FAQ 4: Async support?")
                                        .content("Full async/await support.")
                                ))
                            .child(Text::new(""))
                            .child(Text::new("• Single expansion").fg(muted))
                            .child(Text::new("• FAQ pattern").fg(muted))
                            .child(Text::new("• Keyboard navigation").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" ScrollView ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Scrollable content:").fg(primary))
                            .child(Text::new("Line 1 - Scroll down").fg(text))
                            .child(Text::new("Line 2").fg(text))
                            .child(Text::new("Line 3").fg(text))
                            .child(Text::new("Line 4").fg(text))
                            .child(Text::new("Line 5").fg(text))
                            .child(Text::new("...more content...").fg(muted))
                            .child(Text::new(""))
                            .child(Text::new("• Virtual scrolling").fg(muted))
                            .child(Text::new("• Large content").fg(muted))
                            .child(Text::new("• Keyboard/mouse").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Container ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Generic container:").fg(primary))
                            .child(Border::rounded().child(Text::new("Padded content inside container.")))
                            .child(Text::new(""))
                            .child(Text::new("Centered:").fg(primary))
                            .child(Border::rounded().child(Text::new("Centered content")))
                            .child(Text::new(""))
                            .child(Text::new("• Padding/margin").fg(muted))
                            .child(Text::new("• Alignment options").fg(muted))
                            .child(Text::new("• Border toggle").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Panel ").child(
                        vstack()
                            .gap(1)
                            .child(Border::rounded().title("Info Panel").child(Text::new("This is an informational panel.")))
                            .child(Border::rounded().title("Warning Panel").fg(warning).child(Text::new("This is a warning panel.")))
                            .child(Border::rounded().title("Error Panel").fg(error).child(Text::new("This is an error panel.")))
                            .child(Text::new(""))
                            .child(Text::new("• Color variants").fg(muted))
                            .child(Text::new("• Status display").fg(muted))
                            .child(Text::new("• Styled containers").fg(muted)),
                    ),
                ),
        )
}
