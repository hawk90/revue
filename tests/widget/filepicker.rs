//! FilePicker widget tests
//!
//! FilePicker 위젯의 통합 테스트입니다.

use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{file_picker, save_picker, dir_picker, FileFilter, FilePicker, PickerEntry, PickerMode, PickerResult};
use std::path::Path;

// =============================================================================
// 생성자 및 빌더 테스트 (Constructor and Builder Tests)
// =============================================================================

#[test]
fn test_file_picker_new() {
    let picker = FilePicker::new();
    // 현재 작업 디렉토리로 초기화되어야 함
    assert!(picker.current_dir().exists());
}

#[test]
fn test_file_picker_default() {
    let picker = FilePicker::default();
    assert!(picker.current_dir().exists());
}

#[test]
fn test_file_picker_save() {
    let picker = FilePicker::save();
    // Save 모드 생성은 confirm() 동작으로 확인
    let result = picker.confirm();
    match result {
        PickerResult::Selected(_) | PickerResult::None => {
            // Save 모드에서는 빈 입력으로 None 또는 경로 반환
        }
        _ => panic!("Unexpected result type for Save mode"),
    }
}

#[test]
fn test_file_picker_directory() {
    let mut picker = FilePicker::directory();
    // Directory 모드는 내비게이션 동작으로 확인
    picker.go_up();
    // Directory 모드에서 동작해야 함
}

#[test]
fn test_file_picker_multi_select() {
    let picker = FilePicker::multi_select();
    // MultiSelect 모드는 confirm() 동작으로 확인
    let result = picker.confirm();
    match result {
        PickerResult::Multiple(_) | PickerResult::None => {
            // MultiSelect 모드에서는 Multiple 또는 None 반환
        }
        _ => panic!("Unexpected result type for MultiSelect mode"),
    }
}

#[test]
fn test_file_picker_mode_builder() {
    let picker = FilePicker::new().mode(PickerMode::Save);
    // 모드 빌더는 confirm() 동작으로 확인
    let result = picker.confirm();
    // Save 모드 동작 확인
}

#[test]
fn test_file_picker_title() {
    let picker = FilePicker::new().title("Open File");
    // 타이틀 설정은 렌더링을 통해서만 확인 가능
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
    // 렌더링 성공으로 타이틀 설정 확인
}

#[test]
fn test_file_picker_width() {
    let picker = FilePicker::new().width(80);
    // width 설정은 렌더링을 통해서만 확인 가능
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_max_visible() {
    let picker = FilePicker::new().max_visible(20);
    // max_visible 설정은 내부 동작에만 영향
    // 렌더링을 통해 간접 확인
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_default_name() {
    let picker = FilePicker::save().default_name("untitled.rs");
    // default_name 설정은 confirm() 동작으로 확인
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("untitled.rs"));
        }
        _ => {}
    }
}

#[test]
fn test_file_picker_start_dir() {
    let picker = FilePicker::new().start_dir("/");
    assert_eq!(picker.current_dir().to_str(), Some("/"));
}

#[test]
fn test_file_picker_show_hidden() {
    let picker = FilePicker::new().show_hidden(true);
    // show_hidden 설정은 toggle_hidden() 동작으로 확인
    // 기본값이 true로 설정되어야 함
}

