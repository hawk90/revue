//! Command structure tests

use revue::widget::Command;

// =========================================================================
// Command::new tests
// =========================================================================

#[test]
fn test_command_new() {
    let cmd = Command::new("cmd_id", "Command Label");
    assert_eq!(cmd.id, "cmd_id");
    assert_eq!(cmd.label, "Command Label");
    assert!(cmd.description.is_none());
    assert!(cmd.shortcut.is_none());
    assert!(cmd.category.is_none());
    assert!(cmd.icon.is_none());
    assert!(!cmd.recent);
    assert!(!cmd.pinned);
}

#[test]
fn test_command_new_with_strings() {
    let id = String::from("test_id");
    let label = String::from("Test Label");
    let cmd = Command::new(id.clone(), label.clone());
    assert_eq!(cmd.id, "test_id");
    assert_eq!(cmd.label, "Test Label");
}

// =========================================================================
// Builder methods tests
// =========================================================================

#[test]
fn test_command_description() {
    let cmd = Command::new("id", "Label").description("A description");
    assert_eq!(cmd.description, Some("A description".to_string()));
}

#[test]
fn test_command_description_string() {
    let cmd = Command::new("id", "Label").description(String::from("Desc"));
    assert_eq!(cmd.description, Some("Desc".to_string()));
}

#[test]
fn test_command_shortcut() {
    let cmd = Command::new("id", "Label").shortcut("Ctrl+P");
    assert_eq!(cmd.shortcut, Some("Ctrl+P".to_string()));
}

#[test]
fn test_command_category() {
    let cmd = Command::new("id", "Label").category("File");
    assert_eq!(cmd.category, Some("File".to_string()));
}

#[test]
fn test_command_icon() {
    let cmd = Command::new("id", "Label").icon('📁');
    assert_eq!(cmd.icon, Some('📁'));
}

#[test]
fn test_command_recent() {
    let cmd = Command::new("id", "Label").recent();
    assert!(cmd.recent);
}

#[test]
fn test_command_pinned() {
    let cmd = Command::new("id", "Label").pinned();
    assert!(cmd.pinned);
}

#[test]
fn test_command_builder_chain() {
    let cmd = Command::new("id", "Label")
        .description("Desc")
        .shortcut("Ctrl+P")
        .category("File")
        .icon('F')
        .recent()
        .pinned();

    assert_eq!(cmd.id, "id");
    assert_eq!(cmd.label, "Label");
    assert_eq!(cmd.description, Some("Desc".to_string()));
    assert_eq!(cmd.shortcut, Some("Ctrl+P".to_string()));
    assert_eq!(cmd.category, Some("File".to_string()));
    assert_eq!(cmd.icon, Some('F'));
    assert!(cmd.recent);
    assert!(cmd.pinned);
}

// =========================================================================
// matches tests
// =========================================================================

#[test]
fn test_command_matches_empty_query() {
    let cmd = Command::new("id", "Save File");
    assert!(cmd.matches(""));
}

#[test]
fn test_command_matches_exact() {
    let cmd = Command::new("id", "Save File");
    assert!(cmd.matches("Save"));
}

#[test]
fn test_command_matches_case_insensitive() {
    let cmd = Command::new("id", "Save File");
    assert!(cmd.matches("save"));
}

#[test]
fn test_command_matches_fuzzy() {
    let cmd = Command::new("id", "Save File");
    assert!(cmd.matches("sf"));
}

#[test]
fn test_command_matches_no_match() {
    let cmd = Command::new("id", "Save File");
    assert!(!cmd.matches("xyz"));
}

#[test]
fn test_command_matches_description() {
    let cmd = Command::new("id", "Save").description("Save the file");
    assert!(cmd.matches("file"));
}

#[test]
fn test_command_matches_category() {
    let cmd = Command::new("id", "Save").category("File Operations");
    assert!(cmd.matches("operations"));
}

// =========================================================================
// fuzzy_match tests
// =========================================================================

#[test]
fn test_fuzzy_match_empty_query() {
    let cmd = Command::new("id", "Save File");
    let result = cmd.fuzzy_match("");
    assert!(result.is_some());
}

#[test]
fn test_fuzzy_match_exact() {
    let cmd = Command::new("id", "Save File");
    let result = cmd.fuzzy_match("Save");
    assert!(result.is_some());
}

#[test]
fn test_fuzzy_match_no_match() {
    let cmd = Command::new("id", "Save File");
    let result = cmd.fuzzy_match("xyz");
    assert!(result.is_none());
}

#[test]
fn test_fuzzy_match_description() {
    let cmd = Command::new("id", "Save").description("Save the file");
    let result = cmd.fuzzy_match("file");
    assert!(result.is_some());
}

#[test]
fn test_fuzzy_match_category() {
    let cmd = Command::new("id", "Save").category("File Operations");
    let result = cmd.fuzzy_match("operations");
    assert!(result.is_some());
}

// =========================================================================
// match_score tests
// =========================================================================

#[test]
fn test_match_score_empty_query_default() {
    let cmd = Command::new("id", "Save File");
    let score = cmd.match_score("");
    assert_eq!(score, 0);
}

#[test]
fn test_match_score_empty_query_pinned() {
    let cmd = Command::new("id", "Save File").pinned();
    let score = cmd.match_score("");
    assert_eq!(score, 100);
}

#[test]
fn test_match_score_empty_query_recent() {
    let cmd = Command::new("id", "Save File").recent();
    let score = cmd.match_score("");
    assert_eq!(score, 50);
}

#[test]
fn test_match_score_empty_query_pinned_and_recent() {
    let cmd = Command::new("id", "Save File").pinned().recent();
    let score = cmd.match_score("");
    // Pinned bonus (100) should take precedence, recent adds 25
    assert!(score >= 100);
}

#[test]
fn test_match_score_with_query() {
    let cmd = Command::new("id", "Save File");
    let score = cmd.match_score("save");
    assert!(score > 0);
}

#[test]
fn test_match_score_pinned_bonus() {
    let cmd1 = Command::new("id1", "Save").pinned();
    let cmd2 = Command::new("id2", "Save");
    let score1 = cmd1.match_score("save");
    let score2 = cmd2.match_score("save");
    assert!(score1 > score2);
}

#[test]
fn test_match_score_recent_bonus() {
    let cmd1 = Command::new("id1", "Save").recent();
    let cmd2 = Command::new("id2", "Save");
    let score1 = cmd1.match_score("save");
    let score2 = cmd2.match_score("save");
    assert!(score1 > score2);
}

// =========================================================================
// Clone tests
// =========================================================================

#[test]
fn test_command_clone() {
    let cmd1 = Command::new("id", "Label").description("Desc").pinned();
    let cmd2 = cmd1.clone();
    assert_eq!(cmd1.id, cmd2.id);
    assert_eq!(cmd1.label, cmd2.label);
    assert_eq!(cmd1.description, cmd2.description);
    assert_eq!(cmd1.pinned, cmd2.pinned);
}