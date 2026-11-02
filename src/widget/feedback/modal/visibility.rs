//! Showing and hiding the modal, with or without a focus trap

use super::Modal;
use crate::event::{FocusManager, FocusTrap};

impl Modal {
    /// Show the modal
    ///
    /// While it is open, Tab and clicks only give focus to controls in its body,
    /// never to anything behind it.
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Show the modal and activate focus trapping
    ///
    /// Traps keyboard focus within the modal's buttons so Tab/Shift+Tab
    /// only cycles between modal buttons and doesn't escape to background widgets.
    ///
    /// # Arguments
    /// * `fm` - The focus manager to trap focus in
    /// * `container_id` - Unique ID for this modal's focus trap container
    /// * `button_ids` - Widget IDs for each button (must match button count)
    pub fn show_with_focus_trap(
        &mut self,
        fm: &mut FocusManager,
        container_id: u64,
        button_ids: &[u64],
    ) {
        self.visible = true;
        let mut trap = FocusTrap::new(container_id).with_children(button_ids);
        trap.activate(fm);
        self.focus_trap = Some(trap);
    }

    /// Hide the modal
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Hide the modal and release focus trapping
    ///
    /// Releases the focus trap and restores focus to the previously focused widget.
    pub fn hide_with_focus_restore(&mut self, fm: &mut FocusManager) {
        self.visible = false;
        self.release_focus_trap(fm);
    }

    /// Deactivate and drop the focus trap, if there is one
    pub(super) fn release_focus_trap(&mut self, fm: &mut FocusManager) {
        if let Some(mut trap) = self.focus_trap.take() {
            trap.deactivate(fm);
        }
    }

    /// Check if this modal has an active focus trap
    pub fn has_focus_trap(&self) -> bool {
        self.focus_trap.as_ref().is_some_and(|t| t.is_active())
    }

    /// Toggle visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Check if modal is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
