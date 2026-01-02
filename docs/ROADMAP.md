# Roadmap

## Version Overview

| Version | Theme | Status |
|---------|-------|--------|
| v0.1.0 | Foundation | ✅ Released |
| v0.2.0 | Polish | ✅ Released |
| v0.3.0 | Ecosystem | ✅ Released |
| v0.4.0 | Advanced | 📋 Planned |

---

## v0.1.0 - Foundation ✅

- [x] Core rendering engine
- [x] CSS parser (variables, selectors, transitions)
- [x] Flexbox layout (taffy)
- [x] Signal/Computed/Effect reactivity
- [x] 70+ widgets
- [x] Hot reload & devtools

---

## v0.2.0 - Polish ✅

Focus: Stability, Testing, Performance

### Phase 1: Test Coverage ✅

| Task | Status | Description |
|------|--------|-------------|
| Coverage tooling | ✅ | Setup cargo-llvm-cov |
| Core module tests | ✅ | `reactive/`, `dom/`, `style/` |
| Widget snapshots | ✅ | Expanded from 29 to 65+ |
| Integration tests | ✅ | End-to-end scenarios |

### Phase 2: API Stabilization ✅

| Task | Status | Description |
|------|--------|-------------|
| API audit | ✅ | Review all public types |
| Rustdoc | ✅ | Document all public items |
| Error messages | ✅ | Improve clarity |

### Phase 3: Performance ✅

| Task | Status | Description |
|------|--------|-------------|
| Benchmarks | ✅ | Criterion benchmarks (DOM, CSS, Layout, Render) |
| DOM optimization | ✅ | Incremental build with node reuse (2-54% faster) |
| Transition optimization | ✅ | Node-aware partial rendering |

---

## v0.3.0 - Ecosystem ✅

Focus: Extensibility, Tooling

### Phase 5: Plugin System ✅

| Task | Status | Description |
|------|--------|-------------|
| Plugin trait | ✅ | Lifecycle hooks (init, mount, tick, unmount) |
| PluginContext | ✅ | Plugin data storage, cross-plugin communication |
| PluginRegistry | ✅ | Plugin ordering by priority |
| Example plugins | ✅ | LoggerPlugin, PerformancePlugin |

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn on_init(&mut self, ctx: &mut PluginContext) -> Result<()>;
    fn on_mount(&mut self, ctx: &mut PluginContext) -> Result<()>;
    fn on_tick(&mut self, ctx: &mut PluginContext, delta: Duration) -> Result<()>;
    fn on_unmount(&mut self, ctx: &mut PluginContext) -> Result<()>;
    fn styles(&self) -> Option<&str>;
}
```

### Phase 6: Runtime Theme Switching ✅

| Task | Status | Description |
|------|--------|-------------|
| Signal-based theme | ✅ | `use_theme()` returns `Signal<Theme>` |
| Theme functions | ✅ | `set_theme()`, `toggle_theme()`, `cycle_theme()` |
| ThemePicker widget | ✅ | Interactive theme selection |
| CSS variable generation | ✅ | `theme_to_css_variables()` |

```rust
let theme = use_theme();
set_theme(Theme::Nord);  // Instant UI update

theme_picker()
    .themes(["dracula", "nord", "gruvbox"])
    .on_change(|id| set_theme_by_id(id))
```

### Phase 7: Component Library ✅

| Task | Status | Description |
|------|--------|-------------|
| SearchState | ✅ | Fuzzy/contains/prefix/exact filter modes |
| FormState | ✅ | Field validation, focus navigation |
| NavigationState | ✅ | Browser-like history (back/forward) |
| Route | ✅ | Path matching with parameters |

### Phase 8: CLI Enhancement ✅

| Task | Status | Description |
|------|--------|-------------|
| `revue add` | ✅ | Add component templates (8 types) |
| `revue benchmark` | ✅ | Run Criterion benchmarks |
| Component templates | ✅ | search, form, navigation, modal, toast, command-palette, table, tabs |

---

## v0.4.0 - Advanced 📋

Focus: Async, Accessibility, Animation

### Async Support

| Task | Description |
|------|-------------|
| Async runtime | Tokio integration for async operations |
| Async effects | `use_async()` hook for data fetching |
| Streaming | AI streaming, real-time updates |

### Accessibility

| Task | Description |
|------|-------------|
| Screen reader | Terminal accessibility announcements |
| Focus management | Improved keyboard navigation |
| High contrast | Accessibility-focused themes |

### Animation Engine

| Task | Description |
|------|-------------|
| Keyframes | CSS `@keyframes` animation support |
| Easing functions | cubic-bezier, spring, bounce |
| Choreography | Staggered animations, sequences |

---

## v0.5.0 - Extensibility 📋

Focus: Developer Experience, Ecosystem

### Widget DSL

| Task | Description |
|------|-------------|
| Macro DSL | Declarative widget syntax |
| Template compiler | Compile-time validation |

### Theme Editor

| Task | Description |
|------|-------------|
| Visual editor | Interactive theme customization |
| Export/Import | Share themes as files |

### Plugin Ecosystem

| Task | Description |
|------|-------------|
| Plugin registry | Central plugin discovery |
| Dependency resolution | Plugin dependencies |

---

## Contributing

- [Good First Issues](https://github.com/hawk90/revue/labels/good%20first%20issue)
- [Help Wanted](https://github.com/hawk90/revue/labels/help%20wanted)
- [Contributing Guide](../CONTRIBUTING.md)
