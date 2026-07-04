# Accessibility Guide

Build inclusive terminal applications with Revue's accessibility features.

## Themes

### High Contrast Themes

```rust
use revue::style::{themes, BuiltinTheme};

// High contrast styling is provided as CSS modules
let dark_css = themes::high_contrast_dark::css();
let light_css = themes::high_contrast_light::css();

// The same CSS is also reachable through the BuiltinTheme enum
let dark_css = BuiltinTheme::HighContrastDark.css();

// Apply it to your app
App::builder().css(dark_css).build();
```

### Checking Theme Mode

```rust
use revue::utils::is_high_contrast;

if is_high_contrast() {
    // Use simpler visuals
    Border::new()
} else {
    Border::rounded()
}
```

## Reduced Motion

Respect user preferences for reduced motion:

```rust
use revue::prelude::*;

// Check preference
if prefers_reduced_motion() {
    // Skip animations
} else {
    // Animate normally
}

// Set preference programmatically
set_reduced_motion(true);
```

### Automatic Animation Skipping

All Revue animations respect reduced motion:

```rust
use revue::style::{should_skip_animation, effective_duration};

// Returns true if animations should be skipped
if should_skip_animation() { }

// Returns Duration::ZERO when reduced motion is preferred
let duration = effective_duration(Duration::from_millis(300));
```

## Focus Indicators

### Focus Styles

```rust
use revue::widget::FocusStyle;

// Focus ring appearance is described by the FocusStyle enum
// (Solid, Double, Dotted, Bold, Rounded). These variants are applied
// when drawing a focus ring (see "Drawing Focus Rings" below), not
// through a per-widget builder method.
let style = FocusStyle::Double;
```

### Drawing Focus Rings

```rust
fn render(&self, ctx: &mut RenderContext) {
    // Apply a default focus ring (Rounded + Cyan) around the full area
    ctx.apply_default_focus(self.is_focused);

    // Or use a specific style — coordinates are relative (0,0 = area top-left)
    if self.is_focused {
        ctx.draw_focus_ring(0, 0, ctx.width(), ctx.height(), Color::CYAN, FocusStyle::Rounded);
    }
}
```

## Screen Reader Announcements

### Basic Announcements

```rust
use revue::utils::{announce, announce_now};

// Queue announcement (after current speech)
announce("Item selected");

// Interrupt with important message
announce_now("Error: Invalid input");
```

### Widget-Specific Helpers

```rust
use revue::utils::*;

// Button clicked
announce_button_clicked("Submit");

// Checkbox changed
announce_checkbox_changed("Dark mode", true);

// List selection
announce_list_selection("Settings", 2, 10);

// Tab changed
announce_tab_changed("General", 0, 3);

// Success/error
announce_success("File saved");
announce_error("Failed to connect");

// Dialogs
announce_dialog_opened("Confirm deletion");
announce_dialog_closed();
```

### Retrieving Announcements

```rust
use revue::utils::{take_announcements, has_announcements};

if has_announcements() {
    for msg in take_announcements() {
        // Send to screen reader
        screen_reader.speak(&msg);
    }
}
```

## Focus Management

### Focus Traps

Keep focus within a modal or dialog:

```rust
use revue::event::{FocusTrap, FocusManager};

let mut fm = FocusManager::new();

// Trap focus within a modal identified by its widget id
let mut trap = FocusTrap::new(modal_id)
    .with_children(&[input_id, ok_button_id, cancel_button_id]);

trap.activate(&mut fm);   // Focus is now confined to the modal
render_modal();
trap.deactivate(&mut fm); // Release the trap
```

### Nested Traps

```rust
use revue::event::FocusManager;

let mut fm = FocusManager::new();

// First modal
fm.push_trap(first_modal_id, &first_modal_children);
// Second modal (nested)
fm.push_trap(second_modal_id, &second_modal_children);

// Close second modal
fm.pop_trap();

// Close first modal
fm.pop_trap();
```

