//! Focus management with 2D navigation and focus trapping
//!
//! # Focus Trap Example
//!
//! ```rust,ignore
//! use revue::event::{FocusManager, FocusTrap, FocusTrapConfig};
//!
//! let mut fm = FocusManager::new();
//!
//! // Register some widgets
//! fm.register(1);
//! fm.register(2);
//! fm.register(3);  // Modal content
//! fm.register(4);  // Modal button
//!
//! // Create a focus trap for a modal
//! let trap = FocusTrap::new(100)
//!     .with_children(&[3, 4])
//!     .initial_focus(3)
//!     .restore_focus_on_release(true);
//!
//! // Activate the trap
//! trap.activate(&mut fm);
//!
//! // ... modal is open, Tab only cycles 3 and 4 ...
//!
//! // Release the trap (restores previous focus)
//! trap.deactivate(&mut fm);
//! ```

use crate::layout::Rect;

/// Widget identifier for focus tracking
pub type WidgetId = u64;

/// Direction for 2D navigation
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Navigate upward
    Up,
    /// Navigate downward
    Down,
    /// Navigate left
    Left,
    /// Navigate right
    Right,
}

/// Focusable widget info
#[derive(Clone, Debug)]
struct FocusableWidget {
    id: WidgetId,
    /// Position for 2D navigation (center point)
    position: Option<(u16, u16)>,
    /// Full bounds for hit testing
    bounds: Option<Rect>,
}

/// Focus manager for keyboard navigation
pub struct FocusManager {
    /// Widgets in registration order (Tab navigation)
    widgets: Vec<FocusableWidget>,
    /// Current focus index
    current: Option<usize>,
    /// Focus trap container ID (for modals)
    trap: Option<WidgetId>,
    /// Trapped widget IDs (children of trap)
    trapped_ids: Vec<WidgetId>,
    /// Focus state saved before trapping (for restoration)
    saved_focus: Option<WidgetId>,
    /// Stack of nested traps (for nested modals)
    trap_stack: Vec<TrapState>,
}

/// Saved state for a focus trap
#[derive(Clone, Debug)]
struct TrapState {
    /// Container ID of the trap
    container_id: WidgetId,
    /// Widget IDs in the trap
    trapped_ids: Vec<WidgetId>,
    /// Focus before this trap was activated
    previous_focus: Option<WidgetId>,
}

