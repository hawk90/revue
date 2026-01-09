# Features

## Overview

| Category | Features |
|----------|----------|
| **Styling** | CSS files, variables, selectors, transitions |
| **Layout** | Flexbox, padding, margin, border |
| **Reactivity** | Signal, Computed, Effect |
| **Widgets** | 85+ built-in widgets |
| **Content** | Markdown, presentations, syntax highlighting, images |
| **Navigation** | Routing, focus, layers, command palette |
| **DX** | Hot reload, devtools, testing |

---

## 1. CSS Styling

### External CSS Files

```rust
let mut app = App::builder()
    .style("styles.css")        // Main stylesheet
    .style("theme-dark.css")    // Additional styles
    .build();
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

#### DateTimePicker

Combined date and time selection with calendar-style interface:

```rust
use revue::widget::{datetime_picker, date_picker, time_picker, Date, Time};

// Full datetime picker
datetime_picker()
    .selected_date(Date::new(2025, 6, 15))
    .selected_time(Time::new(14, 30, 0))
    .show_seconds(true)

// Date only
date_picker()
    .selected_date(Date::new(2025, 1, 1))
    .min_date(Date::new(2025, 1, 1))
    .max_date(Date::new(2025, 12, 31))

// Time only
time_picker()
    .selected_time(Time::new(9, 0, 0))
    .show_seconds(false)
```

Features:
- **Calendar date picker**: Visual month view with day selection
- **Time picker**: Hour/minute/second with scrollable fields
- **Keyboard navigation**: Arrow keys, vim-style hjkl, Tab to switch modes
- **Date constraints**: Min/max date validation
- **Flexible formats**: Date only, time only, or combined datetime

| Key | Action |
|-----|--------|
| ←→ / hl | Move cursor left/right |
| ↑↓ / jk | Move cursor up/down or change time value |
| `[` / `]` | Previous/next month |
| `{` / `}` | Previous/next year |
| Tab | Switch between date and time mode |
| Enter | Select current date |

#### TextArea

Multi-line text editor with syntax highlighting, undo/redo, find/replace, and multiple cursors:

```rust
use revue::widget::textarea;

// Basic usage
textarea()
    .content("Initial text")
    .placeholder("Enter text...")
    .on_change(|text| content.set(text))

// With syntax highlighting
textarea()
    .language("rust")
    .theme("one-dark")
    .line_numbers(true)

// Read-only mode
textarea()
    .content(code)
    .read_only(true)
```

**Find/Replace:**

```rust
let mut editor = textarea();

// Open find panel (Ctrl+F)
editor.open_find();
editor.set_find_query("search term");

// Search options
editor.toggle_case_sensitive();
editor.toggle_whole_word();
editor.toggle_regex();

// Navigate matches
editor.find_next();      // F3
editor.find_previous();  // Shift+F3

// Replace (Ctrl+H)
editor.open_replace();
editor.set_replace_text("replacement");
editor.replace_current();
editor.replace_all();
```

**Multiple Cursors:**

```rust
// Add cursor at position
editor.add_cursor_at(5, 10);  // line 5, column 10

// Add cursor above/below current
editor.add_cursor_above();  // Ctrl+Alt+Up
editor.add_cursor_below();  // Ctrl+Alt+Down

// Select next occurrence (Ctrl+D)
editor.select_next_occurrence();

// Clear secondary cursors
editor.clear_secondary_cursors();  // Escape
```

| Key | Action |
|-----|--------|
| **Editing** |
| Ctrl+Z | Undo |
| Ctrl+Y / Ctrl+Shift+Z | Redo |
| Ctrl+D | Duplicate line |
| Ctrl+Shift+K | Delete line |
| **Find/Replace** |
| Ctrl+F | Open find panel |
| Ctrl+H | Open replace panel |
| F3 | Find next |
| Shift+F3 | Find previous |
| Escape | Close find panel |
| **Multiple Cursors** |
| Ctrl+D | Select next occurrence |
| Ctrl+Alt+↑ | Add cursor above |
| Ctrl+Alt+↓ | Add cursor below |
| Escape | Clear secondary cursors |
| **Selection** |
| Ctrl+A | Select all |
| Shift+Arrow | Extend selection |
| Ctrl+Shift+Arrow | Extend selection by word |

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

### Layout Widgets

#### Card

```rust
// Basic card with title
card()
    .title("User Profile")
    .subtitle("Account settings")
    .body(user_info_view())

