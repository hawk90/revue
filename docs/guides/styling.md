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
    border: rounded;
    padding: 1;
}

/* Combining selectors */
Button.primary {
    background: #7aa2f7;
    color: #1a1b26;
}
```

### Applying Classes

```rust
Text::new("Hello")
    .class("title")

Button::new("Submit")
    .id("submit-btn")
    .class("primary")
```

## CSS Properties

### Colors

```css
.widget {
    color: cyan;              /* Named color */
    color: #7aa2f7;           /* Hex color */
    color: rgb(122, 162, 247); /* RGB */
    background: #1a1b26;
}
```

Built-in colors: `red`, `green`, `blue`, `cyan`, `magenta`, `yellow`, `white`, `black`

### Text Styling

```css
.text {
    font-weight: bold;    /* bold, normal */
    font-style: italic;   /* italic, normal */
    text-decoration: underline; /* underline, none */
}
```

### Borders

```css
.panel {
    border: solid;        /* solid, double, rounded, thick, none */
    border-color: white;
}

/* Border shorthand */
.box {
    border: rounded cyan;
}
```

### Spacing

```css
.container {
    padding: 1;           /* All sides */
    padding: 1 2;         /* Vertical, horizontal */
    padding: 1 2 1 2;     /* Top, right, bottom, left */
    margin: 1;
    gap: 1;               /* Space between children */
}
```

### Layout

```css
.stack {
    flex-direction: column;  /* column, row */
    align-items: center;     /* start, center, end, stretch */
    justify-content: center; /* start, center, end, space-between */
}
```

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

Button.primary {
    background: var(--accent);
}
```

## Pseudo-Classes

```css
/* Interactive states */
Button:hover {
    background: #8ab4f8;
}

Button:focus {
    border: double var(--accent);
}

Button:disabled {
    opacity: 0.5;
}

/* Selection state */
List-item:selected {
    background: var(--accent);
    color: var(--bg-primary);
}
```

## Transitions

Smooth property changes:

```css
Button {
    background: #24283b;
    transition: background 0.3s ease;
}

Button:hover {
    background: #7aa2f7;
}
```

Transition syntax: `property duration easing`

Easing functions: `linear`, `ease`, `ease-in`, `ease-out`, `ease-in-out`

## Themes

### Built-in Themes

```rust
use revue::style::Themes;

// Apply a theme
set_theme(Themes::dracula());
set_theme(Themes::nord());
set_theme(Themes::gruvbox());
set_theme(Themes::catppuccin());
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

let my_theme = ThemeBuilder::new("my-theme")
    .bg_primary(Color::rgb(26, 27, 38))
    .fg_primary(Color::rgb(192, 202, 245))
    .accent(Color::rgb(122, 162, 247))
    .build();

register_theme(my_theme);
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

## Example: Complete Stylesheet

```css
/* styles.css */
:root {
    --bg: #1a1b26;
    --fg: #c0caf5;
    --accent: #7aa2f7;
    --border: #565f89;
}

/* Base styles */
* {
    color: var(--fg);
}

/* Layout containers */
.panel {
    border: rounded var(--border);
    padding: 1;
    background: var(--bg);
}

.header {
    padding: 0 1;
    margin-bottom: 1;
}

/* Buttons */
Button {
    padding: 0 2;
    border: solid var(--border);
    transition: all 0.2s ease;
}

Button:hover {
    border-color: var(--accent);
    color: var(--accent);
}

Button:focus {
    border: double var(--accent);
}

Button.primary {
    background: var(--accent);
    color: var(--bg);
}

/* Inputs */
Input {
    border: solid var(--border);
    padding: 0 1;
}

Input:focus {
    border-color: var(--accent);
}

/* Lists */
List-item:selected {
    background: var(--accent);
    color: var(--bg);
}

/* Progress */
Progress {
    color: var(--accent);
}
```
