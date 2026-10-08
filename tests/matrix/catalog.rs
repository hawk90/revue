//! One entry per public widget: a name and a factory that builds a
//! representative instance fed with the case's content.
//!
//! Where a widget takes text, it gets `content.text`; where it takes a list
//! (options, rows, data points, children), it gets `content.items` or numbers
//! derived from them, so the same content drives 0 / 1 / many items. Where a
//! widget has a `focused` builder, the factory passes the case's focus.
//!
//! Not in the catalog: `DockArea`/`DockManager` (not reachable from outside
//! the crate - `widget::layout` is private and they are not re-exported).

use revue::core::app::{DeclarativeRouter, Inspector as AppInspector, WidgetInfo};
use revue::devtools::{
    DevTools, DevToolsTab, EventType as DevEventType, RenderEvent, StateEntry, StateValue,
};
use revue::layout::Rect;
use revue::patterns::form::FormState;
use revue::style::Color;
use revue::widget::*;

use super::Content;

/// A catalog entry.
pub struct Entry {
    pub name: &'static str,
    pub build: fn(&Content, bool) -> Box<dyn View>,
}

fn e(name: &'static str, build: fn(&Content, bool) -> Box<dyn View>) -> Entry {
    Entry { name, build }
}

// ─── Content helpers ────────────────────────────────────────────────────────

fn t(c: &Content) -> String {
    c.text.clone()
}

/// Numbers derived from the items: one per item, spread over 0..100.
fn nums(c: &Content) -> Vec<f64> {
    c.items
        .iter()
        .enumerate()
        .map(|(i, s)| ((i * 37 + s.len() * 11) % 101) as f64)
        .collect()
}

/// A scalar derived from the text: 0 for "", up to 100.
fn val(c: &Content) -> f64 {
    (c.text.chars().count() % 101) as f64
}

/// The items as CSV/log-style lines.
fn lines(c: &Content) -> String {
    c.items.join("\n")
}

fn date() -> Date {
    Date::new(2026, 10, 7)
}

/// A wrapper so `DevTools` (which draws into a buffer directly) joins the matrix.
struct DevToolsView(DevTools);

impl View for DevToolsView {
    fn render(&self, ctx: &mut RenderContext) {
        self.0.render(ctx.buffer, ctx.area);
    }
}

fn devtools(c: &Content, tab: DevToolsTab) -> Box<dyn View> {
    let mut d = DevTools::new();
    d.set_visible(true);
    d.set_tab(tab);
    let root = d.inspector_mut().add_root(t(c));
    for item in &c.items {
        d.inspector_mut().add_child(root, item.clone());
        d.state_mut()
            .add(StateEntry::new(item.clone(), StateValue::String(t(c))));
        d.events_mut().log(DevEventType::KeyPress, item.clone());
    }
    d.styles_mut().set_widget(t(c), Some(t(c)));
    d.styles_mut().add_class(t(c));
    let profiler = d.profiler_mut();
    profiler.start_recording();
    for item in &c.items {
        profiler.start_frame();
        profiler.record_render(RenderEvent::new(
            item.clone(),
            std::time::Duration::from_micros(250),
        ));
        profiler.end_frame();
    }
    Box::new(DevToolsView(d))
}

// ─── The catalog ────────────────────────────────────────────────────────────

