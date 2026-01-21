//! Context tests

#![allow(unused_imports)]

use revue::reactive::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_create_context() {
    let ctx: Context<String> = create_context();
    assert!(ctx.default().is_none());
}

#[test]
fn test_create_context_with_default() {
    let ctx = create_context_with_default("default_value".to_string());
    assert_eq!(ctx.default(), Some(&"default_value".to_string()));
}

#[test]
fn test_provide_and_use_context() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    provide(&theme, "dark".to_string());

    let value = use_context(&theme);
    assert_eq!(value, Some("dark".to_string()));

    clear_all_contexts();
}

#[test]
fn test_use_context_default() {
    clear_all_contexts();

    let ctx = create_context_with_default(42);

    let value = use_context(&ctx);
    assert_eq!(value, Some(42));

    provide(&ctx, 100);
    let value = use_context(&ctx);
    assert_eq!(value, Some(100));

    clear_all_contexts();
}

#[test]
fn test_use_context_no_provider_no_default() {
    clear_all_contexts();

    let ctx: Context<String> = create_context();
    let value = use_context(&ctx);
    assert_eq!(value, None);

    clear_all_contexts();
}

#[test]
fn test_provide_signal_reactive() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    let signal = provide_signal(&theme, "dark".to_string());

    assert_eq!(use_context(&theme), Some("dark".to_string()));

    signal.set("light".to_string());
    assert_eq!(use_context(&theme), Some("light".to_string()));

    clear_all_contexts();
}

#[test]
fn test_use_context_signal() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    provide(&theme, "dark".to_string());

    let signal = use_context_signal(&theme);
    assert!(signal.is_some());
    assert_eq!(signal.unwrap().get(), "dark");

    clear_all_contexts();
}

#[test]
fn test_has_context() {
    clear_all_contexts();

    let ctx: Context<i32> = create_context();
    assert!(!has_context(&ctx));

    provide(&ctx, 42);
    assert!(has_context(&ctx));

    clear_context(&ctx);
    assert!(!has_context(&ctx));

    clear_all_contexts();
}

#[test]
fn test_clear_context() {
    clear_all_contexts();

    let ctx: Context<String> = create_context();
    provide(&ctx, "value".to_string());
    assert!(has_context(&ctx));

    clear_context(&ctx);
    assert!(!has_context(&ctx));
    assert_eq!(use_context(&ctx), None);

    clear_all_contexts();
}

#[test]
fn test_context_scope() {
    clear_all_contexts();

    let theme: Context<String> = create_context();

    assert_eq!(use_context(&theme), None);

    {
        let scope = ContextScope::new();
        scope.provide(&theme, "scoped_dark".to_string());

        assert_eq!(use_context(&theme), Some("scoped_dark".to_string()));
    }

    assert_eq!(use_context(&theme), None);

    clear_all_contexts();
}

#[test]
fn test_nested_context_scopes() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    provide(&theme, "global".to_string());

    assert_eq!(use_context(&theme), Some("global".to_string()));

    {
        let scope1 = ContextScope::new();
        scope1.provide(&theme, "scope1".to_string());

        assert_eq!(use_context(&theme), Some("scope1".to_string()));

        {
            let scope2 = ContextScope::new();
            scope2.provide(&theme, "scope2".to_string());

            assert_eq!(use_context(&theme), Some("scope2".to_string()));
        }

        assert_eq!(use_context(&theme), Some("scope1".to_string()));
    }

    assert_eq!(use_context(&theme), Some("global".to_string()));

    clear_all_contexts();
}

#[test]
fn test_with_context_scope() {
    clear_all_contexts();

    let count: Context<i32> = create_context();

    let result = with_context_scope(|scope| {
        scope.provide(&count, 42);
        use_context(&count).unwrap_or(0)
    });

    assert_eq!(result, 42);

    assert_eq!(use_context(&count), None);

    clear_all_contexts();
}

#[test]
fn test_multiple_contexts() {
    clear_all_contexts();

    let theme: Context<String> = create_context();
    let locale: Context<String> = create_context();
    let count: Context<i32> = create_context();

    provide(&theme, "dark".to_string());
    provide(&locale, "en-US".to_string());
    provide(&count, 100);

    assert_eq!(use_context(&theme), Some("dark".to_string()));
    assert_eq!(use_context(&locale), Some("en-US".to_string()));
    assert_eq!(use_context(&count), Some(100));

    clear_all_contexts();
}

#[test]
fn test_context_id_uniqueness() {
    let ctx1: Context<i32> = create_context();
    let ctx2: Context<i32> = create_context();

    assert_ne!(ctx1.id(), ctx2.id());
}

#[test]
fn test_provider_struct() {
    let ctx: Context<String> = create_context();
    let provider = Provider::new(&ctx, "initial".to_string());

    assert_eq!(provider.get(), "initial");

    provider.set("updated".to_string());
    assert_eq!(provider.get(), "updated");

    provider.update(|s| s.push_str("!"));
    assert_eq!(provider.get(), "updated!");
}

#[test]
fn test_context_clone() {
    let ctx = create_context_with_default(42);
    let ctx_clone = ctx.clone();

    assert_eq!(ctx.id(), ctx_clone.id());
    assert_eq!(ctx.default(), ctx_clone.default());
}
