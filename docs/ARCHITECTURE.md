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

Provides the building blocks for UI construction. Contains 100+ widgets organized by category.

```
src/widget/
├── mod.rs
├── traits.rs              # View, Widget traits
├── macros.rs              # Widget builder macros
│
├── # Layout
├── stack.rs               # VStack, HStack
├── grid.rs                # Grid layout
├── scroll.rs              # Scrollable container
├── splitter.rs            # Split panes
├── layer.rs               # Z-index layering
├── positioned.rs          # Absolute positioning
├── resizable.rs           # Resizable containers
├── collapsible.rs         # Collapsible sections
├── card.rs                # Card container
│
├── # Basic
├── text.rs                # Text display
├── richtext.rs            # Styled text
├── bigtext.rs             # Large ASCII text
├── button.rs              # Button widget
├── link.rs                # Clickable links
├── divider.rs             # Horizontal/vertical divider
├── border.rs              # Border decoration
│
├── # Input
├── input.rs               # Text input
├── textarea.rs            # Multi-line editor
├── masked_input.rs        # Password/masked input
├── number_input.rs        # Numeric input
├── checkbox.rs            # Checkbox
├── radio.rs               # Radio buttons
├── switch.rs              # Toggle switch
├── select.rs              # Dropdown select
├── multi_select.rs        # Multi-select
├── slider.rs              # Slider control
├── color_picker.rs        # Color picker
├── datetime_picker.rs     # Date/time picker
├── calendar.rs            # Calendar widget
├── range_picker.rs        # Range selection
├── autocomplete.rs        # Autocomplete input
├── search_bar.rs          # Search bar
│
├── # Data Display
├── list.rs                # List widget
├── virtuallist.rs         # Virtualized list
├── option_list.rs         # Option list
├── selection_list.rs      # Selection list
├── table.rs               # Data table
├── tree.rs                # Tree view
├── filetree.rs            # File tree
│
├── # Charts (Statistical)
├── chart.rs               # Line/area chart
├── chart_common.rs        # Shared chart types (Axis, Legend, etc.)
├── chart_stats.rs         # Statistical functions (percentile, mean, bins)
├── chart_render.rs        # Common chart rendering utilities
├── barchart.rs            # Bar chart
├── piechart.rs            # Pie/donut chart
├── scatterchart.rs        # Scatter/bubble chart
├── histogram.rs           # Histogram
├── boxplot.rs             # Box-and-whisker plot
├── candlechart.rs         # Candlestick chart
├── heatmap.rs             # Heat map
├── sparkline.rs           # Sparkline
├── gauge.rs               # Gauge/meter
├── timeseries.rs          # Time series
├── streamline.rs          # Streaming chart
├── waveline.rs            # Wave chart
│
├── # Navigation
├── tabs.rs                # Tab container
├── menu.rs                # Menu bar
├── breadcrumb.rs          # Breadcrumb
├── pagination.rs          # Pagination
├── stepper.rs             # Step indicator
├── command_palette.rs     # Ctrl+P command palette
│
├── # Feedback
├── modal.rs               # Modal dialog
├── toast.rs               # Toast notifications
├── notification.rs        # Notification center
├── alert.rs               # Alert box
├── callout.rs             # Callout/admonition
├── progress.rs            # Progress bar
├── spinner.rs             # Loading spinner
├── skeleton.rs            # Skeleton loader
├── status_indicator.rs    # Status dot
├── tooltip.rs             # Tooltip
├── badge.rs               # Badge
├── tag.rs                 # Tag/chip
├── avatar.rs              # Avatar
├── rating.rs              # Star rating
│
├── # Content
├── markdown.rs            # Markdown renderer
├── markdown_presentation.rs # Slidev-style presentations
├── presentation.rs        # Presentation mode
├── slides.rs              # Slide widget
├── syntax.rs              # Syntax highlighting
├── diff.rs                # Diff viewer
├── mermaid.rs             # Mermaid diagrams
├── qrcode.rs              # QR code
├── image.rs               # Kitty image protocol
├── canvas.rs              # Drawing canvas
│
├── # Advanced
├── terminal.rs            # Embedded terminal
├── httpclient.rs          # HTTP client widget
├── aistream.rs            # AI streaming widget
├── filepicker.rs          # File picker dialog
├── dropzone.rs            # Drag-and-drop zone
├── sortable.rs            # Sortable list
├── vim.rs                 # Vim mode
├── theme_picker.rs        # Theme selector
├── timer.rs               # Timer widget
├── digits.rs              # Digital display
├── procmon.rs             # Process monitor
├── richlog.rs             # Rich log viewer
├── accordion.rs           # Accordion
├── zen.rs                 # Zen mode
├── screen.rs              # Screen widget
├── statusbar.rs           # Status bar
├── timeline.rs            # Timeline
└── debug_overlay.rs       # Debug overlay
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
