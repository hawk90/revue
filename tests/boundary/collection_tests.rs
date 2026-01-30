//! Boundary condition tests for collection operations
//!
//! Tests edge cases for collection handling including:
//! - Empty collections
//! - Single item collections
//! - Index boundaries
//! - Collection overflow scenarios

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{Direction, Stack, Text};

/// Test empty container widgets
mod empty_containers {
    use super::*;

    #[test]
    fn test_stack_empty() {
        let stack = Stack::new();
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with empty stack
        stack.render(&mut ctx);
    }

    #[test]
    fn test_hstack_empty() {
        let stack = Stack::new().direction(Direction::Row);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with empty stack
        stack.render(&mut ctx);
    }

    #[test]
    fn test_vstack_empty() {
        let stack = Stack::new().direction(Direction::Column);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with empty stack
        stack.render(&mut ctx);
    }
}

/// Test single item containers
mod single_item_containers {
    use super::*;

    #[test]
    fn test_hstack_single_item() {
        let stack = Stack::new()
            .direction(Direction::Row)
            .child(Text::new("Hello"));

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);

        // Should render the single item
        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().symbol, 'H');
    }

    #[test]
    fn test_vstack_single_item() {
        let stack = Stack::new()
            .direction(Direction::Column)
            .child(Text::new("Hello"));

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);

        let cell = buffer.get(0, 0);
        assert!(cell.is_some());
        assert_eq!(cell.unwrap().symbol, 'H');
    }
}

/// Test container overflow scenarios
mod container_overflow {
    use super::*;

    #[test]
    fn test_hstack_many_items() {
        let mut stack = Stack::new().direction(Direction::Row);
        for i in 0..100 {
            stack = stack.child(Text::new(format!("X{}", i)));
        }

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle overflow gracefully (clip)
        stack.render(&mut ctx);
    }

    #[test]
    fn test_vstack_many_items() {
        let mut stack = Stack::new().direction(Direction::Column);
        for _ in 0..100 {
            stack = stack.child(Text::new("Line"));
        }

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);
    }
}

/// Test container size calculations
mod size_calculations {
    use super::*;

    #[test]
    fn test_hstack_zero_width() {
        let stack = Stack::new()
            .direction(Direction::Row)
            .child(Text::new("A"))
            .child(Text::new("B"));

        let mut buffer = Buffer::new(0, 10);
        let area = Rect::new(0, 0, 0, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with zero width
        stack.render(&mut ctx);
    }

    #[test]
    fn test_vstack_zero_height() {
        let stack = Stack::new()
            .direction(Direction::Column)
            .child(Text::new("A"))
            .child(Text::new("B"));

        let mut buffer = Buffer::new(10, 0);
        let area = Rect::new(0, 0, 10, 0);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should not panic with zero height
        stack.render(&mut ctx);
    }

    #[test]
    fn test_hstack_item_wider_than_container() {
        let stack = Stack::new()
            .direction(Direction::Row)
            .child(Text::new("This is a very long text"));

        let mut buffer = Buffer::new(5, 10);
        let area = Rect::new(0, 0, 5, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should clip to container width
        stack.render(&mut ctx);
    }

    #[test]
    fn test_vstack_item_taller_than_container() {
        let mut stack = Stack::new().direction(Direction::Column);
        for _ in 0..20 {
            stack = stack.child(Text::new("Line"));
        }

        let mut buffer = Buffer::new(10, 5);
        let area = Rect::new(0, 0, 10, 5);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should clip to container height
        stack.render(&mut ctx);
    }
}

/// Test container modification
mod container_modification {
    use super::*;

    #[test]
    fn test_stack_add_to_empty() {
        let stack = Stack::new().child(Text::new("First"));

        let children = stack.children();
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn test_vstack_add_multiple() {
        let mut stack = Stack::new().direction(Direction::Column);
        stack = stack.child(Text::new("1"));
        stack = stack.child(Text::new("2"));
        stack = stack.child(Text::new("3"));

        let children = stack.children();
        assert_eq!(children.len(), 3);
    }
}

/// Test container with empty items
mod empty_items {
    use super::*;

    #[test]
    fn test_hstack_with_empty_text() {
        let stack = Stack::new()
            .direction(Direction::Row)
            .child(Text::new("First"))
            .child(Text::new(""))
            .child(Text::new("Second"));

        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        // Should handle empty text widget
        stack.render(&mut ctx);
    }

    #[test]
    fn test_vstack_with_empty_texts() {
        let stack = Stack::new()
            .direction(Direction::Column)
            .child(Text::new(""))
            .child(Text::new(""))
            .child(Text::new(""));

        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);
    }
}

/// Test large collections
mod large_collections {
    use super::*;

    #[test]
    fn test_hstack_very_many_items() {
        let mut stack = Stack::new().direction(Direction::Row);
        for i in 0..1000 {
            stack = stack.child(Text::new(format!("{}", i % 10)));
        }

        let mut buffer = Buffer::new(100, 10);
        let area = Rect::new(0, 0, 100, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);
    }

    #[test]
    fn test_vstack_very_many_items() {
        let mut stack = Stack::new().direction(Direction::Column);
        for i in 0..1000 {
            stack = stack.child(Text::new(format!("Line {}", i)));
        }

        let mut buffer = Buffer::new(10, 100);
        let area = Rect::new(0, 0, 10, 100);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);
    }

    #[test]
    fn test_stack_with_large_gap() {
        // Use a large but realistic gap value
        let stack = Stack::new()
            .child(Text::new("A"))
            .child(Text::new("B"))
            .gap(50);

        let mut buffer = Buffer::new(200, 10);
        let area = Rect::new(0, 0, 200, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);

        stack.render(&mut ctx);
    }
}
