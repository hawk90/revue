//! IDE Example - Demonstrates Dock, Command Palette, TextArea, Notifications
//!
//! A mini IDE-like application showing Revue's advanced features working together.
//! The layout is a `dock()`: the Explorer on the left, one editor tab per open
//! file in the center, and a terminal panel at the bottom. Drag the dividers
//! with the mouse, click a tab to switch files, and toggle the sidebar and the
//! terminal from the command palette.
//!
//! Run with: cargo run --example ide

use revue::prelude::*;
use revue::utils::unicode::display_width;
use revue::widget::{dock, Command, CommandPalette, DockPosition, DockState, TabBar, TextArea};

use DockPosition::{Bottom, Center, Left};

/// Height of the command palette overlay (3 lines + border).
const PALETTE_ROWS: u16 = 5;

/// Columns `text` occupies, for sizing single-line text in an `hstack`.
fn cols(text: &str) -> u16 {
    display_width(text) as u16
}

/// Main IDE application state
struct IdeApp {
    /// Command palette visibility
    command_palette_open: bool,
    /// Command palette state
    command_palette: CommandPalette,
    /// File tree items
    files: Vec<FileItem>,
    /// Selected file index
    selected_file: usize,
    /// Open files, one editor tab each
    open: Vec<OpenFile>,
    /// Area sizes, the collapsed sidebar or terminal, the selected tab
    dock: DockState,
    /// Status messages
    status_message: String,
    /// Mode (Normal, Insert, Command)
    mode: EditorMode,
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

impl FileItem {
    fn new(name: &str, is_dir: bool, modified: bool) -> Self {
        Self {
            name: name.into(),
            is_dir,
            modified,
        }
    }
}

/// A file open in an editor tab
struct OpenFile {
    name: String,
    editor: TextArea,
}

impl OpenFile {
    fn new(name: &str, content: &str) -> Self {
        let mut editor = TextArea::new();
        editor.set_content(content);
        Self {
            name: name.into(),
            editor,
        }
    }
}

const SAMPLE_CODE: &str = r#"//! Revue - A Vue-style TUI framework for Rust
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
            FileItem::new("src/", true, false),
            FileItem::new("  main.rs", false, true),
            FileItem::new("  lib.rs", false, false),
            FileItem::new("  app/", true, false),
            FileItem::new("    mod.rs", false, false),
            FileItem::new("    screen.rs", false, true),
            FileItem::new("Cargo.toml", false, false),
            FileItem::new("README.md", false, false),
        ];

        // The terminal starts hidden; "Toggle Terminal" shows it
        let mut dock = DockState::new();
        dock.set_collapsed(Bottom, true);

        Self {
            command_palette_open: false,
            command_palette: CommandPalette::new().commands(commands),
            files,
            selected_file: 1,
            open: vec![OpenFile::new("main.rs", SAMPLE_CODE)],
            dock,
            status_message: "Ready".into(),
            mode: EditorMode::Normal,
            notifications: Vec::new(),
        }
    }

    /// The file in the selected editor tab
    fn current_file(&self) -> &OpenFile {
        let selected = self.dock.tabs(Center).selected();
        self.open
            .iter()
            .find(|f| Some(&f.name) == selected.as_ref())
            .unwrap_or(&self.open[0])
    }

    fn editor_mut(&mut self) -> &mut TextArea {
        let selected = self.dock.tabs(Center).selected();
        let index = self
            .open
            .iter()
            .position(|f| Some(&f.name) == selected.as_ref())
            .unwrap_or(0);
        &mut self.open[index].editor
    }

    /// Open the file selected in the Explorer in its own tab, or switch to
    /// its tab if it is open
    fn open_selected_file(&mut self) {
        let Some(item) = self.files.get(self.selected_file) else {
            return;
        };
        if item.is_dir {
            return;
        }
        let name = item.name.trim().to_string();
        if !self.open.iter().any(|f| f.name == name) {
            let content = format!("// {name}\n");
            self.open.push(OpenFile::new(&name, &content));
        }
        self.dock.tabs_mut(Center).select(name.clone());
        self.status_message = format!("Opened {name}");
    }

    fn handle_event(&mut self, event: &Event) -> bool {
        match event {
            // Drag the dividers, click a tab
            Event::Mouse(mouse) if !self.command_palette_open => self.dock.handle_mouse(mouse),
            Event::Key(key) => self.handle_key(&key.key),
            _ => false,
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
                self.editor_mut().move_down();
                true
            }
            Key::Char('k') | Key::Up => {
                self.editor_mut().move_up();
                true
            }
            Key::Char('h') | Key::Left => {
                self.editor_mut().move_left();
                true
            }
            Key::Char('l') | Key::Right => {
                self.editor_mut().move_right();
                true
            }
            Key::Char('p') => {
                self.command_palette_open = true;
                true
            }
            Key::Tab => {
                // Navigate file tree
                self.selected_file = (self.selected_file + 1) % self.files.len();
                true
            }
            Key::Enter => {
                self.open_selected_file();
                true
            }
            // Previous / next editor tab
            Key::Char('[') => self.dock.tabs_mut(Center).select_prev(),
            Key::Char(']') => self.dock.tabs_mut(Center).select_next(),
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
                self.editor_mut().insert_char(*c);
                true
            }
            Key::Enter => {
                self.editor_mut().insert_char('\n');
                true
            }
            Key::Backspace => {
                self.editor_mut().delete_char_before();
                true
            }
            Key::Delete => {
                self.editor_mut().delete_char_at();
                true
            }
            Key::Up => {
                self.editor_mut().move_up();
                true
            }
            Key::Down => {
                self.editor_mut().move_down();
                true
            }
            Key::Left => {
                self.editor_mut().move_left();
                true
            }
            Key::Right => {
                self.editor_mut().move_right();
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
            "file.save" => self.status_message = format!("Saved: {}", self.current_file().name),
            "view.sidebar" => self.dock.toggle(Left),
            "view.terminal" => self.dock.toggle(Bottom),
            _ => {}
        }
    }

    fn execute_vim_command(&mut self, cmd: &str) {
        match cmd {
            "w" => self.status_message = format!("\"{}\" written", self.current_file().name),
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
            let icon = if file.is_dir { "📁 " } else { "📄 " };
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

            tree = tree.child_sized(text, 1);
        }

        Border::rounded().title("Explorer").child(tree)
    }

    /// An editor for one open file: line numbers, the text, the cursor
    fn render_editor(&self, file: &OpenFile) -> impl View {
        let text = file.editor.get_content();
        let lines = text.lines().count().max(1);
        let line_width = lines.to_string().len();

        let mut content = vstack();
        for (i, line) in text.lines().enumerate() {
            let line_num = format!("{:>width$} ", i + 1, width = line_width);
            // The gutter is sized to its text and each line to one row.
            let row = hstack()
                .child_sized(
                    Text::new(&line_num).fg(Color::rgb(100, 100, 100)),
                    cols(&line_num),
                )
                .child(Text::new(line));
            content = content.child_sized(row, 1);
        }

        let (cursor_row, cursor_col) = file.editor.cursor_position();
        let cursor_info = format!("Ln {}, Col {}", cursor_row + 1, cursor_col + 1);

        vstack()
            .child_flex(Border::single().child(content), 1.0)
            .child_sized(Text::new(cursor_info).fg(Color::rgb(128, 128, 128)), 1)
    }

    fn render_terminal(&self) -> impl View {
        vstack()
            .child_sized(Text::new("$ cargo run --example ide"), 1)
            .child_sized(
                Text::new("   Compiling revue v3 (examples)").fg(Color::GREEN),
                1,
            )
            .child_sized(Text::new("    Finished `dev` profile").fg(Color::GREEN), 1)
    }

    fn render_status_bar(&self) -> impl View {
        let mode_label = format!(" {} ", self.mode.name());
        let mode_text = Text::new(&mode_label)
            .fg(Color::BLACK)
            .bg(self.mode.color())
            .bold();

        let file_label = format!("  {} ", self.current_file().name);
        let file_text = Text::new(&file_label);
        let status_text =
            Text::new(format!("  {} ", self.status_message)).fg(Color::rgb(180, 180, 180));

        let (cursor_row, cursor_col) = self.current_file().editor.cursor_position();
        let pos_label = format!(" {}:{} ", cursor_row + 1, cursor_col + 1);
        let pos_text = Text::new(&pos_label);

        hstack()
            .child_sized(mode_text, cols(&mode_label))
            .child_sized(file_text, cols(&file_label))
            .child_flex(status_text, 1.0)
            .child_sized(pos_text, cols(&pos_label))
    }

    fn render_command_palette(&self) -> impl View {
        if !self.command_palette_open {
            return vstack(); // Empty
        }

        let search_box = Border::rounded().title("Command Palette").child(
            vstack()
                .child_sized(
                    Text::new(format!("> {}", self.command_palette.get_query())),
                    1,
                )
                .child_sized(Text::new("─".repeat(40)).fg(Color::rgb(80, 80, 80)), 1)
                .child_sized(
                    Text::new("(Use ↑↓ to select, Enter to execute)").fg(Color::rgb(100, 100, 100)),
                    1,
                ),
        );

        vstack().child_sized(search_box, PALETTE_ROWS)
    }
}

