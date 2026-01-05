# Drag and Drop Guide

Revue provides a complete drag-and-drop framework for terminal UIs.

## Overview

The drag-and-drop system consists of:

- **DragContext** - Manages global drag state
- **DragData** - Type-safe data payload
- **DropTarget** - Defines drop zones
- **DragState** - Tracks operation state

## Basic Usage

### Starting a Drag

```rust
use revue::event::drag::{DragContext, DragData};

let mut ctx = DragContext::new();

// Start drag with text data
ctx.start_drag(DragData::text("Hello World"), x, y);
```

### Tracking Position

```rust
// Update as mouse/cursor moves
ctx.update_position(new_x, new_y);

// Get current position
let (x, y) = ctx.position();

// Get offset from start
let (dx, dy) = ctx.offset();
```

### Completing a Drop

```rust
if ctx.is_over_target() {
    if let Some((data, target_id)) = ctx.end_drag() {
        println!("Dropped {:?} on target {:?}", data.display_label(), target_id);
    }
}
```

### Cancelling a Drag

```rust
ctx.cancel();
```

## DragData Types

### Built-in Types

```rust
use revue::event::drag::DragData;

// Text content
let text = DragData::text("Hello");

// File path
let file = DragData::file("/path/to/file.txt");

// List item (index + label)
let item = DragData::list_item(3, "Item 3");

// Tree node
let node = DragData::tree_node("node-123", "My Node");
```

### Custom Data Types

```rust
#[derive(Debug)]
struct TaskItem {
    id: u64,
    title: String,
    priority: u8,
}

let task = TaskItem {
    id: 42,
    title: "Fix bug".into(),
    priority: 1,
};

// Create custom drag data
let data = DragData::new("task", task)
    .with_label("Fix bug");
```

### Extracting Data

```rust
// Get as text (for text, file, tree_node types)
if let Some(text) = data.as_text() {
    println!("Text: {}", text);
}

// Get list item index
if let Some(index) = data.as_list_index() {
    println!("Item index: {}", index);
}

// Get custom type
if let Some(task) = data.get::<TaskItem>() {
    println!("Task: {} (priority {})", task.title, task.priority);
}

// Check type
if data.is_type("task") {
    // Handle task
}
```

## Drop Targets

### Registering Targets

```rust
use revue::event::drag::DropTarget;
use revue::layout::Rect;

// Create a target that accepts specific types
let target = DropTarget::new(1, Rect::new(10, 10, 30, 10))
    .accepts(&["text", "file"]);

ctx.register_target(target);
```

### Accept All Types

```rust
let target = DropTarget::new(2, Rect::new(50, 10, 30, 10))
    .accepts_all();

ctx.register_target(target);
```

### Managing Targets

```rust
// Unregister a specific target
ctx.unregister_target(target_id);

// Clear all targets (call on layout changes)
ctx.clear_targets();

// Get a target by ID
if let Some(target) = ctx.get_target(1) {
    println!("Target bounds: {:?}", target.bounds);
}

// Check if a target is hovered
if ctx.is_target_hovered(1) {
    // Highlight the target
}
```

## DragState

```rust
use revue::event::drag::DragState;

match ctx.state() {
    DragState::Idle => { /* No drag */ }
    DragState::Pending => { /* Waiting for threshold */ }
    DragState::Dragging => { /* Actively dragging */ }
    DragState::OverTarget => { /* Over valid drop zone */ }
    DragState::Dropped => { /* Just completed */ }
    DragState::Cancelled => { /* Was cancelled */ }
}

// Helper methods
ctx.is_dragging();    // Pending, Dragging, or OverTarget
ctx.is_over_target(); // OverTarget only
```

## Configuration

### Drag Threshold

Set minimum distance before drag starts:

```rust
// Require 5 pixels of movement
let ctx = DragContext::new().threshold(5);
```

### Drag Preview

Enable/disable visual preview:

```rust
let ctx = DragContext::new().preview(true);

// Check in render
if ctx.should_show_preview() {
    // Render preview at ctx.position()
}
```

## Global Context

For simple applications, use the global drag context:

```rust
use revue::event::drag::{start_drag, update_drag_position, end_drag, cancel_drag, is_dragging};

// Start drag
start_drag(DragData::text("Hello"), 10, 10);

// Update position
update_drag_position(25, 15);

// Check state
if is_dragging() {
    // ...
}

// End or cancel
if let Some((data, target)) = end_drag() {
    // Handle drop
}

cancel_drag();
```

## Example: Sortable List

