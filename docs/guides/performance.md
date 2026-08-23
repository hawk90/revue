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

### DOM from the render traversal

**Opt-in, and the reason CSS reaches anything below the root.**

```rust
let mut app = App::builder()
    .dom_from_render(true)
    .build();
```

Off, the DOM only contains widgets exposed through `View::children`, which
almost nothing implements — the idiomatic widget assembles its tree inside
`render`. A real application therefore has a DOM of one node, and CSS matching,
`:focus`/`:hover` and devtools all work against a tree that does not describe
it.

On, the frame renders twice: once to discover the tree, once to paint it.

```text
collect pass  ->  reconcile DOM  ->  compute styles  ->  paint pass
```

Every widget rendered through `RenderContext::render_child` gets a DOM node and
its own computed style. Two passes rather than one because a node's style
depends on the whole tree — `:last-child` and `:nth-child` cannot be resolved
before the siblings are known.

Implies per-frame reconciliation, so `incremental_dom` adds nothing on top.

**Cost**, 120x40 with `cargo bench --bench frame`:

| rows | one pass | two passes |
|---:|---:|---:|
| 10 | 20.7 µs | 31.7 µs |
| 50 | 32.2 µs | 65.6 µs |
| 200 | 56.5 µs | 159.5 µs |

1.5x at ten widgets, 2.8x at two hundred — the second traversal is only part of
it, the rest is that the DOM now actually exists and has to be reconciled and
cascaded. At 60fps the worst of these is under 1% of a frame.

**Writing a container.** Route child rendering through the context rather than
building one by hand, or the child gets no node and no style:

```rust
// Registers a node, delivers the child's computed style
ctx.render_child(child.as_ref(), child_area);

// Does neither
let mut child_ctx = RenderContext::new(ctx.buffer, child_area);
child.render(&mut child_ctx);
```

`Stack`, `Border`, `Positioned` and `Grid` are migrated. A container that is not
keeps working exactly as before.

**Known limit.** A view that delegates its whole body to another widget merges
with it rather than nesting under it:

```rust
fn render(&self, ctx: &mut RenderContext) {
    vstack().element_id("list").child(...).render(ctx);  // "list" gets no node
}
```

`render_child` registers a *child*; a widget rendered into the caller's own
context is the caller's own rendering. Put the id on the view itself.

**Layout properties need one more flag.** `display`, `width`, `height`,
`margin`, `min-*`/`max-*` and `gap` take effect with
[`css_layout`](#css-box-properties) on top of this one. `flex-*` and
`grid-template-*` still do nothing — the container computes those itself.
See [`findings-layout.md`](../refactor/findings-layout.md).

### CSS box properties

**Opt-in, and needs `dom_from_render`.**

```rust
let mut app = App::builder()
    .dom_from_render(true)
    .css_layout(true)
    .build();
```

The container keeps deciding the flow — `vstack()` stacks, its `gap` and
per-child sizes still apply. On top of that, a node's own specified `display`,
`width`, `height`, `margin` and `min-*`/`max-*` adjust the area it was handed.
That is what makes `#sidebar { width: 20; }` and `.hidden { display: none; }` do
something.

**Cost.** Nothing measurable. Within one benchmark run, 120x40:

| rows | `dom_from_render` | `+ css_layout` |
|---:|---:|---:|
| 10 | 32.0 µs | 32.2 µs |
| 50 | 70.2 µs | 69.4 µs |
| 200 | 179.7 µs | 174.5 µs |

A node that specifies no box property — almost all of them — costs a handful of
comparisons and keeps the area it was given.

`gap`, `column-gap` and `row-gap` reach `vstack`, `hstack` and `grid`. They
describe flow, so the container reads them rather than having them applied from
outside — `ctx.gap_or(self.gap)` is the whole of it, and `gap: 0` leaves the
builder's own value alone.

`padding` is not applied: it insets a widget's *content*, and a widget that
draws its own border would have the border move instead. `flex-*` and
`grid-template-*` stay with the container.

**Why not the layout engine.** Making its computed rects authoritative would
first need every widget's layout intent — `vstack()`'s direction, its gap, its
per-child sizes — to reach the DOM as inline style, and then an intrinsic-size
measurement the engine does not have. The comparison is written up in
[`findings-layout.md`](../refactor/findings-layout.md).

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

### How a frame is drawn

Every draw renders the whole view into a back buffer, then diffs that buffer
against the previously presented one and writes only the cells that differ.

```text
view.render()  ->  back buffer  ->  diff vs front buffer  ->  terminal
```

Painting into a buffer is memory traffic; the expensive part of a frame is what
goes down the wire. The diff is what makes rendering from scratch affordable —
an unchanged frame produces zero bytes, and a one-character change produces a
cursor move and a character.

Draws happen on events, not on a timer: `App::run` only draws when the event
handler asks for it, or when a transition is active.

**Cost.** A 120x40 screen, measured with `cargo bench --bench frame`:

| rows | frame with a change | unchanged frame |
|---:|---:|---:|
| 10 | 20.5 µs | 19.6 µs |
| 50 | 31.5 µs | 28.7 µs |
| 200 | 55.8 µs | 52.8 µs |

At 60fps the budget is 16,667 µs, so the worst of these is 0.3% of a frame.

> Revue used to skip rendering when the DOM reported no dirty nodes, and to mask
> the diff to dirty regions. Both were unsound — a widget's content is not part
> of its DOM metadata, so an ordinary state change marked nothing dirty and the
> app stopped repainting entirely. See
> [`docs/refactor/findings-render-pipeline.md`](../refactor/findings-render-pipeline.md).
> Region-based skipping can come back once the DOM actually describes the widget
> tree; until then, correct beats clever.

The selector cache is optimized — parsed selectors are cached once and
referenced without copying, eliminating per-node Vec allocations during style
computation.

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
