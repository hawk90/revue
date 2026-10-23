//! CommandPalette widget integration tests
//!
//! CommandPalette 위젯의 통합 테스트입니다.

mod command;
mod helper;
mod impls;
mod props;
mod view;

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::style::Color;
use revue::widget::theme::{DISABLED_FG, SUBTLE_GRAY};
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
    let mut palette = CommandPalette::new()
        .width(80)
        .commands(vec![Command::new("a", "A"), Command::new("b", "B")]);
    palette.show();
    // An 80-column frame in a 100-column area: columns 10..90.
    let b = view::render(&palette, 100, 12);
    assert_eq!(b.get(9, 2).unwrap().symbol, ' ');
    assert_eq!(b.get(10, 2).unwrap().symbol, '╭');
    assert_eq!(b.get(89, 2).unwrap().symbol, '╮');
    assert_eq!(b.get(90, 2).unwrap().symbol, ' ');
}

#[test]
fn test_command_palette_max_visible() {
    let commands: Vec<_> = (0..20)
        .map(|i| Command::new(format!("c{i}"), format!("Command {i}")))
        .collect();
    let mut palette = CommandPalette::new().max_visible(15).commands(commands);
    palette.show();
    for _ in 0..15 {
        palette.select_next();
    }
    // The sixteenth command scrolls the list by one.
    assert_eq!(palette.selection.offset(), 1);
    let b = view::render(&palette, 70, 24);
    assert!(
        view::row(&b, 5).contains("Command 1 "),
        "{}",
        view::row(&b, 5)
    );
    // Rows 5..19 hold 15 commands; the last of them is under the bottom
    // border (#1091), so only 14 are checked.
    assert!(
        view::row(&b, 18).contains("Command 14 "),
        "{}",
        view::row(&b, 18)
    );
}

#[test]
fn test_command_palette_placeholder() {
    let mut palette = CommandPalette::new()
        .placeholder("Run a command...")
        .command(Command::new("a", "A"));
    palette.show();
    // The first letter is under the cursor (#1091).
    let text = view::row(&view::render(&palette, 70, 12), 3);
    assert!(text.contains("un a command..."), "{text}");
    assert!(!text.contains("Type to search"), "{text}");
}

#[test]
fn test_command_palette_title() {
    let mut palette = CommandPalette::new().title("Command Palette");
    palette.show();
    let b = view::render(&palette, 70, 12);
    assert!(
        view::row(&b, 3).contains("│ Command Palette "),
        "{}",
        view::row(&b, 3)
    );
    assert_eq!(b.get(7, 4).unwrap().symbol, '🔍');
}

#[test]
fn test_command_palette_show_descriptions() {
    let mut palette = CommandPalette::new()
        .command(Command::new("a", "Alpha").description("first"))
        .command(Command::new("b", "Beta"));
    palette.show();
    // "Alpha" at column 9, two blanks, then the dimmed description.
    let b = view::render(&palette, 70, 12);
    let desc: String = (16..21).map(|x| b.get(x, 5).unwrap().symbol).collect();
    assert_eq!(desc, "first");
    assert_eq!(b.get(16, 5).unwrap().fg, Some(SUBTLE_GRAY));
}

#[test]
fn test_command_palette_show_shortcuts() {
    let make = |show: bool| {
        let mut p = CommandPalette::new()
            .show_shortcuts(show)
            .command(Command::new("a", "Alpha").shortcut("Ctrl+K"))
            .command(Command::new("b", "Beta"));
        p.show();
        view::row(&view::render(&p, 70, 12), 5)
    };
    assert!(make(true).contains("Ctrl+K │"), "{}", make(true));
    assert!(!make(false).contains("Ctrl+K"), "{}", make(false));
}

