//! Virtual list widget for efficiently rendering large datasets
//!
//! Only renders visible items, making it suitable for lists with
//! hundreds of thousands of items without performance degradation.
//!
//! # Features
//!
//! - **Variable height items**: Each item can have different heights
//! - **Jump-to-index**: Quickly scroll to any item by index
//! - **Smooth scrolling**: Configurable scroll behavior
//! - **Overscan**: Render extra items for smoother scrolling
//! - **Async loading**: Support for lazy-loaded data sources
//!
//! # Example
//!
//! ```ignore
//! use revue::widget::{VirtualList, VirtualListItem};
//!
//! // Create a virtual list with 100,000 items
//! let items: Vec<String> = (0..100_000)
//!     .map(|i| format!("Item {}", i))
//!     .collect();
//!
//! let list = VirtualList::new(items)
//!     .item_height(1)
//!     .selected(0);
//!
//! // With variable heights
//! let list = VirtualList::new(items)
//!     .variable_height(|item, _idx| if item.len() > 50 { 2 } else { 1 });
//!
//! // Jump to specific index
//! list.jump_to(5000);
//! ```

/// Item renderer function type
pub type ItemRenderer<T> = Box<dyn Fn(&T, usize, bool) -> String>;

/// Height calculator function type for variable heights
pub type HeightCalculator<T> = Box<dyn Fn(&T, usize) -> u16>;

/// Scroll behavior mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollMode {
    /// Item-by-item scrolling
    #[default]
    Item,
    /// Smooth pixel-based scrolling (simulated with sub-item offsets)
    Smooth,
    /// Center selected item when possible
    Center,
}

/// Scroll alignment when jumping to an item
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAlignment {
    /// Align to top of viewport
    #[default]
    Start,
    /// Align to center of viewport
    Center,
    /// Align to bottom of viewport
    End,
    /// Nearest edge (minimal scroll)
    Nearest,
}
