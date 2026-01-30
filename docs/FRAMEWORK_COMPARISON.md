# Revue vs Reference TUI Frameworks

## Overview

| Framework | Language | Rendering | Widgets | Styling | Maturity |
|-----------|----------|-----------|---------|---------|----------|
| **Revue** | Rust | Retained | 100+ | CSS | New |
| **Textual** | Python | Retained | 35+ | TCSS | Mature |
| **ratatui** | Rust | Immediate | 15+ | Inline | Mature |
| **reratui** | Rust | Immediate (on ratatui) | Components (on ratatui) | Inline-style | New |
| **Cursive** | Rust | Retained | 40+ | TOML | Mature |
| **Bubbletea** | Go | Immediate | 15+ | Inline | Mature |

**Note:** reratui (framework) builds on ratatui (library), similar to React (framework) on DOM (library)

---

## 1. Widget Comparison

### 1.1 Form Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Text/Label | ✅ | ✅ | ✅ Paragraph | All have |
| Button | ✅ (329L) | ✅ | ❌ | Revue has |
| Input | ✅ (298L) | ✅ (800L) | ❌ | Textual richer |
| MaskedInput | ✅ | ✅ | ❌ | Revue has |
| TextArea | ✅ (1249L) | ✅ (3000L) | ❌ | Both have undo/redo |
| Checkbox | ✅ (360L) | ✅ | ❌ | Revue has |
| RadioButton | ✅ | ✅ | ❌ | Revue has |
| Switch | ✅ | ✅ | ❌ | Revue has |
| Select/Dropdown | ✅ (732L) | ✅ | ❌ | Both have |
| Slider | ✅ | ✅ | ❌ | Revue has |
| ColorPicker | ✅ | ✅ | ❌ | Revue has |
| FilePicker | ✅ | ❌ | ❌ | Revue unique |

### 1.2 Data Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Table (basic) | ✅ (414L) | ✅ | ✅ (1500L) | Ratatui richer |
| DataGrid | ✅ (820L) | ✅ (3000L) | ❌ | Sort/filter/multi |
| List | ✅ | ✅ | ✅ | All have |
| VirtualList | ✅ (583L) | ✅ | ❌ | Large datasets |
| Tree | ✅ (939L) | ✅ | ❌ | Both have |
| FileTree | ✅ (670L) | ❌ | ❌ | Revue unique |
| OptionList | ✅ | ✅ | ❌ | Both have |
| SelectionList | ✅ | ✅ | ❌ | Both have |

### 1.3 Visualization Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Chart (line) | ✅ (1160L) | ❌ | ✅ (600L) | Revue richer |
| BarChart | ✅ (505L) | ❌ | ✅ (500L) | Equal |
| PieChart | ✅ | ❌ | ❌ | Revue unique (pie/donut) |
| ScatterChart | ✅ | ❌ | ❌ | Revue unique (bubble) |
| Histogram | ✅ | ❌ | ❌ | Revue unique (binning) |
| BoxPlot | ✅ | ❌ | ❌ | Revue unique (quartiles) |
| Sparkline | ✅ (395L) | ✅ | ✅ (260L) | All have |
| CandleChart | ✅ | ❌ | ❌ | Revue unique |
| TimeSeries | ✅ | ❌ | ❌ | Revue unique |
| Waveline | ✅ | ❌ | ❌ | Revue unique |
| Streamline | ✅ | ❌ | ❌ | Revue unique |
| Heatmap | ✅ | ❌ | ❌ | Revue unique |
| Gauge | ✅ | ❌ | ✅ | Both have |
| Progress | ✅ | ✅ | ✅ | All have |
| Canvas | ✅ (1071L) | ❌ | ✅ (400L) | Revue richer |

### 1.4 Layout Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| HStack/VStack | ✅ | ✅ | ❌ | Both have |
| Grid | ✅ | ✅ | ❌ | Both have |
| Scroll | ✅ | ✅ | ✅ | All have |
| Splitter | ✅ | ❌ | ❌ | Revue unique |
| Layer | ✅ | ✅ | ❌ | Overlap support |
| Positioned | ✅ | ❌ | ❌ | Absolute positioning |
| Tabs | ✅ | ✅ | ✅ | All have |
| Accordion | ✅ | ✅ | ❌ | Both have |

