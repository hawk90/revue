# Revue Tutorials

Learn Revue step by step with these hands-on tutorials.

## Tutorial Series

| # | Tutorial | Time | Description |
|---|----------|------|-------------|
| 1 | [Getting Started](./01-getting-started.md) | 5 min | Install Revue and create your first app |
| 2 | [Counter App](./02-counter.md) | 15 min | Learn state management with a counter |
| 3 | [Todo App](./03-todo.md) | 30 min | Build a full-featured todo list |
| 4 | Dashboard App | 1 hour | Create a multi-panel dashboard |
| 5 | Full Application | 2+ hours | Build a complete production app |

## Quick Start

```bash
# Create a new project
cargo new my-revue-app
cd my-revue-app

# Add Revue
cargo add revue

# Run the example
cargo run
```

## Learning Path

### Beginner
1. Start with [Getting Started](./01-getting-started.md)
2. Build the [Counter App](./02-counter.md)
3. Read the [Styling Guide](../guides/styling.md)

### Intermediate
1. Build the [Todo App](./03-todo.md)
2. Learn about [State Management](../guides/state.md)
3. Explore the [Widget Gallery](../../examples/gallery.rs)

### Advanced
1. Build complex dashboards
2. Learn about [Testing](../guides/testing.md)
3. Master [Performance](../guides/performance.md) optimization
4. Implement [Accessibility](../guides/accessibility.md)

## Example Apps

Explore the `examples/` directory for more complete applications:

```bash
# Widget gallery - see all widgets
cargo run --example gallery

# Dashboard example
cargo run --example dashboard

# Interactive todo
cargo run --example reactive_todo

# Text editor
cargo run --example text_editor

# IDE-like interface
cargo run --example ide
```

## Getting Help

- [API Documentation](https://docs.rs/revue)
- [GitHub Issues](https://github.com/anthropics/revue/issues)
- [Architecture Overview](../ARCHITECTURE.md)
