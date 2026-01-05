//! Animation system showcase
//!
//! This example demonstrates the various animation capabilities in Revue:
//! - Tween animations with easing functions
//! - CSS @keyframes style animations
//! - Spring physics for natural motion
//! - Staggered animations for lists
//! - Animation choreography
//!
//! Run with: cargo run --example animations

use revue::prelude::*;
use revue::utils::{Spring, Ticker};
use std::cell::RefCell;
use std::time::Duration;

/// Demo mode selector
#[derive(Clone, Copy, PartialEq, Default)]
enum DemoMode {
    #[default]
    Tween,
    Keyframes,
    Spring,
    Stagger,
    Widget,
}

impl DemoMode {
    fn name(&self) -> &'static str {
        match self {
            DemoMode::Tween => "Tween",
            DemoMode::Keyframes => "Keyframes",
            DemoMode::Spring => "Spring",
            DemoMode::Stagger => "Stagger",
            DemoMode::Widget => "Presets",
        }
    }

    fn next(&self) -> Self {
        match self {
            DemoMode::Tween => DemoMode::Keyframes,
            DemoMode::Keyframes => DemoMode::Spring,
            DemoMode::Spring => DemoMode::Stagger,
            DemoMode::Stagger => DemoMode::Widget,
            DemoMode::Widget => DemoMode::Tween,
        }
    }

    fn prev(&self) -> Self {
        match self {
            DemoMode::Tween => DemoMode::Widget,
            DemoMode::Keyframes => DemoMode::Tween,
            DemoMode::Spring => DemoMode::Keyframes,
            DemoMode::Stagger => DemoMode::Spring,
            DemoMode::Widget => DemoMode::Stagger,
        }
    }
}

/// Animation state that needs interior mutability for rendering
struct AnimationState {
    ticker: Ticker,
    tween: Tween,
    keyframe_anim: KeyframeAnimation,
    spring: Spring,
    stagger_anims: Vec<KeyframeAnimation>,
    widget_anim: KeyframeAnimation,
}

/// Main animation showcase widget
struct AnimationShowcase {
    mode: DemoMode,
    tween_easing_idx: usize,
    spring_target: f64,
    widget_preset_idx: usize,
    // Animation state wrapped in RefCell for interior mutability
    state: RefCell<AnimationState>,
}

const EASING_NAMES: &[&str] = &[
    "linear",
    "ease_in",
    "ease_out",
    "ease_in_out",
    "bounce_out",
    "elastic_out",
    "back_out",
];

const WIDGET_PRESETS: &[&str] = &[
    "fade_in",
    "fade_out",
    "slide_in_left",
    "slide_in_right",
    "scale_up",
    "bounce",
    "shake",
    "pulse",
];

impl AnimationShowcase {
    fn new() -> Self {
        // Create tween animation
        let mut tween =
            Tween::new(0.0, 100.0, Duration::from_millis(1000)).easing(easing::ease_out);
        tween.start();

        // Create keyframe animation
        let mut keyframe_anim = KeyframeAnimation::new("demo")
            .keyframe(0, |kf| kf.set("x", 0.0).set("opacity", 0.0))
            .keyframe(30, |kf| kf.set("x", 50.0).set("opacity", 1.0))
            .keyframe(70, |kf| kf.set("x", 30.0).set("opacity", 1.0))
            .keyframe(100, |kf| kf.set("x", 40.0).set("opacity", 1.0))
            .duration(Duration::from_millis(1500))
            .easing(easing::ease_out)
            .fill_mode(AnimationFillMode::Forwards);
        keyframe_anim.start();

        // Create spring
        let spring = Spring::new(0.0, 50.0).stiffness(180.0).damping(12.0);

        // Create stagger animations
        let stagger = Stagger::new(5, Duration::from_millis(100));
        let mut stagger_anims = stagger.apply(|i| {
            KeyframeAnimation::new(format!("item-{}", i))
                .keyframe(0, |kf| kf.set("x", -20.0).set("opacity", 0.0))
                .keyframe(100, |kf| kf.set("x", 0.0).set("opacity", 1.0))
                .duration(Duration::from_millis(300))
                .easing(easing::ease_out_cubic)
                .fill_mode(AnimationFillMode::Forwards)
        });
        for anim in &mut stagger_anims {
            anim.start();
        }

        // Create widget preset animation
        let mut widget_anim = widget_animations::fade_in(500);
        widget_anim.start();

        Self {
            mode: DemoMode::default(),
            tween_easing_idx: 2, // ease_out
            spring_target: 50.0,
            widget_preset_idx: 0,
            state: RefCell::new(AnimationState {
                ticker: Ticker::new(),
                tween,
                keyframe_anim,
                spring,
                stagger_anims,
                widget_anim,
            }),
        }
    }

