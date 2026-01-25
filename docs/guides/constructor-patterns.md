# Widget Constructor Patterns

This guide defines the standard patterns for widget constructors in revue.

## Overview

Revue uses a **hybrid approach** to widget constructors based on widget complexity:

- **Simple widgets** (< 5 configuration options): Function-style constructors
- **Complex widgets** (>= 5 configuration options): `::new()` or builder pattern

## Function-Style Constructors (Simple Widgets)

For widgets with fewer than 5 configuration options, use lowercase snake_case function-style constructors.

### Pattern

```rust
/// Create a new widget with default values
pub fn widget_name() -> Widget {
    Widget::new()
}

/// Create a new widget with common preset
pub fn widget_preset() -> Widget {
    Widget::new().preset_value()
}
```

### Examples

```rust
// Simple: 0-2 required params
let btn = button("Click me");
let chk = checkbox("Enable feature");
let txt = text("Hello, world!");
let img = image_from_file("path.png");

// With chaining
let btn = button("Click")
    .style(ButtonStyle::Primary)
    .on_click(|_| println!("Clicked!"));
```

### Implementation Template

```rust
// widget.rs

impl Widget {
    /// Create a new widget with defaults
    pub fn new() -> Self {
        Self {
            value: default_value(),
            style: Default::default(),
            props: WidgetProps::new(),
        }
    }

    // Builder methods
    pub fn value(mut self, value: T) -> Self {
        self.value = value;
        self
    }

    pub fn style(mut self, style: WidgetStyle) -> Self {
        self.style = style;
        self
    }
}

// helper.rs or mod.rs
/// Create a new widget
pub fn widget() -> Widget {
    Widget::new()
}

/// Create a new widget with value
pub fn widget_with_value(value: T) -> Widget {
    Widget::new().value(value)
}

/// Preset: special configuration
pub fn widget_preset() -> Widget {
    Widget::new()
        .value(preset_value)
        .style(PresetStyle)
}
```

## `::new()` Constructors (Complex Widgets)

For widgets with 5+ configuration options, use the `::new()` method directly.

### Pattern

```rust
impl ComplexWidget {
    pub fn new() -> Self {
        Self {
            // Many configuration options
            option1: default1,
            option2: default2,
            option3: default3,
            option4: default4,
            option5: default5,
            // ...
        }
    }

    // Many builder methods...
}
```

### Examples

```rust
// Complex: many configuration options
let grid = DataGrid::new()
    .columns(vec![...])
    .data(data)
    .selectable(true)
    .sortable(true)
    .filterable(true)
    .virtual_scroll(true);

let editor = CodeEditor::new()
    .language(Language::Rust)
    .theme(SyntaxTheme::Dark)
    .line_numbers(true)
    .auto_indent(true)
    .bracket_matching(true);
```

## Preset Functions

For common widget configurations, provide preset functions.

### Pattern

```rust
// Specialized constructors for common use cases
pub fn specialized_widget() -> Widget {
    Widget::new()
        .option1(specific_value)
        .option2(specific_value)
}
```

### Examples

```rust
// From existing codebase
pub fn file_picker() -> FilePicker {
    FilePicker::new().mode(FilePickerMode::OpenFile)
}

pub fn save_picker() -> FilePicker {
    FilePicker::new().mode(FilePickerMode::SaveFile)
}

pub fn dir_picker() -> FilePicker {
    FilePicker::new().mode(FilePickerMode::SelectDirectory)
}

pub fn percentage_slider() -> Slider {
    Slider::new().min(0.0).max(100.0).step(1.0)
}

pub fn volume_slider() -> Slider {
    Slider::new().min(0.0).max(1.0).step(0.1)
}

pub fn pomodoro() -> Timer {
    Timer::new()
        .duration(Duration::from_secs(25 * 60))
        .mode(TimerMode::Countdown)
}
```

## Builder Methods

All widgets should support method chaining for configuration.

### Guidelines

1. **Return `Self`**: All builder methods return `Self` for chaining
2. **Take `mut self`**: Consume the widget, modify it, return it
3. **Descriptive names**: Use clear, descriptive method names
4. **Consistent order**: Group related methods together

### Example

```rust
impl Widget {
    // Value configuration
    pub fn value(mut self, value: T) -> Self {
        self.value = value;
        self
    }

    // Style configuration
    pub fn style(mut self, style: WidgetStyle) -> Self {
        self.style = style;
        self
    }

    pub fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    // Behavior configuration
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(mut self, f: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}
```

## Migration Guide

### Adding Function-Style to Existing Widgets

When adding a function-style constructor to an existing widget:

1. **Keep `::new()`**: Don't break existing code
2. **Add helper function**: Create a function that calls `::new()`
3. **Add presets**: Create helper functions for common configurations
4. **Re-export**: Export from the module's `mod.rs`

```rust
// Before
let widget = Widget::new();

// After - both work
let widget = Widget::new();  // Still works
let widget = widget();        // New shorthand
```

### Deprecation (Future Major Version)

When preparing for a major version bump:

1. **Mark old as deprecated**: Use `#[deprecated]` attribute
2. **Provide migration path**: Document the new pattern
3. **Wait for major release**: Remove in next major version

```rust
#[deprecated(since = "2.0.0", note = "Use `widget()` instead")]
pub fn new() -> Self {
    Self::default()
}
```

## Quick Reference

| Widget Type | Pattern | Example |
|-------------|---------|---------|
| Simple (< 5 options) | Function-style | `button("Click")` |
| Complex (>= 5 options) | `::new()` | `DataGrid::new()` |
| Specialized use case | Preset function | `file_picker()` |

## Code Examples

### Simple Widget Implementation

```rust
// src/widget/widgets/button.rs

#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    style: ButtonStyle,
    on_click: Option<Box<dyn Fn() + 'static>>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            label: String::new(),
            style: ButtonStyle::default(),
            on_click: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = label.into();
        self
    }

    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn on_click(mut self, f: impl Fn() + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

// src/widget/widgets/mod.rs
/// Create a new button
pub fn button(label: impl Into<String>) -> Button {
    Button::new().label(label)
}

/// Create a primary button
pub fn primary_button(label: impl Into<String>) -> Button {
    Button::new()
        .label(label)
        .style(ButtonStyle::Primary)
}
```

### Complex Widget Implementation

```rust
// src/widget/data/datagrid/mod.rs

impl DataGrid {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            data: Vec::new(),
            selectable: false,
            sortable: false,
            filterable: false,
            virtual_scroll: false,
            // ... many more options
        }
    }

    // Many builder methods...
}
```

## Testing

When adding or modifying constructors, include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_constructor() {
        let widget = widget();
        assert_eq!(widget.value(), default_value);
    }

    #[test]
    fn test_preset_constructor() {
        let widget = widget_preset();
        assert_eq!(widget.value(), preset_value);
    }

    #[test]
    fn test_builder_chain() {
        let widget = widget()
            .value(custom_value)
            .style(CustomStyle);
        assert_eq!(widget.value(), custom_value);
    }
}
```

## Related Documentation

- [Widget Development Guide](../tutorials/04-custom-widget.md)
- [Testing Guide](testing.md)
- [Architecture](../ARCHITECTURE.md)
