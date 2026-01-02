# Expert Code Review - Revue TUI Framework

**Date:** 2025-12-18
**Reviewers:** 10 Expert Personas
**Tests:** 1309 passed

---

## Review Summary

| Expert | Focus Area | Rust Applicability |
|--------|------------|-------------------|
| Plan Agent | Architecture | ✅ |
| Explore Agent | Codebase Analysis | ✅ |
| Linus Torvalds | Performance, Simplicity | ✅ |
| Rob Pike | Go Philosophy | ⚠️ Reference only |
| Rich Harris | Reactivity (Svelte) | ⚠️ Concepts only |
| Andrew Kelley | Memory/Allocation (Zig) | ✅✅ |
| Kent Beck | TDD/XP | ✅ |
| Martin Fowler | Refactoring | ✅ |
| Uncle Bob | Clean Code/SOLID | ✅ |
| Jon Gjengset | Rust Idioms | ✅✅✅ |

---

## Implementation Status

### ✅ Completed

| Item | Expert | File | Status |
|------|--------|------|--------|
| DataGrid RefCell removal | Jon Gjengset | `datagrid.rs` | ✅ Done |
| Signal<T> zero-copy API | Jon Gjengset | `signal.rs` | ✅ Done |
| Input Vec<char> removal | Jon Gjengset | `input.rs` | ✅ Done |
| Input Undo/Redo bugs | Review Agents | `input.rs` | ✅ Done |
| DataGrid bounds checking | Review Agents | `datagrid.rs` | ✅ Done |
| Event handler removal API | Review Agents | `handler.rs` | ✅ Done |
| CSS Grid repeat/minmax | Review Agents | `parser.rs` | ✅ Done |
| Animation-Rendering connection | Review Agents | `app/mod.rs` | ✅ Done |
| Silent failure logging | Linus Torvalds | `builder.rs` | ✅ Done |
| Debug assertions | Andrew Kelley | `buffer.rs`, `engine.rs` | ✅ Done |
| Dead code deprecation | Linus Torvalds | `runtime.rs` | ✅ Done |
| Split apply_declaration() | Martin Fowler | `parser.rs` | ✅ Done |
| Split DataGrid render() | Martin Fowler | `datagrid.rs` | ✅ Done |
| Extract quit handler | Uncle Bob | `app/mod.rs` | ✅ Done |
| App/Builder tests | Kent Beck | `app/mod.rs`, `builder.rs` | ✅ Done |

### 🔄 In Progress

#### Large Tasks (🔴)
- [x] Split App God Class → StyleManager extracted (6 → 4 fields)
- [x] Split DataGrid God Object → GridColors + GridOptions (42 → 14 fields)
- [x] CSS Parser rewrite → Zero-copy &str slicing (Vec<char> removed)
- [x] Reactive dependency tracking → DependencyTracker + automatic Signal/Effect integration

---

## Detailed Findings by Expert

### 1. Linus Torvalds - Performance & Simplicity

**Critical Issues:**
- `runtime.rs` is dead code (TODO only) → DELETE
- Silent CSS loading failures in `builder.rs:58-62` → ADD LOGGING
- CSS Parser uses `Vec<char>` allocation → REWRITE NEEDED

**Code Locations:**
```
src/app/runtime.rs:17-29     - Dead TODO code
src/app/builder.rs:58-62     - Silent failure
src/style/parser.rs:70       - Vec<char> allocation
```

### 2. Andrew Kelley (Zig) - Memory & Allocation

**Hidden Allocations Found:**
```rust
// parser.rs:70 - Entire CSS to Vec<char>
let chars: Vec<char> = css.chars().collect();

// parser.rs:151 - Repeated in starts_with()
let s_chars: Vec<char> = s.chars().collect();

// datagrid.rs:398 - to_lowercase() in sort comparisons
va.to_lowercase()  // Allocates on every comparison!

// sort.rs:35 - to_lowercase() in natural_cmp
a.to_lowercase().cmp(&b.to_lowercase())  // O(n log n) allocations!
```