    fn restart_current(&mut self) {
        let mut state = self.state.borrow_mut();
        match self.mode {
            DemoMode::Tween => {
                state.tween.reset();
                state.tween.start();
            }
            DemoMode::Keyframes => {
                state.keyframe_anim.reset();
                state.keyframe_anim.start();
            }
            DemoMode::Spring => {
                // Toggle spring target
                self.spring_target = if self.spring_target > 25.0 { 0.0 } else { 50.0 };
                state.spring.set_target(self.spring_target);
            }
            DemoMode::Stagger => {
                for anim in &mut state.stagger_anims {
                    anim.reset();
                    anim.start();
                }
            }
            DemoMode::Widget => {
                state.widget_anim.reset();
                state.widget_anim.start();
            }
        }
    }

    fn change_easing(&mut self, delta: i32) {
        let len = EASING_NAMES.len();
        self.tween_easing_idx =
            ((self.tween_easing_idx as i32 + delta).rem_euclid(len as i32)) as usize;

        let easing_fn = match self.tween_easing_idx {
            0 => easing::linear,
            1 => easing::ease_in,
            2 => easing::ease_out,
            3 => easing::ease_in_out,
            4 => easing::bounce_out,
            5 => easing::elastic_out,
            6 => easing::back_out,
            _ => easing::linear,
        };

        let mut state = self.state.borrow_mut();
        state.tween = Tween::new(0.0, 100.0, Duration::from_millis(1000)).easing(easing_fn);
        state.tween.start();
    }

    fn change_widget_preset(&mut self, delta: i32) {
        let len = WIDGET_PRESETS.len();
        self.widget_preset_idx =
            ((self.widget_preset_idx as i32 + delta).rem_euclid(len as i32)) as usize;

        let mut state = self.state.borrow_mut();
        state.widget_anim = match self.widget_preset_idx {
            0 => widget_animations::fade_in(500),
            1 => widget_animations::fade_out(500),
            2 => widget_animations::slide_in_left(30.0, 500),
            3 => widget_animations::slide_in_right(30.0, 500),
            4 => widget_animations::scale_up(500),
            5 => widget_animations::bounce(500),
            6 => widget_animations::shake(300),
            7 => widget_animations::pulse(800),
            _ => widget_animations::fade_in(500),
        };
        state.widget_anim.start();
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        match key {
            Key::Left | Key::Char('h') => {
                self.mode = self.mode.prev();
                true
            }
            Key::Right | Key::Char('l') => {
                self.mode = self.mode.next();
                true
            }
            Key::Char(' ') | Key::Enter => {
                self.restart_current();
                true
            }
            Key::Up | Key::Char('k') => {
                match self.mode {
                    DemoMode::Tween => self.change_easing(1),
                    DemoMode::Widget => self.change_widget_preset(1),
                    _ => {}
                }
                true
            }
            Key::Down | Key::Char('j') => {
                match self.mode {
                    DemoMode::Tween => self.change_easing(-1),
                    DemoMode::Widget => self.change_widget_preset(-1),
                    _ => {}
                }
                true
            }
            _ => false,
        }
    }