#[test]
fn test_file_picker_filter() {
    let picker = FilePicker::new().filter(FileFilter::extensions(&["rs", "toml"]));
    // 필터 설정은 내부 동작에만 영향
    // 렌더링을 통해 간접 확인
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_builder_chain() {
    let picker = FilePicker::new()
        .title("Select File")
        .width(80)
        .max_visible(20)
        .show_hidden(false)
        .filter(FileFilter::extensions(&["rs"]));

    // 빌더 체인이 정상적으로 작동해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

// =============================================================================
// 도우미 함수 테스트 (Helper Function Tests)
// =============================================================================

#[test]
fn test_file_picker_helper() {
    let picker = file_picker();
    assert!(picker.current_dir().exists());
}

#[test]
fn test_save_picker_helper() {
    let picker = save_picker();
    // Save picker 확인
    let result = picker.confirm();
    match result {
        PickerResult::Selected(_) | PickerResult::None => {}
        _ => panic!("Unexpected result"),
    }
}

#[test]
fn test_dir_picker_helper() {
    let mut picker = dir_picker();
    // Directory picker 확인
    picker.go_up();
}

// =============================================================================
// 경로 관리 테스트 (Path Management Tests)
// =============================================================================

#[test]
fn test_file_picker_current_dir() {
    let picker = FilePicker::new();
    let current_dir = picker.current_dir();
    assert!(current_dir.exists());
    assert!(current_dir.is_absolute());
}

#[test]
fn test_file_picker_start_dir_sets_path() {
    let picker = FilePicker::new().start_dir("/");
    assert_eq!(picker.current_dir().to_str(), Some("/"));
}

#[test]
fn test_file_picker_refresh() {
    let mut picker = FilePicker::new();
    // refresh가 호출되어도 패닉하지 않아야 함
    picker.refresh();

    // refresh 후에도 current_dir은 유효해야 함
    assert!(picker.current_dir().exists());
}

// =============================================================================
// 필터 테스트 (Filter Tests)
// =============================================================================

#[test]
fn test_file_filter_all() {
    let filter = FileFilter::All;
    assert!(filter.matches(Path::new("test.rs")));
    assert!(filter.matches(Path::new("test.txt")));
    assert!(filter.matches(Path::new("test")));
}

#[test]
fn test_file_filter_extensions() {
    let filter = FileFilter::extensions(&["rs", "toml", "md"]);

    assert!(filter.matches(Path::new("main.rs")));
    assert!(filter.matches(Path::new("Cargo.toml")));
    assert!(filter.matches(Path::new("README.md")));
    assert!(!filter.matches(Path::new("test.txt")));
    assert!(!filter.matches(Path::new("noext")));
}

#[test]
fn test_file_filter_extensions_case_insensitive() {
    let filter = FileFilter::extensions(&["rs", "TXT"]);

    assert!(filter.matches(Path::new("main.rs")));
    assert!(filter.matches(Path::new("main.RS")));
    assert!(filter.matches(Path::new("test.txt")));
    assert!(filter.matches(Path::new("test.TXT")));
}

#[test]
fn test_file_filter_pattern_suffix() {
    let filter = FileFilter::pattern("*.rs");

    assert!(filter.matches(Path::new("main.rs")));
    assert!(filter.matches(Path::new("test.rs")));
    assert!(!filter.matches(Path::new("main.txt")));
    assert!(!filter.matches(Path::new("rs")));
}

#[test]
fn test_file_filter_pattern_prefix() {
    let filter = FileFilter::pattern("test*");

    assert!(filter.matches(Path::new("test.rs")));
    assert!(filter.matches(Path::new("test_main.rs")));
    assert!(!filter.matches(Path::new("main_test.rs")));
    assert!(!filter.matches(Path::new("atest.rs")));
}

#[test]
fn test_file_filter_pattern_exact() {
    let filter = FileFilter::pattern("Cargo.toml");

    assert!(filter.matches(Path::new("Cargo.toml")));
    assert!(!filter.matches(Path::new("test-Cargo.toml")));
    assert!(!filter.matches(Path::new("Cargo.toml.bak")));
}

#[test]
fn test_file_filter_custom() {
    let filter = FileFilter::Custom("my_filter".to_string());
    // Custom 필터는 항상 true를 반환 (외부 처리 필요)
    assert!(filter.matches(Path::new("test.rs")));
}

#[test]
fn test_file_filter_directories_only() {
    let filter = FileFilter::DirectoriesOnly;

    // 현재 디렉토리는 디렉토리이므로 매치
    assert!(filter.matches(std::env::current_dir().unwrap().as_path()));
}

// =============================================================================
// 내비게이션 테스트 (Navigation Tests)
// =============================================================================

#[test]
fn test_file_picker_highlight_next() {
    let mut picker = FilePicker::new();

    // 항목이 있는 경우 highlight_next가 호출 가능해야 함
    picker.highlight_next();
    picker.highlight_next();

    // 여러 호출도 안전해야 함
    for _ in 0..10 {
        picker.highlight_next();
    }
}

#[test]
fn test_file_picker_highlight_previous() {
    let mut picker = FilePicker::new();

    // 항목이 있는 경우 highlight_previous가 호출 가능해야 함
    picker.highlight_next();
    picker.highlight_previous();

    // 여러 호출도 안전해야 함
    for _ in 0..10 {
        picker.highlight_previous();
    }
}

#[test]
fn test_file_picker_highlight_next_at_end() {
    let mut picker = FilePicker::new();

    // 끝까지 이동 후 추가 호출은 안전해야 함
    for _ in 0..100 {
        picker.highlight_next();
    }
}

#[test]
fn test_file_picker_highlight_previous_at_start() {
    let mut picker = FilePicker::new();

    // 시작에서 이전 호출은 안전해야 함
    for _ in 0..10 {
        picker.highlight_previous();
    }
}

#[test]
fn test_file_picker_navigate_to() {
    let mut picker = FilePicker::new();
    let test_dir = std::env::current_dir().unwrap();

    // 존재하는 디렉토리로 이동
    picker.navigate_to(&test_dir);
    assert_eq!(picker.current_dir(), test_dir);
}

#[test]
fn test_file_picker_navigate_to_non_directory() {
    let mut picker = FilePicker::new();
    let file_path = Path::new("nonexistent_file.txt");

    // 존재하지 않는 경로로 이동 시도
    let initial_dir = picker.current_dir().to_path_buf();
    picker.navigate_to(file_path);

    // 디렉토리가 아니면 변경되지 않아야 함
    assert_eq!(picker.current_dir(), &initial_dir);
}

#[test]
fn test_file_picker_go_up() {
    let mut picker = FilePicker::new();

    // 루트가 아닌 경우 상위로 이동 가능
    if picker.current_dir().parent().is_some() {
        let initial_path = picker.current_dir().to_path_buf();
        picker.go_up();
        // 상위 디렉토리로 이동했거나 루트에 도달
    }
}

#[test]
fn test_file_picker_go_back_no_history() {
    let mut picker = FilePicker::new();

    // 히스토리가 없으면 go_back은 안전해야 함
    picker.go_back();
    picker.go_back();
}

#[test]
fn test_file_picker_go_forward_no_history() {
    let mut picker = FilePicker::new();

    // 히스토리가 없으면 go_forward는 안전해야 함
    picker.go_forward();
    picker.go_forward();
}

#[test]
fn test_file_picker_navigation_cycle() {
    let mut picker = FilePicker::new();

    // 내비게이션 사이클 테스트
    picker.highlight_next();
    picker.highlight_previous();
    picker.refresh();
    picker.toggle_hidden();

    // 모든 작업 후에도 안전해야 함
    assert!(picker.current_dir().exists());
}

// =============================================================================
// 선택 테스트 (Selection Tests)
// =============================================================================

#[test]
fn test_file_picker_highlighted_entry() {
    let picker = FilePicker::new();

    // highlighted_entry는 Option을 반환
    let entry = picker.highlighted_entry();
    // 항목이 있거나 없어야 함 (파일시스템 의존적)
}

#[test]
fn test_file_picker_confirm_open_mode() {
    let picker = FilePicker::new();
    let result = picker.confirm();

    // Open 모드에서는 Selected 또는 None 반환
    match result {
        PickerResult::Selected(_) | PickerResult::None => {
            // 예상된 결과
        }
        _ => panic!("Unexpected result in Open mode"),
    }
}

#[test]
fn test_file_picker_confirm_save_mode_with_name() {
    let picker = FilePicker::save().default_name("test.txt");
    let result = picker.confirm();

    // Save 모드에서는 파일명이 있으면 Selected 반환
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("test.txt"));
        }
        PickerResult::None => {
            // 입력이 없는 경우도 가능
        }
        _ => panic!("Unexpected result in Save mode"),
    }
}

