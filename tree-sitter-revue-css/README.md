# tree-sitter-revue-css

Tree-sitter grammar for Revue CSS - the styling language for the Revue TUI framework.

## Features

- Full CSS3 syntax support
- CSS custom properties (`--var-name`)
- `var()` function with fallback values
- Keyframe animations
- Media queries
- Pseudo-classes and pseudo-elements
- Complex selectors (combinators, attribute selectors)

## Installation

### Neovim

Add to your tree-sitter configuration:

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()

parser_config.revue_css = {
  install_info = {
    url = "https://github.com/user/revue/tree-sitter-revue-css",
    files = {"src/parser.c"},
  },
  filetype = "css",
}
```

### Helix

Add to `languages.toml`:

```toml
[[language]]
name = "revue-css"
scope = "source.revue-css"
file-types = ["css"]
roots = []

[language.grammar]
name = "revue_css"
source = { git = "https://github.com/user/revue", subdir = "tree-sitter-revue-css" }
```

### VS Code

Install the Revue VS Code extension (coming soon).

## Usage

```bash
# Install dependencies
npm install

# Generate parser
npm run build

# Run tests
npm test

# Parse a file
npm run parse -- example.css
```

## Syntax Highlighting

The grammar includes highlight queries for:

| Node | Highlight Group |
|------|-----------------|
| `.class` | `@attribute` |
| `#id` | `@label` |
| `property` | `@property` |
| `--variable` | `@variable.parameter` |
| `#hex` | `@constant.numeric` |
| `"string"` | `@string` |
| `123px` | `@number` |
| `rgb()` | `@function` |
| `@keyframes` | `@keyword` |

## Example

```css
/* Revue CSS example */
:root {
  --primary: #bd93f9;
  --bg: #282a36;
}

.button {
  color: var(--primary);
  background: var(--bg);
  transition: all 0.3s ease;
}

.button:hover {
  background: #44475a;
}

@keyframes pulse {
  0% { opacity: 1; }
  50% { opacity: 0.5; }
  100% { opacity: 1; }
}
```

## License

MIT