impl FocusManager {
    /// Create a new focus manager
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
            current: None,
            trap: None,
            trapped_ids: Vec::new(),
            saved_focus: None,
            trap_stack: Vec::new(),
        }
    }

    /// Get the list of focusable IDs (considering trap)
    /// Returns a Cow to avoid unnecessary allocations when not trapped
    fn focusable_ids(&self) -> std::borrow::Cow<'_, [WidgetId]> {
        if self.trap.is_some() && !self.trapped_ids.is_empty() {
            std::borrow::Cow::Borrowed(&self.trapped_ids)
        } else {
            std::borrow::Cow::Owned(self.widgets.iter().map(|w| w.id).collect())
        }
    }

    /// Register a widget in the focus order
    pub fn register(&mut self, id: WidgetId) {
        if !self.widgets.iter().any(|w| w.id == id) {
            self.widgets.push(FocusableWidget {
                id,
                position: None,
                bounds: None,
            });
        }
    }

    /// Register a widget with position for 2D navigation
    pub fn register_with_position(&mut self, id: WidgetId, x: u16, y: u16) {
        if let Some(widget) = self.widgets.iter_mut().find(|w| w.id == id) {
            widget.position = Some((x, y));
        } else {
            self.widgets.push(FocusableWidget {
                id,
                position: Some((x, y)),
                bounds: None,
            });
        }
    }

    /// Register a widget with bounds for 2D navigation
    pub fn register_with_bounds(&mut self, id: WidgetId, bounds: Rect) {
        let center_x = bounds.x + bounds.width / 2;
        let center_y = bounds.y + bounds.height / 2;

        if let Some(widget) = self.widgets.iter_mut().find(|w| w.id == id) {
            widget.position = Some((center_x, center_y));
            widget.bounds = Some(bounds);
        } else {
            self.widgets.push(FocusableWidget {
                id,
                position: Some((center_x, center_y)),
                bounds: Some(bounds),
            });
        }
    }

    /// Unregister a widget
    pub fn unregister(&mut self, id: WidgetId) {
        if let Some(pos) = self.widgets.iter().position(|w| w.id == id) {
            self.widgets.remove(pos);
            // Adjust current index if needed
            if let Some(current) = self.current {
                if self.widgets.is_empty() {
                    // No more widgets, clear focus
                    self.current = None;
                } else if pos < current {
                    // Removed before current, shift index down
                    self.current = Some(current.saturating_sub(1));
                } else if pos == current {
                    // Removed current widget
                    if current >= self.widgets.len() {
                        // Was at end, move to new last widget
                        self.current = Some(self.widgets.len().saturating_sub(1));
                    }
                    // else: stay at same index (now points to next widget)
                }
            }
            // Remove from trapped list if present
            self.trapped_ids.retain(|&i| i != id);
        }
    }

    /// Get the currently focused widget
    pub fn current(&self) -> Option<WidgetId> {
        self.current.and_then(|idx| self.widgets.get(idx).map(|w| w.id))
    }

    /// Move focus to next widget (Tab)
    pub fn next(&mut self) {
        let ids = self.focusable_ids();
        if ids.is_empty() {
            return;
        }

        let current_id = self.current();
        let current_idx = current_id.and_then(|id| ids.iter().position(|&i| i == id));

        let next_id = match current_idx {
            Some(idx) => ids[(idx + 1) % ids.len()],
            None => ids[0],
        };

        self.focus(next_id);
    }

    /// Move focus to previous widget (Shift+Tab)
    pub fn prev(&mut self) {
        let ids = self.focusable_ids();
        if ids.is_empty() {
            return;
        }

        let current_id = self.current();
        let current_idx = current_id.and_then(|id| ids.iter().position(|&i| i == id));

        let prev_id = match current_idx {
            Some(0) => ids[ids.len() - 1],
            Some(idx) => ids[idx - 1],
            None => ids[ids.len() - 1],
        };

        self.focus(prev_id);
    }

    /// Focus a specific widget
    pub fn focus(&mut self, id: WidgetId) {
        if let Some(idx) = self.widgets.iter().position(|w| w.id == id) {
            self.current = Some(idx);
        }
    }

    /// Check if a widget is focused
    pub fn is_focused(&self, id: WidgetId) -> bool {
        self.current() == Some(id)
    }

    /// Clear focus
    pub fn blur(&mut self) {
        self.current = None;
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 2D Navigation
    // ─────────────────────────────────────────────────────────────────────────

    /// Move focus in a direction (arrow key navigation)
    pub fn move_focus(&mut self, direction: Direction) -> bool {
        let current_idx = match self.current {
            Some(idx) => idx,
            None => return false,
        };

        let current_pos = match self.widgets.get(current_idx).and_then(|w| w.position) {
            Some(pos) => pos,
            None => return false, // No position, can't do 2D nav
        };

        let ids = self.focusable_ids();
        let candidates: Vec<_> = self.widgets.iter()
            .filter(|w| ids.contains(&w.id))
            .filter(|w| w.id != self.widgets[current_idx].id)
            .filter_map(|w| w.position.map(|p| (w.id, p)))
            .filter(|(_, pos)| {
                match direction {
                    Direction::Up => pos.1 < current_pos.1,
                    Direction::Down => pos.1 > current_pos.1,
                    Direction::Left => pos.0 < current_pos.0,
                    Direction::Right => pos.0 > current_pos.0,
                }
            })
            .collect();

        if candidates.is_empty() {
            return false;
        }

        // Find closest widget in that direction
        let closest = candidates.into_iter()
            .min_by_key(|(_, pos)| {
                let dx = (pos.0 as i32 - current_pos.0 as i32).abs();
                let dy = (pos.1 as i32 - current_pos.1 as i32).abs();
                // Weight primary direction more
                match direction {
                    Direction::Up | Direction::Down => dy * 2 + dx,
                    Direction::Left | Direction::Right => dx * 2 + dy,
                }
            });

        if let Some((id, _)) = closest {
            self.focus(id);
            true
        } else {
            false
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Focus Trapping (for modals)
    // ─────────────────────────────────────────────────────────────────────────

    /// Start trapping focus within a container
    ///
    /// All widgets registered as trapped will be the only ones focusable.
    /// Saves current focus for later restoration.
    pub fn trap_focus(&mut self, container_id: WidgetId) {
        // Save current focus before trapping
        self.saved_focus = self.current();
        self.trap = Some(container_id);
        self.trapped_ids.clear();
    }

    /// Start trapping focus with initial focus target
    pub fn trap_focus_with_initial(&mut self, container_id: WidgetId, initial_focus: WidgetId) {
        self.trap_focus(container_id);
        self.focus(initial_focus);
    }

    /// Add a widget to the trapped focus group
    pub fn add_to_trap(&mut self, id: WidgetId) {
        if self.trap.is_some() && !self.trapped_ids.contains(&id) {
            self.trapped_ids.push(id);
        }
    }

    /// Release focus trap and optionally restore previous focus
    pub fn release_trap(&mut self) {
        self.trap = None;
        self.trapped_ids.clear();
    }

    /// Release focus trap and restore previous focus
    pub fn release_trap_and_restore(&mut self) {
        let saved = self.saved_focus.take();
        self.release_trap();
        if let Some(id) = saved {
            self.focus(id);
        }
    }

    /// Check if focus is currently trapped
    pub fn is_trapped(&self) -> bool {
        self.trap.is_some()
    }

    /// Get the trap container ID
    pub fn trap_container(&self) -> Option<WidgetId> {
        self.trap
    }

    /// Get the saved focus (for manual restoration)
    pub fn saved_focus(&self) -> Option<WidgetId> {
        self.saved_focus
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Nested Focus Traps (for nested modals/dialogs)
    // ─────────────────────────────────────────────────────────────────────────

    /// Push a new focus trap (supports nesting)
    pub fn push_trap(&mut self, container_id: WidgetId, children: &[WidgetId]) {
        // Save current state
        let state = TrapState {
            container_id: self.trap.unwrap_or(0),
            trapped_ids: self.trapped_ids.clone(),
            previous_focus: self.current(),
        };
        self.trap_stack.push(state);

        // Set new trap
        self.trap = Some(container_id);
        self.trapped_ids = children.to_vec();

        // Focus first child if any
        if let Some(&first) = children.first() {
            self.focus(first);
        }
    }

    /// Pop and restore the previous focus trap
    pub fn pop_trap(&mut self) -> bool {
        if let Some(state) = self.trap_stack.pop() {
            // Restore previous trap state
            self.trap = if state.container_id == 0 {
                None
            } else {
                Some(state.container_id)
            };
            self.trapped_ids = state.trapped_ids;

            // Restore focus
            if let Some(id) = state.previous_focus {
                self.focus(id);
            }
            true
        } else {
            // No stack, just release current trap
            self.release_trap_and_restore();
            false
        }
    }

    /// Get the trap stack depth
    pub fn trap_depth(&self) -> usize {
        // Stack contains previous states, so depth = stack.len() when trap is active
        if self.trap.is_some() {
            self.trap_stack.len()
        } else {
            0
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// FocusTrap Helper
// ─────────────────────────────────────────────────────────────────────────────

/// Configuration for a focus trap
#[derive(Clone, Debug)]
pub struct FocusTrapConfig {
    /// Whether to restore focus when trap is released
    pub restore_on_release: bool,
    /// Initial focus target (None = first child)
    pub initial_focus: Option<WidgetId>,
    /// Whether to loop focus at boundaries
    pub loop_focus: bool,
}

impl Default for FocusTrapConfig {
    fn default() -> Self {
        Self {
            restore_on_release: true,
            initial_focus: None,
            loop_focus: true,
        }
    }
}

/// A focus trap helper for modals and dialogs
#[derive(Clone, Debug)]
pub struct FocusTrap {
    /// Container ID
    container_id: WidgetId,
    /// Child widget IDs
    children: Vec<WidgetId>,
    /// Configuration
    config: FocusTrapConfig,
    /// Whether currently active
    active: bool,
}

impl FocusTrap {
    /// Create a new focus trap
    pub fn new(container_id: WidgetId) -> Self {
        Self {
            container_id,
            children: Vec::new(),
            config: FocusTrapConfig::default(),
            active: false,
        }
    }

    /// Add children to the trap
    pub fn with_children(mut self, children: &[WidgetId]) -> Self {
        self.children = children.to_vec();
        self
    }

    /// Add a single child
    pub fn add_child(mut self, id: WidgetId) -> Self {
        if !self.children.contains(&id) {
            self.children.push(id);
        }
        self
    }

    /// Set initial focus target
    pub fn initial_focus(mut self, id: WidgetId) -> Self {
        self.config.initial_focus = Some(id);
        self
    }

    /// Configure focus restoration
    pub fn restore_focus_on_release(mut self, restore: bool) -> Self {
        self.config.restore_on_release = restore;
        self
    }

    /// Configure focus looping
    pub fn loop_focus(mut self, loop_focus: bool) -> Self {
        self.config.loop_focus = loop_focus;
        self
    }

    /// Get the container ID
    pub fn container_id(&self) -> WidgetId {
        self.container_id
    }

    /// Check if trap is active
    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Activate the focus trap
    pub fn activate(&mut self, fm: &mut FocusManager) {
        if self.active {
            return;
        }

        fm.push_trap(self.container_id, &self.children);

        // Set initial focus
        if let Some(initial) = self.config.initial_focus {
            fm.focus(initial);
        } else if let Some(&first) = self.children.first() {
            fm.focus(first);
        }

        self.active = true;
    }

    /// Deactivate the focus trap
    pub fn deactivate(&mut self, fm: &mut FocusManager) {
        if !self.active {
            return;
        }

        fm.pop_trap();
        self.active = false;
    }
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focus_manager_new() {
        let fm = FocusManager::new();
        assert!(fm.current().is_none());
    }

    #[test]
    fn test_focus_register() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);

        // No focus yet
        assert!(fm.current().is_none());
    }

    #[test]
    fn test_focus_next() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);

        fm.next();
        assert_eq!(fm.current(), Some(1));

        fm.next();
        assert_eq!(fm.current(), Some(2));

        fm.next();
        assert_eq!(fm.current(), Some(3));

        // Wrap around
        fm.next();
        assert_eq!(fm.current(), Some(1));
    }

    #[test]
    fn test_focus_prev() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);

        fm.prev();
        assert_eq!(fm.current(), Some(3));

        fm.prev();
        assert_eq!(fm.current(), Some(2));

        fm.prev();
        assert_eq!(fm.current(), Some(1));

        // Wrap around
        fm.prev();
        assert_eq!(fm.current(), Some(3));
    }

    #[test]
    fn test_focus_specific_widget() {
        let mut fm = FocusManager::new();
        fm.register(10);
        fm.register(20);
        fm.register(30);

        fm.focus(20);
        assert_eq!(fm.current(), Some(20));

        fm.focus(30);
        assert_eq!(fm.current(), Some(30));
    }

    #[test]
    fn test_is_focused() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);

        fm.focus(1);
        assert!(fm.is_focused(1));
        assert!(!fm.is_focused(2));
    }

    #[test]
    fn test_blur() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.next();
        assert!(fm.current().is_some());

        fm.blur();
        assert!(fm.current().is_none());
    }

    #[test]
    fn test_unregister() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);

        fm.focus(2);
        fm.unregister(1);

        // Focus should adjust
        assert_eq!(fm.current(), Some(2));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // 2D Navigation Tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_2d_navigation_right() {
        let mut fm = FocusManager::new();
        // Layout:  [1] [2] [3]
        //           x=0  10  20
        fm.register_with_position(1, 0, 0);
        fm.register_with_position(2, 10, 0);
        fm.register_with_position(3, 20, 0);

        fm.focus(1);
        assert!(fm.move_focus(Direction::Right));
        assert_eq!(fm.current(), Some(2));

        assert!(fm.move_focus(Direction::Right));
        assert_eq!(fm.current(), Some(3));

        // No more to the right
        assert!(!fm.move_focus(Direction::Right));
    }

    #[test]
    fn test_2d_navigation_down() {
        let mut fm = FocusManager::new();
        // Layout:  [1]
        //          [2]
        //          [3]
        fm.register_with_position(1, 0, 0);
        fm.register_with_position(2, 0, 10);
        fm.register_with_position(3, 0, 20);

        fm.focus(1);
        assert!(fm.move_focus(Direction::Down));
        assert_eq!(fm.current(), Some(2));

        assert!(fm.move_focus(Direction::Down));
        assert_eq!(fm.current(), Some(3));
    }

    #[test]
    fn test_2d_navigation_grid() {
        let mut fm = FocusManager::new();
        // Layout:  [1] [2]
        //          [3] [4]
        fm.register_with_position(1, 0, 0);
        fm.register_with_position(2, 10, 0);
        fm.register_with_position(3, 0, 10);
        fm.register_with_position(4, 10, 10);

        fm.focus(1);

        // Right to 2
        assert!(fm.move_focus(Direction::Right));
        assert_eq!(fm.current(), Some(2));

        // Down to 4
        assert!(fm.move_focus(Direction::Down));
        assert_eq!(fm.current(), Some(4));

        // Left to 3
        assert!(fm.move_focus(Direction::Left));
        assert_eq!(fm.current(), Some(3));

        // Up to 1
        assert!(fm.move_focus(Direction::Up));
        assert_eq!(fm.current(), Some(1));
    }

    #[test]
    fn test_register_with_bounds() {
        use crate::layout::Rect;

        let mut fm = FocusManager::new();
        let bounds = Rect::new(10, 5, 20, 10);
        fm.register_with_bounds(1, bounds);

        fm.focus(1);
        assert_eq!(fm.current(), Some(1));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Focus Trapping Tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_focus_trap() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);  // Modal button 1
        fm.register(4);  // Modal button 2

        // Focus on widget 1
        fm.focus(1);
        assert_eq!(fm.current(), Some(1));

        // Trap focus to modal (widgets 3 and 4)
        fm.trap_focus(100);  // Modal container ID
        fm.add_to_trap(3);
        fm.add_to_trap(4);

        assert!(fm.is_trapped());

        // Tab should now only cycle between 3 and 4
        fm.focus(3);
        fm.next();
        assert_eq!(fm.current(), Some(4));

        fm.next();
        assert_eq!(fm.current(), Some(3));  // Wraps within trap

        // Release trap
        fm.release_trap();
        assert!(!fm.is_trapped());

        // Now Tab cycles all widgets again
        fm.focus(1);
        fm.next();
        assert_eq!(fm.current(), Some(2));
    }

    #[test]
    fn test_trap_container() {
        let mut fm = FocusManager::new();
        fm.trap_focus(42);
        assert_eq!(fm.trap_container(), Some(42));

        fm.release_trap();
        assert_eq!(fm.trap_container(), None);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Focus Restoration Tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_focus_restoration() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);
        fm.register(4);

        // Focus on widget 2
        fm.focus(2);
        assert_eq!(fm.current(), Some(2));

        // Trap focus
        fm.trap_focus(100);
        fm.add_to_trap(3);
        fm.add_to_trap(4);
        fm.focus(3);

        // Saved focus should be 2
        assert_eq!(fm.saved_focus(), Some(2));

        // Release and restore
        fm.release_trap_and_restore();
        assert_eq!(fm.current(), Some(2));
    }

    #[test]
    fn test_trap_with_initial_focus() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);

        fm.focus(1);
        fm.trap_focus_with_initial(100, 3);
        assert_eq!(fm.current(), Some(3));
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Nested Focus Trap Tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_push_pop_trap() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);
        fm.register(4);
        fm.register(5);

        // Start on widget 1
        fm.focus(1);
        assert_eq!(fm.current(), Some(1));

        // Push first trap (modal 1)
        fm.push_trap(100, &[2, 3]);
        assert_eq!(fm.current(), Some(2));
        assert_eq!(fm.trap_depth(), 1);

        // Push second trap (modal 2)
        fm.push_trap(200, &[4, 5]);
        assert_eq!(fm.current(), Some(4));
        assert_eq!(fm.trap_depth(), 2);

        // Pop second trap - should restore to modal 1
        fm.pop_trap();
        assert_eq!(fm.current(), Some(2));
        assert_eq!(fm.trap_depth(), 1);

        // Pop first trap - should restore to original
        fm.pop_trap();
        assert_eq!(fm.current(), Some(1));
        assert_eq!(fm.trap_depth(), 0);
    }

    #[test]
    fn test_trap_depth() {
        let mut fm = FocusManager::new();
        fm.register(1);

        assert_eq!(fm.trap_depth(), 0);

        fm.push_trap(100, &[1]);
        assert_eq!(fm.trap_depth(), 1);

        fm.push_trap(200, &[1]);
        assert_eq!(fm.trap_depth(), 2);

        fm.pop_trap();
        assert_eq!(fm.trap_depth(), 1);

        fm.pop_trap();
        assert_eq!(fm.trap_depth(), 0);
    }

    // ─────────────────────────────────────────────────────────────────────────
    // FocusTrap Helper Tests
    // ─────────────────────────────────────────────────────────────────────────

    #[test]
    fn test_focus_trap_helper() {
        let mut fm = FocusManager::new();
        fm.register(1);
        fm.register(2);
        fm.register(3);

        fm.focus(1);

        let mut trap = FocusTrap::new(100)
            .with_children(&[2, 3])
            .initial_focus(3);

        assert!(!trap.is_active());

        trap.activate(&mut fm);
        assert!(trap.is_active());
        assert_eq!(fm.current(), Some(3));

        trap.deactivate(&mut fm);
        assert!(!trap.is_active());
        assert_eq!(fm.current(), Some(1));  // Restored
    }

    #[test]
    fn test_focus_trap_add_child() {
        let trap = FocusTrap::new(100)
            .add_child(1)
            .add_child(2)
            .add_child(2);  // Duplicate should be ignored

        assert_eq!(trap.children.len(), 2);
    }

    #[test]
    fn test_focus_trap_config() {
        let trap = FocusTrap::new(100)
            .restore_focus_on_release(false)
            .loop_focus(false)
            .initial_focus(42);

        assert!(!trap.config.restore_on_release);
        assert!(!trap.config.loop_focus);
        assert_eq!(trap.config.initial_focus, Some(42));
    }
}
