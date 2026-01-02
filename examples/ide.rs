//! IDE Example - Demonstrates Screen System, Command Palette, SplitPane, StatusBar
//!
//! A mini IDE-like application showing Revue's advanced features working together.
//!
//! Run with: cargo run --example ide

use revue::prelude::*;
use revue::widget::{Command, CommandPalette, TextArea};

/// Main IDE application state
struct IdeApp {
    /// Command palette visibility
    command_palette_open: bool,
    /// Command palette state
    command_palette: CommandPalette,
    /// Current file name
    current_file: String,
    /// File tree items
    files: Vec<FileItem>,
    /// Selected file index
    selected_file: usize,
    /// Editor content
    editor: TextArea,
    /// Status messages
    status_message: String,
    /// Mode (Normal, Insert, Command)
    mode: EditorMode,
    /// Split ratio
    split_ratio: f32,
    /// Notifications
    notifications: Vec<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum EditorMode {
    Normal,
    Insert,
    Command,
}

impl EditorMode {
    fn name(&self) -> &str {
        match self {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
            EditorMode::Command => "COMMAND",
        }
    }

    fn color(&self) -> Color {
        match self {
            EditorMode::Normal => Color::BLUE,
            EditorMode::Insert => Color::GREEN,
            EditorMode::Command => Color::YELLOW,
        }
    }
}

struct FileItem {
    name: String,
    is_dir: bool,
    modified: bool,
}

impl IdeApp {
    fn new() -> Self {
        let commands = vec![
            Command::new("file.new", "New File").shortcut("Ctrl+N"),
            Command::new("file.open", "Open File").shortcut("Ctrl+O"),
            Command::new("file.save", "Save File").shortcut("Ctrl+S"),
            Command::new("file.close", "Close File").shortcut("Ctrl+W"),
            Command::new("edit.undo", "Undo").shortcut("Ctrl+Z"),
            Command::new("edit.redo", "Redo").shortcut("Ctrl+Y"),
            Command::new("edit.find", "Find").shortcut("Ctrl+F"),
            Command::new("edit.replace", "Replace").shortcut("Ctrl+H"),
            Command::new("view.sidebar", "Toggle Sidebar").shortcut("Ctrl+B"),
            Command::new("view.terminal", "Toggle Terminal").shortcut("Ctrl+`"),
            Command::new("goto.line", "Go to Line").shortcut("Ctrl+G"),
            Command::new("goto.symbol", "Go to Symbol").shortcut("Ctrl+Shift+O"),
        ];

        let files = vec![
            FileItem {
                name: "src/".into(),
                is_dir: true,
                modified: false,
            },
            FileItem {
                name: "  main.rs".into(),
                is_dir: false,
                modified: true,
            },
            FileItem {
                name: "  lib.rs".into(),
                is_dir: false,
                modified: false,
            },
            FileItem {
                name: "  app/".into(),
                is_dir: true,
                modified: false,
            },
            FileItem {
                name: "    mod.rs".into(),
                is_dir: false,
                modified: false,
            },
            FileItem {
                name: "    screen.rs".into(),
                is_dir: false,
                modified: true,
            },
            FileItem {
                name: "Cargo.toml".into(),
                is_dir: false,
                modified: false,
            },
            FileItem {
                name: "README.md".into(),
                is_dir: false,
                modified: false,
            },
        ];

        let sample_code = r#"//! Revue - A Vue-style TUI framework for Rust
//!
//! This is a sample file showing syntax highlighting.

use revue::prelude::*;

fn main() -> Result<()> {
    let mut app = App::builder()
        .style("styles.css")
        .hot_reload(true)
        .build();

    let view = Text::new("Hello, Revue!")
        .fg(Color::CYAN)
        .bold();

    app.run(&view)
}

// TODO: Add more features
// FIXME: Handle edge cases
"#;

        let mut editor = TextArea::new();
        editor.set_content(sample_code);

        Self {
            command_palette_open: false,
            command_palette: CommandPalette::new().commands(commands),
            current_file: "main.rs".into(),
            files,
            selected_file: 1,
            editor,
            status_message: "Ready".into(),
            mode: EditorMode::Normal,
            split_ratio: 0.2,
            notifications: Vec::new(),
        }
    }

