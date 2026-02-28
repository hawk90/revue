//! Time-based chart widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{
    BarChart, Candle, CandleChart, Chart, Series, Sparkline, Timeline, TimelineEvent,
};

pub fn render() -> impl View {
    let (primary, success, _warning, error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Time Series ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Stock price:").fg(primary))
                            .child(
                                Chart::new()
                                    .series(
                                        Series::new("Price")
                                            .data_y(&[
                                                150.0, 155.0, 148.0, 160.0, 158.0, 165.0, 162.0,
                                            ])
                                            .line()
                                            .color(primary),
                                    )
                                    .no_legend(),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Time-stamped data").fg(muted))
                            .child(Text::new("• Sequential points").fg(muted))
                            .child(Text::new("• Continuous timeline").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Candle Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("OHLC data:").fg(primary))
                            .child(
                                CandleChart::new(vec![
                                    Candle::new(100.0, 110.0, 95.0, 105.0),
                                    Candle::new(105.0, 115.0, 102.0, 108.0),
                                    Candle::new(108.0, 112.0, 100.0, 102.0),
                                    Candle::new(102.0, 109.0, 98.0, 107.0),
                                    Candle::new(107.0, 120.0, 105.0, 118.0),
                                ])
                                .bullish_color(success)
                                .bearish_color(error),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Open/High/Low/Close").fg(muted))
                            .child(Text::new("• Financial data").fg(muted))
                            .child(Text::new("• Bull/bear colors").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Gantt Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Project timeline:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Design", 30.0)
                                    .bar("Development", 50.0)
                                    .bar("Testing", 40.0)
                                    .bar("Deploy", 20.0)
                                    .fg(primary),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Task scheduling").fg(muted))
                            .child(Text::new("• Duration display").fg(muted))
                            .child(Text::new("• Dependencies").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Timeline ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Events:").fg(primary))
                            .child(
                                Timeline::new()
                                    .event(TimelineEvent::new("Meeting started").timestamp("10:00"))
                                    .event(
                                        TimelineEvent::new("Decision made")
                                            .timestamp("11:30")
                                            .success(),
                                    )
                                    .event(
                                        TimelineEvent::new("Warning issued")
                                            .timestamp("14:00")
                                            .warning(),
                                    )
                                    .event(
                                        TimelineEvent::new("Error detected")
                                            .timestamp("16:00")
                                            .error(),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Event chronology").fg(muted))
                            .child(Text::new("• Status indicators").fg(muted))
                            .child(Text::new("• Timestamped events").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Interval Chart ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Uptime/downtime:").fg(primary))
                            .child(
                                Sparkline::new(vec![
                                    1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.5,
                                    1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                                ])
                                .fg(success),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Status intervals").fg(muted))
                            .child(Text::new("• Availability view").fg(muted))
                            .child(Text::new("• Color coding").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Date Axis ").child(
                        vstack()
                            .gap(1)
                            .child(Text::new("Weekly view:").fg(primary))
                            .child(
                                BarChart::new()
                                    .bar("Mon", 45.0)
                                    .bar("Tue", 52.0)
                                    .bar("Wed", 48.0)
                                    .bar("Thu", 60.0)
                                    .bar("Fri", 55.0)
                                    .bar("Sat", 70.0)
                                    .bar("Sun", 68.0)
                                    .fg(primary),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Date-based x-axis").fg(muted))
                            .child(Text::new("• Automatic formatting").fg(muted))
                            .child(Text::new("• Time intervals").fg(muted)),
                    ),
                ),
        )
}
