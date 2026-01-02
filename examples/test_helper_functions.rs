//! Test the new helper functions for Input and List widgets
//!
//! Run with: cargo run --example test_helper_functions

use revue::prelude::*;

fn main() -> Result<()> {
    println!("Testing new helper functions...\n");

    // Test input() helper function
    let input_widget = input()
        .placeholder("Type here...")
        .value("Hello from input()!");
    println!("✅ input() helper works: {}", input_widget.text());

    // Test list() helper function
    let list_widget = list(vec!["Item 1", "Item 2", "Item 3"]).selected(1);
    println!(
        "✅ list() helper works: {} items, selected: {}",
        list_widget.len(),
        list_widget.selected_index()
    );

    // Test text() helper (already existed)
    let text_widget = text("Hello from text()!");
    println!("✅ text() helper works: {}", text_widget.content());

    // Test Text::default() (newly added)
    let default_text = Text::default();
    println!("✅ Text::default() works: '{}'", default_text.content());

    println!("\n✨ All helper functions and Default trait working correctly!");

    Ok(())
}
