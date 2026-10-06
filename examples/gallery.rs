//! Widget Gallery - Interactive showcase of all Revue widgets
//!
//! Run with: cargo run --example gallery
//!
//! Navigate with:
//! - Tab/Shift+Tab: Switch categories
//! - Up/Down: Navigate widgets
//! - Enter: Toggle widget demo
//! - q: Quit

use revue::prelude::*;
use revue::utils::unicode::display_width;
use revue::widget::*;

/// Columns `text` occupies, for sizing single-line text in an `hstack`.
fn cols(text: &str) -> u16 {
    display_width(text) as u16
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let gallery = Gallery::new();

    app.run_with_handler(gallery, |event: &KeyEvent, state: &mut Gallery| {
        state.handle_event(event)
    })
}

// =============================================================================
// Gallery State
// =============================================================================

struct Gallery {
    /// Current category index
    category: usize,
    /// Current widget index within category
    widget_idx: usize,
    /// Categories with their widgets
    categories: Vec<Category>,
}

struct Category {
    name: &'static str,
    icon: &'static str,
    widgets: Vec<WidgetDemo>,
}

struct WidgetDemo {
    name: &'static str,
    description: &'static str,
}

impl Gallery {
    fn new() -> Self {
        Self {
            category: 0,
            widget_idx: 0,
            categories: vec![
                Category {
                    name: "Basic",
                    icon: "□",
                    widgets: vec![
                        WidgetDemo {
                            name: "Text",
                            description: "Display styled text content",
                        },
                        WidgetDemo {
                            name: "Button",
                            description: "Clickable button with states",
                        },
                        WidgetDemo {
                            name: "Input",
                            description: "Text input field",
                        },
                        WidgetDemo {
                            name: "Checkbox",
                            description: "Boolean toggle checkbox",
                        },
                        WidgetDemo {
                            name: "Radio",
                            description: "Single selection from options",
                        },
                        WidgetDemo {
                            name: "Switch",
                            description: "iOS-style toggle switch",
                        },
                    ],
                },
                Category {
                    name: "Layout",
                    icon: "⊞",
                    widgets: vec![
                        WidgetDemo {
                            name: "VStack",
                            description: "Vertical stack layout",
                        },
                        WidgetDemo {
                            name: "HStack",
                            description: "Horizontal stack layout",
                        },
                        WidgetDemo {
                            name: "Grid",
                            description: "CSS Grid layout",
                        },
                        WidgetDemo {
                            name: "Splitter",
                            description: "Resizable split panes",
                        },
                        WidgetDemo {
                            name: "Scroll",
                            description: "Scrollable content area",
                        },
                        WidgetDemo {
                            name: "Tabs",
                            description: "Tabbed content panels",
                        },
                    ],
                },
                Category {
                    name: "Data",
                    icon: "▤",
                    widgets: vec![
                        WidgetDemo {
                            name: "Table",
                            description: "Data table with sorting",
                        },
                        WidgetDemo {
                            name: "List",
                            description: "Scrollable item list",
                        },
                        WidgetDemo {
                            name: "Tree",
                            description: "Hierarchical tree view",
                        },
                        WidgetDemo {
                            name: "DataGrid",
                            description: "Editable data grid",
                        },
                        WidgetDemo {
                            name: "VirtualList",
                            description: "Virtualized large list",
                        },
                    ],
                },
                Category {
                    name: "Charts",
                    icon: "▁",
                    widgets: vec![
                        WidgetDemo {
                            name: "BarChart",
                            description: "Vertical/horizontal bars",
                        },
                        WidgetDemo {
                            name: "Sparkline",
                            description: "Inline mini chart",
                        },
                        WidgetDemo {
                            name: "Gauge",
                            description: "Circular gauge meter",
                        },
                        WidgetDemo {
                            name: "Heatmap",
                            description: "2D color intensity map",
                        },
                    ],
                },
                Category {
                    name: "Feedback",
                    icon: "◐",
                    widgets: vec![
                        WidgetDemo {
                            name: "Progress",
                            description: "Progress bar indicator",
                        },
                        WidgetDemo {
                            name: "Spinner",
                            description: "Loading spinner",
                        },
                        WidgetDemo {
                            name: "Toast",
                            description: "Notification toast",
                        },
                        WidgetDemo {
                            name: "Modal",
                            description: "Modal dialog overlay",
                        },
                        WidgetDemo {
                            name: "Skeleton",
                            description: "Loading placeholder",
                        },
                    ],
                },
            ],
        }
    }

    fn handle_event(&mut self, event: &KeyEvent) -> bool {
        match event.key {
            Key::Char('q') | Key::Escape => return false,
            Key::Tab => {
                if event.shift {
                    self.category = self.category.saturating_sub(1);
                } else {
                    self.category = (self.category + 1).min(self.categories.len() - 1);
                }
                self.widget_idx = 0;
            }
            Key::Up => {
                self.widget_idx = self.widget_idx.saturating_sub(1);
            }
            Key::Down => {
                let max = self.categories[self.category]
                    .widgets
                    .len()
                    .saturating_sub(1);
                self.widget_idx = (self.widget_idx + 1).min(max);
            }
            _ => {}
        }
        true
    }

    fn current_widget(&self) -> Option<&WidgetDemo> {
        self.categories
            .get(self.category)
            .and_then(|c| c.widgets.get(self.widget_idx))
    }
}

