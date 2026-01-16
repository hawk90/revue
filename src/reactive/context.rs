//! Context API for sharing state across the component tree.
//!
//! Provides Vue/React-like context for sharing state without prop drilling.
//!
//! # Example
//!
//! ```rust,ignore
//! use revue::reactive::{create_context, provide, use_context};
//!
//! // Define a context
//! let theme_context = create_context::<String>();
//!
//! // Provide a value
//! provide(&theme_context, "dark".to_string());
//!
//! // Consume the value
//! let theme = use_context(&theme_context);
//! assert_eq!(theme, Some("dark".to_string()));
//! ```

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use super::Signal;

// =============================================================================
// Context Types
// =============================================================================

/// Unique identifier for a context
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ContextId(u64);

impl ContextId {
    /// Create a new unique context ID
    fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

/// A context definition for sharing state
///
/// Created with [`create_context`] and used with [`provide`] and [`use_context`].
#[derive(Debug)]
pub struct Context<T> {
    id: ContextId,
    default: Option<T>,
    _marker: PhantomData<T>,
}

impl<T> Context<T> {
    /// Create a new context without a default value
    pub fn new() -> Self {
        Self {
            id: ContextId::new(),
            default: None,
            _marker: PhantomData,
        }
    }

    /// Create a new context with a default value
    pub fn with_default(default: T) -> Self {
        Self {
            id: ContextId::new(),
            default: Some(default),
            _marker: PhantomData,
        }
    }

    /// Get the context ID
    pub fn id(&self) -> ContextId {
        self.id
    }

    /// Get the default value if set
    pub fn default(&self) -> Option<&T> {
        self.default.as_ref()
    }
}

impl<T> Default for Context<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Clone for Context<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            default: self.default.clone(),
            _marker: PhantomData,
        }
    }
}

// =============================================================================
// Context Provider
// =============================================================================

/// A provider that holds a context value
pub struct Provider<T: Clone + Send + Sync + 'static> {
    context_id: ContextId,
    value: Signal<T>,
}

impl<T: Clone + Send + Sync + 'static> Provider<T> {
    /// Create a new provider
    pub fn new(context: &Context<T>, value: T) -> Self {
        Self {
            context_id: context.id,
            value: Signal::new(value),
        }
    }

    /// Get the current value
    pub fn get(&self) -> T {
        self.value.get()
    }

    /// Set a new value
    pub fn set(&self, value: T) {
        self.value.set(value);
    }

    /// Update the value
    pub fn update(&self, f: impl FnOnce(&mut T)) {
        self.value.update(f);
    }

    /// Get the underlying signal
    pub fn signal(&self) -> &Signal<T> {
        &self.value
    }
}

impl<T: Clone + Send + Sync + 'static> Clone for Provider<T> {
    fn clone(&self) -> Self {
        Self {
            context_id: self.context_id,
            value: self.value.clone(),
        }
    }
}

// =============================================================================
// Context Store (Thread-Local)
// =============================================================================

type ContextValue = Arc<dyn Any + Send + Sync>;

thread_local! {
    /// Stack of context scopes for nested providers
    static CONTEXT_STACK: RefCell<Vec<HashMap<ContextId, ContextValue>>> = RefCell::new(Vec::new());

    /// Global context store for root-level providers
    static GLOBAL_CONTEXTS: RefCell<HashMap<ContextId, ContextValue>> = RefCell::new(HashMap::new());
}

// =============================================================================
// Public API
// =============================================================================

/// Create a new context
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::create_context;
///
/// let theme_context = create_context::<String>();
/// ```
pub fn create_context<T>() -> Context<T> {
    Context::new()
}

/// Create a new context with a default value
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::create_context_with_default;
///
/// let theme_context = create_context_with_default("light".to_string());
/// ```
pub fn create_context_with_default<T>(default: T) -> Context<T> {
    Context::with_default(default)
}

/// Provide a context value
///
/// The value will be available to all descendants via [`use_context`].
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
/// ```
pub fn provide<T: Clone + Send + Sync + 'static>(context: &Context<T>, value: T) {
    let signal = Signal::new(value);
    let boxed: ContextValue = Arc::new(signal);

    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().insert(context.id, boxed);
    });
}

/// Provide a context value with a signal for reactive updates
///
/// Returns the signal so you can update the value later.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide_signal};
///
/// let theme = create_context::<String>();
/// let theme_signal = provide_signal(&theme, "dark".to_string());
///
/// // Later, update the theme
/// theme_signal.set("light".to_string());
/// ```
pub fn provide_signal<T: Clone + Send + Sync + 'static>(
    context: &Context<T>,
    value: T,
) -> Signal<T> {
    let signal = Signal::new(value);
    let boxed: ContextValue = Arc::new(signal.clone());

    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().insert(context.id, boxed);
    });

    signal
}

/// Use a context value
///
/// Returns the current value from the nearest provider, or the default value if set.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide, use_context};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
///
/// let current_theme = use_context(&theme);
/// assert_eq!(current_theme, Some("dark".to_string()));
/// ```
pub fn use_context<T: Clone + Send + Sync + 'static>(context: &Context<T>) -> Option<T> {
    // First check the context stack (scoped providers)
    let from_stack = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        // Search from innermost to outermost scope
        for scope in stack.iter().rev() {
            if let Some(value) = scope.get(&context.id) {
                if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                    return Some(signal.get());
                }
            }
        }
        None
    });

    if from_stack.is_some() {
        return from_stack;
    }

    // Then check global contexts
    let from_global = GLOBAL_CONTEXTS.with(|store| {
        let store = store.borrow();
        if let Some(value) = store.get(&context.id) {
            if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                return Some(signal.get());
            }
        }
        None
    });

    if from_global.is_some() {
        return from_global;
    }

    // Fall back to default
    context.default.clone()
}