#[test]
fn test_command_palette_show_icons() {
    let make = |show: bool| {
        let mut p = CommandPalette::new()
            .show_icons(show)
            .command(Command::new("a", "Alpha").icon('@'))
            .command(Command::new("b", "Beta"));
        p.show();
        view::row(&view::render(&p, 70, 12), 5)
    };
    assert!(make(true).contains("│ @ Alpha"), "{}", make(true));
    assert!(make(false).contains("│ Alpha"), "{}", make(false));
    assert!(!make(false).contains('@'), "{}", make(false));
}

#[test]
fn test_command_palette_colors() {
    let mut palette = CommandPalette::new()
        .colors(Color::RED, Color::GREEN, Color::BLUE)
        .commands(vec![Command::new("a", "A"), Command::new("b", "B")]);
    palette.show();
    let b = view::render(&palette, 70, 12);
    // The frame's background and its border.
    assert_eq!(b.get(6, 3).unwrap().bg, Some(Color::RED));
    for (x, y) in [(5, 2), (6, 2), (64, 2), (5, 3), (5, 4), (64, 5)] {
        assert_eq!(b.get(x, y).unwrap().fg, Some(Color::GREEN), "({x}, {y})");
    }
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
fn test_command_palette_j_and_k_are_typed() {
    // A command can start with j or k: "kill" and "jump" are queries,
    // not moves (#889). Arrows move the selection.
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("kill", "Kill"),
        Command::new("jump", "Jump"),
    ]);
    palette.show();
    for ch in "kill".chars() {
        palette.handle_key(&Key::Char(ch));
    }
    assert_eq!(palette.get_query(), "kill");
    assert_eq!(palette.selected_id(), Some("kill"));
}

#[test]
fn test_command_palette_up_wraps_to_the_last() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.handle_key(&Key::Up);

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
fn test_command_palette_arrows_move_with_an_empty_query() {
    let mut palette =
        CommandPalette::new().commands(vec![Command::new("a", "A"), Command::new("b", "B")]);

    palette.show();
    palette.handle_key(&Key::Down);
    assert_eq!(palette.selected_id(), Some("b"));
    palette.handle_key(&Key::Up);
    assert_eq!(palette.selected_id(), Some("a"));
    assert_eq!(palette.get_query(), "");
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

    assert!((0..24).all(|y| view::row(&buffer, y).trim().is_empty()));
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
    let mut palette = CommandPalette::new()
        .command(Command::new("save", "Save File").icon('💾'))
        .command(Command::new("quit", "Quit"));

    palette.show();
    let b = view::render(&palette, 80, 24);

    // An 80-column area puts the 60-column frame at column 10.
    assert_eq!(b.get(12, 5).unwrap().symbol, '💾');
    assert_eq!(b.get(12, 5).unwrap().fg, Some(Color::CYAN));
    assert_eq!(b.get(14, 5).unwrap().symbol, 'S');
}

#[test]
fn test_command_palette_render_with_shortcuts() {
    let mut palette = CommandPalette::new()
        .command(Command::new("save", "Save File").shortcut("Ctrl+S"))
        .command(Command::new("quit", "Quit"));

    palette.show();
    let b = view::render(&palette, 80, 24);

    // Right-aligned: it ends one blank column before the right border (69).
    let shortcut: String = (62..68).map(|x| b.get(x, 5).unwrap().symbol).collect();
    assert_eq!(shortcut, "Ctrl+S");
    assert_eq!(b.get(68, 5).unwrap().symbol, ' ');
    assert_eq!(b.get(69, 5).unwrap().symbol, '│');
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
    let mut palette = CommandPalette::new()
        .placeholder("Go to...")
        .command(Command::new("test", "Test"));

    palette.show();
    let b = view::render(&palette, 80, 24);

    // After the cursor at column 14 (#1091), in the disabled grey.
    let text: String = (15..22).map(|x| b.get(x, 3).unwrap().symbol).collect();
    assert_eq!(text, "o to...");
    assert!((15..22).all(|x| b.get(x, 3).unwrap().fg == Some(DISABLED_FG)));
}

