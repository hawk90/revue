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

Provides the building blocks for UI construction. Contains 100+ widgets grouped into category modules under `src/widget/`. The tree below is representative, not exhaustive — each category holds more widgets than shown.

```
src/widget/
├── mod.rs
├── traits/                # View, Widget traits
├── macros.rs              # Widget builder macros
│
├── layout/                # Containers & structure
│   ├── stack.rs           #   VStack, HStack
│   ├── grid/              #   Grid layout
│   ├── card/              #   Card container
│   ├── border.rs          #   Border decoration
│   ├── scroll.rs          #   Scrollable container
│   ├── splitter.rs        #   Split panes
│   ├── tabs.rs            #   Tab container
│   ├── accordion/         #   Accordion
│   ├── sidebar/           #   Sidebar
│   ├── layer.rs           #   Z-index layering
│   └── collapsible.rs     #   Collapsible sections
│
├── input/                 # Interactive input widgets
│   └── input_widgets/
│       ├── input/         #   Text input
│       ├── textarea/      #   Multi-line editor
│       ├── button.rs      #   Button
│       ├── checkbox.rs    #   Checkbox
│       ├── radio.rs       #   Radio buttons
│       ├── switch.rs      #   Toggle switch
│       ├── select/        #   Dropdown select
│       ├── slider.rs      #   Slider control
│       ├── color_picker/  #   Color picker
│       ├── autocomplete/  #   Autocomplete input
│       └── stepper.rs     #   Step indicator
│
├── data/                  # Data display & visualization
│   ├── list.rs            #   List widget
│   ├── virtuallist/       #   Virtualized list
│   ├── table.rs           #   Data table
│   ├── tree/              #   Tree view
│   ├── filetree/          #   File tree
│   ├── datagrid/          #   Data grid
│   ├── calendar/          #   Calendar
│   ├── timeline.rs        #   Timeline
│   └── chart/             #   Charts
│       ├── barchart.rs    #     Bar chart
│       ├── piechart.rs    #     Pie/donut chart
│       ├── scatterchart.rs#     Scatter/bubble chart
│       ├── histogram/     #     Histogram
│       ├── boxplot/       #     Box-and-whisker plot
│       ├── candlechart.rs #     Candlestick chart
│       ├── heatmap/       #     Heat map
│       ├── sparkline.rs   #     Sparkline
│       └── timeseries/    #     Time series
│
├── display/               # Static/decorative display
│   ├── text.rs            #   Text display
│   ├── richtext.rs        #   Styled text
│   ├── bigtext.rs         #   Large ASCII text
│   ├── badge.rs           #   Badge
│   ├── tag.rs             #   Tag/chip
│   ├── avatar.rs          #   Avatar
│   ├── progress.rs        #   Progress bar
│   ├── spinner.rs         #   Loading spinner
│   ├── skeleton.rs        #   Skeleton loader
│   ├── gauge.rs           #   Gauge/meter
│   └── divider.rs         #   Divider
│
├── feedback/              # Overlays & notifications
│   ├── modal/             #   Modal dialog
│   ├── toast.rs           #   Toast notifications
│   ├── notification/      #   Notification center
│   ├── alert.rs           #   Alert box
│   ├── callout/           #   Callout/admonition
│   ├── tooltip.rs         #   Tooltip
│   ├── popover/           #   Popover
│   ├── menu/              #   Menu
│   └── statusbar.rs       #   Status bar
│
├── developer/             # Developer-facing widgets
│   ├── terminal/          #   Embedded terminal
│   ├── code_editor/       #   Code editor
│   ├── httpclient/        #   HTTP client widget
│   ├── aistream.rs        #   AI streaming widget
│   ├── diff.rs            #   Diff viewer
│   ├── procmon.rs         #   Process monitor
│   ├── vim.rs             #   Vim mode
│   └── tree_sitter_highlight.rs # Syntax highlighting
│
├── markdown/              # Markdown renderer (parser, types, helpers)
│
└── # Additional top-level widgets & categories
    ├── syntax/            # Syntax highlighting support
    ├── mermaid/           # Mermaid diagrams
    ├── canvas/            # Drawing canvas
    ├── command_palette/   # Ctrl+P command palette
    ├── breadcrumb/        # Breadcrumb
    ├── datetime_picker/   # Date/time picker
    ├── range_picker/      # Range selection
    ├── multi_select/      # Multi-select
    ├── filepicker/        # File picker dialog
    ├── dropzone/          # Drag-and-drop zone
    ├── sortable/          # Sortable list
    ├── form/              # Form container
    ├── qrcode.rs          # QR code
    ├── image.rs           # Kitty image protocol
    ├── link.rs            # Clickable links
    ├── pagination.rs      # Pagination
    ├── slides.rs          # Slide widget
    ├── digits.rs          # Digital display
    ├── theme_picker.rs    # Theme selector
    ├── zen.rs             # Zen mode
    └── debug_overlay/     # Debug overlay
```

**Key Traits:**

```rust
/// Core trait for all renderable components
pub trait View {
    fn render(&self, ctx: &mut RenderContext);
}
```

### 2. Style Layer (`src/runtime/style/`)

