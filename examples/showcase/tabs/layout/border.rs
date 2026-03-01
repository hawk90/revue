//! Border widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, info, muted, text, _) = theme_colors();

    vec![
        Example::new(
            "Border Types",
            "Single, double, thick, and rounded border styles",
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
        Example::new(
            "Border Styles",
            "Borders with different color styles applied",
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
        ),
        Example::new(
            "Border Colors",
            "Themed border colors for visual distinction",
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
        ),
        Example::new(
            "Padding",
            "Content padding within borders",
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
        Example::new(
            "Nested Borders",
            "Borders nested within borders for visual hierarchy",
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
        ),
        Example::new(
            "Borders with Icons",
            "Using emoji icons in border titles",
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
        ),
        Example::new(
            "Border States",
            "Borders representing different visual states",
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
        Example::new(
            "Constrained Border",
            "Borders with min/max width constraints",
            Border::rounded().title(" Constrained Border ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Size-limited borders:").fg(primary))
                    .child(
                        Border::rounded()
                            .title("Min 20w")
                            .min_width(20)
                            .child(Text::new("Minimum width 20").fg(text)),
                    )
                    .child(
                        Border::rounded()
                            .title("Max 30w")
                            .max_width(30)
                            .child(Text::new("Maximum width 30").fg(text)),
                    )
                    .child(
                        Border::rounded()
                            .title("15-25w")
                            .min_width(15)
                            .max_width(25)
                            .child(Text::new("Width range").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• min_width() / max_width()").fg(muted))
                    .child(Text::new("• min_height() / max_height()").fg(muted))
                    .child(Text::new("• Responsive sizing").fg(muted)),
            ),
        ),
        Example::new(
            "Height Constraints",
            "Borders with min/max height constraints",
            Border::rounded().title(" Height Constraints ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Height limits:").fg(primary))
                    .child(
                        Border::rounded()
                            .title("Min 3h")
                            .min_height(3)
                            .child(Text::new("Line 1").fg(text))
                            .child(Text::new("Line 2").fg(text)),
                    )
                    .child(
                        Border::rounded()
                            .title("Max 4h")
                            .max_height(4)
                            .child(Text::new("Limited height").fg(text)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Vertical constraints").fg(muted))
                    .child(Text::new("• Prevent overflow").fg(muted))
                    .child(Text::new("• Fixed height areas").fg(muted)),
            ),
        ),
        Example::new(
            "constrain()",
            "Set all size limits with a single call",
            Border::rounded().title(" constrain() ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("All limits at once:").fg(primary))
                    .child(
                        Border::rounded()
                            .title("20-35w, 2-5h")
                            .constrain(20, 2, 35, 5)
                            .child(Text::new("Constrained border").fg(text))
                            .child(Text::new("Multiple lines").fg(muted)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• .constrain(min_w, min_h, max_w, max_h)").fg(muted))
                    .child(Text::new("• One call, all constraints").fg(muted))
                    .child(Text::new("• Cleaner API").fg(muted)),
            ),
        ),
    ]
}
