# Google Engineering Standards - Code Review Culture

Apply Google's engineering practices and code review standards from their public Engineering Practices documentation (google.github.io/eng-practices).

This is not a persona - it's a checklist based on Google's documented standards.

## Google Code Review Principles

### The Standard of Code Review
> "The primary purpose of code review is to ensure the overall code health of Google's codebase is improving over time."

### What to Look For
1. **Design** - Well-designed and appropriate for the system?
2. **Functionality** - Behaves as intended? Good for users?
3. **Complexity** - Could it be simpler? Will others understand it?
4. **Tests** - Correct, sensible, useful tests?
5. **Naming** - Clear names for everything?
6. **Comments** - Clear and useful?
7. **Style** - Follows style guide?
8. **Documentation** - Updated relevant docs?

### CL (Changelist) Size
- **Small CLs are better** - Easier to review, less risk
- Ideal: < 200 lines changed
- If larger, can it be split?

### Speed of Reviews
- Review within one business day
- Don't block on perfection - "LGTM with nits"

## Google-Specific Practices

### Readability
- Code should be readable by someone unfamiliar with it
- "Code is read far more often than it is written"
- Self-documenting code preferred

### Error Handling
- Never ignore errors silently
- Errors should be actionable
- Include context in error messages

### Logging & Observability
- Structured logging
- Appropriate log levels
- Metrics for monitoring

### Testing at Google
```
┌─────────────────────────────────┐
│         E2E Tests (10%)         │  Slow, flaky, expensive
├─────────────────────────────────┤
│     Integration Tests (20%)     │  Service boundaries
├─────────────────────────────────┤
│       Unit Tests (70%)          │  Fast, isolated, numerous
└─────────────────────────────────┘
```

### Code Health
- "Leave the codebase healthier than you found it"
- Boy Scout Rule applied at scale
- Technical debt is tracked and addressed

## Review Comments Style

### Good Comments
- Explain "why" not just "what"
- Offer alternatives, not just criticism
- Distinguish between: must-fix, should-fix, nit, optional

### Comment Prefixes (Google Style)
- `Nit:` - Minor style issue, optional
- `Optional:` - Suggestion, take it or leave it
- `FYI:` - Information, no action needed
- `TODO:` - Should be addressed, can be follow-up
- (no prefix) - Must be addressed before approval

## Output Format
```
## Google Standards Review

### CL Summary
- Size: [Small/Medium/Large] - [recommendation if too large]
- Risk: [Low/Medium/High]

### Design Review
[Is the overall approach sound?]

### Code Health Check
| Aspect | Status | Notes |
|--------|--------|-------|
| Complexity | ✓/⚠/✗ | |
| Readability | ✓/⚠/✗ | |
| Test Coverage | ✓/⚠/✗ | |
| Error Handling | ✓/⚠/✗ | |
| Documentation | ✓/⚠/✗ | |

### Review Comments
1. [Location] - [Severity: Must/Should/Nit] - [Comment]
2. ...

### Verdict
- [ ] LGTM (Looks Good To Me)
- [ ] LGTM with nits
- [ ] Needs changes
- [ ] Needs discussion
```

Review with Google standards: $ARGUMENTS
