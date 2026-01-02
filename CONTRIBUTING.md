# Contributing to Revue

Thank you for your interest in contributing to revue!

## Development Setup

### Prerequisites

```bash
# Rust (1.75+)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Lefthook (Git hooks)
brew install lefthook  # macOS
# or
cargo install lefthook
```

### Project Setup

```bash
git clone https://github.com/user/revue.git
cd revue

# Enable Git hooks
lefthook install

# Verify build
cargo build
cargo test
```

## Git Workflow

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
feat(components): add Button component
fix(parser): resolve memory leak in CSS parser
docs(readme): add installation guide
refactor(layout): simplify flexbox calculation
perf(render): optimize diff algorithm
test(button): add click event tests
chore(deps): update crossterm to 0.28
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

### PR Titles

PR titles must also follow Conventional Commits format:

```
feat: add Button component
fix: resolve memory leak in CSS parser
```

### Merge Strategy

- **Squash and merge** is used to maintain a clean history
- The PR title becomes the final commit message

## Code Style

### Rust

```bash
# Formatting
cargo fmt

# Linting
cargo clippy --all-features -- -D warnings

# Testing
cargo test --all-features
```

### Guidelines

1. **Document public APIs**
   ```rust
   /// Creates a new button with the given label.
   ///
   /// # Examples
   ///
   /// ```
   /// let button = Button::new("Click me");
   /// ```
   pub fn new(label: &str) -> Self { ... }
   ```

2. **Use `thiserror` for error handling**
   ```rust
   #[derive(Debug, thiserror::Error)]
   pub enum ParseError {
       #[error("invalid syntax at line {line}")]
       InvalidSyntax { line: usize },
   }
   ```

3. **Write tests in the same file**
   ```rust
   #[cfg(test)]
   mod tests {
       use super::*;

       #[test]
       fn test_button_new() { ... }
   }
   ```

## PR Checklist

Before submitting a PR:

- [ ] Run `cargo fmt`
- [ ] Run `cargo clippy --all-features` with no warnings
- [ ] Run `cargo test --all-features` and all tests pass
- [ ] Update documentation for public API changes
- [ ] Note breaking changes in PR description

## Release Process

Releases are fully automated:

1. PR is merged to `main`
2. [Release Please](https://github.com/googleapis/release-please) analyzes commits
3. Release PR is automatically created (version bump + CHANGELOG)
4. When Release PR is merged, automatic release:
   - GitHub Release created
   - Binaries built and uploaded
   - Published to crates.io

Version is determined automatically based on Conventional Commits:
- `feat:` → minor (0.1.0 → 0.2.0)
- `fix:` → patch (0.1.0 → 0.1.1)
- `feat!:` or `BREAKING CHANGE:` → major (0.1.0 → 1.0.0)

## Need Help?

- Open an issue on [GitHub Issues](https://github.com/user/revue/issues)
- Look for issues labeled `good first issue` to get started
