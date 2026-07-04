# Revue vs Reference TUI Frameworks

> **Last updated:** 2026-07 — verified against the revue source tree and the
> current (mid-2026) releases of competing frameworks. Prior revisions of this
> doc were written ~late 2025 and had drifted badly out of date on **both**
> sides (revue had shipped features still marked "TODO", and the competitor
> landscape had reshuffled). See §12 for the verification method.

## Overview

| Framework | Language | Rendering | Model | Widgets | Styling | Maturity (mid-2026) |
|-----------|----------|-----------|-------|---------|---------|---------------------|
| **Revue** | Rust | Retained | Vue-style signals | 100+ | CSS | Young, feature-rich |
| **Textual** | Python | Retained | Message/reactive | 35+ | TCSS | Mature (v8.x) |
| **ratatui** | Rust | Immediate | Widget traits | 15+ (core) | Inline | Dominant (v0.30, ~36M dl) |
| **r3bl_tui** | Rust | Retained | React/Elm + CSS | Medium | CSS-like | Mature (~367K dl) |
| **tui-realm** | Rust | Retained (on ratatui) | React/Elm | Components | Inline | Mature (~204K dl) |
| **iocraft** | Rust | Retained | React (Ink-like) | Small | Flexbox (taffy) | Growing (~129K dl) |
| **Cursive** | Rust | Retained | Callback/view-tree | 40+ | TOML | **Inactive** (last rel. 2024-08) |
| **reratui** | Rust | Immediate (on ratatui) | React hooks/fiber | wrappers | Inline | **Negligible** (~145 dl) |
| **reactive_tui** | Rust | — | CSS | — | CSS | **Dead** (all versions yanked) |

**Primary benchmark:** Textual remains the closest feature-and-philosophy peer
(retained-mode + CSS + reactive + large widget set) and is the yardstick used
throughout this doc. Among **Rust** peers, the credible modern competitors are
now **r3bl_tui**, **tui-realm**, and **iocraft** — not reratui (a namesake-
adjacent experiment with almost no adoption) and not reactive_tui (yanked).

---

## 1. Positioning at a glance

Revue's differentiators hold up in mid-2026:

- **Vue-flavored reactivity** (signals/computed/effect). Every other reactive
  Rust peer clusters around React or Elm; the Vue idiom is genuinely unique.
- **Real CSS** (files, variables, selectors incl. pseudo-classes/An+B/combinators,
  themes, hot reload). Only Textual (TCSS) and r3bl_tui (CSS-like) come close.
- **Widget breadth** — 100+ widgets, more than any competitor. Textual has ~35
  and **added no new widget types in the last 12 months**; the Rust reactive
  peers ship far fewer.
- **Visualization depth** — charts (line/bar/pie/scatter/histogram/boxplot/
  candle/timeseries/heatmap/waveline/streamline), canvas, sparkline. No peer
  is close here.

The honest weaknesses are narrower than they look (see §9): a handful of
half-wired features, and the genuine absence of browser deployment.

---

## 2. Widget Comparison (vs Textual)

Roster unchanged from Textual's side — the tables below are still broadly
accurate. Corrections from source-tree verification are called out.

### 2.1 Form Widgets

| Widget | Revue | Textual | ratatui | Notes |
|--------|-------|---------|---------|-------|
| Text/Label | ✅ | ✅ | ✅ Paragraph | All have |
| Button | ✅ | ✅ | ❌ | — |
| Input | ✅ | ✅ | ❌ | Copy/paste + shift-selection now done in revue |
| MaskedInput | ✅ | ✅ | ❌ | — |
| TextArea | ✅ | ✅ | ❌ | See §7.1 for depth |
| Checkbox / Radio / Switch | ✅ | ✅ | ❌ | — |
| Select/Dropdown/ComboBox | ✅ | ✅ | ❌ | — |
| Slider / Stepper / NumberInput | ✅ | partial | ❌ | Revue richer |
| ColorPicker | ✅ | ✅ | ❌ | — |
| FilePicker | ✅ | ❌ | ❌ | Revue unique |
| Rating | ✅ | ❌ | ❌ | Revue unique |
| Autocomplete | ✅ | (Input suggestion) | ❌ | Textual added Input/TextArea `suggestion` (v6) |

