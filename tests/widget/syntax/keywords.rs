//! Tests for syntax keyword detection functions

use revue::widget::syntax::keywords::{
    is_go_keyword, is_javascript_keyword, is_python_keyword, is_rust_keyword, is_shell_keyword,
    is_sql_keyword,
};

// ============================================================
// is_rust_keyword
// ============================================================

#[test]
fn rust_keyword_fn() {
    assert!(is_rust_keyword("fn"));
}

#[test]
fn rust_keyword_let_mut_pub() {
    assert!(is_rust_keyword("let"));
    assert!(is_rust_keyword("mut"));
    assert!(is_rust_keyword("pub"));
}

#[test]
fn rust_keyword_struct_impl_trait() {
    assert!(is_rust_keyword("struct"));
    assert!(is_rust_keyword("impl"));
    assert!(is_rust_keyword("trait"));
}

#[test]
fn rust_keyword_async_match() {
    assert!(is_rust_keyword("async"));
    assert!(is_rust_keyword("match"));
}

#[test]
fn rust_keyword_self_variants() {
    assert!(is_rust_keyword("self"));
    assert!(is_rust_keyword("Self"));
}

#[test]
fn rust_keyword_is_case_sensitive() {
    assert!(!is_rust_keyword("FN"));
    assert!(!is_rust_keyword("Let"));
    assert!(!is_rust_keyword("STRUCT"));
}

#[test]
fn rust_not_keyword() {
    assert!(!is_rust_keyword("println"));
    assert!(!is_rust_keyword("String"));
    assert!(!is_rust_keyword("Vec"));
    assert!(!is_rust_keyword(""));
}

// ============================================================
// is_python_keyword
// ============================================================

#[test]
fn python_keyword_def_class_import() {
    assert!(is_python_keyword("def"));
    assert!(is_python_keyword("class"));
    assert!(is_python_keyword("import"));
}

#[test]
fn python_keyword_if_elif_lambda_yield() {
    assert!(is_python_keyword("if"));
    assert!(is_python_keyword("elif"));
    assert!(is_python_keyword("lambda"));
    assert!(is_python_keyword("yield"));
}

#[test]
fn python_keyword_async_await() {
    assert!(is_python_keyword("async"));
    assert!(is_python_keyword("await"));
}

#[test]
fn python_keyword_is_case_sensitive() {
    assert!(!is_python_keyword("DEF"));
    assert!(!is_python_keyword("Class"));
}

#[test]
fn python_not_keyword() {
    assert!(!is_python_keyword("print"));
    assert!(!is_python_keyword("len"));
    assert!(!is_python_keyword("self"));
    assert!(!is_python_keyword(""));
}

// ============================================================
// is_javascript_keyword
// ============================================================

#[test]
fn javascript_keyword_function_const_let_var() {
    assert!(is_javascript_keyword("function"));
    assert!(is_javascript_keyword("const"));
    assert!(is_javascript_keyword("let"));
    assert!(is_javascript_keyword("var"));
}

#[test]
fn javascript_keyword_class_async_await() {
    assert!(is_javascript_keyword("class"));
    assert!(is_javascript_keyword("async"));
    assert!(is_javascript_keyword("await"));
}

#[test]
fn javascript_keyword_this_typeof() {
    assert!(is_javascript_keyword("this"));
    assert!(is_javascript_keyword("typeof"));
}

#[test]
fn javascript_keyword_is_case_sensitive() {
    assert!(!is_javascript_keyword("CONST"));
    assert!(!is_javascript_keyword("Function"));
}

#[test]
fn javascript_not_keyword() {
    assert!(!is_javascript_keyword("console"));
    assert!(!is_javascript_keyword("undefined"));
    assert!(!is_javascript_keyword("NaN"));
    assert!(!is_javascript_keyword(""));
}

// ============================================================
// is_shell_keyword
// ============================================================

#[test]
fn shell_keyword_if_then_fi() {
    assert!(is_shell_keyword("if"));
    assert!(is_shell_keyword("then"));
    assert!(is_shell_keyword("fi"));
}

#[test]
fn shell_keyword_case_esac() {
    assert!(is_shell_keyword("case"));
    assert!(is_shell_keyword("esac"));
}

#[test]
fn shell_keyword_for_while_do_done() {
    assert!(is_shell_keyword("for"));
    assert!(is_shell_keyword("while"));
    assert!(is_shell_keyword("do"));
    assert!(is_shell_keyword("done"));
}

#[test]
fn shell_keyword_echo_cd_export() {
    assert!(is_shell_keyword("echo"));
    assert!(is_shell_keyword("cd"));
    assert!(is_shell_keyword("export"));
}

#[test]
fn shell_keyword_is_case_sensitive() {
    assert!(!is_shell_keyword("IF"));
    assert!(!is_shell_keyword("Echo"));
}

#[test]
fn shell_not_keyword() {
    assert!(!is_shell_keyword("ls"));
    assert!(!is_shell_keyword("grep"));
    assert!(!is_shell_keyword("awk"));
    assert!(!is_shell_keyword(""));
}

// ============================================================
// is_sql_keyword
// ============================================================

#[test]
fn sql_keyword_select_from_where() {
    assert!(is_sql_keyword("SELECT"));
    assert!(is_sql_keyword("FROM"));
    assert!(is_sql_keyword("WHERE"));
}

#[test]
fn sql_keyword_case_insensitive() {
    assert!(is_sql_keyword("select"));
    assert!(is_sql_keyword("Select"));
    assert!(is_sql_keyword("FROM"));
    assert!(is_sql_keyword("from"));
}

#[test]
fn sql_keyword_join_group_order() {
    assert!(is_sql_keyword("JOIN"));
    assert!(is_sql_keyword("GROUP"));
    assert!(is_sql_keyword("ORDER"));
}

#[test]
fn sql_keyword_insert_update_delete() {
    assert!(is_sql_keyword("INSERT"));
    assert!(is_sql_keyword("UPDATE"));
    assert!(is_sql_keyword("DELETE"));
}

#[test]
fn sql_not_keyword() {
    assert!(!is_sql_keyword("COLUMN_NAME"));
    assert!(!is_sql_keyword("users"));
    assert!(!is_sql_keyword("id"));
    assert!(!is_sql_keyword(""));
}

// ============================================================
// is_go_keyword
// ============================================================

#[test]
fn go_keyword_func_package_import() {
    assert!(is_go_keyword("func"));
    assert!(is_go_keyword("package"));
    assert!(is_go_keyword("import"));
}

#[test]
fn go_keyword_go_chan_defer() {
    assert!(is_go_keyword("go"));
    assert!(is_go_keyword("chan"));
    assert!(is_go_keyword("defer"));
}

#[test]
fn go_keyword_interface_struct_type() {
    assert!(is_go_keyword("interface"));
    assert!(is_go_keyword("struct"));
    assert!(is_go_keyword("type"));
}

#[test]
fn go_keyword_select_range_map() {
    assert!(is_go_keyword("select"));
    assert!(is_go_keyword("range"));
    assert!(is_go_keyword("map"));
}

#[test]
fn go_keyword_is_case_sensitive() {
    assert!(!is_go_keyword("FUNC"));
    assert!(!is_go_keyword("Package"));
}

#[test]
fn go_not_keyword() {
    assert!(!is_go_keyword("fmt"));
    assert!(!is_go_keyword("Println"));
    assert!(!is_go_keyword("main"));
    assert!(!is_go_keyword(""));
}