### 1.5 Feedback Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Modal/Dialog | ✅ (572L) | ✅ Screen | ❌ | Both have |
| Toast | ✅ (419L) | ✅ | ❌ | Both have |
| Notification | ✅ | ✅ | ❌ | Both have |
| Tooltip | ✅ | ✅ | ❌ | Both have |
| Spinner | ✅ | ✅ | ❌ | Both have |
| Skeleton | ✅ | ✅ | ❌ | Both have |

### 1.6 Navigation Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Menu | ✅ | ✅ | ❌ | Both have |
| ContextMenu | ✅ | ❌ | ❌ | Revue unique |
| CommandPalette | ✅ (931L) | ✅ | ❌ | Like VS Code Ctrl+P |
| Breadcrumb | ✅ | ✅ | ❌ | Both have |
| Pagination | ✅ | ❌ | ❌ | Revue unique |
| Stepper | ✅ | ❌ | ❌ | Revue unique |

### 1.7 Rich Content Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Markdown | ✅ | ✅ | ❌ | Both have |
| RichText | ✅ | ✅ | ❌ | Both have |
| RichLog | ✅ | ✅ | ❌ | Both have |
| Image | ✅ | ❌ | ❌ | Revue unique (sixel) |
| Link | ✅ | ✅ | ❌ | OSC-8 hyperlinks |
| Digits | ✅ | ✅ | ❌ | Big text |

### 1.8 Specialized Widgets

| Widget | Revue | Textual | Ratatui | Notes |
|--------|-------|---------|---------|-------|
| Terminal | ✅ (1029L) | ❌ | ❌ | PTY emulator |
| Calendar | ✅ | ❌ | ✅ | Both have |
| Timeline | ✅ | ❌ | ❌ | Revue unique |
| Timer | ✅ | ❌ | ❌ | Revue unique |
| Rating | ✅ | ❌ | ❌ | ★★★★☆ |
| Avatar | ✅ | ❌ | ❌ | Revue unique |
| Badge | ✅ | ✅ | ❌ | Both have |
| Tag | ✅ | ✅ | ❌ | Both have |
| StatusBar | ✅ | ✅ | ❌ | Both have |

---

## 2. Styling System Comparison

| Feature | Revue | Textual | Ratatui | Cursive |
|---------|-------|---------|---------|---------|
| CSS Files | ✅ | ✅ TCSS | ❌ | ❌ |
| CSS Variables | ✅ | ✅ | ❌ | ❌ |
| CSS Selectors | ✅ (partial) | ✅ (full) | ❌ | ❌ |
| Theme System | ✅ | ✅ | ❌ | ✅ TOML |
| Inline Styles | ✅ | ✅ | ✅ | ✅ |
| Hot Reload | ✅ | ✅ | ❌ | ❌ |
| Transitions | ✅ (partial) | ✅ | ❌ | ❌ |

### Revue CSS Example
```css
.button {
    background: #89b4fa;
    color: #1e1e2e;
    padding: 1 2;
}

.button:hover {
    background: #b4befe;
}
```

---

## 3. Layout System Comparison

| Feature | Revue | Textual | Ratatui |
|---------|-------|---------|---------|
| Flexbox | ✅ (Taffy) | ✅ | ❌ |
| Grid | ✅ | ✅ | ❌ |
| Constraint | ❌ | ❌ | ✅ (Cassowary) |
| Linear | ✅ | ✅ | ❌ |
| Dock | ❌ | ✅ | ❌ |
| Percent | ✅ | ✅ | ✅ |
| Auto | ✅ | ✅ | ❌ |

---

## 4. State Management

| Feature | Revue | Textual | Ratatui | Bubbletea |
|---------|-------|---------|---------|-----------|
| Reactive Signals | ✅ | ✅ Descriptor | ❌ | ❌ |
| Computed Values | ✅ | ✅ | ❌ | ❌ |
| Effects/Watch | ✅ | ✅ | ❌ | ❌ |
| Elm Update | ❌ | ❌ | ❌ | ✅ |
| Message Bus | ❌ | ✅ | ❌ | ✅ Cmd |

---

## 5. Developer Experience

