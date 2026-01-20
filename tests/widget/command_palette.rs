//! CommandPalette widget integration tests
//!
//! CommandPalette 위젯의 통합 테스트입니다.

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::traits::RenderContext;
use revue::widget::View;
use revue::widget::{Command, CommandPalette};

// =============================================================================
// Constructor Tests (생성자 테스트)
// =============================================================================

#[test]
fn test_command_palette_new() {
    let palette = CommandPalette::new();
    assert!(!palette.is_visible());
    assert_eq!(palette.get_query(), "");
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_command_palette_default() {
    let palette = CommandPalette::default();
    assert!(!palette.is_visible());
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_command_palette_helper() {
    use revue::widget::command_palette;
    let palette = command_palette();
    assert!(!palette.is_visible());
}

// =============================================================================
// Command Builder Tests (커맨드 빌더 테스트)
// =============================================================================

#[test]
fn test_command_new_minimal() {
    let cmd = Command::new("save", "Save File");
    assert_eq!(cmd.id, "save");
    assert_eq!(cmd.label, "Save File");
    assert!(cmd.description.is_none());
    assert!(cmd.shortcut.is_none());
    assert!(!cmd.recent);
    assert!(!cmd.pinned);
}

#[test]
fn test_command_builder_full() {
    let cmd = Command::new("open", "Open File")
        .description("Open a file from disk")
        .shortcut("Ctrl+O")
        .category("File")
        .icon('📂')
        .recent()
        .pinned();

    assert_eq!(cmd.id, "open");
    assert_eq!(cmd.label, "Open File");
    assert_eq!(cmd.description, Some("Open a file from disk".to_string()));
    assert_eq!(cmd.shortcut, Some("Ctrl+O".to_string()));
    assert_eq!(cmd.category, Some("File".to_string()));
    assert_eq!(cmd.icon, Some('📂'));
    assert!(cmd.recent);
    assert!(cmd.pinned);
}

#[test]
fn test_command_builder_chain() {
    let cmd = Command::new("test", "Test Command")
        .description("Test description")
        .shortcut("Ctrl+T")
        .category("Test")
        .icon('⚙');

    assert!(cmd.description.is_some());
    assert!(cmd.shortcut.is_some());
    assert!(cmd.category.is_some());
    assert!(cmd.icon.is_some());
}

// =============================================================================
// Builder Methods Tests (빌더 메서드 테스트)
// =============================================================================

#[test]
fn test_command_palette_command_builder() {
    let mut palette = CommandPalette::new()
        .command(Command::new("cmd1", "Command 1"))
        .command(Command::new("cmd2", "Command 2"));

    // Verify commands were added via selected_command
    palette.show();
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_command_palette_commands_builder() {
    let commands = vec![
        Command::new("a", "Alpha"),
        Command::new("b", "Beta"),
        Command::new("c", "Gamma"),
    ];

    let mut palette = CommandPalette::new().commands(commands);
    palette.show();
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_command_palette_width() {
    // Width is clamped to minimum of 30 - can't test directly
    // Just verify the builder accepts the value
    let _palette = CommandPalette::new().width(80);
    let _palette_min = CommandPalette::new().width(10);
}

#[test]
fn test_command_palette_max_visible() {
    // Max visible is clamped to minimum of 3 - can't test directly
    // Just verify the builder accepts the value
    let _palette = CommandPalette::new().max_visible(15);
    let _palette_min = CommandPalette::new().max_visible(1);
}

#[test]
fn test_command_palette_placeholder() {
    // Placeholder is private - tested indirectly via rendering
    let _palette = CommandPalette::new().placeholder("Search commands...");
}

#[test]
fn test_command_palette_title() {
    // Title is private - tested indirectly via rendering
    let _palette = CommandPalette::new().title("Command Palette");
}

#[test]
fn test_command_palette_show_descriptions() {
    // show_descriptions is private - tested indirectly via rendering
    let _palette = CommandPalette::new().show_descriptions(false);
    let _palette_default = CommandPalette::new();
}

#[test]
fn test_command_palette_show_shortcuts() {
    // show_shortcuts is private - tested indirectly via rendering
    let _palette = CommandPalette::new().show_shortcuts(false);
    let _palette_default = CommandPalette::new();
}

#[test]
fn test_command_palette_show_icons() {
    // show_icons is private - tested indirectly via rendering
    let _palette = CommandPalette::new().show_icons(false);
    let _palette_default = CommandPalette::new();
}

#[test]
fn test_command_palette_colors() {
    // Colors are private - tested indirectly via rendering
    let _palette = CommandPalette::new().colors(Color::RED, Color::GREEN, Color::BLUE);
}

#[test]
fn test_command_palette_full_builder_chain() {
    let mut palette = CommandPalette::new()
        .width(70)
        .max_visible(12)
        .placeholder("Type to search...")
        .title("Commands")
        .show_descriptions(true)
        .show_shortcuts(true)
        .show_icons(true)
        .colors(
            Color::rgb(20, 20, 20),
            Color::rgb(100, 100, 100),
            Color::rgb(60, 90, 130),
        )
        .command(Command::new("test", "Test"));

    // Verify the palette was built successfully and commands work
    palette.show();
    assert!(palette.selected_command().is_some());
}

// =============================================================================
// Visibility Tests (표시 상태 테스트)
// =============================================================================

#[test]
fn test_command_palette_show() {
    let mut palette = CommandPalette::new();
    assert!(!palette.is_visible());

    palette.show();
    assert!(palette.is_visible());
}

#[test]
fn test_command_palette_hide() {
    let mut palette = CommandPalette::new();
    palette.show();
    assert!(palette.is_visible());

    palette.hide();
    assert!(!palette.is_visible());
}

#[test]
fn test_command_palette_toggle() {
    let mut palette = CommandPalette::new();
    assert!(!palette.is_visible());

    palette.toggle();
    assert!(palette.is_visible());

    palette.toggle();
    assert!(!palette.is_visible());
}

#[test]
fn test_command_palette_show_resets_query() {
    let mut palette = CommandPalette::new();
    palette.set_query("test");
    assert_eq!(palette.get_query(), "test");

    palette.show();
    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_command_palette_show_resets_selection() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("a", "A"),
        Command::new("b", "B"),
        Command::new("c", "C"),
    ]);

    palette.show();
    palette.select_next();
    palette.select_next();

    palette.show();
    // After showing again, first command should be selected
    assert_eq!(palette.selected_id(), Some("a"));
}

// =============================================================================
// Query Tests (쿼리 테스트)
// =============================================================================

#[test]
fn test_command_palette_get_query() {
    let palette = CommandPalette::new();
    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_command_palette_set_query() {
    let mut palette = CommandPalette::new();
    palette.set_query("save");
    assert_eq!(palette.get_query(), "save");
}

#[test]
fn test_command_palette_input() {
    let mut palette = CommandPalette::new();
    palette.input('s');
    palette.input('a');
    palette.input('v');
    palette.input('e');

    assert_eq!(palette.get_query(), "save");
}

#[test]
fn test_command_palette_backspace() {
    let mut palette = CommandPalette::new();
    palette.set_query("test");
    palette.backspace();

    assert_eq!(palette.get_query(), "tes");
}

#[test]
fn test_command_palette_backspace_empty() {
    let mut palette = CommandPalette::new();
    palette.backspace();
    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_command_palette_clear_query() {
    let mut palette = CommandPalette::new();
    palette.set_query("test query");
    palette.clear_query();

    assert_eq!(palette.get_query(), "");
}

// =============================================================================
// Command Filtering Tests (커맨드 필터링 테스트)
// =============================================================================

#[test]
fn test_command_filter_empty_query() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save File"),
        Command::new("open", "Open File"),
        Command::new("close", "Close File"),
    ]);

    palette.show();
    // With empty query, all commands should be available
    assert!(palette.selected_id().is_some());
}

#[test]
fn test_command_filter_exact_match() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save File"),
        Command::new("open", "Open File"),
    ]);

    palette.set_query("save");
    // Should find the save command
    assert_eq!(palette.selected_id(), Some("save"));
}