/// Use a context signal for reactive updates
///
/// Returns a clone of the signal so updates to the signal trigger reactivity.
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, provide, use_context_signal, effect};
///
/// let theme = create_context::<String>();
/// provide(&theme, "dark".to_string());
///
/// let theme_signal = use_context_signal(&theme);
///
/// effect(move || {
///     if let Some(sig) = &theme_signal {
///         println!("Theme changed to: {}", sig.get());
///     }
/// });
/// ```
pub fn use_context_signal<T: Clone + Send + Sync + 'static>(
    context: &Context<T>,
) -> Option<Signal<T>> {
    // First check the context stack
    let from_stack = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for scope in stack.iter().rev() {
            if let Some(value) = scope.get(&context.id) {
                if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                    return Some(signal.clone());
                }
            }
        }
        None
    });

    if from_stack.is_some() {
        return from_stack;
    }

    // Then check global contexts
    GLOBAL_CONTEXTS.with(|store| {
        let store = store.borrow();
        if let Some(value) = store.get(&context.id) {
            if let Some(signal) = value.downcast_ref::<Signal<T>>() {
                return Some(signal.clone());
            }
        }
        None
    })
}

/// Check if a context has a provided value
pub fn has_context<T: Clone + Send + Sync + 'static>(context: &Context<T>) -> bool {
    // Check stack
    let in_stack = CONTEXT_STACK.with(|stack| {
        let stack = stack.borrow();
        for scope in stack.iter().rev() {
            if scope.contains_key(&context.id) {
                return true;
            }
        }
        false
    });

    if in_stack {
        return true;
    }

    // Check global
    GLOBAL_CONTEXTS.with(|store| store.borrow().contains_key(&context.id))
}

/// Clear a context value
pub fn clear_context<T>(context: &Context<T>) {
    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().remove(&context.id);
    });
}

/// Clear all context values (useful for testing)
pub fn clear_all_contexts() {
    GLOBAL_CONTEXTS.with(|store| {
        store.borrow_mut().clear();
    });
    CONTEXT_STACK.with(|stack| {
        stack.borrow_mut().clear();
    });
}

// =============================================================================
// Scoped Context
// =============================================================================

/// A scope for providing context values to a specific subtree
///
/// When the scope is dropped, the context values are removed.
pub struct ContextScope {
    _private: (),
}

impl ContextScope {
    /// Create a new context scope
    pub fn new() -> Self {
        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().push(HashMap::new());
        });
        Self { _private: () }
    }

    /// Provide a value within this scope
    pub fn provide<T: Clone + Send + Sync + 'static>(&self, context: &Context<T>, value: T) {
        let signal = Signal::new(value);
        let boxed: ContextValue = Arc::new(signal);

        CONTEXT_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if let Some(scope) = stack.last_mut() {
                scope.insert(context.id, boxed);
            }
        });
    }

    /// Provide a signal within this scope
    pub fn provide_signal<T: Clone + Send + Sync + 'static>(
        &self,
        context: &Context<T>,
        value: T,
    ) -> Signal<T> {
        let signal = Signal::new(value);
        let boxed: ContextValue = Arc::new(signal.clone());

        CONTEXT_STACK.with(|stack| {
            let mut stack = stack.borrow_mut();
            if let Some(scope) = stack.last_mut() {
                scope.insert(context.id, boxed);
            }
        });

        signal
    }
}

impl Default for ContextScope {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for ContextScope {
    fn drop(&mut self) {
        CONTEXT_STACK.with(|stack| {
            stack.borrow_mut().pop();
        });
    }
}

/// Run a function with a scoped context
///
/// # Example
///
/// ```rust,ignore
/// use revue::reactive::{create_context, with_context_scope, use_context};
///
/// let theme = create_context::<String>();
///
/// with_context_scope(|scope| {
///     scope.provide(&theme, "dark".to_string());
///
///     let value = use_context(&theme);
///     assert_eq!(value, Some("dark".to_string()));
/// });
///
/// // Outside the scope, the value is gone
/// let value = use_context(&theme);
/// assert_eq!(value, None);
/// ```
pub fn with_context_scope<F, R>(f: F) -> R
where
    F: FnOnce(&ContextScope) -> R,
{
    let scope = ContextScope::new();
    f(&scope)
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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

        // No provider, should get default
        let value = use_context(&ctx);
        assert_eq!(value, Some(42));

        // With provider, should get provided value
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

        // Outside scope
        assert_eq!(use_context(&theme), None);

        {
            let scope = ContextScope::new();
            scope.provide(&theme, "scoped_dark".to_string());

            // Inside scope
            assert_eq!(use_context(&theme), Some("scoped_dark".to_string()));
        }

        // Outside scope again
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

            // Back to scope1
            assert_eq!(use_context(&theme), Some("scope1".to_string()));
        }

        // Back to global
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

        // Outside scope
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
}
