# Animation Guide

Revue provides a comprehensive animation system for creating smooth, natural motion in terminal UIs.

## Overview

The animation system consists of two main modules:

- **Style Animations** (`revue::style::animation`) - CSS-like keyframe animations, tweens, and widget presets
- **Utility Animations** (`revue::utils`) - Spring physics, sequences, and timing utilities

## Tween Animations

Tweens interpolate between two values over time.

### Basic Tween

```rust
use revue::prelude::*;
use std::time::Duration;

// Create a tween from 0 to 100 over 1 second
let mut tween = Tween::new(0.0, 100.0, Duration::from_millis(1000));
tween.start();

// In render loop
let value = tween.value();     // Current animated value
let progress = tween.progress(); // 0.0 to 1.0
```

### Easing Functions

Easing controls the rate of change during animation:

```rust
use revue::style::animation::easing;

let tween = Tween::new(0.0, 100.0, Duration::from_millis(500))
    .easing(easing::ease_out);  // Slow end
```

Available easing functions:

| Function | Description |
|----------|-------------|
| `linear` | Constant speed |
| `ease_in` | Slow start |
| `ease_out` | Slow end |
| `ease_in_out` | Slow start and end |
| `ease_in_cubic` | Cubic slow start |
| `ease_out_cubic` | Cubic slow end |
| `bounce_out` | Bouncy finish |
| `elastic_out` | Springy overshoot |
| `back_out` | Slight overshoot |

### Tween Options

```rust
let tween = Tween::new(0.0, 100.0, Duration::from_millis(500))
    .easing(easing::ease_out)
    .delay(Duration::from_millis(200))  // Wait before starting
    .repeat(3)                          // Repeat 3 times
    .reverse(true);                     // Ping-pong mode
```

### Tween Control

```rust
let mut tween = Tween::new(0.0, 100.0, Duration::from_secs(1));

tween.start();      // Start animation
tween.pause();      // Pause
tween.resume();     // Resume from pause
tween.reset();      // Reset to initial state

// Check state
tween.is_running();
tween.is_completed();
```

## Keyframe Animations

CSS @keyframes-style animations with multiple property interpolation.

### Basic Keyframes

```rust
use revue::style::animation::{KeyframeAnimation, AnimationFillMode};

let mut anim = KeyframeAnimation::new("fade-slide")
    .keyframe(0, |kf| kf
        .set("opacity", 0.0)
        .set("x", -20.0))
    .keyframe(50, |kf| kf
        .set("opacity", 1.0)
        .set("x", 10.0))
    .keyframe(100, |kf| kf
        .set("opacity", 1.0)
        .set("x", 0.0))
    .duration(Duration::from_millis(500))
    .easing(easing::ease_out)
    .fill_mode(AnimationFillMode::Forwards);

anim.start();

// Get property values
let opacity = anim.get("opacity");
let x = anim.get("x");
```

### Animation Direction

```rust
use revue::style::animation::AnimationDirection;

let anim = KeyframeAnimation::new("bounce")
    .keyframe(0, |kf| kf.set("y", 0.0))
    .keyframe(100, |kf| kf.set("y", 10.0))
    .direction(AnimationDirection::Alternate)  // Back and forth
    .iterations(4);
```

| Direction | Description |
|-----------|-------------|
| `Normal` | 0% to 100% |
| `Reverse` | 100% to 0% |
| `Alternate` | Alternates each iteration |
| `AlternateReverse` | Starts in reverse, then alternates |

### Fill Mode

Controls behavior before/after animation:

```rust
use revue::style::animation::AnimationFillMode;

let anim = KeyframeAnimation::new("appear")
    .keyframe(0, |kf| kf.set("opacity", 0.0))
    .keyframe(100, |kf| kf.set("opacity", 1.0))
    .fill_mode(AnimationFillMode::Forwards);  // Keep final state
```

| Fill Mode | Description |
|-----------|-------------|
| `None` | Returns to initial state |
| `Forwards` | Keeps final values |
| `Backwards` | Applies initial values during delay |
| `Both` | Both forwards and backwards |

## Spring Physics

Spring animations create natural, physics-based motion.

```rust
use revue::utils::Spring;

// Create spring from 0 to 100
let mut spring = Spring::new(0.0, 100.0)
    .stiffness(180.0)   // Higher = faster, snappier
    .damping(12.0);     // Higher = less bounce

// In render loop
let dt = ticker.tick();  // Get delta time
spring.update(dt);

let value = spring.value();
let velocity = spring.velocity();

if spring.is_settled() {
    // Animation complete
}
```

