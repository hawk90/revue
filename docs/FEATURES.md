# Features

## Overview

| Category | Features |
|----------|----------|
| **Styling** | CSS files, variables, selectors, transitions |
| **Layout** | Flexbox, padding, margin, border |
| **Reactivity** | Signal, Computed, Effect |
| **Widgets** | 15+ built-in widgets |
| **Content** | Markdown, presentations, syntax highlighting, images |
| **Navigation** | Routing, focus, layers, command palette |
| **DX** | Hot reload, devtools, testing |

---

## 1. CSS Styling

### External CSS Files

```rust
App::new()
    .style("styles.css")        // Main stylesheet
    .style("theme-dark.css")    // Additional styles
```

### Selectors

```css
/* Element selector */
text {
    color: white;
}

/* Class selector */
.container {
    padding: 2;
}

/* Multiple classes */
.btn.primary {
    background: blue;
}

/* State selectors */
.btn:focus {
    border-color: cyan;
}

.btn:disabled {
    opacity: 0.5;
}

/* Descendant */
.container text {
    color: gray;
}

/* Direct child */
.container > text {
    color: white;
}
```

### CSS Variables

```css
:root {
    --color-primary: #61afef;
    --color-bg: #282c34;
    --color-text: #abb2bf;
    --spacing-sm: 1;
    --spacing-md: 2;
}

.container {
    background: var(--color-bg);
    padding: var(--spacing-md);
}

/* Override in specific context */
.dark-theme {
    --color-bg: #1e1e1e;
}
```

### Supported Properties

| Property | Values | Example |
|----------|--------|---------|
| `display` | `flex`, `block`, `none` | `display: flex;` |
| `flex-direction` | `row`, `column` | `flex-direction: column;` |
| `justify-content` | `start`, `center`, `end`, `space-between` | `justify-content: center;` |
| `align-items` | `start`, `center`, `end`, `stretch` | `align-items: center;` |
| `gap` | `<number>` | `gap: 1;` |
| `padding` | `<number>` or `<top> <right> <bottom> <left>` | `padding: 1 2;` |
| `margin` | same as padding | `margin: 1;` |
| `width` | `<number>`, `<percent>`, `auto` | `width: 50%;` |
| `height` | same as width | `height: 10;` |
| `min-width` | same as width | `min-width: 20;` |
| `max-width` | same as width | `max-width: 100;` |
| `border` | `<style> <color>` | `border: solid cyan;` |
| `border-style` | `none`, `solid`, `dashed`, `double`, `rounded` | `border-style: rounded;` |
| `color` | `<color>` | `color: #ff0000;` |
| `background` | `<color>` | `background: blue;` |
| `opacity` | `0.0` - `1.0` | `opacity: 0.5;` |
| `visibility` | `visible`, `hidden` | `visibility: hidden;` |
| `text-align` | `left`, `center`, `right` | `text-align: center;` |

### Transitions

```css
.panel {
    opacity: 0.5;
    transition: opacity 200ms ease-in-out;
}

.panel:focus {
    opacity: 1;
}

/* Multiple properties */
.btn {
    transition: background 150ms, border-color 150ms;
}
```

### Colors

```css
/* Named colors */
color: red;
color: cyan;
color: white;

/* Hex */
color: #ff0000;
color: #f00;

/* RGB */
color: rgb(255, 0, 0);

/* ANSI 256 */
color: ansi(196);
```

---

## 2. Flexbox Layout