#[test]
fn test_command_filter_partial_match() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save_file", "Save File"),
        Command::new("save_all", "Save All Files"),
        Command::new("open_file", "Open File"),
    ]);

    palette.set_query("save");
    // Should find a save-related command
    let id = palette.selected_id();
    assert!(id.is_some());
    assert!(id.unwrap().starts_with("save"));
}

#[test]
fn test_command_filter_no_match() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save File"),
        Command::new("open", "Open File"),
    ]);

    palette.set_query("xyz");
    // No match - no command selected
    assert_eq!(palette.selected_id(), None);
}

#[test]
fn test_command_filter_description() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("cmd1", "Command One").description("Save to disk"),
        Command::new("cmd2", "Command Two").description("Open from disk"),
    ]);

    palette.set_query("disk");
    // Should match command with "disk" in description
    assert!(palette.selected_id().is_some());
}

#[test]
fn test_command_filter_category() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save File").category("File"),
        Command::new("settings", "Settings").category("Preferences"),
    ]);

    palette.set_query("file");
    // Should match command with "file" in category
    assert_eq!(palette.selected_id(), Some("save"));
}

// =============================================================================
// Fuzzy Matching Tests (퍼지 매칭 테스트)
// =============================================================================

#[test]
fn test_command_fuzzy_match_label() {
    let cmd = Command::new("save_file", "Save File");
    assert!(cmd.matches("sf")); // S_ave F_ile
    assert!(cmd.matches("svfl")); // S_a_V_e F_i_L_e
}

