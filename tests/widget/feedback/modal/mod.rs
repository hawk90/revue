//! Modal widget tests

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, View};

mod view;

/// Render `view` into a `w` x `h` area at `(x, y)` of a `bw` x `bh` buffer.
fn draw_at(view: &dyn View, bw: u16, bh: u16, area: Rect) -> Buffer {
    let mut buffer = Buffer::new(bw, bh);
    let mut ctx = RenderContext::new(&mut buffer, area);
    view.render(&mut ctx);
    buffer
}

/// Render `view` into a fresh `w` x `h` buffer.
fn draw(view: &dyn View, w: u16, h: u16) -> Buffer {
    draw_at(view, w, h, Rect::new(0, 0, w, h))
}

/// Row `y` as text, continuation cells left out.
fn row(buffer: &Buffer, y: u16) -> String {
    (0..buffer.width())
        .map(|x| buffer.get(x, y).unwrap().symbol)
        .filter(|&c| c != '\0')
        .collect()
}

mod snapshots {
    use revue::prelude::*;
    use revue::testing::{Pilot, TestApp};

    #[test]
    fn test_modal_basic() {
        let mut modal = Modal::new()
            .title("Confirm Action")
            .content("Are you sure you want to proceed?")
            .ok();
        modal.show();

        let mut app = TestApp::new(modal);
        let mut pilot = Pilot::new(&mut app);

        pilot.snapshot("modal_basic");
    }
}

/// Replacing the buttons with fewer left the selected index past them:
/// nothing was highlighted and Enter returned that index (#975).
#[test]
fn modal_selection_follows_new_buttons() {
    use revue::event::Key;
    use revue::widget::{Modal, ModalButton};
    let mut m = Modal::new().yes_no_cancel();
    m.next_button();
    m.next_button();
    let mut m = m.buttons(vec![ModalButton::new("Only")]);
    assert_eq!(m.handle_key(&Key::Enter), Some(0)); // Some(2) before
}