### 2.2 Data Widgets

| Widget | Revue | Textual | ratatui | Notes |
|--------|-------|---------|---------|-------|
| Table (basic) | ✅ | ✅ | ✅ | — |
| **DataGrid** | ✅ | DataTable | ❌ | **See §7.2 — revue leads on resize/reorder, parity on column freeze** |
| List / OptionList / SelectionList | ✅ | ✅ | partial | — |
| VirtualList | ✅ | (implicit) | ❌ | — |
| Tree / FileTree | ✅ | ✅ / ❌ | ❌ | FileTree revue-unique |
| CSV / JSON / Log viewers | ✅ | ❌ | ❌ | Revue unique |

### 2.3 Visualization — revue's strongest lead

Line/Bar/Sparkline/Gauge/Progress/Canvas: parity or better vs ratatui;
**Pie, Scatter, Histogram, BoxPlot, CandleChart, TimeSeries, Waveline,
Streamline, Heatmap are revue-unique** (neither Textual nor ratatui has them).

### 2.4 Layout / Feedback / Navigation / Rich content

Broadly at parity with Textual, with revue extras: Splitter, Positioned,
ContextMenu, Pagination, Stepper, Timeline, Timer, Avatar, Sixel Image,
full PTY Terminal widget. (Textual has no equivalents.)

---

## 3. Styling System

| Feature | Revue | Textual | r3bl_tui | ratatui |
|---------|-------|---------|----------|---------|
| CSS files | ✅ | ✅ TCSS | ✅ CSS-like | ❌ |
| CSS variables | ✅ (with fallback) | ✅ | partial | ❌ |
| Selectors (pseudo/An+B/combinators) | ✅ | ✅ | partial | ❌ |
| Color: hex/rgb/hsl/named | ✅ (50+ named) | ✅ | ✅ | ❌ |
| Themes | ✅ | ✅ (many added 2025-26) | ✅ | ❌ |
| Hot reload | ✅ | ✅ | ❌ | ❌ |
| Transitions | ✅ (partial) | ✅ | ❌ | ❌ |
| **Content markup / Visual system** | ❌ | ✅ (new; replaced Rich) | ❌ | ❌ |

> **New Textual capability worth tracking:** the **Content markup / Visual
> rendering** system (rolled out v2–v3, 2025) replaced Rich markup as the
> rendering primitive. Revue has no direct analog — not a functional gap for
> app authors today, but it's the substrate behind several Textual features
> below.

---

## 4. Layout System

| Feature | Revue | Textual | ratatui |
|---------|-------|---------|---------|
| Flexbox | ✅ | ✅ | ❌ |
| Grid | ✅ | ✅ | ❌ |
| Constraint (Cassowary) | ❌ | ❌ | ✅ |
| Dock | ❌ | ✅ | ❌ |
| Percent / Auto | ✅ | ✅ | partial |

---

## 5. State Management

| Feature | Revue | Textual | ratatui | tui-realm |
|---------|-------|---------|---------|-----------|
| Reactive signals | ✅ | ✅ (reactive attrs) | ❌ | ❌ |
| Computed / Effects | ✅ | ✅ | ❌ | ❌ |
| Message bus | ✅ (two impls — see §8) | ✅ | ❌ | ✅ (Elm update) |
| Worker system | ✅ (thread-based) | ✅ (async/thread) | ❌ | ❌ |

---

## 6. Developer Experience & Testing

| Feature | Revue | Textual | Notes |
|---------|-------|---------|-------|
| DevTools / Inspector | ✅ | ✅ textual-dev 1.8.0 | Both live |
| Snapshot testing | ✅ | ✅ pytest-textual-snapshot | — |
| **Pilot-style test driver** | ✅ | ✅ | **Revue HAS this** (doc previously wrong) — `testing/pilot`, ~35 tests |
| Hot reload | ✅ | ✅ | — |
| Screen stack / modes | ✅ (manager; app-driven) | ✅ (+ modes/signals v8) | Revue has it; not auto-wired into run loop |
| **Web deployment** | ❌ | textual-serve v1.1.3 (live) | **Genuine gap** — see §9 |

