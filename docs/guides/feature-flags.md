# Feature Flags

Revue uses Cargo feature flags to keep compile times fast and binary sizes small. Only enable what you need.

## Default Features

```toml
[dependencies]
revue = "2.70"  # enables: async, config
```

| Feature | Dependencies | Purpose |
|---------|-------------|---------|
| `async` | `tokio` | Async runtime for background tasks, event loops |
| `config` | `serde`, `toml`, `dirs` | Configuration file loading and persistence |

## Preset Groups

For convenience, Revue provides preset feature groups:

| Preset | Includes | Use Case |
|--------|----------|----------|
| `std` | `async`, `config`, `tracing` | Standard apps with structured logging |
| `gui` | `async`, `config`, `markdown`, `image` | Rich content TUI apps |
| `all-gui` | `std` + `syntax-highlighting`, `qrcode`, `sysinfo`, `diff`, `clipboard`, `hot-reload` | Full-featured GUI apps |
| `full` | All features | Development, examples, and testing |

```toml
# Example: use the gui preset
revue = { version = "2.70", features = ["gui"] }
```

## Individual Features

### Content & Rendering

| Feature | Dependencies | Purpose |
|---------|-------------|---------|
| `markdown` | `pulldown-cmark` | Markdown widget for rendering rich text |
| `syntax-highlighting` | `tree-sitter-highlight` + 13 language parsers | Code syntax highlighting (Rust, Python, JS, Go, Bash, HTML, CSS, TOML, YAML, SQL, JSON, Markdown) |
| `image` | `image`, `base64` | Terminal image display via Kitty protocol |
| `qrcode` | `qrcode` | QR code generation widget |

### Utilities

| Feature | Dependencies | Purpose |
|---------|-------------|---------|
| `diff` | `similar` | Diff visualization between text versions |
| `sysinfo` | `sysinfo` | System information widget (CPU, memory, processes) |
| `clipboard` | `arboard` | System clipboard copy/paste support |
| `hot-reload` | `notify` | Watch CSS/layout files and reload on change during development |
| `http` | `reqwest` | HTTP request support |

### Development

| Feature | Dependencies | Purpose |
|---------|-------------|---------|
| `tracing` | `tracing`, `tracing-subscriber` | Structured logging and diagnostics |
| `devtools` | (none) | Developer tools UI overlay |

## Recommendations by Use Case

### Minimal CLI Tool

```toml
# Just the defaults
revue = "2.70"
```

### Dashboard App

```toml
revue = { version = "2.70", features = ["std"] }
```

### Text Editor

```toml
revue = { version = "2.70", features = ["std", "syntax-highlighting", "clipboard"] }
```

### Content Viewer

```toml
revue = { version = "2.70", features = ["gui", "syntax-highlighting"] }
```

### Full-Featured Application

```toml
revue = { version = "2.70", features = ["all-gui"] }
```

### Development with Hot Reload

```toml
# In your development profile
revue = { version = "2.70", features = ["full"] }
```

## Build Impact

The `syntax-highlighting` feature is the heaviest, pulling in 13 tree-sitter language parsers. If you only need a few languages, consider using the base framework and adding tree-sitter parsers manually.

| Configuration | Approximate Dependencies |
|--------------|------------------------|
| Default (`async`, `config`) | ~6 crates |
| `std` | ~10 crates |
| `gui` | ~15 crates |
| `all-gui` | ~35 crates |
| `full` | ~40+ crates |

## Disabling Defaults

To start with no features and add only what you need:

```toml
revue = { version = "2.70", default-features = false, features = ["async"] }
```
