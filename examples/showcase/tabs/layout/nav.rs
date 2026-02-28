//! Navigation widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{
    breadcrumb, step, stepper, Breadcrumb, Pagination, Sidebar, SidebarItem, Stepper,
};

pub fn render() -> impl View {
    let (primary, _success, _warning, _error, _info, muted, text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Tabs Widget ")
                        .min_width(30)
                        .min_height(10)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Tab navigation").fg(primary))
                                .child(
                                    Tabs::new()
                                        .tab("Tab A")
                                        .tab("Tab B")
                                        .tab("Tab C")
                                        .selected(0),
                                )
                                .child(Text::new("Content of Tab A").fg(text))
                                .child(Text::new(""))
                                .child(Text::new("• Selection highlight").fg(muted))
                                .child(Text::new("• Keyboard support").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Breadcrumb ")
                        .min_width(35)
                        .min_height(10)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Navigation path:").fg(primary))
                                .child(
                                    Breadcrumb::new()
                                        .push("Home")
                                        .push("Projects")
                                        .push("Revue")
                                        .push("Widgets"),
                                )
                                .child(Text::new(""))
                                .child(Text::new("With chevron:").fg(primary))
                                .child(breadcrumb().path("/Settings/Appearance/Theme"))
                                .child(Text::new(""))
                                .child(Text::new("• Path display").fg(muted))
                                .child(Text::new("• Click navigation").fg(muted))
                                .child(Text::new("• Custom separators").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Stepper Widget ")
                        .min_width(35)
                        .min_height(10)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Wizard steps:").fg(primary))
                                .child(
                                    Stepper::new()
                                        .add_step("Setup")
                                        .add_step("Configure")
                                        .add_step("Build")
                                        .add_step("Deploy")
                                        .current(1),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Step indicator").fg(muted))
                                .child(Text::new("• Progress display").fg(muted))
                                .child(Text::new("• Wizard pattern").fg(muted)),
                        ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded()
                        .title(" Stepper Variants ")
                        .min_width(35)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Completed steps:").fg(primary))
                                .child(
                                    stepper()
                                        .step(step("Account").complete())
                                        .step(step("Profile").complete())
                                        .step(step("Settings").active())
                                        .step(step("Finish"))
                                        .current(2),
                                )
                                .child(Text::new(""))
                                .child(Text::new("Vertical stepper:").fg(primary))
                                .child(
                                    stepper()
                                        .add_step("Step 1: Initialize")
                                        .add_step("Step 2: Process")
                                        .add_step("Step 3: Complete")
                                        .current(1)
                                        .vertical(),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Progress tracking").fg(muted))
                                .child(Text::new("• Status indicators").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Pagination ")
                        .min_width(30)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Page navigation:").fg(primary))
                                .child(Pagination::new(20).current(5))
                                .child(Text::new(""))
                                .child(Text::new("First page:").fg(primary))
                                .child(Pagination::new(100).current(1))
                                .child(Text::new(""))
                                .child(Text::new("Last page:").fg(primary))
                                .child(Pagination::new(50).current(50))
                                .child(Text::new(""))
                                .child(Text::new("• Previous/Next").fg(muted))
                                .child(Text::new("• Page numbers").fg(muted))
                                .child(Text::new("• Current page indicator").fg(muted)),
                        ),
                )
                .child(
                    Border::rounded()
                        .title(" Sidebar Widget ")
                        .min_width(25)
                        .min_height(14)
                        .child(
                            vstack()
                                .gap(1)
                                .child(Text::new("Vertical navigation:").fg(primary))
                                .child(
                                    Sidebar::new()
                                        .header("MENU")
                                        .items(vec![
                                            SidebarItem::new("dashboard", "Dashboard"),
                                            SidebarItem::new("projects", "Projects"),
                                            SidebarItem::new("settings", "Settings"),
                                        ])
                                        .selected("dashboard")
                                        .expanded_width(18),
                                )
                                .child(Text::new(""))
                                .child(Text::new("• Vertical nav rail").fg(muted))
                                .child(Text::new("• Icon support").fg(muted))
                                .child(Text::new("• Collapsible").fg(muted)),
                        ),
                ),
        )
}