// Card variants
card().outlined()   // Border only
card().filled()     // Background fill
card().elevated()   // Elevated with shadow effect
card().flat()       // No border, minimal

// Collapsible card
card()
    .title("Details")
    .collapsible(true)
    .expanded(false)
    .body(details_view())

// Card with header, body, and footer
card()
    .title("Order Summary")
    .body(order_items())
    .footer(total_view())
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

#### Alert

```rust
// Basic alerts by level
info_alert("Operation completed")
success_alert("File saved successfully")
warning_alert("Connection unstable")
error_alert("Failed to load data")

// Alert variants
alert("Custom message")
    .level(AlertLevel::Warning)
    .variant(AlertVariant::Filled)    // Filled background
    .variant(AlertVariant::Outlined)  // Border only
    .variant(AlertVariant::Minimal)   // Icon and text only
    .dismissible(true)                // Can be dismissed
    .icon(false)                      // Hide icon
    .custom_icon('⚠')                 // Custom icon character
    .title("Warning")                 // Add title
```

#### Callout

```rust
// Callout types for documentation-style highlights
note("General information for the reader")
tip("Helpful suggestion or best practice")
important("Key information to remember")
warning_callout("Potential issues to watch for")
danger("Critical warning - proceed with caution")

// Callout variants
Callout::note("Content here")
    .variant(CalloutVariant::Filled)      // Filled background
    .variant(CalloutVariant::LeftBorder)  // Left accent border
    .variant(CalloutVariant::Minimal)     // Icon and text only
    .collapsible(true)                    // Can collapse/expand
    .title("Custom Title")                // Override default title
```

#### StatusIndicator

```rust
// Status states
online()                    // Green dot
offline()                   // Gray dot
busy_indicator()            // Red dot
away_indicator()            // Yellow dot

// Display styles
status_indicator(Status::Online)
    .indicator_style(StatusStyle::Dot)          // Just the dot
    .indicator_style(StatusStyle::DotWithLabel) // Dot + "Online"
    .indicator_style(StatusStyle::Badge)        // Colored badge
    .size(StatusSize::Small)                    // sm/md/lg
    .label("Available")                         // Custom label
    .pulsing(true)                              // Animated pulse
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

### Chart Widgets

Statistical and data visualization widgets with standardized API.

#### Common Components

All chart widgets share common configuration types:

```rust
use revue::widget::{Axis, Legend, LegendPosition, ColorScheme, ChartGrid};

// Axis configuration
Axis::new()
    .title("X Values")
    .min(0.0)
    .max(100.0)
    .ticks(10)
    .grid(true)

// Legend positioning
Legend::new()
    .position(LegendPosition::TopRight)
    .orientation(LegendOrientation::Horizontal)

// Color palette
ColorScheme::default_palette()  // 10 distinct colors
ColorScheme::monochrome(Color::BLUE)
```

#### PieChart

Pie and donut charts for showing proportions:

```rust
use revue::widget::{pie_chart, donut_chart, PieLabelStyle};

// Basic pie chart
pie_chart()
    .slice("Category A", 30.0)
    .slice("Category B", 50.0)
    .slice("Category C", 20.0)
    .legend(Legend::bottom())
    .labels(PieLabelStyle::Percent)

// Donut chart (pie with hole)
donut_chart()
    .slice("Used", 75.0)
    .slice("Free", 25.0)
    .donut_ratio(0.5)  // 50% hole

// Exploded slice
pie_chart()
    .slice("Highlight", 40.0)
    .slice("Other", 60.0)
    .explode(0)  // Explode first slice
```

Features:
- **Pie/Donut styles**: Solid pie or ring donut
- **Labels**: None, value, percent, or label text
- **Legend**: Configurable position and orientation
- **Explode**: Highlight a slice by pulling it out
- **Custom colors**: Per-slice or auto-palette

#### ScatterChart

Scatter and bubble charts for X-Y data:

```rust
use revue::widget::{scatter_chart, bubble_chart, ScatterSeries, Marker};

// Basic scatter plot
scatter_chart()
    .series(ScatterSeries::new("Data A")
        .points(&[(1.0, 2.0), (3.0, 4.0), (5.0, 3.0)]))
    .x_axis(Axis::new().title("X"))
    .y_axis(Axis::new().title("Y"))

