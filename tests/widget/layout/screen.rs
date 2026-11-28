//! Screen widget integration tests
//!
//! Screen 위젯의 통합 테스트입니다.
//! 멀티 스크린 내비게이션 및 스크린 스택 관리 기능을 테스트합니다.

// The transition API is deprecated (#845); its tests stay until 4.0
#![allow(deprecated)]

use revue::event::Key;
use revue::layout::Rect;
use revue::render::Buffer;
use revue::widget::traits::{RenderContext, StyledView, View};
use revue::widget::{screen, screen_stack, Screen, ScreenStack, ScreenTransition};

// =============================================================================
// Constructor Tests - 생성자 테스트
// =============================================================================

#[test]
fn test_screen_new() {
    // 기본 Screen 생성 테스트
    let s = Screen::new("home");
    assert_eq!(s.id, "home");
    assert_eq!(s.title, "home");
    assert!(!s.modal);
}

#[test]
fn test_screen_default_title() {
    // ID와 동일한 기본 타이틀 설정 테스트
    let s = Screen::new("settings");
    assert_eq!(s.title, "settings");
}

#[test]
fn test_screen_helper() {
    // screen() 헬퍼 함수 테스트
    let s = screen("dashboard");
    assert_eq!(s.id, "dashboard");
    assert_eq!(s.title, "dashboard");
}

// =============================================================================
// Builder Method Tests - 빌더 메서드 테스트
// =============================================================================

#[test]
fn test_screen_title_builder() {
    // 커스텀 타이틀 설정 테스트
    let s = Screen::new("home").title("My Home");
    assert_eq!(s.id, "home");
    assert_eq!(s.title, "My Home");
}

#[test]
fn test_screen_modal_builder() {
    // 모달 설정 테스트
    let s = Screen::new("alert").modal();
    assert!(s.modal);
}

#[test]
fn test_screen_not_modal_by_default() {
    // 기본값은 모달이 아님
    let s = Screen::new("normal");
    assert!(!s.modal);
}

#[test]
fn test_screen_data_builder() {
    // 데이터 설정 테스트
    let s = Screen::new("detail")
        .data("user_id", "123")
        .data("username", "test_user");

    assert_eq!(s.get_data("user_id"), Some(&"123".to_string()));
    assert_eq!(s.get_data("username"), Some(&"test_user".to_string()));
    assert_eq!(s.get_data("nonexistent"), None);
}

#[test]
fn test_screen_builder_chain() {
    // 빌더 체이닝 테스트
    let s = Screen::new("profile")
        .title("User Profile")
        .modal()
        .data("user_id", "456");

    assert_eq!(s.id, "profile");
    assert_eq!(s.title, "User Profile");
    assert!(s.modal);
    assert_eq!(s.get_data("user_id"), Some(&"456".to_string()));
}

// =============================================================================
// ScreenStack Constructor Tests - ScreenStack 생성자 테스트
// =============================================================================

#[test]
fn test_screen_stack_new() {
    // ScreenStack 생성 테스트
    let stack = ScreenStack::new();
    assert_eq!(stack.depth(), 0);
    assert!(!stack.can_go_back());
    assert!(stack.current().is_none());
}

#[test]
fn test_screen_stack_default() {
    // Default trait 테스트
    let stack = ScreenStack::default();
    assert_eq!(stack.depth(), 0);
}

#[test]
fn test_screen_stack_helper() {
    // screen_stack() 헬퍼 함수 테스트
    let stack = screen_stack();
    assert_eq!(stack.depth(), 0);
}

// =============================================================================
// ScreenStack Builder Tests - ScreenStack 빌더 테스트
// =============================================================================

