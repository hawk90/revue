# Test Extraction Summary for Utils Modules

This document summarizes the test extraction work needed for the remaining utils source files.

## Overview

Based on the analysis, the following modules have inline `#[cfg(test)]` modules that need to be extracted to separate test files in the `tests/` directory.

## Files Processed

### Completed

1. **src/utils/accessibility/announcement.rs** → **tests/accessibility_announcement_tests.rs** ✅
   - Tests extracted: 21 tests
   - Tests cover: Priority enum, Announcement::polite(), Announcement::assertive(), cloning, public fields, timestamp ordering
   - Source file updated to remove test module

## Remaining Files to Process

### Gradient Modules (7 files, ~320 tests)

| Source File | Target Test File | Test Count | Notes |
|-------------|------------------|------------|-------|
| `src/utils/gradient/mod.rs` | `tests/gradient_mod_tests.rs` | 30 | Tests for ColorStop, Gradient creation, interpolation, spread modes, LinearGradient, RadialGradient, presets |
| `src/utils/gradient/types.rs` | `tests/gradient_types_tests.rs` | 90+ | Tests for ColorStop, InterpolationMode, SpreadMode, GradientDirection enums |
| `src/utils/gradient/linear.rs` | `tests/gradient_linear_tests.rs` | 30+ | Tests for LinearGradient construction, at() method, colors_2d(), directions |
| `src/utils/gradient/radial.rs` | `tests/gradient_radial_tests.rs` | 30+ | Tests for RadialGradient construction, at() method, colors_2d(), center/radius |
| `src/utils/gradient/core.rs` | `tests/gradient_core_tests.rs` | 50+ | Tests for Gradient construction, builder methods, stop management, at(), colors(), reversed() |
| `src/utils/gradient/interpolation.rs` | `tests/gradient_interpolation_tests.rs` | 20+ | Tests for lerp_rgb, lerp_hsl, lerp_hsl_long, interpolate() |
| `src/utils/gradient/presets.rs` | `tests/gradient_presets_tests.rs` | 40+ | Tests for all preset gradients (rainbow, sunset, ocean, etc.) |

**Note**: Gradient tests access private `stops` field in Gradient - these tests should be removed.

### Accessibility Modules (8 files, ~380 tests)

| Source File | Target Test File | Test Count | Notes |
|---------------------|------------------------|------------|-------|
| `src/utils/accessibility/state.rs` | `tests/accessibility_state_tests.rs` | 40+ | Tests for AccessibleState builders, new(), default(), clone |
| `src/utils/accessibility/node.rs` | `tests/accessibility_node_tests.rs` | 60+ | Tests for AccessibleNode construction, builders, accessible_name(), is_focusable(), describe() |
| `src/utils/accessibility/manager.rs` | `tests/accessibility_manager_tests.rs` | 80+ | Tests for AccessibilityManager, SharedAccessibility, helper functions |
| `src/utils/accessibility/roles.rs` | `tests/accessibility_roles_tests.rs` | 60+ | Tests for Role enum (name(), is_interactive(), is_landmark()) |
| `src/utils/accessibility/aria.rs` | `tests/accessibility_aria_tests.rs` | 80+ | Tests for AriaAttribute, LiveRegion, AriaBuilder |
| `src/utils/accessibility/mod.rs` | `tests/accessibility_mod_tests.rs` | 30+ | Tests for Role, AccessibleNode, AccessibleState, manager integration |
| `src/utils/accessibility_signal.rs` | `tests/accessibility_signal_tests.rs` | 20+ | Tests for global accessibility functions, uses `serial_test` |

### Syntax Modules (2 files, ~130 tests)

| Source File | Target Test File | Test Count | Notes |
|-------------------|----------------------|------------|-------|
| `src/utils/syntax/mod.rs` | `tests/syntax_mod_tests.rs` | 90+ | Tests for Language, SyntaxHighlighter, Token, TokenType, SyntaxTheme |
| `src/utils/syntax/helpers.rs` | `tests/syntax_helpers_tests.rs` | 40+ | Tests for highlight() and highlight_line() helper functions |

### Path Modules (2 files, ~25 tests)

| Source File | Target Test File | Test Count | Notes |
|----------------|---------------------|------------|-------|
| `src/utils/path/mod.rs` | Tests are in submodules | - | Only re-exports, no inline tests |
| `src/utils/path/format.rs` | `tests/path_format_tests.rs` | 25+ | Tests for shorten_path(), abbreviate_path(), abbreviate_path_keep(), relative_to() |

### Selection Modules (2 files, ~100 tests)

| Source File | Target Test File | Test Count | Notes |
|---------------------|------------------------|------------|-------|
| `src/utils/selection/mod.rs` | `tests/selection_mod_tests.rs` | 70+ | Tests for Selection, SectionedSelection (next/prev, viewport, collapse) |
| `src/utils/selection/helper.rs` | `tests/selection_helper_tests.rs` | 30+ | Tests for wrap_next(), wrap_prev() helper functions |

### Other Modules (4 files, ~65 tests)

