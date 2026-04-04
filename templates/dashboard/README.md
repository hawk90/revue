# Dashboard App

Multi-panel dashboard with real-time data updates and CSS styling.

## Run

```bash
cargo run
```

Press `q` to quit.

## What's included

- Multiple bordered panels with `Border::panel()`
- `hstack()` / `vstack()` grid layout
- Progress bars for CPU/Memory
- External CSS file (`src/style.css`) with CSS variables
- Real-time data updates via `Event::Tick`
- CSS classes for status colors (`.status-ok`, `.status-error`)
