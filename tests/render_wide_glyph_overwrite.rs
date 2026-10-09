//! Writing over half of a wide glyph clears the other half, so the buffer
//! never holds a glyph the terminal has erased, or a continuation with no
//! glyph (#867)

use revue::layout::Rect;
use revue::render::{Buffer, Cell};

fn row(buffer: &Buffer) -> String {
    (0..buffer.width())
        .map(|x| match buffer.get(x, 0).unwrap().symbol {
            '\0' => '_', // continuation
            c => c,
        })
        .collect()
}

fn with_glyph() -> Buffer {
    let mut buffer = Buffer::new(4, 1);
    buffer.put_str(0, 0, "日ab");
    assert_eq!(row(&buffer), "日_ab");
    buffer
}

#[test]
fn a_cell_over_the_continuation_blanks_the_glyph() {
    let mut buffer = with_glyph();
    buffer.set(1, 0, Cell::new('│'));
    assert_eq!(row(&buffer), " │ab");
}

#[test]
fn a_cell_over_the_glyph_blanks_its_continuation() {
    let mut buffer = with_glyph();
    buffer.set(0, 0, Cell::new('x'));
    assert_eq!(row(&buffer), "x ab");
}

#[test]
fn a_string_over_the_continuation_blanks_the_glyph() {
    let mut buffer = with_glyph();
    buffer.put_str(1, 0, "zz");
    assert_eq!(row(&buffer), " zzb");
}

#[test]
fn a_wide_glyph_over_a_wide_glyph_shifted_by_one() {
    let mut buffer = Buffer::new(4, 1);
    buffer.put_str(1, 0, "日a");
    buffer.put_str(0, 0, "月");
    // 月 covers 0-1; the old 日 at 1-2 loses its continuation at 2
    assert_eq!(row(&buffer), "月_ a");
}

#[test]
fn a_wide_glyph_written_in_place_stays_whole() {
    let mut buffer = with_glyph();
    buffer.put_str(0, 0, "月");
    assert_eq!(row(&buffer), "月_ab");
}

#[test]
fn a_fill_over_half_a_glyph_blanks_the_other_half() {
    let mut buffer = Buffer::new(6, 1);
    buffer.put_str(0, 0, "日ab日");
    assert_eq!(row(&buffer), "日_ab日_");
    buffer.fill(1, 0, 3, 1, Cell::new('='));
    assert_eq!(row(&buffer), " ===日_");
    buffer.fill(3, 0, 2, 1, Cell::new('-'));
    assert_eq!(row(&buffer), " ==-- ");
}

#[test]
fn clearing_a_region_over_half_a_glyph_blanks_the_other_half() {
    let mut buffer = with_glyph();
    buffer.clear_regions(&[Rect::new(1, 0, 1, 1)]);
    assert_eq!(row(&buffer), "  ab");
}
