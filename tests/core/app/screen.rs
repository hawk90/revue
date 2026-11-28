//! A modal or popup screen is drawn over the screen below it; a fullscreen
//! screen hides everything below.

use revue::core::app::{Screen, ScreenConfig, ScreenId, ScreenManager, ScreenMode};
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::RenderContext;

/// A screen that writes `mark` into its first cell, or into every cell when
/// `fill` is set.
struct Marked {
    id: &'static str,
    mode: ScreenMode,
    mark: char,
    fill: bool,
}

impl Screen for Marked {
    fn id(&self) -> ScreenId {
        self.id.into()
    }

    fn config(&self) -> ScreenConfig {
        ScreenConfig::default().mode(self.mode)
    }

    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let (w, h) = if self.fill {
            (area.width, area.height)
        } else {
            (1, 1)
        };
        for y in 0..h {
            for x in 0..w {
                ctx.set(x, y, revue::render::Cell::new(self.mark));
            }
        }
    }
}

fn manager() -> ScreenManager {
    let mut manager = ScreenManager::new();
    let screens = [
        ("base", ScreenMode::Fullscreen, 'b', true),
        ("modal", ScreenMode::Modal, 'M', false),
        ("popup", ScreenMode::Popup, 'P', false),
        ("page", ScreenMode::Fullscreen, 'F', false),
    ];
    for (id, mode, mark, fill) in screens {
        manager.register(id, move || {
            Box::new(Marked {
                id,
                mode,
                mark,
                fill,
            })
        });
    }
    manager.push("base");
    manager
}

/// The symbols at (0, 0) and (5, 2).
fn draw(manager: &ScreenManager) -> (char, char) {
    let mut buffer = Buffer::new(10, 4);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 4));
    manager.render(&mut ctx);
    (
        buffer.get(0, 0).unwrap().symbol,
        buffer.get(5, 2).unwrap().symbol,
    )
}

#[test]
fn a_modal_is_drawn_over_the_screen_below() {
    let mut manager = manager();
    manager.push("modal");
    assert_eq!(draw(&manager), ('M', 'b'), "the screen below was hidden");
}

#[test]
fn a_popup_is_drawn_over_the_screen_below() {
    let mut manager = manager();
    manager.push("popup");
    assert_eq!(draw(&manager), ('P', 'b'), "the screen below was hidden");
}

#[test]
fn modals_stack_over_the_same_screen() {
    let mut manager = manager();
    manager.push("modal");
    manager.push("popup");
    assert_eq!(draw(&manager), ('P', 'b'));
}

#[test]
fn a_fullscreen_screen_hides_everything_below() {
    let mut manager = manager();
    manager.push("modal");
    manager.push("page");
    assert_eq!(draw(&manager), ('F', ' '));
}

#[test]
fn popping_back_to_a_modal_shows_the_screen_below_it_again() {
    let mut manager = manager();
    manager.push("modal");
    manager.push("page");
    manager.pop();
    assert_eq!(draw(&manager), ('M', 'b'));
}

// Screen transitions are stored but not drawn, and are deprecated (#845)

/// A screen with the fade `ScreenConfig::modal` sets
struct Fading;

impl Screen for Fading {
    fn id(&self) -> ScreenId {
        "fading".into()
    }

    fn config(&self) -> ScreenConfig {
        ScreenConfig::modal()
    }

    fn render(&self, ctx: &mut RenderContext) {
        ctx.set(0, 0, revue::render::Cell::new('M'));
    }
}

/// Every cell of a 10x4 draw
fn cells(manager: &ScreenManager) -> Vec<(char, Option<revue::style::Color>)> {
    let mut buffer = Buffer::new(10, 4);
    manager.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 4)));
    let mut cells = Vec::new();
    for y in 0..4 {
        for x in 0..10 {
            let cell = buffer.get(x, y).unwrap();
            cells.push((cell.symbol, cell.fg));
        }
    }
    cells
}

/// A modal's fade draws nothing: mid-transition the screen is drawn the
/// same as after it, while the deprecated timer still runs for the fade's
/// duration (an app may read it, so it stays as it was), and a pop takes
/// the screen away at once
#[test]
#[allow(deprecated)]
fn a_modal_fade_is_not_drawn_and_only_its_timer_runs() {
    let time = revue::testing::MockTime::new();
    let _clock = time.install();
    let mut manager = manager();
    manager.register("fading", || Box::new(Fading));
    manager.push("fading");

    assert!(manager.is_transitioning());
    assert_eq!(manager.transition_progress(), Some(0.0));
    let during = cells(&manager);
    assert_eq!(during[0], ('M', None));
    assert!(during[1..].iter().all(|&cell| cell == ('b', None)));

    time.advance_ms(100);
    assert_eq!(manager.transition_progress(), Some(0.5));
    assert_eq!(cells(&manager), during);

    time.advance_ms(100);
    manager.update();
    assert!(!manager.is_transitioning());
    assert_eq!(
        cells(&manager),
        during,
        "the end of the fade changes nothing"
    );

    manager.push("fading");
    manager.pop();
    assert_eq!(manager.current_id(), Some("base".into()));
    assert!(cells(&manager).iter().all(|&cell| cell == ('b', None)));
}
