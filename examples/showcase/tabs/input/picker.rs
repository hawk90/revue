//! Picker widget demos (ColorPicker, FilePicker, DateTimePicker)

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{date_picker, datetime_picker, file_picker, time_picker, ColorPicker};

pub fn examples() -> Vec<Example> {
    let (primary, success, warning, error, info, muted, _text, _) = theme_colors();

    vec![
        Example::new(
            "Color Picker",
            "Color selection with RGB/HEX input and palette",
            Border::rounded().title(" Color Picker ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Select color:").fg(primary))
                    .child(ColorPicker::new().color(primary))
                    .child(Text::new(""))
                    .child(Text::new("Palette selection:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(ColorPicker::new().color(error))
                            .child(ColorPicker::new().color(warning))
                            .child(ColorPicker::new().color(success))
                            .child(ColorPicker::new().color(info))
                            .child(ColorPicker::new().color(primary)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• RGB/HEX input").fg(muted))
                    .child(Text::new("• Color preview").fg(muted))
                    .child(Text::new("• Palette support").fg(muted)),
            ),
        ),
        Example::new(
            "Date Picker",
            "Calendar-based date selection with range support",
            Border::rounded().title(" Date Picker ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Select date:").fg(primary))
                    .child(date_picker())
                    .child(Text::new(""))
                    .child(Text::new("Date range:").fg(primary))
                    .child(
                        hstack()
                            .gap(1)
                            .child(date_picker())
                            .child(Text::new("→"))
                            .child(date_picker()),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Calendar popup").fg(muted))
                    .child(Text::new("• Date range selection").fg(muted))
                    .child(Text::new("• Format options").fg(muted)),
            ),
        ),
        Example::new(
            "Time Picker",
            "Hour and minute selection with 12/24h formats",
            Border::rounded().title(" Time Picker ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Select time:").fg(primary))
                    .child(time_picker())
                    .child(Text::new(""))
                    .child(Text::new("12-hour format:").fg(primary))
                    .child(time_picker().use_24h(false))
                    .child(Text::new(""))
                    .child(Text::new("24-hour format:").fg(primary))
                    .child(time_picker().use_24h(true))
                    .child(Text::new(""))
                    .child(Text::new("• Hour/minute selection").fg(muted))
                    .child(Text::new("• AM/PM toggle").fg(muted))
                    .child(Text::new("• Step increments").fg(muted)),
            ),
        ),
        Example::new(
            "File Picker",
            "File browser with multi-select and type filters",
            Border::rounded().title(" File Picker ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Single file:").fg(primary))
                    .child(file_picker())
                    .child(Text::new(""))
                    .child(Text::new("Multiple files:").fg(primary))
                    .child(file_picker())
                    .child(Text::new(""))
                    .child(Text::new("Filter by type:").fg(primary))
                    .child(file_picker())
                    .child(Text::new(""))
                    .child(Text::new("• File browser dialog").fg(muted))
                    .child(Text::new("• Multi-select support").fg(muted))
                    .child(Text::new("• Type filters").fg(muted)),
            ),
        ),
        Example::new(
            "Date-Time Picker",
            "Combined date and time selection with ISO output",
            Border::rounded().title(" Date-Time Picker ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Combined picker:").fg(primary))
                    .child(datetime_picker())
                    .child(Text::new(""))
                    .child(Text::new("• Date + time combined").fg(muted))
                    .child(Text::new("• ISO format output").fg(muted)),
            ),
        ),
    ]
}