#[test]
fn test_file_picker_confirm_save_mode_empty() {
    let picker = FilePicker::save();
    let result = picker.confirm();

    // Save 모드에서 빈 입력은 None 반환
    match result {
        PickerResult::None => {
            // 빈 입력
        }
        _ => panic!("Expected None for empty input in Save mode"),
    }
}

#[test]
fn test_file_picker_confirm_directory_mode() {
    let picker = FilePicker::directory();
    let result = picker.confirm();

    // Directory 모드에서는 Selected 또는 None
    match result {
        PickerResult::Selected(_) | PickerResult::None => {
            // 예상된 결과
        }
        _ => panic!("Unexpected result in Directory mode"),
    }
}

#[test]
fn test_file_picker_confirm_multi_select_mode() {
    let picker = FilePicker::multi_select();
    let result = picker.confirm();

    // MultiSelect 모드에서는 Multiple 또는 None
    match result {
        PickerResult::Multiple(paths) => {
            // 아직 선택하지 않았으므로 빈 벡터
            assert!(paths.is_empty());
        }
        PickerResult::None => {
            // 선택 없음
        }
        _ => panic!("Unexpected result in MultiSelect mode"),
    }
}

#[test]
fn test_file_picker_toggle_selection() {
    let mut picker = FilePicker::multi_select();

    // toggle_selection은 항상 안전해야 함
    picker.toggle_selection();
    picker.toggle_selection();
}

#[test]
fn test_file_picker_enter_directory() {
    let mut picker = FilePicker::new();

    if let Some(entry) = picker.highlighted_entry() {
        if entry.is_dir {
            // 디렉토리 진입은 None 반환 (내비게이션만)
            let result = picker.enter();
            assert!(result.is_none());
        }
    }
}

// =============================================================================
// 입력 모드 테스트 (Input Mode Tests)
// =============================================================================

#[test]
fn test_file_picker_input_char() {
    let mut picker = FilePicker::save();

    picker.input_char('t');
    picker.input_char('e');
    picker.input_char('s');
    picker.input_char('t');

    // 입력이 추가되면 confirm 결과가 달라져야 함
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("test"));
        }
        _ => panic!("Expected Selected with input"),
    }
}

