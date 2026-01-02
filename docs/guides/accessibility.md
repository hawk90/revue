# Accessibility Guide

Build inclusive terminal applications with Revue's accessibility features.

## Themes

### High Contrast Themes

```rust
use revue::style::Themes;

// WCAG AAA compliant themes
set_theme(Themes::high_contrast_dark());
set_theme(Themes::high_contrast_light());
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

// Built-in focus styles
Input::new().focus_style(FocusStyle::Solid)
Input::new().focus_style(FocusStyle::Double)
Input::new().focus_style(FocusStyle::Dotted)
Input::new().focus_style(FocusStyle::Bold)
```

### Drawing Focus Rings

```rust
use revue::utils::{draw_focus_ring, draw_focus_underline};

fn render(&self, ctx: &mut RenderContext) {
    if self.is_focused {
        draw_focus_ring(ctx.buffer, rect, Color::CYAN);
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
announce_dialog_closed("Confirm deletion");
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
use revue::widget::FocusTrap;

// Trap focus within modal
let _trap = FocusTrap::new();
render_modal();
// Focus released when _trap drops
```

### Nested Traps

```rust
use revue::event::FocusManager;

let fm = FocusManager::new();

// First modal
fm.push_trap();
// Second modal (nested)
fm.push_trap();

// Close second modal
fm.pop_trap();

// Close first modal
fm.pop_trap();
```

### Focus Restoration

```rust
// Save current focus
fm.push_trap();

// ... modal interaction

// Restore focus to previous element
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

## ARIA-like Properties

### Roles and Labels

```rust
// Semantic widget types
Button::new("Submit")
    .role("button")
    .label("Submit form")

// Status regions
Text::new(status)
    .role("status")
    .live("polite")  // Announce changes
```

### Live Regions

```rust
// Polite: Wait for pause in speech
.live("polite")

// Assertive: Interrupt immediately
.live("assertive")
```

## Color Accessibility

### Contrast Checking

Ensure sufficient contrast ratios:

```rust
use revue::utils::contrast_ratio;

let ratio = contrast_ratio(fg_color, bg_color);
// WCAG AA: 4.5:1 for normal text, 3:1 for large text
// WCAG AAA: 7:1 for normal text, 4.5:1 for large text
assert!(ratio >= 4.5);
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

    // Tab cycles through focusable elements
    pilot.press(Key::Tab);
    pilot.assert_focused("#input1");

    pilot.press(Key::Tab);
    pilot.assert_focused("#input2");

    // Shift+Tab goes back
    pilot.press_shift(Key::Tab);
    pilot.assert_focused("#input1");
}

#[test]
fn test_announcements() {
    let mut app = TestApp::new(MyApp::new());
    let mut pilot = Pilot::new(&mut app);

    pilot.press(Key::Enter);

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
// Icon buttons need labels
Button::icon("🔍")
    .label("Search")
    .tooltip("Search items")
```

### 3. Maintain Focus Context

```rust
// When showing a dialog, trap focus
let _trap = FocusTrap::new();
show_dialog();

// When hiding, restore focus
// (automatic when _trap drops)
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

// Full keyboard help
KeyboardHelp::new()
    .shortcut("↑/↓", "Navigate items")
    .shortcut("Enter", "Select item")
    .shortcut("Esc", "Close")
```