    fn handle_key(&mut self, key: &Key) -> bool {
        // Command palette handling
        if self.command_palette_open {
            match key {
                Key::Escape => {
                    self.command_palette_open = false;
                    self.command_palette.clear_query();
                    return true;
                }
                Key::Enter => {
                    if let Some(cmd) = self.command_palette.selected_command() {
                        self.execute_command(&cmd.id.clone());
                    }
                    self.command_palette_open = false;
                    self.command_palette.clear_query();
                    return true;
                }
                Key::Up => {
                    self.command_palette.select_prev();
                    return true;
                }
                Key::Down => {
                    self.command_palette.select_next();
                    return true;
                }
                Key::Char(c) => {
                    self.command_palette.input(*c);
                    return true;
                }
                Key::Backspace => {
                    self.command_palette.backspace();
                    return true;
                }
                _ => return false,
            }
        }

        // Mode-specific handling
        match self.mode {
            EditorMode::Normal => self.handle_normal_mode(key),
            EditorMode::Insert => self.handle_insert_mode(key),
            EditorMode::Command => self.handle_command_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: &Key) -> bool {
        match key {
            Key::Char('i') => {
                self.mode = EditorMode::Insert;
                self.status_message = "-- INSERT --".into();
                true
            }
            Key::Char(':') => {
                self.mode = EditorMode::Command;
                self.status_message = ":".into();
                true
            }
            Key::Char('j') | Key::Down => {
                self.editor.move_down();
                true
            }
            Key::Char('k') | Key::Up => {
                self.editor.move_up();
                true
            }
            Key::Char('h') | Key::Left => {
                self.editor.move_left();
                true
            }
            Key::Char('l') | Key::Right => {
                self.editor.move_right();
                true
            }
            Key::Char('p') if key == &Key::Char('p') => {
                // Ctrl+P opens command palette
                self.command_palette_open = true;
                true
            }
            Key::Tab => {
                // Navigate file tree
                self.selected_file = (self.selected_file + 1) % self.files.len();
                true
            }
            _ => false,
        }
    }

    fn handle_insert_mode(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.mode = EditorMode::Normal;
                self.status_message = "Ready".into();
                true
            }
            Key::Char(c) => {
                self.editor.insert_char(*c);
                true
            }
            Key::Enter => {
                self.editor.insert_char('\n');
                true
            }
            Key::Backspace => {
                self.editor.delete_char_before();
                true
            }
            Key::Delete => {
                self.editor.delete_char_at();
                true
            }
            Key::Up => {
                self.editor.move_up();
                true
            }
            Key::Down => {
                self.editor.move_down();
                true
            }
            Key::Left => {
                self.editor.move_left();
                true
            }
            Key::Right => {
                self.editor.move_right();
                true
            }
            _ => false,
        }
    }

    fn handle_command_mode(&mut self, key: &Key) -> bool {
        match key {
            Key::Escape => {
                self.mode = EditorMode::Normal;
                self.status_message = "Ready".into();
                true
            }
            Key::Enter => {
                let cmd = self.status_message.trim_start_matches(':').to_string();
                self.execute_vim_command(&cmd);
                self.mode = EditorMode::Normal;
                true
            }
            Key::Char(c) => {
                self.status_message.push(*c);
                true
            }
            Key::Backspace => {
                if self.status_message.len() > 1 {
                    self.status_message.pop();
                }
                true
            }
            _ => false,
        }
    }

    fn execute_command(&mut self, id: &str) {
        self.status_message = format!("Executed: {}", id);
        self.add_notification(format!("Command: {}", id));

        match id {
            "file.new" => self.status_message = "New file created".into(),
            "file.save" => self.status_message = format!("Saved: {}", self.current_file),
            "view.sidebar" => self.split_ratio = if self.split_ratio > 0.1 { 0.0 } else { 0.2 },
            _ => {}
        }
    }

    fn execute_vim_command(&mut self, cmd: &str) {
        match cmd {
            "w" => self.status_message = format!("\"{}\" written", self.current_file),
            "q" => self.status_message = "Use Ctrl+C to quit".into(),
            "wq" => self.status_message = "Saved and quit".into(),
            _ => self.status_message = format!("Unknown command: {}", cmd),
        }
    }

    fn add_notification(&mut self, msg: String) {
        self.notifications.push(msg);
        if self.notifications.len() > 3 {
            self.notifications.remove(0);
        }
    }