impl View for IdeApp {
    fn render(&self, ctx: &mut RenderContext) {
        // The workbench: Explorer | editor tabs, over a terminal
        let workbench = self
            .open
            .iter()
            .fold(dock(&self.dock), |d, file| {
                d.panel(Center, file.name.as_str(), self.render_editor(file))
            })
            .panel(Left, "Explorer", self.render_file_tree())
            .panel(Bottom, "Terminal", self.render_terminal())
            .size(Left, 0.25)
            .min_size(Left, 12)
            .size(Bottom, 0.3)
            // An editor shows its file's tab even when it is the only one
            .tab_bar(Center, TabBar::Always)
            .tab_bar(Bottom, TabBar::Always);

        // Header
        let header = hstack()
            .child_sized(
                Text::new(" Revue IDE ").fg(Color::CYAN).bold(),
                cols(" Revue IDE "),
            )
            .child(
                Text::new(
                    " | p: Commands | Tab: Files | Enter: Open | [ ]: Tabs | i: Insert | :: Command ",
                )
                .fg(Color::rgb(100, 100, 100)),
            );

        // Main view
        let main_view = vstack()
            .child_sized(header, 1)
            .child(workbench)
            .child_sized(self.render_status_bar(), 1);

        main_view.render(ctx);

        // Overlay command palette
        if self.command_palette_open {
            // Wipe the rows the palette covers so the editor doesn't show through.
            ctx.clear(0, 0, ctx.area.width, PALETTE_ROWS);
            self.render_command_palette().render(ctx);
        }

        // Notifications (bottom right, just above the status bar)
        if !self.notifications.is_empty() {
            let (x, y, w, h) =
                notification_area(ctx.area.width, ctx.area.height, &self.notifications);
            let mut notif_stack = vstack();
            for msg in &self.notifications {
                // Pad every line to the box width so the editor can't show
                // through beside a short message.
                let line = format!(" {} ", msg);
                let pad = (w as usize).saturating_sub(display_width(&line));
                notif_stack = notif_stack.child_sized(
                    Text::new(format!("{}{}", line, " ".repeat(pad)))
                        .fg(Color::WHITE)
                        .bg(Color::rgb(60, 60, 60)),
                    1,
                );
            }
            ctx.clear(x, y, w, h);
            let area = ctx.sub_area(x, y, w, h);
            notif_stack.render(&mut ctx.sub_ctx(area));
        }
    }
}