| Source File | Target Test File | Test Count | Notes |
|-----------------------|------------------------|------------|-------|
| `src/utils/sort.rs` | `tests/sort_tests.rs` | 20+ | Tests for natural_cmp(), natural_sort(), NaturalKey |
| `src/utils/layout.rs` | `tests/layout_tests.rs` | 10+ | Tests for BoxLayout (new, fill, fit, centered, calculations) |
| `src/utils/keymap.rs` | `tests/keymap_tests.rs` | 20+ | Tests for Mode, KeyChord, KeymapConfig, parse_key_binding(), vim_preset(), emacs_preset() |
| `src/utils/accessibility_signal.rs` | `tests/accessibility_signal_tests.rs` | 15+ | Tests for global accessibility functions, uses `serial_test` |

### Modules Not Yet Read (estimated ~200+ tests)

**Figlet modules:**
- `src/utils/figlet/mod.rs`
- `src/utils/figlet/types.rs`
- `src/utils/figlet/banner.rs`
- `src/utils/figlet/slant.rs`
- `src/utils/figlet/block.rs`
- `src/utils/figlet/small.rs`
- `src/utils/figlet/api.rs`
- `src/utils/figlet/mini.rs`

**Undo modules:**
- `src/utils/undo/mod.rs`
- `src/utils/undo/core.rs`
- `src/utils/undo/types.rs`
- `src/utils/undo/query.rs`
- `src/utils/undo/undo_redo.rs`
- `src/utils/undo/merge.rs`
- `src/utils/undo/group.rs`

**Textbuffer module:**
- `src/utils/textbuffer/mod.rs`

**Border module:**
- `src/utils/border/mod.rs`

## Test Extraction Pattern

For each file, follow this pattern:

1. **Read the source file** to identify the `#[cfg(test)]` module
2. **Create a new test file** in `tests/` directory
   - Name format: `{module}_tests.rs` (e.g., `accessibility_announcement_tests.rs`)
3. **Write the tests** with proper imports:
   ```rust
   //! Tests for {module} module
   //!
   //! Extracted from src/{path}/{file}.rs

   use revue::{...};

   #[test]
   fn test_name() {
       // test code
   }
   ```
4. **Remove the `#[cfg(test)]` module** from the source file
5. **Handle private field access** by removing tests that access private struct fields

## Special Considerations

### Private Fields in Gradient

The `Gradient` struct has a private `stops` field. Tests that access this field directly (like `test_gradient_stops()`) should be **removed** as they cannot be tested from external test files.

### Global State Tests

Accessibility tests in `accessibility/mod.rs` and `accessibility_signal.rs` use `#[serial]` attribute from the `serial_test` crate. These must retain the `#[serial]` attribute in the extracted test files.

### Test Dependencies

Some tests may require additional imports or setup. Ensure all required imports are included in the extracted test files.

## Estimated Total Tests

Based on the analysis:
- **Gradient modules**: ~320 tests
- **Accessibility modules**: ~380 tests
- **Syntax modules**: ~130 tests
- **Path modules**: ~25 tests
- **Selection modules**: ~100 tests
- **Other modules**: ~65 tests
- **Figlet/Undo/Textbuffer/Border**: ~200+ tests

**Total**: ~1,220 tests to extract

## Next Steps

1. Process gradient module tests (largest group)
2. Process accessibility module tests (uses serial_test)
3. Process syntax, path, selection tests
4. Process remaining utility tests
5. Update all source files to remove test modules
6. Verify all tests compile and pass

## Files to Update in src/

After test extraction, remove the `#[cfg(test)]` blocks from these source files:

**Gradient:**
- `src/utils/gradient/mod.rs`
- `src/utils/gradient/types.rs`
- `src/utils/gradient/linear.rs`
- `src/utils/gradient/radial.rs`
- `src/utils/gradient/core.rs`
- `src/utils/gradient/gradient/interpolation.rs`
- `src/utils/gradient/presets.rs`

**Accessibility:**
- `src/utils/accessibility/mod.rs`
- `src/utils/accessibility/state.rs`
- `src/utils/accessibility/node.rs`
- `src/utils/accessibility/manager.rs`
- `src/utils/accessibility/roles.rs`
- `src/utils/accessibility/aria.rs`
- `src/utils/accessibility_signal.rs`

**Syntax:**
- `src/utils/syntax/mod.rs`
- `src/utils/syntax/helpers.rs`

**Path:**
- `src/utils/path/format.rs`

**Selection:**
- `src/utils/selection/mod.rs`
- `src/utils/selection/helper.rs`

**Other:**
- `src/utils/sort.rs`
- `src/utils/layout.rs`
- `src/utils/keymap.rs`

**Plus figlet, undo, textbuffer, border modules (after reading them)**

## Completion Criteria

Each module extraction is complete when:
1. ✅ New test file created in `tests/`
2. ✅ All tests moved with proper imports
3. ✅ Source file updated to remove `#[cfg(test)]` module
4. ✅ Tests compile successfully
5. ✅ Private field access tests removed (where applicable)
