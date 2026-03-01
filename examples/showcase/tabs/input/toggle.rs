//! Toggle widget demos (Checkbox, Switch)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Checkbox, Switch};

pub fn examples(checkbox_a: bool, switch_a: bool) -> Vec<Example> {
    let (primary, _success, _, _, _info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Checkbox",
            "Binary on/off selection with multiple selections",
            Border::rounded().title(" Checkbox ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Single checkbox:").fg(primary))
                    .child(Checkbox::new("Enable feature A").checked(checkbox_a))
                    .child(Checkbox::new("Enable feature B").checked(!checkbox_a))
                    .child(Text::new(""))
                    .child(Text::new("Checkbox group:").fg(primary))
                    .child(Checkbox::new("Option 1").checked(true))
                    .child(Checkbox::new("Option 2").checked(false))
                    .child(Checkbox::new("Option 3").checked(true))
                    .child(Text::new(""))
                    .child(Text::new("[c] toggle checkbox").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• Binary on/off selection").fg(muted))
                    .child(Text::new("• Multiple selections allowed").fg(muted)),
            ),
        ),
        Example::new(
            "Switch",
            "Toggle switches for settings and preferences",
            Border::rounded().title(" Switch ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Toggle switches:").fg(primary))
                    .child(Switch::new().on(switch_a).label("Dark mode"))
                    .child(Switch::new().on(!switch_a).label("Auto-save"))
                    .child(Switch::new().on(true).label("Notifications"))
                    .child(Switch::new().on(false).label("Analytics"))
                    .child(Text::new(""))
                    .child(Text::new("[s] toggle switch").fg(muted))
                    .child(Text::new(""))
                    .child(Text::new("• On/off toggle").fg(muted))
                    .child(Text::new("• More visual than checkbox").fg(muted))
                    .child(Text::new("• Best for settings").fg(muted)),
            ),
        ),
        Example::new(
            "States",
            "Checkbox and switch states including disabled",
            Border::rounded().title(" States ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Checkbox states:").fg(primary))
                    .child(Checkbox::new("Checked").checked(true))
                    .child(Checkbox::new("Unchecked").checked(false))
                    .child(Checkbox::new("Mixed").checked(true))
                    .child(Checkbox::new("Disabled").checked(true).disabled(true))
                    .child(Text::new(""))
                    .child(Text::new("Switch states:").fg(primary))
                    .child(Switch::new().on(true).label("On"))
                    .child(Switch::new().on(false).label("Off"))
                    .child(Switch::new().on(true).label("Disabled").disabled(true)),
            ),
        ),
    ]
}