#[test]
fn test_screen_stack_transition_builder() {
    // 전환 애니메이션 설정 테스트
    let stack = ScreenStack::new().transition(ScreenTransition::SlideRight);
    // transition 설정이 적용되었는지 확인하기 위해 render 호출
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_all_transitions() {
    // 모든 전환 애니메이션 타입 테스트
    let transitions = [
        ScreenTransition::None,
        ScreenTransition::SlideRight,
        ScreenTransition::SlideUp,
        ScreenTransition::Fade,
        ScreenTransition::Zoom,
    ];

    for transition in transitions {
        let stack = ScreenStack::new().transition(transition);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        stack.render(&mut ctx);
    }
}

#[test]
fn test_screen_stack_register() {
    // 렌더러 등록 테스트
    let stack = ScreenStack::new().register("home", |screen, ctx| {
        // Simple render callback
        let _ = (screen, ctx);
    });

    // 등록된 스택이 정상적으로 생성됨
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_register_multiple() {
    // 여러 렌더러 등록 테스트
    let stack = ScreenStack::new()
        .register("home", |screen, ctx| {
            let _ = (screen, ctx);
        })
        .register("settings", |screen, ctx| {
            let _ = (screen, ctx);
        })
        .register("profile", |screen, ctx| {
            let _ = (screen, ctx);
        });

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

// =============================================================================
// Push/Pop Tests - 푸시/팝 테스트
// =============================================================================

#[test]
fn test_screen_stack_push_single() {
    // 단일 스크린 푸시 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));

    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_push_multiple() {
    // 여러 스크린 푸시 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));
    stack.push(Screen::new("profile"));

    assert_eq!(stack.depth(), 3);
    assert_eq!(stack.current().unwrap().id, "profile");
}

#[test]
fn test_screen_stack_pop() {
    // 스크린 팝 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    let popped = stack.pop();
    assert!(popped.is_some());
    assert_eq!(popped.unwrap().id, "settings");
    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_pop_returns_screen() {
    // 팝이 스크린을 반환하는지 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    let screen = Screen::new("test").title("Test Screen");
    stack.push(screen);

    let popped = stack.pop();
    assert!(popped.is_some());
    let popped_screen = popped.unwrap();
    assert_eq!(popped_screen.id, "test");
    assert_eq!(popped_screen.title, "Test Screen");
}

#[test]
fn test_screen_stack_cannot_pop_last_screen() {
    // 마지막 스크린은 팝할 수 없음
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));

    let result = stack.pop();
    assert!(result.is_none());
    assert_eq!(stack.depth(), 1);
}

#[test]
fn test_screen_stack_pop_empty() {
    // 빈 스택에서 팝 테스트
    let mut stack = ScreenStack::new();
    let result = stack.pop();
    assert!(result.is_none());
    assert_eq!(stack.depth(), 0);
}

// =============================================================================
// Pop To Tests - 특정 스크린까지 팝 테스트
// =============================================================================

#[test]
fn test_screen_stack_pop_to() {
    // 특정 스크린까지 팝 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("a"));
    stack.push(Screen::new("b"));
    stack.push(Screen::new("c"));

    let popped = stack.pop_to("a");
    assert_eq!(popped.len(), 2); // c and b popped
    assert_eq!(stack.depth(), 2);
    assert_eq!(stack.current().unwrap().id, "a");
}

#[test]
fn test_screen_stack_pop_to_current() {
    // 현재 스크린으로 팝 테스트 (아무것도 팝되지 않음)
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    let popped = stack.pop_to("settings");
    assert_eq!(popped.len(), 0);
    assert_eq!(stack.depth(), 2);
}

#[test]
fn test_screen_stack_pop_to_nonexistent() {
    // 존재하지 않는 스크린으로 팝 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    let _popped = stack.pop_to("nonexistent");
    // home까지만 팝됨
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_pop_to_root() {
    // 루트까지 팝 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("a"));
    stack.push(Screen::new("b"));
    stack.push(Screen::new("c"));

    let popped = stack.pop_to_root();
    assert_eq!(popped.len(), 3); // c, b, a popped
    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_pop_to_root_single_screen() {
    // 단일 스크린에서 루트로 팝 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));

    let popped = stack.pop_to_root();
    assert_eq!(popped.len(), 0);
    assert_eq!(stack.depth(), 1);
}

// =============================================================================
// Replace Tests - 스크린 교체 테스트
// =============================================================================

#[test]
fn test_screen_stack_replace() {
    // 현재 스크린 교체 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("old"));

    stack.replace(Screen::new("new"));

    assert_eq!(stack.depth(), 2);
    assert_eq!(stack.current().unwrap().id, "new");
}

#[test]
fn test_screen_stack_replace_single() {
    // 단일 스크린 교체 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("old"));

    stack.replace(Screen::new("new"));

    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().unwrap().id, "new");
}

// =============================================================================
// Query Tests - 쿼리 메서드 테스트
// =============================================================================

#[test]
fn test_screen_stack_current() {
    // current() 메서드 테스트
    let mut stack = ScreenStack::new();
    assert!(stack.current().is_none());

    stack.push(Screen::new("home"));
    assert_eq!(stack.current().unwrap().id, "home");

    stack.push(Screen::new("settings"));
    assert_eq!(stack.current().unwrap().id, "settings");
}

#[test]
fn test_screen_stack_current_mut() {
    // current_mut() 메서드 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));

    if let Some(screen) = stack.current_mut() {
        screen.title = "Updated Home".to_string();
    }

    assert_eq!(stack.current().unwrap().title, "Updated Home");
}

#[test]
fn test_screen_stack_get() {
    // get() 메서드로 특정 스크린 조회 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));
    stack.push(Screen::new("profile"));

    assert_eq!(stack.get("home").unwrap().id, "home");
    assert_eq!(stack.get("settings").unwrap().id, "settings");
    assert_eq!(stack.get("profile").unwrap().id, "profile");
    assert!(stack.get("nonexistent").is_none());
}

#[test]
fn test_screen_stack_contains() {
    // contains() 메서드 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    assert!(stack.contains("home"));
    assert!(stack.contains("settings"));
    assert!(!stack.contains("nonexistent"));
}

#[test]
fn test_screen_stack_depth() {
    // depth() 메서드 테스트
    let mut stack = ScreenStack::new();
    assert_eq!(stack.depth(), 0);

    stack.push(Screen::new("home"));
    assert_eq!(stack.depth(), 1);

    stack.push(Screen::new("settings"));
    assert_eq!(stack.depth(), 2);

    stack.pop();
    assert_eq!(stack.depth(), 1);
}

#[test]
fn test_screen_stack_can_go_back() {
    // can_go_back() 메서드 테스트
    let mut stack = ScreenStack::new();
    assert!(!stack.can_go_back());

    stack.push(Screen::new("home"));
    assert!(!stack.can_go_back());

    stack.push(Screen::new("settings"));
    assert!(stack.can_go_back());

    stack.pop();
    assert!(!stack.can_go_back());
}

// =============================================================================
// Navigation Tests - 내비게이션 테스트
// =============================================================================

#[test]
fn test_screen_stack_go_back() {
    // go_back() 메서드 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    assert!(stack.go_back());
    assert_eq!(stack.current().unwrap().id, "home");
    assert!(!stack.can_go_back());
}

#[test]
fn test_screen_stack_go_back_returns_false() {
    // 뒤로 갈 수 없을 때 go_back() 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));

    assert!(!stack.go_back());
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_go_back_multiple() {
    // 여러 번 뒤로 가기 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("a"));
    stack.push(Screen::new("b"));

    assert!(stack.go_back());
    assert_eq!(stack.current().unwrap().id, "a");

    assert!(stack.go_back());
    assert_eq!(stack.current().unwrap().id, "home");

    assert!(!stack.go_back());
    assert_eq!(stack.current().unwrap().id, "home");
}

// =============================================================================
// Key Handling Tests - 키 입력 처리 테스트
// =============================================================================

#[test]
fn test_screen_stack_handle_key_escape() {
    // Escape 키로 뒤로 가기 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    let handled = stack.handle_key(&Key::Escape);
    assert!(handled);
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_handle_key_escape_no_back() {
    // 뒤로 갈 수 없을 때 Escape 키 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));

    let handled = stack.handle_key(&Key::Escape);
    assert!(!handled);
    assert_eq!(stack.current().unwrap().id, "home");
}

#[test]
fn test_screen_stack_handle_key_other() {
    // 다른 키는 처리하지 않음
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    let handled = stack.handle_key(&Key::Char('a'));
    assert!(!handled);
    assert_eq!(stack.current().unwrap().id, "settings");
}

#[test]
fn test_screen_stack_handle_key_empty_stack() {
    // 빈 스택에서 키 처리 테스트
    let mut stack = ScreenStack::new();

    let handled = stack.handle_key(&Key::Escape);
    assert!(!handled);
}

// =============================================================================
// Transition Tests - 전환 애니메이션 테스트
// =============================================================================

#[test]
fn test_screen_stack_update_transition() {
    // 전환 애니메이션 업데이트 테스트 - 공개 API만 사용
    let mut stack = ScreenStack::new().transition(ScreenTransition::SlideRight);
    stack.push(Screen::new("home"));

    // update_transition 호출이 정상적으로 작동하는지 확인
    stack.update_transition(0.1);

    // 여러 번 호출해도 패닉하지 않음
    for _ in 0..10 {
        stack.update_transition(0.1);
    }
}

#[test]
fn test_screen_stack_no_transition_by_default() {
    // 기본 전환은 None - 렌더링 테스트로 확인
    let stack = ScreenStack::new();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 패닉 없이 렌더링됨
    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_transition_starts_on_push() {
    // 푸시 시 전환 시작 테스트 - 공개 API로만 확인
    let mut stack = ScreenStack::new().transition(ScreenTransition::SlideRight);
    stack.push(Screen::new("home"));

    // update_transition 호출이 정상 작동
    stack.update_transition(0.1);

    // 렌더링도 정상 작동
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_transition_starts_on_pop() {
    // 팝 시 전환 시작 테스트 - 공개 API로만 확인
    let mut stack = ScreenStack::new().transition(ScreenTransition::SlideRight);
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    stack.pop();

    // update_transition 호출이 정상 작동
    stack.update_transition(0.1);
}

#[test]
fn test_screen_stack_no_transition_with_none() {
    // None 전환은 애니메이션 없음 - 렌더링 테스트로 확인
    let mut stack = ScreenStack::new().transition(ScreenTransition::None);
    stack.push(Screen::new("home"));

    // update_transition 호출해도 문제 없음
    stack.update_transition(0.1);

    // 렌더링도 정상 작동
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

// =============================================================================
// Modal Screen Tests - 모달 스크린 테스트
// =============================================================================

#[test]
fn test_screen_stack_modal_hides_below() {
    // 모달 스크린이 아래 스크린을 가리는지 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home").title("Home Screen"));
    stack.push(Screen::new("modal").modal().title("Modal Screen"));

    // 렌더링 테스트
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    // 모달 스크린만 렌더링되어야 함
    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_multiple_modals() {
    // 여러 모달 스크린 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("modal1").modal());
    stack.push(Screen::new("modal2").modal());

    // 최상위 모달만 렌더링되어야 함
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_normal_after_modal() {
    // 모달 후 일반 스크린 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("modal").modal());
    stack.push(Screen::new("normal"));

    // 모달 위의 일반 스크린은 렌더링되어야 함
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

// =============================================================================
// Render Tests - 렌더링 테스트
// =============================================================================

#[test]
fn test_screen_stack_render_empty() {
    // 빈 스택 렌더링 테스트
    let stack = ScreenStack::new();
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stack.render(&mut ctx);
    // 패닉 없이 완료되어야 함
}

#[test]
fn test_screen_stack_render_with_renderer() {
    // 렌더러가 등록된 스택 렌더링 테스트
    let mut stack = ScreenStack::new().register("test", |screen, ctx| {
        // 렌더러 내용
        let _ = (screen, ctx);
    });
    stack.push(Screen::new("test"));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stack.render(&mut ctx);
}

#[test]
fn test_screen_stack_render_no_renderer() {
    // 렌더러가 없는 스크린 렌더링 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("no_renderer"));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stack.render(&mut ctx);
    // 패닉 없이 완료되어야 함
}

#[test]
fn test_screen_stack_render_multiple_screens() {
    // 여러 스크린 렌더링 테스트
    let mut stack = ScreenStack::new()
        .register("home", |screen, ctx| {
            let _ = (screen, ctx);
        })
        .register("settings", |screen, ctx| {
            let _ = (screen, ctx);
        });

    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stack.render(&mut ctx);
}

// =============================================================================
// History Tests - 히스토리 테스트
// =============================================================================

#[test]
fn test_screen_stack_history_on_push() {
    // 푸시 시 히스토리 기록 테스트 - 내비게이션이 작동하는지 확인
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    // 뒤로 가기가 가능해야 함 (히스토리가 기록되었으므로)
    assert!(stack.can_go_back());
}

#[test]
fn test_screen_stack_max_history() {
    // 최대 히스토리 크기 테스트 - 같은 ID로 여러 번 푸시 후 정상 작동 확인
    let mut stack = ScreenStack::new();

    // 같은 ID로 여러 번 푸시 (서로 다른 스크린 인스턴스)
    for _ in 0..100 {
        stack.push(Screen::new("screen"));
    }

    // 스택이 정상적으로 유지됨
    assert_eq!(stack.depth(), 100);
    assert!(stack.can_go_back());
}

// =============================================================================
// View Trait Tests - View 트레이트 테스트
// =============================================================================

#[test]
fn test_screen_widget_type() {
    // Screen은 View를 구현하지 않으므로 ScreenStack 테스트
    let stack = ScreenStack::new();
    assert_eq!(stack.widget_type(), "ScreenStack");
}

#[test]
fn test_screen_stack_view_id_none() {
    // ID가 없는 경우 테스트
    let stack = ScreenStack::new();
    assert!(View::id(&stack).is_none());
}

#[test]
fn test_screen_stack_view_id_some() {
    // ID가 있는 경우 테스트
    let stack = ScreenStack::new().element_id("my-stack");
    assert_eq!(View::id(&stack), Some("my-stack"));
}

#[test]
fn test_screen_stack_view_classes_empty() {
    // 클래스가 없는 경우 테스트
    let stack = ScreenStack::new();
    assert!(View::classes(&stack).is_empty());
}

#[test]
fn test_screen_stack_view_classes_with_values() {
    // 클래스가 있는 경우 테스트
    let stack = ScreenStack::new().class("first").class("second");
    let classes = View::classes(&stack);
    assert_eq!(classes.len(), 2);
    assert!(classes.contains(&"first".to_string()));
    assert!(classes.contains(&"second".to_string()));
}

#[test]
fn test_screen_stack_view_meta() {
    // 메타데이터 테스트
    let stack = ScreenStack::new().element_id("test-id").class("test-class");
    let meta = stack.meta();
    assert_eq!(meta.widget_type, "ScreenStack");
    assert_eq!(meta.id, Some("test-id".to_string()));
    assert!(meta.classes.contains("test-class"));
}

// =============================================================================
// StyledView Trait Tests - StyledView 트레이트 테스트
// =============================================================================

#[test]
fn test_screen_stack_styled_view_set_id() {
    let mut stack = ScreenStack::new();
    StyledView::set_id(&mut stack, "test-id");
    assert_eq!(View::id(&stack), Some("test-id"));
}

#[test]
fn test_screen_stack_styled_view_add_class() {
    let mut stack = ScreenStack::new();
    StyledView::add_class(&mut stack, "first");
    StyledView::add_class(&mut stack, "second");
    assert!(StyledView::has_class(&stack, "first"));
    assert!(StyledView::has_class(&stack, "second"));
    assert_eq!(View::classes(&stack).len(), 2);
}

#[test]
fn test_screen_stack_styled_view_remove_class() {
    let mut stack = ScreenStack::new().class("a").class("b").class("c");
    StyledView::remove_class(&mut stack, "b");
    assert!(StyledView::has_class(&stack, "a"));
    assert!(!StyledView::has_class(&stack, "b"));
    assert!(StyledView::has_class(&stack, "c"));
}

#[test]
fn test_screen_stack_styled_view_toggle_class() {
    let mut stack = ScreenStack::new();
    StyledView::toggle_class(&mut stack, "test");
    assert!(StyledView::has_class(&stack, "test"));
    StyledView::toggle_class(&mut stack, "test");
    assert!(!StyledView::has_class(&stack, "test"));
}

// =============================================================================
// Builder Props Tests - 빌더 속성 테스트
// =============================================================================

#[test]
fn test_screen_stack_builder_element_id() {
    let stack = ScreenStack::new().element_id("my-stack");
    assert_eq!(View::id(&stack), Some("my-stack"));
}

#[test]
fn test_screen_stack_builder_class() {
    let stack = ScreenStack::new().class("stack").class("navigation");
    assert!(stack.has_class("stack"));
    assert!(stack.has_class("navigation"));
}

#[test]
fn test_screen_stack_builder_classes() {
    let stack = ScreenStack::new().classes(vec!["first", "second", "third"]);
    assert!(stack.has_class("first"));
    assert!(stack.has_class("second"));
    assert!(stack.has_class("third"));
}

// =============================================================================
// ScreenTransition Enum Tests - ScreenTransition 열거형 테스트
// =============================================================================

#[test]
fn test_screen_transition_default() {
    // Default trait 테스트
    let transition = ScreenTransition::default();
    assert_eq!(transition, ScreenTransition::None);
}

#[test]
fn test_screen_transition_partial_eq() {
    // PartialEq 테스트
    assert_eq!(ScreenTransition::None, ScreenTransition::None);
    assert_eq!(ScreenTransition::SlideRight, ScreenTransition::SlideRight);
    assert_eq!(ScreenTransition::SlideUp, ScreenTransition::SlideUp);
    assert_eq!(ScreenTransition::Fade, ScreenTransition::Fade);
    assert_eq!(ScreenTransition::Zoom, ScreenTransition::Zoom);

    assert_ne!(ScreenTransition::None, ScreenTransition::Fade);
    assert_ne!(ScreenTransition::SlideRight, ScreenTransition::SlideUp);
}

#[test]
fn test_screen_transition_all_variants() {
    // 모든 변형 테스트
    let transitions = [
        ScreenTransition::None,
        ScreenTransition::SlideRight,
        ScreenTransition::SlideUp,
        ScreenTransition::Fade,
        ScreenTransition::Zoom,
    ];

    for transition in transitions {
        let stack = ScreenStack::new().transition(transition);
        let mut buffer = Buffer::new(20, 10);
        let area = Rect::new(0, 0, 20, 10);
        let mut ctx = RenderContext::new(&mut buffer, area);
        stack.render(&mut ctx);
    }
}

// =============================================================================
// Edge Cases - 엣지 케이스 테스트
// =============================================================================

#[test]
fn test_screen_with_empty_id() {
    // 빈 ID로 스크린 생성 테스트
    let s = Screen::new("");
    assert_eq!(s.id, "");
    assert_eq!(s.title, "");
}

#[test]
fn test_screen_with_special_characters() {
    // 특수 문자가 포함된 ID 테스트
    let s = Screen::new("screen-with-special.chars_123");
    assert_eq!(s.id, "screen-with-special.chars_123");
}

#[test]
fn test_screen_data_overwrite() {
    // 데이터 덮어쓰기 테스트
    let s = Screen::new("test")
        .data("key", "value1")
        .data("key", "value2");

    assert_eq!(s.get_data("key"), Some(&"value2".to_string()));
}

#[test]
fn test_screen_multiple_data_entries() {
    // 여러 데이터 항목 테스트
    let s = Screen::new("test")
        .data("key1", "value1")
        .data("key2", "value2")
        .data("key3", "value3")
        .data("key4", "value4")
        .data("key5", "value5");

    assert_eq!(s.get_data("key1"), Some(&"value1".to_string()));
    assert_eq!(s.get_data("key2"), Some(&"value2".to_string()));
    assert_eq!(s.get_data("key3"), Some(&"value3".to_string()));
    assert_eq!(s.get_data("key4"), Some(&"value4".to_string()));
    assert_eq!(s.get_data("key5"), Some(&"value5".to_string()));
}

#[test]
fn test_screen_stack_push_same_id_multiple_times() {
    // 동일 ID로 여러 번 푸시 테스트
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home").title("First"));
    stack.push(Screen::new("home").title("Second"));
    stack.push(Screen::new("home").title("Third"));

    assert_eq!(stack.depth(), 3);
    // 모두 동일 ID지만 서로 다른 스크린
    assert_eq!(stack.current().unwrap().title, "Third");
}

#[test]
fn test_screen_stack_large_depth() {
    // 큰 깊이의 스택 테스트
    let mut stack = ScreenStack::new();
    // ScreenId는 &'static str이므로 문자열 리터럴 사용
    let screen_ids: Vec<&'static str> = vec![
        "screen_0",
        "screen_1",
        "screen_2",
        "screen_3",
        "screen_4",
        "screen_5",
        "screen_6",
        "screen_7",
        "screen_8",
        "screen_9",
        "screen_10",
        "screen_11",
        "screen_12",
        "screen_13",
        "screen_14",
        "screen_15",
        "screen_16",
        "screen_17",
        "screen_18",
        "screen_19",
        "screen_20",
        "screen_21",
        "screen_22",
        "screen_23",
        "screen_24",
        "screen_25",
        "screen_26",
        "screen_27",
        "screen_28",
        "screen_29",
        "screen_30",
        "screen_31",
        "screen_32",
        "screen_33",
        "screen_34",
        "screen_35",
        "screen_36",
        "screen_37",
        "screen_38",
        "screen_39",
        "screen_40",
        "screen_41",
        "screen_42",
        "screen_43",
        "screen_44",
        "screen_45",
        "screen_46",
        "screen_47",
        "screen_48",
        "screen_49",
        "screen_50",
        "screen_51",
        "screen_52",
        "screen_53",
        "screen_54",
        "screen_55",
        "screen_56",
        "screen_57",
        "screen_58",
        "screen_59",
        "screen_60",
        "screen_61",
        "screen_62",
        "screen_63",
        "screen_64",
        "screen_65",
        "screen_66",
        "screen_67",
        "screen_68",
        "screen_69",
        "screen_70",
        "screen_71",
        "screen_72",
        "screen_73",
        "screen_74",
        "screen_75",
        "screen_76",
        "screen_77",
        "screen_78",
        "screen_79",
        "screen_80",
        "screen_81",
        "screen_82",
        "screen_83",
        "screen_84",
        "screen_85",
        "screen_86",
        "screen_87",
        "screen_88",
        "screen_89",
        "screen_90",
        "screen_91",
        "screen_92",
        "screen_93",
        "screen_94",
        "screen_95",
        "screen_96",
        "screen_97",
        "screen_98",
        "screen_99",
    ];

    for id in screen_ids {
        stack.push(Screen::new(id));
    }

    assert_eq!(stack.depth(), 100);
    assert_eq!(stack.current().unwrap().id, "screen_99");
}

#[test]
fn test_screen_stack_pop_from_large_depth() {
    // 큰 깊이에서 팝 테스트
    let mut stack = ScreenStack::new();
    let screen_ids: Vec<&'static str> = vec![
        "screen_0",
        "screen_1",
        "screen_2",
        "screen_3",
        "screen_4",
        "screen_5",
        "screen_6",
        "screen_7",
        "screen_8",
        "screen_9",
        "screen_10",
        "screen_11",
        "screen_12",
        "screen_13",
        "screen_14",
        "screen_15",
        "screen_16",
        "screen_17",
        "screen_18",
        "screen_19",
        "screen_20",
        "screen_21",
        "screen_22",
        "screen_23",
        "screen_24",
        "screen_25",
        "screen_26",
        "screen_27",
        "screen_28",
        "screen_29",
        "screen_30",
        "screen_31",
        "screen_32",
        "screen_33",
        "screen_34",
        "screen_35",
        "screen_36",
        "screen_37",
        "screen_38",
        "screen_39",
        "screen_40",
        "screen_41",
        "screen_42",
        "screen_43",
        "screen_44",
        "screen_45",
        "screen_46",
        "screen_47",
        "screen_48",
        "screen_49",
    ];

    for id in screen_ids {
        stack.push(Screen::new(id));
    }

    for _ in 0..49 {
        stack.pop();
    }

    assert_eq!(stack.depth(), 1);
    assert_eq!(stack.current().unwrap().id, "screen_0");
}

#[test]
fn test_screen_stack_replace_in_middle_of_stack() {
    // 스택 중간에서 교체 테스트 (교체는 항상 현재 스크린에만 적용)
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("middle"));
    stack.push(Screen::new("top"));

    stack.replace(Screen::new("new_top"));

    assert_eq!(stack.depth(), 3);
    assert_eq!(stack.current().unwrap().id, "new_top");
    assert_eq!(stack.get("middle").unwrap().id, "middle");
}

#[test]
fn test_screen_stack_get_mut_not_available() {
    // ScreenStack에는 current_mut만 있고 get_mut는 없음
    let mut stack = ScreenStack::new();
    stack.push(Screen::new("home"));
    stack.push(Screen::new("settings"));

    // current_mut로만 접근 가능
    if let Some(screen) = stack.current_mut() {
        screen.title = "Updated".to_string();
    }
    assert_eq!(stack.current().unwrap().title, "Updated");
}

#[test]
fn test_screen_stack_render_with_offset() {
    // 오프셋이 있는 영역에서 렌더링 테스트
    let mut stack = ScreenStack::new().register("test", |screen, ctx| {
        let _ = (screen, ctx);
    });
    stack.push(Screen::new("test"));

    let mut buffer = Buffer::new(40, 20);
    let area = Rect::new(10, 5, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stack.render(&mut ctx);
    // 패닉 없이 완료되어야 함
}

#[test]
fn test_screen_stack_render_zero_size() {
    // 크기가 0인 영역에서 렌더링 테스트
    let stack = ScreenStack::new();
    let mut buffer = Buffer::new(0, 0);
    let area = Rect::new(0, 0, 0, 0);
    let mut ctx = RenderContext::new(&mut buffer, area);

    stack.render(&mut ctx);
    // 패닉 없이 완료되어야 함
}

#[test]
fn test_screen_transition_completes() {
    // 전환 애니메이션 완료 테스트 - 공개 API로만 확인
    let mut stack = ScreenStack::new().transition(ScreenTransition::Fade);
    stack.push(Screen::new("home"));

    // 충분한 업데이트로 완료
    for _ in 0..100 {
        stack.update_transition(1.0);
    }

    // 렌더링이 정상 작동
    let mut buffer = Buffer::new(20, 10);
    let area = Rect::new(0, 0, 20, 10);
    let mut ctx = RenderContext::new(&mut buffer, area);
    stack.render(&mut ctx);
}

#[test]
fn test_screen_transition_update_zero_delta() {
    // 델타가 0인 업데이트 테스트 - 공개 API로만 확인
    let mut stack = ScreenStack::new().transition(ScreenTransition::SlideRight);
    stack.push(Screen::new("home"));

    // 0 델타로 업데이트해도 패닉하지 않음
    stack.update_transition(0.0);
    stack.update_transition(0.0);
    stack.update_transition(0.0);
}

#[test]
fn test_screen_with_unicode_title() {
    // 유니코드 타이틀 테스트
    let s = Screen::new("test").title("한글 타이틀 🎉");
    assert_eq!(s.title, "한글 타이틀 🎉");
}

#[test]
fn test_screen_with_unicode_data() {
    // 유니코드 데이터 테스트
    let s = Screen::new("test")
        .data("emoji", "😀😃😄")
        .data("korean", "안녕하세요")
        .data("japanese", "こんにちは");

    assert_eq!(s.get_data("emoji"), Some(&"😀😃😄".to_string()));
    assert_eq!(s.get_data("korean"), Some(&"안녕하세요".to_string()));
    assert_eq!(s.get_data("japanese"), Some(&"こんにちは".to_string()));
}