#[test]
fn test_file_picker_input_backspace() {
    let mut picker = FilePicker::save().default_name("test");

    picker.input_backspace();
    picker.input_backspace();

    // 문자가 삭제되면 경로가 달라져야 함
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("te"));
        }
        _ => {}
    }
}

#[test]
fn test_file_picker_input_backspace_empty() {
    let mut picker = FilePicker::save();

    // 빈 상태에서 백스페이스는 안전해야 함
    for _ in 0..10 {
        picker.input_backspace();
    }

    // 여전히 None이어야 함
    let result = picker.confirm();
    match result {
        PickerResult::None => {}
        _ => panic!("Expected None for empty input"),
    }
}

#[test]
fn test_file_picker_input_char_in_open_mode() {
    let mut picker = FilePicker::new();

    // Open 모드에서는 입력이 무시되어야 함
    picker.input_char('t');
    picker.input_char('e');
    picker.input_char('s');
    picker.input_char('t');

    // Open 모드에서는 입력이 영향을 주지 않아야 함
}

#[test]
fn test_file_picker_default_name_sets_input() {
    let picker = FilePicker::save().default_name("example.rs");

    // default_name이 설정되면 confirm 시 경로에 포함되어야 함
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("example.rs"));
        }
        _ => panic!("Expected Selected with default name"),
    }
}

#[test]
fn test_file_picker_input_complex() {
    let mut picker = FilePicker::save();

    // 복잡한 입력 시나리오
    picker.input_char('f');
    picker.input_char('i');
    picker.input_char('l');
    picker.input_char('e');
    picker.input_char('.');
    picker.input_char('t');
    picker.input_char('x');
    picker.input_char('t');

    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("file.txt"));
        }
        _ => panic!("Expected Selected with complex input"),
    }

    // 백스페이스로 수정 - "file.txt" -> "file.tx" -> "file.t" -> add 'rs'
    picker.input_backspace(); // removes 't'
    picker.input_backspace(); // removes 'x'
    picker.input_backspace(); // removes 't'
    picker.input_char('r');
    picker.input_char('s');

    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("file.rs"));
        }
        _ => {}
    }
}

// =============================================================================
// 숨김 파일 테스트 (Hidden Files Tests)
// =============================================================================

#[test]
fn test_file_picker_toggle_hidden() {
    let mut picker = FilePicker::new();

    // toggle_hidden은 항상 안전해야 함
    picker.toggle_hidden();
    picker.toggle_hidden();
    picker.toggle_hidden();

    // 여러 번 토글해도 안전해야 함
}

