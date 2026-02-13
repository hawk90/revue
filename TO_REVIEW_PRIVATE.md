# Fields/Methods to Revert to Private

This file tracks public fields/methods that were added for testing purposes and should be reverted to private.

## Format
- **File**: `src/path/to/file.rs`
  - Line X: `pub field_name` -> revert to private
  - Line Y: `#[doc(hidden)] pub fn method_name()` -> remove or make private

---

## Canvas
- `src/widget/canvas/braille/mod.rs`: Various `#[doc(hidden)]` test accessors

## Developer
- `src/widget/developer/terminal/ansi.rs`: Lines 61-74 - `#[doc(hidden)] pub fn state()`, `fg()`, `bg()`

## Form (Rich Text Editor)
- `src/widget/form/rich_text_editor/mod.rs`: All submodules made `pub mod` for testing

## Data
- `src/widget/data/datagrid/core.rs`: `#[doc(hidden)]` getters
- `src/widget/data/json_viewer/helpers.rs`: `#[doc(hidden)]` getters
- `src/widget/data/virtuallist/core.rs`: `#[doc(hidden)]` getters

## Layout
- `src/widget/layout/grid/layout.rs`: Lines 241-263 - `#[doc(hidden)] pub fn test_*()` methods
- `src/widget/layout/card/core.rs`: `#[doc(hidden)]` getters (if any)

## Feedback
- `src/widget/feedback/modal.rs`: `#[doc(hidden)]` getters

## Input
- `src/widget/input/input_widgets/autocomplete/core.rs`: `#[doc(hidden)]` getters
- `src/widget/input/input_widgets/number_input/core.rs`: `#[doc(hidden)]` getters

---

## Strategy for Reverting
1. For `#[doc(hidden)]` methods: Can keep as-is (they're documented as test-only)
2. For `pub` fields: Consider if tests really need direct access, or use getter methods
3. For `pub mod` changes: Revert to `mod` and move tests back to source files if needed