**Missing Debug Assertions:**
```rust
// buffer.rs:20 - No overflow check
let size = (width as usize) * (height as usize);

// buffer.rs:165 - Division by zero risk
let y = (idx / self.width as usize) as u16;

// engine.rs:80-87 - Unchecked f32 → u16 cast
x: layout.location.x as u16  // Could overflow!
```

### 3. Martin Fowler - Refactoring

**Long Methods:**
| File | Method | Lines | Action |
|------|--------|-------|--------|
| `parser.rs` | `apply_declaration()` | 175 | Extract by category |
| `datagrid.rs` | `render()` | 201 | Extract sub-methods |
| `app/mod.rs` | `run_loop_with_tick()` | 84 | Extract handlers |

**God Objects:**
| Class | Fields | Responsibilities | Action |
|-------|--------|-----------------|--------|
| `DataGrid` | 42 | Data, State, Style, Cache, Edit | Split into 4 structs |
| `App` | 6 | Style, Lifecycle, Transitions | Split into 3 structs |
| `Style` | 30+ | Layout, Grid, Spacing, Visual | Split into 4 structs |

**Code Duplication:**
- Undo/redo logic in `input.rs:317-386` (60+ lines duplicated)
- Event handler quit logic repeated 5x in `app/mod.rs`
- Color disabled logic repeated across widgets

### 4. Uncle Bob - Clean Code & SOLID

**SRP Violations:**
- `App` manages: styles, lifecycle, transitions, rendering, events
- `DataGrid` manages: data, sorting, filtering, editing, rendering, styling

**DIP Violations:**
- `App` directly creates `Terminal::new(stdout())` - not injectable
- `EventReader::new()` hardcoded 16ms timeout

**Missing Tests:**
- `app/mod.rs` - 410 lines, 0 tests
- `app/builder.rs` - 0 tests
- `widget/traits.rs` - 0 tests

### 5. Kent Beck - TDD/XP

**Test Issues:**
- Tests verify implementation, not behavior
- `test_button_new()` checks internal fields, not user actions
- Pilot framework underutilized (only 2 basic tests)

**Recommendations:**
- Write behavior tests: "clicking button triggers action"
- Use Pilot for integration workflows
- Extract shared test helpers

### 6. Jon Gjengset - Rust Idioms

**Completed Fixes:**
- ✅ DataGrid `RefCell<Option<Vec>>` → `Vec` with eager recompute
- ✅ Signal `get()` clone → `borrow()`, `with()` zero-copy
- ✅ Input `Vec<char>` → iterator-based word navigation

**Remaining:**
- CSS Parser `Vec<char>` → `&str` slicing (large rewrite)
- Consider `parking_lot::RwLock` for Signal if multi-threading needed

---

## Priority Implementation Order

### Phase 1: Small Tasks (Today)
1. ✅ Add logging to silent failures
2. ✅ Add debug assertions
3. ✅ Delete dead runtime.rs code

### Phase 2: Medium Tasks
4. Split `apply_declaration()` into category functions
5. Split `DataGrid::render()` into sub-methods
6. Extract common event handler logic
7. Add App/Builder tests

### Phase 3: Large Refactoring
8. Split DataGrid into focused structs
9. Split App into focused structs
10. Rewrite CSS Parser without Vec<char>
11. Implement reactive dependency tracking

---

## Files to Modify

| Priority | File | Changes |
|----------|------|---------|
| 🟢 | `app/builder.rs` | Add error logging |
| 🟢 | `app/runtime.rs` | Delete or implement |
| 🟢 | `render/buffer.rs` | Add debug assertions |
| 🟢 | `layout/engine.rs` | Add bounds checks |
| 🟡 | `style/parser.rs` | Split apply_declaration |
| 🟡 | `widget/datagrid.rs` | Split render method |
| 🟡 | `app/mod.rs` | Extract event handlers |
| 🔴 | `widget/datagrid.rs` | God Object split |
| 🔴 | `app/mod.rs` | God Class split |
| 🔴 | `style/parser.rs` | Full rewrite |

---

## Metrics After Review