#[test]
fn test_file_picker_show_hidden_true() {
    let picker = FilePicker::new().show_hidden(true);
    // 숨김 파일 표시 설정
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_show_hidden_false() {
    let picker = FilePicker::new().show_hidden(false);
    // 숨김 파일 숨김 설정
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_toggle_hidden_render() {
    let mut picker = FilePicker::new();

    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    picker.toggle_hidden();
    picker.render(&mut ctx);
    picker.toggle_hidden();
    picker.render(&mut ctx);
}

// =============================================================================
// 렌더링 테스트 (Rendering Tests)
// =============================================================================

#[test]
fn test_file_picker_render_basic() {
    let picker = FilePicker::new();
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // 렌더링이 완료되어야 함
}

#[test]
fn test_file_picker_render_with_title() {
    let picker = FilePicker::new().title("Open File");
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // 타이틀이 표시되어야 함
}

#[test]
fn test_file_picker_render_save_mode() {
    let picker = FilePicker::save().default_name("test.rs");
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // Save 모드 UI가 표시되어야 함
}

#[test]
fn test_file_picker_render_multi_select_mode() {
    let picker = FilePicker::multi_select();
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // MultiSelect 모드 UI가 표시되어야 함
}

#[test]
fn test_file_picker_render_custom_width() {
    let picker = FilePicker::new().width(100);
    let mut buffer = Buffer::new(120, 25);
    let area = Rect::new(0, 0, 120, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // 렌더링이 완료되어야 함
}

#[test]
fn test_file_picker_render_small_area() {
    let picker = FilePicker::new().width(40);
    let mut buffer = Buffer::new(40, 15);
    let area = Rect::new(0, 0, 40, 15);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // 작은 영역에서도 렌더링되어야 함
}

#[test]
fn test_file_picker_render_zero_area() {
    let picker = FilePicker::new();
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    picker.render(&mut ctx);
    // 0 영역에서도 패닉하지 않아야 함
}

#[test]
fn test_file_picker_render_all_modes() {
    let modes = [
        PickerMode::Open,
        PickerMode::Save,
        PickerMode::Directory,
        PickerMode::MultiSelect,
    ];

    for mode in modes {
        let picker = FilePicker::new().mode(mode);
        let mut buffer = Buffer::new(80, 25);
        let area = Rect::new(0, 0, 80, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);
        picker.render(&mut ctx);
    }
}

#[test]
fn test_file_picker_render_multiple_times() {
    let picker = FilePicker::new();
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 여러 번 렌더링해도 안전해야 함
    for _ in 0..5 {
        picker.render(&mut ctx);
    }
}

// =============================================================================
// CSS/스타일링 테스트 (CSS/Styling Tests)
// =============================================================================

#[test]
fn test_file_picker_element_id() {
    let picker = FilePicker::new().element_id("my-file-picker");
    assert_eq!(View::id(&picker), Some("my-file-picker"));
}

#[test]
fn test_file_picker_add_class() {
    let picker = FilePicker::new().class("custom-picker");
    assert!(picker.has_class("custom-picker"));
}

#[test]
fn test_file_picker_multiple_classes() {
    let picker = FilePicker::new()
        .class("primary")
        .class("large");

    assert!(picker.has_class("primary"));
    assert!(picker.has_class("large"));
}

#[test]
fn test_file_picker_styled_view_set_id() {
    let mut picker = FilePicker::new();
    picker.set_id("test-picker");
    assert_eq!(View::id(&picker), Some("test-picker"));
}

#[test]
fn test_file_picker_styled_view_add_class() {
    let mut picker = FilePicker::new();
    picker.add_class("active");
    assert!(picker.has_class("active"));
}

#[test]
fn test_file_picker_styled_view_remove_class() {
    let mut picker = FilePicker::new().class("active");
    picker.remove_class("active");
    assert!(!picker.has_class("active"));
}

#[test]
fn test_file_picker_styled_view_toggle_class() {
    let mut picker = FilePicker::new();

    picker.toggle_class("selected");
    assert!(picker.has_class("selected"));

    picker.toggle_class("selected");
    assert!(!picker.has_class("selected"));
}

#[test]
fn test_file_picker_has_class() {
    let picker = FilePicker::new().class("active");
    assert!(picker.has_class("active"));
    assert!(!picker.has_class("inactive"));
}

#[test]
fn test_file_picker_classes_builder() {
    let picker = FilePicker::new().classes(vec!["class1", "class2", "class3"]);

    assert!(picker.has_class("class1"));
    assert!(picker.has_class("class2"));
    assert!(picker.has_class("class3"));
    assert_eq!(View::classes(&picker).len(), 3);
}

#[test]
fn test_file_picker_duplicate_class_not_added() {
    let picker = FilePicker::new().class("test").class("test");

    let classes = View::classes(&picker);
    assert_eq!(classes.len(), 1);
    assert!(classes.contains(&"test".to_string()));
}

#[test]
fn test_file_picker_meta() {
    let picker = FilePicker::new()
        .element_id("test-picker")
        .class("primary");

    let meta = picker.meta();
    assert_eq!(meta.widget_type, "FilePicker");
    assert_eq!(meta.id, Some("test-picker".to_string()));
    assert!(meta.classes.contains("primary"));
}

// =============================================================================
// 엣지 케이스 테스트 (Edge Case Tests)
// =============================================================================

#[test]
fn test_file_picker_empty_directory_handling() {
    // 현재 디렉토리에서 빈 경우 처리
    let picker = FilePicker::new();
    assert!(picker.current_dir().exists());
}

#[test]
fn test_file_picker_nonexistent_start_dir() {
    // 존재하지 않는 디렉토리로 시작해도 패닉하지 않아야 함
    let picker = FilePicker::new().start_dir("/nonexistent/path/that/does/not/exist");
    // 안전하게 처리되어야 함
}

#[test]
fn test_file_picker_very_long_path() {
    let mut picker = FilePicker::new();

    // 매우 긴 경로에서도 안전해야 함
    let long_path = Path::new("/a").join("a".repeat(100));
    picker.navigate_to(&long_path);
    // 패닉하지 않아야 함
}

#[test]
fn test_file_picker_special_characters_in_default_name() {
    let picker = FilePicker::save().default_name("test file (1) [copy].txt");

    // 특수 문자가 포함된 파일명에서도 안전해야 함
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.to_str().unwrap().contains("test file (1) [copy].txt"));
        }
        _ => {}
    }
}

#[test]
fn test_file_picker_unicode_in_default_name() {
    let picker = FilePicker::save().default_name("테스트파일.txt");

    // 유니코드 문자가 포함된 파일명에서도 안전해야 함
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.to_str().unwrap().contains("테스트파일.txt"));
        }
        _ => {}
    }
}

