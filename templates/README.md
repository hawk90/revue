# Revue Starter Templates

Ready-to-use project templates for building TUI applications with revue.

## Templates

| Template | Description | Complexity |
|----------|-------------|------------|
| [basic](./basic/) | Minimal hello world app | Beginner |
| [counter](./counter/) | Reactive counter with Signal/Computed | Beginner |
| [form-app](./form-app/) | Form with inputs, validation, feedback | Intermediate |
| [dashboard](./dashboard/) | Multi-panel dashboard with CSS styling | Intermediate |

## Quick Start

Copy any template directory to start a new project:

```bash
# Copy template
cp -r templates/counter my-app
cd my-app

# Update package name in Cargo.toml
# Then build and run
cargo run
```

The templates depend on `revue = "3.0.0"` from crates.io. In 3.0 the
stylesheet reaches every widget and stacks size their children to their
content; see the [3.0 migration guide](../docs/migration/v3.0.0.md).

In every template the event handler returns `true` when the view changed
(that is what triggers a redraw) and quits with `app.quit()`, which lets
`run` restore the terminal.

## What Each Template Demonstrates

### basic
- `vstack()` layout composition
- `Text` widget presets (heading, muted, info)
- Minimal event handling

### counter
- `Signal<T>` mutable reactive state
- `Computed<T>` derived cached values
- A `Border` container styled from CSS
- Inline CSS with `App::builder().css()`

### form-app
- Multiple Signal fields for form state
- Field navigation with Tab
- Client-side validation
- Success/error feedback

### dashboard
- Multi-panel layout with `hstack()` / `vstack()`, `child_flex` and `child_sized`
- Progress bars
- External CSS file with CSS variables
- Real-time data updates via `Event::Tick`
- CSS classes for conditional styling