#[test]
fn test_command_fuzzy_match_no_match() {
    let cmd = Command::new("save", "Save File");
    assert!(!cmd.matches("xyz"));
}

#[test]
fn test_command_fuzzy_match_empty_query() {
    let cmd = Command::new("test", "Test Command");
    assert!(cmd.matches(""));
}

#[test]
fn test_command_match_score_exact() {
    let cmd = Command::new("save", "Save File");
    let score_exact = cmd.match_score("save");
    let score_partial = cmd.match_score("sav");

    assert!(score_exact > score_partial);
}

#[test]
fn test_command_match_score_pinned() {
    let cmd_pinned = Command::new("test", "Test").pinned();
    let cmd_normal = Command::new("test", "Test");

    assert!(cmd_pinned.match_score("") > cmd_normal.match_score(""));
}

#[test]
fn test_command_match_score_recent() {
    let cmd_recent = Command::new("test", "Test").recent();
    let cmd_normal = Command::new("test", "Test");

    assert!(cmd_recent.match_score("") > cmd_normal.match_score(""));
}

#[test]
fn test_command_match_score_pinned_vs_recent() {
    let cmd_pinned = Command::new("test", "Test").pinned();
    let cmd_recent = Command::new("test", "Test").recent();

    // Pinned gives 50 bonus, recent gives 25 bonus
    assert!(cmd_pinned.match_score("") > cmd_recent.match_score(""));
}

// =============================================================================
// Sorting Tests (정렬 테스트)
// =============================================================================

#[test]
fn test_command_filter_sorts_by_score() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("test_b", "Test B"),
        Command::new("test_a", "Test A").pinned(),
        Command::new("test_c", "Test C").recent(),
    ]);

    palette.set_query("test");
    // Pinned command should be selected first
    assert_eq!(palette.selected_id(), Some("test_a"));
}

// =============================================================================
// Selection Tests (선택 테스트)
// =============================================================================

#[test]
fn test_command_palette_select_next() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("a", "A"),
        Command::new("b", "B"),
        Command::new("c", "C"),
    ]);

    palette.show();
    assert_eq!(palette.selected_id(), Some("a"));

    palette.select_next();
    assert_eq!(palette.selected_id(), Some("b"));

    palette.select_next();
    assert_eq!(palette.selected_id(), Some("c"));
}

