# Styling Guide

Revue uses CSS for styling, bringing familiar web development patterns to terminal UIs.

## CSS Basics

### Loading Stylesheets

```rust
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)  // Auto-reload on file changes
    .build();
```

A rule reaches every widget its parent renders as a child, so in
`vstack().child(Text::new("Hi").class("title"))` a `.title` rule styles the
text. Colors apply to every widget; borders to widgets that draw one; text
properties to `Text`. `width`, `height`, `margin`, `min-*`/`max-*`, `display`
and `gap` adjust the box a container gave the widget, while the container
still decides the flow. Some properties
parse but do nothing yet - see [Not applied yet](#parsed-but-not-applied-yet-planned-for-3x).
Before 3.0 only the root widget was styled unless the app opted in - see the
[migration guide](../migration/v3.0.0.md#1-css-reaches-every-widget-by-default).

**Put classes on children.** A widget your view renders as its whole body -
`vstack()...render(ctx)` at the end of `render` - *is* the view's own node, so
a class on it matches nothing. Select it by the view's type (the name its
`widget_type` / `WidgetMeta` reports), or wrap the part you want to style in a
child.

### Basic Selectors

```css
/* Type selector - matches widget type */
Text {
    color: cyan;
}

/* Class selector */
.title {
    color: white;
    font-weight: bold;
}

/* ID selector */
#main-panel {
    border: rounded cyan;
    margin: 1;
}

/* Combining selectors */
Button.primary {
    background: #7aa2f7;
    color: #1a1b26;
}
```

### Applying Classes

```rust
vstack()
    .child(Text::new("Hello").class("title"))
    .child(Border::new().element_id("main-panel").child(Text::new("Body")))
    .child(Button::new("Submit").element_id("submit-btn").class("primary"))
```

## CSS Properties

### Colors

```css
.widget {
    color: cyan;                    /* Named color */
    color: #7aa2f7;                 /* Hex color */
    color: #f00;                    /* Short hex */
    color: rgb(122, 162, 247);      /* RGB */
    color: hsl(220, 90%, 72%);      /* HSL */
    color: hsla(120, 100%, 50%, 1); /* HSLA */
    background: #1a1b26;
}
```

50+ CSS named colors: `red`, `green`, `blue`, `cyan`, `magenta`, `yellow`, `white`, `black`, `orange`, `coral`, `salmon`, `pink`, `hotpink`, `deeppink`, `purple`, `rebeccapurple`, `indigo`, `violet`, `gold`, `lime`, `olive`, `teal`, `navy`, `royalblue`, `dodgerblue`, `skyblue`, `brown`, `maroon`, `crimson`, `gray`/`grey`, `silver`, and more.

`color` inherits to children. A `background` fills the widget's whole box,
under whatever the widget paints; a widget's own builder background (`.bg(...)`)
wins over the stylesheet.

### Text Styling

Read by `Text`:

```css
.text {
    font-weight: bold;              /* bold, normal */
    text-decoration: underline;     /* underline, line-through, none */
    text-align: center;             /* left, center, right */
    visibility: hidden;             /* visible, hidden */
}
```

A builder `.align(..)` wins over `text-align`. `font-weight` and
`text-decoration` can only turn a flag on: a stylesheet cannot un-bold a
`Text` built with `.bold()`.

### Borders

On widgets that draw a border - `Border`, `Card`, `Accordion`, `Alert`,
`Popover`, `Form`, `ErrorBoundary`, `DropZone`:

```css
.panel {
    border-style: double;   /* solid, double, rounded, dashed, none */
    border-color: white;
}

/* Shorthand: <style> [color] */
.box {
    border: rounded cyan;
}
```

`dashed` draws a single line - terminals have no dashed box-drawing set.
`border-style: none` removes a border the builder drew.

### Box

```css
.sidebar {
    width: 20;              /* cells, or a percentage: 30% */
    min-height: 3;
    max-width: 40;
    margin: 1;              /* also 1 2, 1 2 1 2, and margin-top/-right/-bottom/-left */
}

.hidden {
    display: none;          /* takes no space, keeps its node */
}

.list {
    gap: 1;                 /* vstack / hstack / grid: space between children */
    column-gap: 2;          /* grid */
    row-gap: 1;             /* grid */
    overflow: hidden;       /* Border / stacks: clip children to the box */
}

.tags {
    flex-wrap: wrap;        /* hstack: what does not fit goes to the next line */
}
```

The container still places each child; these adjust the box it was handed.

**Overflow.** As in CSS, `overflow` defaults to `visible`: a child whose
`width`/`height` makes it larger than the slot its container gave it paints
past that slot, over its neighbors. Put `overflow: hidden` on the container to
keep it inside. The clip covers everything the subtree draws, including widgets
that write to `ctx.buffer` directly (canvases, custom widgets); overlays such as
dropdowns, tooltips and toasts are drawn after the tree and are not clipped.
In a content-sized `vstack`/`hstack` a child's `height`/`width`, `min-*`/`max-*`
and margins along the stack count toward its slot, so `margin-top: 2` moves a
line of text down. `gap: 0` closes a gap the builder opened.

**Nested stacks.** A stack inside a stack is sized with its CSS: its own
`gap`, and the sizes and margins of its children - and of their children,
at any depth - all count, so `.form { gap: 1 }` on a form column inside the
screen's `vstack` keeps its last field on screen. Another widget that sizes
itself from its content but wraps styled spacing (say, a `Border` around a
`.form` stack with a `gap`) cannot add that spacing up, so it takes the
space left over instead of being cut short.

**`flex-wrap: wrap`** on an `hstack` moves items that do not fit onto the
next line. Each line is as tall as its tallest item, `gap` separates the
items and the lines, and a content-sized parent reserves every line.

### Parsed but not applied yet (planned for 3.x)

The parser accepts these without an error, but nothing acts on them at paint
time yet. Builders cover what they would do:

| Property | Use instead |
|---|---|
| `padding`, `padding-*` | `margin` on the child, or a builder padding where the widget has one |
| `flex-direction` | `vstack()` / `hstack()` |
| `justify-content`, `align-items`, `align-self` | `Text::align`, `child_sized`, `Positioned` |
| `flex`, `flex-grow`, `flex-shrink`, `flex-basis`, `order` | `child_flex(..)`, `child_sized(..)`, child order |
| `grid-template-*`, `grid-row`, `grid-column` | `Grid` builders |
| `opacity` | `Text::dim()` |
| `font-style` | `.italic()` where the widget has it |
| `z-index`, `position`, `top`/`right`/`bottom`/`left` | `Positioned`, `Layers` |
| `transition`, `animation` | `revue::style::TransitionManager` driven from your own code |
| `color: transparent` | parses as black; a cell has no alpha |

`:active`, `:checked` and `:selected` parse and match node state, but nothing
sets that state in a running app yet, so rules naming them never match.

## CSS Variables

Define reusable values:

```css
:root {
    --bg-primary: #1a1b26;
    --bg-secondary: #24283b;
    --fg-primary: #c0caf5;
    --accent: #7aa2f7;
    --success: #9ece6a;
    --error: #f7768e;
}

.panel {
    background: var(--bg-secondary);
    color: var(--fg-primary);
}

/* Fallback values for undefined variables */
.button {
    background: var(--accent, orange);  /* uses orange if --accent undefined */
    color: var(--text, white);
}
```

`var()` works anywhere in a value, so shorthands take it too:
`border: rounded var(--accent)`, `margin: var(--gap-y) var(--gap-x)`. A
variable's value and a fallback may themselves use `var()`. An undefined
variable with no fallback makes the declaration do nothing.

## Pseudo-Classes

> `:hover` and `:focus` follow the mouse because the app builds its DOM from
> the render traversal ([`dom_from_render`](app-builder.md#dom_from_renderenabled),
> on by default since 3.0). That is what associates every widget with a screen
> area, so the pointer can find it; a left click focuses the nearest enclosing
> input widget, which is what makes `:focus` match. With
> `dom_from_render(false)` no widget below the root has an area and these rules
> never match. [`tab_navigation(true)`](app-builder.md#tab_navigationenabled)
> lets Tab move `:focus` too.
>
> Keyboard is the primary modality in a terminal - treat hover as an
> enhancement, and never put information only there.

```css
/* Interactive states */
Button:hover {
    background: #8ab4f8;
}

Button:focus {
    background: #7aa2f7;
    color: #1a1b26;
}

/* Button::new("Save").disabled(true) */
Button:disabled {
    color: gray;
}
```

### Structural pseudo-classes

`:first-child`, `:last-child`, `:only-child` and `:nth-child(An+B)` (with
`odd` / `even`) count a widget's siblings in the DOM:

```css
.row:nth-child(odd)  { background: #2a2a3a; }
.row:nth-child(even) { background: #1e1e2e; }
.row:nth-child(-n+3) { font-weight: bold; }  /* first 3 rows */
```

```rust
let rows = ["alpha", "beta", "gamma", "delta"]
    .iter()
    .fold(vstack(), |list, name| list.child(Text::new(*name).class("row")));
```

## Themes

### Built-in Themes

```rust
use revue::style::Themes;

// Apply a theme
set_theme(Themes::dracula());
set_theme(Themes::nord());
set_theme(Themes::monokai());
set_theme(Themes::solarized_dark());
```

### Reactive Themes

```rust
// Get current theme as signal
let theme = use_theme();

// Toggle between themes
toggle_theme();

// Cycle through themes
cycle_theme();
```

### Custom Themes

```rust
use revue::style::{Theme, ThemeBuilder};

let my_theme = Themes::custom("my-theme")
    .background(Color::rgb(26, 27, 38))
    .text(Color::rgb(192, 202, 245))
    .primary(Color::rgb(122, 162, 247))
    .build();

register_theme("my-theme", my_theme);
set_theme_by_id("my-theme");
```

## Inline Styles

For one-off styling:

```rust
Text::new("Colored")
    .fg(Color::CYAN)
    .bg(Color::rgb(40, 40, 40))
    .bold()
```

A custom widget that keeps a `WidgetProps` and uses `impl_view_meta!` can take
a whole `Style` through `WidgetProps::style`. It reaches the widget's DOM node as
an inline style: the cascade applies it after every stylesheet rule, so it wins
over `#id` and class rules, and the widget reads it back through `ctx.css_color`
and friends.

```rust
let props = WidgetProps::new().id("swatch").style(my_style);
```

## Example: Complete Stylesheet

```css
/* styles.css */
:root {
    --bg: #1a1b26;
    --fg: #c0caf5;
    --accent: #7aa2f7;
    --border: #565f89;
}

/* Inherited by everything below the root */
* {
    color: var(--fg);
}

.panel {
    border-style: rounded;
    border-color: var(--border);
    background: var(--bg);
}

.header {
    color: var(--accent);
    font-weight: bold;
    margin-bottom: 1;
}

/* Buttons */
Button {
    background: var(--border);
}

Button:hover {
    background: var(--accent);
    color: var(--bg);
}

Button:focus {
    background: var(--accent);
    color: var(--bg);
}

Button.primary {
    background: var(--accent);
    color: var(--bg);
}

/* Inputs */
Input {
    background: var(--bg);
}

/* Progress */
Progress {
    color: var(--accent);
}
```

```rust
use revue::prelude::*;

struct Settings;

impl View for Settings {
    fn render(&self, ctx: &mut RenderContext) {
        // The panel is a child, so `.panel` has a node to match; `child_flex`
        // gives it the rest of the screen (see "Box" for why).
        vstack()
            .child_flex(
                Border::new().class("panel").child(
                    vstack()
                        .child(Text::new("Settings").class("header"))
                        .child(Input::new().placeholder("Name"))
                        .child(Progress::new(0.4))
                        .child(Button::new("Save").class("primary")),
                ),
                1.0,
            )
            .render(ctx);
    }
}
```