CSS parsing, selector matching, and style computation. The CSS engine is a
custom, hand-written parser — Revue does **not** depend on `cssparser` or the
`selectors` crate.

```
src/runtime/style/
├── mod.rs
├── parser/          # Custom CSS parser (scanner, tokenizer, parse, apply)
├── properties/      # CSS property definitions
├── computed.rs      # Computed style values
├── transition.rs    # CSS transitions
├── animation/       # @keyframes / animation support
├── theme.rs         # Theme definitions
└── error.rs         # ParseError
```

Selector matching lives alongside the DOM/cascade code in `src/runtime/dom/`
(`selector/`, `cascade/`).

**Style Resolution Flow:**

```
CSS File → Parse → Selector Match → Cascade → Compute → Apply
    │                    │              │         │
custom parser      dom selector    specificity  inheritance
```

**Supported CSS Properties:**

| Category | Properties |
|----------|------------|
| Layout | `display`, `position`, `flex-direction`, `flex-wrap`, `flex-grow`, `flex`, `justify-content`, `align-items`, `align-self`, `order`, `gap`, `column-gap`, `row-gap` |
| Spacing | `padding`, `margin`, `width`, `height`, `min-*`, `max-*`, `top`, `right`, `bottom`, `left` |
| Border | `border` (shorthand), `border-style`, `border-color` |
| Colors | `color`, `background` — formats: hex, rgb, hsl/hsla, 50+ named colors, `transparent` |
| Text | `text-align`, `font-weight`, `text-decoration` |
| Visual | `opacity`, `visibility`, `overflow`, `z-index` |
| Variables | `:root { --name: value; }`, `var(--name)`, `var(--name, fallback)` |
| Selectors | `:nth-child(odd/even/An+B)`, `:focus`, `:hover`, `:disabled`, `:not()` |
| Animation | `transition`, `@keyframes`, animation shorthand |

### 3. Reactive Layer (`src/state/reactive/`)

Vue-inspired reactivity system.

```
src/state/reactive/
├── mod.rs
├── signal.rs        # Signal<T> - reactive state
├── signal_vec.rs    # SignalVec<T> - reactive collections
├── computed.rs      # Computed<T> - derived values
├── effect.rs        # Effect - side effects
├── context.rs       # Reactive context/scope management
├── batch.rs         # Batched updates
├── tracker.rs       # Dependency tracking
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

### 4. Layout Layer (`src/runtime/layout/`)

Custom flexbox and grid layout engine. Revue implements its own layout solver —
it does **not** depend on `taffy`.

```
src/runtime/layout/
├── mod.rs
├── engine.rs        # Layout engine entry point
├── node.rs          # Layout node model
├── tree.rs          # Layout tree management
├── compute.rs       # Layout computation
├── flex.rs          # Flexbox algorithm
├── grid.rs          # Grid algorithm
├── block.rs         # Block layout
├── position.rs      # Absolute/relative positioning
└── responsive.rs    # Responsive breakpoints
```

**Layout Process:**

```
Widget Tree → Style Resolution → Layout Tree → Compute → Position Map
     │              │                 │            │
  build()     apply_styles()     add_node()   compute()
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

### 6. Render Layer (`src/runtime/render/`)

Double buffering and terminal output.

```
src/runtime/render/
├── mod.rs
├── buffer.rs          # Double buffer implementation
├── diff.rs            # Buffer diff algorithm
├── cell.rs            # Terminal cell representation
├── batch.rs           # Batched draw commands
├── image_protocol.rs  # Kitty image protocol
├── backend/           # Rendering backends
└── terminal/          # Terminal driver
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

### 7. Event Layer (`src/runtime/event/`)

Keyboard input and event dispatch.

```
src/runtime/event/
├── mod.rs
├── reader.rs        # Terminal event reader
├── handler.rs       # Event handler registry
├── focus.rs         # Focus management
├── keymap.rs        # Key binding system
├── click.rs         # Mouse click handling
├── drag.rs          # Drag handling
├── gesture/         # Gesture recognition
└── ime.rs           # IME / composition input
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

### 8. App Layer (`src/core/app/`)

Application lifecycle and coordination.

```
src/core/app/
├── mod.rs
├── builder.rs             # App builder pattern
├── router.rs              # Screen routing
├── declarative_router/    # Declarative routing
├── screen/                # Screen management
├── hot_reload.rs          # CSS hot reload
├── inspector.rs           # Widget inspector
├── profiler.rs            # Performance profiler
└── snapshot.rs            # State snapshots
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
5. Build layout tree
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
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("CSS error: {0}")]
    Css(#[from] style::ParseError),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Layout error: {0}")]
    Layout(#[from] layout::LayoutError),

    #[error("Render error: {0}")]
    Render(String),

    #[error("Unexpected error: {0}")]
    Other(#[from] anyhow::Error),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;
```

## Extension Points

### Custom Widgets

```rust
pub struct MyWidget {
    // ...
}

impl View for MyWidget {
    fn render(&self, ctx: &mut RenderContext) {
        // Custom rendering logic — all draw_* coordinates are relative
        // (0,0) = top-left of the widget's area
        ctx.draw_text(0, 0, "Hello", Color::WHITE);
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
