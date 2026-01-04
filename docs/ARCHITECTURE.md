# Architecture

## Overview

Revue is a layered TUI framework inspired by web technologies (Vue.js, CSS) but built for terminal environments with Rust's performance.

```
┌─────────────────────────────────────────────────────────────────┐
│                        Application                               │
│                    (User's App Code)                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐              │
│  │   Widget    │  │   Style     │  │  Reactive   │              │
│  │   Layer     │  │   Layer     │  │   Layer     │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│         └────────────────┼────────────────┘                      │
│                          │                                       │
│                   ┌──────▼──────┐                                │
│                   │   Layout    │                                │
│                   │   Layer     │                                │
│                   └──────┬──────┘                                │
│                          │                                       │
│         ┌────────────────┼────────────────┐                      │
│         │                │                │                      │
│  ┌──────▼──────┐  ┌──────▼──────┐  ┌──────▼──────┐              │
│  │    Text     │  │   Render    │  │   Event     │              │
│  │   Layer     │  │   Layer     │  │   Layer     │              │
│  └──────┬──────┘  └──────┬──────┘  └──────┬──────┘              │
│         │                │                │                      │
│         └────────────────┼────────────────┘                      │
│                          │                                       │
│                   ┌──────▼──────┐                                │
│                   │  Terminal   │                                │
│                   │ (crossterm) │                                │
│                   └─────────────┘                                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

## Layer Descriptions

### 1. Widget Layer (`src/widget/`)

Provides the building blocks for UI construction.

```
src/widget/
├── mod.rs
├── traits.rs        # View, Widget traits
├── box.rs           # Box container
├── text.rs          # Text display
├── list.rs          # List widget
├── table.rs         # Table widget
├── input.rs         # Text input
├── select.rs        # Dropdown select
├── tabs.rs          # Tab container
├── modal.rs         # Modal dialog
├── toast.rs         # Toast notifications
├── markdown.rs      # Markdown renderer
├── image.rs         # Kitty image
├── command_palette.rs  # Ctrl+P command palette
└── menu.rs          # Menu bar
```

**Key Traits:**

```rust
/// Core trait for all renderable components
pub trait View {
    fn render(&self, ctx: &mut RenderContext) -> Element;
}

/// Stateful widget with internal state
pub trait StatefulWidget {
    type State: Default;
    fn render(&self, state: &mut Self::State, ctx: &mut RenderContext) -> Element;
}
```

### 2. Style Layer (`src/style/`)

CSS parsing, selector matching, and style computation.

```
src/style/
├── mod.rs
├── parser.rs        # CSS file parsing (cssparser)
├── selector.rs      # Selector matching (selectors)
├── properties.rs    # CSS property definitions
├── variables.rs     # CSS variables (--custom-prop)
├── transition.rs    # CSS transitions
├── computed.rs      # Computed style values
└── cache.rs         # Style caching
```

**Style Resolution Flow:**

```
CSS File → Parse → Selector Match → Cascade → Compute → Apply
    │                    │              │         │
cssparser           selectors      specificity  inheritance
```

**Supported CSS Properties:**

| Category | Properties |
|----------|------------|
| Layout | `display`, `flex-direction`, `flex-wrap`, `justify-content`, `align-items`, `gap` |
| Spacing | `padding`, `margin`, `width`, `height`, `min-*`, `max-*` |
| Border | `border`, `border-style`, `border-color` |
| Colors | `color`, `background`, `background-color` |
| Text | `text-align`, `text-wrap`, `font-weight` |
| Visual | `opacity`, `visibility` |
| Animation | `transition`, `transition-duration`, `transition-property` |

### 3. Reactive Layer (`src/reactive/`)

Vue-inspired reactivity system.

```
src/reactive/
├── mod.rs
├── signal.rs        # Signal<T> - reactive state
├── computed.rs      # Computed<T> - derived values
├── effect.rs        # Effect - side effects
├── scope.rs         # Reactive scope management
└── runtime.rs       # Reactivity runtime
```

**Core Types:**

```rust
/// Reactive state container
pub struct Signal<T> {
    value: Rc<RefCell<T>>,
    subscribers: Rc<RefCell<Vec<Subscriber>>>,
}

