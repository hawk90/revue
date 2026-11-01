//! A builder color wins over the stylesheet (P1, `docs/guides/styling.md`), for
//! every widget with `.fg()` or `.bg()`.
//!
//! `DropZone::fg` colors the placeholder only, and the stylesheet's `color` also
//! reaches its border, which has its own builder; `dropzone/view.rs` tests those.
//! `MaskedInput::bg` paints the field and not the brackets around it, which
//! `form/masked_input` tests.

use revue::render::Buffer;
use revue::style::Color;
use revue::testing::PipelineHarness;
use revue::widget::traits::View;
use revue::widget::*;

const CSS: &str = "#a { color: #fe0102; background: #03fd04; }";
const CSS_FG: Color = Color::rgb(254, 1, 2);
const CSS_BG: Color = Color::rgb(3, 253, 4);
const FG: Color = Color::rgb(1, 2, 3);
const BG: Color = Color::rgb(4, 5, 6);

fn drawn(view: &dyn View, mut each: impl FnMut(&Buffer)) {
    let mut harness = PipelineHarness::with_css(CSS, 40, 12).dom_from_render(true);
    harness.draw(&view);
    each(harness.buffer());
}

fn cells(buffer: &Buffer) -> impl Iterator<Item = revue::render::Cell> + '_ {
    (0..12).flat_map(move |y| (0..40).map(move |x| *buffer.get(x, y).unwrap()))
}

macro_rules! fg_cases {
    ($($name:literal => $view:expr),* $(,)?) => {
        vec![$(($name, Box::new($view.fg(FG).element_id("a")) as Box<dyn View>)),*]
    };
}

macro_rules! bg_cases {
    ($($name:literal => $view:expr),* $(,)?) => {
        vec![$(($name, Box::new($view.bg(BG).element_id("a")) as Box<dyn View>)),*]
    };
}

#[test]
fn a_builder_foreground_wins_over_the_stylesheet() {
    let items = || vec!["one".to_string(), "two".to_string()];
    let cases = fg_cases![
        "Text" => Text::new("text"),
        "Input" => Input::new().value("text"),
        "Select" => Select::new().options(items()),
        "Combobox" => Combobox::new().options(items()).value("one"),
        "TextArea" => TextArea::new().content("text"),
        "RadioGroup" => RadioGroup::new(items()),
        "SelectionList" => SelectionList::new(items()),
        "MaskedInput" => MaskedInput::password().show_strength(false).value("text"),
        "Badge" => Badge::new("text"),
        "Spinner" => Spinner::new(),
        "BigText" => BigText::new("A", 3),
        "Digits" => Digits::new(12),
        "Avatar" => avatar("John Doe"),
        "Link" => Link::new("u").text("text"),
        "Tree" => Tree::new().node(TreeNode::new("one")).node(TreeNode::new("two")).selected(1),
        "Tabs" => Tabs::new().tab("one").tab("two"),
        "Border" => Border::single(),
        "Sidebar" => Sidebar::new().items(vec![SidebarItem::new("a", "one"), SidebarItem::new("b", "two")]),
        "BarChart" => BarChart::new().bar("a", 1.0),
        "Sparkline" => Sparkline::new(vec![1.0, 2.0]),
        "Tooltip" => Tooltip::new("text").visible(true),
        "StatusBar" => StatusBar::new().left(StatusSection::new("a")),
    ];
    let mut failures = Vec::new();
    for (name, view) in cases {
        drawn(&*view, |buffer| {
            let red = cells(buffer).filter(|c| c.fg == Some(CSS_FG)).count();
            let own = cells(buffer).filter(|c| c.fg == Some(FG)).count();
            if red > 0 || own == 0 {
                failures.push(format!(
                    "{name}: {red} cells in the stylesheet's, {own} in the builder's"
                ));
            }
        });
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn a_builder_background_wins_over_the_stylesheet() {
    let items = || vec!["one".to_string(), "two".to_string()];
    let cases = bg_cases![
        "Text" => Text::new("text"),
        "Input" => Input::new().value("text"),
        "Select" => Select::new().options(items()),
        "Combobox" => Combobox::new().options(items()).value("one"),
        "TextArea" => TextArea::new().content("text"),
        "SelectionList" => SelectionList::new(items()),
        "Badge" => Badge::new("text"),
        "BigText" => BigText::new("A", 3),
        "Digits" => Digits::new(12),
        "Avatar" => avatar("John Doe"),
        "Link" => Link::new("u").text("text"),
        "Tree" => Tree::new().node(TreeNode::new("one")).node(TreeNode::new("two")).selected(1),
        "Tabs" => Tabs::new().tab("one").tab("two"),
        "Border" => Border::single(),
        "Sidebar" => Sidebar::new().items(vec![SidebarItem::new("a", "one"), SidebarItem::new("b", "two")]),
        "DropZone" => DropZone::new("text"),
        "Sparkline" => Sparkline::new(vec![1.0, 2.0]),
        "Tooltip" => Tooltip::new("text").visible(true),
        "StatusBar" => StatusBar::new().left(StatusSection::new("a")),
    ];
    let mut failures = Vec::new();
    for (name, view) in cases {
        drawn(&*view, |buffer| {
            // The stylesheet's may fill under what the widget leaves bare; a cell
            // the widget paints in a color keeps the builder's.
            let own = cells(buffer).filter(|c| c.bg == Some(BG)).count();
            let drawn_on_css = cells(buffer)
                .filter(|c| c.bg == Some(CSS_BG) && c.symbol != ' ')
                .count();
            if own == 0 || drawn_on_css > 0 {
                failures.push(format!("{name}: {own} cells in the builder's, {drawn_on_css} glyphs on the stylesheet's"));
            }
        });
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn a_histogram_fill_color_wins_over_the_stylesheet() {
    let view = Histogram::new(&[1.0, 2.0, 2.0, 3.0])
        .fill_color(FG)
        .element_id("a");
    drawn(&view, |buffer| {
        assert!(cells(buffer).any(|c| c.fg == Some(FG)));
        assert!(!cells(buffer).any(|c| c.fg == Some(CSS_FG) && c.symbol == '█'));
    });
}

#[test]
fn without_a_builder_color_the_stylesheet_still_reaches_them() {
    let bars = BarChart::new().bar("a", 1.0).element_id("a");
    drawn(&bars, |buffer| {
        assert!(cells(buffer).any(|c| c.fg == Some(CSS_FG) && c.symbol == '█'));
    });
    let histogram = Histogram::new(&[1.0, 2.0, 2.0, 3.0]).element_id("a");
    drawn(&histogram, |buffer| {
        assert!(cells(buffer).any(|c| c.fg == Some(CSS_FG)));
    });
}

#[cfg(feature = "qrcode")]
#[test]
fn a_qr_code_takes_the_builders_colors_and_else_the_stylesheets() {
    let built = QrCodeWidget::new("x").fg(FG).bg(BG).element_id("a");
    drawn(&built, |buffer| {
        assert!(cells(buffer).any(|c| c.fg == Some(FG) && c.bg == Some(BG)));
        assert!(!cells(buffer).any(|c| c.fg == Some(CSS_FG)));
        assert!(!cells(buffer).any(|c| c.bg == Some(CSS_BG) && c.symbol != ' '));
    });
    let plain = QrCodeWidget::new("x").element_id("a");
    drawn(&plain, |buffer| {
        assert!(cells(buffer).any(|c| c.bg == Some(CSS_BG) && c.symbol != ' '));
    });
}
