# Contributing to Revue

Thank you for your interest in contributing to Revue!

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
- [ ] Note breaking changes in PR description (use `feat!:` or `BREAKING CHANGE:`)

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

## Need Help?

- Open an issue on [GitHub Issues](https://github.com/hawk90/revue/issues)
- Look for issues labeled `good first issue` to get started