### Spring Presets

```rust
let snappy = Spring::snappy();   // Fast, no bounce
let gentle = Spring::gentle();   // Slow, smooth
let bouncy = Spring::bouncy();   // Oscillating
let slow = Spring::slow();       // Very slow
```

### Dynamic Target

```rust
let mut spring = Spring::at(0.0);  // Start at 0, no animation

// Later, animate to new target
spring.set_target(100.0);

// Or set value immediately (no animation)
spring.set_value(50.0);
```

## Staggered Animations

Animate multiple elements with cascading delays.

```rust
use revue::style::animation::Stagger;

// 5 items, 100ms delay between each
let stagger = Stagger::new(5, Duration::from_millis(100));

// Create animations with staggered delays
let animations = stagger.apply(|i| {
    KeyframeAnimation::new(format!("item-{}", i))
        .keyframe(0, |kf| kf.set("opacity", 0.0).set("x", -20.0))
        .keyframe(100, |kf| kf.set("opacity", 1.0).set("x", 0.0))
        .duration(Duration::from_millis(300))
        .easing(easing::ease_out_cubic)
        .fill_mode(AnimationFillMode::Forwards)
});

// Start all animations
for anim in &mut animations {
    anim.start();
}
```

### Custom Stagger Timing

```rust
let stagger = Stagger::new(10, Duration::from_millis(50))
    .start_delay(Duration::from_millis(200))  // Initial delay
    .easing(easing::ease_out);  // Non-linear delay distribution

// Get delay for specific index
let delay = stagger.delay_for(3);  // Delay for 4th item
```

## Animation Groups

Run multiple animations in parallel or sequence.

### Parallel Animations

```rust
use revue::style::animation::AnimationGroup;

let mut group = AnimationGroup::parallel()
    .with_animation(KeyframeAnimation::new("fade")
        .keyframe(0, |kf| kf.set("opacity", 0.0))
        .keyframe(100, |kf| kf.set("opacity", 1.0))
        .duration(Duration::from_millis(300)))
    .with_animation(KeyframeAnimation::new("scale")
        .keyframe(0, |kf| kf.set("scale", 0.8))
        .keyframe(100, |kf| kf.set("scale", 1.0))
        .duration(Duration::from_millis(400)));

group.start();

// Total duration is max of all animations
let total = group.total_duration();  // 400ms
```

### Sequential Animations

```rust
let mut group = AnimationGroup::sequential()
    .with_animation(KeyframeAnimation::new("step1")
        .duration(Duration::from_millis(200)))
    .with_animation(KeyframeAnimation::new("step2")
        .duration(Duration::from_millis(300)));

group.start();

// Total duration is sum of all animations
let total = group.total_duration();  // 500ms
```

## Choreographer

Manage complex animation sequences with named groups.

```rust
use revue::style::animation::Choreographer;

let mut choreo = Choreographer::new();

// Add staggered entrance animation
choreo.add_staggered(
    "list-items",
    5,
    Duration::from_millis(50),
    |i| KeyframeAnimation::new(format!("item-{}", i))
        .keyframe(0, |kf| kf.set("opacity", 0.0).set("y", 10.0))
        .keyframe(100, |kf| kf.set("opacity", 1.0).set("y", 0.0))
        .duration(Duration::from_millis(200))
        .fill_mode(AnimationFillMode::Forwards)
);

// Add a group
choreo.add_group("header", AnimationGroup::parallel()
    .with_animation(widget_animations::fade_in(300))
);

// Start animations
choreo.start("list-items");
choreo.start("header");

// Get values
let opacity = choreo.get_staggered("list-items", 2, "opacity");

// Check completion
if choreo.is_completed("list-items") {
    // All items have animated in
}
```

## Widget Animation Presets

Pre-built animations for common widget effects.

```rust
use revue::style::animation::widget_animations;

// Fade animations
let fade_in = widget_animations::fade_in(300);
let fade_out = widget_animations::fade_out(300);

// Slide animations
let slide_left = widget_animations::slide_in_left(30.0, 400);
let slide_right = widget_animations::slide_in_right(30.0, 400);
let slide_top = widget_animations::slide_in_top(20.0, 400);
let slide_bottom = widget_animations::slide_in_bottom(20.0, 400);

// Scale animations
let scale_up = widget_animations::scale_up(300);
let scale_down = widget_animations::scale_down(300);

// Effects
let bounce = widget_animations::bounce(500);
let shake = widget_animations::shake(300);      // For errors
let pulse = widget_animations::pulse(800);      // Repeating
let blink = widget_animations::blink(1000);     // Repeating
let spin = widget_animations::spin(1000);       // Repeating

// UI-specific
let cursor_blink = widget_animations::cursor_blink();
let toast_enter = widget_animations::toast_enter();
let toast_exit = widget_animations::toast_exit();
let modal_enter = widget_animations::modal_enter();
let modal_exit = widget_animations::modal_exit();
let shimmer = widget_animations::shimmer(1500);  // Loading effect
```

