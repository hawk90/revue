# Contributing to Revue

Thank you for your interest in contributing to Revue! We welcome contributions of all kinds.

---

## Development Setup

### Prerequisites

```bash
# Rust (1.87+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Optional: typos (spell checker)
cargo install typos-cli
```

### Project Setup

```bash
git clone https://github.com/hawk90/revue.git
cd revue

# Verify build
cargo build
cargo test
```

---

## Git Workflow

We use a **simplified trunk-based flow** (no develop/release branches):

```
main (protected, always deployable)
  │
  └── feature branches → PR → squash merge → main
```

### Branch Strategy

```
main (protected)
  │
  ├── feat/*      New features
  ├── fix/*       Bug fixes
  ├── docs/*      Documentation
  ├── refactor/*  Refactoring
  ├── perf/*      Performance improvements
  ├── test/*      Tests
  ├── examples/*  Example applications
  └── chore/*     Configuration, dependencies
```

### Branch Naming

```bash
# Format
<type>/<short-description>

# Examples
feat/add-button-component
fix/memory-leak-in-parser
docs/update-api-reference
refactor/simplify-layout-engine
```

### Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/).

```bash
# Format
<type>(<scope>): <description>

# Examples
feat(widget): add Button component
fix(parser): resolve memory leak in CSS parser
docs(readme): add installation guide
refactor(layout): simplify flexbox calculation
perf(render): optimize diff algorithm
test(button): add click event tests
chore(deps): update crossterm to 0.28

# Breaking changes - add ! after type
feat!(api): change View trait signature
```

**Types:**

| Type | Description |
|------|-------------|
| `feat` | A new feature |
| `fix` | A bug fix |
| `docs` | Documentation only changes |
| `style` | Code formatting (no functional changes) |
| `refactor` | Code refactoring (no functional changes) |
| `perf` | Performance improvements |
| `test` | Adding or updating tests |
| `build` | Build system changes |
| `ci` | CI configuration changes |
| `chore` | Other changes |
| `revert` | Revert a commit |

### PR Workflow

1. Create a feature branch from `main`
2. Make changes, commit with conventional commits
3. Push and open a PR
4. CI runs automatically (clippy, fmt, tests, security audit)
5. Get review and approval
6. Squash and merge

### Merge Strategy

- **Squash and merge** is used to maintain a clean history
- The PR title becomes the final commit message
- Delete branch after merge

---

## Code Quality

### Pre-commit Checks

Run these before committing:

```bash
# Format code
cargo fmt

# Lint
cargo clippy --all-features -- -D warnings

# Run tests
cargo test --all-features

# Optional: check typos
typos
```

### CI Checks

PRs must pass all CI checks:

- `cargo fmt --check`
- `cargo clippy --all-features -- -D warnings`
- `cargo test --all-features`
- `cargo-deny` (license and security)
- `typos` (spell check)

### Code Guidelines

#### 1. Document Public APIs

All public items must have documentation:

```rust
/// Creates a new button with the given label.
///
/// # Examples
///
/// ```
/// use revue::widget::Button;
/// let button = Button::new("Click me");
/// ```
///
/// # Arguments
///
/// * `label` - The text to display on the button
///
/// # Returns
///
/// A new Button widget
pub fn new(label: &str) -> Self { ... }
```

#### 2. Use `thiserror` for Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid syntax at line {line}")]
    InvalidSyntax { line: usize },

    #[error("unexpected token: {0}")]
    UnexpectedToken(String),
}
```

#### 3. Write Tests in the Same File

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_new() {
        let button = Button::new("Test");
        assert_eq!(button.label, "Test");
    }
}
```

#### 4. Use `Result` for Fallible Operations

```rust
pub fn parse_css(input: &str) -> Result<StyleSheet, ParseError> {
    // ...
}
```

#### 5. Prefer Builder Pattern for Widgets

```rust
pub struct Button {
    label: String,
    on_click: Option<Callback>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, callback: Callback) -> Self {
        self.on_click = Some(callback);
        self
    }
}
```

#### 6. Module Documentation

Each module should have a top-level doc comment:

```rust
//! Button widget for clickable actions
//!
//! This module provides the Button widget and related types.
//!
//! # Examples
//!
//! ```rust,ignore
//! use revue::widget::Button;
//!
//! let button = Button::new("Click me")
//!     .on_click(|| println!("Clicked!"));
//! ```
```

---

## Testing Guidelines

### Test Coverage

- Aim for >80% code coverage
- Write unit tests for all public functions
- Write integration tests for complex interactions
- Use `#[test]` for simple tests
- Use `#[tokio::test]` for async tests

### Test Organization

```rust
// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() { }
}

// Integration tests (in tests/ directory)
// #[cfg(test)]
// mod integration_tests {
//     use super::*;
//
//     #[test]
//     fn test_full_workflow() { }
// }
```

### Property-Based Testing

For functions with simple input/output relationships, consider property-based testing:

```rust
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_roundtrip(input in any::<String>()) {
            let parsed = parse(&input);
            prop_assert_eq!(input, parsed);
        }
    }
}
```

---

## Documentation Guidelines

### Rust Doc Comments

- Use `///` for item documentation
- Use `//!` for module-level documentation
- Include examples for all public APIs
- Use proper markdown formatting

### Markdown Documentation

- Use tables for comparisons
- Use code blocks for examples
- Include syntax highlighting
- Add badges and links where appropriate

### Examples

All examples in documentation should:

- Compile and run correctly
- Demonstrate real usage
- Include necessary imports
- Show expected output

---

## Project Structure

```
revue/
├── src/
│   ├── app/           # Application lifecycle
│   ├── dom/           # Virtual DOM and CSS cascade
│   ├── event/         # Event handling
│   ├── layout/        # Layout engine
│   ├── reactive/      # Signal/Computed/Effect
│   ├── render/        # Terminal rendering
│   ├── style/         # CSS parsing and theming
│   ├── widget/        # Widget implementations
│   ├── patterns/      # Reusable patterns
│   ├── utils/         # Utility functions
│   └── lib.rs         # Library root
├── docs/              # Documentation
├── examples/          # Example applications
├── tests/             # Integration tests
└── Cargo.toml         # Package manifest
```

---

## PR Checklist

Before submitting a PR:

- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy --all-features` with no warnings
- [ ] Run `cargo test --all-features` and all tests pass
- [ ] Update documentation for public API changes
- [ ] Add tests for new functionality
- [ ] Note breaking changes in PR description (use `feat!:` or `BREAKING CHANGE:`)

---

## Release Process

Releases are fully automated via [Release Please](https://github.com/googleapis/release-please):

1. PR is merged to `main`
2. Release Please analyzes commits
3. Release PR is automatically created (version bump + CHANGELOG)
4. When Release PR is merged:
   - GitHub Release created
   - Published to crates.io

Version is determined automatically based on Conventional Commits:
- `fix:` → patch (0.1.0 → 0.1.1)
- `feat:` → minor (0.1.0 → 0.2.0)
- `feat!:` or `BREAKING CHANGE:` → major (0.1.0 → 1.0.0)

---

## Getting Help

- **[GitHub Issues](https://github.com/hawk90/revue/issues)** - Bug reports and feature requests
- **[GitHub Discussions](https://github.com/hawk90/revue/discussions)** - Questions and discussions
- **[Documentation](https://docs.rs/revue)** - API reference

Look for issues labeled `good first issue` to get started!

---

## Code of Conduct

Be respectful, inclusive, and constructive. We're all here to build something great together.

---

## License

By contributing to Revue, you agree that your contributions will be licensed under the [MIT License](LICENSE).
