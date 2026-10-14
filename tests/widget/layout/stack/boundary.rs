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
        stack.render(&mut ctx);
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        assert!((0..buffer.height()).all(|y| row(y).is_empty()));
    }

    #[test]
    fn test_hstack_empty() {
        let stack = Stack::new().direction(Direction::Row);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        stack.render(&mut ctx);
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        assert!((0..buffer.height()).all(|y| row(y).is_empty()));
    }

    #[test]
    fn test_vstack_empty() {
        let stack = Stack::new().direction(Direction::Column);
        let mut buffer = Buffer::new(10, 10);
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        stack.render(&mut ctx);
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        assert!((0..buffer.height()).all(|y| row(y).is_empty()));
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // Measured children that overflow shrink in proportion: one column
        // each for the first ten, on one row
        assert_eq!(row(0), "XXXXXXXXXX");
        assert!((1..10).all(|y| row(y).is_empty()));
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // One row each for the first ten
        assert!((0..10).all(|y| row(y) == "Line"));
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
        stack.render(&mut ctx);
        // Nothing to draw into
        assert_eq!(buffer.width(), 0);
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
        stack.render(&mut ctx);
        assert_eq!(buffer.height(), 0);
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // The text wraps within the five columns
        assert_eq!(row(0), "This");
        assert!(row(0).chars().count() <= 5);
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // The first five lines, one row each
        assert!((0..5).all(|y| row(y) == "Line"));
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // An empty text takes no columns
        assert_eq!(row(0), "FirstSecond");
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        assert!((0..buffer.height()).all(|y| row(y).is_empty()));
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // The first hundred, one column each
        assert_eq!(row(0), "0123456789".repeat(10));
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // The first hundred, one row each
        assert_eq!(row(0), "Line 0");
        assert_eq!(row(99), "Line 99");
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
        let row = |y: u16| -> String {
            (0..buffer.width())
                .map(|x| buffer.get(x, y).unwrap().symbol)
                .collect::<String>()
                .trim_end()
                .to_string()
        };
        // A row by default: A, fifty columns, B
        assert_eq!(row(0), format!("A{}B", " ".repeat(50)));
    }
}
