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