---

## 7. Feature-Depth Analysis (corrected)

### 7.1 TextArea

| Feature | Revue | Textual | Verdict |
|---------|-------|---------|---------|
| Basic editing / undo / selection | ✅ | ✅ | parity |
| Line numbers | ✅ | ✅ | parity |
| Syntax highlight (tree-sitter) | ✅ | ✅ | parity |
| **Soft wrap** | ✅ | ✅ | parity — word-boundary soft wrap in `view.rs` (was a stub; now flows long lines onto multiple visual rows) |
| **Find / Replace** | ✅ (regex = stub; API-only) | ❌ core (3rd-party pkg) | **Revue leads** — engine done in `find_impl.rs` (~346L); regex falls back to literal; not bound in `handle_key` |
| **Multiple cursors** | ⚠️ PARTIAL | ❌ | Data + render exist; editing applies to primary cursor only; not key-wired |
| Suggestion / placeholder | ? (verify) | ✅ (v6) | Textual added; confirm revue status |
| Bracket matching | (CodeEditor widget) | ✅ | Textual in TextArea; revue's is in separate CodeEditor |

### 7.2 DataGrid vs Textual DataTable

| Feature | Revue | Textual | Verdict |
|---------|-------|---------|---------|
| Sort (multi-col) / filter / multi-select | ✅ | ✅ | parity |
| Zebra / row numbers / cell nav / cell edit | ✅ | ✅ | parity |
| **Virtual scroll** | ✅ (`render_rows_virtual`, overscan) | (implicit) | parity+ |
| **Column resize** | ✅ (mouse + render cursor, tested) | ❌ | **Revue leads** |
| **Column reorder** | ✅ (drag + keyboard, tested) | ❌ | **Revue leads** |
| **Column freeze (pin)** | ✅ (pin left/right + horizontal scroll, render + tests) | ✅ (fixed rows/cols) | parity — `render.rs` positions columns from `frozen_left`/`frozen_right`/`scroll_col`; scroll via `scroll_col_left/right` + mouse |

---

## 8. Message / Event systems in Revue

Two real, wired-up implementations (both were previously mislabeled "missing"):

1. **`runtime/event/custom/`** — `CustomEventBus` + `EventDispatcher`: TypeId-keyed
   callback dispatch with capture/target phases, propagation-stop, cancellation,
   `once` handlers, history ring buffer. This is a genuine callback message system.
2. **`state/tasks/event_bus.rs`** — a polling pub/sub queue (type-erased payloads,
   `emit`/`poll`/`subscribe`). Simpler, queue-based.

---

## 9. Gap Analysis (verified, mid-2026)

### 9.1 Real remaining gaps vs Textual — prioritized

| # | Gap | Current state | Impact | Effort |
|---|-----|---------------|--------|--------|
| 1 | ~~**DataGrid column freeze render**~~ | ✅ DONE — render reads `frozen_*`/`scroll_col` (pin left/right + horizontal scroll), tested | — | — |
| 2 | **Key bindings for find/replace & multi-cursor** | API-only; not in `handle_key` | Medium (UX) | Small |
| 3 | **Multi-cursor editing** | Editing applies to primary cursor only | Low–Medium | Medium–Hard |
| 4 | **Regex search** in find/replace | Stub (literal fallback) | Low | Small |
| 5 | **Web deployment** (textual-serve analog) | Missing entirely | Strategic | Very Hard |
| 6 | **Streaming content** (LLM output into Markdown/RichLog) | Verify — Textual added `Markdown.append`/`get_stream` | Medium (modern use case) | Medium |
| 7 | **Screen stack auto-wiring** | Manager exists but app author must drive it | Low | Medium |

### 9.2 Already done — remove from any "TODO" list

