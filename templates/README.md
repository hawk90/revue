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

## What Each Template Demonstrates

### basic
- `vstack()` layout composition
- `Text` widget presets (heading, muted, info)
- Minimal event handling

### counter
- `Signal<T>` mutable reactive state
- `Computed<T>` derived cached values
- `Border::panel()` styled container
- Inline CSS with `App::builder().css()`

### form-app
- Multiple Signal fields for form state
- Field navigation with Tab
- Client-side validation
- Success/error feedback

### dashboard
- Multi-panel layout with `hstack()` / `vstack()`
- Progress bars
- External CSS file with CSS variables
- Real-time data updates via `Event::Tick`
- CSS classes for conditional styling
