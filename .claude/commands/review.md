# Code Review Pipeline

Run all experts in parallel to review code changes.

## Pipeline Stages

### Stage 1: Core Review (Parallel)
Launch these 4 agents in parallel:

1. **Uncle Bob** - SOLID, Clean Code, function length, naming
2. **Linus Torvalds** - Performance, memory, allocations
3. **Martin Fowler** - Code smells, refactoring opportunities
4. **Rust Review** - Rust idioms, ownership, clippy-style checks

### Stage 2: Specialized Review (Parallel)
After Stage 1, launch these in parallel:

1. **API Design** - Public API ergonomics
2. **Security** - Input validation, safe patterns
3. **TUI UX** - Keyboard navigation, accessibility

### Stage 3: Architecture
1. **Staff Engineer** - Overall design, trade-offs

## Output Format

Compile all results into a single summary:

```markdown
# Review Summary

**Files:** [list of files reviewed]
**Date:** [date]

## Critical Issues (Fix Now)
- [ ] [issue from any expert]

## Warnings
- [ ] [warning from any expert]

## Suggestions
- [ ] [suggestion from any expert]

## Expert Notes
### Uncle Bob
[summary]

### Linus
[summary]

### Fowler
[summary]

### Rust
[summary]
```

Save results to `REVIEW.md`

## Target
$ARGUMENTS
