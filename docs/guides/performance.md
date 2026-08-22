# Performance Guide

Optimize your Revue applications for smooth 60fps rendering.

## Render Optimization

### Virtual Lists

For large lists, use `VirtualList` to only render visible items:

```rust
use revue::widget::VirtualList;

// Only renders visible rows
let items: Vec<String> = (0..10_000).map(|i| format!("Item {i}")).collect();
VirtualList::new(items)  // Owns the item data
    .item_height(1)
    .renderer(|item, _idx, _selected| item.clone())
```

Variable height support:

```rust
VirtualList::new(items)
    .variable_height(|item, _idx| {
        if item.is_header { 2 } else { 1 }
    })
    .renderer(|item, _idx, _selected| render_item(item))
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

let loader = ProgressiveLoader::new(items, 100);

// Load chunks on demand until every item is loaded
while !loader.is_complete() {
    let chunk = loader.load_next();
    process_chunk(chunk);
}
```

### Render Batching

Batch multiple render operations:

```rust
use revue::render::RenderBatch;

let mut batch = RenderBatch::new();

for item in items {
    batch.text(x, y, &item.text, Some(Color::WHITE), None);
}

batch.apply_to_buffer(&mut buffer);
```

## Memory Optimization

### Object Pooling

Reuse allocations:

```rust
use revue::dom::{ObjectPool, BufferPool, StringPool};

// Reuse buffers
let pool = BufferPool::new();
let buffer = pool.acquire(80, 24);
// ... use buffer
pool.release(buffer);  // Returns to pool

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

## Incremental DOM (Reconciliation)

**Opt-in.** By default the DOM is built once, on the first frame, and then stops
following the view — so a widget added later is invisible to CSS matching, to
layout, and to devtools until something forces a rebuild.

Turn on per-frame reconciliation with:

```rust
let mut app = App::builder()
    .incremental_dom(true)
    .build();
```

With it on, every frame reconciles the view against the existing tree. A node
that still matches keeps its `DomId`, its state (focus, hover, selection) and
its cached style; only what actually changed is marked dirty, and layout is
rebuilt only when the *shape* of the tree changed.

> This is opt-in while its performance is measured against the committed
> baseline in [`docs/refactor/phase0-baseline.md`](../refactor/phase0-baseline.md).
> It will become the default.

### Give collection items a key

Matching priority is:

```text
key  >  element id  >  position + widget type  >  build a new node
```

Without a key a widget's identity is its **position** among its siblings. That
is fine for a fixed layout and wrong for a dynamic collection: prepend a row and
every row below it reconciles against its neighbor's node, so focus, selection
and scroll offset all shift by one.

```rust
use revue::dom::WidgetKey;

impl View for TodoRow {
    fn key(&self) -> Option<WidgetKey> {
        Some(WidgetKey::from(self.todo.id))
    }
}
```

Use the identity of the **data**, never the loop index — an index is positional
identity spelled differently, and it changes the moment the list reorders.

`WidgetKey` is `Int(u64)` or `Str(String)`, with `From` impls for the usual
integer types, `&str` and `String`.

Two siblings claiming the same key is a bug in your code: the first one wins the
existing node and the second gets a fresh one.

### Dirty Rect Optimization

Revue automatically tracks dirty regions and only re-renders what changed. When a widget's state changes, only the affected screen area is updated — unchanged pixels are preserved from the previous frame.

This happens transparently without requiring user code changes:

- **No dirty regions**: Previous buffer is reused (zero rendering work)
- **Partial dirty**: Old buffer copied, only dirty regions cleared and re-rendered
- **Full screen dirty**: Falls back to full clear (e.g., on resize)

The selector cache is also optimized — parsed selectors are cached once and referenced without copying, eliminating per-node Vec allocations during style computation.

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
easing::ease_out_cubic(t)
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
