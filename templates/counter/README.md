# Counter App

Reactive counter demonstrating Signal and Computed state management.

## Run

```bash
cargo run
```

## Controls

- `+` / `=` / `Up` — Increment
- `-` / `Down` — Decrement
- `r` — Reset to zero
- `q` — Quit

## What's included

- `Signal<i32>` for mutable reactive state
- `Computed<i32>` for derived values (auto-cached)
- `Border::panel()` layout with inline CSS
- Event handling with key matching
