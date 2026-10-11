//! Every public widget whose fields allow it implements `Clone`.
//!
//! The widgets left out hold child views (`Box<dyn View>`), boxed callbacks or
//! other non-`Clone` state; see `docs/refactor/findings-widget-clone.md`.

use revue::widget::*;

fn assert_clone<T: Clone>() {}

#[test]
fn widgets_implement_clone() {
    assert_clone::<Accordion>();
    assert_clone::<AiStream>();
    assert_clone::<Alert>();
    assert_clone::<Autocomplete>();
    assert_clone::<Avatar>();
    assert_clone::<Badge>();
    assert_clone::<BarChart>();
    assert_clone::<BigText>();
    assert_clone::<BoxPlot>();
    assert_clone::<BrailleCanvas<fn(&mut BrailleContext)>>();
    assert_clone::<Breadcrumb>();
    assert_clone::<Button>();
    assert_clone::<Calendar>();
    assert_clone::<Callout>();
    assert_clone::<CandleChart>();
    assert_clone::<Canvas<fn(&mut DrawContext)>>();
    assert_clone::<Chart>();
    assert_clone::<Checkbox>();
    assert_clone::<CodeEditor>();
    assert_clone::<Collapsible>();
    assert_clone::<ColorPicker>();
    assert_clone::<CommandPalette>();
    assert_clone::<CsvViewer>();
    assert_clone::<DateTimePicker>();
    assert_clone::<DebugOverlay<Text>>();
    assert_clone::<Diagram>();
    #[cfg(feature = "diff")]
    assert_clone::<DiffViewer>();
    assert_clone::<Digits>();
    assert_clone::<Divider>();
    assert_clone::<DropZone>();
    assert_clone::<EmptyState>();
    assert_clone::<FilePicker>();
    assert_clone::<FileTree>();
    assert_clone::<Tree>();
    assert_clone::<ScreenStack>();
    assert_clone::<Screen>();
    assert_clone::<Form>();
    assert_clone::<FormFieldWidget>();
    assert_clone::<Gauge>();
    assert_clone::<GradientBox>();
    assert_clone::<HSplit>();
    assert_clone::<HeatMap>();
    assert_clone::<Histogram>();
    assert_clone::<HttpClient>();
    #[cfg(feature = "image")]
    assert_clone::<Image>();
    assert_clone::<Input>();
    assert_clone::<revue::core::app::Inspector>();
    assert_clone::<JsonViewer>();
    assert_clone::<Link>();
    assert_clone::<List<String>>();
    assert_clone::<LogViewer>();
    #[cfg(feature = "markdown")]
    assert_clone::<Markdown>();
    #[cfg(feature = "markdown")]
    assert_clone::<MarkdownPresentation>();
    assert_clone::<MaskedInput>();
    assert_clone::<MultiSelect>();
    assert_clone::<NotificationCenter>();
    assert_clone::<NumberInput>();
    assert_clone::<OptionList>();
    assert_clone::<Pagination>();
    assert_clone::<PieChart>();
    assert_clone::<Popover>();
    assert_clone::<Presentation>();
    assert_clone::<Progress>();
    #[cfg(feature = "qrcode")]
    assert_clone::<QrCodeWidget>();
    assert_clone::<RadioGroup>();
    assert_clone::<RangePicker>();
    assert_clone::<Rating>();
    assert_clone::<Resizable>();
    assert_clone::<RichLog>();
    assert_clone::<RichText>();
    assert_clone::<RichTextEditor>();
    assert_clone::<ScatterChart>();
    assert_clone::<ScrollView>();
    assert_clone::<SearchBar>();
    assert_clone::<Select>();
    assert_clone::<SelectionList>();
    assert_clone::<Skeleton>();
    assert_clone::<Slider>();
    assert_clone::<Sparkline>();
    assert_clone::<Spinner>();
    assert_clone::<Splitter>();
    assert_clone::<StatusBar>();
    assert_clone::<StatusIndicator>();
    assert_clone::<Stepper>();
    assert_clone::<Stopwatch>();
    assert_clone::<Streamline>();
    assert_clone::<Switch>();
    assert_clone::<Table>();
    assert_clone::<Tabs>();
    assert_clone::<Tag>();
    assert_clone::<Terminal>();
    assert_clone::<Text>();
    assert_clone::<TextArea>();
    assert_clone::<ThemePicker>();
    assert_clone::<TimeSeries>();
    assert_clone::<Timeline>();
    assert_clone::<Timer>();
    assert_clone::<Toast>();
    assert_clone::<ToastQueue>();
    assert_clone::<Tooltip>();
    assert_clone::<Transition>();
    assert_clone::<TransitionGroup>();
    assert_clone::<VSplit>();
    assert_clone::<VirtualList<String>>();
    assert_clone::<Waveline>();
}

// ─── What a clone shares ────────────────────────────────────────────────────

use revue::layout::Rect;
use revue::patterns::form::FormState;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;

fn screen(view: &dyn View) -> Vec<String> {
    let mut buffer = Buffer::new(30, 10);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 30, 10));
    view.render(&mut ctx);
    (0..10)
        .map(|y| {
            (0..30)
                .filter_map(|x| buffer.get(x, y).map(|c| c.symbol))
                .collect()
        })
        .collect()
}

#[test]
fn a_cloned_slider_is_independent() {
    let mut original = Slider::new().value(40.0);
    let copy = original.clone();
    assert_eq!(screen(&copy), screen(&original));

    original.set_value(80.0);
    assert_eq!(copy.get_value(), 40.0);
    assert_eq!(original.get_value(), 80.0);
}

#[test]
fn a_cloned_color_picker_is_independent() {
    let mut original = ColorPicker::new().color(Color::rgb(10, 20, 30));
    let copy = original.clone();
    assert_eq!(screen(&copy), screen(&original));

    original.set_color(Color::rgb(200, 100, 50));
    assert_eq!(copy.get_color(), Color::rgb(10, 20, 30));
}

#[test]
fn a_cloned_calendar_is_independent() {
    let mut original = Calendar::new(2026, 10).selected(Date::new(2026, 10, 7));
    let copy = original.clone();
    assert_eq!(screen(&copy), screen(&original));

    original.select(Date::new(2026, 10, 20));
    original.next_month();
    assert_eq!(copy.get_selected(), Some(Date::new(2026, 10, 7)));
    assert_ne!(screen(&copy), screen(&original));
}

/// A `Form` is a view over a `FormState`, whose fields are reactive signals:
/// a clone is a second view over the same form, not a copy of its values.
#[test]
fn a_cloned_form_shares_its_form_state() {
    let state = FormState::new().field("name", |f| f.label("Name")).build();
    let original = Form::new(state);
    let copy = original.clone();

    original.form_state().set_value("name", "Ada");
    assert_eq!(copy.form_state().value("name").as_deref(), Some("Ada"));
}

/// A cloned `Tree` moves its own selection but calls the same `on_select`.
#[test]
fn a_cloned_tree_has_its_own_selection_and_shares_on_select() {
    use std::cell::Cell;
    use std::rc::Rc;

    let calls = Rc::new(Cell::new(0));
    let seen = calls.clone();
    let original = Tree::new()
        .node(TreeNode::new("a"))
        .node(TreeNode::new("b"))
        .on_select(move |_| seen.set(seen.get() + 1));
    let mut copy = original.clone();

    copy.select_next();
    assert_eq!(copy.selected_index(), 1);
    assert_eq!(
        original.selected_index(),
        0,
        "the clone moved the original's selection"
    );

    copy.toggle_select();
    assert!(calls.get() > 0, "the clone lost the on_select callback");
}