| Feature | Revue | Textual | Ratatui |
|---------|-------|---------|---------|
| DevTools/Inspector | ✅ | ✅ textual-dev | ❌ |
| Snapshot Testing | ✅ | ❌ | ❌ |
| Hot Reload | ✅ | ✅ | ❌ |
| Pilot Testing | ❌ | ✅ | ❌ |
| Web Deployment | ❌ | ✅ textual-serve | ❌ |

---

## 6. Code Size Comparison (lines)

| Widget | Revue | Textual* | Ratatui |
|--------|-------|---------|---------|
| TextArea | 1,249 | ~3,000 | N/A |
| DataGrid | 820 | ~3,000 | N/A |
| Table | 414 | ~500 | 1,500 |
| Chart | 1,160 | N/A | 600 |
| Canvas | 1,071 | N/A | 400 |
| Tree | 939 | ~1,000 | N/A |
| Input | 298 | ~800 | N/A |
| **Total (widgets)** | **42,418** | N/A | ~8,900 |

*Textual Python lines (different language comparison)

---

## 7. Feature Depth Analysis

### 7.1 TextArea Features

| Feature | Revue | Textual |
|---------|-------|---------|
| Basic editing | ✅ | ✅ |
| Undo/Redo | ✅ | ✅ |
| Selection | ✅ | ✅ |
| Word navigation | ✅ | ✅ |
| Line numbers | ✅ | ✅ |
| Soft wrap | ✅ | ✅ |
| Syntax highlight | ✅ | ✅ |
| Multiple cursors | ❌ | ❌ |
| Find/Replace | ❌ | ❌ |

### 7.2 DataGrid Features

| Feature | Revue | Textual |
|---------|-------|---------|
| Sort by column | ✅ | ✅ |
| Filter | ✅ | ✅ |
| Multi-select | ✅ | ✅ |
| Zebra striping | ✅ | ✅ |
| Row numbers | ✅ | ✅ |
| Cell navigation | ✅ | ✅ |
| Cell editing | ✅ | ✅ |
| Column resize | ❌ | ✅ |
| Column reorder | ❌ | ✅ |
| Column freeze | ❌ | ✅ |
| Virtual scroll | ❌ | ✅ |

---

## 8. Revue Unique Features

Features that Revue has but competitors don't:

1. **Terminal Widget** - Full PTY emulator (1029L)
2. **CandleChart** - Financial charts
3. **TimeSeries** - Real-time data
4. **Waveline** - Audio-style visualization
5. **Streamline** - Streaming data
6. **Heatmap** - Heat map visualization
7. **FilePicker** - File selection dialog
8. **FileTree** - Directory browser
9. **Timeline** - Event timeline
10. **ContextMenu** - Right-click menus
11. **Splitter** - Resizable panes
12. **Positioned** - Absolute positioning
13. **Image** - Sixel image support
14. **Avatar** - User avatars
15. **Rating** - Star ratings

---

## 9. Gap Analysis

### 9.1 Critical Gaps (vs Textual)

| Gap | Impact | Difficulty | Status |
|-----|--------|------------|--------|
| ~~Input: Ctrl+C/V copy-paste~~ | ~~High~~ | ~~Easy~~ | ✅ Done |
| ~~Input: Selection with Shift~~ | ~~High~~ | ~~Medium~~ | ✅ Done |
| DataGrid: Virtual scroll | High | Hard | ❌ TODO |
| DataGrid: Column resize | Medium | Medium | ❌ TODO |
| DataGrid: Column reorder | Medium | Medium | ❌ TODO |
| DataGrid: Column freeze | Medium | Medium | ❌ TODO |
| ~~Syntax highlighting~~ | ~~Medium~~ | ~~Hard~~ | ✅ Done |
| TextArea: Multiple cursors | Low | Hard | ❌ TODO |
| TextArea: Find/Replace | Medium | Medium | ❌ TODO |

### 9.2 Nice to Have (vs Textual)

| Feature | Impact | Difficulty | Status |
|---------|--------|------------|--------|
| ~~CSS Grid layout~~ | ~~Medium~~ | ~~Medium~~ | ✅ Done |
| Worker system | Medium | Hard | ✅ Done |
| Web deployment | Low | Very Hard | ❌ TODO |
| Screen stack | Low | Medium | ❌ TODO |

