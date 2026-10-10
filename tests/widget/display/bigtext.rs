//! BigText widget tests

use revue::layout::Rect;
use revue::render::{Buffer, Modifier};
use revue::style::Color;
use revue::utils::figlet::FigletFont;
use revue::widget::traits::{RenderContext, View};
use revue::widget::{bigtext, h1, h2, h3, BigText};

fn render(bt: &BigText) -> Buffer {
    let mut buffer = Buffer::new(80, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 80, 10));
    bt.render(&mut ctx);
    buffer
}

fn has_content(buffer: &Buffer) -> bool {
    (0..buffer.height())
        .any(|y| (0..buffer.width()).any(|x| buffer.get(x, y).unwrap().symbol != ' '))
}

#[test]
fn test_bigtext_creation() {
    let bt = BigText::new("Hello", 1);
    assert_eq!(bt.get_text(), "Hello");
    assert_eq!(bt.get_tier(), 1);
    assert!(bt.height() > 0);
}

#[test]
fn test_tier_clamping() {
    assert_eq!(BigText::new("Test", 10).get_tier(), 6);
    assert_eq!(BigText::new("Test", 0).get_tier(), 1);
    assert_eq!(BigText::new("Test", 3).tier(9).get_tier(), 6);
}

#[test]
fn test_helper_functions() {
    assert_eq!(h1("Header 1").get_tier(), 1);
    assert_eq!(h2("Header 2").get_tier(), 2);
    assert_eq!(h3("Header 3").get_tier(), 3);
    let b = bigtext("Header 4", 4);
    assert_eq!((b.get_text(), b.get_tier()), ("Header 4", 4));
}

#[test]
fn test_builder_pattern() {
    let bt = BigText::h1("Test")
        .fg(Color::CYAN)
        .bg(Color::BLACK)
        .figlet_font(FigletFont::Slant)
        .force_figlet(true);

    assert_eq!(bt.get_fg(), Some(Color::CYAN));
    assert_eq!(bt.get_bg(), Some(Color::BLACK));
    assert_eq!(bt.get_figlet_font(), FigletFont::Slant);
    assert!(bt.get_force_figlet());
}

#[test]
fn test_render_figlet() {
    let bt = BigText::h1("Hi")
        .force_figlet(true)
        .fg(Color::CYAN)
        .bg(Color::BLACK);
    let buffer = render(&bt);

    assert!(has_content(&buffer), "Figlet should render some content");
    // Drawn cells carry the colors and bold
    let cell = (0..10)
        .flat_map(|y| (0..80).map(move |x| (x, y)))
        .map(|(x, y)| buffer.get(x, y).unwrap())
        .find(|c| c.symbol != ' ')
        .unwrap();
    assert_eq!(cell.fg, Some(Color::CYAN));
    assert_eq!(cell.bg, Some(Color::BLACK));
    assert!(cell.modifier.contains(Modifier::BOLD));
    // Nothing below the font height
    let h = bt.height();
    assert!((0..80).all(|x| buffer.get(x, h).unwrap().symbol == ' '));
}

#[test]
fn test_font_for_tier() {
    // H1 uses the configured font, H2 slant, H3 small, H4-H6 mini
    assert_eq!(BigText::h1("T").get_font_for_tier(), FigletFont::Block);
    assert_eq!(
        BigText::h1("T")
            .figlet_font(FigletFont::Slant)
            .get_font_for_tier(),
        FigletFont::Slant
    );
    assert_eq!(BigText::h2("T").get_font_for_tier(), FigletFont::Slant);
    assert_eq!(BigText::h3("T").get_font_for_tier(), FigletFont::Small);
    for tier in 4..=6 {
        assert_eq!(
            BigText::new("T", tier).get_font_for_tier(),
            FigletFont::Mini
        );
    }

    // Heights follow the font: block is tallest, mini the shortest
    let height = |b: BigText| b.force_figlet(true).height();
    assert!(height(BigText::h1("T")) >= height(BigText::h2("T")));
    assert!(height(BigText::h2("T")) >= height(BigText::h3("T")));
    assert!(height(BigText::h3("T")) >= height(BigText::h6("T")));
    assert_eq!(height(BigText::h4("T")), height(BigText::h6("T")));
}

#[test]
fn test_empty_text() {
    let buffer = render(&BigText::h1("").force_figlet(true));
    assert!(!has_content(&buffer));
}

#[test]
fn test_text_sizing_rendering() {
    let mut buffer = Buffer::new(80, 10);
    let area = Rect::new(0, 0, 80, 10);

    let bt = BigText::h1("Test");

    // Call render_text_sizing directly (bypasses the is_supported check)
    bt.test_render_text_sizing(&mut RenderContext::new(&mut buffer, area));

    // Verify that a sequence was registered
    assert_eq!(buffer.sequences().len(), 1);

    // Verify the sequence contains OSC 66 marker
    let seq = &buffer.sequences()[0];
    assert!(seq.contains("\x1b]66;"), "Should contain OSC 66 marker");
    assert!(seq.contains("Test"), "Should contain the text");

    // Verify the first cell has a sequence_id
    let first_cell = buffer.get(0, 0).unwrap();
    assert!(
        first_cell.sequence_id.is_some(),
        "First cell should have sequence_id"
    );

    // Verify continuation cells
    let cont_cell = buffer.get(1, 0).unwrap();
    assert!(
        cont_cell.is_continuation(),
        "Adjacent cells should be continuations"
    );
}

#[test]
fn test_text_sizing_height() {
    // With text sizing the heading is 2 rows; without, the figlet height
    let bt = BigText::h1("Test");
    let figlet = BigText::h1("Test").force_figlet(true).height();
    assert!(bt.height() == 2 || bt.height() == figlet);
}
