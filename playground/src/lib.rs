//! Revue Playground - Online WASM-based TUI preview
//!
//! Provides a web-based terminal emulator for running Revue applications.

use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

/// Terminal cell
#[derive(Clone, Debug)]
pub struct Cell {
    pub ch: char,
    pub fg: String,
    pub bg: String,
    pub bold: bool,
    pub italic: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            ch: ' ',
            fg: "#c0caf5".to_string(),
            bg: "#1a1b26".to_string(),
            bold: false,
            italic: false,
        }
    }
}

/// Virtual terminal for WASM
#[wasm_bindgen]
pub struct Terminal {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    cursor_x: u32,
    cursor_y: u32,
    cursor_visible: bool,
    cell_width: f64,
    cell_height: f64,
}

#[wasm_bindgen]
impl Terminal {
    /// Create a new terminal
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Self {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let cells = vec![Cell::default(); (width * height) as usize];

        Self {
            width,
            height,
            cells,
            cursor_x: 0,
            cursor_y: 0,
            cursor_visible: true,
            cell_width: 10.0,
            cell_height: 20.0,
        }
    }

    /// Resize terminal
    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.cells = vec![Cell::default(); (width * height) as usize];
    }

    /// Clear terminal
    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = Cell::default();
        }
    }

    /// Set a cell
    pub fn set_cell(&mut self, x: u32, y: u32, ch: char, fg: &str, bg: &str, bold: bool, italic: bool) {
        if x < self.width && y < self.height {
            let idx = (y * self.width + x) as usize;
            self.cells[idx] = Cell {
                ch,
                fg: fg.to_string(),
                bg: bg.to_string(),
                bold,
                italic,
            };
        }
    }

    /// Write text at position
    pub fn write(&mut self, x: u32, y: u32, text: &str, fg: &str, bg: &str) {
        for (i, ch) in text.chars().enumerate() {
            self.set_cell(x + i as u32, y, ch, fg, bg, false, false);
        }
    }

    /// Move cursor
    pub fn move_cursor(&mut self, x: u32, y: u32) {
        self.cursor_x = x.min(self.width - 1);
        self.cursor_y = y.min(self.height - 1);
    }

    /// Show/hide cursor
    pub fn set_cursor_visible(&mut self, visible: bool) {
        self.cursor_visible = visible;
    }

    /// Render to canvas
    pub fn render(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let ctx = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;

        let canvas_width = self.width as f64 * self.cell_width;
        let canvas_height = self.height as f64 * self.cell_height;

        canvas.set_width(canvas_width as u32);
        canvas.set_height(canvas_height as u32);

        // Clear canvas
        ctx.set_fill_style(&JsValue::from_str("#1a1b26"));
        ctx.fill_rect(0.0, 0.0, canvas_width, canvas_height);

        // Draw cells
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width + x) as usize;
                let cell = &self.cells[idx];

                let px = x as f64 * self.cell_width;
                let py = y as f64 * self.cell_height;

                // Background
                if cell.bg != "#1a1b26" {
                    ctx.set_fill_style(&JsValue::from_str(&cell.bg));
                    ctx.fill_rect(px, py, self.cell_width, self.cell_height);
                }

                // Character
                if cell.ch != ' ' {
                    let font = if cell.bold && cell.italic {
                        "bold italic 16px monospace"
                    } else if cell.bold {
                        "bold 16px monospace"
                    } else if cell.italic {
                        "italic 16px monospace"
                    } else {
                        "16px monospace"
                    };
                    ctx.set_font(font);
                    ctx.set_fill_style(&JsValue::from_str(&cell.fg));
                    ctx.fill_text(&cell.ch.to_string(), px + 1.0, py + 16.0)?;
                }
            }
        }

        // Draw cursor
        if self.cursor_visible {
            let cx = self.cursor_x as f64 * self.cell_width;
            let cy = self.cursor_y as f64 * self.cell_height;
            ctx.set_fill_style(&JsValue::from_str("#c0caf5"));
            ctx.fill_rect(cx, cy + self.cell_height - 2.0, self.cell_width, 2.0);
        }

        Ok(())
    }

    /// Get terminal width
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Get terminal height
    pub fn height(&self) -> u32 {
        self.height
    }
}

/// Playground state
#[wasm_bindgen]
pub struct Playground {
    terminal: Terminal,
    code: String,
}

#[wasm_bindgen]
impl Playground {
    /// Create new playground
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            terminal: Terminal::new(80, 24),
            code: String::new(),
        }
    }

    /// Set code
    pub fn set_code(&mut self, code: &str) {
        self.code = code.to_string();
    }

    /// Get code
    pub fn get_code(&self) -> String {
        self.code.clone()
    }

    /// Run code (simulated)
    pub fn run(&mut self) {
        self.terminal.clear();

        // Simple demo - parse and display
        self.terminal.write(1, 1, "Revue Playground", "#7aa2f7", "#1a1b26");
        self.terminal.write(1, 2, "─".repeat(40).as_str(), "#565f89", "#1a1b26");

        // Show code preview
        for (i, line) in self.code.lines().take(15).enumerate() {
            let display = if line.len() > 70 {
                format!("{}...", &line[..67])
            } else {
                line.to_string()
            };
            self.terminal.write(1, 4 + i as u32, &display, "#c0caf5", "#1a1b26");
        }

        self.terminal.write(1, 21, "Press Ctrl+Enter to run", "#565f89", "#1a1b26");
    }

    /// Render to canvas
    pub fn render(&self, canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
        self.terminal.render(canvas)
    }

    /// Handle key event
    pub fn handle_key(&mut self, event: &KeyboardEvent) -> bool {
        let key = event.key();

        if event.ctrl_key() && key == "Enter" {
            self.run();
            return true;
        }

        false
    }

    /// Get terminal reference
    pub fn terminal(&self) -> *const Terminal {
        &self.terminal
    }
}

impl Default for Playground {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize playground
#[wasm_bindgen(start)]
pub fn init() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
