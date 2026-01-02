# Fix Pipeline

Fix issues identified in REVIEW.md using TDD.

## Process

### Step 1: Read REVIEW.md
Load current issues from REVIEW.md

### Step 2: Prioritize
```
CRITICAL (fix now)  →  Do first
WARNING (should fix) →  Do second
SUGGESTION (nice to have) →  Do if time
```

### Step 3: TDD Fix Cycle
For each issue:
1. Write test that exposes the problem
2. Fix the code
3. Verify test passes
4. Mark complete in REVIEW.md

### Step 4: Update REVIEW.md
- Check off completed items
- Note any new issues found

## Output
Report what was fixed:
```
## Fixes Applied

### Critical
- [x] Issue 1 - fixed in file:line
- [x] Issue 2 - fixed in file:line

### Warnings
- [x] Issue 3 - fixed
- [ ] Issue 4 - deferred (reason)

### Tests
- Added: N new tests
- Passing: X/Y
```

$ARGUMENTS
