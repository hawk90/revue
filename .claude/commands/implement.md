# Implementation Pipeline (TDD)

Complete implementation workflow using TDD cycle.

## Pipeline Flow

```
┌─────────────────────────────────────────────────────────┐
│  1. PLAN                                                │
│     └─ Staff Engineer: Design approach                  │
├─────────────────────────────────────────────────────────┤
│  2. TDD CYCLE (repeat)                                  │
│     ├─ RED: Write failing test                          │
│     ├─ GREEN: Minimal code to pass                      │
│     └─ REFACTOR: Clean up                               │
├─────────────────────────────────────────────────────────┤
│  3. REVIEW (parallel)                                   │
│     ├─ Uncle Bob: Clean code                            │
│     ├─ Linus: Performance                               │
│     ├─ Fowler: Refactoring                              │
│     └─ Rust Review: Idioms                              │
├─────────────────────────────────────────────────────────┤
│  4. FIX: Address critical issues                        │
├─────────────────────────────────────────────────────────┤
│  5. VERIFY: Run tests, check coverage                   │
└─────────────────────────────────────────────────────────┘
```

## Instructions

### Step 1: Plan
- Analyze the requirement
- Identify affected files
- Design minimal solution (YAGNI)

### Step 2: TDD Loop
For each piece of functionality:
1. Write one failing test
2. Write minimal code to pass
3. Refactor if needed
4. Repeat

### Step 3: Review
After implementation complete:
- Run /review on changed files
- Fix any critical issues

### Step 4: Verify
- Run `cargo test`
- Run `cargo clippy`
- Ensure all tests pass

## Target Feature
$ARGUMENTS