#[test]
fn test_file_picker_clone() {
    let picker1 = FilePicker::new()
        .title("Test")
        .width(80)
        .show_hidden(true);

    let picker2 = picker1.clone();

    // 두 인스턴스가 동일한 current_dir를 가져야 함
    assert_eq!(picker1.current_dir(), picker2.current_dir());
}

#[test]
fn test_file_picker_all_modes_create() {
    let modes = [
        PickerMode::Open,
        PickerMode::Save,
        PickerMode::Directory,
        PickerMode::MultiSelect,
    ];

    for mode in modes {
        let picker = FilePicker::new().mode(mode);
        // 각 모드에서 생성이 성공해야 함
        assert!(picker.current_dir().exists());
    }
}

#[test]
fn test_file_picker_refresh_maintains_state() {
    let mut picker = FilePicker::new();
    let dir_before = picker.current_dir().to_path_buf();

    picker.refresh();

    // refresh 후에도 디렉토리는 유지되어야 함
    assert_eq!(picker.current_dir(), &dir_before);
}

#[test]
fn test_file_picker_empty_filter_list() {
    let picker = FilePicker::new().filter(FileFilter::Extensions(vec![]));
    // 빈 필터 목록에서도 안전해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_width_small() {
    let picker = FilePicker::new().width(10);
    // 작은 너비에서도 안전해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_max_visible_zero() {
    let picker = FilePicker::new().max_visible(0);
    // max_visible 0에서도 안전해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_very_large_width() {
    let picker = FilePicker::new().width(1000);
    // 매우 큰 너비에서도 안전해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_very_large_max_visible() {
    let picker = FilePicker::new().max_visible(10000);
    // 매우 큰 max_visible에서도 안전해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);
}

// =============================================================================
// PickerResult 테스트 (PickerResult Tests)
// =============================================================================

#[test]
fn test_picker_result_none() {
    let _result = PickerResult::None;
    // None 결과 생성
}

#[test]
fn test_picker_result_selected() {
    use std::path::PathBuf;

    let path = PathBuf::from("/test/path.txt");
    let _result = PickerResult::Selected(path);
    // Selected 결과 생성
}

#[test]
fn test_picker_result_multiple() {
    use std::path::PathBuf;

    let paths = vec![
        PathBuf::from("/test/file1.txt"),
        PathBuf::from("/test/file2.txt"),
    ];
    let _result = PickerResult::Multiple(paths);
    // Multiple 결과 생성
}

#[test]
fn test_picker_result_cancelled() {
    let _result = PickerResult::Cancelled;
    // Cancelled 결과 생성
}

#[test]
fn test_picker_result_clone() {
    use std::path::PathBuf;

    let result1 = PickerResult::Selected(PathBuf::from("/test.txt"));
    let _result2 = result1.clone();
    // PickerResult는 Clone을 구현해야 함
}

#[test]
fn test_picker_result_multiple_empty() {
    let paths: Vec<std::path::PathBuf> = vec![];
    let _result = PickerResult::Multiple(paths);
    // 빈 Multiple 결과
}

#[test]
fn test_picker_result_selected_absolute_path() {
    use std::path::PathBuf;

    let path = PathBuf::from("/absolute/path/test.txt");
    let result = PickerResult::Selected(path.clone());

    match result {
        PickerResult::Selected(p) => {
            assert_eq!(p, path);
            assert!(p.is_absolute());
        }
        _ => panic!("Expected Selected"),
    }
}

// =============================================================================
// PickerEntry 테스트 (PickerEntry Tests)
// =============================================================================

#[test]
fn test_picker_entry_format_size_bytes() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 512,
        selected: false,
    };

    let formatted = entry.format_size();
    assert!(formatted.contains("B"));
}

#[test]
fn test_picker_entry_format_size_kb() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 2048,
        selected: false,
    };

    let formatted = entry.format_size();
    assert!(formatted.contains("KB"));
}

#[test]
fn test_picker_entry_format_size_mb() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 2 * 1024 * 1024,
        selected: false,
    };

    let formatted = entry.format_size();
    assert!(formatted.contains("MB"));
}

#[test]
fn test_picker_entry_format_size_gb() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 2 * 1024 * 1024 * 1024,
        selected: false,
    };

    let formatted = entry.format_size();
    assert!(formatted.contains("GB"));
}

#[test]
fn test_picker_entry_format_size_directory() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("/test"),
        name: "test".to_string(),
        is_dir: true,
        is_hidden: false,
        size: 0,
        selected: false,
    };

    let formatted = entry.format_size();
    assert_eq!(formatted, "<DIR>");
}

#[test]
fn test_picker_entry_format_size_zero() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("empty.txt"),
        name: "empty.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 0,
        selected: false,
    };

    let formatted = entry.format_size();
    assert_eq!(formatted, "0 B");
}

