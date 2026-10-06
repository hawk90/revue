# Dashboard App

Multi-panel dashboard with real-time data updates and CSS styling.

## Run

```bash
cargo run
```

Press `q` to quit.

## What's included

- Bordered panels (`Border::new()`) whose border style and color come from CSS
- `hstack()` / `vstack()` grid layout: `child_flex` splits a row's width
  evenly, `child_sized` fixes a row's height, and a flexible body keeps the
  footer on the last line
- Progress bars for CPU/Memory
- External CSS file (`src/style.css`) with CSS variables
- Data updates on `Event::Tick`, throttled to twice a second; the handler
  returns `true` to redraw
- CSS classes for the title and status colors (`.panel-title`, `.status-ok`,
  `.status-error`)
