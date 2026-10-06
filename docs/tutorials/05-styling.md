# Styling with CSS

Learn how to style your Revue apps using CSS.

## Loading Stylesheets

```rust
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)  // Changes apply without restart
    .build();
```

A rule reaches every widget its parent renders as a child, so in
`vstack().child(Text::new("Hi").class("title"))` a `.title` rule styles the
text. Colors apply to every widget; borders to widgets that draw one; text
properties to `Text`. `width`, `height`, `margin`, `min-*`/`max-*`, `display`
and `gap` adjust the box a container gave the widget, while the container
still decides the flow. Some properties parse but do nothing yet - see
[Not applied yet](#not-applied-yet). Before 3.0 only the root widget was
styled unless the app opted in - see the
[migration guide](../migration/v3.0.0.md#1-css-reaches-every-widget-by-default).

Put classes on children: a widget your view renders as its whole body
(`vstack()...render(ctx)`) *is* the view's own node, so a class on it matches
nothing.

## Selectors

### Type Selector

Match widgets by type:

```css
Text {
    color: cyan;
}

Button {
    background: #24283b;
}
```

### Class Selector

```css
.title {
    font-weight: bold;
    color: white;
}

.muted {
    color: gray;
}
```

Apply in Rust:

```rust
Text::new("Hello").class("title")
Text::new("Subtitle").class("muted")
```

### ID Selector

```css
#main-panel {
    border: rounded cyan;
    margin: 1;
}
```

Apply in Rust:

```rust
Border::new().element_id("main-panel")
```

### Combined Selectors

```css
Button.primary {
    background: #7aa2f7;
    color: #1a1b26;
}

Text.error {
    color: red;
    font-weight: bold;
}
```

## Properties

### Colors

```css
.widget {
    color: cyan;                /* Named */
    color: #7aa2f7;             /* Hex */
    color: rgb(122, 162, 247);  /* RGB */
    background: #1a1b26;
}
```

Named colors: `red`, `green`, `blue`, `cyan`, `magenta`, `yellow`, `white`, `black`

### Text

Read by `Text`:

```css
.text {
    font-weight: bold;
    text-decoration: underline;   /* underline, line-through, none */
    text-align: center;           /* left, center, right */
}
```

### Borders

On widgets that draw a border (`Border`, `Card`, `Accordion`, `Alert`, ...):

```css
.panel {
    border-style: double;   /* solid, double, rounded, dashed, none */
    border-color: cyan;
}

/* Shorthand */
.box {
    border: rounded cyan;
}
```

### Box

```css
.sidebar {
    width: 20;          /* cells, or a percentage: 30% */
    max-width: 40;
    margin: 1;          /* also margin-top / -right / -bottom / -left */
}

.hidden {
    display: none;      /* takes no space */
}

.list {
    gap: 1;             /* vstack / hstack / grid: between children */
}
```

The container still places each child; these adjust the box it was handed.

**Nested stacks.** A content-sized stack sizes a child stack from that
stack's content without its CSS, so a `margin` or `gap` a stylesheet adds
*inside* the child is not reserved and the child's last line is cut. Give the
child's slot a size - `child_flex(.., 1.0)` or `child_sized(.., n)` - when its
contents carry vertical margins or a CSS `gap`. (Lifting this needs `measure`
to see styles; planned for 3.x.)

### Not applied yet

`padding`, `flex-direction`, `justify-content`, `align-items`, `flex`,
`order`, `flex-wrap`, `grid-template-*`, `opacity`, `font-style`, `z-index`,
`position` and `transition` parse but do nothing yet (planned for 3.x). The
container's builder decides layout instead: `vstack()` / `hstack()`,
`child_sized`, `child_flex`, `Text::align`. See the
[Styling Guide](../guides/styling.md#parsed-but-not-applied-yet-planned-for-3x)
for the full list.

## CSS Variables

Define reusable values:

```css
:root {
    --bg: #1a1b26;
    --fg: #c0caf5;
    --accent: #7aa2f7;
    --success: #9ece6a;
    --error: #f7768e;
}

.panel {
    background: var(--bg);
    color: var(--fg);
}

Button.primary {
    background: var(--accent);
}
```

`var()` has to be the whole value: `border-color: var(--accent)` works,
`border: rounded var(--accent)` does not.

## Pseudo-Classes

> `:hover` and `:focus` follow the mouse because the app builds its DOM from
> the render traversal ([`dom_from_render`](../guides/app-builder.md#dom_from_renderenabled),
> on by default since 3.0). That is what associates every widget with a screen
> area, so the pointer can find it; a left click focuses the nearest enclosing
> input widget, which is what makes `:focus` match. With
> `dom_from_render(false)` no widget below the root has an area and these rules
> never match.
>
> Keyboard is the primary modality in a terminal - treat hover as an
> enhancement, and never put information only there.

### Interactive States

```css
Button:hover {
    background: #8ab4f8;
}

Button:focus {
    background: #7aa2f7;
    color: #1a1b26;
}

Button:disabled {
    color: gray;
}
```

`:checked` and `:selected` parse, but nothing sets that state in a running
app yet.

## Themes

### Built-in Themes

```rust
use revue::style::Themes;

set_theme(Themes::dracula());
set_theme(Themes::nord());
set_theme(Themes::monokai());
set_theme(Themes::solarized_dark());
```

### Theme Switching

```rust
// Toggle between light/dark
toggle_theme();

// Cycle through themes
cycle_theme();

// Get current theme
let theme = use_theme();
```

### Custom Themes

```rust
let my_theme = Themes::custom("my-theme")
    .primary(Color::rgb(122, 162, 247))
    .background(Color::rgb(26, 27, 38))
    .text(Color::rgb(192, 202, 245))
    .build();

register_theme(my_theme);
set_theme_by_id("my-theme");
```

## Complete Example

```css
/* styles.css */
:root {
    --bg: #1a1b26;
    --fg: #c0caf5;
    --accent: #7aa2f7;
    --border: #565f89;
}

.panel {
    border-style: rounded;
    border-color: var(--border);
    background: var(--bg);
    color: var(--fg);
}

.title {
    color: var(--accent);
    font-weight: bold;
    margin-bottom: 1;
}

Button:hover {
    background: var(--accent);
    color: var(--bg);
}

Button.primary {
    background: var(--accent);
    color: var(--bg);
}
```

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .style("styles.css")
        .hot_reload(true)
        .build();

    app.run(MyApp, |event, _view, app| {
        if let Event::Key(k) = event {
            if k.key == Key::Char('q') {
                app.quit();  // Only app.quit() exits; returning a bool controls redraw
            }
        }
        true
    })
}

struct MyApp;

impl View for MyApp {
    fn render(&self, ctx: &mut RenderContext) {
        // The panel is a child, so `.panel` has a node to match; a class on
        // the outer vstack - this view's own body - would not. `child_flex`
        // gives the panel the rest of the screen, so the title's CSS margin
        // has room.
        vstack()
            .child_flex(
                Border::new().class("panel").child(
                    vstack()
                        .child(Text::new("Styled App").class("title"))
                        .child(Input::new().placeholder("Enter text..."))
                        .child(Button::new("Submit").class("primary")),
                ),
                1.0,
            )
            .render(ctx);
    }
}
```

## Next Steps

- [Forms Tutorial](./06-forms.md) - Form handling with validation
- [Theme Switcher Example](../../examples/theme_switcher.rs) - Runtime theme switching
- [Styling Guide](../guides/styling.md) - Full CSS reference