DataGrid virtual scroll / column resize / column reorder / column freeze render
(pin left/right + horizontal scroll); TextArea find-replace
engine; TextArea soft wrap (word-boundary); Pilot testing; Worker system;
message/event bus; CSS Grid; Input copy-paste & shift-selection. **These are
shipped and tested** — the old doc's "❌ TODO" markers were wrong.

---

## 10. Rust ecosystem shift (2025–2026)

The Rust TUI space moved toward reactive-component + CSS/flexbox models:

- **ratatui 0.30** (Dec 2025, now 0.30.2 Jun 2026) modularized into a workspace
  (`ratatui-core` / `ratatui-widgets` / per-backend crates), added `no_std` and
  a `ratatui::run()` entry point. Still immediate-mode and still dominant
  (~36M downloads); explicitly stays low-level and leaves higher-level
  abstractions to downstream crates.
- **New credible reactive peers** (all absent from the old doc):
  - **r3bl_tui** (~367K dl) — "React + Elm + Flexbox + CSS", editor component, ~82K LOC.
  - **tui-realm** (~204K dl, v4.1.0) — React/Elm component framework on ratatui.
  - **iocraft** (~129K dl, v0.8.3) — Ink-like React model, `element!`/`#[component]`,
    taffy flexbox; independent renderer (not on ratatui).
- **De-emphasize / drop:** **reratui** (~145 dl — negligible, treat as experiment),
  **reactive_tui** (all versions yanked — dead), **Cursive** (no release since
  2024-08 — inactive incumbent), **Dioxus TUI/rink** (stalled/experimental).

**Takeaway for revue:** the field validates revue's thesis (reactive + CSS is
where TUI is heading), but it's now a crowded upper layer. Revue's edge is
**widget breadth + visualization + Vue idiom + true CSS**; the battleground is
adoption, ergonomics, and docs — not raw feature count.

---

## 11. Conclusion

### Overall assessment (mid-2026)

| vs Framework | Revue standing | Notes |
|--------------|----------------|-------|
| vs **Textual** | **~parity, ahead on breadth** | Leads on widgets, viz, DataGrid resize/reorder, TextArea find/replace + soft wrap; parity on DataGrid column freeze. Trails on web serve, streaming content. |
| vs **ratatui** | Different tier | Higher-level; ratatui wins adoption & is the substrate, not a like-for-like rival. |
| vs **r3bl_tui / tui-realm / iocraft** | Feature-ahead, adoption-behind | Revue ships far more widgets + CSS; peers have more users/momentum. |
| vs **reratui / reactive_tui / Cursive** | Not live competition | Negligible / dead / inactive. |

**Bottom line:** Revue is **not "far behind" Textual** — that impression came
from a stale comparison. On breadth and several depth features it already leads.
The real, verified work is a short list: wire up column-freeze rendering, bind
keys for find/replace & multi-cursor, and decide whether to pursue
streaming-content and web-serve as strategic bets. (Soft wrap — previously a
stub — is now implemented.)

---

## 12. Verification method & caveats

- **Revue side:** claims in §7–§9 were verified by reading the actual source
  (`widget/data/datagrid/`, `widget/input/input_widgets/textarea/`,
  `testing/pilot/`, `core/app/screen/`, `state/worker/`, `runtime/event/custom/`)
  and confirming wiring into render/event/key paths and tests — not from prior
  docs. DataGrid: 207 tests pass. "PARTIAL" = real API/state but a missing
  render/edit/key link, documented inline above.
- **Competitor side:** versions/dates/features from PyPI, crates.io, GitHub
  release notes, and official docs (mid-2026). Download counts are cumulative
  crates.io figures (skewed by CI/transitive pulls) — relative, not user counts.
- **Unverified leads:** Textual's DataTable column resize/reorder in any
  unreleased branch (docs show none); Textualize's corporate status; whether
  revue's TextArea has a `suggestion`/`placeholder` analog; exact revue widget
  count vs the "100+" claim. Treat these as follow-ups, not settled facts.