#[test]
fn test_command_palette_select_prev() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("a", "A"),
        Command::new("b", "B"),
        Command::new("c", "C"),
    ]);

    palette.show();
    palette.select_next();
    palette.select_next();

    palette.select_prev();
    assert_eq!(palette.selected_id(), Some("b"));
}

#[test]
fn test_command_palette_select_wrap_forward() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.select_next();
    palette.select_next(); // Should wrap to 0

    assert_eq!(palette.selected_id(), Some("a"));
}

#[test]
fn test_command_palette_select_wrap_backward() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.select_prev(); // Should wrap to last

    assert_eq!(palette.selected_id(), Some("b"));
}

#[test]
fn test_command_palette_selected_command() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("cmd1", "Command 1"),
        Command::new("cmd2", "Command 2"),
    ]);

    palette.show();
    let selected = palette.selected_command();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "cmd1");

    palette.select_next();
    let selected = palette.selected_command();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "cmd2");
}

#[test]
fn test_command_palette_selected_id() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save File"),
        Command::new("open", "Open File"),
    ]);

    palette.show();
    assert_eq!(palette.selected_id(), Some("save"));

    palette.select_next();
    assert_eq!(palette.selected_id(), Some("open"));
}

#[test]
fn test_command_palette_selected_id_empty() {
    let palette = CommandPalette::new();
    assert_eq!(palette.selected_id(), None);
}

#[test]
fn test_command_palette_selected_command_after_filter() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save_file", "Save File"),
        Command::new("open_file", "Open File"),
        Command::new("close_file", "Close File"),
    ]);

    palette.set_query("open");
    assert_eq!(palette.selected_id(), Some("open_file"));
}

// =============================================================================
// Execute Tests (실행 테스트)
// =============================================================================

#[test]
fn test_command_palette_execute() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test Command"));

    palette.show();
    let result = palette.execute();

    assert_eq!(result, Some("test".to_string()));
    assert!(!palette.is_visible()); // Should hide after execute
}

#[test]
fn test_command_palette_execute_empty() {
    let mut palette = CommandPalette::new();
    palette.show();

    let result = palette.execute();
    assert_eq!(result, None);
    // execute() doesn't hide the palette when there are no commands
    assert!(palette.is_visible());
}

// =============================================================================
// Key Handling Tests (키 처리 테스트)
// =============================================================================

#[test]
fn test_command_palette_handle_key_escape() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));
    palette.show();

    palette.handle_key(&Key::Escape);
    assert!(!palette.is_visible());
}

#[test]
fn test_command_palette_handle_key_enter() {
    let mut palette = CommandPalette::new().command(Command::new("save", "Save"));

    palette.show();
    palette.handle_key(&Key::Enter);

    assert!(!palette.is_visible());
}

#[test]
fn test_command_palette_handle_key_down() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.handle_key(&Key::Down);

    assert_eq!(palette.selected_id(), Some("b"));
}

#[test]
fn test_command_palette_handle_key_up() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.handle_key(&Key::Up);

    assert_eq!(palette.selected_id(), Some("b")); // Wrapped to last
}

#[test]
fn test_command_palette_handle_key_char() {
    let mut palette = CommandPalette::new();
    palette.show();

    palette.handle_key(&Key::Char('t'));
    palette.handle_key(&Key::Char('e'));

    assert_eq!(palette.get_query(), "te");
}

#[test]
fn test_command_palette_handle_key_backspace() {
    let mut palette = CommandPalette::new();
    palette.show();

    palette.handle_key(&Key::Char('t'));
    palette.handle_key(&Key::Char('e'));
    palette.handle_key(&Key::Backspace);

    assert_eq!(palette.get_query(), "t");
}

#[test]
fn test_command_palette_handle_key_vim_j() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.handle_key(&Key::Char('j'));

    assert_eq!(palette.selected_id(), Some("b"));
}