impl<T: Clone> Signal<T> {
    pub fn get(&self) -> T;
    pub fn set(&self, value: T);
    pub fn update(&self, f: impl FnOnce(&mut T));
}

/// Derived reactive value
pub struct Computed<T> {
    compute: Box<dyn Fn() -> T>,
    cached: RefCell<Option<T>>,
}

/// Side effect that runs when dependencies change
pub struct Effect {
    effect_fn: Box<dyn Fn()>,
    dependencies: Vec<SignalId>,
}
```

**Reactivity Flow:**

```
Signal::set() → Notify Subscribers → Mark Dirty → Schedule Re-render
                      │
                      ├── Computed (recalculate)
                      └── Effect (re-run)
```

### 4. Layout Layer (`src/layout/`)

Flexbox layout computation using taffy.

```
src/layout/
├── mod.rs
├── engine.rs        # Layout engine wrapper
├── convert.rs       # CSS → taffy style conversion
├── tree.rs          # Layout tree management
└── rect.rs          # Rectangle utilities
```

**Layout Process:**

```
Widget Tree → Style Resolution → Taffy Tree → Compute → Position Map
     │              │                │            │
  build()     apply_styles()   add_node()   compute()
```

### 5. Text Layer (`src/text/`)

Unicode handling and text measurement.

```
src/text/
├── mod.rs
├── width.rs         # Character width calculation
├── detect.rs        # Terminal width detection
├── wrap.rs          # Text wrapping
├── grapheme.rs      # Grapheme cluster handling
└── table.rs         # CharWidthTable
```

**Width Detection:**

```rust
/// Detect actual character width in current terminal
pub fn detect_char_width(ch: char) -> u8 {
    // 1. Save cursor position
    // 2. Print character
    // 3. Query cursor position
    // 4. Calculate difference
}

/// Cached width table
pub struct CharWidthTable {
    cjk: u8,           // Default: 2
    emoji: u8,         // Default: 2
    nerd_font: u8,     // Configurable
    overrides: HashMap<char, u8>,
}
```

### 6. Render Layer (`src/render/`)

Double buffering and terminal output.

```
src/render/
├── mod.rs
├── buffer.rs        # Double buffer implementation
├── diff.rs          # Buffer diff algorithm
├── cell.rs          # Terminal cell representation
├── frame.rs         # Frame rendering
└── kitty.rs         # Kitty image protocol
```

**Rendering Pipeline:**

```
Widget.render() → Buffer (new) → Diff (old vs new) → Commands → Terminal
                      │                │                │
                  write cells     find changes    crossterm
```

**Double Buffering:**

```rust
pub struct Buffer {
    cells: Vec<Cell>,
    width: u16,
    height: u16,
}

pub struct Cell {
    symbol: String,
    fg: Color,
    bg: Color,
    modifiers: Modifiers,
}

/// Compute minimal updates
pub fn diff(old: &Buffer, new: &Buffer) -> Vec<Command>;
```

### 7. Event Layer (`src/event/`)

Keyboard input and event dispatch.

```
src/event/
├── mod.rs
├── handler.rs       # Event handler registry
├── focus.rs         # Focus management
├── keymap.rs        # Key binding system
└── dispatch.rs      # Event dispatching
```

**Event Flow:**

```
Terminal Input → crossterm::Event → Dispatch → Focused Widget → Bubble Up
                       │                │              │
                   read_event()    find_focus()   on_key()
```

**Focus Management:**

```rust
pub struct FocusManager {
    focus_order: Vec<WidgetId>,
    current: Option<usize>,
}

