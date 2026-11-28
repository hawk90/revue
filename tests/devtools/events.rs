//! The event logger cuts event details to the row with one `…` cell (#1611).

use revue::devtools::{DevToolsConfig, EventLogger, EventType};
use revue::layout::Rect;
use revue::render::Buffer;

/// Draw one event with `details` into a 30-column area at (3, 1); the
/// details column is 30 - 20 = 10 wide and starts 4 columns in.
fn draw(details: &str) -> Buffer {
    let mut logger = EventLogger::new();
    logger.log(EventType::KeyPress, details);
    let mut buffer = Buffer::new(40, 12);
    logger.render_content(
        &mut buffer,
        Rect::new(3, 1, 30, 10),
        &DevToolsConfig::default(),
    );
    buffer
}

/// The symbols of the event row (area y + 3) from column `from` to `to` of
/// the area (exclusive).
fn symbols(buffer: &Buffer, from: u16, to: u16) -> String {
    (from..to)
        .map(|x| buffer.get(3 + x, 4).unwrap().symbol)
        .collect()
}

#[test]
fn long_details_end_in_one_ellipsis_cell() {
    let buffer = draw("abcdefghijklmnop");
    // 9 characters, `…` in the 10th column, then the space before the age
    assert_eq!(symbols(&buffer, 4, 15), "abcdefghi… ");
}

#[test]
fn details_that_fit_are_not_cut() {
    let buffer = draw("abcdefghij");
    assert_eq!(symbols(&buffer, 4, 15), "abcdefghij ");
}

#[test]
fn wide_details_end_in_one_ellipsis_cell() {
    // 12 columns cut to 4 Hangul (8 columns) and `…`: 9 of the 10 columns
    let buffer = draw("가나다라마바");
    for (i, ch) in ['가', '나', '다', '라'].into_iter().enumerate() {
        assert_eq!(buffer.get(3 + 4 + i as u16 * 2, 4).unwrap().symbol, ch);
    }
    assert_eq!(symbols(&buffer, 12, 14), "… ");
}