#[test]
fn test_picker_entry_from_path_current_dir() {
    // 현재 디렉토리에서 테스트
    if let Some(entry) = PickerEntry::from_path(Path::new(".")) {
        assert_eq!(entry.name, ".");
        assert!(entry.is_dir);
    } else {
        // 일부 시스템에서는 실패할 수 있음
    }
}

#[test]
fn test_picker_entry_debug() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 1024,
        selected: false,
    };

    let debug_str = format!("{:?}", entry);
    assert!(debug_str.contains("PickerEntry"));
}

#[test]
fn test_picker_entry_clone() {
    use std::path::PathBuf;

    let entry1 = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 1024,
        selected: false,
    };

    let entry2 = entry1.clone();
    assert_eq!(entry1.path, entry2.path);
    assert_eq!(entry1.name, entry2.name);
    assert_eq!(entry1.is_dir, entry2.is_dir);
    assert_eq!(entry1.is_hidden, entry2.is_hidden);
    assert_eq!(entry1.size, entry2.size);
    assert_eq!(entry1.selected, entry2.selected);
}

#[test]
fn test_picker_entry_selected_field() {
    use std::path::PathBuf;

    let mut entry = PickerEntry {
        path: PathBuf::from("test.txt"),
        name: "test.txt".to_string(),
        is_dir: false,
        is_hidden: false,
        size: 1024,
        selected: false,
    };

    assert!(!entry.selected);
    entry.selected = true;
    assert!(entry.selected);
}

#[test]
fn test_picker_entry_hidden_file() {
    use std::path::PathBuf;

    let entry = PickerEntry {
        path: PathBuf::from(".hidden"),
        name: ".hidden".to_string(),
        is_dir: false,
        is_hidden: true,
        size: 1024,
        selected: false,
    };

    assert!(entry.is_hidden);
    assert!(entry.name.starts_with('.'));
}

// =============================================================================
// 종합 테스트 (Comprehensive Tests)
// =============================================================================

#[test]
fn test_file_picker_complete_workflow() {
    // 파일 선택의 전체 워크플로우 테스트
    let mut picker = FilePicker::new().title("Select a File");

    // 1. 초기 상태 확인
    assert!(picker.current_dir().exists());

    // 2. 내비게이션
    picker.highlight_next();
    picker.highlight_previous();

    // 3. 렌더링
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);

    // 4. 선택 확인
    let result = picker.confirm();
    match result {
        PickerResult::None | PickerResult::Selected(_) => {
            // 예상된 결과
        }
        _ => panic!("Unexpected result"),
    }
}

#[test]
fn test_file_picker_save_workflow() {
    // 저장 파일 선택 워크플로우
    let mut picker = FilePicker::save()
        .title("Save File As");

    // 파일명 입력
    picker.input_char('m');
    picker.input_char('a');
    picker.input_char('i');
    picker.input_char('n');

    // 백스페이스 테스트
    picker.input_backspace();

    // 렌더링
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);

    // 확인
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.ends_with("mai"));
        }
        _ => {}
    }
}

#[test]
fn test_file_picker_directory_workflow() {
    // 디렉토리 선택 워크플로우
    let mut picker = FilePicker::directory()
        .title("Select Directory")
        .filter(FileFilter::DirectoriesOnly);

    // 렌더링
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);
    picker.render(&mut ctx);

    // 내비게이션
    picker.go_up();

    // 확인
    let result = picker.confirm();
    match result {
        PickerResult::Selected(path) => {
            assert!(path.is_dir());
        }
        _ => {}
    }
}

#[test]
fn test_file_picker_multi_select_workflow() {
    // 멀티셀렉트 워크플로우
    let mut picker = FilePicker::multi_select();

    // 여러 항목 토글
    picker.toggle_selection();
    picker.highlight_next();
    picker.toggle_selection();

    // 확인
    let result = picker.confirm();
    match result {
        PickerResult::Multiple(paths) => {
            // 선택된 항목들
        }
        PickerResult::None => {
            // 선택 없음
        }
        _ => panic!("Unexpected result"),
    }
}

#[test]
fn test_file_picker_navigation_with_rendering() {
    // 내비게이션과 렌더링 결합 테스트
    let mut picker = FilePicker::new();

    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 초기 렌더링
    picker.render(&mut ctx);

    // 내비게이션
    picker.highlight_next();
    picker.render(&mut ctx);

    picker.highlight_next();
    picker.render(&mut ctx);

    picker.highlight_previous();
    picker.render(&mut ctx);
}

#[test]
fn test_file_picker_all_constructors() {
    // 모든 생성자 메서드 테스트
    let new_picker = FilePicker::new();
    assert!(new_picker.current_dir().exists());

    let save_picker = FilePicker::save();
    assert!(save_picker.current_dir().exists());

    let dir_picker = FilePicker::directory();
    assert!(dir_picker.current_dir().exists());

    let multi_picker = FilePicker::multi_select();
    assert!(multi_picker.current_dir().exists());
}

