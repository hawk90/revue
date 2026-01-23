
        // Draw border
        let border_chars = ['╭', '╮', '╰', '╯', '─', '│'];
        let mut current_y = y;

        // Top border
        let mut tl = Cell::new(border_chars[0]);
        tl.fg = Some(self.border_color);
        ctx.buffer.set(x, current_y, tl);

        for dx in 1..width - 1 {
            let mut h = Cell::new(border_chars[4]);
            h.fg = Some(self.border_color);
            ctx.buffer.set(x + dx, current_y, h);
        }

        let mut tr = Cell::new(border_chars[1]);
        tr.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, current_y, tr);

        current_y += 1;

        // Title (if present)
        if let Some(ref title) = self.title {
            let mut left = Cell::new(border_chars[5]);
            left.fg = Some(self.border_color);
            ctx.buffer.set(x, current_y, left);

            let title_x = x + 2;
            for (i, ch) in title.chars().enumerate() {
                if title_x + i as u16 >= x + width - 2 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(Color::CYAN);
                cell.bg = Some(self.bg_color);
                cell.modifier |= Modifier::BOLD;
                ctx.buffer.set(title_x + i as u16, current_y, cell);
            }

            let mut right = Cell::new(border_chars[5]);
            right.fg = Some(self.border_color);
            ctx.buffer.set(x + width - 1, current_y, right);

            current_y += 1;
        }

        // Search input line
        let mut left = Cell::new(border_chars[5]);
        left.fg = Some(self.border_color);
        ctx.buffer.set(x, current_y, left);

        // Search icon
        let search_icon = Cell::new('🔍');
        ctx.buffer.set(x + 2, current_y, search_icon);

        // Query or placeholder
        let input_x = x + 4;
        let display_text = if self.query.is_empty() {
            &self.placeholder
        } else {
            &self.query
        };
        let text_color = if self.query.is_empty() {
            Color::rgb(100, 100, 100)
        } else {
            Color::WHITE
        };

        for (i, ch) in display_text.chars().enumerate() {
            if input_x + i as u16 >= x + width - 2 {
                break;
            }
            let mut cell = Cell::new(ch);
            cell.fg = Some(text_color);
            cell.bg = Some(self.bg_color);
            ctx.buffer.set(input_x + i as u16, current_y, cell);
        }

        // Cursor
        if !self.query.is_empty() || display_text == &self.placeholder {
            let cursor_x = input_x + self.query.len() as u16;
            if cursor_x < x + width - 2 {
                let mut cursor = Cell::new('▏');
                cursor.fg = Some(Color::WHITE);
                cursor.bg = Some(self.bg_color);
                ctx.buffer.set(cursor_x, current_y, cursor);
            }
        }

        let mut right = Cell::new(border_chars[5]);
        right.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, current_y, right);

        current_y += 1;

        // Separator
        let mut sl = Cell::new('├');
        sl.fg = Some(self.border_color);
        ctx.buffer.set(x, current_y, sl);

        for dx in 1..width - 1 {
            let mut h = Cell::new('─');
            h.fg = Some(self.border_color);
            ctx.buffer.set(x + dx, current_y, h);
        }

        let mut sr = Cell::new('┤');
        sr.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, current_y, sr);

        current_y += 1;

        // Command items
        let scroll_offset = self.selection.offset();
        let visible_items: Vec<_> = self
            .filtered
            .iter()
            .skip(scroll_offset)
            .take(self.max_visible as usize)
            .collect();

        for (i, &cmd_idx) in visible_items.iter().enumerate() {
            let cmd = &self.commands[*cmd_idx];
            let is_selected = self.selection.is_selected(scroll_offset + i);
            let item_y = current_y + i as u16;

            // Left border
            let mut left = Cell::new(border_chars[5]);
            left.fg = Some(self.border_color);
            ctx.buffer.set(x, item_y, left);

            // Background for selected
            let row_bg = if is_selected {
                self.selected_bg
            } else {
                self.bg_color
            };
            for dx in 1..width - 1 {
                let mut cell = Cell::new(' ');
                cell.bg = Some(row_bg);
                ctx.buffer.set(x + dx, item_y, cell);
            }

            let mut content_x = x + 2;

            // Icon
            if self.show_icons {
                if let Some(icon) = cmd.icon {
                    let mut cell = Cell::new(icon);
                    cell.fg = Some(Color::CYAN);
                    cell.bg = Some(row_bg);
                    ctx.buffer.set(content_x, item_y, cell);
                }
                content_x += 2;
            }

            // Label with highlighting
            let highlighted = self.highlight_match(&cmd.label);
            for (ch, is_match) in highlighted {
                if content_x >= x + width - 15 {
                    break;
                }
                let mut cell = Cell::new(ch);
                cell.fg = Some(if is_match {
                    self.match_color
                } else {
                    Color::WHITE
                });
                cell.bg = Some(row_bg);
                if is_match {
                    cell.modifier |= Modifier::BOLD;
                }
                ctx.buffer.set(content_x, item_y, cell);
                content_x += 1;
            }

            // Shortcut (right-aligned)
            if self.show_shortcuts {
                if let Some(ref shortcut) = cmd.shortcut {
                    let shortcut_x = x + width - 2 - shortcut.len() as u16;
                    for (i, ch) in shortcut.chars().enumerate() {
                        let mut cell = Cell::new(ch);
                        cell.fg = Some(Color::rgb(120, 120, 120));
                        cell.bg = Some(row_bg);
                        ctx.buffer.set(shortcut_x + i as u16, item_y, cell);
                    }
                }
            }

            // Right border
            let mut right = Cell::new(border_chars[5]);
            right.fg = Some(self.border_color);
            ctx.buffer.set(x + width - 1, item_y, right);
        }

        // Fill remaining space if fewer items than max_visible
        for i in visible_items.len()..self.max_visible as usize {
            let item_y = current_y + i as u16;
            if item_y >= y + height - 1 {
                break;
            }

            let mut left = Cell::new(border_chars[5]);
            left.fg = Some(self.border_color);
            ctx.buffer.set(x, item_y, left);

            let mut right = Cell::new(border_chars[5]);
            right.fg = Some(self.border_color);
            ctx.buffer.set(x + width - 1, item_y, right);
        }

        // Bottom border
        let bottom_y = y + height - 1;
        let mut bl = Cell::new(border_chars[2]);
        bl.fg = Some(self.border_color);
        ctx.buffer.set(x, bottom_y, bl);

        for dx in 1..width - 1 {
            let mut h = Cell::new(border_chars[4]);
            h.fg = Some(self.border_color);
            ctx.buffer.set(x + dx, bottom_y, h);
        }

        let mut br = Cell::new(border_chars[3]);
        br.fg = Some(self.border_color);
        ctx.buffer.set(x + width - 1, bottom_y, br);

        // Results count
        let count_str = format!("{}/{}", self.filtered.len(), self.commands.len());
        let count_x = x + width - 2 - count_str.len() as u16;
        for (i, ch) in count_str.chars().enumerate() {
            let mut cell = Cell::new(ch);
            cell.fg = Some(Color::rgb(80, 80, 80));
            cell.bg = Some(self.bg_color);
            ctx.buffer.set(count_x + i as u16, bottom_y, cell);
        }
    }
}

impl_styled_view!(CommandPalette);
impl_props_builders!(CommandPalette);

/// Helper to create a command palette
