//! Drag and Drop example
//!
//! This demonstrates the drag-and-drop system including:
//! - SortableList for reorderable items
//! - DropZone for drag targets
//! - DragContext for manual drag operations
//!
//! Run with: cargo run --example drag_drop

use revue::event::drag::{DragContext, DragData};
use revue::layout::Rect;
use revue::prelude::*;
use revue::widget::{DropZone, SortableList};

// =============================================================================
// Demo App
// =============================================================================

struct DragDropDemo {
    // Sortable list items
    tasks: Vec<String>,
    // Drop zone received items
    completed: Vec<String>,
    trash: Vec<String>,
    // Manual drag state
    drag_ctx: DragContext,
    // Selected tab
    active_tab: usize,
    // Status message
    status: String,
}

impl DragDropDemo {
    fn new() -> Self {
        Self {
            tasks: vec![
                "Review pull request".to_string(),
                "Write documentation".to_string(),
                "Fix bug #123".to_string(),
                "Add unit tests".to_string(),
                "Refactor module".to_string(),
            ],
            completed: Vec::new(),
            trash: Vec::new(),
            drag_ctx: DragContext::new(),
            active_tab: 0,
            status: "Use ↑↓ to select, Enter to drag, Tab to switch zones".to_string(),
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Tab => {
                self.active_tab = (self.active_tab + 1) % 3;
                self.status = format!("Switched to tab {}", self.active_tab + 1);
                true
            }
            Key::Char('1') => {
                self.active_tab = 0;
                true
            }
            Key::Char('2') => {
                self.active_tab = 1;
                true
            }
            Key::Char('3') => {
                self.active_tab = 2;
                true
            }
            Key::Char('r') => {
                // Reset demo
                *self = Self::new();
                self.status = "Demo reset!".to_string();
                true
            }
            _ => false,
        }
    }
}

impl View for DragDropDemo {
    fn render(&self, ctx: &mut RenderContext) {
        let view = vstack()
            .gap(1)
            .child(self.render_header())
            .child(self.render_tabs())
            .child(self.render_main_content())
            .child(self.render_status())
            .child(self.render_controls());

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("DragDropDemo")
    }
}

impl DragDropDemo {
    fn render_header(&self) -> impl View {
        vstack()
            .child(
                Text::new("🎯 Drag & Drop Demo")
                    .bold()
                    .fg(Color::CYAN)
                    .align(Alignment::Center),
            )
            .child(Text::muted("Sortable lists and drop zones").align(Alignment::Center))
    }

    fn render_tabs(&self) -> impl View {
        hstack()
            .gap(2)
            .child(self.tab_button("Tasks", 0))
            .child(self.tab_button("Completed", 1))
            .child(self.tab_button("Trash", 2))
    }

    fn tab_button(&self, label: &str, index: usize) -> Text {
        if self.active_tab == index {
            Text::new(format!("[{}]", label)).fg(Color::CYAN).bold()
        } else {
            Text::new(format!(" {} ", label)).fg(Color::WHITE)
        }
    }

    fn render_main_content(&self) -> impl View {
        hstack()
            .gap(2)
            .child(self.render_task_list())
            .child(self.render_drop_zones())
    }

    fn render_task_list(&self) -> Border {
        let items: Vec<Text> = self
            .tasks
            .iter()
            .enumerate()
            .map(|(i, task)| {
                let prefix = if i == 0 { "▶ " } else { "  " };
                Text::new(format!("{}{}", prefix, task))
            })
            .collect();

        let content = if items.is_empty() {
            vstack().child(Text::muted("No tasks"))
        } else {
            let mut stack = vstack();
            for item in items {
                stack = stack.child(item);
            }
            stack
        };

        Border::panel()
            .title(format!("📋 Tasks ({})", self.tasks.len()))
            .child(content)
    }

    fn render_drop_zones(&self) -> impl View {
        vstack()
            .gap(1)
            .child(
                Border::rounded()
                    .title(format!("✅ Completed ({})", self.completed.len()))
                    .fg(Color::GREEN)
                    .child(self.render_completed_items()),
            )
            .child(
                Border::rounded()
                    .title(format!("🗑️  Trash ({})", self.trash.len()))
                    .fg(Color::RED)
                    .child(self.render_trash_items()),
            )
    }

    fn render_completed_items(&self) -> impl View {
        if self.completed.is_empty() {
            vstack().child(Text::muted("Drop completed tasks here"))
        } else {
            let mut stack = vstack();
            for item in &self.completed {
                stack = stack.child(Text::new(format!("✓ {}", item)).fg(Color::GREEN));
            }
            stack
        }
    }

    fn render_trash_items(&self) -> impl View {
        if self.trash.is_empty() {
            vstack().child(Text::muted("Drop items to delete"))
        } else {
            let mut stack = vstack();
            for item in &self.trash {
                stack = stack.child(Text::new(format!("✗ {}", item)).fg(Color::RED));
            }
            stack
        }
    }

    fn render_status(&self) -> impl View {
        Border::single().child(
            Text::new(&self.status)
                .fg(Color::YELLOW)
                .align(Alignment::Center),
        )
    }