impl FocusManager {
    pub fn next(&mut self);      // Tab
    pub fn prev(&mut self);      // Shift+Tab
    pub fn focus(&mut self, id: WidgetId);
}
```

### 8. App Layer (`src/app/`)

Application lifecycle and coordination.

```
src/app/
├── mod.rs
├── builder.rs       # App builder pattern
├── runtime.rs       # Main event loop
├── router.rs        # Screen routing
└── layer.rs         # Layer (z-index) management
```

**App Lifecycle:**

```
App::builder() → .style() → .build() → .run(view, handler)
                     │          │              │
               load CSS    create App     event loop
```

**Event Loop:**

```rust
loop {
    // 1. Poll events
    if let Some(event) = poll_event()? {
        dispatch_event(event);
    }

    // 2. Process reactive updates
    reactive_runtime.flush();

    // 3. Layout
    layout_engine.compute();

    // 4. Render
    if needs_render {
        render_frame();
    }

    // 5. Handle async tasks
    tokio::task::yield_now().await;
}
```

## Module Dependencies

```
                    ┌─────────┐
                    │   app   │
                    └────┬────┘
                         │
         ┌───────────────┼───────────────┐
         │               │               │
    ┌────▼────┐    ┌─────▼─────┐   ┌─────▼─────┐
    │ widget  │    │  reactive │   │   event   │
    └────┬────┘    └───────────┘   └───────────┘
         │
    ┌────▼────┐
    │  style  │
    └────┬────┘
         │
    ┌────▼────┐
    │ layout  │
    └────┬────┘
         │
    ┌────▼────┐
    │ render  │◄─────┐
    └────┬────┘      │
         │      ┌────┴────┐
         │      │  text   │
         │      └─────────┘
    ┌────▼────┐
    │crossterm│
    └─────────┘
```

## Data Flow

### 1. Initial Render

```
1. App::run()
2. Load CSS → StyleSheet
3. Call mount() → Widget Tree
4. Resolve styles for each widget
5. Build taffy tree
6. Compute layout
7. Render to buffer
8. Flush to terminal
```

### 2. User Input

```
1. crossterm::read() → Event
2. Dispatch to focused widget
3. Widget handles event
4. Signal::set() if state changes
5. Notify subscribers
6. Schedule re-render
7. Re-render affected widgets
```

### 3. Hot Reload (CSS)

```
1. notify::watch() detects file change
2. Re-parse CSS file
3. Invalidate style cache
4. Recompute styles
5. Re-layout if needed
6. Re-render
```

## Thread Model

```
┌──────────────────────────────────────┐
│            Main Thread               │
│  ┌────────────────────────────────┐  │
│  │        Event Loop              │  │
│  │  - Input polling               │  │
│  │  - Reactive updates            │  │
│  │  - Rendering                   │  │
│  └────────────────────────────────┘  │
└──────────────────────────────────────┘
              │
              │ spawn
              ▼
┌──────────────────────────────────────┐
│          Tokio Runtime               │
│  - Async I/O (file, network)        │
│  - Timers                           │
│  - Background tasks                 │
└──────────────────────────────────────┘
```

## Error Handling

```rust
/// Framework-level errors
#[derive(thiserror::Error, Debug)]
pub enum RevueError {
    #[error("CSS parse error: {0}")]
    CssParseError(String),

    #[error("Layout error: {0}")]
    LayoutError(String),

    #[error("Render error: {0}")]
    RenderError(#[from] std::io::Error),

    #[error("Widget error: {0}")]
    WidgetError(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, RevueError>;
```

## Extension Points

### Custom Widgets

```rust
pub struct MyWidget {
    // ...
}

impl View for MyWidget {
    fn render(&self, ctx: &mut RenderContext) -> Element {
        // Custom rendering logic
    }
}
```

### Custom CSS Properties

```rust
// Register custom property
style_registry.register_property(
    "--my-custom-prop",
    PropertyType::Color,
    default_value,
);
```

### Plugins

```rust
pub trait Plugin {
    fn name(&self) -> &str;
    fn init(&self, app: &mut App);
}

app.plugin(MyPlugin::new());
```
