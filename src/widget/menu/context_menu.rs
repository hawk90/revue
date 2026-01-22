//! Context menu widget

use super::super::traits::{RenderContext, View, WidgetProps};
use super::types::MenuItem;
use crate::event::Key;
use crate::render::Cell;
use crate::style::Color;
use crate::{impl_props_builders, impl_styled_view};

/// Context menu (popup menu)
pub struct ContextMenu {
    /// Menu items
    pub(crate) items: Vec<MenuItem>,
    /// Position
    x: u16,
    y: u16,
    /// Selected item
    pub(crate) selected: usize,
    /// Visible
    visible: bool,
    /// Colors
    pub(crate) bg: Color,
    pub(crate) fg: Color,
    selected_bg: Color,
    selected_fg: Color,
    /// Widget properties
    props: WidgetProps,
}

impl Default for ContextMenu {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMenu {
    /// Create a new context menu
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            x: 0,
            y: 0,
            selected: 0,
            visible: false,
            bg: Color::rgb(40, 40, 40),
            fg: Color::WHITE,
            selected_bg: Color::rgb(60, 100, 180),
            selected_fg: Color::WHITE,
            props: WidgetProps::new(),
        }
    }

    /// Add an item
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(item);
        self
    }

    /// Add items
    pub fn items(mut self, items: Vec<MenuItem>) -> Self {
        self.items.extend(items);
        self
    }

    /// Show at position
    pub fn show(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
        self.visible = true;
        self.selected = 0;
    }

    /// Hide menu
    pub fn hide(&mut self) {
        self.visible = false;
    }

    /// Is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }

    /// Handle key
    pub fn handle_key(&mut self, key: &Key) -> bool {
        if !self.visible {
            return false;
        }

        match key {
            Key::Up | Key::Char('k') => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
                true
            }
            Key::Down | Key::Char('j') => {
                if self.selected < self.items.len().saturating_sub(1) {
                    self.selected += 1;
                }
                true
            }
            Key::Enter | Key::Char(' ') => {
                if let Some(item) = self.items.get(self.selected) {
                    item.execute();
                }
                self.hide();
                true
            }
            Key::Escape => {
                self.hide();
                true
            }
            _ => false,
        }
    }
}

impl View for ContextMenu {
    crate::impl_view_meta!("ContextMenu");

    fn render(&self, ctx: &mut RenderContext) {
        if !self.visible || self.items.is_empty() {
            return;
        }

        let width = self.items.iter().map(|i| i.label.len()).max().unwrap_or(10) as u16 + 4;
        let height = self.items.len() as u16 + 2;

        // Adjust position to fit in area
        let x = self.x.min(ctx.area.width.saturating_sub(width));
        let y = self.y.min(ctx.area.height.saturating_sub(height));

        // Draw border and background
        for dy in 0..height {
            for dx in 0..width {
                let ch = if dy == 0 && dx == 0 {
                    '┌'
                } else if dy == 0 && dx == width - 1 {
                    '┐'
                } else if dy == height - 1 && dx == 0 {
                    '└'
                } else if dy == height - 1 && dx == width - 1 {
                    '┘'
                } else if dy == 0 || dy == height - 1 {
                    '─'
                } else if dx == 0 || dx == width - 1 {
                    '│'
                } else {
                    ' '
                };

                let mut cell = Cell::new(ch);
                cell.bg = Some(self.bg);
                cell.fg = Some(self.fg);
                ctx.buffer.set(x + dx, y + dy, cell);
            }
        }

        // Draw items
        for (i, item) in self.items.iter().enumerate() {
            let item_y = y + 1 + i as u16;
            let is_selected = i == self.selected;

            let bg = if is_selected {
                self.selected_bg
            } else {
                self.bg
            };
            let fg = if is_selected {
                self.selected_fg
            } else {
                self.fg
            };

            // Fill row
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(bg);
                ctx.buffer.set(x + dx, item_y, cell);
            }

            // Draw label
            for (j, ch) in item.label.chars().enumerate() {
                if j as u16 + 2 >= width - 1 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(fg);
                cell.bg = Some(bg);
                ctx.buffer.set(x + 2 + j as u16, item_y, cell);
            }
        }
    }
}

impl_styled_view!(ContextMenu);
impl_props_builders!(ContextMenu);