    fn render_controls(&self) -> impl View {
        Border::single().title("Controls").child(
            vstack()
                .child(
                    hstack()
                        .gap(4)
                        .child(Text::muted("[1-3]"))
                        .child(Text::new("Switch panels")),
                )
                .child(
                    hstack()
                        .gap(4)
                        .child(Text::muted("[Tab]"))
                        .child(Text::new("Next panel")),
                )
                .child(
                    hstack()
                        .gap(4)
                        .child(Text::muted("[r]"))
                        .child(Text::new("Reset demo")),
                )
                .child(
                    hstack()
                        .gap(4)
                        .child(Text::muted("[q]"))
                        .child(Text::new("Quit")),
                ),
        )
    }
}

// =============================================================================
// Sortable List Demo (separate view for API showcase)
// =============================================================================

struct SortableDemo {
    items: Vec<String>,
    reorder_count: usize,
}

impl SortableDemo {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            items: vec![
                "First item".to_string(),
                "Second item".to_string(),
                "Third item".to_string(),
                "Fourth item".to_string(),
                "Fifth item".to_string(),
            ],
            reorder_count: 0,
        }
    }
}

impl View for SortableDemo {
    fn render(&self, ctx: &mut RenderContext) {
        // This demonstrates the SortableList widget API
        let _sortable = SortableList::new(self.items.clone())
            .selected_color(Color::CYAN)
            .handles(true);

        // For the demo, we'll render a simulated view
        let view = vstack()
            .gap(1)
            .child(Text::new("SortableList API Demo").bold())
            .child(Text::muted("Drag items to reorder"))
            .child(Border::single().child({
                let mut stack = vstack();
                for (i, item) in self.items.iter().enumerate() {
                    stack = stack.child(
                        hstack()
                            .gap(1)
                            .child(Text::new("⠿").fg(Color::CYAN))
                            .child(Text::new(format!("{}. {}", i + 1, item))),
                    );
                }
                stack
            }))
            .child(Text::muted(format!(
                "Reordered {} times",
                self.reorder_count
            )));

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("SortableDemo")
    }
}

// =============================================================================
// DropZone Demo (separate view for API showcase)
// =============================================================================

struct DropZoneDemo {
    dropped_items: Vec<String>,
}

impl DropZoneDemo {
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            dropped_items: Vec::new(),
        }
    }
}

impl View for DropZoneDemo {
    fn render(&self, ctx: &mut RenderContext) {
        // This demonstrates the DropZone widget API
        let _dropzone = DropZone::new("Drop items here").accepts(&["text", "file", "list_item"]);

        // For the demo, we'll render a simulated view
        let view = vstack()
            .gap(1)
            .child(Text::new("DropZone API Demo").bold())
            .child(
                Border::rounded().fg(Color::CYAN).child(
                    vstack()
                        .child(Text::new("📥 Drop Zone").align(Alignment::Center))
                        .child(Text::muted("Accepts: text, file, list_item")),
                ),
            )
            .child(Text::new(format!(
                "Received {} items",
                self.dropped_items.len()
            )));

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("DropZoneDemo")
    }
}

// =============================================================================
// DragContext Demo (manual drag API showcase)
// =============================================================================

fn demonstrate_drag_context_api() {
    // Create a drag context
    let mut ctx = DragContext::new();

    // Start a drag operation with text data
    ctx.start_drag(DragData::text("Hello World"), 10, 5);

    // Check drag state
    assert!(ctx.is_dragging());
    assert_eq!(ctx.position(), (10, 5));

    // Update position as "mouse" moves
    ctx.update_position(15, 8);
    assert_eq!(ctx.offset(), (5, 3));

    // Register drop targets (normally done by DropZone widget)
    use revue::event::drag::DropTarget;
    let target1 = DropTarget::new(1, Rect::new(0, 0, 20, 10));
    let target2 = DropTarget::new(2, Rect::new(25, 0, 20, 10));
    ctx.register_target(target1);
    ctx.register_target(target2);

    // Check if over a target
    if ctx.is_over_target() {
        println!("Over a drop target: {:?}", ctx.hovered_target());
    }

    // End the drag (would return data if over valid target)
    let result = ctx.end_drag();
    println!("Drag result: {:?}", result);

    // Different data types
    let _text_data = DragData::text("Plain text");
    let _file_data = DragData::file("/path/to/file.txt");
    let _list_data = DragData::list_item(0, "First Item");
    let _tree_data = DragData::tree_node("node-1", "Root Node");

    // Custom data
    #[derive(Debug)]
    struct CustomItem {
        id: u64,
        name: String,
    }

    let custom = CustomItem {
        id: 42,
        name: "My Item".to_string(),
    };
    let _custom_data = DragData::new("custom", custom).with_label("My Item");
}

// =============================================================================
// Main
// =============================================================================

fn main() -> Result<()> {
    println!("🎯 Drag & Drop Example");
    println!("======================\n");
    println!("This example demonstrates:");
    println!("  • SortableList - reorderable list widget");
    println!("  • DropZone - drag target areas");
    println!("  • DragContext - manual drag state management");
    println!("  • DragData - type-safe drag payloads\n");

    // Demonstrate the API (prints to console before TUI starts)
    demonstrate_drag_context_api();

    let mut app = App::builder().build();
    let demo = DragDropDemo::new();

    app.run(demo, |event, app_view, _app| match event {
        Event::Key(key_event) => app_view.handle_key(&key_event.key),
        _ => false,
    })
}
