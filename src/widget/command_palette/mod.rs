pub mod command;
pub mod core;
pub mod default;
pub mod helper;
pub mod impls;
pub mod styled;
#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod tests {
        use crate::layout::Rect;
        use crate::render::Buffer;
        use crate::widget::command_palette::{command_palette, Command, CommandPalette};
        use crate::widget::RenderContext;

        #[test]
        fn test_command_new() {
            let cmd = Command::new("test", "Test Command");
            assert_eq!(cmd.id, "test");
            assert_eq!(cmd.label, "Test Command");
        }

        #[test]
        fn test_command_builder() {
            let cmd = Command::new("save", "Save File")
                .description("Save the current file")
                .shortcut("Ctrl+S")
                .category("File")
                .icon('💾')
                .recent()
                .pinned();

            assert_eq!(cmd.description, Some("Save the current file".to_string()));
            assert_eq!(cmd.shortcut, Some("Ctrl+S".to_string()));
            assert!(cmd.recent);
            assert!(cmd.pinned);
        }

        #[test]
        fn test_command_matches() {
            let cmd = Command::new("save_file", "Save File").description("Save to disk");

            assert!(cmd.matches("save"));
            assert!(cmd.matches("file"));
            assert!(cmd.matches("sf")); // fuzzy: S_ave F_ile
            assert!(cmd.matches("svfl")); // fuzzy: S_a_V_e F_i_L_e
            assert!(cmd.matches("disk")); // description
            assert!(!cmd.matches("xyz"));
        }

        #[test]
        fn test_command_score() {
            let cmd = Command::new("save", "Save").pinned();

            assert!(cmd.match_score("save") > cmd.match_score("sav"));
            assert!(cmd.match_score("") > 0); // Pinned bonus
        }

        #[test]
        fn test_palette_new() {
            let p = CommandPalette::new();
            assert!(!p.is_visible());
            assert!(p.commands.is_empty());
        }

        #[test]
        fn test_palette_show_hide() {
            let mut p = CommandPalette::new();

            p.show();
            assert!(p.is_visible());

            p.hide();
            assert!(!p.is_visible());

            p.toggle();
            assert!(p.is_visible());
        }

        #[test]
        fn test_palette_add_commands() {
            let p = CommandPalette::new()
                .command(Command::new("a", "Alpha"))
                .command(Command::new("b", "Beta"));

            assert_eq!(p.commands.len(), 2);
        }

        #[test]
        fn test_palette_filter() {
            let mut p = CommandPalette::new()
                .command(Command::new("save", "Save File"))
                .command(Command::new("open", "Open File"))
                .command(Command::new("close", "Close File"));

            p.show();
            assert_eq!(p.filtered.len(), 3);

            p.set_query("save");
            assert_eq!(p.filtered.len(), 1);

            p.clear_query();
            assert_eq!(p.filtered.len(), 3);
        }

        #[test]
        fn test_palette_selection() {
            let mut p = CommandPalette::new()
                .command(Command::new("a", "A"))
                .command(Command::new("b", "B"))
                .command(Command::new("c", "C"));

            p.show();

            assert_eq!(p.selection.index, 0);

            p.select_next();
            assert_eq!(p.selection.index, 1);

            p.select_next();
            assert_eq!(p.selection.index, 2);

            p.select_next();
            assert_eq!(p.selection.index, 0); // Wrap

            p.select_prev();
            assert_eq!(p.selection.index, 2); // Wrap back
        }

        #[test]
        fn test_palette_execute() {
            let mut p = CommandPalette::new().command(Command::new("test", "Test"));

            p.show();
            let result = p.execute();

            assert_eq!(result, Some("test".to_string()));
            assert!(!p.is_visible());
        }

        #[test]
        fn test_palette_input() {
            let mut p = CommandPalette::new();
            p.show();

            p.input('t');
            p.input('e');
            p.input('s');
            p.input('t');

            assert_eq!(p.query, "test");

            p.backspace();
            assert_eq!(p.query, "tes");
        }

        #[test]
        fn test_palette_helper() {
            let p = command_palette().width(50);
            assert_eq!(p.width, 50);
        }

        #[test]
        fn test_highlight_match() {
            let mut p = CommandPalette::new();
            p.query = "".to_string();
            let result = p.highlight_match("Hello");
            assert_eq!(result.len(), 5);
            assert!(result.iter().all(|(_, m)| !m));

            let mut p = CommandPalette::new();
            p.query = "ell".to_string();
            let result = p.highlight_match("Hello");
            assert!(result[1].1); // 'e' matched
            assert!(result[2].1); // 'l' matched
            assert!(result[3].1); // 'l' matched
        }

        #[test]
        fn test_palette_selection_utility() {
            // Test that Selection utility is properly integrated
            let mut p = CommandPalette::new()
                .command(Command::new("cmd1", "Command 1"))
                .command(Command::new("cmd2", "Command 2"))
                .command(Command::new("cmd3", "Command 3"));

            p.show();

            // Initial selection should be 0
            assert_eq!(p.selection.index, 0);

            // Navigate next
            p.select_next();
            assert_eq!(p.selection.index, 1);

            // Navigate prev
            p.select_prev();
            assert_eq!(p.selection.index, 0);

            // Wrap around forward
            p.select_next();
            p.select_next();
            p.select_next();
            assert_eq!(p.selection.index, 0);

            // Wrap around backward
            p.select_prev();
            assert_eq!(p.selection.index, 2);
        }

        #[test]
        fn test_palette_max_visible() {
            let p = CommandPalette::new().max_visible(5);
            assert_eq!(p.max_visible, 5);
        }
    }
}
pub mod view;

pub use command::*;
pub use core::CommandPalette;
pub use helper::command_palette;