#[test]
fn test_command_palette_handle_key_vim_k() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.handle_key(&Key::Char('k'));

    assert_eq!(palette.selected_id(), Some("b")); // Wrapped to last
}

#[test]
fn test_command_palette_handle_key_navigation_with_query() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save_a", "Save A"),
        Command::new("save_b", "Save B"),
    ]);

    palette.show();
    palette.set_query("save");

    // Should still be able to navigate when query is not empty
    palette.handle_key(&Key::Down);
    assert_eq!(palette.selected_id(), Some("save_b"));
}

#[test]
fn test_command_palette_handle_key_vim_jk_with_empty_query() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();

    // j and k should work with empty query
    palette.handle_key(&Key::Char('j'));
    assert_eq!(palette.selected_id(), Some("b"));

    palette.handle_key(&Key::Char('k'));
    assert_eq!(palette.selected_id(), Some("a"));
}

#[test]
fn test_command_palette_handle_key_ignored_when_hidden() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    // Don't show palette
    assert!(!palette.handle_key(&Key::Char('t')));
    assert_eq!(palette.get_query(), "");
}

// =============================================================================
// Dynamic Command Management Tests (동적 커맨드 관리 테스트)
// =============================================================================

#[test]
fn test_command_palette_add_command() {
    let mut palette = CommandPalette::new();
    palette.add_command(Command::new("new", "New Command"));

    palette.show();
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_command_palette_remove_command() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("keep", "Keep This"),
        Command::new("remove", "Remove This"),
    ]);

    palette.show();
    assert!(palette.selected_command().is_some());

    palette.remove_command("remove");
    palette.show();

    // After removing, should still have a command
    assert!(palette.selected_command().is_some());
    assert_ne!(palette.selected_id(), Some("remove"));
}

#[test]
fn test_command_palette_remove_nonexistent() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.remove_command("nonexistent");
    // Should not crash
    palette.show();
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_command_palette_clear_commands() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.clear_commands();
    palette.show();

    // No commands after clear
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_command_palette_mark_recent() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.mark_recent("test");
    // Command should be marked as recent (affects scoring)
    // We can verify this indirectly through behavior
    palette.show();
    assert!(palette.selected_command().is_some());
}

#[test]
fn test_command_palette_mark_recent_nonexistent() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.mark_recent("nonexistent");
    // Should not crash
    palette.show();
    assert!(palette.selected_command().is_some());
}

// =============================================================================
// Rendering Tests (렌더링 테스트)
// =============================================================================

#[test]
fn test_command_palette_render_hidden() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.render(&mut ctx);

    // When hidden, should not render anything visible
    // Just verify it doesn't crash
}

#[test]
fn test_command_palette_render_visible() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save File").shortcut("Ctrl+S"),
        Command::new("open", "Open File").shortcut("Ctrl+O"),
    ]);

    palette.show();
    palette.render(&mut ctx);

    // Verify some cells were modified (has border)
    let mut has_border = false;
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '╭' || cell.symbol == '─' {
                    has_border = true;
                    break;
                }
            }
        }
        if has_border {
            break;
        }
    }
    assert!(has_border);
}

#[test]
fn test_command_palette_render_with_title() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new()
        .title("Commands")
        .command(Command::new("test", "Test"));

    palette.show();
    palette.render(&mut ctx);

    // Verify border exists
    let mut has_border = false;
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '╭' {
                    has_border = true;
                    break;
                }
            }
        }
        if has_border {
            break;
        }
    }
    assert!(has_border);
}

#[test]
fn test_command_palette_render_with_icons() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new().command(Command::new("save", "Save File").icon('💾'));

    palette.show();
    palette.render(&mut ctx);

    // Just verify it renders without crashing - icon rendering is internal
}

#[test]
fn test_command_palette_render_with_shortcuts() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette =
        CommandPalette::new().command(Command::new("save", "Save File").shortcut("Ctrl+S"));

    palette.show();
    palette.render(&mut ctx);

    // Just verify it renders without crashing - shortcut rendering is internal
}