#[test]
fn test_command_palette_render_custom_colors() {
    let mut palette = CommandPalette::new()
        .colors(Color::RED, Color::GREEN, Color::BLUE)
        .commands(vec![
            Command::new("test", "Test"),
            Command::new("other", "Other"),
            Command::new("last", "Last"),
        ]);

    palette.show();
    palette.select_next();
    let b = view::render(&palette, 80, 24);

    // The selected second row takes the selection color, the first keeps
    // the background.
    assert!((11..69).all(|x| b.get(x, 6).unwrap().bg == Some(Color::BLUE)));
    assert!((11..69).all(|x| b.get(x, 5).unwrap().bg == Some(Color::RED)));
}

#[test]
fn test_command_palette_render_selected_item() {
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("test", "Test Command"),
        Command::new("other", "Other"),
    ]);

    palette.show();
    let b = view::render(&palette, 80, 24);

    let selected = Color::rgb(50, 80, 120);
    assert!(view::row(&b, 5).contains("Test Command"));
    assert!((11..69).all(|x| b.get(x, 5).unwrap().bg == Some(selected)));
    assert_eq!(b.get(14, 5).unwrap().fg, Some(Color::WHITE));
}

#[test]
fn test_command_palette_render_many_commands() {
    let commands: Vec<_> = (0..20)
        .map(|i| Command::new(format!("cmd{}", i), format!("Command {}", i)))
        .collect();

    let mut palette = CommandPalette::new().commands(commands).max_visible(10);

    palette.show();
    let b = view::render(&palette, 80, 24);

    // The first rows in order; no row past the tenth command. The tenth row
    // is under the bottom border (#1091).
    for i in 0..9u16 {
        let text = view::row(&b, 5 + i);
        assert!(text.contains(&format!("Command {i} ")), "{text}");
    }
    for y in 0..24 {
        let text = view::row(&b, y);
        assert!(
            (10..20).all(|i| !text.contains(&format!("Command {i}"))),
            "{text}"
        );
    }
    assert!(view::row(&b, 14).contains("20/20"), "{}", view::row(&b, 14));
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
    let mut palette = CommandPalette::new().commands(vec![
        Command::new("test", "Test"),
        Command::new("other", "Other"),
    ]);

    palette.show();
    let b = view::render(&palette, 40, 10);

    // The frame narrows to the 40-column area.
    assert_eq!(b.get(0, 2).unwrap().symbol, '╭');
    assert_eq!(b.get(39, 2).unwrap().symbol, '╮');
    assert!(
        view::row(&b, 5).contains("│   Test "),
        "{}",
        view::row(&b, 5)
    );
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
    let mut palette = CommandPalette::new()
        .command(Command::new("test", long_label))
        .command(Command::new("other", "Other"));

    palette.show();
    let b = view::render(&palette, 80, 24);

    // The label runs from column 14 up to the label column's end (55).
    let text: String = (14..56).map(|x| b.get(x, 5).unwrap().symbol).collect();
    let want: String = long_label.chars().take(41).collect();
    assert_eq!(text, format!("{want} "));
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
    let mut palette = CommandPalette::new()
        .width(10)
        .commands(vec![Command::new("a", "A"), Command::new("b", "B")]);
    palette.show();
    // At least 30 columns: 20..50 in a 70-column area.
    let b = view::render(&palette, 70, 12);
    assert_eq!(b.get(20, 2).unwrap().symbol, '╭');
    assert_eq!(b.get(49, 2).unwrap().symbol, '╮');
}