/// Where the notification stack goes, relative to the screen area: flush
/// right, with its last line on the row just above the status bar.
/// Returns `(x, y, width, height)`.
fn notification_area(width: u16, height: u16, messages: &[String]) -> (u16, u16, u16, u16) {
    let widest = messages
        .iter()
        .map(|msg| display_width(msg) + 2)
        .max()
        .unwrap_or(0);
    let w = widest.min(width as usize) as u16;
    let above_status = height.saturating_sub(1);
    let h = messages.len().min(above_status as usize) as u16;
    (width - w, above_status - h, w, h)
}

fn main() -> Result<()> {
    let mut app = App::builder().build();
    let ide = IdeApp::new();

    app.run(ide, |event, ide, _app| ide.handle_event(event))
}

#[cfg(test)]
mod tests {
    use super::*;
    use revue::testing::TestApp;

    fn screen(ide: IdeApp) -> TestApp<IdeApp> {
        let mut app = TestApp::with_size(ide, 80, 24);
        app.render();
        app
    }

    #[test]
    fn notification_area_sits_bottom_right_above_status_bar() {
        let msgs = vec!["Command: file.new".to_string(), "short".to_string()];
        // The widest line is " Command: file.new " = 19 columns.
        assert_eq!(notification_area(80, 24, &msgs), (61, 21, 19, 2));
    }

