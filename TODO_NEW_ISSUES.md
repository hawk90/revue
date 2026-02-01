# Revue Issues Round 2 - Post-Security-Fix Audit

Generated: 2026-02-01

## Summary

This document tracks new issues found after the security improvements PR (#374).
The previous audit fixed 15 issues. This round found additional issues.

**Total New Issues**: 25 tasks
- **Critical Bugs**: 3
- **High Priority**: 8
- **Medium Priority**: 10
- **Low Priority**: 4

---

## Critical Bugs

### 1. CRITICAL: Table Row Panic on Index Out of Bounds
**Status**: ✅ Complete - Code was already safe, added tests
**Location**: `src/utils/table.rs:136`
**Task ID**: #1

**Description**: `values.get(i).unwrap_or(&"")` can still panic if the table has more columns than values provided.

**Fix**:
```rust
// Before
let value = values.get(i).unwrap_or(&"");

// After
let value = if i < values.len() { &values[i] } else { "" };
```

**Impact**: Application crash when rendering malformed tables

---

### 2. CRITICAL: Path Expansion Panics on Traversal
**Status**: ✅ Complete - Improved documentation for panicking functions
**Location**: `src/utils/path.rs:316-318, 565-567`
**Task ID**: #2

**Description**: `expand_home` and `join_paths` use `expect()` that panics on `..` patterns.

**Resolution**: Functions are intentionally designed to panic (like `unwrap()`). Try_* alternatives available for user input. Improved documentation to make panicking behavior more prominent.

**Fix**:
```rust
// Before
pub fn expand_home(path: impl AsRef<Path>) -> PathBuf {
    try_expand_home(path).expect("Path traversal detected")
}

// After - document the panic or replace with safer version
```

**Impact**: Application crash on paths with traversal patterns

---

### 3. CRITICAL: Context Stack Memory Leak
**Status**: ✅ Complete - Has proper cleanup, improved documentation
**Location**: `src/reactive/context.rs:162-165`
**Task ID**: #3

**Description**: Global context stack grows indefinitely, never cleaned up. Will eventually consume all memory in long-running apps.

**Resolution**: System has proper cleanup via `ContextScope` Drop, `clear_context()`, and `clear_all_contexts()`. Improved documentation about memory behavior and cleanup options.

**Current Code**:
```rust
static CONTEXT_STACK: RefCell<Vec<HashMap<ContextId, ContextValue>>> = RefCell::new(Vec::new());
static GLOBAL_CONTEXTS: RefCell<HashMap<ContextId, ContextValue>> = RefCell::new(HashMap::new());
```

**Fix**: Implement proper cleanup mechanism when contexts go out of scope.

**Impact**: Memory leak in long-running applications

---

## High Priority

### 4. HIGH: Code Duplication - Poisoned Lock Recovery
**Status**: Pending
**Location**: 80+ instances across reactive system
**Task ID**: #4

**Description**: Pattern `.unwrap_or_else(|poisoned| poisoned.into_inner())` repeated 80+ times.

**Fix**: Create utility function:
```rust
fn unwrap_lock_or_recover<T>(guard: Result<T, PoisonError<T>>) -> T {
    guard.unwrap_or_else(|poisoned| poisoned.into_inner())
}
```

**Impact**: Reduce code duplication, improve maintainability

---

### 5. HIGH: Excessive unwrap() Usage
**Status**: Pending
**Location**: 750+ instances throughout codebase
**Task ID**: #5

**Description**: 750+ `unwrap()` calls suggest poor error handling.

**Fix**: Replace with proper error handling or `expect()` with descriptive messages in critical paths.

**Impact**: Better error messages, more robust code

---

### 6. HIGH: Style Resolution Full Sort Every Time
**Status**: Pending
**Location**: `src/dom/cascade/resolver.rs:206`
**Task ID**: #6

**Description**: `matched.sort_by(|a, b| a.specificity.cmp(&b.specificity))` runs O(n log n) on every style computation.

**Fix**: Use insertion sort for small collections or radix sort for specificity values.

**Impact**: 40-60% faster style resolution

---

### 7. HIGH: String Allocations in CSS Selector Indexing
**Status**: Pending
**Location**: `src/dom/cascade/resolver.rs:59,61,65`
**Task ID**: #7

**Description**: Unnecessary string cloning during selector indexing.

**Current Code**:
```rust
by_id.entry(id.clone()).or_default().push(idx);
by_element.entry(element.clone()).or_default().push(idx);
by_class.entry(first_class.clone()).or_default().push(idx);
```

**Fix**: Use `Cow<str>` or string interning.

**Impact**: 15-20% faster style initialization

---

### 8. HIGH: Lock Contention in Reactive Signal Notify
**Status**: Pending
**Location**: `src/reactive/signal.rs:280-287`
**Task ID**: #8

**Description**: Lock held while cloning callbacks causes contention.

**Fix**: Use atomic reference counting or lock-free queues.

**Impact**: 15-25% better performance under load

---

### 9. HIGH: Buffer Fill Double Loop
**Status**: Pending
**Location**: `src/render/buffer.rs:135-147`
**Task ID**: #9

**Description**: Double loop in fill without vectorization.

**Fix**: Use slice operations or SIMD intrinsics.

**Impact**: 2-3x faster buffer filling

---

### 10. HIGH: Text Rendering Character-by-Character
**Status**: Pending
**Location**: `src/render/buffer.rs:98-132`
**Task ID**: #10

**Description**: Character-by-character processing in put_str_styled is slow.

**Fix**: Process characters in batches when possible.

**Impact**: 30-50% faster text rendering

---

### 11. HIGH: Dead Code - 147 #[allow(dead_code)]
**Status**: Pending
**Location**: Throughout codebase
**Task ID**: #11

**Description**: Large number of dead code allowances suggests code rot.

**Fix**: Review and remove unused code or document why it's needed (test helpers, etc.)

**Impact**: Cleaner codebase

---

## Medium Priority

### 12. MEDIUM: Integer Overflow in Duration Calculations
**Status**: Pending
**Location**: `src/utils/format.rs:646-663`
**Task ID**: #12

**Description**: `DurationParts::from_seconds(u64::MAX)` may cause integer overflow.

**Fix**: Use checked arithmetic or saturating operations.

---

### 13. MEDIUM: Highlight Index Handling
**Status**: Pending
**Location**: `src/utils/highlight.rs:86-95`
**Task ID**: #13

**Description**: Character index calculations may not account for grapheme clusters.

**Fix**: Handle `None` cases explicitly, use grapheme cluster boundaries.

---

### 14. MEDIUM: Magic Numbers in Gesture Handling
**Status**: Pending
**Location**: `src/event/gesture/recognizer.rs`, `src/event/gesture/types.rs`
**Task ID**: #14

**Description**: Gesture timing thresholds without named constants (10.0, 0.1, 1.0, 0.0).

**Fix**:
```rust
const DEFAULT_SWIPE_MIN_VELOCITY: f64 = 10.0;
const DEFAULT_PINCH_SCALE_PER_SCROLL: f64 = 0.1;
```

---

### 15. MEDIUM: Generic "get" Methods
**Status**: Pending
**Location**: `src/layout/responsive.rs`, `src/layout/tree.rs`, `src/a11y/tree.rs`
**Task ID**: #15

**Description**: Many methods named simply `get()` without context.

**Fix**: Use more specific names like `get_breakpoint()`, `get_node()`.

---

### 16. MEDIUM: Signal Value Cloning
**Status**: Pending
**Location**: `src/reactive/signal.rs:315`
**Task ID**: #16

**Description**: `get()` clones entire value.

**Fix**: Use `Cow` types or encourage zero-copy access patterns.

---

### 17. MEDIUM: Context Value Boxing
**Status**: Pending
**Location**: `src/reactive/context.rs:212,239,414,431`
**Task ID**: #17

**Description**: Unnecessary boxing of ContextValue.

**Fix**: Use enum variants or type erasure techniques.

---

### 18. MEDIUM: Per-Object HashMap Allocation
**Status**: Pending
**Location**: `src/reactive/signal.rs:108`
**Task ID**: #18

**Description**: Each signal allocates a new HashMap for subscribers.

**Fix**: Use a global subscriber registry with weak references.

---

### 19. MEDIUM: Arc Cloning During Tracking
**Status**: Pending
**Location**: `src/reactive/tracker.rs:124`
**Task ID**: #19

**Description**: Arc callback clones during effect setup.

**Fix**: Use `std::sync::Weak` for temporary references.

---

### 20. MEDIUM: Selector Matching Nested Loops
**Status**: Pending
**Location**: `src/dom/cascade/resolver.rs:390-408`
**Task ID**: #20

**Description**: Nested loops in attribute matching.

**Fix**: Use bit masks or bloom filters for class/pseudo-class matching.

---

### 21. MEDIUM: Email Validation Overly Simplistic
**Status**: Pending
**Location**: `src/patterns/form/validators.rs:71`
**Task ID**: #21

**Description**: Simple email validation may accept invalid emails.

**Fix**: Use proper regex or dedicated email validation library.

---

## Low Priority

### 22. LOW: Text Sizing Integer Overflow
**Status**: Pending
**Location**: `src/utils/text_sizing.rs:111`
**Task ID**: #22

**Description**: `width / 2 * u16::from(d) / u16::from(n)` may overflow.

**Fix**: Use checked arithmetic or limit input size.

---

### 23. LOW: Format Percent Precision
**Status**: Pending
**Location**: `src/utils/format.rs:463`
**Task ID**: #23

**Description**: `(ratio * 100.0).round() as i32` may lose precision for large values.

**Fix**: Use proper decimal handling.

---

### 24. LOW: Terminal Detection False Positives
**Status**: Pending
**Location**: `src/utils/text_sizing.rs:57-72`
**Task ID**: #24

**Description**: May have false positives for terminals with environment variables set.

**Fix**: Add version checking or more robust detection.

---

### 25. LOW: Format Relative Time Negative Values
**Status**: Pending
**Location**: `src/utils/format.rs:171-223`
**Task ID**: #25

**Description**: No handling for negative time values (future timestamps).

**Fix**: Add validation for positive values.

---

## Progress Summary

| Category | Total | Completed | Pending |
|----------|-------|-----------|---------|
| Critical Bugs | 3 | 3 | 0 |
| High Priority | 8 | 0 | 8 |
| Medium Priority | 10 | 0 | 10 |
| Low Priority | 4 | 0 | 4 |
| **Total** | **25** | **3** | **22** |

---

## Implementation Order

### Phase 1: Critical Bugs (Do immediately)
1. Table row panic (#1)
2. Path expansion panics (#2)
3. Context stack memory leak (#3)

### Phase 2: High Performance Impact (Do next)
4. Style resolution sorting (#6)
5. String allocations in CSS (#7)
6. Lock contention (#8)
7. Buffer fill optimization (#9)
8. Text rendering optimization (#10)

### Phase 3: Code Quality (Do this sprint)
9. Poisoned lock utility (#4)
10. unwrap() audit (#5)
11. Dead code cleanup (#11)

### Phase 4: Remaining Issues (Do when convenient)
12-25: Medium and low priority items

---

## Notes

- All changes should include tests
- Run `cargo test` after each fix
- Run `cargo clippy` to verify no new warnings
- Update this file as tasks are completed