// Multiple series
scatter_chart()
    .series(ScatterSeries::new("Group 1").points(&data1))
    .series(ScatterSeries::new("Group 2").points(&data2))
    .legend(Legend::top_right())

// Bubble chart (size by value)
bubble_chart()
    .series(ScatterSeries::new("Bubbles")
        .points(&[(1.0, 2.0), (3.0, 4.0)])
        .sizes(&[10.0, 20.0]))  // Bubble sizes
```

Features:
- **Multiple series**: Compare different datasets
- **Bubble mode**: Size points by third variable
- **Markers**: Circle, square, triangle, diamond, cross
- **Grid**: X/Y gridlines with configurable style
- **Auto-scaling**: Automatic axis bounds calculation

#### Histogram

Distribution histograms for statistical data:

```rust
use revue::widget::{histogram, BinConfig};

// Basic histogram
histogram(&data)
    .bins(BinConfig::Count(20))
    .x_axis(Axis::new().title("Value"))
    .y_axis(Axis::new().title("Frequency"))

// Automatic binning (Sturges' rule)
histogram(&data)
    .bins(BinConfig::Auto)

// Density histogram (normalized)
histogram(&data)
    .density(true)
    .y_axis(Axis::new().title("Density"))

// With statistics overlay
histogram(&data)
    .show_stats(true)  // Mean/median lines
    .cumulative(true)  // Cumulative distribution

// Custom bin edges
histogram(&data)
    .bins(BinConfig::Edges(vec![0.0, 10.0, 20.0, 50.0, 100.0]))
```

Features:
- **Binning**: Auto, count, width, or custom edges
- **Density**: Normalize to probability density
- **Cumulative**: Show cumulative distribution
- **Statistics**: Mean/median lines overlay
- **Orientation**: Vertical or horizontal bars

#### BoxPlot

Box-and-whisker plots for distribution comparison:

```rust
use revue::widget::{boxplot, BoxGroup, WhiskerStyle};

// Basic box plot
boxplot()
    .group("Group A", &data_a)
    .group("Group B", &data_b)
    .group("Group C", &data_c)
    .value_axis(Axis::new().title("Values"))

// Show outliers
boxplot()
    .group("Data", &data)
    .show_outliers(true)
    .whisker_style(WhiskerStyle::IQR)  // 1.5 * IQR

// Notched box plot
boxplot()
    .group("Sample", &data)
    .notched(true)  // Show confidence interval

// Horizontal orientation
boxplot()
    .group("Distribution", &data)
    .horizontal()
```

Features:
- **Multiple groups**: Compare distributions side-by-side
- **Outliers**: Show/hide outlier points
- **Whisker styles**: IQR (1.5×), min-max, or percentile
- **Notched**: Show median confidence interval
- **Orientation**: Vertical or horizontal
- **Statistics**: Min, Q1, median, Q3, max, outliers

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
let mut app = App::builder()
    .style("styles.css")
    .hot_reload(true)  // Enable in dev
    .build();
```

### Devtools

```rust
// Enable devtools (Ctrl+Shift+D)
let mut app = App::builder()
    .devtools(true)
    .build();
```

**Features:**
- Widget tree inspector
- Style debugger
- Event logger
- Performance monitor

### Testing

```rust
use revue::testing::{TestApp, TestPilot};

#[test]
fn test_counter() {
    let mut app = TestApp::new(Counter::new());

    // Simulate key presses
    app.press_key(Key::Up);
    app.press_key(Key::Up);

    // Render and check output
    let output = app.render_to_string();
    assert!(output.contains("Count: 2"));
}

#[test]
fn test_snapshot() {
    let mut app = TestApp::new(Counter::new());
    insta::assert_snapshot!(app.render_to_string());
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
let mut app = App::builder()
    .style("themes/dark.css")
    .build();
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
app.run(view, |event, view, app| {
    if let Event::Key(key) = event {
        match (key.key, key.ctrl) {
            (Key::Char('q'), false) => { app.quit(); true }
            (Key::Char('p'), true) => { show_command_palette(); true }
            (Key::Char('s'), true) => { save(); true }
            _ => false,
        }
    } else {
        false
    }
})
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