```rust
use revue::prelude::*;
use revue::event::drag::{DragContext, DragData, DropTarget};

struct SortableList {
    items: Vec<String>,
    drag_ctx: DragContext,
    dragging_index: Option<usize>,
}

impl SortableList {
    fn new(items: Vec<String>) -> Self {
        Self {
            items,
            drag_ctx: DragContext::new().threshold(3),
            dragging_index: None,
        }
    }

    fn handle_mouse(&mut self, event: &MouseEvent) -> bool {
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                // Find which item was clicked
                let index = (event.row as usize).saturating_sub(1);
                if index < self.items.len() {
                    let label = self.items[index].clone();
                    self.drag_ctx.start_drag(
                        DragData::list_item(index, &label),
                        event.column,
                        event.row,
                    );
                    self.dragging_index = Some(index);
                }
                true
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                self.drag_ctx.update_position(event.column, event.row);
                true
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if let Some((data, target_id)) = self.drag_ctx.end_drag() {
                    if let (Some(from), Some(to)) = (data.as_list_index(), target_id) {
                        self.reorder(from, to as usize);
                    }
                }
                self.dragging_index = None;
                true
            }
            _ => false,
        }
    }

    fn reorder(&mut self, from: usize, to: usize) {
        if from != to && from < self.items.len() && to < self.items.len() {
            let item = self.items.remove(from);
            self.items.insert(to, item);
        }
    }
}

impl View for SortableList {
    fn render(&self, ctx: &mut RenderContext) {
        // Register drop targets for each item slot
        for (i, _) in self.items.iter().enumerate() {
            let y = i as u16 + 1;
            let target = DropTarget::new(i as u64, Rect::new(0, y, 40, 1))
                .accepts(&["list_item"]);
            // Note: In real code, register through a mutable reference
        }

        let mut list = vstack();

        for (i, item) in self.items.iter().enumerate() {
            let is_dragging = self.dragging_index == Some(i);
            let is_drop_target = self.drag_ctx.is_target_hovered(i as u64);

            let style = if is_dragging {
                "opacity: 0.5"
            } else if is_drop_target {
                "background: blue"
            } else {
                ""
            };

            list = list.child(Text::new(format!("  {}  ", item)).style(style));
        }

        // Render drag preview
        if self.drag_ctx.should_show_preview() {
            if let Some(data) = self.drag_ctx.data() {
                let (x, y) = self.drag_ctx.position();
                // Render floating preview at (x, y)
            }
        }

        list.render(ctx);
    }
}
```

## Example: File Drop Zone

```rust
use revue::prelude::*;
use revue::event::drag::{DragContext, DragData, DropTarget, DragState};

struct FileDropZone {
    files: Vec<String>,
    drag_ctx: DragContext,
    is_hovered: bool,
}

impl FileDropZone {
    fn new() -> Self {
        let mut drag_ctx = DragContext::new();

        // Register this widget as a drop target
        drag_ctx.register_target(
            DropTarget::new(1, Rect::new(5, 5, 40, 10))
                .accepts(&["file", "text"])
        );

        Self {
            files: Vec::new(),
            drag_ctx,
            is_hovered: false,
        }
    }

    fn handle_drop(&mut self, data: DragData) {
        if let Some(path) = data.as_text() {
            self.files.push(path.to_string());
        }
    }
}

impl View for FileDropZone {
    fn render(&self, ctx: &mut RenderContext) {
        let border_color = if self.drag_ctx.is_target_hovered(1) {
            Color::CYAN
        } else {
            Color::WHITE
        };

        let content = if self.files.is_empty() {
            Text::new("Drop files here").muted()
        } else {
            let list: String = self.files.iter()
                .map(|f| format!("  - {}", f))
                .collect::<Vec<_>>()
                .join("\n");
            Text::new(list)
        };

        Border::single()
            .border_color(border_color)
            .title("Drop Zone")
            .child(content)
            .render(ctx);
    }
}
```

## Best Practices

### 1. Clear Targets on Layout Change

```rust
// When layout changes, clear and re-register targets
fn on_resize(&mut self) {
    self.drag_ctx.clear_targets();
    self.register_drop_zones();
}
```

### 2. Use Type-Safe Data

```rust
// Good: Type-checked extraction
if let Some(task) = data.get::<Task>() {
    handle_task(task);
}

// Avoid: Assuming type
let task = unsafe { ... }; // Don't do this
```

### 3. Provide Visual Feedback

```rust
// Show drag state
if ctx.is_dragging() {
    // Dim the source item
}

if ctx.is_over_target() {
    // Highlight the target
}

if ctx.should_show_preview() {
    // Show floating preview
}
```

### 4. Handle Cancellation

```rust
// ESC key to cancel
if key == Key::Esc && ctx.is_dragging() {
    ctx.cancel();
}
```

### 5. Set Appropriate Threshold

```rust
// Small threshold for precise control
let ctx = DragContext::new().threshold(2);

// Larger threshold to prevent accidental drags
let ctx = DragContext::new().threshold(5);
```

## See Also

- [Event Handling Guide](events.md)
- [Mouse Events](events.md#mouse-events)
