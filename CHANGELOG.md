# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.6](https://github.com/hawk90/revue/compare/v1.0.5...v1.0.6) (2026-01-03)


### Bug Fixes

* remove environment from publish job for trusted publishing ([2f24c9d](https://github.com/hawk90/revue/commit/2f24c9dc753e50be1b000e2cd90c8ce3241fddc0))

## [1.0.5](https://github.com/hawk90/revue/compare/v1.0.4...v1.0.5) (2026-01-03)


### Bug Fixes

* add codecov token to coverage job ([47b0334](https://github.com/hawk90/revue/commit/47b033498bbb8199db2c16a1a8eff7c06f27e8ca))
* configure trusted publishing for crates.io ([b016f41](https://github.com/hawk90/revue/commit/b016f411a7b4f2ea1de3f77039fb21b7d8ee3956))

## [1.0.4](https://github.com/hawk90/revue/compare/v1.0.3...v1.0.4) (2026-01-03)


### Bug Fixes

* remove binary build for library crate ([0980f08](https://github.com/hawk90/revue/commit/0980f0888cfd88a3bd0f682766180f5d25497cd0))

## [1.0.3](https://github.com/hawk90/revue/compare/v1.0.2...v1.0.3) (2026-01-03)


### Bug Fixes

* mark flaky accessibility test as ignored ([38d001c](https://github.com/hawk90/revue/commit/38d001c1562cc2a96fa7d230d180664ff2246ad0))

## [1.0.2](https://github.com/hawk90/revue/compare/v1.0.1...v1.0.2) (2026-01-03)


### Bug Fixes

* correct Quick Start anchor link in README ([8d98b4b](https://github.com/hawk90/revue/commit/8d98b4b0dca6339ba80d918f2defd2562a601c43))

## [1.0.1](https://github.com/hawk90/revue/compare/v1.0.0...v1.0.1) (2026-01-03)


### Bug Fixes

* add allow(dead_code) to test functions and adjust banner size ([31fa7a9](https://github.com/hawk90/revue/commit/31fa7a9e5c78e78d9c59d5fdc32297ffe5cdd725))
* add allow(dead_code) to test structs ([99e16bc](https://github.com/hawk90/revue/commit/99e16bc201f56fe97ba68eb55f5491721529cfd5))
* add fxhash and zune-jpeg version advisories ([b0c79bd](https://github.com/hawk90/revue/commit/b0c79bd42c92b4c7c8d3b63b46273363ff6f6f20))
* add missing licenses and relax FPS test threshold ([fd054b4](https://github.com/hawk90/revue/commit/fd054b40e4da7787257996939553183e7febc5a2))
* add poison recovery to Effect callback handling ([38dfa12](https://github.com/hawk90/revue/commit/38dfa124c4c5404cb8b1e7fc7762fe0a58990132))
* **ci:** remove missing labels from dependabot ([1889bdd](https://github.com/hawk90/revue/commit/1889bdd2adac56292be1cf500a9f237716c617aa))
* **ci:** skip commitlint on initial push ([950463d](https://github.com/hawk90/revue/commit/950463dd48585c0d860b6c17d78e69213804a194))
* **ci:** update deny.toml for cargo-deny v0.18+ ([7f2ec16](https://github.com/hawk90/revue/commit/7f2ec16fa93ae67d2cd3b5483c12eb5bcbaa7d95))
* **ci:** update deny.toml for cargo-deny v0.18+ compatibility ([fd9fbad](https://github.com/hawk90/revue/commit/fd9fbad20589261d4b694bb140ff3409d742e7d2))
* downgrade zune-jpeg to 0.4.21 for MSRV compatibility ([50f9d10](https://github.com/hawk90/revue/commit/50f9d10e0d325639809f8d3e713e0d016aa07763))
* ignore flaky test and add CI job dependencies ([b1f7bdd](https://github.com/hawk90/revue/commit/b1f7bdd2868a82ccfc7b4e39e87e1142a92bd6ee))
* ignore unmaintained advisories and update CI badges ([35c0db1](https://github.com/hawk90/revue/commit/35c0db1c8820570748979d322870b5a0aad9847d))
* resolve all example warnings for CI ([4fa0360](https://github.com/hawk90/revue/commit/4fa03602991088e826df4c5738936460491b6ba8))
* resolve CI failures across all platforms ([e654e23](https://github.com/hawk90/revue/commit/e654e235a85370f8be49997f08daac3582610212))
* resolve CI failures and reorganize workflow ([2df63d6](https://github.com/hawk90/revue/commit/2df63d632970cf9ac55ab0d534c4b764abc15f69))
* resolve clippy warnings (impl derive, repeat_n, div_ceil) ([e17d8d8](https://github.com/hawk90/revue/commit/e17d8d859e9531bed2aedf7d1a445638f2b7ae39))
* resolve clippy warnings and flaky test issues ([5d3ab22](https://github.com/hawk90/revue/commit/5d3ab22625644f724281901d2cb05c3f954999bd))
* resolve critical bugs and improve code quality ([eef7b15](https://github.com/hawk90/revue/commit/eef7b15947f76335c343389b1db136d0868b29d1))
* resolve doc links and security workflow ([bc12246](https://github.com/hawk90/revue/commit/bc1224663d1f510eb06d174644dc890906428eed))
* resolve doc warnings and update MSRV to 1.85 ([ed7cfbf](https://github.com/hawk90/revue/commit/ed7cfbfaa7ca808cbfe63f8ae155c7f67c15a3a3))
* update to taffy 0.9.2 API and remove dead_code allow ([e919b41](https://github.com/hawk90/revue/commit/e919b41a15f70410313ec35e1962d7433ac4d57c))

## [Unreleased]

## [1.0.0] - 2026-01-02

### Highlights

**Revue 1.0 is here!** After 10 releases of iterative development, Revue is now production-ready.

### Added

- **Production Ready**
  - Stable API with semantic versioning guarantee
  - Complete documentation with tutorials and guides
  - 80%+ test coverage with visual regression testing
  - Cross-platform support (Linux, macOS, Windows)

### Changed

- **API Stabilization**
  - Removed all deprecated APIs for clean 1.0 release
  - Finalized widget constructor patterns
  - Stabilized reactive system API

### Removed

- `Runtime` struct (deprecated since v0.8.0) - use `App::builder()` instead

### Migration from v0.9.0

No breaking changes from v0.9.0. Simply update your `Cargo.toml`:

```toml
[dependencies]
revue = "1.0"
```

## [0.9.0] - 2026-01-02

### Added

- **Documentation**
  - Complete tutorial series (Getting Started, Counter, Todo)
  - Comprehensive guides:
    - Styling Guide - CSS, variables, themes, transitions
    - State Management Guide - Signals, computed, effects, async
    - Testing Guide - Pilot framework, snapshots, visual testing
    - Performance Guide - Optimization, profiling, memory
    - Accessibility Guide - WCAG, focus management, screen readers

### Changed

- Updated tutorials to use correct v0.9 API patterns
- Fixed API examples in documentation

## [0.8.0] - 2026-01-02

### Added

- **API Stabilization**
  - `View` implementation for `Box<dyn View>` enabling boxed views as children
  - Migration guide documentation (`docs/migration/v0.8.0.md`)
  - Property-based tests using proptest for core components

- **Testing Improvements**
  - 11 new property-based tests for Rect, Color, and Signal
  - Proptest integration for fuzz testing

### Changed

- **Widget API Consistency**
  - Fixed gallery example to use correct widget APIs
  - Documented constructor patterns (convenience vs builder)

### Fixed

- Gallery example compilation errors with correct API usage
- Border widget usage patterns (Border::rounded().child())
- Checkbox, Switch, and Gauge API usage

## [0.7.0] - 2026-01-02

### Added

- **Plugin CLI Commands**
  - `revue plugin list` - List installed plugins
  - `revue plugin search <query>` - Search crates.io for plugins
  - `revue plugin install <name>` - Install a plugin
  - `revue plugin info <name>` - Show plugin information
  - `revue plugin new <name>` - Create new plugin project

- **VS Code Extension** (`extensions/vscode/`)
  - Revue CSS syntax highlighting
  - 25+ Rust widget snippets (vstack, hstack, border, button, input, etc.)
  - CSS snippets for Revue-specific properties
  - Language configuration for `.rcss` files

- **Zed Extension** (`extensions/zed/`)
  - Revue CSS language support
  - Syntax highlighting via tree-sitter
  - Language configuration

- **Online Playground** (`playground/`)
  - WASM-based terminal emulator
  - Live code editing with syntax highlighting
  - Example templates (Hello World, Counter, Todo, Dashboard)
  - Share functionality via URL

- **Theme Builder** (`tools/theme-builder/`)
  - Interactive TUI for creating themes
  - Live preview of color changes
  - Preset themes (Tokyo Night, Dracula, Nord, Gruvbox, Catppuccin)
  - CSS export with full widget styles

### Changed

- CLI updated with ureq and serde_json for HTTP API calls
- Plugin ecosystem now supports crates.io discovery

## [0.6.0] - 2026-01-02

*Note: v0.5.0 features were merged into v0.6.0 release*

### v0.5.0 Features (DX & Testing)

- **Visual Regression Testing**
  - `VisualTest` for pixel-perfect UI comparisons
  - `VisualCapture` with color, style, and text capture
  - `VisualDiff` for detailed difference reporting
  - Golden file serialization/deserialization
  - Color tolerance for fuzzy matching
  - `REVUE_UPDATE_VISUALS=1` for updating golden files

- **CI Integration**
  - `CiEnvironment` with auto-detection for GitHub Actions, GitLab CI, CircleCI, Travis CI, Jenkins, Azure Pipelines
  - `TestReport` with markdown generation
  - GitHub Actions annotations for test failures
  - Artifact collection for failed tests
  - Branch, commit, PR detection

- **DevTools**
  - `Inspector` - Widget tree viewer
  - `StateDebugger` - Reactive state viewer
  - `StyleInspector` - CSS style inspector with property sources
  - `EventLogger` - Event stream logger with filtering
  - Tabbed panel UI with position options (Right, Bottom, Left, Overlay)
  - F12 toggle support

- **Performance Profiler**
  - `Profiler` with hierarchical timing
  - `profile()` macro for easy instrumentation
  - Statistics: count, total, min, max, average
  - Global profiler instance
  - Report generation

- **Examples & Gallery**
  - 22+ example applications
  - Widget gallery (`gallery.rs`)
  - Dashboard, IDE, chat, todo examples

### Added

- **Drag & Drop System**
  - `DragContext` for managing drag state and data transfer
  - `Draggable` trait for widgets that can be dragged
  - `DragData` with type-erased payload and MIME types
  - `DropZone` widget for drop targets with visual feedback
  - `SortableList` widget for reorderable lists
  - `DragState` enum: Idle, Dragging, Over, Dropped

- **Resize & Layout**
  - `Resizable` widget wrapper for dynamic sizing
  - `ResizeHandle` with 8 directions (corners + edges)
  - Aspect ratio preservation and grid snapping
  - `Breakpoints` system for responsive layouts (XS, SM, MD, LG, XL)
  - `ResponsiveValue<T>` for breakpoint-aware values
  - `MediaQuery` for width/height-based conditionals

- **Focus Management**
  - Nested focus traps with `push_trap()` / `pop_trap()`
  - Focus restoration with `release_trap_and_restore()`
  - `FocusTrap` helper struct with RAII-style cleanup
  - `FocusTrapConfig` for customizing trap behavior
  - `trap_depth()` for querying nesting level

- **Performance Optimizations**
  - `VirtualList` variable height support with `HeightCalculator`
  - Binary search for O(log n) row lookup in virtual lists
  - `ScrollMode` (Item, Pixel) and `ScrollAlignment` options
  - `jump_to()`, `scroll_by()`, `scroll_position()` methods
  - Lazy loading patterns: `LazyData`, `LazyReloadable`, `LazySync`
  - `PagedData` for paginated datasets
  - `ProgressiveLoader` for chunked loading
  - `RenderBatch` for batched terminal operations
  - `RenderOp` enum for optimized render commands
  - Consecutive cell merging into text operations
  - Object pooling: `ObjectPool<T>`, `SyncObjectPool<T>`
  - `BufferPool` for render buffer reuse
  - `StringPool` / `SyncStringPool` for string interning
  - `VecPool<T>` for vector reuse
  - `Pooled<T>` RAII guard for automatic pool return
  - `PoolStats` for monitoring cache hit rates

### Changed

- Prelude now exports drag/drop, resize, and lazy loading types
- `FocusManager` supports nested traps with stack-based management
- Widget module exports `DropZone`, `SortableList`, `Resizable`
- Layout module exports `Breakpoints`, `ResponsiveValue`, `MediaQuery`
- Patterns module exports lazy loading types and constructors
- Render module exports `RenderBatch`, `RenderOp`, `BatchStats`
- DOM module exports pooling types

## [0.4.0] - 2026-01-02

### Added

- **Thread-Safe Reactive System**
  - `Signal<T>` now uses `Arc<RwLock<T>>` for thread-safety
  - Async hooks: `use_async()`, `use_async_poll()`, `use_async_immediate()`
  - `AsyncState` and `AsyncResult` types for async operations
  - Thread-safe tracker, effect, and computed primitives

- **Accessibility System**
  - High-contrast themes: `HighContrastDark`, `HighContrastLight` (WCAG AAA compliant)
  - `BuiltinTheme::accessibility()` and `is_accessibility()` helpers
  - Focus indicator rendering: `draw_focus_ring()`, `draw_focus_underline()`, `draw_focus_marker()`
  - `FocusStyle` enum: Solid, Rounded, Double, Dotted, Bold, Ascii
  - Screen reader announcements: `announce()`, `announce_now()`, `take_announcements()`
  - Widget-specific helpers: `announce_button_clicked()`, `announce_checkbox_changed()`, etc.
  - Preference functions: `prefers_reduced_motion()`, `is_high_contrast()`

- **Animation Engine**
  - `KeyframeAnimation` - CSS @keyframes style with percentage-based keyframes
  - `CssKeyframe` for defining property values at each percentage
  - `AnimationDirection`: Normal, Reverse, Alternate, AlternateReverse
  - `AnimationFillMode`: None, Forwards, Backwards, Both
  - `Stagger` for staggered animation delays across multiple elements
  - `AnimationGroup` for parallel/sequential animation coordination
  - `Choreographer` for managing multiple named animation sets
  - `widget_animations` module with pre-built effects:
    - `fade_in()`, `fade_out()`, `slide_in_left/right/top/bottom()`
    - `scale_up()`, `scale_down()`, `bounce()`, `shake()`
    - `pulse()`, `blink()`, `spin()`, `cursor_blink()`
    - `toast_enter()`, `toast_exit()`, `modal_enter()`, `modal_exit()`
    - `shimmer()` for loading effects

- **Reduced Motion Support**
  - `should_skip_animation()` - check if animations should be skipped
  - `effective_duration()` - returns zero duration when reduced motion preferred
  - All animations automatically respect user's reduced motion preference
  - `TransitionManager` skips transitions when reduced motion is enabled

### Changed

- Prelude now exports animation types and accessibility functions
- `Signal::new()` returns thread-safe signal usable across threads
- All reactive primitives support concurrent access

## [0.3.0] - 2026-01-02

### Added

- **Plugin System**
  - `Plugin` trait with lifecycle hooks: `on_init`, `on_mount`, `on_tick`, `on_unmount`
  - `PluginContext` for plugin data storage and cross-plugin communication
  - `PluginRegistry` for managing plugin ordering by priority
  - Built-in plugins: `LoggerPlugin`, `PerformancePlugin`
  - `App::builder().plugin()` method for registering plugins
  - Plugin styles collected and merged with app stylesheet

- **Runtime Theme Switching**
  - Signal-based theme system via `use_theme()` returning `Signal<Theme>`
  - Theme functions: `set_theme()`, `set_theme_by_id()`, `toggle_theme()`, `cycle_theme()`
  - Theme registration: `register_theme()`, `get_theme()`, `theme_ids()`
  - `theme_to_css_variables()` for generating CSS variable stylesheets
  - `ThemePicker` widget for interactive theme selection

- **New Patterns**
  - `SearchState` for list filtering with fuzzy/contains/prefix/exact modes
  - `FormState` with field validation, focus navigation, and submit handling
  - `FormField` with validators: required, min/max length, email, numeric, custom
  - `NavigationState` for browser-like history with back/forward navigation
  - `Route` with path matching and parameters
  - `build_breadcrumbs()` helper for navigation trails

- **CLI Enhancements**
  - `revue add <component>` command for generating component templates
    - Components: search, form, navigation, modal, toast, command-palette, table, tabs
  - `revue benchmark` command for running Criterion benchmarks
  - Component templates with full working examples

### Changed

- Prelude now exports theme functions and new pattern types
- Widget module exports `ThemePicker` and `theme_picker()` constructor
- Patterns module exports search, form, and navigation types

## [0.2.0] - 2025-01-02

### Added

- **Testing Infrastructure**
  - Expanded widget snapshot tests from 29 to 65 test cases
  - Criterion benchmarks for DOM, CSS, Layout, and Rendering
  - DOM benchmark measuring incremental build performance

- **Performance Optimizations**
  - Incremental DOM build: reuses existing nodes by ID or position
    - 2% faster for 10 children
    - 36% faster for 50 children
    - 54% faster for 100 children
  - Node-aware transition tracking for partial rendering
  - Style cache preservation for unchanged nodes

- **Transition System Enhancements**
  - `TransitionManager::start_for_node()` for element-specific transitions
  - `TransitionManager::active_node_ids()` for partial rendering optimization
  - `TransitionManager::get_for_node()` and `current_values_for_node()`

- **DOM Renderer Methods**
  - `DomRenderer::invalidate()` to force fresh rebuild
  - `DomRenderer::build_incremental()` for efficient updates

### Changed

- `DomRenderer::build()` now performs incremental updates when possible
- Transition updates now process both legacy and node-aware transitions
- Dirty rect calculation uses node-specific areas for active transitions

### Fixed

- Documentation links with proper module prefixes (`widget::`, `app::`, etc.)
- Escaped brackets in widget documentation (checkbox, radio, switch)
- Ambiguous doc links using `mod@` prefix
- Missing `keys` module export in patterns

## [0.1.0] - 2024-XX-XX

### Added

- Core rendering engine with double buffering
- CSS parser with variables, selectors, transitions, and animations
- Flexbox layout powered by taffy
- Reactive state management (Signal, Computed, Effect)
- 70+ widgets
  - Layout: Stack, Grid, Scroll, Tabs, Accordion, Splitter
  - Input: Input, TextArea, Select, Checkbox, Switch, Slider, ColorPicker
  - Display: Text, RichText, Markdown, Table, Progress, Badge, Image
  - Feedback: Modal, Toast, Notification, Tooltip
  - Data Viz: BarChart, LineChart, Sparkline, Heatmap
  - Advanced: Terminal, Vim mode, AI Streaming, Mermaid diagrams
- Hot reload for CSS files
- Widget inspector (devtools)
- Snapshot testing utilities
- Built-in themes (Dracula, Nord, Monokai, Gruvbox, Catppuccin)
- Kitty graphics protocol support for images
- Unicode and emoji support
- Clipboard integration
- i18n support

[Unreleased]: https://github.com/hawk90/revue/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/hawk90/revue/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/hawk90/revue/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/hawk90/revue/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/hawk90/revue/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/hawk90/revue/compare/v0.4.0...v0.6.0
[0.4.0]: https://github.com/hawk90/revue/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/hawk90/revue/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/hawk90/revue/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/hawk90/revue/releases/tag/v0.1.0