---

## 10. Conclusion

### Revue Strengths
- **70+ widgets** (more than any competitor)
- **CSS styling** (only Textual has this)
- **Visualization** (best chart/canvas support)
- **Developer tools** (inspector, hot reload, snapshot testing)
- **Unique widgets** (Terminal, FilePicker, Timeline, etc.)

### Revue Weaknesses
- **DataGrid** needs virtual scrolling, column resize/reorder/freeze
- **TextArea** needs multiple cursors, find/replace
- **No web deployment** (Textual has textual-serve)

### Overall Assessment

| vs Framework | Revue Score | Notes |
|--------------|-------------|-------|
| vs Textual | **90%** | Missing: DataGrid advanced features, TextArea find/replace |
| vs ratatui | **110%** | More widgets, CSS styling, reactive state system |
| vs reratui | **95%** | More widgets, CSS styling, but reratui has React-like ergonomics |
| vs Cursive | **110%** | Modern API, CSS styling |
| vs Bubbletea | **130%** | More widgets, better styling |

**Revue is already competitive with mature frameworks and has unique strengths in visualization and styling.**

---

## 11. ratatui vs reratui: Which One to Choose?

### Key Differences

| Aspect | ratatui | reratui |
|--------|---------|---------|
| **Type** | TUI library (low-level) | TUI framework (high-level) |
| **Architecture** | Widget traits | Component-based (React-like) |
| | `Widget`, `StatefulWidget` | `Component`, `useReducer`, `useEffect` |
| **State Management** | Manual (within StatefulWidget) | Hooks-based (useReducer, useEffect, useState) |
| **Rendering** | Immediate-mode | Immediate-mode (on ratatui) |
| **Styling** | Inline (Style API) | Inline-style API |
| **Layout** | Constraint-based (Cassowary) | Flex-like (built on ratatui) |
| **Reactivity** | Manual re-renders | Automatic with hooks |
| **Maturity** | Mature (2023+) | New (Oct 2025) |

### When to Use ratatui

✅ Choose **ratatui** if you:
- Want low-level control over every render
- Prefer explicit state management
- Are building a custom framework on top
- Need maximum performance
- Want minimal dependencies

**Example:**
```rust
impl Widget for MyApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Direct buffer manipulation
        buf.set_string(area.x, area.y, "Hello", Style::default());
    }
}
```

### When to Use reratui

✅ Choose **reratui** if you:
- Like React's component model
- Want hooks-based state management
- Prefer declarative UI patterns
- Are building complex interactive apps
- Want automatic re-renders on state change

**Example:**
```rust
#[component]
fn Counter(cx: Scope) -> Element {
    let mut count = use cx.use_state(|| 0);

    let on_increment = |_| {
        count.set(*count + 1);
    };

    vstack!([
        Button::new(format!("Count: {}", count)).on_click(on_increment),
        Button::new("Increment").on_click(on_increment),
    ])
}
```

### Feature Comparison

| Feature | ratatui | reratui | Revue |
|---------|---------|---------|-------|
| Widgets | 15 built-in | Component wrappers | 100+ |
| Layout | Constraint-based | Flex-like (on ratatui) | CSS (Flex + Grid) |
| Styling | Inline `Style` API | Inline-style | CSS files |
| State | Manual (StatefulWidget) | Hooks (useReducer, useState) | Reactive signals |
| Hot Reload | ❌ | ❌ | ✅ |
| CSS Support | ❌ | ❌ | ✅ |
| Async | ❌ | ❌ | ✅ (worker system) |

### Summary

- **ratatui** = React's DOM - low-level, full control
- **reratui** = React - high-level, developer-friendly
- **Revue** = React + CSS + Signals = Modern web-like TUI

**Sources:**
- [ratatui on crates.io](https://crates.io/crates/ratatui) - 2000+ crates depend on it
- [reratui on crates.io](https://crates.io/crates/reratui) - React-like for TUI
- [Ratatui official docs](https://ratatui.rs/) - "Cook up delicious terminal UIs"
- [reratui GitHub](https://github.com/sabry-awad97/reratui) - Component framework


