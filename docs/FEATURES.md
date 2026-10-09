# Features

> **상태·목록의 정본은 [docs/specs/features.yaml](specs/features.yaml)** — the feature list, per-feature
> status (`done` / `partial` / `todo` / `dropped`), acceptance criteria and code evidence live there.
> This page keeps only the narrative: what Revue is for, and why it behaves the way it does.
> Schema and update rules: [docs/specs/README.md](specs/README.md). API reference: [docs.rs/revue](https://docs.rs/revue).

## Status summary (2026-10-09, verified against the code)

| | done | partial | todo | dropped | total |
|---|---|---|---|---|---|
| CSS (`CSS-*`) | 12 | 7 | 1 | 0 | 20 |
| Layout (`LAY-*`) | 4 | 2 | 0 | 0 | 6 |
| Reactivity (`RX-*`) | 5 | 0 | 0 | 0 | 5 |
| Widgets (`WID-*`) | 41 | 3 | 1 | 0 | 45 |
| Charts (`CHART-*`) | 8 | 0 | 0 | 0 | 8 |
| Navigation (`NAV-*`) | 3 | 0 | 0 | 0 | 3 |
| Unicode (`TEXT-*`) | 2 | 1 | 1 | 0 | 4 |
| Developer experience (`DX-*`) | 3 | 1 | 0 | 0 | 4 |
| Theming (`THEME-*`) | 2 | 0 | 1 | 0 | 3 |
| Keyboard & clipboard (`KEY-*`) | 3 | 1 | 0 | 0 | 4 |
| Utilities (`UTIL-*`) | 5 | 0 | 0 | 0 | 5 |
| **Total** | **88** | **15** | **4** | **0** | **107** |

Known gaps worth knowing before you build on them: CSS `transparent` paints black, `opacity` /
`z-index` / `position` are parsed but not applied, CSS transitions are not triggered by state changes,
the `Image` widget paints placeholders (no Kitty output yet), and `devtools(true)` does not mount an
overlay. Details are in each item's `notes`.

## What Revue is

Revue is a Vue-inspired TUI framework for Rust: you describe a view, style it with real CSS files, and
drive it from reactive state (`signal` / `computed` / `effect`). It ships a large widget library
(120 `View` types: inputs, data tables, charts, markdown, pickers, developer tools) so an application
mostly composes rather than draws.

## 1. CSS styling

Stylesheets are ordinary files loaded with `App::builder().style("app.css")`; later sheets merge into the
same cascade. The selector engine covers type, class, id, compound, state pseudo-classes
(`:focus`, `:hover`, `:disabled`, …), structural pseudo-classes including `:nth-child(An+B)`, the
descendant/child/sibling combinators and attribute selectors. Custom properties are declared in `:root`
and read with `var(--name, fallback)`.

### Every widget is styled by default

Since 3.0 a stylesheet reaches every widget a parent renders as a child: the DOM is built from the
render traversal (`dom_from_render`, on by default), so paint properties — colors, border, text — apply
to each widget's own node. Box properties (`display`, `width`, `height`, `margin`, `min-*`/`max-*`)
override geometry a container already computed (`css_layout`, also on by default).
`App::builder().dom_from_render(false).css_layout(false)` restores 2.x, where only the root widget was
styled; `css_layout(false)` alone keeps paint properties and drops box properties.

A property whose *initial* value is also its "off" value has to track whether it was specified at all,
or a stylesheet cannot set it back to that value. `gap`, `border-style` and the two grid gaps all do, so
`gap: 0` closes a gap the builder opened and `border-style: none` removes a border it drew. A stylesheet
that says nothing still leaves the builder's value alone — that is the point of the distinction.

`gap` (and `column-gap` / `row-gap`) reaches `vstack`, `hstack` and `grid` under `css_layout`. The
remaining flow properties (`flex-*`, `grid-template-*`) are the container's own and are not applied from
CSS — see [Layout Findings](refactor/findings-layout.md).

`border-style: dashed` draws a single line: terminals have no dashed box-drawing set, and silently
mapping it to something else would hide the fact.

### Parsed but not applied

`opacity`, `z-index`, `position` and `top`/`right`/`bottom`/`left` are accepted by the parser and reach
the computed style, but nothing reads them at paint time. They stay parseable so the gap is findable:

- **opacity** — a terminal cell has no alpha. Applying it means deciding what to blend against, and cells
  do not carry a reliable backdrop. `Text::dim()` is the terminal-native approximation.
- **z-index** — needs paint ordering. Widgets paint in traversal order today, and overlays are a separate
  queue.
- **position** and its offsets — read by the layout engine, whose rects nothing paints from (see
  [Layout Findings](refactor/findings-layout.md)).

`Positioned` and `Layers` cover absolute placement and stacking as builders.

## 2. Layout

Layout is computed by Revue's own in-tree engine (`src/runtime/layout/`) — a hand-written flexbox and
grid solver with no third-party layout dependency, so the semantics stay under the project's control. In
practice containers (`vstack`, `hstack`, `grid`) still place their own children from builder settings
(`gap`, `child_sized`, `child_flex`, `constrain`, and each unsized child's measured content size -
`View::measure`); the engine's output is consulted only for the
`css_layout` overrides. Closing that gap is tracked in [Layout Findings](refactor/findings-layout.md).

## 3. Reactivity

State is held in signals: `signal(v)` to create, `get` / `set` / `update` to use, `computed` for derived
values and `effect` for side effects. Writing a signal schedules a re-render, so the view function stays
a pure description of current state. Collections are signals too (`signal(vec![..])`, or `SignalVec`).

## 4. Widgets

Widgets are **state objects, not callback registries.** A `List`, `Select`, `Tabs` or `Checkbox` owns its
selection and exposes it (`selected_index()`, `is_checked()`, `handle_key()` return values); the
application reads that state in its event handler and decides what happens. That is why most widgets have
no `.on_change(closure)` — it keeps ownership simple and avoids `'static` closure plumbing through the
view tree. (`Form`, menus and trees are the exceptions that take callbacks.)

The library spans basic and input widgets, navigation (tabs, menu bar), layout (card, layers, grid),
feedback (modal, toast, alert, callout, status indicator, empty state), content (markdown, slide
presentations, images), charts (line, bar, pie/donut, scatter/bubble, histogram, box plot, heatmap,
candlestick) and developer tools (process monitor, HTTP client). The registry lists each one with its
acceptance and where it is tested.

## 5. Navigation and focus

Screens are just state: keep the current screen in a signal and match on it in the view, or use the
`Router` for path-based navigation (see [Routing](guides/routing.md)).

Tab order is **document order** — the order the reader meets things. There is no `focus_order` to assign;
move the widget and it moves in the ring. `App::builder().tab_navigation(true)`
makes Tab and Shift+Tab move `:focus`. `disabled` widgets are left out of the ring entirely, and a left
click focuses the nearest enclosing focusable widget. Both need `dom_from_render` (on by default) — with
it turned off no node below the root is associated with an area and neither `:focus` nor `:hover`
matches anything. See
[App builder › tab_navigation](guides/app-builder.md#tab_navigationenabled).

## 6. Unicode

Every cell computation is width-aware: ASCII is one cell, CJK and emoji two, zero-width joiners and
combining marks zero. Terminals and fonts disagree about ambiguous glyphs (Nerd Font icons especially),
which is why a configurable `CharWidthTable` exists — wiring it into rendering is still open.

## 7. Developer experience

The loop Revue aims for is: edit CSS, see it immediately (`hot_reload(true)`, or `revue dev`, which also
rebuilds and restarts the app on Rust changes), inspect what matched
(devtools panels: inspector, style inspector, event logger, profiler), and pin behavior down with
headless tests (`TestApp` + `insta` snapshots) that drive the same event and render pipeline as the real
terminal.

## 8. Theming

Themes are CSS: built-in theme stylesheets (Dracula, Nord, Catppuccin, Gruvbox, Monokai, high-contrast)
expose their palette as CSS variables, and `set_theme` / `toggle_theme` switch them at runtime, so
widget styles reference `var(--…)` instead of hard-coded colors.

## 9–11. Clipboard, keyboard, event utilities

Clipboard access goes through a pluggable backend (system or in-memory, with history) so tests never
touch the real clipboard. Keys arrive as `KeyEvent { key, ctrl, alt, shift }` in the `App::run` handler;
the `KeyMap` utility adds modes and chords (with Vim/Emacs presets) for apps that want them. `Debouncer`
and `Throttle` (leading/trailing/both edges) tame bursty input, the animation utilities (`Sequence`,
easing, springs) produce frame values, and the worker pool runs blocking work off the UI thread.

## Peers

The credible modern peers are Textual (closest by philosophy) and, among Rust frameworks, r3bl_tui,
tui-realm and iocraft. (`reratui` — an immediate-mode wrapper over ratatui with negligible adoption — is
not a meaningful peer.) Revue's distinguishing bets are real CSS files with variables and selectors,
Vue-style signals, and a batteries-included widget set compiled into a single binary; where a claimed
advantage is not fully delivered yet (devtools overlay, Kitty images) the registry says so.