| Metric | Before | After | Target |
|--------|--------|-------|--------|
| Tests | 1309 | 1355 | 1400+ |
| Longest Method | 201 lines | ~50 lines | <50 lines |
| DataGrid Fields | 42 | 14 | <15 ✅ |
| App Fields | 6 | 4 | <6 ✅ |
| Style Fields | 27 | 4 | <6 ✅ |
| RefCell Usage | 2 | 1 | 0 |
| Vec<char> in Parser | 2 | 0 | 0 ✅ |
| Auto Dependency Tracking | ❌ | ✅ | ✅ |
| TextArea Syntax Highlighting | ❌ | ✅ | ✅ |
| Languages Supported | 0 | 12 | 10+ ✅ |

### Changes Made (Phase 1: Small/Medium Tasks)
- `apply_declaration()` split into 5 category functions (~35 lines each)
- `DataGrid::render()` split into 7 helper methods (~30 lines each)
- Extracted `is_quit_key()` and `with_quit_handler()` helpers
- Added 17 new tests for App and Builder modules
- Added tracing-based logging for CSS loading failures
- Added debug assertions for buffer overflow and layout bounds
- Deprecated unused Runtime struct with documentation

### Changes Made (Phase 2: Large Tasks)
- **DataGrid God Object Split**: Extracted `GridColors` (7 fields) and `GridOptions` (5 fields)
- **App God Class Split**: Extracted `StyleManager` (3 fields) for CSS/stylesheet management
- **CSS Parser Rewrite**: Replaced `Vec<char>` with zero-copy `&str` slicing
  - Removed `chars.collect()` allocation in main parse function
  - Removed `s.chars().collect()` in `starts_with()` helper
  - New byte-based parsing functions: `skip_whitespace_bytes`, `parse_*_str`
  - ~150 lines of legacy code removed
- **Reactive Dependency Tracking**: Vue.js/SolidJS-style automatic tracking
  - New `tracker.rs` module with thread-local `DependencyTracker`
  - `Signal::get/borrow/with` automatically registers dependencies
  - `Signal::set/update` notifies all auto-tracked dependents
  - `Effect` runs within tracking context, auto-subscribes to signals
  - Clean disposal when effect is dropped
  - 7 integration tests for automatic tracking
- **Style God Object Split**: Organized 27+ fields into 4 focused sub-structs
  - `LayoutStyle` (12 fields): display, position, flex, grid properties
  - `SpacingStyle` (6 fields): padding, margin, position offsets
  - `SizingStyle` (6 fields): width, height, min/max constraints
  - `VisualStyle` (7 fields): colors, border, opacity, visibility
  - Backward-compatible accessor methods preserved
  - Updated all files: cascade.rs, convert.rs, parser.rs, engine.rs, renderer.rs

### Gap Analysis Implementation (vs Textual/Ratatui)
- **Mouse Support**: Already implemented ✅
  - `MouseEvent`, `MouseButton`, `MouseEventKind` in `src/event/mod.rs`
  - Mouse event parsing in `src/event/reader.rs`
  - `EnableMouseCapture` in terminal initialization
- **DataGrid Cell Editing**: Already implemented ✅
  - `EditState`, `start_edit()`, `commit_edit()`, `cancel_edit()` in `datagrid.rs`
- **Event Bubbling/Capture**: Already implemented ✅
  - `EventContext` with `stop_propagation()`, `prevent_default()` in `handler.rs`
  - `EventPhase` (Capture, Target, Bubble) support
- **TextArea Syntax Highlighting**: NEW ✅
  - New `src/widget/syntax.rs` module with 12 language support
  - `Language` enum: Rust, Python, JavaScript, JSON, TOML, YAML, Markdown, Shell, SQL, HTML, CSS, Go
  - `SyntaxTheme` with dark(), light(), monokai() themes
  - `SyntaxHighlighter` with `highlight_line()` method
  - TextArea integration: `.syntax(Language::Rust)`, `.syntax_with_theme()`
  - 4 new tests for syntax highlighting

---

*Generated by Claude Code Expert Review System*