impl View for Gallery {
    fn render(&self, ctx: &mut RenderContext) {
        let cat = &self.categories[self.category];

        // Main layout. Fixed-height rows are `child_sized`; the content row and
        // its two panels take the rest with `child_flex` - a stack would
        // otherwise size them to their content.
        vstack()
            .gap(1)
            // Header
            .child_sized(
                vstack()
                    .child_sized(Text::new("Revue Widget Gallery").bold(), 1)
                    .child_sized(Text::muted("Interactive showcase of 70+ widgets"), 1),
                2,
            )
            // Category tabs
            .child_sized(self.render_tabs(), 1)
            // Content
            .child_flex(
                hstack()
                    .gap(2)
                    .child_flex(self.render_widget_list(cat), 1.0)
                    .child_flex(self.render_preview(), 1.0),
                1.0,
            )
            // Footer
            .child_sized(Text::muted("[Tab] Category  [↑↓] Navigate  [q] Quit"), 1)
            .render(ctx);
    }
}

impl Gallery {
    fn render_tabs(&self) -> impl View {
        let mut row = hstack().gap(2);

        for (i, cat) in self.categories.iter().enumerate() {
            let label = format!("{} {}", cat.icon, cat.name);
            let is_active = i == self.category;

            let tab = if is_active {
                Text::new(format!("[{}]", label)).bold()
            } else {
                Text::muted(format!(" {} ", label))
            };

            row = row.child_sized(tab, cols(&label) + 2);
        }

        row
    }

    fn render_widget_list(&self, cat: &Category) -> impl View {
        let mut list = vstack();

        for (i, widget) in cat.widgets.iter().enumerate() {
            let is_selected = i == self.widget_idx;
            let prefix = if is_selected { "▶ " } else { "  " };
            let line = format!("{}{}", prefix, widget.name);

            let text = if is_selected {
                Text::new(line).bold()
            } else {
                Text::new(line)
            };

            list = list.child_sized(text, 1);
        }

        Border::rounded()
            .child(list)
            .title(format!(" {} Widgets ", cat.name))
    }

    fn render_preview(&self) -> impl View {
        if let Some(widget) = self.current_widget() {
            let content = vstack()
                .gap(1)
                .child_sized(Text::new(widget.name).bold(), 1)
                .child_sized(Text::muted(widget.description), 1)
                .child_sized(Text::new(""), 1)
                .child(self.render_demo(widget.name));

            Border::rounded().child(content).title(" Preview ")
        } else {
            Border::rounded()
                .child(Text::new("Select a widget"))
                .title(" Preview ")
        }
    }

    fn render_demo(&self, name: &str) -> Box<dyn View> {
        match name {
            "Text" => Box::new(demo_text()),
            "Button" => Box::new(demo_button()),
            "Progress" => Box::new(demo_progress()),
            "Spinner" => Box::new(demo_spinner()),
            "Checkbox" => Box::new(demo_checkbox()),
            "Switch" => Box::new(demo_switch()),
            "Badge" => Box::new(demo_badge()),
            "Gauge" => Box::new(demo_gauge()),
            _ => Box::new(demo_placeholder(name)),
        }
    }
}

// =============================================================================
// Demo Widgets
// =============================================================================

fn demo_text() -> impl View {
    vstack()
        .child_sized(Text::new("Normal text"), 1)
        .child_sized(Text::new("Bold text").bold(), 1)
        .child_sized(Text::muted("Muted text"), 1)
}

fn demo_button() -> impl View {
    hstack()
        .gap(2)
        .child_sized(Button::primary("Primary"), cols("Primary") + 4)
        .child_sized(Button::new("Secondary"), cols("Secondary") + 4)
        .child(Button::new("Disabled").disabled(true))
}

fn demo_progress() -> impl View {
    vstack()
        .gap(1)
        .child_sized(Progress::new(0.3), 1)
        .child_sized(Progress::new(0.7), 1)
}

fn demo_spinner() -> impl View {
    hstack()
        .gap(2)
        .child_sized(Spinner::new(), 1)
        .child(Text::new("Loading..."))
}

fn demo_checkbox() -> impl View {
    vstack()
        .child_sized(Checkbox::new("Option A").checked(true), 1)
        .child_sized(Checkbox::new("Option B").checked(false), 1)
        .child_sized(Checkbox::new("Option C").checked(true), 1)
}

fn demo_switch() -> impl View {
    vstack()
        .child_sized(Switch::new().on(true).label("Enabled"), 1)
        .child_sized(Switch::new().on(false).label("Disabled"), 1)
}

fn demo_badge() -> impl View {
    hstack()
        .gap(1)
        .child_sized(Badge::new("New"), cols("New") + 2)
        .child_sized(
            Badge::new("Hot").variant(BadgeVariant::Warning),
            cols("Hot") + 2,
        )
        .child(Badge::new("Error").variant(BadgeVariant::Error))
}

fn demo_gauge() -> impl View {
    Gauge::new().value(0.65).label("CPU")
}

fn demo_placeholder(name: &str) -> impl View {
    vstack()
        .child_sized(Text::muted(format!("{} widget demo", name)), 1)
        .child_sized(Text::new("Coming soon..."), 1)
}