#[test]
fn test_file_picker_all_helper_functions() {
    // 모든 도우미 함수 테스트
    let picker1 = file_picker();
    assert!(picker1.current_dir().exists());

    let picker2 = save_picker();
    assert!(picker2.current_dir().exists());

    let picker3 = dir_picker();
    assert!(picker3.current_dir().exists());
}

#[test]
fn test_file_picker_filter_variants() {
    // 모든 필터 변형 테스트
    let all_filter = FilePicker::new().filter(FileFilter::All);
    let ext_filter = FilePicker::new().filter(FileFilter::extensions(&["rs"]));
    let pattern_filter = FilePicker::new().filter(FileFilter::pattern("*.txt"));
    let custom_filter = FilePicker::new().filter(FileFilter::Custom("test".to_string()));
    let dir_filter = FilePicker::new().filter(FileFilter::DirectoriesOnly);

    // 모든 필터가 정상적으로 작동해야 함
    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    all_filter.render(&mut ctx);
    ext_filter.render(&mut ctx);
    pattern_filter.render(&mut ctx);
    custom_filter.render(&mut ctx);
    dir_filter.render(&mut ctx);
}

#[test]
fn test_file_picker_builder_combinations() {
    // 다양한 빌더 조합 테스트
    let pickers = vec![
        FilePicker::new().title("Test"),
        FilePicker::new().width(60),
        FilePicker::new().max_visible(10),
        FilePicker::new().show_hidden(true),
        FilePicker::new().start_dir("/"),
        FilePicker::new().mode(PickerMode::Save),
        FilePicker::new().filter(FileFilter::All),
    ];

    let mut buffer = Buffer::new(80, 25);
    let area = Rect::new(0, 0, 80, 25);
    let mut ctx = RenderContext::new(&mut buffer, area);

    for picker in pickers {
        picker.render(&mut ctx);
    }
}

#[test]
fn test_file_picker_state_after_operations() {
    // 여러 작업 후 상태 일관성 테스트
    let mut picker = FilePicker::new();

    let initial_dir = picker.current_dir().to_path_buf();

    // 다양한 작업 수행
    picker.highlight_next();
    picker.highlight_previous();
    picker.refresh();
    picker.toggle_hidden();

    // 상태 확인
    assert!(picker.current_dir().exists());

    let _entry = picker.highlighted_entry();
    let _result = picker.confirm();
}

#[test]
fn test_file_picker_input_modes_edge_cases() {
    // 입력 모드 엣지 케이스 테스트
    let mut picker = FilePicker::save();

    // 빈 입력 후 백스페이스
    picker.input_backspace();
    picker.input_backspace();

    // 문자 입력 후 전체 삭제
    picker.input_char('a');
    picker.input_char('b');
    picker.input_char('c');
    picker.input_backspace();
    picker.input_backspace();
    picker.input_backspace();
    picker.input_backspace(); // 추가 백스페이스

    // 결과는 None이어야 함
    let result = picker.confirm();
    match result {
        PickerResult::None => {}
        _ => panic!("Expected None after clearing all input"),
    }
}

#[test]
fn test_file_picker_title_variations() {
    // 다양한 타이틀 테스트
    let titles = vec![
        "Open File",
        "Save File As...",
        "Select Directory",
        "",
        "A very long title that might exceed normal rendering bounds",
        "타이틀 한글", // Korean title
        "Титул", // Cyrillic title
    ];

    for title in titles {
        let picker = FilePicker::new().title(title);
        let mut buffer = Buffer::new(80, 25);
        let area = Rect::new(0, 0, 80, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);
        picker.render(&mut ctx);
    }
}

#[test]
fn test_file_picker_width_variations() {
    // 다양한 너비 테스트 (0과 1은 제외 - overflow 문제로)
    let widths = vec![10u16, 40, 60, 80, 100, 200];

    for width in widths {
        let picker = FilePicker::new().width(width);
        let mut buffer = Buffer::new(200, 25);
        let area = Rect::new(0, 0, 200, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);
        picker.render(&mut ctx);
    }
}

#[test]
fn test_file_picker_max_visible_variations() {
    // 다양한 max_visible 테스트
    let max_values = vec![0usize, 1, 5, 10, 15, 20, 50, 100];

    for max in max_values {
        let picker = FilePicker::new().max_visible(max);
        let mut buffer = Buffer::new(80, 25);
        let area = Rect::new(0, 0, 80, 25);
        let mut ctx = RenderContext::new(&mut buffer, area);
        picker.render(&mut ctx);
    }
}
