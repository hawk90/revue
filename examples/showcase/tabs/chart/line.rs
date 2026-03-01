//! Line chart widget demos

use crate::example::Example;
use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Chart, ScatterChart, ScatterSeries, Series};

pub fn examples(frame: u64) -> Vec<Example> {
    let (primary, success, warning, _error, info, muted, _text, _) = theme_colors();

    let line_data: Vec<f64> = (0..25)
        .map(|i| {
            let x = i as f64 * 0.3 + frame as f64 * 0.02;
            50.0 + 30.0 * x.sin() + 10.0 * (x * 2.3).cos()
        })
        .collect();

    let trend_data: Vec<f64> = (0..20)
        .map(|i| {
            let x = i as f64;
            20.0 + x * 3.5 + (x * 0.5).sin() * 5.0
        })
        .collect();

    vec![
        Example::new(
            "Line Chart",
            "Time series trend analysis with smooth curves",
            Border::rounded().title(" Line Chart ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Trend Analysis:").fg(primary))
                    .child(
                        Chart::new()
                            .series(Series::new("Trend").data_y(&line_data).line().color(info)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Time series data").fg(muted))
                    .child(Text::new("• Trend visualization").fg(muted))
                    .child(Text::new("• Smooth curves").fg(muted)),
            ),
        ),
        Example::new(
            "Multi-Series",
            "Comparing multiple metrics on a single chart",
            Border::rounded().title(" Multi-Series ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Comparing metrics:").fg(primary))
                    .child(
                        Chart::new()
                            .series(
                                Series::new("Series A")
                                    .data_y(&line_data)
                                    .line()
                                    .color(success),
                            )
                            .series(
                                Series::new("Series B")
                                    .data_y(&trend_data)
                                    .line()
                                    .color(warning),
                            ),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Multiple lines").fg(muted))
                    .child(Text::new("• Color legend").fg(muted))
                    .child(Text::new("• Comparison view").fg(muted)),
            ),
        ),
        Example::new(
            "Area Chart",
            "Filled area chart for volume display",
            Border::rounded().title(" Area Chart ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Filled area:").fg(primary))
                    .child(
                        Chart::new().series(Series::new("Area").data_y(&line_data).area(primary)),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Filled visualization").fg(muted))
                    .child(Text::new("• Volume display").fg(muted))
                    .child(Text::new("• Gradient fill").fg(muted)),
            ),
        ),
        Example::new(
            "Scatter Plot",
            "Data point distribution and correlation view",
            Border::rounded().title(" Scatter Plot ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Data points:").fg(primary))
                    .child(
                        ScatterChart::new()
                            .series(
                                ScatterSeries::new("Data")
                                    .points(&[
                                        (10.0, 20.0),
                                        (15.0, 35.0),
                                        (25.0, 30.0),
                                        (30.0, 45.0),
                                        (40.0, 40.0),
                                        (50.0, 55.0),
                                        (55.0, 50.0),
                                        (65.0, 70.0),
                                        (75.0, 65.0),
                                        (85.0, 80.0),
                                    ])
                                    .color(primary),
                            )
                            .no_legend(),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Point distribution").fg(muted))
                    .child(Text::new("• Correlation view").fg(muted))
                    .child(Text::new("• Cluster analysis").fg(muted)),
            ),
        ),
        Example::new(
            "Step Chart",
            "Discrete step changes for inventory and level data",
            Border::rounded().title(" Step Chart ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("Step progression:").fg(primary))
                    .child(
                        Chart::new()
                            .series(
                                Series::new("Steps")
                                    .data_y(&[10.0, 10.0, 25.0, 25.0, 40.0, 40.0, 35.0, 35.0])
                                    .step()
                                    .color(info),
                            )
                            .no_legend(),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Discrete changes").fg(muted))
                    .child(Text::new("• Level transitions").fg(muted))
                    .child(Text::new("• Inventory levels").fg(muted)),
            ),
        ),
        Example::new(
            "Trend Line",
            "Data with overlaid trend line for forecasting",
            Border::rounded().title(" Trend Line ").child(
                vstack()
                    .gap(1)
                    .child(Text::new("With trend:").fg(primary))
                    .child(
                        Chart::new()
                            .series(Series::new("Data").data_y(&line_data).line().color(muted))
                            .series(
                                Series::new("Trend")
                                    .data_y(&trend_data)
                                    .line()
                                    .color(success),
                            )
                            .no_legend(),
                    )
                    .child(Text::new(""))
                    .child(Text::new("• Data + trend").fg(muted))
                    .child(Text::new("• Linear regression").fg(muted))
                    .child(Text::new("• Forecast support").fg(muted)),
            ),
        ),
    ]
}