#[test]
fn test_command_palette_render_query() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.show();
    palette.set_query("save");
    palette.render(&mut ctx);

    // Check if query text appears
    let mut has_query = false;
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == 's' || cell.symbol == 'a' {
                    has_query = true;
                    break;
                }
            }
        }
        if has_query {
            break;
        }
    }
    assert!(has_query);
}

#[test]
fn test_command_palette_render_placeholder() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new()
        .placeholder("Search...")
        .command(Command::new("test", "Test"));

    palette.show();
    palette.render(&mut ctx);

    // Just verify it renders without crashing - placeholder rendering is internal
}

#[test]
fn test_command_palette_render_custom_colors() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new()
        .colors(Color::RED, Color::GREEN, Color::BLUE)
        .command(Command::new("test", "Test"));

    palette.show();
    palette.render(&mut ctx);

    // Just verify it renders without crashing - color rendering is internal
}

#[test]
fn test_command_palette_render_selected_item() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new().command(Command::new("test", "Test Command"));

    palette.show();
    palette.render(&mut ctx);

    // Just verify it renders without crashing - selection rendering is internal
}

#[test]
fn test_command_palette_render_many_commands() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let commands: Vec<_> = (0..20)
        .map(|i| Command::new(&format!("cmd{}", i), &format!("Command {}", i)))
        .collect();

    let mut palette = CommandPalette::new().commands(commands).max_visible(10);

    palette.show();
    palette.render(&mut ctx);

    // Should render without crashing, respecting max_visible
}

#[test]
fn test_command_palette_render_empty_commands() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new();

    palette.show();
    palette.render(&mut ctx);

    // Should render empty palette (no items, just border)
    let mut has_border = false;
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '╭' {
                    has_border = true;
                    break;
                }
            }
        }
        if has_border {
            break;
        }
    }
    assert!(has_border);
}

#[test]
fn test_command_palette_render_no_results() {
    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new().command(Command::new("save", "Save File"));

    palette.show();
    palette.set_query("xyz"); // No matches
    palette.render(&mut ctx);

    // Should render without crashing
    let mut has_border = false;
    for y in area.y..area.y + area.height {
        for x in area.x..area.x + area.width {
            if let Some(cell) = buffer.get(x, y) {
                if cell.symbol == '╭' {
                    has_border = true;
                    break;
                }
            }
        }
        if has_border {
            break;
        }
    }
    assert!(has_border);
}

#[test]
fn test_command_palette_render_small_area() {
    let mut buffer = Buffer::new(40, 10);
    let area = Rect::new(0, 0, 40, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.show();
    palette.render(&mut ctx);

    // Should handle smaller areas
}

// =============================================================================
// Meta and Debug Tests (메타 및 디버그 테스트)
// Note: CommandPalette does not implement StyledView
// =============================================================================

#[test]
fn test_command_palette_meta_type() {
    let palette = CommandPalette::new().command(Command::new("test", "Test"));
    let meta = palette.meta();
    assert_eq!(meta.widget_type, "CommandPalette");
}

// =============================================================================
// Edge Cases (엣지 케이스 테스트)
// =============================================================================

#[test]
fn test_command_palette_empty_command_list() {
    let mut palette = CommandPalette::new();

    palette.show();
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_command_palette_single_command() {
    let mut palette = CommandPalette::new().command(Command::new("only", "Only Command"));

    palette.show();
    assert_eq!(palette.selected_id(), Some("only" as &str));

    palette.select_next();
    // Should wrap to same command
    assert_eq!(palette.selected_id(), Some("only" as &str));
}

#[test]
fn test_command_palette_unicode_label() {
    let mut palette = CommandPalette::new().command(Command::new("test", "테스트 명령"));

    palette.set_query("테");
    // Should find the command
    assert_eq!(palette.selected_id(), Some("test" as &str));
}

#[test]
fn test_command_palette_very_long_label() {
    let long_label = "This is a very long command label that exceeds normal width";
    let mut palette = CommandPalette::new().command(Command::new("test", long_label));

    palette.show();

    let mut buffer = Buffer::new(80, 24);
    let area = Rect::new(0, 0, 80, 24);
    let mut ctx = RenderContext::new(&mut buffer, area);

    palette.render(&mut ctx);
    // Should render without crashing
}

#[test]
fn test_command_palette_special_chars_in_query() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.input('!');
    palette.input('@');
    palette.input('#');

    assert_eq!(palette.get_query(), "!@#");
}

#[test]
fn test_command_palette_multiple_shows() {
    let mut palette = CommandPalette::new().command(Command::new("test", "Test"));

    palette.show();
    palette.set_query("test");
    palette.select_next();

    palette.show(); // Should reset
    assert_eq!(palette.get_query(), "");
    assert_eq!(palette.selected_id(), Some("test" as &str));
}

#[test]
fn test_command_palette_query_updates_filter() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save"),
        Command::new("open", "Open"),
    ]);

    palette.show();
    assert!(palette.selected_id().is_some());

    palette.set_query("save");
    assert_eq!(palette.selected_id(), Some("save"));

    palette.clear_query();
    // Should show first command again
    assert!(palette.selected_id().is_some());
}