### Focus Restoration

```rust
// Save current focus and trap within the modal
fm.push_trap(modal_id, &modal_children);

// ... modal interaction

// Restore focus to the previously focused element
fm.release_trap_and_restore();
```

## Keyboard Navigation

### Standard Keys

Support standard navigation patterns:

```rust
fn handle_key(&mut self, key: &Key) -> bool {
    match key {
        // Arrow keys for navigation
        Key::Up | Key::Char('k') => self.move_up(),
        Key::Down | Key::Char('j') => self.move_down(),

        // Tab for focus cycling
        Key::Tab => self.focus_next(),
        Key::BackTab => self.focus_prev(),

        // Enter for activation
        Key::Enter | Key::Char(' ') => self.activate(),

        // Escape for cancel/close
        Key::Escape => self.cancel(),

        _ => return false,
    }
    true
}
```

### Skip Links

For complex layouts:

```rust
fn handle_key(&mut self, key: &Key) -> bool {
    match key {
        // Skip to main content
        Key::Char('m') if event.ctrl => {
            self.focus_main_content();
            true
        }
        // Skip to navigation
        Key::Char('n') if event.ctrl => {
            self.focus_navigation();
            true
        }
        _ => false,
    }
}
```

## Color Accessibility

### Contrast Checking

Pick a readable foreground color for a given background:

```rust
use revue::utils::contrast_color;

// Returns black or white — whichever contrasts better with the input
let fg = contrast_color(bg_color);
text.fg(fg).bg(bg_color)
// Aim for WCAG AA (4.5:1 normal text, 3:1 large text) or
// AAA (7:1 normal text, 4.5:1 large text) contrast.
```

### Color Independence

Don't rely solely on color:

```rust
// Bad: Color only
if error {
    text.fg(Color::RED)
} else {
    text.fg(Color::GREEN)
}

// Good: Color + symbol
if error {
    text.fg(Color::RED).prefix("✗ ")
} else {
    text.fg(Color::GREEN).prefix("✓ ")
}
```

## Testing Accessibility

### Automated Checks

```rust
#[test]
fn test_focus_management() {
    let mut app = TestApp::new(MyApp::new());
    let mut pilot = Pilot::new(&mut app);

    // Tab cycles forward through focusable elements
    pilot.press_tab();
    pilot.press_tab();

    // Shift+Tab (backtab) cycles backward
    pilot.press_backtab();
}

#[test]
fn test_announcements() {
    let mut app = TestApp::new(MyApp::new());
    let mut pilot = Pilot::new(&mut app);

    pilot.press_key(Key::Enter);

    let announcements = take_announcements();
    assert!(announcements.contains(&"Button clicked".to_string()));
}
```

## Best Practices

### 1. Always Support Keyboard

Every interactive element must be keyboard accessible:

```rust
impl Interactive for MyWidget {
    fn handle_key(&mut self, event: &KeyEvent) -> EventResult {
        match event.key {
            Key::Enter | Key::Char(' ') => {
                self.activate();
                EventResult::Consumed
            }
            _ => EventResult::Ignored,
        }
    }
}
```

### 2. Provide Text Alternatives

```rust
// Button::icon() takes a single char
Button::icon('🔍')

// Announce the button's purpose to screen readers
announce_button_clicked("Search");
```

### 3. Maintain Focus Context

```rust
// When showing a dialog, trap focus within it
let mut trap = FocusTrap::new(dialog_id).with_children(&dialog_children);
trap.activate(&mut fm);
show_dialog();

// When hiding, release the trap to restore focus
trap.deactivate(&mut fm);
```

### 4. Test with Screen Readers

Common screen readers:
- **macOS**: VoiceOver (Cmd+F5)
- **Windows**: NVDA (free), JAWS
- **Linux**: Orca

### 5. Document Keyboard Shortcuts

Show available shortcuts:

```rust
// Help text
Text::muted("[↑↓] Navigate  [Enter] Select  [Esc] Cancel")
```