    fn render_file_tree(&self) -> impl View {
        let mut tree = vstack();

        for (i, file) in self.files.iter().enumerate() {
            let icon = if file.is_dir { "" } else { "" };
            let modified = if file.modified { " [+]" } else { "" };
            let name = format!("{}{}{}", icon, file.name, modified);

            let text = if i == self.selected_file {
                Text::new(name).fg(Color::CYAN).bold()
            } else if file.is_dir {
                Text::new(name).fg(Color::BLUE)
            } else if file.modified {
                Text::new(name).fg(Color::YELLOW)
            } else {
                Text::new(name)
            };

            tree = tree.child(text);
        }

        Border::rounded().title("Explorer").child(tree)
    }

    fn render_editor(&self) -> impl View {
        // Create line numbers
        let text = self.editor.get_content();
        let lines = text.lines().count().max(1);
        let line_width = lines.to_string().len();

        let mut content = vstack();
        for (i, line) in text.lines().enumerate() {
            let line_num = format!("{:>width$} ", i + 1, width = line_width);
            let row = hstack()
                .child(Text::new(line_num).fg(Color::rgb(100, 100, 100)))
                .child(Text::new(line));
            content = content.child(row);
        }

        let (cursor_row, cursor_col) = self.editor.cursor_position();
        let cursor_info = format!("Ln {}, Col {}", cursor_row + 1, cursor_col + 1);

        let header = hstack()
            .child(Text::new(format!(" {} ", self.current_file)).bg(Color::rgb(50, 50, 50)))
            .child(if self.files[self.selected_file].modified {
                Text::new(" [Modified]").fg(Color::YELLOW)
            } else {
                Text::new("")
            });

        vstack()
            .child(header)
            .child(Border::single().child(content))
            .child(Text::new(cursor_info).fg(Color::rgb(128, 128, 128)))
    }

    fn render_status_bar(&self) -> impl View {
        let mode_text = Text::new(format!(" {} ", self.mode.name()))
            .fg(Color::BLACK)
            .bg(self.mode.color())
            .bold();

        let file_text = Text::new(format!("  {} ", self.current_file));
        let status_text =
            Text::new(format!("  {} ", self.status_message)).fg(Color::rgb(180, 180, 180));

        let (cursor_row, cursor_col) = self.editor.cursor_position();
        let pos_text = Text::new(format!(" {}:{} ", cursor_row + 1, cursor_col + 1));

        hstack()
            .child(mode_text)
            .child(file_text)
            .child(status_text)
            .child(pos_text)
    }

    fn render_command_palette(&self) -> impl View {
        if !self.command_palette_open {
            return vstack(); // Empty
        }

        // Use the CommandPalette's built-in rendering
        // Just show a simple overlay since CommandPalette has its own render
        let search_box = Border::rounded().title("Command Palette").child(
            vstack()
                .child(Text::new(format!("> {}", self.command_palette.get_query())))
                .child(Text::new("─".repeat(40)).fg(Color::rgb(80, 80, 80)))
                .child(
                    Text::new("(Use ↑↓ to select, Enter to execute)").fg(Color::rgb(100, 100, 100)),
                ),
        );

        vstack().child(search_box)
    }
}

impl View for IdeApp {
    fn render(&self, ctx: &mut RenderContext) {
        // Main layout: sidebar + editor
        let sidebar = self.render_file_tree();
        let editor = self.render_editor();

        // Create split pane
        let main_content = if self.split_ratio > 0.05 {
            hstack().child(sidebar).child(editor)
        } else {
            hstack().child(editor)
        };

        // Header
        let header = hstack()
            .child(Text::new(" Revue IDE ").fg(Color::CYAN).bold())
            .child(
                Text::new(" | Ctrl+P: Commands | Tab: Files | i: Insert | :: Command ")
                    .fg(Color::rgb(100, 100, 100)),
            );

        // Main view
        let main_view = vstack()
            .child(header)
            .child(main_content)
            .child(self.render_status_bar());

        main_view.render(ctx);

        // Overlay command palette
        if self.command_palette_open {
            self.render_command_palette().render(ctx);
        }

        // Notifications (bottom right)
        if !self.notifications.is_empty() {
            let mut notif_stack = vstack();
            for msg in &self.notifications {
                notif_stack = notif_stack.child(
                    Text::new(format!(" {} ", msg))
                        .fg(Color::WHITE)
                        .bg(Color::rgb(60, 60, 60)),
                );
            }
            notif_stack.render(ctx);
        }
    }
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let ide = IdeApp::new();

    app.run_with_handler(ide, |key_event, ide| ide.handle_key(&key_event.key))
}