    #[test]
    fn notification_area_fits_a_tiny_screen() {
        let msgs = vec!["a very long notification message".to_string(); 3];
        assert_eq!(notification_area(10, 3, &msgs), (0, 0, 10, 2));
    }

    #[test]
    fn notifications_render_bottom_right() {
        let mut ide = IdeApp::new();
        ide.execute_command("file.new");
        let app = screen(ide);

        assert!(app.get_line(22).ends_with(" Command: file.new"));
        assert!(!app.get_line(0).contains("Command: file.new"));
        assert!(app.get_line(23).contains("New file created"));
    }

    #[test]
    fn the_explorer_and_the_editor_tab_share_the_middle() {
        let app = screen(IdeApp::new());
        let (explorer_x, _) = app.find_text("Explorer").expect("explorer");
        let (tab_x, tab_y) = app.find_text("main.rs").expect("editor tab");
        assert!(explorer_x < 20 && tab_x > 20, "explorer left, editor right");
        assert_eq!(tab_y, 1, "the editor's tab bar sits under the header");
        assert!(!app.contains("Terminal"), "the terminal starts hidden");
    }

    #[test]
    fn the_sidebar_and_terminal_toggle_through_the_dock() {
        let mut ide = IdeApp::new();
        ide.execute_command("view.sidebar");
        ide.execute_command("view.terminal");
        let app = screen(ide);

        assert!(!app.contains("Explorer"));
        assert_eq!(app.find_text("main.rs").map(|(x, _)| x), Some(1));
        assert!(app.contains("Terminal"));
        assert!(app.contains("$ cargo run"));
    }

    #[test]
    fn opening_a_file_adds_and_selects_its_tab() {
        let mut ide = IdeApp::new();
        ide.selected_file = 2; // lib.rs
        ide.handle_key(&Key::Enter);
        assert_eq!(ide.current_file().name, "lib.rs");

        let app = screen(ide);
        let tab_bar = app.get_line(1);
        assert!(tab_bar.contains("main.rs") && tab_bar.contains("lib.rs"));
        assert!(app.contains("// lib.rs"), "the new tab's editor is shown");
    }

    #[test]
    fn typing_edits_the_selected_tab_only() {
        let mut ide = IdeApp::new();
        ide.selected_file = 2;
        ide.handle_key(&Key::Enter);
        let main_rs = ide.open[0].editor.get_content();
        ide.handle_key(&Key::Char('i'));
        ide.handle_key(&Key::Char('x'));

        assert!(ide.open[1].editor.get_content().starts_with('x'));
        assert_eq!(ide.open[0].editor.get_content(), main_rs);
    }

    #[test]
    fn dragging_the_sidebar_divider_resizes_it() {
        let mut app = screen(IdeApp::new());
        // The dock's divider, between the Explorer's border and the
        // editor's: the middle of three bars
        let divider = |app: &TestApp<IdeApp>| {
            (1..79u16).find(|&x| (x - 1..=x + 1).all(|x| app.get_cell(x, 5) == Some('│')))
        };
        let before = divider(&app).expect("a divider");

        let ide = app.view_mut();
        let button = MouseButton::Left;
        for kind in [
            MouseEventKind::Down(button),
            MouseEventKind::Drag(button),
            MouseEventKind::Up(button),
        ] {
            let x = if matches!(kind, MouseEventKind::Down(_)) {
                before
            } else {
                before + 10
            };
            assert!(ide.handle_event(&Event::Mouse(MouseEvent::new(x, 5, kind))));
        }
        app.render();
        assert_eq!(divider(&app), Some(before + 10));
    }
}