    fn render_progress_bar(&self, value: f32, width: u16, color: Color) -> impl View {
        let filled = ((value / 100.0) * width as f32) as usize;
        let empty = width as usize - filled;

        let bar = format!("{}{}", "█".repeat(filled), "░".repeat(empty));

        Text::new(bar).fg(color)
    }

    fn render_tween_demo(&self) -> impl View {
        let mut state = self.state.borrow_mut();
        let value = state.tween.value();
        let progress = state.tween.progress();
        let anim_state = if state.tween.is_completed() {
            "Completed"
        } else {
            "Running"
        };

        vstack()
            .gap(1)
            .child(Text::new("Tween Animation").bold().fg(Color::CYAN))
            .child(Text::new(format!(
                "Easing: {} (↑/↓ to change)",
                EASING_NAMES[self.tween_easing_idx]
            )))
            .child(Text::new(format!("Value: {:.1}", value)))
            .child(Text::new(format!("Progress: {:.0}%", progress * 100.0)))
            .child(Text::new(format!("State: {}", anim_state)).fg(Color::YELLOW))
            .child(self.render_progress_bar(value, 40, Color::GREEN))
            .child(Text::muted("Press SPACE to restart"))
    }

    fn render_keyframes_demo(&self) -> impl View {
        let mut state = self.state.borrow_mut();
        let x = state.keyframe_anim.get("x");
        let opacity = state.keyframe_anim.get("opacity");
        let progress = state.keyframe_anim.progress();

        // Visual representation with opacity
        let alpha = (opacity * 255.0) as u8;
        let color = Color::rgb(100, 200, alpha);

        let offset = x as usize;
        let indicator = format!("{}●", " ".repeat(offset.min(50)));

        vstack()
            .gap(1)
            .child(Text::new("CSS @keyframes Animation").bold().fg(Color::CYAN))
            .child(Text::new("Keyframes: 0% → 30% → 70% → 100%"))
            .child(Text::new(format!("X: {:.1}, Opacity: {:.2}", x, opacity)))
            .child(Text::new(format!("Progress: {:.0}%", progress * 100.0)))
            .child(Text::new(indicator).fg(color))
            .child(Text::muted("Press SPACE to restart"))
    }

    fn render_spring_demo(&self) -> impl View {
        let mut state = self.state.borrow_mut();
        // Update spring physics
        let dt = state.ticker.tick();
        state.spring.update(dt);

        let value = state.spring.value();
        let velocity = state.spring.velocity();
        let settled = state.spring.is_settled();

        let offset = value as usize;
        let indicator = format!("{}◆", " ".repeat(offset.min(50)));

        vstack()
            .gap(1)
            .child(Text::new("Spring Physics").bold().fg(Color::CYAN))
            .child(Text::new(format!("Target: {:.0}", self.spring_target)))
            .child(Text::new(format!("Value: {:.2}", value)))
            .child(Text::new(format!("Velocity: {:.2}", velocity)))
            .child(
                Text::new(format!(
                    "Status: {}",
                    if settled { "Settled" } else { "Moving" }
                ))
                .fg(if settled { Color::GREEN } else { Color::YELLOW }),
            )
            .child(Text::new(indicator).fg(Color::MAGENTA))
            .child(Text::muted("Press SPACE to toggle target"))
    }

    fn render_stagger_demo(&self) -> impl View {
        let mut state = self.state.borrow_mut();
        let mut items = vstack().gap(0);

        for (i, anim) in state.stagger_anims.iter_mut().enumerate() {
            let x = anim.get("x");
            let opacity = anim.get("opacity");

            let offset = ((x + 20.0) / 2.0) as usize; // Normalize from -20..0 to 0..10
            let prefix = " ".repeat(offset.min(20));
            let alpha = (opacity * 100.0) as u8;

            let text = format!("{}Item {}", prefix, i + 1);
            let color = Color::rgb(100, 200, 100 + alpha);

            items = items.child(Text::new(text).fg(color));
        }

        vstack()
            .gap(1)
            .child(Text::new("Staggered Animation").bold().fg(Color::CYAN))
            .child(Text::new("5 items with 100ms delay each"))
            .child(items)
            .child(Text::muted("Press SPACE to restart"))
    }