#[test]
fn test_command_palette_max_visible_clamping() {
    let commands: Vec<_> = (0..6)
        .map(|i| Command::new(format!("c{i}"), format!("Command {i}")))
        .collect();
    let mut palette = CommandPalette::new().max_visible(1).commands(commands);
    palette.show();
    // At least three rows: the fourth command scrolls by one.
    for _ in 0..3 {
        palette.select_next();
    }
    assert_eq!(palette.selection.offset(), 1);
    let b = view::render(&palette, 70, 12);
    assert!(
        view::row(&b, 5).contains("Command 1 "),
        "{}",
        view::row(&b, 5)
    );
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

// Every width from 0 up: the frame, title, label, shortcut and result count
// positions are all computed from the palette's width and must not underflow
// when it is narrower than they are.
#[test]
fn test_command_palette_render_fits_very_narrow_areas() {
    for w in 0..=24 {
        let mut palette = CommandPalette::new().title("Commands").commands(vec![
            Command::new("save", "Save File").shortcut("Ctrl+Shift+S"),
            Command::new("open", "Open File").shortcut("Ctrl+O"),
        ]);
        palette.show();
        let mut buffer = Buffer::new(w + 4, 14);
        let area = Rect::new(2, 1, w, 12);
        let mut ctx = RenderContext::new(&mut buffer, area);
        palette.render(&mut ctx);
        for (x, y, cell) in buffer.iter_cells() {
            let inside = (2..2 + w).contains(&x) && (1..13).contains(&y);
            assert!(
                inside || cell.symbol == ' ',
                "width {w}: wrote {:?} outside the area at ({x},{y})",
                cell.symbol
            );
        }
    }
}

/// Paint `palette` and return its rows as text.
fn palette_text(palette: &CommandPalette) -> String {
    let mut buffer = Buffer::new(70, 14);
    let mut ctx = RenderContext::new(&mut buffer, Rect::new(0, 0, 70, 14));
    palette.render(&mut ctx);
    (0..14)
        .map(|y| {
            (0..70)
                .map(|x| buffer.get(x, y).map_or(' ', |c| c.symbol))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// #799: `show_descriptions` was stored and the descriptions never drawn.
#[test]
fn test_command_palette_draws_descriptions() {
    let make = |show: bool| {
        let mut p = CommandPalette::new()
            .width(60)
            .command(
                Command::new("save", "Save")
                    .description("Write to disk")
                    .shortcut("Ctrl+S"),
            )
            .command(Command::new("quit", "Quit"))
            .show_descriptions(show);
        p.show();
        p
    };

    let shown = palette_text(&make(true));
    assert!(shown.contains("Write to disk"), "{shown}");
    assert!(
        shown.contains("Ctrl+S"),
        "the shortcut lost its place:\n{shown}"
    );

    let hidden = palette_text(&make(false));
    assert!(!hidden.contains("Write to disk"), "{hidden}");
}

#[test]
fn a_wide_glyph_that_would_cross_the_title_end_is_not_drawn() {
    // 10 wide: text from column 2 to 7, padding at 8, border at 9. "a가나다"
    // fits "a가나" (5 columns); "다" would take 7-8 and is left out (#889)
    let mut palette = CommandPalette::new().width(10).title("a가나다");
    palette.show();
    let mut buffer = Buffer::new(10, 8);
    palette.render(&mut RenderContext::new(&mut buffer, Rect::new(0, 0, 10, 8)));
    let y = (0..8)
        .find(|&y| buffer.get(2, y).unwrap().symbol == 'a')
        .expect("title row");
    assert_eq!(buffer.get(5, y).unwrap().symbol, '나');
    assert_eq!(buffer.get(7, y).unwrap().symbol, ' ');
    assert!(!buffer.get(8, y).unwrap().is_continuation());
}

#[test]
fn test_command_palette_leaves_a_ctrl_or_alt_letter_to_the_app() {
    use revue::event::KeyEvent;
    let mut palette = CommandPalette::new().commands(vec![Command::new("kill", "Kill")]);
    palette.show();
    assert!(!palette.handle_key_event(&KeyEvent::ctrl(Key::Char('k'))));
    assert!(!palette.handle_key_event(&KeyEvent::alt(Key::Char('k'))));
    assert_eq!(palette.get_query(), "");
    assert!(palette.handle_key_event(&KeyEvent::new(Key::Char('k'))));
    assert_eq!(palette.get_query(), "k");
}
