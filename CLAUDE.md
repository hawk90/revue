# Claude Code Instructions for Revue

## Commit Message Guidelines

### Release-Please Triggers

**Triggers version bump (avoid for non-functional changes):**
- `fix:` → patch version bump
- `feat:` → minor version bump
- `feat!:` or `BREAKING CHANGE:` → major version bump

**Does NOT trigger version bump (use for non-functional changes):**
- `docs:` → documentation only
- `chore:` → maintenance tasks
- `ci:` → CI/CD configuration
- `test:` → test changes only
- `style:` → code formatting
- `refactor:` → code refactoring (no behavior change)

### Examples

```bash
# Documentation changes - NO version bump
docs: update README with new examples
docs(api): add JSDoc comments to public functions

# CI/CD changes - NO version bump
ci: add concurrency to release-please workflow
ci: update GitHub Actions versions

# Bug fixes - PATCH version bump
fix(parser): resolve memory leak in CSS parser
fix: correct off-by-one error in layout

# New features - MINOR version bump
feat(widget): add Dropdown component
feat: implement keyboard navigation
```

### Branch Naming

Match commit prefix with branch prefix:
- Documentation → `docs/` branch → `docs:` commit
- Bug fix → `fix/` branch → `fix:` commit
- Feature → `feat/` branch → `feat:` commit
- CI changes → `ci/` branch → `ci:` commit

## Testing

### Global State Tests

Use `serial_test` crate for tests that access global state:

```rust
use serial_test::serial;

#[test]
#[serial]
fn test_with_global_state() {
    // Test code accessing global state
}
```

## Project Structure

- `src/` - Main library code
- `tests/` - Integration tests
- `benches/` - Benchmarks
- `examples/` - Example applications
- `docs/` - Documentation

## Commands

```bash
cargo build              # Build
cargo test               # Run tests
cargo clippy --all-features -- -D warnings  # Lint
cargo fmt                # Format
typos                    # Check typos
```
