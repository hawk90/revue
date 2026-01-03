//! Data Explorer Example - Demonstrates DataGrid, Notifications, StatusBar
//!
//! A data exploration tool showing filtering, sorting, and data visualization.
//!
//! Run with: cargo run --example data_explorer

use revue::prelude::*;

/// Sample data record
#[derive(Clone)]
struct Record {
    id: u32,
    name: String,
    email: String,
    department: String,
    salary: u32,
    status: Status,
}

#[derive(Clone, Copy, PartialEq)]
enum Status {
    Active,
    Inactive,
    Pending,
}

impl Status {
    fn name(&self) -> &str {
        match self {
            Status::Active => "Active",
            Status::Inactive => "Inactive",
            Status::Pending => "Pending",
        }
    }

    fn color(&self) -> Color {
        match self {
            Status::Active => Color::GREEN,
            Status::Inactive => Color::RED,
            Status::Pending => Color::YELLOW,
        }
    }
}

/// Sort direction
#[derive(Clone, Copy, PartialEq)]
enum SortDir {
    Asc,
    Desc,
}

/// Sortable column
#[derive(Clone, Copy, PartialEq, Debug)]
enum SortColumn {
    Id,
    Name,
    Department,
    Salary,
}

/// Notification message
struct Notification {
    message: String,
    level: NotificationLevel,
    ttl: u8, // Time to live in render cycles
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum NotificationLevel {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationLevel {
    fn color(&self) -> Color {
        match self {
            NotificationLevel::Info => Color::BLUE,
            NotificationLevel::Success => Color::GREEN,
            NotificationLevel::Warning => Color::YELLOW,
            NotificationLevel::Error => Color::RED,
        }
    }

    fn icon(&self) -> &str {
        match self {
            NotificationLevel::Info => "i",
            NotificationLevel::Success => "v",
            NotificationLevel::Warning => "!",
            NotificationLevel::Error => "x",
        }
    }
}

/// Data explorer application state
struct DataExplorer {
    /// All records
    records: Vec<Record>,
    /// Filtered and sorted records (indices into records)
    view: Vec<usize>,
    /// Selected row index
    selected: usize,
    /// Filter text
    filter: String,
    /// Filter mode active
    filter_mode: bool,
    /// Sort column
    sort_column: SortColumn,
    /// Sort direction
    sort_dir: SortDir,
    /// Notifications
    notifications: Vec<Notification>,
    /// Detail view open
    detail_open: bool,
    /// Statistics
    stats: Stats,
}

struct Stats {
    total: usize,
    active: usize,
    inactive: usize,
    pending: usize,
    avg_salary: f32,
    max_salary: u32,
}

impl DataExplorer {
    fn new() -> Self {
        let records = vec![
            Record {
                id: 1,
                name: "Alice Johnson".into(),
                email: "alice@example.com".into(),
                department: "Engineering".into(),
                salary: 95000,
                status: Status::Active,
            },
            Record {
                id: 2,
                name: "Bob Smith".into(),
                email: "bob@example.com".into(),
                department: "Marketing".into(),
                salary: 72000,
                status: Status::Active,
            },
            Record {
                id: 3,
                name: "Carol Williams".into(),
                email: "carol@example.com".into(),
                department: "Engineering".into(),
                salary: 88000,
                status: Status::Pending,
            },
            Record {
                id: 4,
                name: "David Brown".into(),
                email: "david@example.com".into(),
                department: "Sales".into(),
                salary: 65000,
                status: Status::Active,
            },
            Record {
                id: 5,
                name: "Eva Martinez".into(),
                email: "eva@example.com".into(),
                department: "Engineering".into(),
                salary: 105000,
                status: Status::Active,
            },
            Record {
                id: 6,
                name: "Frank Garcia".into(),
                email: "frank@example.com".into(),
                department: "HR".into(),
                salary: 58000,
                status: Status::Inactive,
            },
            Record {
                id: 7,
                name: "Grace Lee".into(),
                email: "grace@example.com".into(),
                department: "Engineering".into(),
                salary: 92000,
                status: Status::Active,
            },
            Record {
                id: 8,
                name: "Henry Wilson".into(),
                email: "henry@example.com".into(),
                department: "Sales".into(),
                salary: 78000,
                status: Status::Pending,
            },
            Record {
                id: 9,
                name: "Ivy Chen".into(),
                email: "ivy@example.com".into(),
                department: "Marketing".into(),
                salary: 68000,
                status: Status::Active,
            },
            Record {
                id: 10,
                name: "Jack Taylor".into(),
                email: "jack@example.com".into(),
                department: "Engineering".into(),
                salary: 98000,
                status: Status::Active,
            },
            Record {
                id: 11,
                name: "Kate Anderson".into(),
                email: "kate@example.com".into(),
                department: "HR".into(),
                salary: 62000,
                status: Status::Active,
            },
            Record {
                id: 12,
                name: "Leo Thomas".into(),
                email: "leo@example.com".into(),
                department: "Sales".into(),
                salary: 71000,
                status: Status::Inactive,
            },
            Record {
                id: 13,
                name: "Mia Jackson".into(),
                email: "mia@example.com".into(),
                department: "Engineering".into(),
                salary: 115000,
                status: Status::Active,
            },
            Record {
                id: 14,
                name: "Noah White".into(),
                email: "noah@example.com".into(),
                department: "Marketing".into(),
                salary: 75000,
                status: Status::Pending,
            },
            Record {
                id: 15,
                name: "Olivia Harris".into(),
                email: "olivia@example.com".into(),
                department: "Engineering".into(),
                salary: 101000,
                status: Status::Active,
            },
        ];

        let view: Vec<usize> = (0..records.len()).collect();
        let stats = Self::calculate_stats(&records, &view);

        let mut explorer = Self {
            records,
            view,
            selected: 0,
            filter: String::new(),
            filter_mode: false,
            sort_column: SortColumn::Id,
            sort_dir: SortDir::Asc,
            notifications: Vec::new(),
            detail_open: false,
            stats,
        };

        explorer.add_notification("Welcome to Data Explorer!", NotificationLevel::Info);
        explorer
    }

    fn calculate_stats(records: &[Record], view: &[usize]) -> Stats {
        let filtered: Vec<&Record> = view.iter().map(|&i| &records[i]).collect();
        let total = filtered.len();
        let active = filtered
            .iter()
            .filter(|r| r.status == Status::Active)
            .count();
        let inactive = filtered
            .iter()
            .filter(|r| r.status == Status::Inactive)
            .count();
        let pending = filtered
            .iter()
            .filter(|r| r.status == Status::Pending)
            .count();
        let total_salary: u32 = filtered.iter().map(|r| r.salary).sum();
        let avg_salary = if total > 0 {
            total_salary as f32 / total as f32
        } else {
            0.0
        };
        let max_salary = filtered.iter().map(|r| r.salary).max().unwrap_or(0);

        Stats {
            total,
            active,
            inactive,
            pending,
            avg_salary,
            max_salary,
        }
    }

    fn apply_filter(&mut self) {
        let filter_lower = self.filter.to_lowercase();
        self.view = self
            .records
            .iter()
            .enumerate()
            .filter(|(_, r)| {
                if filter_lower.is_empty() {
                    return true;
                }
                r.name.to_lowercase().contains(&filter_lower)
                    || r.email.to_lowercase().contains(&filter_lower)
                    || r.department.to_lowercase().contains(&filter_lower)
            })
            .map(|(i, _)| i)
            .collect();

        self.apply_sort();
        self.stats = Self::calculate_stats(&self.records, &self.view);
        self.selected = 0;
    }

    fn apply_sort(&mut self) {
        let records = &self.records;
        let sort_column = self.sort_column;
        let sort_dir = self.sort_dir;

        self.view.sort_by(|&a, &b| {
            let cmp = match sort_column {
                SortColumn::Id => records[a].id.cmp(&records[b].id),
                SortColumn::Name => records[a].name.cmp(&records[b].name),
                SortColumn::Department => records[a].department.cmp(&records[b].department),
                SortColumn::Salary => records[a].salary.cmp(&records[b].salary),
            };
            if sort_dir == SortDir::Desc {
                cmp.reverse()
            } else {
                cmp
            }
        });
    }

    fn toggle_sort(&mut self, column: SortColumn) {
        if self.sort_column == column {
            self.sort_dir = if self.sort_dir == SortDir::Asc {
                SortDir::Desc
            } else {
                SortDir::Asc
            };
        } else {
            self.sort_column = column;
            self.sort_dir = SortDir::Asc;
        }
        self.apply_sort();

        let dir = if self.sort_dir == SortDir::Asc {
            "ascending"
        } else {
            "descending"
        };
        self.add_notification(
            format!("Sorted by {:?} ({})", column, dir),
            NotificationLevel::Info,
        );
    }

    fn add_notification(&mut self, message: impl Into<String>, level: NotificationLevel) {
        self.notifications.push(Notification {
            message: message.into(),
            level,
            ttl: 30,
        });

        // Keep only last 5 notifications
        if self.notifications.len() > 5 {
            self.notifications.remove(0);
        }
    }

    fn update_notifications(&mut self) {
        self.notifications.retain_mut(|n| {
            n.ttl = n.ttl.saturating_sub(1);
            n.ttl > 0
        });
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        // Handle filter mode
        if self.filter_mode {
            match key {
                Key::Escape => {
                    self.filter_mode = false;
                    return true;
                }
                Key::Enter => {
                    self.filter_mode = false;
                    self.apply_filter();
                    let count = self.view.len();
                    self.add_notification(
                        format!("Filter applied: {} records found", count),
                        if count > 0 {
                            NotificationLevel::Success
                        } else {
                            NotificationLevel::Warning
                        },
                    );
                    return true;
                }
                Key::Char(c) => {
                    self.filter.push(*c);
                    self.apply_filter();
                    return true;
                }
                Key::Backspace => {
                    self.filter.pop();
                    self.apply_filter();
                    return true;
                }
                _ => return false,
            }
        }

        // Handle detail view
        if self.detail_open {
            if matches!(key, Key::Escape | Key::Enter) {
                self.detail_open = false;
                return true;
            }
            return false;
        }

        // Normal mode
        match key {
            Key::Up | Key::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.selected < self.view.len().saturating_sub(1) {
                    self.selected += 1;
                }
                true
            }
            Key::Enter => {
                self.detail_open = true;
                if let Some(&idx) = self.view.get(self.selected) {
                    self.add_notification(
                        format!("Viewing: {}", self.records[idx].name),
                        NotificationLevel::Info,
                    );
                }
                true
            }
            Key::Char('/') | Key::Char('f') => {
                self.filter_mode = true;
                self.add_notification("Filter mode: type to filter", NotificationLevel::Info);
                true
            }
            Key::Char('c') => {
                self.filter.clear();
                self.apply_filter();
                self.add_notification("Filter cleared", NotificationLevel::Success);
                true
            }
            Key::Char('1') => {
                self.toggle_sort(SortColumn::Id);
                true
            }
            Key::Char('2') => {
                self.toggle_sort(SortColumn::Name);
                true
            }
            Key::Char('3') => {
                self.toggle_sort(SortColumn::Department);
                true
            }
            Key::Char('4') => {
                self.toggle_sort(SortColumn::Salary);
                true
            }
            Key::Char('d') => {
                if let Some(&idx) = self.view.get(self.selected) {
                    let name = self.records[idx].name.clone();
                    self.add_notification(format!("Deleted: {}", name), NotificationLevel::Warning);
                    // Don't actually delete, just show notification
                }
                true
            }
            Key::Char('e') => {
                if let Some(&idx) = self.view.get(self.selected) {
                    let name = self.records[idx].name.clone();
                    self.add_notification(
                        format!("Exported: {}", name),
                        NotificationLevel::Success,
                    );
                }
                true
            }
            _ => false,
        }
    }

    fn render_header(&self) -> impl View {
        hstack()
            .child(Text::new(" Data Explorer ").fg(Color::CYAN).bold())
            .child(
                Text::new(format!(
                    " | {} records | Filter: '{}' ",
                    self.stats.total,
                    if self.filter.is_empty() {
                        "<none>"
                    } else {
                        &self.filter
                    }
                ))
                .fg(Color::rgb(128, 128, 128)),
            )
    }

    fn render_stats(&self) -> impl View {
        let active = Text::new(format!(" Active: {} ", self.stats.active)).fg(Color::GREEN);
        let inactive = Text::new(format!(" Inactive: {} ", self.stats.inactive)).fg(Color::RED);
        let pending = Text::new(format!(" Pending: {} ", self.stats.pending)).fg(Color::YELLOW);
        let avg = Text::new(format!(" Avg Salary: ${:.0} ", self.stats.avg_salary)).fg(Color::CYAN);
        let max = Text::new(format!(" Max: ${} ", self.stats.max_salary)).fg(Color::MAGENTA);

        Border::rounded().title("Statistics").child(
            hstack()
                .gap(2)
                .child(active)
                .child(inactive)
                .child(pending)
                .child(avg)
                .child(max),
        )
    }

    fn render_table(&self) -> impl View {
        let sort_indicator = |col: SortColumn| -> &str {
            if self.sort_column == col {
                if self.sort_dir == SortDir::Asc {
                    " ^"
                } else {
                    " v"
                }
            } else {
                ""
            }
        };

        // Header row
        let header = hstack()
            .child(
                Text::new(format!("{:>4} ID{}", "", sort_indicator(SortColumn::Id)))
                    .bold()
                    .fg(Color::CYAN),
            )
            .child(
                Text::new(format!(
                    "{:<20} Name{}",
                    "",
                    sort_indicator(SortColumn::Name)
                ))
                .bold()
                .fg(Color::CYAN),
            )
            .child(
                Text::new(format!(
                    "{:<15} Dept{}",
                    "",
                    sort_indicator(SortColumn::Department)
                ))
                .bold()
                .fg(Color::CYAN),
            )
            .child(
                Text::new(format!(
                    "{:>10} Salary{}",
                    "",
                    sort_indicator(SortColumn::Salary)
                ))
                .bold()
                .fg(Color::CYAN),
            )
            .child(Text::new("  Status").bold().fg(Color::CYAN));

        let mut table = vstack().child(header);
        table = table.child(Text::new("─".repeat(80)).fg(Color::rgb(80, 80, 80)));

        // Data rows (show up to 10)
        let start = self.selected.saturating_sub(5);
        let visible: Vec<_> = self.view.iter().skip(start).take(10).collect();

        for (offset, &&idx) in visible.iter().enumerate() {
            let record = &self.records[idx];
            let row_idx = start + offset;
            let is_selected = row_idx == self.selected;

            let id_text = format!("{:>4}", record.id);
            let name_text = format!("{:<20}", truncate(&record.name, 18));
            let dept_text = format!("{:<15}", record.department);
            let salary_text = format!("${:>9}", record.salary);
            let status_text = format!("  {}", record.status.name());

            let row = hstack()
                .child(Text::new(&id_text))
                .child(Text::new(&name_text))
                .child(Text::new(&dept_text))
                .child(Text::new(&salary_text))
                .child(Text::new(&status_text).fg(record.status.color()));

            let row = if is_selected {
                hstack().child(Text::new("> ").fg(Color::CYAN)).child(row)
            } else {
                hstack().child(Text::new("  ")).child(row)
            };

            table = table.child(row);
        }

        Border::rounded().title("Records").child(table)
    }

    fn render_detail(&self) -> Stack {
        if !self.detail_open {
            return vstack();
        }

        let Some(&idx) = self.view.get(self.selected) else {
            return vstack();
        };

        let record = &self.records[idx];

        let content = vstack()
            .child(Text::new(format!("ID:         {}", record.id)))
            .child(Text::new(format!("Name:       {}", record.name)))
            .child(Text::new(format!("Email:      {}", record.email)))
            .child(Text::new(format!("Department: {}", record.department)))
            .child(Text::new(format!("Salary:     ${}", record.salary)))
            .child(
                hstack()
                    .child(Text::new("Status:     "))
                    .child(Text::new(record.status.name()).fg(record.status.color())),
            )
            .child(Text::new(""))
            .child(Text::new("Press Enter or Escape to close").fg(Color::rgb(100, 100, 100)));

        vstack().child(
            Border::double()
                .title(format!("Detail: {}", record.name))
                .child(content),
        )
    }

    fn render_filter_input(&self) -> impl View {
        if !self.filter_mode {
            return hstack();
        }

        hstack()
            .child(Text::new(" Filter: ").fg(Color::YELLOW))
            .child(Text::new(&self.filter).fg(Color::WHITE))
            .child(Text::new("_").fg(Color::WHITE)) // Cursor
    }

    fn render_notifications(&self) -> Stack {
        let mut stack = vstack();

        for notif in &self.notifications {
            let icon = Text::new(format!("[{}]", notif.level.icon())).fg(notif.level.color());
            let msg = Text::new(format!(" {}", notif.message));

            stack = stack.child(hstack().child(icon).child(msg));
        }

        if self.notifications.is_empty() {
            stack
        } else {
            vstack().child(Border::rounded().title("Notifications").child(stack))
        }
    }

    fn render_help(&self) -> impl View {
        Text::new(
            "j/k: Navigate | Enter: Detail | f: Filter | c: Clear | 1-4: Sort | e: Export | d: Delete | q: Quit"
        ).fg(Color::rgb(100, 100, 100))
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}...", &s[..max_len.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

impl View for DataExplorer {
    fn render(&self, ctx: &mut RenderContext) {
        let main = vstack()
            .child(self.render_header())
            .child(self.render_stats())
            .child(self.render_filter_input())
            .child(self.render_table())
            .child(self.render_notifications())
            .child(self.render_help());

        main.render(ctx);

        // Overlay detail view
        if self.detail_open {
            self.render_detail().render(ctx);
        }
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let explorer = DataExplorer::new();

    app.run_with_handler(explorer, |key_event, explorer| {
        explorer.update_notifications();
        explorer.handle_key(&key_event.key)
    })
}
