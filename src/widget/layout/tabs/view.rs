//! Drawing the bar, and the columns each label takes in it

use super::{Tab, Tabs};
use crate::layout::Rect;
use crate::render::Cell;
use crate::style::Color;
use crate::widget::traits::{RenderContext, View};

impl View for Tabs {
    /// Its own bounds, which `render` puts back.
    fn held_size(&self) -> (u16, u16) {
        (self.min_width, self.min_height)
    }

    /// One row as wide as the bar: every label with its padding and dividers.
    fn measure(&self, max_width: u16, max_height: u16) -> Option<(u16, u16)> {
        let spans = label_spans(self.tabs.iter().map(|t| t.label.as_str()));
        let natural = spans.last().map_or(0, |&(_, end)| end);
        let area = self.apply_constraints(Rect::new(0, 0, natural, 1));
        Some((area.width.min(max_width), area.height.min(max_height)))
    }
    fn render(&self, ctx: &mut RenderContext) {
        let area = self.apply_constraints(ctx.area);
        if area.width < 3 || area.height < 1 || self.tabs.is_empty() {
            return;
        }

        // The bar scrolls (in columns) to keep the active tab in view and
        // starts at a whole tab (#973)
        let first = self.first_tab.unwrap_or_else(|| {
            let spans = label_spans(self.tabs.iter().map(|t| t.label.as_str()));
            bar_window(&spans, &self.scroll, self.selection.index, area.width)
        });

        let mut x: u16 = 0;

        for (i, tab) in self.tabs.iter().enumerate().skip(first) {
            x = self.draw_tab(ctx, area, x, i, tab);
        }
    }

    crate::impl_view_meta!("Tabs");
}

impl Tabs {
    /// The tab's fg and bg: the active highlight, or the builder's colors
    /// and else the stylesheet's
    fn tab_colors(&self, ctx: &RenderContext, is_active: bool) -> (Option<Color>, Option<Color>) {
        if is_active {
            (self.active_fg, self.active_bg)
        } else {
            // Inactive tabs take the stylesheet's colors; the active tab
            // keeps its highlight, which a rule cannot address separately.
            (
                self.fg.or_else(|| ctx.css_color_if_set()),
                self.bg.or_else(|| ctx.css_background_if_set()),
            )
        }
    }

    /// The label from `x`, clipped to the bar, bold when active; returns
    /// the column after it
    fn draw_label(
        ctx: &mut RenderContext,
        area: Rect,
        mut x: u16,
        label: &str,
        fg: Option<Color>,
        bg: Option<Color>,
        is_active: bool,
    ) -> u16 {
        let clip = area.width.saturating_sub(x);
        if let Some(f) = fg {
            if is_active {
                if let Some(b) = bg {
                    ctx.draw_text_clipped_bg_bold(x, 0, label, f, b, clip);
                } else {
                    ctx.draw_text_clipped_bold(x, 0, label, f, clip);
                }
            } else if let Some(b) = bg {
                ctx.draw_text_clipped_bg(x, 0, label, f, b, clip);
            } else {
                ctx.draw_text_clipped(x, 0, label, f, clip);
            }
            x += (crate::utils::display_width(label) as u16).min(clip);
        } else {
            // fg is None: preserve terminal default color
            for ch in label.chars() {
                let cw = crate::utils::char_width(ch) as u16;
                if x + cw > area.width {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = fg;
                cell.bg = bg;
                if is_active {
                    cell.modifier |= crate::render::Modifier::BOLD;
                }
                ctx.set(x, 0, cell);
                x += cw;
            }
        }
        x
    }

    /// Tab `i` from column `x`: a padded label in its colors (the active
    /// tab's highlight, else the stylesheet's), then the divider unless it is
    /// the last; returns the column after it
    fn draw_tab(
        &self,
        ctx: &mut RenderContext,
        area: Rect,
        mut x: u16,
        i: usize,
        tab: &Tab,
    ) -> u16 {
        let is_active = i == self.selection.index;
        let (fg, bg) = self.tab_colors(ctx, is_active);

        // Draw padding
        let mut cell = Cell::new(' ');
        cell.fg = fg;
        cell.bg = bg;
        if x < area.width {
            ctx.set(x, 0, cell);
            x += 1;
        }

        // Draw label
        x = Self::draw_label(ctx, area, x, &tab.label, fg, bg, is_active);

        // Draw padding
        if x < area.width {
            let mut cell = Cell::new(' ');
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(x, 0, cell);
            x += 1;
        }

        // Draw divider (unless last tab)
        if i < self.tabs.len() - 1 && x < area.width {
            let mut cell = Cell::new(self.divider);
            let (fg, bg) = self.tab_colors(ctx, false);
            cell.fg = fg;
            cell.bg = bg;
            ctx.set(x, 0, cell);
            x += 1;
        }
        x
    }
}

/// The columns each label takes in the bar `Tabs` draws, relative to its
/// left edge, as `start..end`: a space, the label, a space, then a divider
/// column before the next. Columns past the bar's width are not drawn.
pub(crate) fn label_spans<'a>(labels: impl IntoIterator<Item = &'a str>) -> Vec<(u16, u16)> {
    let mut x: u16 = 0;
    labels
        .into_iter()
        .map(|label| {
            let start = x;
            let width = u16::try_from(crate::utils::display_width(label)).unwrap_or(u16::MAX);
            x = x.saturating_add(width).saturating_add(2);
            let span = (start, x);
            x = x.saturating_add(1);
            span
        })
        .collect()
}

/// The first tab a bar `width` columns wide draws, keeping tab `selected` in
/// view: `scroll` (in columns over `spans`) reveals the selected span when
/// it changes, and the bar starts at the first whole tab at or after the
/// offset, which still shows the selected one.
pub(crate) fn bar_window(
    spans: &[(u16, u16)],
    scroll: &crate::utils::ScrollState,
    selected: usize,
    width: u16,
) -> usize {
    scroll.set_content(spans.last().map_or(0, |&(_, end)| usize::from(end)));
    scroll.set_viewport(usize::from(width));
    if let Some(&(start, end)) = spans.get(selected) {
        scroll.track(usize::from(start)..usize::from(end));
    }
    spans
        .iter()
        .position(|&(start, _)| usize::from(start) >= scroll.offset())
        .unwrap_or(spans.len())
}

/// `spans` as drawn from tab `first`: those before it take no columns
pub(crate) fn shift_spans(spans: &[(u16, u16)], first: usize) -> Vec<(u16, u16)> {
    let base = spans.get(first).map_or(0, |&(start, _)| start);
    spans
        .iter()
        .enumerate()
        .map(|(i, &(start, end))| {
            if i < first {
                (0, 0)
            } else {
                (start - base, end - base)
            }
        })
        .collect()
}