    fn render_widget_presets_demo(&self) -> impl View {
        let mut state = self.state.borrow_mut();
        let preset_name = WIDGET_PRESETS[self.widget_preset_idx];

        // Get animation values
        let opacity = state.widget_anim.get("opacity");
        let x = state.widget_anim.get("x");
        let y = state.widget_anim.get("y");
        let scale = state.widget_anim.get("scale");

        let offset_x = (x + 30.0).max(0.0) as usize;
        let prefix = " ".repeat(offset_x.min(40));

        let alpha = (opacity.clamp(0.0, 1.0) * 255.0) as u8;
        let color = Color::rgb(alpha, alpha, 255);

        let demo_text = format!("{}[Demo Widget]", prefix);

        vstack()
            .gap(1)
            .child(Text::new("Widget Animation Presets").bold().fg(Color::CYAN))
            .child(Text::new(format!(
                "Preset: {} (↑/↓ to change)",
                preset_name
            )))
            .child(
                Text::new(format!(
                    "opacity: {:.2}, x: {:.1}, y: {:.1}, scale: {:.2}",
                    opacity, x, y, scale
                ))
                .fg(Color::rgb(128, 128, 128)),
            )
            .child(Text::new(demo_text).fg(color))
            .child(Text::muted("Press SPACE to restart"))
    }

    fn render_mode_tabs(&self) -> impl View {
        let modes = [
            DemoMode::Tween,
            DemoMode::Keyframes,
            DemoMode::Spring,
            DemoMode::Stagger,
            DemoMode::Widget,
        ];

        let mut tabs = hstack().gap(2);

        for mode in modes {
            let name = mode.name();
            let text = if mode == self.mode {
                Text::new(format!("[{}]", name)).bold().fg(Color::CYAN)
            } else {
                Text::new(format!(" {} ", name)).fg(Color::rgb(128, 128, 128))
            };
            tabs = tabs.child(text);
        }

        tabs
    }
}

impl View for AnimationShowcase {
    fn render(&self, ctx: &mut RenderContext) {
        let demo_content: Box<dyn View> = match self.mode {
            DemoMode::Tween => Box::new(self.render_tween_demo()),
            DemoMode::Keyframes => Box::new(self.render_keyframes_demo()),
            DemoMode::Spring => Box::new(self.render_spring_demo()),
            DemoMode::Stagger => Box::new(self.render_stagger_demo()),
            DemoMode::Widget => Box::new(self.render_widget_presets_demo()),
        };

        let view = vstack()
            .gap(1)
            .child(
                Border::panel().title("Animation Showcase").child(
                    vstack()
                        .gap(2)
                        .child(self.render_mode_tabs())
                        .child(demo_content),
                ),
            )
            .child(
                Border::single()
                    .title("Controls")
                    .child(vstack().child(Text::new(
                        "[←/→] Switch demo  [↑/↓] Change option  [Space] Restart  [q] Quit",
                    ))),
            );

        view.render(ctx);
    }

    fn meta(&self) -> WidgetMeta {
        WidgetMeta::new("AnimationShowcase")
    }
}

fn main() -> Result<()> {
    println!("🎬 Animation Showcase");
    println!("Demonstrating Revue's animation capabilities\n");

    let mut app = App::builder().build();
    let showcase = AnimationShowcase::new();

    app.run(showcase, |event, showcase, _app| match event {
        Event::Key(key_event) => showcase.handle_key(&key_event.key),
        _ => false,
    })
}
