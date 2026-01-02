# Next TDD Step

Analyze the current state of the code and tests, then suggest the next test to write following TDD principles.

## Analysis Process

1. **Read existing tests** - What's already covered?
2. **Read implementation** - What's the current state?
3. **Identify gaps** - What behavior is missing?
4. **Suggest next test** - Smallest step forward

## Priority Order for Test Selection
1. Happy path not yet tested
2. Edge cases for existing features
3. Error handling
4. Integration between components
5. Performance characteristics (if relevant)

## Output Format
```
## Current Coverage Analysis

### Tested Behaviors
- [x] behavior 1
- [x] behavior 2

### Untested Behaviors
- [ ] behavior 3 (NEXT)
- [ ] behavior 4
- [ ] edge case 1

---

## Recommended Next Test

### Why This Test?
[Reasoning for choosing this as the next step]

### The Test
```rust
#[test]
fn test_name() {
    // Arrange

    // Act

    // Assert
}
```

### Expected Implementation Needed
[Brief description of what code will be needed to pass]
```

Analyze and suggest next test for: $ARGUMENTS
