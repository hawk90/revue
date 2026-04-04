# API Stability

## Version Policy

Revue follows [Semantic Versioning](https://semver.org/):

- **Patch** (2.x.Y): Bug fixes, performance improvements, documentation updates
- **Minor** (2.X.0): New features, new widgets, new CSS properties (backward compatible)
- **Major** (X.0.0): Breaking API changes (rare, with migration guide)

## Stability Tiers

### Stable (safe to depend on)

These APIs will not change without a major version bump:

| API | Since |
|-----|-------|
| `App::builder().build().run()` | v2.0 |
| `View` trait (`render`, `meta`) | v2.0 |
| `RenderContext` drawing methods (`set`, `put_str`, `sub_area`) | v2.0 |
| `Signal<T>`, `Computed<T>`, `Effect` | v2.0 |
| `signal()`, `computed()`, `effect()` | v2.0 |
| Widget constructors (`vstack()`, `hstack()`, `text()`, etc.) | v2.0 |
| `Key`, `KeyEvent`, `Event` types | v2.0 |
| CSS property parsing (all documented properties) | v2.60 |
| `Color::rgb()`, `Color::hex()`, named colors | v2.0 |
| `FocusManager`, `FocusTrap` | v2.61 |

### Evolving (may change in minor versions)

These APIs work but may be refined:

| API | Notes |
|-----|-------|
| `RenderContext::with_clip()` | Clipping API is new (v2.66) |
| `overflow` CSS enforcement | Container clipping is new |
| `Store`, `StoreRegistry` | Pinia-style stores, API stabilizing |
| DevTools inspector | Developer-facing, may change |
| `FormState` validation | Pattern stabilizing |

### Experimental (may change or be removed)

| API | Notes |
|-----|-------|
| `image` feature (Sixel/Kitty/iTerm2) | Terminal-dependent |
| `terminal` widget (PTY) | Platform-dependent |
| `sysinfo` feature | System monitoring |

## Minimum Supported Rust Version (MSRV)

**Current MSRV: 1.87**

MSRV bumps are treated as minor version changes and documented in CHANGELOG.md.

## Deprecation Policy

1. Deprecated items are marked with `#[deprecated(since = "x.y.z", note = "use X instead")]`
2. Deprecated items are kept for at least 2 minor versions
3. Removal is documented in CHANGELOG.md with migration instructions

## CSS Property Support

All CSS properties documented in [docs/FEATURES.md](docs/FEATURES.md) are considered stable. New properties are added in minor versions and never removed.

### Supported Property Categories

- **Layout**: display, position, flex-direction, flex-wrap, flex-grow, justify-content, align-items, align-self, order, gap
- **Spacing**: padding, margin, width, height, min/max constraints
- **Visual**: color, background, border (shorthand), opacity, visibility, overflow, z-index
- **Text**: text-align, font-weight, text-decoration
- **Colors**: hex, rgb, hsl/hsla, 50+ named colors, transparent, CSS variables with fallback
- **Selectors**: type, class, ID, pseudo-classes (:hover, :focus, :nth-child An+B), combinators
- **Animation**: transition, @keyframes
