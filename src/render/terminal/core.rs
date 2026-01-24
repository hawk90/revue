//! Terminal backend core implementation using crossterm

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{DisableMouseCapture, EnableMouseCapture},
    execute, queue,
    style::{Attribute, ResetColor, SetAttribute},
    terminal::{
        self, disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};
use std::io::Write;

use super::super::{diff, Buffer};
use crate::layout::Rect;
use crate::Result;

use super::types::Terminal;

impl<W: Write> Terminal<W> {
    /// Create a new terminal with the given writer
    pub fn new(writer: W) -> Result<Self> {
        let (width, height) = terminal::size()?;
        Ok(Self {
            writer,
            current: Buffer::new(width, height),
            raw_mode: false,
            mouse_capture: false,
        })
    }

    /// Initialize the terminal for TUI mode with mouse capture
    pub fn init(&mut self) -> Result<()> {
        self.init_with_mouse(true)
    }

    /// Initialize the terminal for TUI mode with optional mouse capture
    ///
    /// When `mouse_capture` is false, text selection in terminal works normally.
    /// Use this for keyboard-only applications.
    pub fn init_with_mouse(&mut self, mouse_capture: bool) -> Result<()> {
        enable_raw_mode()?;
        self.raw_mode = true;
        self.mouse_capture = mouse_capture;
        if mouse_capture {
            execute!(
                self.writer,
                EnterAlternateScreen,
                EnableMouseCapture,
                Hide,
                Clear(ClearType::All)
            )?;
        } else {
            execute!(
                self.writer,
                EnterAlternateScreen,
                Hide,
                Clear(ClearType::All)
            )?;
        }
        Ok(())
    }

    /// Restore the terminal to normal mode
    pub fn restore(&mut self) -> Result<()> {
        if self.raw_mode {
            if self.mouse_capture {
                execute!(
                    self.writer,
                    DisableMouseCapture,
                    ResetColor,
                    Show,
                    LeaveAlternateScreen
                )?;
            } else {
                execute!(self.writer, ResetColor, Show, LeaveAlternateScreen)?;
            }
            disable_raw_mode()?;
            self.raw_mode = false;
        }
        Ok(())
    }

    /// Get terminal size
    pub fn size(&self) -> (u16, u16) {
        (self.current.width(), self.current.height())
    }

    /// Resize the terminal buffer
    pub fn resize(&mut self, width: u16, height: u16) {
        self.current.resize(width, height);
    }

    /// Render a buffer to the terminal using diff-based updates
    ///
    /// This performs a full-screen diff. For optimized rendering with dirty regions,
    /// use [`render_dirty`](Self::render_dirty) instead.
    pub fn render(&mut self, buffer: &Buffer) -> Result<()> {
        let changes = diff::diff(&self.current, buffer, &[]);

        self.draw_changes(changes, buffer)
    }

    /// Render a buffer with dirty-rect tracking for optimized updates
    ///
    /// Only cells within the specified dirty regions will be compared and updated.
    /// This significantly reduces CPU usage for mostly-static UIs where only
    /// specific regions change each frame.
    ///
    /// # Arguments
    ///
    /// * `buffer` - The new buffer state to render
    /// * `dirty_rects` - Regions that may have changed since last render
    ///
    /// # Example
    ///
    /// ```ignore
    /// use revue::layout::Rect;
    ///
    /// // Only diff the area where a widget was updated
    /// let dirty = [Rect::new(10, 5, 20, 3)];
    /// terminal.render_dirty(&buffer, &dirty)?;
    /// ```
    pub fn render_dirty(&mut self, buffer: &Buffer, dirty_rects: &[Rect]) -> Result<()> {
        let changes = diff::diff(&self.current, buffer, dirty_rects);
        self.draw_changes(changes, buffer)
    }

    /// Force a full redraw
    pub fn force_redraw(&mut self, buffer: &Buffer) -> Result<()> {
        queue!(self.writer, Clear(ClearType::All))?;

        let mut state = super::types::RenderState::default();

        for (x, y, cell) in buffer.iter_cells() {
            if !cell.is_continuation() {
                let hyperlink_url = cell.hyperlink_id.and_then(|id| buffer.get_hyperlink(id));
                let escape_sequence = cell.sequence_id.and_then(|id| buffer.get_sequence(id));
                self.draw_cell_stateful(x, y, cell, hyperlink_url, escape_sequence, &mut state)?;
            }
            // Update current buffer (Cell is Copy, no allocation)
            self.current.set(x, y, *cell);
        }

        // Close any open hyperlink at end of frame
        if state.hyperlink_id.is_some() {
            self.write_hyperlink_end()?;
        }

        // Reset state at end of frame
        if state.fg.is_some() || state.bg.is_some() || !state.modifier.is_empty() {
            queue!(self.writer, SetAttribute(Attribute::Reset))?;
        }

        self.writer.flush()?;
        Ok(())
    }

    /// Clear the screen
    pub fn clear(&mut self) -> Result<()> {
        execute!(self.writer, Clear(ClearType::All))?;
        self.current.clear();
        Ok(())
    }

    /// Show the cursor
    pub fn show_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Show)?;
        Ok(())
    }

    /// Hide the cursor
    pub fn hide_cursor(&mut self) -> Result<()> {
        execute!(self.writer, Hide)?;
        Ok(())
    }

    /// Move cursor to position
    pub fn set_cursor(&mut self, x: u16, y: u16) -> Result<()> {
        execute!(self.writer, MoveTo(x, y))?;
        Ok(())
    }
}

impl<W: Write> Drop for Terminal<W> {
    fn drop(&mut self) {
        let _ = self.restore();
    }
}
