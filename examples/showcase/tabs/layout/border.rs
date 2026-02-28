//! Border widget demos

use crate::theme_colors;
use revue::prelude::*;

pub fn render() -> impl View {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack().gap(3).child(
                Border::rounded().title(" Border Types ").child(
                    hstack()
                        .gap(1)
                        .child(
                            Border::single()
                                .title("Single")
                                .child(Text::new("Content").fg(text)),
                        )
                        .child(
                            Border::double()
                                .title("Double")
                                .child(Text::new("Content").fg(text)),
                        )
                        .child(
                            Border::thick()
                                .title("Thick")
                                .child(Text::new("Content").fg(text)),
                        )
                        .child(
                            Border::rounded()
                                .title("Rounded")
                                .child(Text::new("Content").fg(text)),
                        ),
                ),
            ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Border Styles ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Styled borders:").fg(primary))
                            .child(Text::new(""))
                            .child(
                                Border::single()
                                    .title("Default")
                                    .child(Text::new("Standard border").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Primary")
                                    .fg(primary)
                                    .child(Text::new("Primary color").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Success")
                                    .fg(success)
                                    .child(Text::new("Success color").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Warning")
                                    .fg(warning)
                                    .child(Text::new("Warning color").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Error")
                                    .fg(error)
                                    .child(Text::new("Error color").fg(text)),
                            ),
                    ),
                )
                .child(
                    Border::rounded().title(" Border Colors ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Border::rounded()
                                    .title("Primary")
                                    .fg(primary)
                                    .child(Text::new("Themed border").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Success")
                                    .fg(success)
                                    .child(Text::new("Themed border").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Warning")
                                    .fg(warning)
                                    .child(Text::new("Themed border").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Error")
                                    .fg(error)
                                    .child(Text::new("Themed border").fg(text)),
                            ),
                    ),
                )
                .child(
                    Border::rounded().title(" Padding ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("With padding:").fg(primary))
                            .child(
                                Border::rounded()
                                    .title("No Padding")
                                    .child(Text::new("Tight").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Padding 1")
                                    .child(Text::new("Normal").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Padding 2")
                                    .child(Text::new("Spacious").fg(text)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Content spacing").fg(muted))
                            .child(Text::new("• Adjustable margins").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Nested Borders ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Border nesting:").fg(primary))
                            .child(
                                Border::single().title("Outer").child(
                                    Border::double().title("Middle").child(
                                        Border::rounded()
                                            .title("Inner")
                                            .child(Text::new("Deeply nested").fg(text)),
                                    ),
                                ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Multiple levels").fg(muted))
                            .child(Text::new("• Visual hierarchy").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Borders with Icons ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Border::rounded()
                                    .title(" 📁 Files ")
                                    .child(Text::new("File browser").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title(" ⚙ Settings ")
                                    .child(Text::new("Configuration").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title(" 🔔 Notifications ")
                                    .child(Text::new("Alerts").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title(" 📊 Analytics ")
                                    .child(Text::new("Statistics").fg(text)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Emoji icons").fg(muted))
                            .child(Text::new("• Visual identification").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Border States ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Border::rounded()
                                    .title("Normal")
                                    .child(Text::new("Standard state").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Info")
                                    .fg(info)
                                    .child(Text::new("Info state").fg(text)),
                            )
                            .child(
                                Border::rounded()
                                    .title("Muted")
                                    .fg(muted)
                                    .child(Text::new("Muted state").fg(muted)),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Visual states").fg(muted))
                            .child(Text::new("• Color-based feedback").fg(muted)),
                    ),
                ),
        )
}
