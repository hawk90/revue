//! View trait implementation for SortableList

use crate::layout::Rect;
use crate::widget::traits::{Interactive, RenderContext, View};
use crate::{impl_styled_view, impl_view_meta, impl_widget_builders};

use super::core::SortableList;

impl View for SortableList {
    fn render(&self, ctx: &mut RenderContext) {
        let area = ctx.area;
        let visible_count = (area.height / self.item_height) as usize;

        for (i, item) in self
            .items
            .iter()
            .enumerate()
            .skip(self.scroll)
            .take(visible_count)
        {
            let y = area.y + ((i - self.scroll) as u16 * self.item_height);

            // Determine colors
            let is_selected = self.selected == Some(i);
            let is_dragging = item.dragging;
            let is_drop_target = self.drop_target == Some(i);

            let fg = if is_dragging {
                self.drag_color
            } else if is_selected {
                self.selected_color
            } else {
                self.item_color
            };

            // Draw drop indicator
            if is_drop_target && self.dragging.is_some() {
                ctx.draw_hline(area.x, y, area.width, '─', self.drag_color);
                continue;
            }

            // Draw handle if enabled
            let mut x = area.x;
            if self.show_handles {
                let handle = if is_dragging { "↕ " } else { "≡ " };
                for ch in handle.chars() {
                    if let Some(cell) = ctx.buffer.get_mut(x, y) {
                        cell.symbol = ch;
                        cell.fg = Some(crate::style::Color::rgb(100, 100, 100));
                    }
                    x += 1;
                }
            }

            // Selection indicator
            let prefix = if is_selected { "▶ " } else { "  " };
            for ch in prefix.chars() {
                if let Some(cell) = ctx.buffer.get_mut(x, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                }
                x += 1;
            }

            // Item label
            let max_len = (area.x + area.width).saturating_sub(x) as usize;
            for (j, ch) in item.label.chars().take(max_len).enumerate() {
                if let Some(cell) = ctx.buffer.get_mut(x + j as u16, y) {
                    cell.symbol = ch;
                    cell.fg = Some(fg);
                    if is_selected {
                        cell.modifier |= crate::render::Modifier::BOLD;
                    }
                    if is_dragging {
                        cell.modifier |= crate::render::Modifier::DIM;
                    }
                }
            }
        }

        // Draw final drop indicator at end if needed
        if let Some(target) = self.drop_target {
            if target == self.items.len() && self.dragging.is_some() {
                let y = area.y
                    + (visible_count.min(self.items.len() - self.scroll) as u16 * self.item_height);
                if y < area.y + area.height {
                    ctx.draw_hline(area.x, y, area.width, '─', self.drag_color);
                }
            }
        }
    }

    impl_view_meta!("SortableList");
}

impl_styled_view!(SortableList);
impl_widget_builders!(SortableList);

impl Interactive for SortableList {
    fn handle_key(&mut self, event: &crate::event::KeyEvent) -> crate::widget::traits::EventResult {
        use crate::event::Key;

        match event.key {
            Key::Up | Key::Char('k') => {
                if event.shift || event.alt {
                    self.move_up();
                } else {
                    self.select_prev();
                }
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            Key::Down | Key::Char('j') => {
                if event.shift || event.alt {
                    self.move_down();
                } else {
                    self.select_next();
                }
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            Key::Home => {
                self.selected = if self.items.is_empty() { None } else { Some(0) };
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            Key::End => {
                self.selected = if self.items.is_empty() {
                    None
                } else {
                    Some(self.items.len() - 1)
                };
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            Key::Escape if self.is_dragging() => {
                self.cancel_drag();
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            _ => crate::widget::traits::EventResult::Ignored,
        }
    }

    fn handle_mouse(
        &mut self,
        event: &crate::event::MouseEvent,
        area: Rect,
    ) -> crate::widget::traits::EventResult {
        if !area.contains(event.x, event.y) {
            return crate::widget::traits::EventResult::Ignored;
        }

        let relative_y = event.y.saturating_sub(area.y) as usize;
        let clicked_idx = (relative_y / self.item_height as usize + self.scroll)
            .min(self.items.len().saturating_sub(1));

        match event.kind {
            crate::event::MouseEventKind::Down(crate::event::MouseButton::Left) => {
                self.selected = Some(clicked_idx);
                // Check if clicking on handle area to start drag
                if self.show_handles && event.x < area.x + 2 {
                    self.start_drag();
                }
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            crate::event::MouseEventKind::Drag(crate::event::MouseButton::Left)
                if self.is_dragging() =>
            {
                self.update_drop_target(event.y, area.y);
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            crate::event::MouseEventKind::Up(crate::event::MouseButton::Left)
                if self.is_dragging() =>
            {
                self.end_drag();
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            crate::event::MouseEventKind::ScrollDown => {
                if self.scroll < self.items.len().saturating_sub(1) {
                    self.scroll += 1;
                }
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            crate::event::MouseEventKind::ScrollUp => {
                self.scroll = self.scroll.saturating_sub(1);
                crate::widget::traits::EventResult::ConsumedAndRender
            }
            _ => crate::widget::traits::EventResult::Ignored,
        }
    }
}

impl crate::widget::traits::Draggable for SortableList {
    fn can_drag(&self) -> bool {
        self.selected.is_some()
    }

    fn drag_data(&self) -> Option<crate::event::drag::DragData> {
        self.selected.map(|idx| {
            let label = self
                .items
                .get(idx)
                .map(|i| i.label.clone())
                .unwrap_or_default();
            crate::event::drag::DragData::list_item(idx, label)
        })
    }

    fn drag_preview(&self) -> Option<String> {
        self.selected
            .and_then(|idx| self.items.get(idx).map(|i| format!("↕ {}", i.label)))
    }

    fn on_drag_start(&mut self) {
        self.start_drag();
    }

    fn on_drag_end(&mut self, result: crate::event::drag::DropResult) {
        match result {
            crate::event::drag::DropResult::Accepted => self.end_drag(),
            _ => self.cancel_drag(),
        }
    }

    fn can_drop(&self) -> bool {
        true
    }

    fn accepted_types(&self) -> &[&'static str] {
        &["list_item"]
    }

    fn on_drop(&mut self, data: crate::event::drag::DragData) -> bool {
        if let Some(from_idx) = data.as_list_index() {
            if let Some(to_idx) = self.drop_target {
                // Reorder
                if from_idx < self.items.len() && from_idx != to_idx {
                    let item = self.items.remove(from_idx);
                    let insert_idx = if to_idx > from_idx {
                        to_idx - 1
                    } else {
                        to_idx
                    };
                    self.items.insert(insert_idx.min(self.items.len()), item);
                    self.selected = Some(insert_idx.min(self.items.len() - 1));

                    if let Some(ref mut callback) = self.on_reorder {
                        callback(from_idx, insert_idx);
                    }
                    return true;
                }
            }
        }
        false
    }
}
