//! Table widget demos

use crate::theme_colors;
use revue::prelude::*;
use revue::widget::{Column, DataGrid, GridColumn, GridRow, Pagination, Table};

pub fn render() -> impl View {
    let (_primary, _success, _warning, _error, _info, muted, _text, _) = theme_colors();

    vstack()
        .gap(2)
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Table ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Table::new(vec![
                                    Column::new("ID"),
                                    Column::new("Name"),
                                    Column::new("Status"),
                                    Column::new("CPU"),
                                ])
                                .row(vec!["001", "web-server", "●", "42%"])
                                .row(vec!["002", "api-gateway", "●", "67%"])
                                .row(vec!["003", "database", "●", "89%"])
                                .row(vec!["004", "cache", "○", "0%"])
                                .row(vec!["005", "worker", "●", "23%"]),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Column headers").fg(muted))
                            .child(Text::new("• Row data").fg(muted))
                            .child(Text::new("• Sortable columns").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Data Grid ").child(
                        vstack()
                            .gap(1)
                            .child(
                                DataGrid::new()
                                    .column(GridColumn::new("Product", "Product"))
                                    .column(GridColumn::new("Q1", "Q1"))
                                    .column(GridColumn::new("Q2", "Q2"))
                                    .column(GridColumn::new("Q3", "Q3"))
                                    .column(GridColumn::new("Q4", "Q4"))
                                    .header(true)
                                    .row(
                                        GridRow::new()
                                            .cell("Product", "Widget A")
                                            .cell("Q1", "120")
                                            .cell("Q2", "145")
                                            .cell("Q3", "138")
                                            .cell("Q4", "160"),
                                    )
                                    .row(
                                        GridRow::new()
                                            .cell("Product", "Widget B")
                                            .cell("Q1", "85")
                                            .cell("Q2", "92")
                                            .cell("Q3", "105")
                                            .cell("Q4", "98"),
                                    )
                                    .row(
                                        GridRow::new()
                                            .cell("Product", "Widget C")
                                            .cell("Q1", "200")
                                            .cell("Q2", "180")
                                            .cell("Q3", "220")
                                            .cell("Q4", "250"),
                                    )
                                    .row(
                                        GridRow::new()
                                            .cell("Product", "Widget D")
                                            .cell("Q1", "45")
                                            .cell("Q2", "52")
                                            .cell("Q3", "48")
                                            .cell("Q4", "55"),
                                    ),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Editable cells").fg(muted))
                            .child(Text::new("• Row selection").fg(muted))
                            .child(Text::new("• Spreadsheet-like").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Styled Table ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Table::new(vec![
                                    Column::new("Service").width(12),
                                    Column::new("Status").width(8),
                                    Column::new("Health").width(8),
                                ])
                                .row(vec!["API", "Running", "98%"])
                                .row(vec!["Database", "Running", "76%"])
                                .row(vec!["Cache", "Stopped", "0%"]),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Colored cells").fg(muted))
                            .child(Text::new("• Conditional styling").fg(muted))
                            .child(Text::new("• Status indicators").fg(muted)),
                    ),
                ),
        )
        .child(
            hstack()
                .gap(3)
                .child(
                    Border::rounded().title(" Paginated ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Table::new(vec![
                                    Column::new("#"),
                                    Column::new("Item"),
                                    Column::new("Value"),
                                ])
                                .row(vec!["1", "Alpha", "$120"])
                                .row(vec!["2", "Beta", "$85"])
                                .row(vec!["3", "Gamma", "$200"])
                                .row(vec!["4", "Delta", "$45"])
                                .row(vec!["5", "Epsilon", "$95"]),
                            )
                            .child(Pagination::new(10).current(1))
                            .child(Text::new(""))
                            .child(Text::new("• Large datasets").fg(muted))
                            .child(Text::new("• Navigation controls").fg(muted))
                            .child(Text::new("• Page size options").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Sortable Table ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Table::new(vec![
                                    Column::new("Name"),
                                    Column::new("Date"),
                                    Column::new("Size"),
                                ])
                                .row(vec!["config.toml", "2026-01-15", "2.1 KB"])
                                .row(vec!["main.rs", "2026-02-20", "15.3 KB"])
                                .row(vec![
                                    "data.json",
                                    "2026-02-28",
                                    "8.7 KB",
                                ]),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Click headers to sort").fg(muted))
                            .child(Text::new("• Ascending/descending").fg(muted))
                            .child(Text::new("• Sort indicator").fg(muted)),
                    ),
                )
                .child(
                    Border::rounded().title(" Selection ").child(
                        vstack()
                            .gap(1)
                            .child(
                                Table::new(vec![
                                    Column::new(""),
                                    Column::new("Task"),
                                    Column::new("Due"),
                                ])
                                .row(vec!["☐", "Write docs", "2026-03-01"])
                                .row(vec!["☑", "Fix bugs", "2026-03-02"])
                                .row(vec!["☐", "Review PR", "2026-03-03"])
                                .row(vec![
                                    "☑",
                                    "Deploy",
                                    "2026-03-05",
                                ]),
                            )
                            .child(Text::new(""))
                            .child(Text::new("• Row selection").fg(muted))
                            .child(Text::new("• Multi-select option").fg(muted))
                            .child(Text::new("• Checkbox column").fg(muted)),
                    ),
                ),
        )
}
