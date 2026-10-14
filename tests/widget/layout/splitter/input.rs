//! Splitter divider resizing and keyboard handling

use revue::event::Key;
use revue::layout::Rect;
use revue::widget::{pane, splitter, Splitter};

fn widths(s: &Splitter) -> Vec<u16> {
    // 42 columns: 41 for the panes plus a one-column divider. An odd pane
    // budget keeps every ratio used below away from a rounding boundary.
    s.pane_areas(Rect::new(0, 0, 42, 10))
        .iter()
        .map(|(_, r)| r.width)
        .collect()
}

fn two_panes() -> Splitter {
    splitter().pane(pane("left")).pane(pane("right"))
}

#[test]
fn test_splitter_resize_without_active_divider_does_nothing() {
    let mut s = two_panes();
    s.resize(10);
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_resize_moves_divider() {
    let mut s = two_panes();
    s.start_resize(0);
    s.resize(10);
    assert_eq!(widths(&s), vec![24, 17]);
    s.resize(-20);
    assert_eq!(widths(&s), vec![16, 25]);
}

#[test]
fn test_splitter_resize_ratio_is_clamped() {
    let mut s = two_panes();
    s.start_resize(0);
    s.resize(100);
    // Ratios clamp at 0.9 / 0.1
    assert_eq!(widths(&s), vec![36, 5]);
}

#[test]
fn test_splitter_stop_resize() {
    let mut s = two_panes();
    s.start_resize(0);
    s.stop_resize();
    s.resize(10);
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_start_resize_rejects_out_of_range_divider() {
    let mut s = two_panes();
    // Two panes have a single divider (index 0)
    s.start_resize(1);
    s.resize(10);
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_start_resize_without_panes() {
    let mut s = splitter();
    s.start_resize(0);
    s.resize(10);
    assert!(s.pane_areas(Rect::new(0, 0, 10, 10)).is_empty());
    assert!(!s.handle_key(&Key::Right));
}

#[test]
fn test_splitter_arrow_keys_need_active_divider() {
    let mut s = two_panes();
    assert!(!s.handle_key(&Key::Right));
    assert!(!s.handle_key(&Key::Enter));
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_keys_resize_in_steps_of_five() {
    let mut s = two_panes();
    s.start_resize(0);
    assert!(s.handle_key(&Key::Right));
    assert_eq!(widths(&s), vec![22, 19]);
    assert!(s.handle_key(&Key::Char('l')));
    assert!(s.handle_key(&Key::Down));
    assert!(s.handle_key(&Key::Char('j')));
    assert_eq!(widths(&s), vec![28, 13]);
    assert!(s.handle_key(&Key::Left));
    assert!(s.handle_key(&Key::Char('h')));
    assert!(s.handle_key(&Key::Up));
    assert!(s.handle_key(&Key::Char('k')));
    assert_eq!(widths(&s), vec![20, 21]);
}

#[test]
fn test_splitter_enter_and_escape_end_resize() {
    for key in [Key::Enter, Key::Escape] {
        let mut s = two_panes();
        s.start_resize(0);
        assert!(s.handle_key(&key));
        assert!(!s.handle_key(&Key::Right));
        assert_eq!(widths(&s), vec![20, 21]);
    }
}

#[test]
fn test_splitter_tab_focuses_next_pane() {
    let mut s = two_panes();
    assert_eq!(s.focused(), Some("left"));
    assert!(s.handle_key(&Key::Tab));
    assert_eq!(s.focused(), Some("right"));
    assert!(s.handle_key(&Key::Tab));
    assert_eq!(s.focused(), Some("left"));
}

#[test]
fn test_splitter_focus_skips_collapsed_pane() {
    let mut s = splitter()
        .pane(pane("a"))
        .pane(pane("b").collapsible())
        .pane(pane("c"));
    s.toggle_pane(1);
    s.focus_next();
    assert_eq!(s.focused(), Some("c"));
    s.focus_prev();
    assert_eq!(s.focused(), Some("a"));
}

#[test]
fn test_splitter_toggle_pane_hides_it_from_layout() {
    let mut s = splitter()
        .pane(pane("a"))
        .pane(pane("b").collapsible())
        .pane(pane("c"));
    s.toggle_pane(1);
    let ids: Vec<String> = s
        .pane_areas(Rect::new(0, 0, 41, 10))
        .into_iter()
        .map(|(id, _)| id)
        .collect();
    assert_eq!(ids, vec!["a", "c"]);
    s.toggle_pane(1);
    assert_eq!(s.pane_areas(Rect::new(0, 0, 41, 10)).len(), 3);
    // Out-of-range index is ignored
    s.toggle_pane(9);
    assert_eq!(s.pane_areas(Rect::new(0, 0, 41, 10)).len(), 3);
}

#[test]
fn test_splitter_vertical_uses_the_same_keys() {
    let mut s = splitter().vertical().pane(pane("top")).pane(pane("bottom"));
    let heights = |s: &Splitter| -> Vec<u16> {
        s.pane_areas(Rect::new(0, 0, 10, 42))
            .iter()
            .map(|(_, r)| r.height)
            .collect()
    };
    s.start_resize(0);
    assert!(s.handle_key(&Key::Down));
    assert_eq!(heights(&s), vec![22, 19]);
    assert!(s.handle_key(&Key::Up));
    assert_eq!(heights(&s), vec![20, 21]);
}

#[test]
fn test_splitter_other_keys_are_not_taken() {
    let mut s = two_panes();
    s.start_resize(0);
    for key in [Key::BackTab, Key::Char('x'), Key::Home, Key::Char('H')] {
        assert!(!s.handle_key(&key), "{key:?}");
    }
    assert_eq!(widths(&s), vec![20, 21]);
    assert_eq!(s.focused(), Some("left"));
}

mod model {
    //! Focus, the active divider and resizing against a plain model.

    use proptest::prelude::*;
    use revue::event::Key;
    use revue::layout::Rect;
    use revue::render::Buffer;
    use revue::style::Color;
    use revue::widget::traits::RenderContext;
    use revue::widget::{pane, splitter, Splitter, View};

    #[derive(Clone, Copy, Debug)]
    enum Op {
        Key(Key),
        StartResize(usize),
        StopResize,
        Toggle(usize),
        FocusPrev,
    }

    fn op() -> impl Strategy<Value = Op> {
        prop_oneof![
            4 => prop_oneof![
                Just(Key::Tab), Just(Key::BackTab), Just(Key::Left), Just(Key::Right),
                Just(Key::Up), Just(Key::Down), Just(Key::Char('h')), Just(Key::Char('j')),
                Just(Key::Char('k')), Just(Key::Char('l')), Just(Key::Enter), Just(Key::Escape),
                Just(Key::Char('x')),
            ].prop_map(Op::Key),
            2 => (0usize..5).prop_map(Op::StartResize),
            1 => Just(Op::StopResize),
            2 => (0usize..6).prop_map(Op::Toggle),
            1 => Just(Op::FocusPrev),
        ]
    }

    struct Model {
        ratios: Vec<f32>,
        collapsible: Vec<bool>,
        collapsed: Vec<bool>,
        focused: usize,
        active: Option<usize>,
    }

    impl Model {
        fn visible(&self) -> Vec<usize> {
            (0..self.ratios.len())
                .filter(|&i| !self.collapsed[i])
                .collect()
        }
        fn divider_panes(&self, d: usize) -> Option<(usize, usize)> {
            let v = self.visible();
            Some((*v.get(d)?, *v.get(d.checked_add(1)?)?))
        }
    }

    /// Divider positions along the 101-column split (the column after
    /// each pane but the last)
    fn dividers(s: &Splitter) -> Vec<u16> {
        let areas = s.pane_areas(Rect::new(0, 0, 101, 1));
        areas
            .iter()
            .take(areas.len().saturating_sub(1))
            .map(|(_, r)| r.x + r.width)
            .collect()
    }

    /// The divider drawn in the active color, if any
    fn highlighted(s: &Splitter) -> Option<usize> {
        let mut buffer = Buffer::new(101, 1);
        let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 101, 1));
        s.render(&mut ctx);
        dividers(s)
            .iter()
            .position(|&x| buffer.get(x, 0).unwrap().fg == Some(Color::GREEN))
    }

    proptest! {
        /// Tab and `focus_prev` cycle the focus through the visible panes;
        /// `start_resize` takes only a divider that is drawn; the arrows and
        /// hjkl are taken only while a divider is active, Enter and Escape
        /// end it, other keys are never taken. The active divider is the
        /// highlighted one.
        ///
        /// A step that keeps both ratios at the divider inside 0.1..0.9
        /// moves that divider by the step and no divider before it (the
        /// ones after it by at most a rounding cell). Steps that clamp move
        /// other panes too (#885), so
        /// widths are not checked after the first one.
        ///
        /// Collapsing the focused pane leaves the focus stuck on it (#885),
        /// so those toggles are skipped.
        #[test]
        fn prop_splitter_keys_match_the_model(
            ratios in prop::collection::vec(0.15f32..=0.85, 1..6),
            collapsible in prop::collection::vec(any::<bool>(), 6),
            ops in prop::collection::vec(op(), 0..40),
        ) {
            let n = ratios.len();
            let mut s = splitter().active_color(Color::GREEN);
            for i in 0..n {
                let mut p = pane(format!("p{i}")).ratio(ratios[i]).min_size(0);
                if collapsible[i] {
                    p = p.collapsible();
                }
                s = s.pane(p);
            }
            let mut m = Model {
                ratios: ratios.clone(),
                collapsible: collapsible[..n].to_vec(),
                collapsed: vec![false; n],
                focused: 0,
                active: None,
            };
            let mut widths_known = true;

            for op in ops {
                let before = dividers(&s);
                let mut moved: Option<(usize, i16)> = None;
                match op {
                    Op::Toggle(i) => {
                        if i == m.focused && i < n && m.collapsible[i] && !m.collapsed[i] {
                            continue; // #885
                        }
                        s.toggle_pane(i);
                        if i < n && m.collapsible[i] {
                            m.collapsed[i] = !m.collapsed[i];
                        }
                    }
                    Op::StartResize(d) => {
                        s.start_resize(d);
                        if m.divider_panes(d).is_some() {
                            m.active = Some(d);
                        }
                    }
                    Op::StopResize => {
                        s.stop_resize();
                        m.active = None;
                    }
                    Op::FocusPrev => {
                        s.focus_prev();
                        let v = m.visible();
                        if let Some(p) = v.iter().position(|&i| i == m.focused) {
                            m.focused = v[(p + v.len() - 1) % v.len()];
                        }
                    }
                    Op::Key(key) => {
                        let taken = s.handle_key(&key);
                        let expected = match key {
                            Key::Tab => {
                                let v = m.visible();
                                if let Some(p) = v.iter().position(|&i| i == m.focused) {
                                    m.focused = v[(p + 1) % v.len()];
                                }
                                true
                            }
                            Key::Left | Key::Up | Key::Char('h') | Key::Char('k') if m.active.is_some() => {
                                moved = Some((m.active.unwrap(), -5));
                                true
                            }
                            Key::Right | Key::Down | Key::Char('l') | Key::Char('j') if m.active.is_some() => {
                                moved = Some((m.active.unwrap(), 5));
                                true
                            }
                            Key::Enter | Key::Escape if m.active.is_some() => {
                                m.active = None;
                                true
                            }
                            _ => false,
                        };
                        prop_assert_eq!(taken, expected, "{:?}", key);
                    }
                }

                // Resizing: the model's ratios, and what the screen shows
                if let Some((d, delta)) = moved {
                    if let Some((a, b)) = m.divider_panes(d) {
                        let change = delta as f32 * 0.01;
                        let (na, nb) = (m.ratios[a] + change, m.ratios[b] - change);
                        let clamped = !(0.1..=0.9).contains(&na) || !(0.1..=0.9).contains(&nb);
                        m.ratios[a] = na.clamp(0.1, 0.9);
                        m.ratios[b] = nb.clamp(0.1, 0.9);
                        if clamped {
                            widths_known = false;
                        }
                        let after = dividers(&s);
                        if widths_known {
                            for k in 0..before.len() {
                                if k < d {
                                    prop_assert_eq!(after[k], before[k], "divider {} moved", k);
                                } else if k > d {
                                    prop_assert!(after[k].abs_diff(before[k]) <= 1, "divider {} moved", k);
                                } else if delta > 0 {
                                    prop_assert!(after[k] > before[k], "divider {} did not move right", k);
                                } else {
                                    prop_assert!(after[k] < before[k], "divider {} did not move left", k);
                                }
                            }
                        }
                    }
                } else if matches!(op, Op::Key(_) | Op::StartResize(_) | Op::StopResize | Op::FocusPrev) {
                    prop_assert_eq!(dividers(&s), before, "{:?} moved a divider", op);
                }

                let focused = format!("p{}", m.focused);
                prop_assert_eq!(s.focused(), Some(focused.as_str()));
                let drawn = m.active.filter(|&d| m.divider_panes(d).is_some());
                prop_assert_eq!(highlighted(&s), drawn);
            }
        }
    }
}
