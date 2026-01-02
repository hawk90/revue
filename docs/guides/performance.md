# Performance Guide

Optimize your Revue applications for smooth 60fps rendering.

## Render Optimization

### Virtual Lists

For large lists, use `VirtualList` to only render visible items:

```rust
use revue::widget::VirtualList;

// Only renders visible rows
VirtualList::new(10_000)  // Total items
    .row_height(1)
    .render_row(|idx| {
        Text::new(format!("Item {}", idx))
    })
```

Variable height support:

```rust
VirtualList::new(items.len())
    .height_calculator(|idx| {
        if items[idx].is_header { 2 } else { 1 }
    })
    .render_row(|idx| render_item(&items[idx]))
```

### Lazy Loading

Load data on demand:

```rust
use revue::patterns::LazyData;

let data = LazyData::new(|| {
    // Expensive computation
    load_large_dataset()
});

// Only loads when accessed
if let Some(value) = data.get() {
    render(value);
}
```

Progressive loading:

```rust
use revue::patterns::ProgressiveLoader;

let loader = ProgressiveLoader::new(total_items)
    .chunk_size(100)
    .on_chunk(|chunk| {
        process_chunk(chunk);
    });
```

### Render Batching

Batch multiple render operations:

```rust
use revue::render::RenderBatch;

let mut batch = RenderBatch::new();

for item in items {
    batch.text(x, y, &item.text, style);
}

batch.flush(buffer);
```

## Memory Optimization

### Object Pooling

Reuse allocations:

```rust
use revue::dom::{ObjectPool, BufferPool, StringPool};

// Reuse buffers
let pool = BufferPool::new();
let buffer = pool.get(80, 24);
// ... use buffer
pool.put(buffer);  // Returns to pool

// String interning
let strings = StringPool::new();
let s1 = strings.intern("hello");
let s2 = strings.intern("hello");
assert!(std::ptr::eq(s1.as_str(), s2.as_str()));
```

### Avoid Allocations in Render

```rust
impl View for MyWidget {
    fn render(&self, ctx: &mut RenderContext) {
        // Bad: Allocates every frame
        let text = format!("Count: {}", self.count);

        // Good: Reuse buffer
        self.buffer.clear();
        write!(&mut self.buffer, "Count: {}", self.count);
    }
}
```

## Profiling

### Built-in Profiler

```rust
use revue::utils::profiler::{profile, profiler_report};

fn expensive_operation() {
    profile!("expensive_operation");
    // ... work
}

// Print report
println!("{}", profiler_report());
```

### Timing Specific Sections

```rust
use revue::utils::profiler::{Profiler, start_profile};

let _guard = start_profile("render_list");
for item in items {
    let _item_guard = start_profile("render_item");
    render_item(item);
}
// Guards auto-complete timing on drop
```

## Incremental Rendering

Revue uses incremental DOM updates by default:

```rust
// DOM diff tracks which nodes changed
let renderer = DomRenderer::new();

// First render builds full tree
renderer.build(&widget);

// Subsequent renders only update changes
renderer.build_incremental(&widget);
```

### Dirty Rect Optimization

Only re-render changed regions:

```rust
// Transitions track affected nodes
let active_nodes = transitions.active_node_ids();

// Only redraw changed areas
for id in active_nodes {
    let rect = layout.get_rect(id);
    buffer.clear_rect(rect);
    render_node(id, buffer);
}
```

## Animation Performance

### Reduced Motion

Respect user preferences:

```rust
use revue::style::should_skip_animation;

if should_skip_animation() {
    // Instant change
    set_value(target);
} else {
    // Animate
    animate_to(target);
}
```

### Efficient Easing

Use built-in easing functions:

```rust
use revue::style::easing;

// Pre-computed curves
easing::ease_in_out(t)
easing::cubic_bezier(0.4, 0.0, 0.2, 1.0, t)
```

## Benchmarking

Run benchmarks:

```bash
cargo bench
```

Revue includes benchmarks for:
- DOM building
- CSS parsing
- Layout computation
- Rendering

Example benchmark:

```rust
use criterion::{criterion_group, Criterion};

fn bench_render(c: &mut Criterion) {
    c.bench_function("render_list_1000", |b| {
        let items: Vec<_> = (0..1000).collect();
        b.iter(|| {
            let list = List::new(&items);
            render(&list);
        });
    });
}
```

## Best Practices

### 1. Minimize Signal Updates

```rust
// Bad: Updates signal on every keystroke
input.on_change(|text| {
    search_signal.set(text);
    perform_search();  // Expensive!
});

// Good: Debounce updates
input.on_change(|text| {
    debounce(Duration::from_millis(300), || {
        search_signal.set(text);
        perform_search();
    });
});
```

### 2. Use Keys for Lists

```rust
// Help DOM diffing with stable keys
List::new(items)
    .key(|item| item.id)
```

### 3. Avoid Deep Nesting

```rust
// Bad: Deep widget tree
vstack().child(
    vstack().child(
        vstack().child(
            // ...
        )
    )
)

// Good: Flatten when possible
vstack()
    .child(header)
    .child(content)
    .child(footer)
```

### 4. Profile Before Optimizing

Always measure first:

```rust
let _guard = start_profile("suspected_slow_code");
// ... code
// Check profiler_report() for actual timings
```

## Troubleshooting

### Slow Rendering

1. Check for allocations in render loops
2. Use VirtualList for large lists
3. Profile to find hotspots

### High Memory Usage

1. Enable object pooling
2. Check for signal leaks
3. Use lazy loading

### Choppy Animations

1. Check reduced motion setting
2. Simplify animated properties
3. Use hardware-friendly durations (16ms multiples)