pub fn catalog() -> Vec<Entry> {
    // Only feature-gated entries push to it.
    #[allow(unused_mut)]
    let mut v = vec![
        // Display
        e("Alert", |c, _| {
            Box::new(Alert::new(t(c)).title(t(c)).dismissible(true))
        }),
        e("Avatar", |c, _| Box::new(Avatar::new(t(c)).online())),
        e("Badge", |c, _| Box::new(Badge::new(t(c)))),
        e("BigText", |c, _| Box::new(BigText::new(t(c), 1))),
        e("Callout", |c, _| {
            Box::new(Callout::new(t(c)).title(t(c)).collapsible(true))
        }),
        e("Digits", |c, _| {
            Box::new(Digits::from_float(val(c) * 1234.5, 2))
        }),
        e("Divider", |c, _| Box::new(Divider::new().label(t(c)))),
        e("DividerVertical", |c, _| {
            Box::new(Divider::vertical().label(t(c)))
        }),
        e("EmptyState", |c, _| {
            Box::new(EmptyState::new(t(c)).description(t(c)).action(t(c)))
        }),
        e("Gauge", |c, _| {
            Box::new(Gauge::new().percent(val(c)).label(t(c)).title(t(c)))
        }),
        e("GradientBox", |_, _| Box::new(GradientBox::new(20, 5))),
        e("Link", |c, f| {
            Box::new(Link::new(t(c)).text(t(c)).focused(f))
        }),
        e("Progress", |c, _| {
            Box::new(Progress::new(val(c) as f32 / 100.0))
        }),
        e("RichLog", |c, _| {
            let mut log = RichLog::new();
            for item in &c.items {
                log.info(format!("{} {item}", c.text));
            }
            Box::new(log)
        }),
        e("RichText", |c, _| Box::new(RichText::markup(&c.text))),
        e("Skeleton", |c, _| {
            Box::new(Skeleton::new().lines(c.items.len() as u16).paragraph())
        }),
        e("Spinner", |c, _| Box::new(Spinner::new().label(t(c)))),
        e("StatusIndicator", |c, _| {
            Box::new(StatusIndicator::online().label(t(c)))
        }),
        e("Tag", |c, _| Box::new(Tag::new(t(c)).closable())),
        e("Text", |c, _| Box::new(Text::new(t(c)))),
        // Layout
        e("Accordion", |c, _| {
            Box::new(
                Accordion::new().sections(
                    c.items
                        .iter()
                        .map(|s| {
                            AccordionSection::new(s.clone())
                                .content(t(c))
                                .expanded(true)
                        })
                        .collect(),
                ),
            )
        }),
        e("Border", |c, _| {
            Box::new(Border::rounded().title(t(c)).child(Text::new(t(c))))
        }),
        e("Card", |c, _| {
            Box::new(
                Card::new()
                    .title(t(c))
                    .subtitle(t(c))
                    .body(Text::new(t(c)))
                    .footer(Button::new(t(c))),
            )
        }),
        e("Collapsible", |c, _| {
            Box::new(Collapsible::new(t(c)).content(t(c)).expanded(true))
        }),
        e("Grid", |c, _| {
            let mut g = Grid::new().cols(2).gap(1);
            for s in &c.items {
                g = g.child(Text::new(s.clone()));
            }
            Box::new(g)
        }),
        e("HSplit", |_, _| Box::new(HSplit::new(0.5))),
        e("Layers", |c, _| {
            Box::new(
                Layers::new()
                    .child(Text::new(t(c)))
                    .child(Button::new(t(c))),
            )
        }),
        e("Positioned", |c, _| {
            Box::new(Positioned::new(Text::new(t(c))).at(3, 2))
        }),
        e("Resizable", |_, _| Box::new(Resizable::new(20, 5))),
        e("ScreenStack", |c, _| {
            let mut s = ScreenStack::new().register("home", |screen, ctx| {
                if let Some(text) = screen.get_data("text") {
                    ctx.draw_text(0, 0, text, Color::WHITE);
                }
            });
            s.push(Screen::new("home").data("text", t(c)));
            Box::new(s)
        }),
        e("ScrollView", |c, _| {
            Box::new(
                ScrollView::new()
                    .content_height(c.items.len() as u16)
                    .scroll_offset(1),
            )
        }),
        e("Sidebar", |c, _| {
            Box::new(
                Sidebar::new().header(t(c)).footer(t(c)).items(
                    c.items
                        .iter()
                        .enumerate()
                        .map(|(i, s)| SidebarItem::new(format!("i{i}"), s.clone()))
                        .collect(),
                ),
            )
        }),
        e("Splitter", |c, _| {
            Box::new(
                Splitter::new().panes(
                    c.items
                        .iter()
                        .enumerate()
                        .map(|(i, _)| Pane::new(format!("p{i}")))
                        .collect(),
                ),
            )
        }),
        e("Stack", |c, _| {
            let mut s = vstack().gap(1);
            for item in &c.items {
                s = s.child(Text::new(item.clone()));
            }
            Box::new(s.child(Button::new(t(c))))
        }),
        e("HStack", |c, _| {
            let mut s = hstack().gap(1);
            for item in &c.items {
                s = s.child(Text::new(item.clone()));
            }
            Box::new(s.child(Button::new(t(c))))
        }),
        e("Tabs", |c, _| Box::new(Tabs::new().tabs(c.items.clone()))),
        e("VSplit", |_, _| Box::new(VSplit::new(0.5))),
        // Input
        e("Autocomplete", |c, _| {
            Box::new(Autocomplete::new().suggestions(c.items.clone()).value(t(c)))
        }),
        e("Button", |c, f| Box::new(Button::new(t(c)).focused(f))),
        e("Checkbox", |c, f| {
            Box::new(Checkbox::new(t(c)).checked(true).focused(f))
        }),
        e("ColorPicker", |_, _| Box::new(ColorPicker::new())),
        e("Combobox", |c, _| {
            let mut cb = Combobox::new().options(c.items.clone()).value(t(c));
            cb.open_dropdown();
            Box::new(cb)
        }),
        e("Input", |c, f| {
            Box::new(Input::new().value(t(c)).placeholder(t(c)).focused(f))
        }),
        e("NumberInput", |c, f| {
            Box::new(NumberInput::new().value(val(c)).prefix(t(c)).focused(f))
        }),
        e("RadioGroup", |c, f| {
            Box::new(RadioGroup::new(c.items.clone()).focused(f))
        }),
        e("Rating", |c, _| {
            Box::new(Rating::new().value(val(c) as f32 / 20.0).label(t(c)))
        }),
        e("SearchBar", |c, f| {
            let mut s = SearchBar::new().placeholder(t(c));
            if f {
                s.focus();
            }
            Box::new(s)
        }),
        e("Select", |c, f| {
            Box::new(
                Select::new()
                    .options(c.items.clone())
                    .placeholder(t(c))
                    .focused(f),
            )
        }),
        e("SelectionList", |c, f| {
            Box::new(SelectionList::new(c.items.clone()).title(t(c)).focused(f))
        }),
        e("Slider", |c, f| {
            Box::new(Slider::new().value(val(c)).label(t(c)).focused(f))
        }),
        e("Stepper", |c, _| {
            Box::new(
                Stepper::new()
                    .steps(c.items.iter().map(|s| Step::new(s.clone())).collect())
                    .current(1),
            )
        }),
        e("Switch", |c, f| {
            Box::new(Switch::new().label(t(c)).focused(f))
        }),
        e("TextArea", |c, f| {
            Box::new(TextArea::new().content(t(c)).line_numbers(true).focused(f))
        }),
        // Pickers, forms, navigation
        e("Breadcrumb", |c, _| {
            let mut b = Breadcrumb::new();
            for s in &c.items {
                b = b.push(s.clone());
            }
            Box::new(b)
        }),
        e("Calendar", |_, f| {
            Box::new(Calendar::new(2026, 10).selected(date()).focused(f))
        }),
        e("CommandPalette", |c, _| {
            let mut p = CommandPalette::new().title(t(c)).commands(
                c.items
                    .iter()
                    .map(|s| Command::new(s.clone(), s.clone()))
                    .collect(),
            );
            p.show();
            p.set_query(t(c));
            Box::new(p)
        }),
        e("DateTimePicker", |_, f| {
            Box::new(DateTimePicker::new().focused(f))
        }),
        e("DropZone", |c, _| Box::new(drop_zone(t(c)))),
        e("FilePicker", |c, _| Box::new(FilePicker::new().title(t(c)))),
        e("Form", |c, _| {
            let label = t(c);
            let state = FormState::new()
                .field("name", |f| f.label(label.clone()))
                .field("other", |f| f.label(label))
                .build();
            state.set_value("name", t(c));
            Box::new(Form::new(state))
        }),
        e("FormFieldWidget", |c, _| {
            Box::new(
                FormFieldWidget::new("name")
                    .placeholder(t(c))
                    .helper_text(t(c)),
            )
        }),
        e("MaskedInput", |c, f| {
            Box::new(MaskedInput::password().value(t(c)).label(t(c)).focused(f))
        }),
        e("MultiSelect", |c, _| {
            Box::new(
                MultiSelect::new()
                    .options(c.items.clone())
                    .selected_indices(vec![0]),
            )
        }),
        e("OptionList", |c, f| {
            let mut o = OptionList::new().title(t(c));
            for s in &c.items {
                o = o.option(s.clone(), t(c));
            }
            Box::new(o.focused(f))
        }),
        e("Pagination", |c, f| {
            let p = Pagination::new(c.items.len() as u16).current(1);
            Box::new(if f { p.focused() } else { p })
        }),
        e("RangePicker", |_, _| Box::new(RangePicker::new())),
        e("RichTextEditor", |c, f| {
            Box::new(RichTextEditor::new().content(t(c)).focused(f))
        }),
        e("SortableList", |c, _| {
            Box::new(SortableList::new(c.items.clone()))
        }),
        e("ThemePicker", |_, _| Box::new(ThemePicker::new())),
        e("RouterLink", |c, _| {
            Box::new(revue::core::app::Link::new("/", t(c)))
        }),
        e("DeclarativeRouter", |_, _| {
            Box::new(DeclarativeRouter::new().route("/", "home", |_, ctx| {
                ctx.draw_text(0, 0, "home", Color::WHITE);
            }))
        }),
        // Data
        e("CsvViewer", |c, _| {
            Box::new(CsvViewer::from_content(&format!(
                "name,value\n{}",
                lines(c)
            )))
        }),
        e("DataGrid", |c, _| {
            Box::new(
                DataGrid::new()
                    .column(GridColumn::new("name", t(c)))
                    .column(GridColumn::new("value", "Value"))
                    .data(c.items.iter().map(|s| vec![s.clone(), t(c)]).collect()),
            )
        }),
        e("FileTree", |c, _| {
            Box::new(FileTree::new().root(vec![FileEntry::directory(t(c), "/d")
                    .expanded(true)
                    .children(
                        c.items
                            .iter()
                            .map(|s| FileEntry::file(s.clone(), format!("/d/{s}")))
                            .collect(),
                    )]))
        }),
        e("JsonViewer", |c, _| {
            Box::new(JsonViewer::from_content(&format!(
                "{{\"text\": {:?}, \"items\": {:?}}}",
                c.text, c.items
            )))
        }),
        e("List", |c, _| {
            Box::new(List::new(c.items.clone()).selected(1))
        }),
        e("LogViewer", |c, _| {
            let mut l = LogViewer::new().show_line_numbers(true);
            l.load(&format!("{}\n{}", t(c), lines(c)));
            Box::new(l)
        }),
        e("Table", |c, _| {
            Box::new(
                Table::new(vec![Column::new(t(c)), Column::new("Value")])
                    .rows(c.items.iter().map(|s| vec![s.clone(), t(c)]).collect())
                    .selected(1),
            )
        }),
        e("Timeline", |c, _| {
            Box::new(
                Timeline::new().events(
                    c.items
                        .iter()
                        .map(|s| {
                            TimelineEvent::new(s.clone())
                                .description(t(c))
                                .timestamp(t(c))
                        })
                        .collect(),
                ),
            )
        }),
        e("Timer", |_, _| Box::new(Timer::countdown(90))),
        e("Stopwatch", |c, _| Box::new(Stopwatch::new().title(t(c)))),
        e("Tree", |c, _| {
            Box::new(
                Tree::new().nodes(
                    c.items
                        .iter()
                        .map(|s| {
                            TreeNode::new(s.clone())
                                .expanded(true)
                                .child(TreeNode::leaf(t(c)))
                        })
                        .collect(),
                ),
            )
        }),
        e("VirtualList", |c, _| {
            Box::new(VirtualList::new(c.items.clone()).selected(1))
        }),
        // Charts
        e("BarChart", |c, _| {
            let mut b = BarChart::new();
            for (s, n) in c.items.iter().zip(nums(c)) {
                b = b.bar(s.clone(), n);
            }
            Box::new(b)
        }),
        e("BarChartHorizontal", |c, _| {
            let mut b = BarChart::new().horizontal();
            for (s, n) in c.items.iter().zip(nums(c)) {
                b = b.bar(s.clone(), n);
            }
            Box::new(b)
        }),
        e("BoxPlot", |c, _| {
            Box::new(BoxPlot::new().title(t(c)).group(t(c), &nums(c)))
        }),
        e("CandleChart", |c, _| {
            Box::new(
                CandleChart::new(
                    nums(c)
                        .iter()
                        .map(|&n| Candle::with_volume(n, n + 5.0, n - 5.0, n + 1.0, n * 10.0))
                        .collect(),
                )
                .title(t(c)),
            )
        }),
        e("Chart", |c, _| {
            Box::new(
                Chart::new()
                    .title(t(c))
                    .series(Series::new(t(c)).data_y(&nums(c))),
            )
        }),
        e("HeatMap", |c, _| {
            let n = nums(c);
            Box::new(HeatMap::new(n.chunks(5).map(|r| r.to_vec()).collect()).title(t(c)))
        }),
        e("Histogram", |c, _| {
            Box::new(Histogram::new(&nums(c)).title(t(c)))
        }),
        e("PieChart", |c, _| {
            let mut p = PieChart::new().title(t(c));
            for (s, n) in c.items.iter().zip(nums(c)) {
                p = p.slice(s.clone(), n + 1.0);
            }
            Box::new(p)
        }),
        e("ScatterChart", |c, _| {
            let pts: Vec<(f64, f64)> = nums(c)
                .iter()
                .enumerate()
                .map(|(i, &n)| (i as f64, n))
                .collect();
            Box::new(
                ScatterChart::new()
                    .title(t(c))
                    .series(ScatterSeries::new(t(c)).points(&pts)),
            )
        }),
        e("Sparkline", |c, _| Box::new(Sparkline::new(nums(c)))),
        e("Streamline", |c, _| {
            Box::new(
                Streamline::new()
                    .title(t(c))
                    .layer(StreamLayer::new(t(c)).data(nums(c)))
                    .layer(StreamLayer::new("b").data(nums(c))),
            )
        }),
        e("TimeSeries", |c, _| {
            let pts: Vec<(u64, f64)> = nums(c)
                .iter()
                .enumerate()
                .map(|(i, &n)| (1_700_000_000 + i as u64 * 60, n))
                .collect();
            Box::new(
                TimeSeries::new()
                    .title(t(c))
                    .series(TimeSeriesData::new(t(c)).points(pts)),
            )
        }),
        e("Waveline", |c, _| {
            Box::new(Waveline::new(nums(c)).label(t(c)))
        }),
        e("Diagram", |c, _| {
            let mut d = Diagram::new().title(t(c));
            for (i, s) in c.items.iter().enumerate() {
                d = d.node(DiagramNode::new(format!("n{i}"), s.clone()));
            }
            Box::new(d)
        }),
        e("Canvas", |_, _| {
            Box::new(canvas(|ctx| {
                let (w, h) = (ctx.width(), ctx.height());
                ctx.rect(0, 0, w, h, None);
                ctx.hline(0, h / 2, w + 10, '-', None);
                ctx.vline(w / 2, 0, h + 10, '|', None);
            }))
        }),
        e("BrailleCanvas", |_, _| {
            Box::new(braille_canvas(|ctx| {
                let (w, h) = (ctx.width(), ctx.height());
                for i in 0..w + 10 {
                    ctx.set(i, i * h / (w + 1), Color::WHITE);
                }
            }))
        }),
        // Feedback and overlays
        e("ContextMenu", |c, _| {
            let mut m = ContextMenu::new().items(
                c.items
                    .iter()
                    .map(|s| MenuItem::new(s.clone()).shortcut(t(c)))
                    .collect(),
            );
            m.show(1, 1);
            Box::new(m)
        }),
        e("MenuBar", |c, _| {
            Box::new(MenuBar::new().menu(
                Menu::new(t(c)).items(c.items.iter().map(|s| MenuItem::new(s.clone())).collect()),
            ))
        }),
        e("Modal", |c, _| {
            let mut m = Modal::new().title(t(c)).content(t(c)).ok_cancel();
            m.show();
            Box::new(m)
        }),
        e("NotificationCenter", |c, f| {
            let mut n = NotificationCenter::new().focused(f);
            for s in &c.items {
                n.push(Notification::new(s.clone()));
            }
            Box::new(n)
        }),
        e("Popover", |c, _| {
            Box::new(Popover::new(t(c)).title(t(c)).anchor(2, 2).open(true))
        }),
        e("StatusBar", |c, _| {
            Box::new(
                StatusBar::new()
                    .left_text(t(c))
                    .center_text(t(c))
                    .right_text(t(c))
                    .keys(
                        c.items
                            .iter()
                            .map(|s| KeyHint::new("k", s.clone()))
                            .collect(),
                    ),
            )
        }),
        e("Toast", |c, _| Box::new(Toast::new(t(c)))),
        e("ToastQueue", |c, _| {
            let mut q = ToastQueue::new();
            for s in &c.items {
                q.info(s.clone());
            }
            Box::new(q)
        }),
        e("Tooltip", |c, _| {
            Box::new(Tooltip::new(t(c)).title(t(c)).anchor(2, 2).visible(true))
        }),
        e("ErrorBoundary", |c, _| {
            Box::new(ErrorBoundary::new().child(Text::new(t(c))))
        }),
        e("Transition", |c, _| {
            Box::new(AnimationTransition::new(t(c)))
        }),
        e("TransitionGroup", |c, _| {
            Box::new(TransitionGroup::new(c.items.clone()))
        }),
        e("ZenMode", |c, _| Box::new(ZenMode::new(Text::new(t(c))))),
        // Developer
        e("AiStream", |c, _| Box::new(AiStream::new().content(t(c)))),
        e("CodeEditor", |c, f| {
            Box::new(
                CodeEditor::new()
                    .content(t(c))
                    .line_numbers(true)
                    .focused(f),
            )
        }),
        e("DebugOverlay", |c, _| {
            Box::new(DebugOverlay::wrap(Text::new(t(c))).visible(true))
        }),
        e("HttpClient", |c, _| {
            Box::new(HttpClient::new().url(t(c)).body(t(c)))
        }),
        e("Inspector", |c, _| {
            let mut i = AppInspector::new();
            i.set_root(WidgetInfo::new(t(c), Rect::new(0, 0, 10, 3)));
            i.show();
            Box::new(i)
        }),
        e("Presentation", |c, _| {
            Box::new(
                Presentation::new().title(t(c)).slides(
                    c.items
                        .iter()
                        .map(|s| Slide::new(s.clone()).line(t(c)))
                        .collect(),
                ),
            )
        }),
        e("Terminal", |c, _| {
            let mut term = Terminal::new(20, 5);
            term.write(&c.text);
            Box::new(term)
        }),
        e("DevToolsInspector", |c, _| {
            devtools(c, DevToolsTab::Inspector)
        }),
        e("DevToolsState", |c, _| devtools(c, DevToolsTab::State)),
        e("DevToolsStyles", |c, _| devtools(c, DevToolsTab::Styles)),
        e("DevToolsEvents", |c, _| devtools(c, DevToolsTab::Events)),
        e("DevToolsProfiler", |c, _| {
            devtools(c, DevToolsTab::Profiler)
        }),
        e("DevToolsTimeTravel", |c, _| {
            devtools(c, DevToolsTab::TimeTravel)
        }),
    ];

    #[cfg(feature = "diff")]
    v.push(e("DiffViewer", |c, _| {
        Box::new(DiffViewer::new().compare(lines(c), format!("{}\n{}", t(c), lines(c))))
    }));
    #[cfg(feature = "image")]
    v.push(e("Image", |_, _| {
        Box::new(Image::from_rgb(vec![200; 4 * 4 * 3], 4, 4))
    }));
    #[cfg(feature = "markdown")]
    {
        v.push(e("Markdown", |c, _| {
            Box::new(Markdown::new(format!(
                "# {}\n\n- {}\n\n```\n{}\n```",
                t(c),
                lines(c),
                t(c)
            )))
        }));
        v.push(e("MarkdownPresentation", |c, _| {
            Box::new(MarkdownPresentation::new(format!(
                "# {}\n\n---\n\n# {}",
                t(c),
                lines(c)
            )))
        }));
    }
    #[cfg(feature = "qrcode")]
    v.push(e("QrCodeWidget", |c, _| Box::new(QrCodeWidget::new(t(c)))));
    #[cfg(feature = "sysinfo")]
    v.push(e("ProcessMonitor", |_, _| Box::new(ProcessMonitor::new())));

    v
}