## Utility Animations

The `revue::utils` module provides additional animation primitives.

### Timer

```rust
use revue::utils::Timer;

let mut timer = Timer::from_millis(500);
timer.start();

// Check state
timer.is_running();
timer.is_finished();
timer.progress();        // 0.0 to 1.0
timer.elapsed();         // Duration
timer.remaining();       // Duration

// Control
timer.pause();
timer.resume();
timer.restart();
```

### Ticker

Frame rate tracking for animation loops:

```rust
use revue::utils::Ticker;

let mut ticker = Ticker::new();

// Or with target FPS
let mut ticker = Ticker::with_target_fps(60.0);

loop {
    let dt = ticker.tick();  // Delta time in seconds

    // Update animations with dt
    spring.update(dt);

    // Check frame rate
    let fps = ticker.fps();

    // For frame limiting
    if ticker.should_render() {
        // Render frame
    }
}
```

### Sequence Animations

Step-based sequential animations:

```rust
use revue::utils::Sequence;
use revue::utils::easing::Easing;

let mut seq = Sequence::new()
    .then(Duration::from_millis(200), 0.5)   // Animate to 0.5
    .then_eased(Duration::from_millis(300), 1.0, Easing::OutQuad)
    .delay(Duration::from_millis(100))        // Hold
    .then(Duration::from_millis(200), 0.0)
    .repeat(true);                            // Loop

seq.start();

// Get current value
let value = seq.value();

// Check state
seq.is_running();
seq.is_complete();
```

### Utility Presets

```rust
use revue::utils::animation::presets;

let fade_in = presets::fade_in(300);
let fade_out = presets::fade_out(300);
let pulse = presets::pulse(500);
let blink = presets::blink(1000);
let bounce = presets::bounce(400);
let elastic = presets::elastic(600);
let typewriter = presets::typewriter(50, 20.0);  // 50 chars at 20/sec
```

## Accessibility

Revue respects the user's reduced motion preference:

```rust
use revue::utils::prefers_reduced_motion;

if prefers_reduced_motion() {
    // Skip animations or use instant transitions
}
```

`KeyframeAnimation::start()` automatically checks this preference and skips to the end state if reduced motion is preferred.

## Example: Animated List

```rust
use revue::prelude::*;
use revue::style::animation::{Stagger, KeyframeAnimation, AnimationFillMode, easing};
use std::cell::RefCell;

struct AnimatedList {
    items: Vec<String>,
    animations: RefCell<Vec<KeyframeAnimation>>,
}

impl AnimatedList {
    fn new(items: Vec<String>) -> Self {
        let stagger = Stagger::new(items.len(), Duration::from_millis(50));
        let mut animations = stagger.apply(|i| {
            KeyframeAnimation::new(format!("item-{}", i))
                .keyframe(0, |kf| kf.set("opacity", 0.0).set("x", -20.0))
                .keyframe(100, |kf| kf.set("opacity", 1.0).set("x", 0.0))
                .duration(Duration::from_millis(300))
                .easing(easing::ease_out_cubic)
                .fill_mode(AnimationFillMode::Forwards)
        });

        for anim in &mut animations {
            anim.start();
        }

        Self {
            items,
            animations: RefCell::new(animations),
        }
    }
}

impl View for AnimatedList {
    fn render(&self, ctx: &mut RenderContext) {
        let mut anims = self.animations.borrow_mut();
        let mut list = vstack().gap(0);

        for (i, item) in self.items.iter().enumerate() {
            let opacity = anims[i].get("opacity");
            let x = anims[i].get("x") as usize;

            let prefix = " ".repeat(x.min(20));
            let alpha = (opacity * 255.0) as u8;

            list = list.child(
                Text::new(format!("{}{}", prefix, item))
                    .fg(Color::rgb(alpha, alpha, alpha))
            );
        }

        list.render(ctx);
    }
}
```

## See Also

- [Styling Guide](styling.md) - CSS transitions
- [examples/animations.rs](../../examples/animations.rs) - Comprehensive animation showcase