Powered by [taffy](https://github.com/DioxusLabs/taffy), the same engine used by Dioxus and Bevy.

### Direction

```css
.row { flex-direction: row; }
.column { flex-direction: column; }
```

```
flex-direction: row          flex-direction: column
┌─────────────────────┐      ┌─────────────────────┐
│ [A] [B] [C]         │      │ [A]                 │
└─────────────────────┘      │ [B]                 │
                             │ [C]                 │
                             └─────────────────────┘
```

### Justify & Align

```css
justify-content: start | center | end | space-between | space-around

align-items: start | center | end | stretch
```

```
justify-content: space-between    align-items: center
┌─────────────────────┐           ┌─────────────────────┐
│[A]       [B]     [C]│           │                     │
└─────────────────────┘           │     [A] [B] [C]     │
                                  │                     │
                                  └─────────────────────┘
```

### Gap

```css
.container {
    gap: 1;      /* Both row and column gap */
    row-gap: 1;
    column-gap: 2;
}
```

### Sizing

```css
.sidebar {
    width: 30;          /* Fixed 30 columns */
    min-width: 20;
    max-width: 50;
}

.main {
    width: 100%;        /* Fill remaining */
    flex-grow: 1;
}
```

---

## 3. Reactivity

Vue-inspired Signal/Computed/Effect pattern.

### Signal

```rust
// Create reactive state
let count = signal(0);

// Read value
let current = count.get();  // 0

// Set value (triggers re-render)
count.set(5);

// Update based on current value
count.update(|n| n + 1);
```

### Computed

```rust
let count = signal(0);
let doubled = computed(move || count.get() * 2);

count.set(5);
doubled.get()  // 10 (automatically updated)
```

### Effect

```rust
let count = signal(0);

// Runs whenever count changes
effect(move || {
    println!("Count changed to: {}", count.get());
});
```

### Reactive Collections

```rust
let items = signal(vec!["a", "b", "c"]);

// Update collection
items.update(|v| v.push("d"));

// Reactive iteration
for item in items.get().iter() {
    // ...
}
```

---

## 4. Widgets

### Basic Widgets

#### Box

```rust
box_view()
    .class("container")
    .child(/* ... */)
```

#### Text

```rust
text("Hello World")
    .class("title")

// With formatting
text(format!("Count: {}", count.get()))
```

#### List

```rust
list(items.get())
    .class("menu")
    .on_select(|idx| selected.set(idx))
```

#### Table

```rust
table()
    .headers(["Name", "Email", "Role"])
    .rows(users.iter().map(|u| [&u.name, &u.email, &u.role]))
    .on_select(|row| /* ... */)
```

### Input Widgets

#### Input

```rust
input()
    .placeholder("Enter name...")
    .value(name.get())
    .on_change(|v| name.set(v))
```

#### Select

```rust
select()
    .options(["Option 1", "Option 2", "Option 3"])
    .selected(selected.get())
    .on_change(|idx| selected.set(idx))
```

#### Checkbox

```rust
checkbox()
    .label("Enable feature")
    .checked(enabled.get())
    .on_change(|v| enabled.set(v))
```

### Navigation Widgets

#### Tabs

```rust
tabs()
    .tabs(["Tab 1", "Tab 2", "Tab 3"])
    .active(active_tab.get())
    .on_change(|idx| active_tab.set(idx))
    .content(match active_tab.get() {
        0 => tab1_content(),
        1 => tab2_content(),
        _ => tab3_content(),
    })
```

#### Menu Bar

```rust
menu_bar()
    .menu("File", [
        menu_item("New", 'n', || /* ... */),
        menu_item("Open", 'o', || /* ... */),
        menu_separator(),
        menu_item("Quit", 'q', || /* ... */),
    ])
    .menu("Edit", [
        menu_item("Cut", 'x', || /* ... */),
        menu_item("Copy", 'c', || /* ... */),
        menu_item("Paste", 'v', || /* ... */),
    ])
```

### Feedback Widgets

#### Modal

```rust
if show_modal.get() {
    modal()
        .title("Confirm")
        .content(text("Are you sure?"))
        .actions([
            button("Cancel").on_click(|| show_modal.set(false)),
            button("OK").on_click(|| {
                do_action();
                show_modal.set(false);
            }),
        ])
}
```

#### Toast

```rust
// Show toast notification
toast("Item saved successfully")
    .duration(3000)
    .position(ToastPosition::BottomRight)
    .show();
```

#### Progress

```rust
progress()
    .value(progress.get())
    .max(100)
    .label(format!("{}%", progress.get()))
```

### Content Widgets

#### Markdown

```rust
markdown(r#"
# Hello World

This is **bold** and *italic*.

```rust
fn main() {
    println!("Hello!");
}
```

- Item 1
- Item 2
"#)
```

#### Markdown Presentation

Slidev-style terminal presentations with header sizing support:

```rust
let pres = MarkdownPresentation::new(r#"
# Title Slide

Welcome!

---

## Slide 2

- Point 1
- Point 2
"#)
    .accent(Color::CYAN)
    .text_sizing(true);  // Use Kitty Text Sizing Protocol

// Navigation
pres.next_slide();
pres.toggle_mode();  // Switch between preview/slides
```

Features:
- **Slide parsing**: Uses `---` delimiter (Slidev/Marp compatible)
- **Header sizing**: H1-H6 render at different sizes in Kitty terminal
- **Two modes**: Preview (scrollable) and Slides (one at a time)
- **Speaker notes**: `<!-- notes: ... -->` extracted but hidden

#### Image (Kitty Protocol)

```rust
image("path/to/image.png")
    .width(40)
    .height(20)
```

### Special Widgets

#### Command Palette

```rust
// Built-in Ctrl+P command palette
app.command_palette()
    .command("Open File", 'o', || /* ... */)
    .command("Save", 's', || /* ... */)
    .command("Settings", ',', || /* ... */)
```

---

## 5. Navigation

### Screen Routing

```rust
enum Screen {
    Home,
    Detail(String),
    Settings,
}

let screen = signal(Screen::Home);

// Navigate
screen.set(Screen::Detail("item-1".into()));

// Render based on screen
match screen.get() {
    Screen::Home => home_view(),
    Screen::Detail(id) => detail_view(id),
    Screen::Settings => settings_view(),
}
```

### Focus Management

```rust
// Auto focus order (Tab / Shift+Tab)
vbox().children([
    input().focus_order(1),
    input().focus_order(2),
    button("Submit").focus_order(3),
])

// Manual focus
focus_manager.focus("input-1");
```

### Layers (z-index)

```rust
// Base layer
layer(0, main_content())

// Popup layer
layer(1, popup_menu())

// Modal layer (highest)
layer(2, modal_dialog())
```

---

## 6. Unicode & Font Support

### Character Width Detection

```rust
// Auto-detect terminal character widths
let table = CharWidthTable::detect();

// Manual configuration
let table = CharWidthTable::new()
    .cjk(2)
    .emoji(2)
    .nerd_font(1);  // Depends on your font
```

### Configuration

```toml
# revue.toml
[font]
detect = true      # Auto-detect widths

[font.width]
cjk = 2
emoji = 2
nerd_font = 1
```

### Supported Characters

| Type | Example | Default Width |
|------|---------|---------------|
| ASCII | `a`, `Z`, `!` | 1 |
| CJK | `한`, `日`, `中` | 2 |
| Emoji | `😀`, `🔥`, `👍` | 2 |
| Nerd Font | ``, ``, `` | Configurable |
| Combined | `👨‍👩‍👧‍👦` | 2 |

---

## 7. Developer Experience

### Hot Reload

```bash
# CSS changes apply instantly without restart
revue dev --watch
```

```rust
App::new()
    .style("styles.css")
    .hot_reload(true)  // Enable in dev
```

### Devtools

```rust
// Enable devtools (Ctrl+Shift+D)
App::new()
    .devtools(true)
```

**Features:**
- Widget tree inspector
- Style debugger
- Event logger
- Performance monitor

### Testing

```rust
#[tokio::test]
async fn test_counter() {
    let app = App::new().mount(counter_view);

    // Simulate user interaction
    let pilot = app.pilot();

    pilot.press('j').await;  // Increment
    pilot.press('j').await;

    // Assert state
    assert_eq!(pilot.query(".count").text(), "Count: 2");
}

#[test]
fn test_snapshot() {
    let widget = counter_view();
    insta::assert_snapshot!(widget.render_to_string());
}
```

### Error Handling

```rust
// Error boundary catches widget panics
error_boundary()
    .fallback(|err| text(format!("Error: {}", err)))
    .child(risky_widget())
```

---

## 8. Theming

### Theme Files

```toml
# themes/dark.toml
[colors]
primary = "#61afef"
secondary = "#98c379"
background = "#282c34"
surface = "#3e4451"
text = "#abb2bf"
text-muted = "#5c6370"
error = "#e06c75"
warning = "#e5c07b"
success = "#98c379"

[font]
nerd_font = true
```

### Applying Themes

```rust
App::new()
    .theme("themes/dark.toml")
```

### Runtime Theme Switching

```rust
let theme = signal("dark");

// Switch theme
theme.set("light");

// In CSS
.container {
    background: var(--color-background);
    color: var(--color-text);
}
```

---

## 9. Clipboard

```rust
// Copy to clipboard
clipboard::copy("Hello World")?;

// Paste from clipboard
let text = clipboard::paste()?;

// In widgets
input()
    .on_key(Ctrl('c'), |text| clipboard::copy(text))
    .on_key(Ctrl('v'), || {
        if let Ok(text) = clipboard::paste() {
            // Insert text
        }
    })
```

---

## 10. Keyboard Shortcuts

### Global Shortcuts

```rust
App::new()
    .on_key('q', || app.quit())
    .on_key(Ctrl('p'), || show_command_palette())
    .on_key(Ctrl('s'), || save())
```

### Widget Shortcuts

```rust
list(items)
    .on_key('j', || move_down())
    .on_key('k', || move_up())
    .on_key(Enter, || select())
    .on_key('/', || start_search())
```

### Key Modifiers

```rust
Key::Char('a')           // 'a'
Key::Ctrl('c')           // Ctrl+C
Key::Alt('x')            // Alt+X
Key::Shift(Enter)        // Shift+Enter
Key::Ctrl(Shift('p'))    // Ctrl+Shift+P
```

---

## Feature Comparison

| Feature | Revue | Textual | Ratatui | Cursive |
|---------|-------|---------|---------|---------|
| CSS Styling | ✅ | ✅ | ❌ | ❌ |
| CSS Variables | ✅ | ✅ | ❌ | ❌ |
| Flexbox | ✅ | ✅ | ❌ | △ |
| Signal/Reactivity | ✅ | ✅ | ❌ | ❌ |
| Hot Reload | ✅ | ✅ | ❌ | ❌ |
| Devtools | ✅ | ✅ | ❌ | ❌ |
| Command Palette | ✅ | ✅ | ❌ | ❌ |
| Markdown | ✅ | ✅ | △ | ❌ |
| Image | ✅ Kitty | △ | △ | ❌ |
| Single Binary | ✅ | ❌ | ✅ | ✅ |
| Performance | ⚡⚡⚡ | ⚡ | ⚡⚡⚡ | ⚡⚡⚡ |