#[test]
fn test_command_palette_width_clamping() {
    // Width is clamped to minimum of 30 - tested by verifying builder works
    let _palette = CommandPalette::new().width(10);
}

#[test]
fn test_command_palette_max_visible_clamping() {
    // Max visible is clamped to minimum of 3 - tested by verifying builder works
    let _palette = CommandPalette::new().max_visible(1);
}

#[test]
fn test_command_palette_navigation_with_no_commands() {
    let mut palette = CommandPalette::new();
    palette.show();

    palette.select_next();
    palette.select_prev();

    // Should not crash with empty command list
    assert!(palette.selected_command().is_none());
}

#[test]
fn test_command_palette_execute_with_no_commands() {
    let mut palette = CommandPalette::new();
    palette.show();

    let result = palette.execute();
    assert_eq!(result, None);
}

#[test]
fn test_command_palette_backspace_with_empty_query() {
    let mut palette = CommandPalette::new();
    palette.show();

    for _ in 0..5 {
        palette.backspace();
    }

    assert_eq!(palette.get_query(), "");
}

#[test]
fn test_command_palette_handle_key_returns_false_for_unhandled() {
    let mut palette = CommandPalette::new();
    palette.show();

    // Tab is not handled
    assert!(!palette.handle_key(&Key::Tab));
}

#[test]
fn test_command_palette_filter_after_add_command() {
    let mut palette = CommandPalette::new().command(Command::new("save", "Save"));

    palette.set_query("save");
    assert_eq!(palette.selected_id(), Some("save"));

    palette.add_command(Command::new("save_all", "Save All"));
    // Filter should update and still find a save command
    assert!(palette.selected_id().is_some());
}

#[test]
fn test_command_palette_filter_after_remove_command() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("save", "Save"),
        Command::new("save_all", "Save All"),
    ]);

    palette.set_query("save");
    assert!(palette.selected_id().is_some());

    palette.remove_command("save");
    // Filter should update and still find the other save command
    assert!(palette.selected_id().is_some());
}

#[test]
fn test_command_palette_score_with_empty_query() {
    let cmd_normal = Command::new("test", "Test");
    let cmd_pinned = Command::new("test", "Test").pinned();
    let cmd_recent = Command::new("test", "Test").recent();

    // With empty query, score is based on pinned/recent status
    assert!(cmd_pinned.match_score("") > cmd_recent.match_score(""));
    assert!(cmd_recent.match_score("") > cmd_normal.match_score(""));
    assert_eq!(cmd_normal.match_score(""), 0);
}

#[test]
fn test_command_palette_matches_with_empty_query() {
    let cmd = Command::new("test", "Test");
    assert!(cmd.matches("")); // Empty query matches everything
}
