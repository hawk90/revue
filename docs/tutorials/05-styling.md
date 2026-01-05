# Styling with CSS

Learn how to style your Revue apps using CSS.

## Loading Stylesheets

```rust
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)  // Changes apply without restart
    .build();
```

## Selectors

### Type Selector

Match widgets by type:

```css
Text {
    color: cyan;
}

Button {
    padding: 0 2;
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
    border: rounded;
    padding: 1;
}
```

Apply in Rust:

```rust
Border::new().id("main-panel")
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

```css
.text {
    font-weight: bold;
    font-style: italic;
    text-decoration: underline;
}
```

### Borders

```css
.panel {
    border: solid;      /* solid, double, rounded, thick, none */
    border-color: cyan;
}

/* Shorthand */
.box {
    border: rounded cyan;
}
```

### Spacing

```css
.container {
    padding: 1;         /* All sides */
    padding: 1 2;       /* Vertical, horizontal */
    margin: 1;
    gap: 1;             /* Between children */
}
```

### Layout

```css
.stack {
    flex-direction: column;   /* column, row */
    align-items: center;      /* start, center, end */
    justify-content: center;
}
```

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

## Pseudo-Classes

### Interactive States

```css
Button:hover {
    background: #8ab4f8;
}

Button:focus {
    border: double cyan;
}

Button:disabled {
    opacity: 0.5;
}
```

### Selection

```css
List-item:selected {
    background: var(--accent);
    color: var(--bg);
}
```

## Transitions

Animate property changes:

```css
Button {
    background: #24283b;
    transition: background 0.3s ease;
}

Button:hover {
    background: #7aa2f7;
}
```

Syntax: `property duration easing`

Easing: `linear`, `ease`, `ease-in`, `ease-out`, `ease-in-out`

## Themes

### Built-in Themes

```rust
use revue::style::Themes;

set_theme(Themes::dracula());
set_theme(Themes::nord());
set_theme(Themes::gruvbox());
set_theme(Themes::catppuccin());
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
let my_theme = ThemeBuilder::new("my-theme")
    .bg_primary(Color::rgb(26, 27, 38))
    .fg_primary(Color::rgb(192, 202, 245))
    .accent(Color::rgb(122, 162, 247))
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
    border: rounded var(--border);
    padding: 1;
    background: var(--bg);
}

Button {
    padding: 0 2;
    border: solid var(--border);
    transition: all 0.2s ease;
}

Button:hover {
    border-color: var(--accent);
}

Button.primary {
    background: var(--accent);
    color: var(--bg);
}

Input {
    border: solid var(--border);
    padding: 0 1;
}

Input:focus {
    border-color: var(--accent);
}
```

```rust
use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .style("styles.css")
        .hot_reload(true)
        .build();

    app.run(MyApp, |event, _view, _app| {
        !matches!(event, Event::Key(k) if k.key == Key::Char('q'))
    })
}

struct MyApp;

impl View for MyApp {
    fn render(&self, ctx: &mut RenderContext) {
        vstack()
            .class("panel")
            .child(Text::new("Styled App").class("title"))
            .child(Input::new().placeholder("Enter text..."))
            .child(Button::new("Submit").class("primary"))
            .render(ctx);
    }
}
```

## Next Steps

- [Forms Tutorial](./06-forms.md) - Form handling with validation
- [Theme Switcher Example](../../examples/theme_switcher.rs) - Runtime theme switching
- [Styling Guide](../guides/styling.md) - Full CSS reference
